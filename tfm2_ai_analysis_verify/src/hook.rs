//! 검증 목록의 Win64 정수 ABI만 지원하는 실험용 inline detour.
//! armed 설정/ABI와 relocation 없는 명령 경계 검증은 상위 manifest가 책임진다.
//! 경기·배경 시뮬 없는 초기화 상태에서 armed를 켜는 사용자 수동 전제이며 자동 판별하지 않는다.
//! 실게임 미검증.
use crate::platform;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub struct HookSpec {
    pub rva: usize,
    pub prologue: &'static [u8],
    pub body: &'static [u8],
    pub wrapper: usize,
    pub allow_foreign_chain: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct InstalledHook {
    pub original: usize,
    pub foreign_chain: bool,
    /// vanilla는 원 함수 진입, FOREIGN_CHAIN은 기존 48B8 detour의 다음 target.
    pub target: usize,
}
// 실패 후에도 같은 RVA를 재시도하지 않는다. 프레임별 재후킹/체인 변경 추적 없음.
static ATTEMPTED: Mutex<Vec<usize>> = Mutex::new(Vec::new());

fn rip_jump(target: usize, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    bytes.extend_from_slice(&(target as u64).to_le_bytes());
}
fn mov_rax_target(bytes: &[u8]) -> Option<usize> {
    if bytes.len() >= 12 && bytes[..2] == [0x48, 0xb8] && bytes[10..12] == [0xff, 0xe0] {
        Some(u64::from_le_bytes(bytes[2..10].try_into().ok()?) as usize)
    } else { None }
}
fn inspect_chain(first: usize, entry: usize, wrapper: usize) -> Result<(), &'static str> {
    let mut seen = [0usize; 16];
    let mut target = first;
    for i in 0..seen.len() {
        if target == entry || target == wrapper || seen[..i].contains(&target) {
            return Err("foreign detour cycle/own-wrapper target refused");
        }
        platform::executable_range(target, 12)?;
        seen[i] = target;
        let mut head = [0u8; 12];
        platform::safe_read_bytes(target, &mut head)?;
        if let Some(next) = mov_rax_target(&head) { target = next; }
        else { return Ok(()); }
    }
    Err("foreign detour chain depth exceeds bound")
}

/// original은 wrapper가 Acquire로 읽는다. wrapper는 원본을 딱 한 번 실행해야 한다.
/// 호출자는 모든 설치 결과/domain 게시를 끝내기 전까지 비교 기능을 꺼 두어야 한다.
/// 오류가 나도 original!=0이면 코드 쓰기가 시도된 상태: 해제/0 복원/재시도 금지.
/// 동적 unwind 등록/일반 relocation/원래 return-address 동일성 보존은 제공하지 않는다.
pub fn install_once(spec: &HookSpec, original: &AtomicUsize) -> Result<InstalledHook, &'static str> {
    let mut attempted = ATTEMPTED.lock().map_err(|_| "hook installer lock poisoned")?;
    if attempted.contains(&spec.rva) { return Err("hook RVA already attempted; restart required"); }
    attempted.push(spec.rva);
    if original.load(Ordering::Acquire) != 0 { return Err("original pointer already published"); }
    if spec.prologue.len() < 12 || spec.prologue.len() > 64 || spec.body.len() < spec.prologue.len()
        || spec.body.len() > 16 * 1024 * 1024 || !spec.body.starts_with(spec.prologue) {
        return Err("invalid pinned body/prologue specification");
    }
    platform::init_safe_reader()?;
    let base = platform::module_base()?;
    let entry = base.checked_add(spec.rva).ok_or("hook RVA overflow")?;
    platform::executable_range(entry, spec.body.len())?;
    platform::executable_range(spec.wrapper, 1)?;
    if spec.wrapper == entry { return Err("wrapper equals original entry"); }
    let mut before = vec![0u8; spec.body.len()];
    platform::safe_read_bytes(entry, &mut before)?;
    let foreign_target = mov_rax_target(&before);
    let foreign = before != spec.body;
    let target = if foreign {
        if !spec.allow_foreign_chain { return Err("body differs; foreign chain disabled"); }
        let next = foreign_target.ok_or("unknown entry patch/body change refused")?;
        if before[12..] != spec.body[12..] { return Err("foreign body differs outside first 12 bytes"); }
        inspect_chain(next, entry, spec.wrapper)?;
        next
    } else { entry };
    let patch_len = if foreign { 12 } else { spec.prologue.len() };
    let mut trampoline = Vec::with_capacity(spec.prologue.len() + 14);
    if foreign {
        // 기존 entry의 MOV RAX,next / JMP RAX를 그대로 재현한다.
        // RIP 간접 jump만 쓰면 외부 wrapper 진입 RAX가 CALL 시점 잔류값이 된다.
        trampoline.extend_from_slice(&before[..12]);
    }
    else {
        // copied prologue에 RIP/분기/CALL/분할 명령이 없다는 근거는 고정 manifest에 둔다.
        trampoline.extend_from_slice(spec.prologue);
        let resume = entry.checked_add(spec.prologue.len()).ok_or("trampoline resume overflow")?;
        rip_jump(resume, &mut trampoline);
    }
    let stub = platform::allocate_code(&trampoline)?;
    let mut patch = vec![0x90u8; patch_len];
    patch[..2].copy_from_slice(&[0x48, 0xb8]);
    patch[2..10].copy_from_slice(&(spec.wrapper as u64).to_le_bytes());
    patch[10..12].copy_from_slice(&[0xff, 0xe0]);
    let mut recheck = vec![0u8; spec.body.len()];
    // 이 아래는 다른 스레드가 정지할 수 있다. Rust 할당/로그/일반 콜백 없음.
    let mut stopped = match platform::SuspendedThreads::acquire(entry, patch_len) {
        Ok(g) => g,
        Err(e) => { unsafe { platform::free_unpublished_code(stub); } return Err(e); }
    };
    let checked = platform::safe_read_bytes(entry, &mut recheck);
    if checked.is_err() || recheck != before {
        let resumed = stopped.resume_all();
        unsafe { platform::free_unpublished_code(stub); }
        resumed?;
        return Err("body changed/unreadable after suspension; refusing patch");
    }
    // 원본을 먼저 게시한다. 패치 직후 wrapper가 실행돼도 null 원본을 볼 수 없다.
    original.store(stub, Ordering::SeqCst);
    let outcome = unsafe { platform::write_code(entry, &patch) };
    if !outcome.attempted {
        // 쓰기를 시작하지 않았고 현재 entry가 우리 wrapper로 바뀐 적 없는 경로만 철회.
        original.store(0, Ordering::SeqCst);
    }
    let resumed = stopped.resume_all();
    if !outcome.attempted { unsafe { platform::free_unpublished_code(stub); } }
    // 쓰기 시도 뒤에는 성공/부분 실패와 관계없이 RX stub/original을 영구 유지한다.
    outcome.result?;
    resumed?;
    Ok(InstalledHook { original: stub, foreign_chain: foreign, target })
}
