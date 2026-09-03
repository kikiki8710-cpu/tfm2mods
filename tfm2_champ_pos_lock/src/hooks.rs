//! hooks.rs — 게임함수 detour (축 2: 포지션 배정 강제).
//! ===========================================================================
//! Hook A: champ→eligible-positions 비트마스크 산출기 detour.
//!   RE(2026-08-19, RE\2026-08-19_0.5.5-포지션배정-RE.md): 0.5.5 RVA_POS_MASK=0x1294180,
//!   `u32 fn(agent* rcx, u64 champ_id rdx, u8 flag r8b)` — 5포지션 순회 fast_pos_fit_score
//!   → 문턱 이내 포지션들의 하위 5비트 마스크 반환. 서버가 AI 팀 (champ,pos) 배정과
//!   플랜 평가에 사용. 원본 실행 후 lock 챔프면 (orig & allowed), 공집합이면 allowed 로
//!   교체 = AI 는 그 챔프를 허용 포지션에만 배정.
//!   champ_id = available_champions 인덱스 전제(DraftScoreHook candidate 와 동일 축).
//! Hook B: 참가자레코드 조립기(0x1a636c0) post-call 교정 — 후속 RE 대기 중(미구현).
//! ===========================================================================
#![allow(dead_code)]

use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

use crate::config;
use crate::config::MASK_ALL;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentProcessId() -> u32;
    fn GetCurrentThreadId() -> u32;
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> usize;
    fn Thread32First(snap: usize, te: *mut ThreadEntry32) -> i32;
    fn Thread32Next(snap: usize, te: *mut ThreadEntry32) -> i32;
    fn OpenThread(access: u32, inherit: i32, tid: u32) -> usize;
    fn SuspendThread(h: usize) -> u32;
    fn ResumeThread(h: usize) -> u32;
    fn GetThreadContext(h: usize, ctx: *mut u8) -> i32;
    fn CloseHandle(h: usize) -> i32;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;
const TH32CS_SNAPTHREAD: u32 = 0x00000004;
const THREAD_SUSPEND_RESUME: u32 = 0x0002;
const THREAD_GET_CONTEXT: u32 = 0x0008;
const CONTEXT_CONTROL_AMD64: u32 = 0x00100001;
const CTX_RIP_OFF: usize = 0xf8; // x64 CONTEXT.Rip
const CTX_FLAGS_OFF: usize = 0x30; // x64 CONTEXT.ContextFlags

#[repr(C)]
struct ThreadEntry32 {
    dw_size: u32,
    cnt_usage: u32,
    th32_thread_id: u32,
    th32_owner_process_id: u32,
    tp_base_pri: i32,
    tp_delta_pri: i32,
    dw_flags: u32,
}

/// ★스레드 안전 진입 패치(2026-08-21): recommend 처럼 워커스레드에서 동시 실행되는 함수는
/// 진입 12B 를 비원자 패치하면 그 코드를 실행 중인 워커가 반쯤 패치된 명령을 밟아 하드행
/// (=시작 검은화면 실사고). 다른 스레드를 전부 suspend → 프롤로그 [fn,fn+12) 실행 중인
/// 스레드가 하나라도 있으면 패치 보류(다음 프레임 재시도) → 없을 때만 패치 → resume.
/// ⚠suspend 중엔 절대 힙 할당 금지(suspend된 스레드가 힙락 보유 시 데드락) → 핸들·ctx버퍼는
/// suspend 전에 미리 확보.
unsafe fn patch_entry_thread_safe(fn_addr: usize, repl: usize) -> Result<(), &'static str> {
    let pid = GetCurrentProcessId();
    let my_tid = GetCurrentThreadId();
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == 0 || snap == usize::MAX {
        return Err("snapshot");
    }
    // 1) 스레드 핸들 전부 미리 확보(아직 suspend 안 함 = 할당 안전).
    let mut handles: Vec<usize> = Vec::with_capacity(64);
    let mut te: ThreadEntry32 = core::mem::zeroed();
    te.dw_size = core::mem::size_of::<ThreadEntry32>() as u32;
    if Thread32First(snap, &mut te) != 0 {
        loop {
            if te.th32_owner_process_id == pid && te.th32_thread_id != my_tid {
                let h = OpenThread(THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT, 0, te.th32_thread_id);
                if h != 0 {
                    handles.push(h);
                }
            }
            if Thread32Next(snap, &mut te) == 0 {
                break;
            }
        }
    }
    CloseHandle(snap);
    // ctx 버퍼(16정렬, x64 CONTEXT ~1232B) — suspend 전 확보.
    let mut ctx_store = vec![0u8; 1232 + 16];
    let ctx_ptr = (ctx_store.as_mut_ptr() as usize + 15) & !15;

    // 2) 전부 suspend (여기서부터 resume 까지 할당 금지).
    for &h in &handles {
        SuspendThread(h);
    }
    // 3) 프롤로그 실행 중 스레드 검사.
    let mut in_prologue = false;
    for &h in &handles {
        core::ptr::write_bytes(ctx_ptr as *mut u8, 0, 1232);
        *((ctx_ptr + CTX_FLAGS_OFF) as *mut u32) = CONTEXT_CONTROL_AMD64;
        if GetThreadContext(h, ctx_ptr as *mut u8) != 0 {
            let rip = *((ctx_ptr + CTX_RIP_OFF) as *const u64) as usize;
            if rip >= fn_addr && rip < fn_addr + 12 {
                in_prologue = true;
                break;
            }
        }
    }
    // 4) 패치(아무도 프롤로그에 없을 때만).
    let result = if in_prologue {
        Err("in-prologue-retry")
    } else {
        let mut patch = [0u8; 12];
        patch[0] = 0x48;
        patch[1] = 0xb8;
        patch[2..10].copy_from_slice(&repl.to_le_bytes());
        patch[10] = 0xff;
        patch[11] = 0xe0;
        let mut old: u32 = 0;
        if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 {
            Err("vprotect")
        } else {
            core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
            VirtualProtect(fn_addr, 12, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
            Ok(())
        }
    };
    // 5) 전부 resume + 핸들 닫기.
    for &h in &handles {
        ResumeThread(h);
        CloseHandle(h);
    }
    drop(ctx_store);
    result
}

// ── 0.5.6 RVA (패치 시 재핀 대상 — 이 파일이 단일 수정점) ──────────────────
/// champ→eligible-positions 비트마스크 산출기.
/// ★0.5.5 0x1294180 → 0.5.6 = **0xf83830** (ghidra-re 2026-08-21 확정).
///   ⚠구 0x2e739e0 은 migrate_rva 오매칭(Skia GPU 렌더러 — 프롤로그 우연일치) = hookA 0회 발화.
///   본문 변경으로 body-sig 이설 실패했던 함수라, fast_pos_fit(0xf5bdd0) 호출자 역추적+계약대조로 확정.
///   호출경로: AI밴픽스코어러 → 캐시게터 0x10659d0(미스 시) → 0xf83830 → fast_pos_fit.
///   캐시 미스 때만 호출 = 챔프당 1회 발화(교정값 캐시 영속) = 정상.
const RVA_POS_MASK: usize = 0x12153e0;
/// 프롤로그 12B = push r15/r14/r13/r12/rsi/rdi/rbp/rbx … (전부 1~2B push — 12B 경계 클린)
const PROL_POS_MASK: [u8; 12] = [
    0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53,
];

// Hook C (유저 픽 차단 — 피어리스 픽불가 차용, RE 2026-08-20 4차):
/// contains 헬퍼 `bool al = contains(rcx=&String{cap,ptr,len}, rdx=list.ptr, r8=list.len)`
/// (0.5.5 0x943440 → 0.5.6 재핀·프롤로그 동일)
const RVA_CONTAINS: usize = 0xc26cd0;
/// contains 프롤로그 14B(사이트 검증용 — 함수 자체는 패치하지 않음)
const PROL_CONTAINS: [u8; 14] = [
    0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xEC, 0x20, 0x4D, 0x85, 0xC0,
];
/// slot_widget(0x1971b00) 내부 contains E8 콜사이트 2곳 — 피어리스 회색+클릭게이트 지배점.
/// ⚠커밋기(0x12156b0) 내 4콜은 일부러 안 건드림: AI가 fail-open으로 위반 픽을 냈을 때
/// 서버 커밋이 거부되면 턴이 막힐 수 있다 — 차단은 UI(클릭 전)에서만.
const SITES_CONTAINS: [usize; 2] = [0x21676df, 0x21676f8];

// Hook D (밴픽 씬 ptr 캡처 — RE 2026-08-20 5차):
/// slot_widget = banpick_champion_slot 렌더러. 씬 ptr = 스택 인자 arg15 = 진입 시 [rsp+0x78].
/// ⚠2026-08-21 인게임: slot_widget entry 훅은 타 모드에 덮여 고아화 + 그리드셀 렌더는
///   씬 인자=0 → 씬 캡처 실패. **scene_step(아래)로 대체.** slot_widget 은 hookC(회색화
///   콜사이트)만 계속 사용(씬 캡처 용도로는 미사용).
const RVA_SLOT_WIDGET: usize = 0x2161560;
/// 프롤로그 12B push열(그 뒤 mov eax+call chkstk는 원위치 실행 — resume = fn+12)
const PROL_SLOT_WIDGET: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];

// Hook D' (밴픽 씬 ptr 캡처 — scene_step, tfm2_banpick_order 재사용 2026-08-21):
/// scene_step(&BanpickScene)->u8 = 밴픽 phase leaf. **rcx = 진짜 클라 밴픽 씬**(23 client
///   콜러·매프레임 호출·lookahead 아님·T1/T2 구분). banpick_order A'(0.5.6 재핀 0x24d1dc0).
///   씬 오프셋: T1BAN=0x140/T2BAN=0x158/T1PICK=0x170/T2PICK=0x188 (내 O_BAN1/2·O_PICK1/2와 동일).
const RVA_SCENE_STEP: usize = 0x215be10;
/// 프롤로그 **14B**: mov rax,[rcx+0x160](7B) + mov rdx,[rcx+0x178](7B).
/// ⚠★2026-08-23 버그수정: 구버전은 이걸 **12B 로 알고 있었다**. 두 명령은 7+7=14B 라
///   12B 만 떠오면 **두 번째 명령이 중간에서 잘린다** → 스텁 실행 시 `48 8B 91 78 01` 뒤에
///   우리가 이어붙인 `49 BB`(movabs) 가 disp32 로 먹혀 **엉뚱한 주소를 읽고 죽는다**.
///   실측 크래시: `code=0xc0000005, RIP=모듈 밖(=우리 RWX 스텁)`. (유저 제보: user_pick_block=1 일 때만 튕김)
/// (rip-rel 없음·push 없음 → resume=fn+12)
const PROL_SCENE_STEP: [u8; 14] = [
    0x48, 0x8B, 0x81, 0x60, 0x01, 0x00, 0x00, 0x48, 0x8B, 0x91, 0x78, 0x01, 0x00, 0x00,
];

// Hook E (로드된 Assets 캡처 — RE 2026-08-20): 관리화면 챔프 아이콘 렌더가 쓰는
//   진짜 Assets 포인터를 얻어, 설정 팝업에서 공식 아이콘 함수로 raw-aseprite 챔프까지
//   게임 그대로 그린다. 두 지점에서 캡처(둘 다 같은 GAME_ASSETS 에 write):
//   ①챔프아이콘 세터 FUN_14250bc30(RVA 0x250bc30) 진입 rcx=param_1=Assets — 게임정보
//     "챔피언 정보" 탭(champion_info_ui)이 슬롯마다 호출(RE 2026-08-20 확정).
//   ②set_entity_icon(0x2517620) 진입 rdx=param_2=Assets — 밴픽/팀상세 렌더.
/// 챔프아이콘 세터. 진입 rcx = 로드된 Assets(AssetServer/로더).
const RVA_ICON_SETTER: usize = 0x2170920;
/// set_entity_icon leaf. 진입 rdx = 로드된 Assets.
const RVA_ENTITY_ICON: usize = 0x2179a30;
/// 두 함수 공통 프롤로그 첫 12B = push rbp/r15/r14/r13/r12/rsi/rdi/rbx (그 뒤 sub rsp — 원위치)
const PROL_ICON8: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
/// [0]=Assets ptr(rdx) [1]=발화 스탬프. 스텁이 raw write(락 없음 — 재진입 안전).
#[repr(align(16))]
struct AssetsCap([u64; 2]);
static mut ASSETS_CAP: AssetsCap = AssetsCap([0; 2]);
pub static INSTALL_STATE_E: AtomicUsize = AtomicUsize::new(0);

/// 로드된 Assets 포인터(0=미캡처). set_cell_icon 이 공식 함수 호출에 사용.
pub fn game_assets() -> usize {
    unsafe {
        let p = core::ptr::addr_of!(ASSETS_CAP.0);
        core::ptr::read_volatile(p as *const u64) as usize
    }
}

/// 로더 detour(inject.rs)가 매 에셋 로드마다 넘겨받는 `am`(=현 씬 Assets)을 기록.
/// 관리 씬에선 이게 챔프 로드된 AssetServer(매 프레임 신선) → 챔프 화면 렌더를 안 봐도 캡처됨.
/// (메인메뉴 등에선 챔프 없는 assets라 setter 호출이 no-op → 재현 폴백 = 안전.)
pub fn note_assets(am: usize) {
    if (0x10000..1usize << 48).contains(&am) {
        unsafe {
            let p = core::ptr::addr_of_mut!(ASSETS_CAP.0) as *mut u64;
            core::ptr::write_volatile(p, am as u64);
        }
    }
}

/// 게임 챔프아이콘 세터 FUN_14250bc30 의 실주소(0=BASE 미설정). set_cell_icon 이 직접 호출.
/// 시그니처 = fn(assets:rcx, node:rdx, id_ptr:r8, id_len:r9, w, h, scale)  [w/h/scale=스택].
/// ⚠진입이 Hook E 스텁으로 패치돼 있어도 스텁은 rcx 캡처 후 원본 본문으로 이어짐(투과) = 정상.
pub fn icon_setter_fn() -> usize {
    let b = BASE.load(Ordering::Relaxed);
    if b == 0 {
        0
    } else {
        b + RVA_ICON_SETTER
    }
}

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP_POS_MASK: AtomicUsize = AtomicUsize::new(0);
static CONTAINS_FN: AtomicUsize = AtomicUsize::new(0);

pub static CNT_MASK_FIRE: AtomicU64 = AtomicU64::new(0); // detour 진입 자체(함수 호출 여부 진단)
pub static CNT_MASK_CALL: AtomicU64 = AtomicU64::new(0);
pub static CNT_MASK_ADJ: AtomicU64 = AtomicU64::new(0);
/// 진단: 모델 챔프 수(agent+0x10) + 관측된 max champ_id. NAMES.len()과 대조해 인덱스 정합 확인.
pub static MASK_MODEL_CNT: AtomicU64 = AtomicU64::new(0);
pub static MASK_MAX_RDX: AtomicU64 = AtomicU64::new(0);
/// 진단: detour가 받은 rdx(champ_id) 샘플 몇 개 — 인덱스 정합 확인용.
pub static DBG_RDX: [AtomicU64; 4] = [
    AtomicU64::new(u64::MAX),
    AtomicU64::new(u64::MAX),
    AtomicU64::new(u64::MAX),
    AtomicU64::new(u64::MAX),
];
pub static CNT_UI_QUERY: AtomicU64 = AtomicU64::new(0);
pub static CNT_UI_BLOCK: AtomicU64 = AtomicU64::new(0);

/// 설치 결과(1회 기록): 0=미시도 1=OK 2=실패/비활성
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0);
/// Hook C 설치 결과: 0=미시도 1=OK(2사이트) 2=실패/비활성
pub static INSTALL_STATE_C: AtomicUsize = AtomicUsize::new(0);
/// Hook D 설치 결과: 0=미시도 1=OK 2=실패/비활성
pub static INSTALL_STATE_D: AtomicUsize = AtomicUsize::new(0);

// ── Hook D: 씬 캡처 저장소 (스텁이 raw write — UI 메인스레드 단일 작성자) ──
/// [0]=씬 ptr(arg15), [1]=발화 카운터(프레임 활동 감지 — post_update가 "이번 프레임
/// slot_widget이 실제로 돌았나"를 이걸로 판단해, 밴픽 종료 후 stale 씬 read를 차단).
#[repr(align(16))]
struct SceneCap([u64; 2]);
static mut SCENE_CAP: SceneCap = SceneCap([0; 2]);

/// (scene_ptr, stamp) — stamp가 직전 관측과 같으면 이번 프레임 밴픽 렌더 없음.
pub fn scene_cap() -> (usize, u64) {
    unsafe {
        let p = core::ptr::addr_of!(SCENE_CAP.0);
        let s = core::ptr::read_volatile((p as *const u64).add(0));
        let c = core::ptr::read_volatile((p as *const u64).add(1));
        (s as usize, c)
    }
}

// ── Hook A detour 본문 ─────────────────────────────────────────────────────
// 원본과 동일 ABI(레지스터 3인자·반환 eax). r9 는 만약을 위해 투과.
extern "C" fn pos_mask_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let tramp = TRAMP_POS_MASK.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0; // 설치 전 호출 불가(설치 후에만 패치됨) — 방어
    }
    let orig: extern "C" fn(usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(tramp) };
    let ret = orig(rcx, rdx, r8, r9);
    // 진단: detour 진입 자체 카운트 + rdx 샘플(첫 4개 서로 다른 값) 기록.
    CNT_MASK_FIRE.fetch_add(1, Ordering::Relaxed);
    // 인덱스 정합 진단: 모델 챔프수(agent rcx +0x10) + max champ_id.
    if (0x10000..1usize << 48).contains(&rcx) {
        let mc = unsafe { core::ptr::read_unaligned((rcx + 0x10) as *const u32) } as u64;
        if mc < 10000 {
            MASK_MODEL_CNT.store(mc, Ordering::Relaxed);
        }
    }
    if (rdx as u64) < 10000 && rdx as u64 > MASK_MAX_RDX.load(Ordering::Relaxed) {
        MASK_MAX_RDX.store(rdx as u64, Ordering::Relaxed);
    }
    for slot in DBG_RDX.iter() {
        let cur = slot.load(Ordering::Relaxed);
        if cur == rdx as u64 {
            break;
        }
        if cur == u64::MAX
            && slot
                .compare_exchange(u64::MAX, rdx as u64, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        {
            break;
        }
    }
    // 교정 — detour 본문은 절대 unwind 금지
    let adj = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_assign_mask {
            return None;
        }
        let masks = crate::masks()?;
        let m = *masks.get(rdx)?;
        if m == MASK_ALL || m == 0 {
            // MASK_ALL=제한없음. m==0=어떤 화이트리스트에도 없는 챔프 → 0마스크를 게임에
            // 넘기면 "어느 포지션도 못 뜀"이라 AI 라인업/밴 시뮬이 발산·행 가능 → fail-open.
            return None;
        }
        CNT_MASK_CALL.fetch_add(1, Ordering::Relaxed);
        let game = (ret & 0x1f) as u8;
        let and = game & m;
        let newm = if and != 0 { and } else { m }; // 게임이 다 별로라 해도 허용 포지션은 강제 유지
        if newm == game {
            return None;
        }
        CNT_MASK_ADJ.fetch_add(1, Ordering::Relaxed);
        Some((ret & !0x1fusize) | newm as usize)
    }));
    match adj {
        Ok(Some(v)) => v,
        _ => ret,
    }
}

// ── Hook C: 유저 픽 차단 (blocklist → 피어리스처럼 회색+클릭불가) ──────────
use std::collections::HashSet;
use std::sync::Mutex;

/// 현재 차단 목록(소문자 챔프 id). post_update가 매 프레임 재계산·게시.
pub static BLOCKLIST: Mutex<Option<HashSet<String>>> = Mutex::new(None);
/// 차단 사유: 챔프 id(소문자) → 막힌 포지션 라벨(예: "미드", "바텀/서폿").
///   회색 셀 호버 툴팁("해당 포지션은 더이상 선택할 수 없습니다: {}")에 쓴다.
pub static BLOCK_REASON: Mutex<Option<std::collections::HashMap<String, String>>> =
    Mutex::new(None);

/// 챔프의 차단 사유 포지션 라벨 조회.
pub fn block_reason(name: &str) -> Option<String> {
    let g = plock(&BLOCK_REASON);
    g.as_ref().and_then(|m| m.get(name).cloned())
}

/// ★회색 처리한 셀의 화면 rect(x,y,w,h) — 커서 히트테스트로 툴팁을 띄우기 위해 수집.
///   셀 순회 훅(cell_paint)이 매 프레임 갱신한다. 노드 rect = +0x240..+0x24c.
static GREY_RECTS: Mutex<Vec<(String, f32, f32, f32, f32)>> = Mutex::new(Vec::new());

#[link(name = "kernel32")]
extern "system" {
    fn GetProcessHeap() -> usize;
    fn HeapAlloc(h: usize, flags: u32, n: usize) -> usize;
    fn HeapFree(h: usize, flags: u32, p: usize) -> i32;
}

/// UI 노드의 자식 검색. 자식 배열 = node+0x20(ptr) / +0x28(개수), **stride 0x268 인라인 배열**,
/// 각 자식 이름 = +0x08(ptr)/+0x10(len). (게임 원본 FUN_1401a8680 재현)
unsafe fn find_child(node: usize, want: &str) -> Option<usize> {
    let arr = safe_rd_u64(node + 0x20)? as usize;
    let cnt = safe_rd_u64(node + 0x28)? as usize;
    if !ptr_ok(arr) || cnt == 0 || cnt > 64 {
        return None;
    }
    for i in 0..cnt {
        let c = arr + i * 0x268;
        let np = safe_rd_u64(c + 8).unwrap_or(0) as usize;
        let nl = safe_rd_u64(c + 0x10).unwrap_or(0) as usize;
        if nl == 0 || nl > 64 || !ptr_ok(np) {
            continue;
        }
        let Some(b) = safe_bytes(np, nl) else { continue };
        if core::str::from_utf8(&b).map(|t| t == want).unwrap_or(false) {
            return Some(c);
        }
    }
    None
}

/// 게임 소유 String(cap@0, ptr@+8, len@+0x10) 을 새 문자열로 교체.
///   ★버퍼는 반드시 게임과 같은 Win32 힙(HeapAlloc(GetProcessHeap()))으로 잡아야 한다
///   (Rust 버퍼를 넘기면 게임이 HeapFree 할 때 크래시).
///   내용이 같으면 아무것도 하지 않는다(매 프레임 재할당 방지).
unsafe fn set_game_string(slot: usize, text: &str) -> bool {
    if !ptr_ok(slot) {
        return false;
    }
    let cap = safe_rd_u64(slot).unwrap_or(0) as usize;
    let ptr = safe_rd_u64(slot + 8).unwrap_or(0) as usize;
    let len = safe_rd_u64(slot + 0x10).unwrap_or(0) as usize;
    if ptr_ok(ptr) && len == text.len() {
        let Some(cur) = safe_bytes(ptr, len) else { return false };
        if cur == text.as_bytes() {
            return true; // 이미 동일 → write 생략
        }
    }
    let heap = GetProcessHeap();
    if heap == 0 {
        return false;
    }
    let n = text.len();
    let p = HeapAlloc(heap, 0, n.max(1));
    if p == 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping(text.as_ptr(), p as *mut u8, n);
    if cap != 0 && ptr_ok(ptr) {
        HeapFree(heap, 0, ptr);
    }
    core::ptr::write(slot as *mut u64, n as u64);
    core::ptr::write((slot + 8) as *mut u64, p as u64);
    core::ptr::write((slot + 0x10) as *mut u64, n as u64);
    true
}

/// 셀의 fearless_tooltip 에 문구를 넣는다(폭도 넓힘). 게임이 호버 판정·표시·배치를 담당.
unsafe fn set_cell_tooltip(node: usize, msg: &str) {
    let Some(tip) = find_child(node, "fearless_tooltip") else {
        return;
    };
    let rdf = |a: usize| -> f32 {
        if ptr_ok(a) { core::ptr::read(a as *const f32) } else { 0.0 }
    };
    // ── ①같은 셀의 pos_tooltip(주 사용 포지션) 실측 폭 ──
    //   ★계산 rect(+0x240 x / +0x248 w)는 **그 프레임에 실제로 보인 노드에만** 채워지고,
    //     안 보이면 전부 0 이다(실측 2026-08-22). 그래서 w>1 일 때만 유효값으로 취급한다.
    let pos_rect_w = find_child(node, "pos_tooltip")
        .map(|pt| rdf(pt + 0x248))
        .unwrap_or(0.0);

    let fit = {
        let (mut k, mut a) = (0usize, 0usize);
        for c in msg.chars() {
            if (c as u32) > 0x7f { k += 1 } else { a += 1 }
        }
        // ★게임 내 이분탐색 실측(2026-08-22, 0.5.6): 줄바꿈이 생기지 않는 최소 폭은
        //   한글 1자 ≈ 13.0 / ASCII 1자 ≈ 4.0, 여백 항 ≈ 0.
        //     15한글 → 192 / 30한글 → 389 / 18한글+7ASCII → 255.5 (모두 ±3)
        //   여기에 안전여유 10 만 얹는다 ⟹ 포지션이 1~4개로 늘어도 글자 수에 맞춰 자동으로 늘고 준다.
        (k as f32 * 13.0 + a as f32 * 4.0 + 10.0).clamp(140.0, 700.0)
    };
    let w = fit;

    // ── ②폭: 문구가 한 줄에 들어가는 크기(style 4블록, stride 0x80 / value=+0x74) ──
    //   폭 모드(+0x70): 1=px 고정. (0=부모 폭 채우기, 2=% — 내용 맞춤 모드는 없다. 실측 2026-08-22)
    for o in [0x70usize, 0xf0, 0x170, 0x1f0] {
        let p = tip + o;
        if ptr_ok(p) {
            core::ptr::write(p as *mut u32, 1u32);
        }
    }
    for o in [0x74usize, 0xf4, 0x174, 0x1f4] {
        let p = tip + o;
        if ptr_ok(p) {
            core::ptr::write(p as *mut f32, w);
        }
    }
    // ── ③위치: 오른쪽 끝만 pos_tooltip 에 맞추고 그만큼 **왼쪽으로 튀어나오게** ──
    //   ★계산 rect(+0x240)에 x 를 직접 써도 소용없다 — 레이아웃이 매 프레임 덮어쓴다(실측).
    //     두 툴팁 모두 앵커/피벗 X = 0.5/0.5(셀 중앙 정렬, +0xa0/+0xa8) 이므로
    //     **X 오프셋(+0x84, mode=+0x80)** 으로 왼쪽으로 민다:
    //       offset = −(내 폭 − pos_tooltip 폭) / 2   ⟹ 두 박스의 오른쪽 끝이 일치.
    //   pos_tooltip 폭은 그 프레임에 보일 때만 유효하므로 마지막 실측값을 캐시한다.
    let pos_w = if pos_rect_w > 1.0 {
        POS_TIP_W.store(pos_rect_w.to_bits(), Ordering::Relaxed);
        pos_rect_w
    } else {
        f32::from_bits(POS_TIP_W.load(Ordering::Relaxed))
    };
    let off = -((w - pos_w) * 0.5).max(0.0);
    for o in [0x84usize, 0x104, 0x184, 0x204] {
        let p = tip + o;
        if ptr_ok(p) {
            core::ptr::write(p as *mut f32, off);
        }
    }
    let Some(txt) = find_child(tip, "text") else {
        return;
    };
    let lr = safe_rd_u64(txt + 0x230).unwrap_or(0) as usize;
    if ptr_ok(lr) {
        set_game_string(lr + 0x160, msg);
    }
}

/// 차단 해제 시 그 셀의 툴팁을 숨긴다(state 가 {0,1,6,7} 밖이면 게임이 더는 손대지 않으므로).
unsafe fn hide_cell_tooltip(node: usize) {
    if let Some(tip) = find_child(node, "fearless_tooltip") {
        let a = tip + 0x260;
        if ptr_ok(a) {
            core::ptr::write(a as *mut u8, 0u8);
        }
    }
}

/// 커서 좌표에 있는 회색 셀의 챔프 id 반환.
pub fn grey_hit(mx: f32, my: f32) -> Option<String> {
    let g = GREY_RECTS.lock().unwrap_or_else(|e| e.into_inner());
    g.iter()
        .find(|(_, x, y, w, h)| *w > 0.0 && *h > 0.0 && mx >= *x && mx <= *x + *w && my >= *y && my <= *y + *h)
        .map(|(n, ..)| n.clone())
}

fn grey_rect_note(name: &str, node: usize) {
    let rd = |o: usize| -> f32 {
        unsafe { safe_rd_u64(node + o) }
            .map(|v| f32::from_bits((v & 0xffff_ffff) as u32))
            .unwrap_or(0.0)
    };
    let (x, y, w, h) = (rd(0x240), rd(0x244), rd(0x248), rd(0x24c));
    if !(w > 0.0 && h > 0.0) {
        return;
    }
    let mut g = GREY_RECTS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(e) = g.iter_mut().find(|(n, ..)| n == name) {
        *e = (name.to_string(), x, y, w, h);
    } else if g.len() < 256 {
        g.push((name.to_string(), x, y, w, h));
    }
}

/// 회색이 아닌(=해제된) 셀은 히트 목록에서 제거.
fn grey_rect_drop(name: &str) {
    let mut g = GREY_RECTS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(i) = g.iter().position(|(n, ..)| n == name) {
        g.remove(i);
    }
}
/// 게임이 피어리스로 회색 처리한 챔프(orig contains==true 관측 누적) — fail-open 풀 계산용.
pub static FEARLESS_SEEN: Mutex<Option<HashSet<String>>> = Mutex::new(None);

fn plock<'a, T>(m: &'a Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// needle = &String{cap@0, ptr@+8, len@+0x10} — 범위 체크 후 소문자 사본.
unsafe fn read_needle(needle: usize) -> Option<String> {
    if !(0x10000..1usize << 48).contains(&needle) {
        return None;
    }
    let ptr = core::ptr::read((needle + 8) as *const usize);
    let len = core::ptr::read((needle + 0x10) as *const usize);
    if !(0x10000..1usize << 48).contains(&ptr) || len == 0 || len > 64 {
        return None;
    }
    let bytes = safe_bytes(ptr, len)?;
    let s = core::str::from_utf8(&bytes).ok()?;
    Some(s.to_ascii_lowercase())
}

/// contains 콜사이트 wrapper — 원본 contains를 직접 호출(함수 무손상) 후,
/// blocklist에 있으면 true(피어리스와 동일 처리)로 승격. 그 외엔 원본 그대로.
extern "C" fn contains_wrapper(needle: usize, list_ptr: usize, list_len: usize) -> u8 {
    let f = CONTAINS_FN.load(Ordering::Relaxed);
    if f == 0 {
        return 0;
    }
    let orig_fn: extern "C" fn(usize, usize, usize) -> u8 = unsafe { core::mem::transmute(f) };
    let orig = orig_fn(needle, list_ptr, list_len);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        CNT_UI_QUERY.fetch_add(1, Ordering::Relaxed);
        let name = unsafe { read_needle(needle) }?;
        if orig != 0 {
            // 게임 자체 피어리스 회색 — fail-open 풀 계산 재료로 누적
            plock(&FEARLESS_SEEN)
                .get_or_insert_with(HashSet::new)
                .insert(name);
            return None;
        }
        let g = plock(&BLOCKLIST);
        if g.as_ref().is_some_and(|b| b.contains(&name)) {
            CNT_UI_BLOCK.fetch_add(1, Ordering::Relaxed);
            return Some(());
        }
        None
    }));
    match r {
        Ok(Some(())) => 1,
        _ => orig,
    }
}

/// 콜사이트(E8 rel32) ±2GB 안에 실행 가능한 12B 점프 썽크를 할당.
unsafe fn alloc_near(site: usize) -> Option<usize> {
    let mut step: usize = 0x100000; // 1MB 단위로 아래→위 탐색
    while step < 0x7000_0000 {
        for cand in [site.saturating_sub(step), site + step] {
            let a = cand & !0xFFFF; // 64KB 정렬
            if a < 0x10000 {
                continue;
            }
            let p = VirtualAlloc(a, 64, MEM_CR, RWX);
            if p != 0 {
                let d = (p as isize) - (site as isize);
                if d.unsigned_abs() < 0x7fff_0000 {
                    return Some(p);
                }
                // 너무 멀면 반납 없이 다음 후보(64B 누수 — 설치 1회라 무해)
            }
        }
        step += 0x100000;
    }
    None
}

/// slot_widget 내부 contains 콜사이트 2곳을 wrapper 썽크로 교체.
/// 사이트 검증: E8 rel32이고 타깃이 정확히 contains(0x943440)일 때만 패치.
unsafe fn install_contains_sites() -> Result<(), &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let contains_fn = base + RVA_CONTAINS;
    // contains 자체 프롤로그 검증(RVA 어긋남 방지)
    for (i, b) in PROL_CONTAINS.iter().enumerate() {
        if core::ptr::read((contains_fn + i) as *const u8) != *b {
            return Err("contains prologue mismatch");
        }
    }
    CONTAINS_FN.store(contains_fn, Ordering::SeqCst);
    for &rva in &SITES_CONTAINS {
        let site = base + rva;
        if core::ptr::read(site as *const u8) != 0xE8 {
            return Err("site not E8");
        }
        let rel = core::ptr::read_unaligned((site + 1) as *const i32) as isize;
        let tgt = (site as isize + 5 + rel) as usize;
        if tgt != contains_fn {
            return Err("site target mismatch");
        }
        let thunk = alloc_near(site).ok_or("alloc_near")?;
        let mut t: Vec<u8> = Vec::with_capacity(12);
        t.extend_from_slice(&[0x48, 0xb8]); // movabs rax, wrapper
        t.extend_from_slice(&(contains_wrapper as usize).to_le_bytes());
        t.extend_from_slice(&[0xff, 0xe0]); // jmp rax (콜사이트의 call이 만든 retaddr로 wrapper가 ret)
        core::ptr::copy_nonoverlapping(t.as_ptr(), thunk as *mut u8, t.len());
        let new_rel = (thunk as isize) - (site as isize + 5);
        let mut old: u32 = 0;
        if VirtualProtect(site + 1, 4, RWX, &mut old) == 0 {
            return Err("VirtualProtect site");
        }
        core::ptr::write_unaligned((site + 1) as *mut i32, new_rel as i32);
        VirtualProtect(site + 1, 4, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), site, 5);
    }
    Ok(())
}

// ── Hook D: slot_widget 진입 캡처 스텁 ──────────────────────────────────────
/// 스텁: (프롤로그 push 전 rsp 기준) [rsp+0x78]=씬 ptr 저장 + 카운터 증가 →
/// 원본 12B push열(또는 체인 점프) → resume(fn+12). rax/r11 = volatile 비인자라 안전.
unsafe fn install_scene_capture() -> Result<(), &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + RVA_SLOT_WIDGET;
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL_SLOT_WIDGET {
        return Err("slot_widget prologue mismatch");
    }
    let stub = VirtualAlloc(0, 96, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let cap_addr = core::ptr::addr_of_mut!(SCENE_CAP.0) as usize;
    let mut s: Vec<u8> = Vec::with_capacity(64);
    s.extend_from_slice(&[0x48, 0x8B, 0x44, 0x24, 0x78]); // mov rax,[rsp+0x78] (씬)
    s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, &SCENE_CAP
    s.extend_from_slice(&cap_addr.to_le_bytes());
    s.extend_from_slice(&[0x49, 0x89, 0x03]); // mov [r11], rax
    s.extend_from_slice(&[0x49, 0xFF, 0x43, 0x08]); // inc qword [r11+8]
    s.extend_from_slice(&cur); // 원본 12B push열 또는 외부훅 점프(체인)
    if !chained {
        s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, fn+12 (resume: mov eax+chkstk 원위치)
        s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        s.extend_from_slice(&[0x41, 0xFF, 0xE3]); // jmp r11
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    write_entry_patch(fn_addr, stub)
}

// ── Hook D': scene_step 진입 캡처 스텁 (rcx = 진짜 밴픽 씬) ──────────────────
/// 스텁: movabs r11,&SCENE_CAP; mov [r11],rcx; inc [r11+8]; 원본 12B(또는 체인); jmp fn+12.
///   rcx 는 인자(씬)지만 이 함수는 rcx 를 소비하기 전 우리가 먼저 읽어 SCENE_CAP 에 저장 →
///   원본 12B(mov rax,[rcx+..]; mov rdx,[rcx+..])가 그대로 실행돼 rcx 보존, 게임 로직 무손상.
/// r11 = volatile 비인자라 클로버 안전. ★scene_step 은 banpick_order 가 "전체 대체"할 수 있어
///   체인 분기 필수(진입부가 이미 외부훅이면 그 12B 를 담아 그쪽으로 점프 = 순차 발화).
unsafe fn install_scene_step_capture() -> Result<(), &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + RVA_SCENE_STEP;
    let mut cur = [0u8; 14];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 14);
    // 외부 훅 체인 마커(`movabs rax,tgt; jmp rax` = 12B)는 앞 12B 에 있다.
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL_SCENE_STEP {
        return Err("scene_step prologue mismatch");
    }
    let stub = VirtualAlloc(0, 96, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let cap_addr = core::ptr::addr_of_mut!(SCENE_CAP.0) as usize;
    let mut s: Vec<u8> = Vec::with_capacity(64);
    s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, &SCENE_CAP
    s.extend_from_slice(&cap_addr.to_le_bytes());
    s.extend_from_slice(&[0x49, 0x89, 0x0B]); // mov [r11], rcx  (씬 ptr)
    s.extend_from_slice(&[0x49, 0xFF, 0x43, 0x08]); // inc qword [r11+8]  (발화 스탬프)
    s.extend_from_slice(if chained { &cur[..12] } else { &cur[..] }); // 원본 14B 또는 외부훅 12B훅 점프
    if !chained {
        s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, fn+14 (resume) ★12 아님
        s.extend_from_slice(&(fn_addr + 14).to_le_bytes());
        s.extend_from_slice(&[0x41, 0xFF, 0xE3]); // jmp r11
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    write_entry_patch(fn_addr, stub)
}

// ── Hook P: 유저 픽 차단(클릭막기) — 픽 디스패처 게이트 ──────────────────────
// RE 2026-08-21: 회색화(+0x1d3)는 렌더 전용이라 클릭을 못 막음. 실제 차단은 픽 커밋 경로.
// 픽 디스패처 FUN_1424a2e20(rcx=controller, rdx=view_root, r8=name_ptr, r9=name_len)에서
// blocklist 챔프면 원본 미호출 return → 픽 커밋 드롭(클릭해도 무효). 밴 디스패처는 별도라
// 밴은 무영향. banpick_order 미후킹 함수(커밋터 0x24b6c10만 후킹)라 단독 detour 안전.
/// 픽 디스패처. 프롤로그 = push rbp/r15/r14/r12/rsi/rdi/rbx; sub rsp,0x40 (14B·rip-rel 없음).
const RVA_PICK_DISPATCH: usize = 0x212d090;
const PROL_PICK_DISPATCH: [u8; 14] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x83, 0xEC, 0x40,
];
static TRAMP_PICK: AtomicUsize = AtomicUsize::new(0);
/// Hook P 설치 결과: 0=미시도 1=OK 2=실패.
pub static INSTALL_STATE_P: AtomicUsize = AtomicUsize::new(0);
pub static CNT_PICK_SEEN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_PICK_BLOCK: AtomicUsize = AtomicUsize::new(0);
/// 디스패처가 넘겨준 밴픽 뷰 루트(rdx) — 회색화 셀 조회용으로 캡처.
pub static PICK_VIEW_ROOT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn pick_dispatch_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) {
    let tramp = TRAMP_PICK.load(Ordering::Relaxed);
    if tramp == 0 {
        return; // 설치 후에만 패치됨 — 방어
    }
    CNT_PICK_SEEN.fetch_add(1, Ordering::Relaxed);
    if (0x10000..1usize << 48).contains(&rdx) {
        PICK_VIEW_ROOT.store(rdx, Ordering::Relaxed);
    }
    // blocklist 판정 — detour 본문은 절대 unwind 금지.
    let blocked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let cfg = config::get();
        if !cfg.enabled || !cfg.user_pick_block {
            return false;
        }
        // r8=name_ptr, r9=name_len (raw 챔프 내부이름 슬라이스).
        if !(0x10000..1usize << 48).contains(&r8) || r9 == 0 || r9 > 64 {
            if cfg.debug {
                config::dlog(&format!("pickgate: BADNAME r8=0x{r8:x} r9={r9}"));
            }
            return false;
        }
        let bytes = unsafe { core::slice::from_raw_parts(r8 as *const u8, r9) };
        let Ok(s) = core::str::from_utf8(bytes) else {
            if cfg.debug {
                config::dlog("pickgate: name utf8 실패");
            }
            return false;
        };
        let name = s.to_ascii_lowercase();
        // 팀 구분 진단: 씬의 T1/T2 픽 길이(이 픽 직전 상태) + controller rcx.
        let (t1, t2) = {
            let (scene, _) = scene_cap();
            if scene >= 0x10000 {
                unsafe {
                    (
                        read_scene_vec(scene, O_PICK1).map(|v| v.len() as i64).unwrap_or(-1),
                        read_scene_vec(scene, O_PICK2).map(|v| v.len() as i64).unwrap_or(-1),
                    )
                }
            } else {
                (-1, -1)
            }
        };
        let g = plock(&BLOCKLIST);
        let blsz = g.as_ref().map(|b| b.len()).unwrap_or(usize::MAX); // MAX=None
        let hit = g.as_ref().is_some_and(|b| b.contains(&name));
        if cfg.debug {
            config::dlog(&format!(
                "pickgate: name='{name}' rcx=0x{rcx:x} t1={t1} t2={t2} blsz={blsz} blocked={hit}"
            ));
        }
        hit
    }))
    .unwrap_or(false);
    if blocked {
        CNT_PICK_BLOCK.fetch_add(1, Ordering::Relaxed);
        return; // 픽 커밋 드롭 — 이 챔프는 픽 안 됨(클릭 무효). 유저는 턴 유지.
    }
    // 통과 — 원본 픽 디스패처 실행.
    let orig: extern "C" fn(usize, usize, usize, usize) = unsafe { core::mem::transmute(tramp) };
    orig(rcx, rdx, r8, r9);
}

/// Hook P 설치 (post_update 1회) — 14B 프롤로그 재배치 트램폴린 + 진입 패치.
pub fn install_once_p() {
    if INSTALL_STATE_P.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block {
        INSTALL_STATE_P.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        let base = BASE.load(Ordering::Relaxed);
        if base == 0 {
            INSTALL_STATE_P.store(2, Ordering::Relaxed);
            return;
        }
        let fn_addr = base + RVA_PICK_DISPATCH;
        let mut cur = [0u8; 14];
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 14);
        let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
        if !chained && cur != PROL_PICK_DISPATCH {
            INSTALL_STATE_P.store(2, Ordering::Relaxed);
            config::dlog("hookP 프롤로그 미스매치  // — 픽 차단 비활성");
            return;
        }
        // 트램폴린: 원본 14B(push열+sub) → jmp fn+14.
        let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
        if stub == 0 {
            INSTALL_STATE_P.store(2, Ordering::Relaxed);
            return;
        }
        let mut s: Vec<u8> = Vec::with_capacity(32);
        s.extend_from_slice(&cur); // 원본 14B (rip-rel 없음)
        if !chained {
            s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, fn+14
            s.extend_from_slice(&(fn_addr + 14).to_le_bytes());
            s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
        }
        core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
        TRAMP_PICK.store(stub, Ordering::SeqCst);
        match write_entry_patch(fn_addr, pick_dispatch_detour as usize) {
            Ok(()) => {
                INSTALL_STATE_P.store(1, Ordering::Relaxed);
                config::dlog("hookP(픽 디스패처 차단) 설치 OK");
            }
            Err(e) => {
                INSTALL_STATE_P.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookP 설치 실패: {e}"));
            }
        }
    }
}

// ── Hook COMMIT: 서버 권위 커밋 강제 거부 게이트 (매치별 독립·전 매치) ────────
// RE 2026-08-21: FUN_1410b0530(rmi rcx, acting_team rdx, champ r8)->u8(1성공/0거부).
//   반환 0 = 게임이 "무효 커밋 거부"로 안전 처리(상태 무변). champ+8=이름ptr/+0x10=len.
//   rmi=각 매치 MatchSetInfo → 매치별 독립. 모든 매치 커밋에서 호출 → 전 매치 적용.
//   ⟹ 픽이 그 팀 라인업을 infeasible하게 만들면 orig 미호출·0 반환 = 강제 차단.
//   ★banpick_order도 이 함수 후킹 → 체인. 프롤로그 push8=12B 클린(sub는 13번째).
const RVA_COMMIT: usize = 0x1056c00;
const PROL_COMMIT: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
static TRAMP_COMMIT: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_CM: AtomicUsize = AtomicUsize::new(0);
pub static CNT_CM_SEEN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_CM_BLOCK: AtomicUsize = AtomicUsize::new(0); // reject 수
pub static CNT_CM_REDIR: AtomicUsize = AtomicUsize::new(0); // redirect 수
type CommitFn = extern "C" fn(usize, usize, usize) -> u8;

// ── Hook R: recommend(0x2148ca0) available 후보 필터 (AI 결정단계 하드블록) ──────
//   RE(2026-08-21 RE\recommend-레지스터계약·score_pick-미발화): CompositeBanpickAgent::recommend.
//   rcx=agent, rdx=&available{cap@0,ptr@8,len@0x10 · u64 champ 인덱스 stride8},
//   r8=&ally픽(동구조), r9=&enemy픽. available 는 빌림만(free 안 함) → 필터 복사본 치환 안전.
//   ★SDK score_pick 훅이 유저 라이브/코치 매치엔 미발화(agent+0xf60 훅배열 빔) → 여기서 직접
//   필터해야 유저 매치도 막힘. recommend 는 코치위임·상대AI·서버AI·백그라운드 sim 공용.
const RVA_RECOMMEND: usize = 0x1b58370;
const PROL_RECOMMEND: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
static TRAMP_RECOMMEND: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_RC: AtomicUsize = AtomicUsize::new(0);
pub static CNT_RC_SEEN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_RC_FILT: AtomicUsize = AtomicUsize::new(0);
type RecommendFn = extern "C" fn(usize, usize, usize, usize) -> u64;

// ── Hook DP2: AI 결정 디스패처 FUN_142079730 (RE 2026-08-22 7차) ────────────────
//   결정당 1회 호출(얕은 문맥·상태머신 바로 아래). 출력(rcx=sret 0x58B):
//   +0x10 cap/+0x18 ptr/+0x20 len = Vec<String>: [0]=이번 턴 선택 챔프, [1..≤7]=점수순 차순위.
//   +0x40=team_id, +0x50=턴종류(0/1). String={cap@0,ptr@8,len@0x10} stride 0x18.
//   1단계 = 로그 전용(라이브 위임이 이 경로인지 + Vec[0]==커밋인지 확증). 2단계 = Vec[0] 위반 시
//   [1..]의 첫 합법 후보와 스왑(사후 교정 — 코치 점수순 보존).
//   ⚠detour 본문 = 복사·판정만(포맷/로그 금지 — post_update 드레인). 프롤로그 = 8-push(커밋 동일).
const RVA_DISPATCH: usize = 0x1a78ea0;
   // ★★0.5.7 재핀(2026-08-26): ~~0.5.6 0x2079730~~ → **0x232a950**. skel/head/마스크시그 전부 NONE(함수 대개편)이라 **콜리 지문**(구 함수가 호출하는 callee 를 재핀 후 그들을 가장 많이 호출하는 신 함수 탐색)으로 확정 — 24/26 일치(2위 16). size 5345→5589 · 프롬로그 8push 12B 동일(PROL_RECOMMEND 무수정, chkstk imm은 0x6ab8→0x6c18이나 검증 범위 밖). ⚠banpick_order AITURN 컨테이너와 **동일 함수**.
static TRAMP_DISPATCH: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_DP2: AtomicUsize = AtomicUsize::new(0);
pub static CNT_DP_SEEN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_DP_LIVE: AtomicUsize = AtomicUsize::new(0);
type DispatchFn = extern "C" fn(usize, usize, usize, usize) -> u64;

/// 디스패처 관측 엔트리(post_update 가 드레인해 로그).
pub struct DpEntry {
    pub kind: u64,
    pub team: u64,
    pub live: bool,
    pub cands: Vec<String>,
    /// out+0x00..0x58 raw u64 덤프(레이아웃 규명용).
    pub raw: [u64; 12],
}
pub static DP_RING: std::sync::Mutex<Vec<DpEntry>> = std::sync::Mutex::new(Vec::new());

/// out 의 후보 Vec<String>에서 최대 n개 이름 읽기(안전읽기).
unsafe fn read_out_cands(out: usize, n: usize) -> Vec<String> {
    let mut v = Vec::new();
    let ptr = match safe_rd_u64(out + 0x18) {
        Some(p) => p as usize,
        None => return v,
    };
    let len = match safe_rd_u64(out + 0x20) {
        Some(l) => l as usize,
        None => return v,
    };
    if len == 0 || len > 8 || !ptr_ok(ptr) {
        return v;
    }
    for k in 0..len.min(n) {
        let e = ptr + k * 0x18;
        let sp = match safe_rd_u64(e + 8) {
            Some(p) => p as usize,
            None => break,
        };
        let sl = match safe_rd_u64(e + 0x10) {
            Some(l) => l as usize,
            None => break,
        };
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            break;
        }
        let mut buf = vec![0u8; sl];
        let mut ok = true;
        for i in 0..sl {
            match safe_rd_u64(sp + i) {
                Some(b) => buf[i] = b as u8,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }
        match core::str::from_utf8(&buf) {
            Ok(s) => v.push(s.to_ascii_lowercase()),
            Err(_) => break,
        }
    }
    v
}

extern "C" fn dispatch_hook(out: usize, ctx: usize, mstate: usize, mode: usize) -> u64 {
    let tramp = TRAMP_DISPATCH.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0;
    }
    let orig: DispatchFn = unsafe { core::mem::transmute(tramp) };
    let r = orig(out, ctx, mstate, mode);
    CNT_DP_SEEN.fetch_add(1, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let live = scene_cap().0 != 0;
        if live {
            CNT_DP_LIVE.fetch_add(1, Ordering::Relaxed);
        }
        // ★인자 배치 규명: 4개 인자 값 + 반환값. 어느 것이 sret(출력)인지 판별.
        let kind = mode as u64; // r9
        let team = r; // 반환값
        let cands = read_out_cands(out, 4);
        let mut raw = [0u64; 12];
        raw[0] = out as u64;
        raw[1] = ctx as u64;
        raw[2] = mstate as u64;
        raw[3] = mode as u64;
        raw[4] = r;
        // rdx(ctx)가 sret일 가능성 → 그 앞 4개 u64
        for i in 0..4 {
            raw[5 + i] = safe_rd_u64(ctx + i * 8).unwrap_or(u64::MAX);
        }
        // r8(mstate) 앞 3개
        for i in 0..3 {
            raw[9 + i] = safe_rd_u64(mstate + i * 8).unwrap_or(u64::MAX);
        }
        // cands 빈 경우도 기록(출력 레이아웃 검증용 — raw 덤프로 실제 오프셋 규명).
        let mut g = DP_RING.lock().unwrap_or_else(|e| e.into_inner());
        if g.len() < 64 {
            g.push(DpEntry { kind, team, live, cands, raw });
        }
    }));
    r
}

/// Hook DP2 설치 (post_update — 스레드 안전 패치·재시도).
pub fn install_once_dispatch() {
    let st = INSTALL_STATE_DP2.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_DP2.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        if TRAMP_DISPATCH.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_DISPATCH, &PROL_RECOMMEND) {
                Ok((stub, _)) => TRAMP_DISPATCH.store(stub, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_DP2.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookDP2 스텁 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_DISPATCH;
        match patch_entry_thread_safe(fn_addr, dispatch_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_DP2.store(1, Ordering::Relaxed);
                config::dlog("hookDP2(결정 디스패처 관측) 설치 OK (thread-safe)");
            }
            Err("in-prologue-retry") => {}
            Err(e) => {
                INSTALL_STATE_DP2.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookDP2 설치 실패: {e}"));
            }
        }
    }
}

// ── Hook FZ: 결정 확정 초크포인트 FUN_141f338a0 (RE 2026-08-22 7차, 2순위 권고) ──────
//   디스패처·서버워커 **양쪽이 수렴**하는 최종 확정 지점. 챔프 **이름(String)** 을 sret 로 반환하고,
//   인자에 match_info(+0x38/0x50/0x68/0x88 = 밴2·픽2 **이름** Vec)·is_pick 이 있어 자족 판정 가능.
//   1단계 = 관측(반환 이름 + 인자 덤프로 계약 확정), 2단계 = 위반 시 합법 챔프 이름으로 교체.
const RVA_FINALIZE: usize = 0x201da90;
static TRAMP_FINALIZE: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_FZ: AtomicUsize = AtomicUsize::new(0);
pub static CNT_FZ_SEEN: AtomicUsize = AtomicUsize::new(0);
type FinalizeFn = extern "C" fn(
    usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
) -> usize;

/// sret String(={cap@0,ptr@8,len@0x10})에서 이름 읽기.
unsafe fn read_string_at(p: usize) -> Option<String> {
    if !ptr_ok(p) {
        return None;
    }
    let sp = safe_rd_u64(p + 8)? as usize;
    let sl = safe_rd_u64(p + 0x10)? as usize;
    if !ptr_ok(sp) || sl == 0 || sl > 64 {
        return None;
    }
    let mut buf = vec![0u8; sl];
    for i in 0..sl {
        buf[i] = safe_rd_u64(sp + i)? as u8;
    }
    core::str::from_utf8(&buf).ok().map(|s| s.to_ascii_lowercase())
}

pub struct FzEntry {
    pub name: Option<String>,
    pub args: [usize; 10],
    pub live: bool,
}
pub static FZ_RING: std::sync::Mutex<Vec<FzEntry>> = std::sync::Mutex::new(Vec::new());

#[allow(clippy::too_many_arguments)]
extern "C" fn finalize_hook(
    a0: usize, a1: usize, a2: usize, a3: usize, a4: usize,
    a5: usize, a6: usize, a7: usize, a8: usize, a9: usize,
) -> usize {
    let tramp = TRAMP_FINALIZE.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0;
    }
    let orig: FinalizeFn = unsafe { core::mem::transmute(tramp) };
    let r = orig(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
    CNT_FZ_SEEN.fetch_add(1, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let live = scene_cap().0 != 0;
        if !live {
            return; // 라이브 매치 관측만(로그 스팸 방지)
        }
        // 반환 String 은 sret(a0) 또는 반환 포인터(r) 둘 중 하나 → 둘 다 시도.
        let name = read_string_at(a0).or_else(|| read_string_at(r));
        let mut g = FZ_RING.lock().unwrap_or_else(|e| e.into_inner());
        if g.len() < 40 {
            g.push(FzEntry {
                name,
                args: [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9],
                live,
            });
        }
    }));
    r
}

/// Hook FZ 설치 (post_update — 스레드 안전 패치·재시도).
pub fn install_once_finalize() {
    let st = INSTALL_STATE_FZ.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_FZ.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        if TRAMP_FINALIZE.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_FINALIZE, &PROL_RECOMMEND) {
                Ok((stub, _)) => TRAMP_FINALIZE.store(stub, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_FZ.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookFZ 스텁 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_FINALIZE;
        match patch_entry_thread_safe(fn_addr, finalize_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_FZ.store(1, Ordering::Relaxed);
                config::dlog("hookFZ(결정 확정 초크포인트) 설치 OK (thread-safe)");
            }
            Err("in-prologue-retry") => {}
            Err(e) => {
                INSTALL_STATE_FZ.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookFZ 설치 실패: {e}"));
            }
        }
    }
}

// ★픽 recommend = with_ban_counts 변형 0x214a4f0 (RE 2026-08-21 6차: recommend 는 pick/ban ×
//   plain/wbc 4개. 0x2148ca0=plain 은 **밴 페이즈 전용**이라 픽에 안 잡혔던 것. 픽 턴 = 이 함수.
//   여기도 agent+0xf58/+0xf60 score_pick 디스패치 있음 → 같은 주입으로 코치 픽 재검토 발화).
//   10인자: rcx=agent, rdx=&available, r8=&ally픽, r9=&enemy픽, 스택 [0x20]=&ally밴 [0x28]=&enemy밴
//   [0x30]=bool [0x38]=난이도 [0x40]=턴종류 [0x48]=밴카운트. 반환 rax=선택 champ id.
const RVA_RECOMMEND_WBC: usize = 0x1b59b00;
static TRAMP_RC_WBC: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_RW: AtomicUsize = AtomicUsize::new(0);
pub static CNT_RW_SEEN: AtomicUsize = AtomicUsize::new(0);
/// 유저 밴픽 씬 활성 중 픽 recommend 호출 수(진단): 0이면 코치 픽이 이 함수를 안 씀.
pub static CNT_RW_LIVE: AtomicUsize = AtomicUsize::new(0);
type RecommendWbcFn =
    extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> u64;

extern "C" fn recommend_wbc_hook(
    agent: usize,
    available: usize,
    ally: usize,
    enemy: usize,
    ally_ban: usize,
    enemy_ban: usize,
    p7: usize,
    diff: usize,
    turn_kind: usize,
    ban_cnt: usize,
) -> u64 {
    let tramp = TRAMP_RC_WBC.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0;
    }
    let orig: RecommendWbcFn = unsafe { core::mem::transmute(tramp) };
    CNT_RW_SEEN.fetch_add(1, Ordering::Relaxed);
    // ★진단: 유저 밴픽 씬이 살아있는 동안(=유저 매치 진행중) 이 픽 함수가 불리는가.
    //   씬 캡처 ptr 유효 = 유저 밴픽 화면 활성. 이게 0이면 코치 픽은 이 함수를 안 씀.
    if scene_cap().0 != 0 {
        CNT_RW_LIVE.fetch_add(1, Ordering::Relaxed);
    }
    // 라이브/코치 에이전트(훅 Vec 빔)에 score_pick 훅 주입 — 게임 자체 디스패치가 veto 호출.
    inject_score_hook(agent);
    orig(agent, available, ally, enemy, ally_ban, enemy_ban, p7, diff, turn_kind, ban_cnt)
}

/// Hook RW 설치 (post_update — 스레드 안전 패치·재시도). 픽 recommend(wbc) 진입.
/// ⚠재시도 쿨다운(2026-08-22): in-prologue-retry 가 매 프레임 전 스레드 suspend 를 반복하면
///   로딩 중 게임이 멈춘 것처럼 됨(검은화면 간헐 재발 원인). 실패 시 120프레임 쉬고 재시도.
pub fn install_once_recommend_wbc() {
    let st = INSTALL_STATE_RW.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    // 쿨다운 중이면 skip.
    let cd = RW_COOLDOWN.load(Ordering::Relaxed);
    if cd > 0 {
        RW_COOLDOWN.store(cd - 1, Ordering::Relaxed);
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_RW.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        if TRAMP_RC_WBC.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_RECOMMEND_WBC, &PROL_RECOMMEND) {
                Ok((stub, _)) => TRAMP_RC_WBC.store(stub, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_RW.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookRW 스텁 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_RECOMMEND_WBC;
        match patch_entry_thread_safe(fn_addr, recommend_wbc_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_RW.store(1, Ordering::Relaxed);
                config::dlog("hookRW(픽 recommend wbc  // — score_pick 주입) 설치 OK (thread-safe)");
            }
            Err("in-prologue-retry") => {
                RW_COOLDOWN.store(120, Ordering::Relaxed); // ~2초 쉬고 재시도(매프레임 suspend 금지)
            }
            Err(e) => {
                INSTALL_STATE_RW.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookRW 설치 실패: {e}"));
            }
        }
    }
}
/// hookRW 설치 재시도 쿨다운(프레임).
static RW_COOLDOWN: AtomicUsize = AtomicUsize::new(0);

// ══ VEH 기반 fault-safe u64 읽기 (recommend_filter 워커스레드 raw read 세그폴트 방지) ══
//   raw ru64는 범위내 unmapped 포인터에서 AV 세그폴트(catch_unwind 못 잡음 = 크래시). ai_adjust
//   mem_safety 포팅: cpl_rd8_f의 MOV가 폴트하면 VEH가 RIP를 cpl_rd8_l(return 0)로 리다이렉트 → None.
core::arch::global_asm!(
    ".globl cpl_rd8", ".globl cpl_rd8_f", ".globl cpl_rd8_l",
    "cpl_rd8:", "cpl_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "cpl_rd8_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn cpl_rd8(addr: usize, out: *mut u64) -> u32;
    static cpl_rd8_f: u8;
    static cpl_rd8_l: u8;
}
#[repr(C)]
struct ExcRecord {
    code: u32,
    flags: u32,
    next: usize,
    addr: usize,
    np: u32,
    _pad: u32,
    params: [usize; 15],
}
#[repr(C)]
struct ExcPointers {
    rec: *mut ExcRecord,
    ctx: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize,
    alloc_base: usize,
    alloc_protect: u32,
    _p0: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    typ: u32,
    _p1: u32,
}
extern "system" {
    fn AddVectoredExceptionHandler(first: u32, h: extern "system" fn(*mut ExcPointers) -> i32) -> usize;
    fn VirtualQuery(addr: usize, buf: *mut MemBasicInfo, len: usize) -> usize;
}
static SEH_INSTALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// PAGE_GUARD 재무장(가드위반 0x80000001 소비 후 원주인 스택 안 깨지게).
unsafe fn seh_rearm_guard(rec: *mut ExcRecord) {
    if (*rec).np < 2 {
        return;
    }
    let fa = (*rec).params[1];
    if fa < 0x10000 || fa >= 0x0001_0000_0000_0000 {
        return;
    }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(fa, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 {
        return;
    }
    if mbi.state != 0x1000 {
        return;
    }
    let mut old = 0u32;
    VirtualProtect(fa & !0xfff, 0x1000, mbi.protect | 0x100, &mut old);
}

extern "system" fn seh_veh(p: *mut ExcPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() {
            return CONTINUE_SEARCH;
        }
        let rec = (*p).rec;
        if rec.is_null() {
            return CONTINUE_SEARCH;
        }
        let code = (*rec).code;
        if code != 0xC000_0005 && code != 0x8000_0001 {
            return CONTINUE_SEARCH;
        }
        let ctx = (*p).ctx as usize;
        if ctx == 0 {
            return CONTINUE_SEARCH;
        }
        let rip = *((ctx + 0xF8) as *const u64) as usize; // CONTEXT.Rip
        if rip == core::ptr::addr_of!(cpl_rd8_f) as usize {
            if code == 0x8000_0001 {
                seh_rearm_guard(rec);
            }
            *((ctx + 0xF8) as *mut u64) = core::ptr::addr_of!(cpl_rd8_l) as u64;
            return CONTINUE_EXECUTION;
        }
        CONTINUE_SEARCH
    }
}

pub fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) {
        return;
    }
    unsafe {
        AddVectoredExceptionHandler(1, seh_veh);
    }
}

/// fault-safe u64 읽기. 범위내 unmapped/guard 페이지면 None(크래시 대신).
unsafe fn safe_rd_u64(a: usize) -> Option<u64> {
    if a < 0x10000 {
        return None;
    }
    let mut o = 0u64;
    if cpl_rd8(a, &mut o) != 0 {
        Some(o)
    } else {
        None
    }
}

/// Vec 헤더(cap@0,ptr@8,len@0x10)에서 u64 인덱스 배열 읽기 — ★fault-safe(safe_rd_u64).
unsafe fn read_idx_vec(hdr: usize) -> Option<Vec<usize>> {
    if !ptr_ok(hdr) {
        return None;
    }
    let ptr = safe_rd_u64(hdr + 8)? as usize;
    let len = safe_rd_u64(hdr + 0x10)? as usize;
    if len == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(ptr) || len > 512 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    for k in 0..len {
        out.push(safe_rd_u64(ptr + k * 8)? as usize);
    }
    Some(out)
}

/// 필터 결과 = 내 사본(leak-alive). rdx 를 이걸로 교체·orig 후 mem::forget(해제 안 함=UAF 없음).
struct FilteredAvail {
    _buf: Vec<u64>,
    _hdr: Box<[usize; 3]>,
    hdr_addr: usize,
}

/// ★비파괴 필터: 게임 available 배열에 **쓰지 않고**(읽기전용 페이지면 write 세그폴트 → 실제
///  크래시 원인 강한 후보), 내 사본 버퍼를 만들어 rdx 로 준다. recommend 는 available 를 값복사만
///  하고 retain 안 함(RE 확정)이나, 만약을 위해 호출측이 mem::forget(leak)으로 수명 보장.
///  반환 (사본, 원본len, k). None=필터 불요/불가(fail-open). 모든 raw 읽기 ptr_ok 가드.
unsafe fn recommend_filter(available: usize, ally: usize) -> Option<(FilteredAvail, usize, usize)> {
    let masks = crate::masks()?;
    let mask_of = |i: usize| masks.get(i).copied().unwrap_or(config::MASK_ALL);
    let avail = read_idx_vec(available)?; // ptr_ok(available)+원소 가드 내장
    if avail.len() < 2 {
        return None;
    }
    let ally_idx = read_idx_vec(ally)?; // ptr_ok(ally) 가드 내장
    let pinned: Vec<u8> = ally_idx
        .iter()
        .map(|&i| mask_of(i))
        .filter(|&m| m != config::MASK_ALL)
        .collect();
    if pinned.is_empty() {
        return None;
    }
    let mut kept: Vec<u64> = Vec::with_capacity(avail.len());
    for &c in &avail {
        let m = mask_of(c);
        let ok = if m == config::MASK_ALL {
            true
        } else {
            let mut v = pinned.clone();
            v.push(m);
            crate::feasible(&mut v)
        };
        if ok {
            kept.push(c as u64);
        }
    }
    if kept.len() == avail.len() || kept.is_empty() {
        return None; // 변화 없음 or 전부 무효(fail-open)
    }
    let (orig_len, k) = (avail.len(), kept.len());
    let buf = kept;
    let hdr = Box::new([buf.len(), buf.as_ptr() as usize, buf.len()]);
    let hdr_addr = hdr.as_ref().as_ptr() as usize;
    Some((FilteredAvail { _buf: buf, _hdr: hdr, hdr_addr }, orig_len, k))
}

/// 이 recommend 호출이 "유저 본인 라이브 매치"인가 = 아군픽(r8)이 유저 클라 씬(scene_step 캡처)
/// 픽과 일치. 로드 중 백그라운드 리그 replay(다른 매치)는 씬과 안 맞아 false → 필터 안 함(크래시
/// 회피). 유저 매치만 true → 필터. 씬 캡처 없으면(밴픽 UI 없음) false.
unsafe fn ally_is_user_match(ally: usize) -> bool {
    let (t1, t2) = match scene_pick_names() {
        Some(v) => v,
        None => return false,
    };
    if !ptr_ok(ally) {
        return false;
    }
    let aptr = ru64(ally + 8) as usize;
    let alen = ru64(ally + 0x10) as usize;
    if alen == 0 || alen > 16 || !ptr_ok(aptr) {
        return false;
    }
    let Some(names) = crate::names() else {
        return false;
    };
    let mut ally_names: Vec<String> = Vec::with_capacity(alen);
    for k in 0..alen {
        let idx = ru64(aptr + k * 8) as usize;
        if let Some(n) = names.get(idx) {
            ally_names.push(n.to_ascii_lowercase());
        }
    }
    if ally_names.is_empty() {
        return false;
    }
    // 아군픽 전부가 씬의 한 팀에 포함 = 유저 매치.
    let ov = |t: &[String]| ally_names.iter().filter(|n| t.contains(n)).count();
    ov(&t1) == ally_names.len() || ov(&t2) == ally_names.len()
}

extern "C" fn recommend_hook(agent: usize, available: usize, ally: usize, enemy: usize) -> u64 {
    let tramp = TRAMP_RECOMMEND.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0;
    }
    let orig: RecommendFn = unsafe { core::mem::transmute(tramp) };
    CNT_RC_SEEN.fetch_add(1, Ordering::Relaxed);
    // ★★score_pick 훅 주입(2026-08-21, RE\2026-08-21_scorepick-훅주입): recommend 를 직접 필터하면
    //   워커 스택 오버플로. 대신 여기서 라이브/코치 에이전트(훅 Vec 빔)에 백그라운드에서 캡처한
    //   score_pick 훅 Vec 를 주입 → 게임 자체 디스패치가 내 veto 를 부름(스택안전·검증된 경로).
    //   진입에서 u64 몇 개 읽기/쓰기뿐이라 재귀·저스택에도 안전.
    inject_score_hook(agent);
    probe_agent_layout(agent);
    orig(agent, available, ally, enemy)
}

/// ★[2026-09-03 진단] agent 구조체에서 "Vec 다운" 3연속 워드를 실측한다.
///   훅 Vec 오프셋(구 +0xf58/+0xf60/+0xf68)이 0.5.8 에서 어긋난 것으로 의심돼,
///   실제 레이아웃을 찾기 위한 일회성 프로브. 워커 스레드 경로라 최소 부하로 짠다:
///   최초 PROBE_MAX 회만 · safe_rd_u64 만 · 발견 시 한 줄.
static PROBE_N: AtomicUsize = AtomicUsize::new(0);
const PROBE_MAX: usize = 40;

#[inline(never)]
fn probe_agent_layout(agent: usize) {
    if !config::get().debug {
        return;
    }
    if PROBE_N.fetch_add(1, Ordering::Relaxed) >= PROBE_MAX {
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !ptr_ok(agent) {
            return;
        }
        let mut hits: Vec<String> = Vec::new();
        // 구 오프셋 주변을 넉넉히 훑는다(8B 정렬).
        let mut off = 0xE00usize;
        while off <= 0x1200 {
            let p = match safe_rd_u64(agent + off) {
                Some(v) => v as usize,
                None => {
                    off += 8;
                    continue;
                }
            };
            let a = safe_rd_u64(agent + off + 8).unwrap_or(0) as usize;
            let b = safe_rd_u64(agent + off + 16).unwrap_or(0) as usize;
            // {ptr, len, cap} 또는 {ptr, cap, len} 둘 다 허용
            let vec_like = ptr_ok(p)
                && ((a >= 1 && a <= 8 && b >= a && b <= 64) || (b >= 1 && b <= 8 && a >= b && a <= 64));
            if vec_like {
                hits.push(format!("+0x{off:x}{{p=ok,{a},{b}}}"));
                if hits.len() >= 6 {
                    break;
                }
            }
            off += 8;
        }
        if !hits.is_empty() {
            config::dlog(&format!("agentprobe: {}", hits.join(" ")));
        } else {
            config::dlog("agentprobe: (Vec 다운 패턴 없음)");
        }
    }));
}

/// 훅 Vec 캡처 버퍼(엔트리 = {ArcInner_ptr, vtable} 16B × ≤8). Arc 는 registry 가 strong ref
/// 영구 보유 = 프로세스 영속(free 위험 없음). 라이브 에이전트 +0xf58 이 이걸 가리키게 함.
static MOD_HOOK_BUF: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
static MOD_HOOK_LEN: AtomicUsize = AtomicUsize::new(0);
static HOOK_CAPTURED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// 주입 발생 수(진단): 0이면 "len==0 에이전트가 recommend 에 안 옴" = 코치가 recommend 미경유.
pub static CNT_RC_INJ: AtomicUsize = AtomicUsize::new(0);
/// 진단: recommend(plain+wbc)에 온 에이전트 len==0 / len>0 관측 수.
pub static CNT_AG_LEN0: AtomicUsize = AtomicUsize::new(0);
pub static CNT_AG_LENP: AtomicUsize = AtomicUsize::new(0);

/// ★[2026-09-03] 게임이 만든 진짜 훅 엔트리 `{data=1, vtable}` 를 찾는다(**읽기 전용**).
///   5·6차에서 "모드가 만든 엔트리" 주입이 2회 크래시했으므로, 원본을 게임에서 떠 오는 쪽으로
///   방향을 바꿨다. 우리 훅은 ZST 라 data==1 이고 vtable 은 모듈 영역 주소 — 매우 특징적이다.
///   scan 은 로그만 남긴다(주입 없음).
pub fn scan_real_hook_entry(ctx: usize, my_vt: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let mut found: Vec<String> = Vec::new();
        let is_mod_addr = |v: usize| v >= 0x7ff0_0000_0000 && v < 0x8000_0000_0000;

        // 후보 영역 ①ctx 주변 ②현재 스택 위쪽
        let probe_stack = 0usize;
        let sp = &probe_stack as *const usize as usize;
        let regions: [(usize, usize, &str); 2] = [
            (ctx.saturating_sub(0x100), 0x400, "ctx"),
            (sp, 0x1800, "stack"),
        ];

        for (base, size, tag) in regions {
            let mut off = 0usize;
            while off + 16 <= size {
                let a = match safe_rd_u64(base + off) {
                    Some(v) => v as usize,
                    None => {
                        off += 8;
                        continue;
                    }
                };
                if a == 1 {
                    if let Some(b) = safe_rd_u64(base + off + 8) {
                        let b = b as usize;
                        if is_mod_addr(b) {
                            found.push(format!(
                                "{tag}+0x{off:x}{{1,0x{b:x}}}{}",
                                if b == my_vt { "=MINE" } else { "" }
                            ));
                            if found.len() >= 6 {
                                break;
                            }
                        }
                    }
                }
                off += 8;
            }
            if found.len() >= 6 {
                break;
            }
        }
        if found.is_empty() {
            config::dlog("hookscan: {1,vtable} 패턴 없음 (ctx/stack)");
        } else {
            config::dlog(&format!("hookscan: {}", found.join(" ")));
        }
    }));
}

/// ★★[2026-09-03] 훅 Vec 원본을 **모드가 직접 만든다**(백그라운드 캡처 대기 폐기).
///   4차까지 실측: 훅 R(밴)·RW(픽) 을 둘 다 살려도 `agp=0` — len>0 에이전트가 어느 경로에도
///   오지 않아 캡처 원본을 못 구했다(rc_inj=0 → veto=0 → 코치 위임 픽 무차단).
///   버퍼 엔트리는 `{ArcInner_ptr, vtable}` 16B = `Arc<dyn ModDraftScoreHook>` 의 fat pointer 다.
///   모드는 `reg.add_draft_score_hook(PosLockDraftAi)` 로 **같은 타입을 이미 등록**하므로,
///   같은 Arc 를 하나 더 만들어 (ptr, vtable) 을 그대로 쓰면 된다.
///   ⚠`mem::forget` 으로 영구 유지 — registry 가 strong ref 를 붙잡는 것과 동일한 수명 보장이라
///     게임이 이 엔트리를 언제 만져도 안전하다(free 위험 0).
pub fn seed_hook_buf_from_self() {
    if HOOK_CAPTURED.load(Ordering::Acquire) {
        return;
    }
    // ★★[2026-09-03 교정] ~~Arc::new~~ → **ZST 정적 참조**.
    //   실측: score_pick 진입의 진짜 훅 인스턴스가 `data=0x1 self=0x1` 이다
    //   (`PosLockDraftAi` 는 유닛 구조체 = ZST 라 힙 실체가 없고 더미 주소 0x1 을 쓴다).
    //   5차는 Arc::new 로 **힙 ArcInner 주소**를 첫 워드에 넣어, 게임이 그걸 self 로 넘기는
    //   순간 refcount 영역을 self 로 오인해 즉사했다. ⟹ ZST 참조의 fat pointer 를 쓴다.
    let dref: &'static dyn mod_api::ModDraftScoreHook = &crate::PosLockDraftAi;
    let pair: (u64, u64) = unsafe { core::mem::transmute(dref) };
    if pair.0 == 0 || pair.1 == 0 {
        config::dlog("seedhook: transmute 결과 0 — 시드 취소");
        return;
    }
    MOD_HOOK_BUF[0].store(pair.0, Ordering::Relaxed);
    MOD_HOOK_BUF[1].store(pair.1, Ordering::Relaxed);
    MOD_HOOK_LEN.store(1, Ordering::Relaxed);
    HOOK_CAPTURED.store(true, Ordering::Release);
    config::dlog(&format!(
        "seedhook: 자체 Arc 시드 OK ptr=0x{:x} vt=0x{:x}",
        pair.0, pair.1
    ));
}

/// recommend 진입 시: len>0(백그라운드) 에이전트에서 훅 Vec 1회 딥카피 → len==0(라이브/코치)
/// 에이전트에 그 캡처본 주입. 스택복사본 agent 라 free 없음. +0xf68(cap) 미변경.
#[inline(never)]
fn inject_score_hook(agent: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if !ptr_ok(agent) {
            return;
        }
        let len = match safe_rd_u64(agent + 0xf60) {
            Some(v) => v as usize,
            None => return,
        };
        if len == 0 {
            CNT_AG_LEN0.fetch_add(1, Ordering::Relaxed);
        } else {
            CNT_AG_LENP.fetch_add(1, Ordering::Relaxed);
        }
        if len > 0 {
            // 백그라운드 에이전트 = 훅 Vec 보유 → 최초 1회 엔트리 딥카피(값 복사, ptr 저장 X).
            if len <= 8 && !HOOK_CAPTURED.load(Ordering::Acquire) {
                let src = match safe_rd_u64(agent + 0xf58) {
                    Some(v) => v as usize,
                    None => return,
                };
                if !ptr_ok(src) {
                    return;
                }
                for i in 0..(len * 2) {
                    match safe_rd_u64(src + i * 8) {
                        Some(v) => MOD_HOOK_BUF[i].store(v, Ordering::Relaxed),
                        None => return,
                    }
                }
                MOD_HOOK_LEN.store(len, Ordering::Relaxed);
                HOOK_CAPTURED.store(true, Ordering::Release);
                CNT_RC_FILT.fetch_add(1, Ordering::Relaxed); // 캡처 완료 표시(진단)
            }
        } else if HOOK_CAPTURED.load(Ordering::Acquire) {
            // 라이브/코치 에이전트(훅 Vec 빔) → 캡처본 주입.
            let mlen = MOD_HOOK_LEN.load(Ordering::Relaxed);
            if mlen > 0 {
                let buf = core::ptr::addr_of!(MOD_HOOK_BUF) as usize;
                core::ptr::write((agent + 0xf58) as *mut u64, buf as u64);
                core::ptr::write((agent + 0xf60) as *mut u64, mlen as u64);
                // ★★[2026-09-03 7차] `+0xf68`(cap) 도 **같이** 쓴다.
                //   구 코드는 여기를 미변경으로 뒀는데, 코치 에이전트는 훅 Vec 이 비어 있어
                //   그 워드가 0 이다 ⟹ **cap=0 인데 len=1** 인 모순 Vec 이 되어 게임이
                //   순회/정리할 때 죽었던 것으로 본다(6차 크래시. 값 `{1,vtable}` 자체는
                //   스택 스캔으로 게임 것과 일치함을 확인했다 — hookscan …=MINE).
                //   ⚠필드 순서가 (ptr,len,cap)인지 (ptr,cap,len)인지 확정되지 않았지만,
                //     **둘 다 같은 값**이면 어느 순서든 일관되므로 순서 규명 없이 안전하다.
                core::ptr::write((agent + 0xf68) as *mut u64, mlen as u64);
                CNT_RC_INJ.fetch_add(1, Ordering::Relaxed); // 주입 발생 수(진단)
            }
        }
    }));
}

/// 무거운 필터 프레임을 top-level 1회만 할당(recommend_hook 은 작게 유지 = 재귀 스택 절약).
#[inline(never)]
fn rc_do_filter(
    agent: usize,
    available: usize,
    ally: usize,
    enemy: usize,
    orig: RecommendFn,
) -> u64 {
    let filt = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        recommend_filter(available, ally)
    }))
    .ok()
    .flatten();
    if let Some((fa, _ol, _k)) = filt {
        CNT_RC_FILT.fetch_add(1, Ordering::Relaxed);
        let hdr = fa.hdr_addr;
        let r = orig(agent, hdr, ally, enemy); // 내 사본 available (중첩 재귀는 IN_REC로 통과)
        std::mem::forget(fa); // leak — UAF 원천 차단, 게임 배열 무손상
        r
    } else {
        orig(agent, available, ally, enemy)
    }
}

thread_local! {
    /// recommend_hook 재귀 진입 가드 — top-level 만 필터(중첩 lookahead 는 통과 = 스택 절약).
    static IN_REC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub static MIN_STK: AtomicUsize = AtomicUsize::new(usize::MAX);

/// 현재 스레드 남은 스택(rsp − TEB.StackLimit). gs:[0x10] = x64 TEB.StackLimit(하한).
#[inline(always)]
fn stack_remaining() -> usize {
    let rsp: usize;
    let limit: usize;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack, preserves_flags));
        core::arch::asm!("mov {}, gs:[0x10]", out(reg) limit, options(nomem, nostack, preserves_flags));
    }
    rsp.saturating_sub(limit)
}

/// 라이브 밴픽 UI 활성 여부(=유저가 밴픽 화면에 있음). recommend 필터 게이트.
/// scene_step(hookD') 이 매프레임 유저 클라 밴픽 씬을 캡처 → post_update 가 stamp 변화로 판정.
pub static BANPICK_UI_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// 필터 마스터. ⚠OFF(안정) — recommend 직접 필터는 워커 스택 오버플로로 크래시(재설계 중).
const RC_FILTER_MASTER: bool = false;
/// 메인 스레드(포그라운드 경기) id — post_update 첫 호출서 학습. 0=미학습.
/// recommend 워커스레드(백그라운드 다른 경기) 호출과 구분해, 메인 호출만 필터(크래시 회피).
pub static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static BP_LAST_STAMP: AtomicU64 = AtomicU64::new(u64::MAX);
static BP_ACTIVE_CD: AtomicUsize = AtomicUsize::new(0);

/// post_update 매프레임 호출 — 씬 stamp 가 이번 프레임 증가했으면(밴픽 렌더 중) 활성.
/// 미증가 프레임이 CD 만큼 지나면 비활성. (로드/유휴 시 scene_step 미발화 → stamp 정지 → 비활성.)
pub fn update_banpick_active() {
    // post_update = 메인 스레드 → 첫 호출서 메인 tid 학습(recommend 워커 구분용).
    if MAIN_TID.load(Ordering::Relaxed) == 0 {
        MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    }
    let (_scene, stamp) = scene_cap();
    let last = BP_LAST_STAMP.swap(stamp, Ordering::Relaxed);
    if stamp != 0 && stamp != last {
        BP_ACTIVE_CD.store(60, Ordering::Relaxed); // 활성 유지 프레임(약 1초)
    } else {
        let cd = BP_ACTIVE_CD.load(Ordering::Relaxed);
        if cd > 0 {
            BP_ACTIVE_CD.store(cd - 1, Ordering::Relaxed);
        }
    }
    BANPICK_UI_ACTIVE.store(BP_ACTIVE_CD.load(Ordering::Relaxed) > 0, Ordering::Relaxed);
}

/// Hook R 설치 (post_update — 스레드 안전 패치, 프롤로그 실행중이면 다음 프레임 재시도).
/// state: 0=미완(재시도) / 1=완료 / 2=영구실패.
pub fn install_once_recommend() {
    let st = INSTALL_STATE_RC.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        // ★★[2026-09-03] `any_restricted()` 는 **세이브 로드 전엔 false** 다(포지션 설정이
        //   세이브에 딸려 온다). 구 코드는 이때 state=2(영구실패)로 못박아, 이후 세이브를
        //   불러 제한이 생겨도 훅이 영원히 안 붙었다(실사고: CM=2 → 코치 위임 픽 무차단).
        //   ⟹ 설정으로 끈 경우만 영구실패, "아직 제한 없음"은 재시도(state 0 유지).
        INSTALL_STATE_RC.store(2, Ordering::Relaxed);
        return;
    }
    if !config::any_restricted() {
        return; // ★제한이 아직 없음(세이브 로드 전) — state 0 유지 = 다음 프레임 재시도
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        // 스텁은 1회만 생성(진입 패치 전이라 프롤로그 원본 = 유효).
        if TRAMP_RECOMMEND.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_RECOMMEND, &PROL_RECOMMEND) {
                Ok((stub, _)) => TRAMP_RECOMMEND.store(stub, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_RC.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookR 스텁 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_RECOMMEND;
        match patch_entry_thread_safe(fn_addr, recommend_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_RC.store(1, Ordering::Relaxed);
                config::dlog("hookR(recommend available 필터) 설치 OK (thread-safe)");
            }
            Err("in-prologue-retry") => {
                // 워커가 프롤로그 실행 중 → 이번 프레임 보류, 다음 프레임 재시도(state 0 유지).
            }
            Err(e) => {
                INSTALL_STATE_RC.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookR 설치 실패: {e}"));
            }
        }
    }
}

/// 유저가 보고 있는 경기의 두 팀 id(클라 씬 +0x3d0/+0x3d8). 백그라운드 매치 배제용.
pub static MY_T1: AtomicU64 = AtomicU64::new(0);
pub static MY_T2: AtomicU64 = AtomicU64::new(0);
/// 직전 거부한 (rmi,total) 키 — (구 redirect 휴리스틱, 미사용).
static LAST_REJ_KEY: AtomicU64 = AtomicU64::new(u64::MAX);
/// (rmi,total,champ) 해시 → 거부 횟수. 자동펌프(코치/AI)가 같은 챔프를 매프레임
/// 재제출하면 카운트가 쌓임 → REJECT_LIMIT 도달 시 fail-open(멈춤 방지). 사람은
/// 거부되면 다른 챔프를 고르므로 특정 챔프 카운트가 안 쌓임 = 항상 REJECT(픽 불변).
static RETRY_MAP: OnceLock<std::sync::Mutex<std::collections::HashMap<u64, u32>>> = OnceLock::new();
/// 같은 무효 챔프를 이 횟수만큼 거부한 뒤엔 통과(코치/AI 멈춤 방지). 사람은 도달 어려움.
const REJECT_LIMIT: u32 = 6;
static CM_TEAM_LOGGED: AtomicU64 = AtomicU64::new(u64::MAX);

static MY_TEAMS_LOGGED: AtomicU64 = AtomicU64::new(u64::MAX);
/// scene_step이 캡처한 클라 씬에서 유저 매치 두 팀 id를 학습(post_update 호출).
pub fn note_my_teams() {
    let (scene, _) = scene_cap();
    if scene < 0x10000 {
        return;
    }
    unsafe {
        let a = ru64(scene + 0x3d0);
        let b = ru64(scene + 0x3d8);
        if a != 0 && b != 0 {
            MY_T1.store(a, Ordering::Relaxed);
            MY_T2.store(b, Ordering::Relaxed);
        }
        // 진단: 씬의 팀id 후보 오프셋들(커밋 rmi+0x140/+0x148과 대조용). 값 바뀔 때만 1회.
        if config::get().debug {
            let sig = a ^ b.rotate_left(1);
            if MY_TEAMS_LOGGED.swap(sig, Ordering::Relaxed) != sig {
                config::dlog(&format!(
                    "myteams: scene=0x{scene:x} +3d0=0x{a:x} +3d8=0x{b:x} +3c0=0x{:x} +1d0=0x{:x} +1e8=0x{:x} +3e0=0x{:x}",
                    ru64(scene + 0x3c0), ru64(scene + 0x1d0), ru64(scene + 0x1e8), ru64(scene + 0x3e0)
                ));
            }
        }
    }
}

/// 유저 매치 판정(★집합 대조 — 유일성 확보). 커밋 레코드(last)의 taken 챔프(4벡터)가
/// 전부 유저 클라 씬(scene_step)의 taken에 들어있으면 유저가 보는 그 매치.
/// total/ban/rule 카운트 대조는 유일하지 않아(백그라운드 매치 오매칭) 실패 → 챔프 내용으로.
/// 반환 (matched, rec_taken) — rec_taken 은 대체 후보 탐색에 재사용.
fn my_scene_matches(last: usize) -> (bool, std::collections::HashSet<String>) {
    let mut rec: std::collections::HashSet<String> = std::collections::HashSet::new();
    unsafe {
        for off in [0x38usize, 0x50, 0x68, 0x80] {
            for s in read_rec_names(last, off) {
                rec.insert(s);
            }
        }
    }
    if rec.is_empty() {
        return (false, rec); // 첫 픽 전(taken 없음) — 개입 안 함(첫 픽은 항상 유효)
    }
    let (sc, _) = scene_cap();
    if sc < 0x10000 {
        return (false, rec);
    }
    let mut scn: std::collections::HashSet<String> = std::collections::HashSet::new();
    unsafe {
        for off in [O_BAN1, O_BAN2, O_PICK1, O_PICK2] {
            if let Some(v) = read_scene_vec(sc, off) {
                for s in v {
                    scn.insert(s);
                }
            }
        }
    }
    // ★대칭차 ≤ 1: 레코드 taken 과 씬 taken 이 최대 1챔프만 다름(=같은 매치·타이밍 오차 1).
    //   단순 부분집합은 작은 백그라운드 매치가 큰 유저 씬에 포함돼 오매칭 → 대칭차로 배제.
    let a = rec.iter().filter(|c| !scn.contains(*c)).count();
    let b = scn.iter().filter(|c| !rec.contains(*c)).count();
    let matched = a + b <= 1;
    (matched, rec)
}

#[allow(dead_code)]
fn is_my_match(t1: u64, t2: u64) -> bool {
    let (a, b) = (MY_T1.load(Ordering::Relaxed), MY_T2.load(Ordering::Relaxed));
    if a == 0 && b == 0 {
        return false;
    }
    (t1 == a && t2 == b) || (t1 == b && t2 == a)
}

enum CommitAction {
    Pass,
    Reject,
    Redirect(String),
}

/// banpick_order seq 캐시: (rule, 팀당밴, phase열). 픽/밴 판정용(커스텀 순서 정합).
/// phase 인코딩 = banpick_order와 동일: P1=0 P2=1 B1=2 B2=3 (bit1=밴).
static BO_SEQS: OnceLock<(bool, Vec<(u8, u64, Vec<u8>)>)> = OnceLock::new();

fn parse_bo_cfg() -> (bool, Vec<(u8, u64, Vec<u8>)>) {
    // 형제 모드 폴더의 cfg 를 읽어 seq 파싱. 실패 = 빈 목록(→ vanilla 폴백).
    let path = match crate::mod_dir() {
        Some(d) => {
            // <...>\mods\tfm2_champ_pos_lock → \mods\tfm2_banpick_order\tfm2_banpick_order.cfg
            if let Some(i) = d.rfind('\\') {
                format!("{}\\tfm2_banpick_order\\tfm2_banpick_order.cfg", &d[..i])
            } else {
                return (false, Vec::new());
            }
        }
        None => return (false, Vec::new()),
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return (false, Vec::new());
    };
    let mut apply_all = false;
    let mut out: Vec<(u8, u64, Vec<u8>)> = Vec::new();
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if k == "apply_all" {
            apply_all = v != "0";
        } else if k == "enabled" && v == "0" {
            return (false, Vec::new()); // banpick_order 꺼짐 → 커스텀 순서 없음
        } else if let Some(rest) = k.strip_prefix("seq_") {
            let rule = match rest.get(..3) {
                Some("2v2") => 0u8,
                Some("3v3") => 1,
                Some("4v4") => 2,
                Some("5v5") => 3,
                _ => continue,
            };
            let mut seq = Vec::new();
            let mut ok = true;
            for t in v.split_whitespace() {
                seq.push(match t.to_ascii_uppercase().as_str() {
                    "P1" => 0u8,
                    "P2" => 1,
                    "B1" => 2,
                    "B2" => 3,
                    _ => {
                        ok = false;
                        break;
                    }
                });
            }
            if ok && !seq.is_empty() {
                let ban = seq.iter().filter(|&&p| p == 2).count() as u64; // B1 개수 = 팀당 밴
                out.push((rule, ban, seq));
            }
        }
    }
    (apply_all, out)
}

/// (rule, 팀당밴, total) → 그 액션의 phase. banpick_order 커스텀 순서 정합.
/// None = 커스텀 미적용(apply_all=0/off/미매칭) → 호출측이 vanilla 공식 사용.
fn bo_phase(rule: u8, ban: u64, total: u64) -> Option<u8> {
    let (apply_all, seqs) = BO_SEQS.get_or_init(parse_bo_cfg);
    if !apply_all {
        return None;
    }
    for (r, b, seq) in seqs.iter() {
        if *r == rule && *b == ban {
            return seq.get(total as usize).copied();
        }
    }
    None
}

/// 레코드 rec 의 Vec<String>(ptr@rec+off, len@rec+off+8, 원소 String{ptr@+8,len@+0x10} stride 0x18)
/// → 소문자 이름 벡터.
unsafe fn read_rec_names(rec: usize, off: usize) -> Vec<String> {
    let mut out = Vec::new();
    let p = ru64(rec + off) as usize;
    let n = ru64(rec + off + 8) as usize;
    if !ptr_ok(p) || n > 32 {
        return out;
    }
    for i in 0..n {
        let e = p + i * 0x18;
        let sp = ru64(e + 8) as usize;
        let sl = ru64(e + 0x10) as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            continue;
        }
        let Some(b) = safe_bytes(sp, sl) else { continue };
        if let Ok(s) = core::str::from_utf8(&b) {
            out.push(s.to_ascii_lowercase());
        }
    }
    out
}

/// 이 커밋(rmi, acting_team, champ)이 "그 매치의 acting팀 라인업을 깨는 픽"인가.
/// 반환 = Pass(원본)/Reject(거부·인간용)/Redirect(대체 챔프·AI코치용).
fn commit_decide(rmi: usize, acting_team: usize, champ: usize) -> CommitAction {
    use CommitAction::*;
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block || !config::any_restricted() {
        return Pass;
    }
    unsafe {
        if !ptr_ok(rmi) {
            return Pass;
        }
        let rlen = ru64(rmi + 0x10) as usize;
        let rptr = ru64(rmi + 8) as usize;
        if rlen == 0 || !ptr_ok(rptr) {
            return Pass;
        }
        let last = rptr.wrapping_add((rlen - 1).wrapping_mul(0x100));
        if !ptr_ok(last) {
            return Pass;
        }
        if core::ptr::read((last + 0xc0) as *const u32) != 9 {
            return Pass; // 유효 레코드 아님
        }
        let (b1, b2, p1, p2) = (
            ru64(last + 0x40),
            ru64(last + 0x58),
            ru64(last + 0x70),
            ru64(last + 0x88),
        );
        let total = b1.wrapping_add(b2).wrapping_add(p1).wrapping_add(p2);
        let ban_limit = ru64(last + 0xf0);
        let rule = ru8(last + 0xf9);
        let side = (ru8(last + 0xf8) & 1) as usize;
        // ★유저 매치만 개입(집합 대조): 레코드 taken 챔프가 전부 유저 씬 taken에 있으면 그 매치.
        let (matched, taken) = my_scene_matches(last);
        if config::get().debug {
            let sig = (total ^ (last as u64)) ^ ((matched as u64) << 40);
            if CM_TEAM_LOGGED.swap(sig, Ordering::Relaxed) != sig {
                config::dlog(&format!(
                    "cmmatch: total={total} rec_taken={} matched={matched}",
                    taken.len()
                ));
            }
        }
        if !matched {
            return Pass;
        }
        let np = ru64(champ + 8) as usize;
        let nl = ru64(champ + 0x10) as usize;
        if !ptr_ok(np) || nl == 0 || nl > 64 {
            return Pass;
        }
        let Some(nb) = safe_bytes(np, nl) else { return Pass };
        let name = match core::str::from_utf8(&nb) {
            Ok(s) => s.to_ascii_lowercase(),
            Err(_) => return Pass,
        };
        let is_pick = match bo_phase(rule, ban_limit, total) {
            Some(ph) => (ph & 2) == 0,
            None => total >= ban_limit.wrapping_mul(2),
        };
        // acting 팀의 픽 버킷: T1(+0x68)=team[side^1], T2(+0x80)=team[side].
        let ta = ru64(rmi + 0x140 + 8 * ((side ^ 1) & 1));
        let tb = ru64(rmi + 0x140 + 8 * (side & 1));
        let pick_off = if acting_team as u64 == ta {
            0x68usize
        } else if acting_team as u64 == tb {
            0x80
        } else {
            return Pass; // 팀 불명 → 개입 안 함
        };
        let picks = read_rec_names(last, pick_off);
        let cand = config::mask_of(&name);
        let cand_restricted = cand != config::MASK_ALL;
        let cur_masks: Vec<u8> = picks
            .iter()
            .map(|s| config::mask_of(s))
            .filter(|&m| m != config::MASK_ALL)
            .collect();
        let cur_ok = crate::feasible(&mut cur_masks.clone());
        let mut with = cur_masks.clone();
        if cand_restricted {
            with.push(cand);
        }
        let new_ok = crate::feasible(&mut with);
        // 개입 필요 = 픽 + 제한챔프 + 현재는 가능한데 이 챔프 추가 시 불가능(=이 챔프가 깸).
        if !(is_pick && cand_restricted && cur_ok && !new_ok) {
            return Pass;
        }
        // ★fail-open(데드락 방지): 이 팀이 앞으로 feasible 하게 채울 수 있는 픽이 하나도
        //   없으면 커밋 거부하지 않는다(Pass). score_pick 은 "전 후보 무효"면 fail-open 으로
        //   그냥 최고점(무효) 챔프를 뽑는데, 여기서 그걸 거부하면 결정(선택)↔커밋(거부)이
        //   모순돼 코치가 재시도도 못하고 멈춘다(19/20 harpy 프리즈 실사고). 유저 요구
        //   "고를 수 있는 챔피언이 없으면 제한 풀려서 아무나" 와도 일치.
        let any_feasible = crate::names()
            .map(|names| {
                names.iter().any(|n| {
                    let low = n.to_ascii_lowercase();
                    if low == name || taken.contains(&low) {
                        return false;
                    }
                    let m = config::mask_of(&low);
                    if m == config::MASK_ALL {
                        return true;
                    }
                    let mut v = cur_masks.clone();
                    v.push(m);
                    crate::feasible(&mut v)
                })
            })
            .unwrap_or(false);
        if !any_feasible {
            if cfg.debug {
                config::dlog(&format!(
                    "commitgate: name='{name}' total={total} act=0x{acting_team:x} → failopen(no-feasible)"
                ));
            }
            return Pass;
        }
        // 이 픽은 포지션 중복을 만들고, feasible한 대체도 존재한다.
        // ⚠★커밋 REJECT 비활성(2026-08-21): 코치 위임 시 커밋 거부는 코치를 영구 행(freeze)
        //   시킨다 — 코치는 거부에 재시도조차 안 함(로그 cnt=1 뒤 무반응·19/20 프리즈 실사고).
        //   그리고 커밋 시점엔 사람/코치를 구분할 수 없어 "사람만 REJECT"가 불가능.
        //   ⟹ 하드블록은 결정단계 후보필터(0x2148ca0, RE 진행중)로 이관. 여기선 로그만 남기고
        //   무조건 Pass 해서 프리즈/크래시 원천 차단. (REDIRECT=순서 desync 프리즈, REJECT=코치
        //   행, 둘 다 폐기 — 커밋 시점 개입은 안전한 fail-open 판정 외엔 하지 않는다.)
        let _ = REJECT_LIMIT;
        if cfg.debug {
            config::dlog(&format!(
                "commitgate: name='{name}' total={total} picks=[{}] act=0x{acting_team:x} → would-block(reject-disabled→pass)",
                picks.join(",")
            ));
        }
        Pass
    }
}

/// ★전 매치 최종 라인업 관측(백그라운드 포함) — 커밋 후 팀 픽이 5개 차면 1회 기록.
///   "veto 가 실제로 중복을 막는가"를 결과로 검증(발화 카운터만으론 근거 부족).
pub struct FinalLineup {
    pub picks: Vec<String>,
    pub dup: bool,
}
pub static FINAL_RING: std::sync::Mutex<Vec<FinalLineup>> = std::sync::Mutex::new(Vec::new());
static FINAL_SEEN: std::sync::Mutex<Vec<u64>> = std::sync::Mutex::new(Vec::new());
/// 관측 총량 상한(커밋 훅 부하 차단 — 락 잡기 전에 값싼 체크).
pub static FINAL_N: AtomicUsize = AtomicUsize::new(0);

unsafe fn observe_final_lineup(rmi: usize) {
    if !ptr_ok(rmi) {
        return;
    }
    let rlen = safe_rd_u64(rmi + 0x10).unwrap_or(0) as usize;
    let rptr = safe_rd_u64(rmi + 8).unwrap_or(0) as usize;
    if rlen == 0 || !ptr_ok(rptr) {
        return;
    }
    let last = rptr.wrapping_add((rlen - 1).wrapping_mul(0x100));
    if !ptr_ok(last) {
        return;
    }
    for off in [0x68usize, 0x80] {
        let picks = read_rec_names(last, off);
        if picks.len() != 5 {
            continue;
        }
        // 중복 판정: 제한 마스크들로 Hall 조건 검사.
        let masks: Vec<u8> = picks
            .iter()
            .map(|n| config::mask_of(n))
            .filter(|&m| m != config::MASK_ALL)
            .collect();
        let dup = !crate::feasible(&mut masks.clone());
        let mut h: u64 = 0xcbf29ce484222325;
        for n in &picks {
            for b in n.as_bytes() {
                h = (h ^ *b as u64).wrapping_mul(0x100000001b3);
            }
        }
        {
            let mut seen = FINAL_SEEN.lock().unwrap_or_else(|e| e.into_inner());
            if seen.contains(&h) {
                continue;
            }
            if seen.len() < 512 {
                seen.push(h);
            }
        }
        FINAL_N.fetch_add(1, Ordering::Relaxed);
        let mut g = FINAL_RING.lock().unwrap_or_else(|e| e.into_inner());
        if g.len() < 64 {
            g.push(FinalLineup { picks, dup });
        }
    }
}

extern "C" fn commit_gate(rmi: usize, acting_team: usize, champ: usize) -> u8 {
    let tramp = TRAMP_COMMIT.load(Ordering::Relaxed);
    if tramp == 0 {
        return 0;
    }
    let orig: CommitFn = unsafe { core::mem::transmute(tramp) };
    CNT_CM_SEEN.fetch_add(1, Ordering::Relaxed);
    let action = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        commit_decide(rmi, acting_team, champ)
    }))
    .unwrap_or(CommitAction::Pass);
    match action {
        CommitAction::Pass => {
            let r = orig(rmi, acting_team, champ);
            // ★커밋 후 전 매치 최종 라인업 관측(중복 검증).
            //   ⚠커밋 훅은 리그 전 경기가 워커에서 호출 → 무거우면 관리화면 멈춤(실측).
            //   ⟹ 링이 찼거나 스택 여유 부족이면 즉시 skip(관측은 표본이면 충분).
            // ⚠비활성(2026-08-22): 커밋 훅에서 레코드 관측 = 관리화면 멈춤/크래시(실측 2/2).
            //   커밋 훅은 리그 전 경기가 워커에서 호출 → 어떤 추가 작업도 위험. 관측은 메인스레드
            //   (씬 읽기) 쪽으로 이관.
            let _ = FINAL_N.load(Ordering::Relaxed);
            r
        }
        CommitAction::Reject => {
            CNT_CM_BLOCK.fetch_add(1, Ordering::Relaxed);
            0 // 거부 — orig 미호출(게임이 안전 처리). 인간=재픽, AI=재시도→다음에 redirect.
        }
        CommitAction::Redirect(name) => {
            CNT_CM_REDIR.fetch_add(1, Ordering::Relaxed);
            // champ 를 대체 챔프 이름 String 으로 바꿔 커밋. String {cap@0,ptr@+8,len@+0x10}.
            // 원본이 이름 바이트를 복사하므로 fake 는 이 호출 동안만 살아있으면 안전(RE 확인).
            let bytes = name.as_bytes();
            let fake: [usize; 3] = [bytes.len(), bytes.as_ptr() as usize, bytes.len()];
            orig(rmi, acting_team, fake.as_ptr() as usize)
        }
    }
}

/// Hook COMMIT 설치 (post_update 1회) — build_tramp 로 체인(banpick_order 공존).
pub fn install_once_commit() {
    if INSTALL_STATE_CM.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block {
        // ★★[2026-09-03] `any_restricted()` 는 **세이브 로드 전엔 false** 다(포지션 설정이
        //   세이브에 딸려 온다). 구 코드는 이때 state=2(영구실패)로 못박아, 이후 세이브를
        //   불러 제한이 생겨도 훅이 영원히 안 붙었다(실사고: CM=2 → 코치 위임 픽 무차단).
        //   ⟹ 설정으로 끈 경우만 영구실패, "아직 제한 없음"은 재시도(state 0 유지).
        INSTALL_STATE_CM.store(2, Ordering::Relaxed);
        return;
    }
    if !config::any_restricted() {
        return; // ★제한이 아직 없음(세이브 로드 전) — state 0 유지 = 다음 프레임 재시도
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let r = build_tramp(RVA_COMMIT, &PROL_COMMIT).and_then(|(stub, fn_addr)| {
            TRAMP_COMMIT.store(stub, Ordering::SeqCst);
            write_entry_patch(fn_addr, commit_gate as usize)
        });
        match r {
            Ok(()) => {
                INSTALL_STATE_CM.store(1, Ordering::Relaxed);
                config::dlog("hookCOMMIT(서버 커밋 강제거부) 설치 OK");
            }
            Err(e) => {
                INSTALL_STATE_CM.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookCOMMIT 설치 실패: {e}"));
            }
        }
    }
}

// ── Hook E: 함수 진입 시 특정 레지스터(=Assets) 캡처 스텁 ──────────────────
/// 스텁: movabs r11,&ASSETS_CAP; <cap_mov = mov [r11],reg>; inc [r11+8]; 원본 12B push열; jmp fn+12.
///   r11 = volatile 비인자라 진입부 클로버 안전. 캡처 레지스터(rcx/rdx) 보존.
/// cap_mov: rcx=[0x49,0x89,0x0B], rdx=[0x49,0x89,0x13].
unsafe fn install_asset_capture(rva: usize, cap_mov: &[u8]) -> Result<(), &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + rva;
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL_ICON8 {
        return Err("prologue mismatch");
    }
    let stub = VirtualAlloc(0, 96, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let cap_addr = core::ptr::addr_of_mut!(ASSETS_CAP.0) as usize;
    let mut s: Vec<u8> = Vec::with_capacity(64);
    s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, &ASSETS_CAP
    s.extend_from_slice(&cap_addr.to_le_bytes());
    s.extend_from_slice(cap_mov); // mov [r11], <reg>
    s.extend_from_slice(&[0x49, 0xFF, 0x43, 0x08]); // inc qword [r11+8]
    s.extend_from_slice(&cur); // 원본 12B push열 또는 외부훅 점프(체인)
    if !chained {
        s.extend_from_slice(&[0x49, 0xBB]); // movabs r11, fn+12 (resume = sub rsp 원위치)
        s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        s.extend_from_slice(&[0x41, 0xFF, 0xE3]); // jmp r11
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    write_entry_patch(fn_addr, stub)
}

/// Hook E 설치 (post_update 1회) — 아이콘 표시용이라 lock 유무와 무관하게 enabled면 설치.
/// 두 지점: ①세터 0x250bc30(rcx, 게임정보 탭) ②set_entity_icon 0x2517620(rdx, 밴픽/팀상세).
/// 하나만 성공해도 캡처 가능(둘 다 같은 GAME_ASSETS write). 세터 성공 = OK 로 본다.
pub fn install_once_e() {
    if INSTALL_STATE_E.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled {
        INSTALL_STATE_E.store(2, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let setter = install_asset_capture(RVA_ICON_SETTER, &[0x49, 0x89, 0x0B]); // mov [r11],rcx
        let leaf = install_asset_capture(RVA_ENTITY_ICON, &[0x49, 0x89, 0x13]); // mov [r11],rdx
        config::dlog(&format!(
            "hookE: setter={} leaf={}",
            match &setter {
                Ok(()) => "OK".into(),
                Err(e) => format!("실패({e})"),
            },
            match &leaf {
                Ok(()) => "OK".into(),
                Err(e) => format!("실패({e})"),
            }
        ));
        INSTALL_STATE_E.store(if setter.is_ok() || leaf.is_ok() { 1 } else { 2 }, Ordering::Relaxed);
    }
}

// ── 씬 읽기 헬퍼 (0.5.5 유효 확인된 오프셋 — RE 5차 방증 4건) ───────────────
pub const O_BAN1: usize = 0x140; // ptr / len=+8
pub const O_BAN2: usize = 0x158;
pub const O_PICK1: usize = 0x170;
pub const O_PICK2: usize = 0x188;
pub const O_RULE: usize = 0xce;
pub const O_BANCNT: usize = 0x3c0;
pub const O_SELTEAM: usize = 0x3d0;
pub const O_T2TEAM: usize = 0x3d8;
pub const O_APPCTX: usize = 0x388;
pub const O_USERTEAM_IN_APP: usize = 0xe3b8; // ⚠0.5.2 채록·0.5.5 미재검증(실패 시 보수 폴백)

/// ★유저(플레이어)의 팀 id — RE 확정 경로 `*(*(scene+0x388) + 0xe3b8)`.
///   ⚠`scene+0x3d0`(O_SELTEAM)은 "내 팀"이 아니라 **T1 팀 id** 다.
///   Bo3 처럼 **세트마다 진영이 바뀌면** T1/T2 도 바뀌므로, 캐시하지 말고 매번 읽는다.
pub unsafe fn scene_user_team(scene: usize) -> Option<u64> {
    if !ptr_ok(scene) {
        return None;
    }
    let app = safe_rd_u64(scene + O_APPCTX)? as usize;
    if !ptr_ok(app) {
        return None;
    }
    let t = safe_rd_u64(app + O_USERTEAM_IN_APP)?;
    // ★★2026-08-23: 여기서 `t == 0` 을 무효로 걸렀던 것이 **모든 오판의 진짜 원인**이었다.
    //   유저 팀 id 가 실제로 **0** 인 세이브였고(SDK `player_team_id()`=0 으로 확정),
    //   그 값을 버리고 `sel_team`(=team1) 으로 폴백해서 "내 팀 = 상대"가 됐다.
    //   ⟹ 0 은 정상 팀 id 다. u64::MAX(미확보)만 무효.
    if t == u64::MAX {
        return None;
    }
    Some(t)
}

/// 내 팀이 T2(=picks_e/+0x1b0 쪽)인가. 판독 실패 시 None → 호출측이 폴백.
pub unsafe fn scene_my_is_t2(scene: usize) -> Option<bool> {
    let me = scene_user_team(scene)?;
    let t1 = safe_rd_u64(scene + O_SELTEAM)?;
    let t2 = safe_rd_u64(scene + O_T2TEAM)?;
    if me == t1 && me != t2 {
        Some(false)
    } else if me == t2 && me != t1 {
        Some(true)
    } else {
        None
    }
}

#[inline]
pub fn ptr_ok(a: usize) -> bool {
    (0x10000..1usize << 48).contains(&a)
}
pub unsafe fn ru8(a: usize) -> u8 {
    core::ptr::read(a as *const u8)
}
pub unsafe fn ru64(a: usize) -> u64 {
    core::ptr::read(a as *const u64)
}

/// 씬의 Vec<String>(원소 24B {cap,ptr@+8,len@+0x10})을 소문자 사본으로.
/// 형태가 이상하면 None(= 씬 stale/오프셋 불일치 신호 — 호출측이 전체 중단).
pub unsafe fn read_scene_vec(scene: usize, off: usize) -> Option<Vec<String>> {
    // ★★2026-08-23 크래시 대응: 여기는 **워커 스레드(score_pick)** 에서 100만 회 단위로 불린다.
    //   raw `ru64` 로 읽고 있었는데, 밴픽 씬이 해제된 뒤에도 stale 포인터로 계속 읽어
    //   재활용/언매핑된 페이지를 건드리면 그대로 세그폴트다(catch_unwind 로 못 잡음).
    //   ⟹ 전부 fault-safe 읽기로 교체(+ post_update 의 `scene_gc()` 가 포인터 자체를 무효화).
    let p = safe_rd_u64(scene + off)? as usize;
    let n = safe_rd_u64(scene + off + 8)? as usize;
    if n == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(p) || n > 16 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let e = p + i * 0x18;
        let sp = safe_rd_u64(e + 8)? as usize;
        let sl = safe_rd_u64(e + 0x10)? as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            return None;
        }
        // 마지막 방벽: 문자열 본문도 fault-safe 로 한 번 훑어 매핑을 확인한 뒤 복사.
        let mut buf = Vec::with_capacity(sl);
        let mut k = 0usize;
        while k < sl {
            let w = safe_rd_u64(sp + k)?;
            for b in 0..8 {
                if k + b < sl {
                    buf.push(((w >> (b * 8)) & 0xff) as u8);
                }
            }
            k += 8;
        }
        out.push(core::str::from_utf8(&buf).ok()?.to_ascii_lowercase());
    }
    Some(out)
}

/// ★밴픽 씬이 죽었으면 **캡처 포인터를 0으로 무효화**한다(post_update 매 프레임 호출).
///   `scene_cap()` 의 stamp 는 slot_widget 이 돌 때만 올라간다. 몇 프레임 연속 정지 =
///   밴픽 화면 종료 ⟹ 그 포인터를 쓰는 **워커 스레드 경로**(score_pick·커밋 훅)가
///   해제된 메모리를 읽지 않도록 원천 차단한다.
///   (2026-08-23 크래시: `guard:` 로그의 `sel=` 이 613→732→3→0 으로 널뛰다 죽었다.)
pub fn scene_gc() {
    let (p, s) = scene_cap();
    if p == 0 {
        SCENE_IDLE.store(0, Ordering::Relaxed);
        return;
    }
    if SCENE_LAST_STAMP.swap(s, Ordering::Relaxed) != s {
        SCENE_IDLE.store(0, Ordering::Relaxed);
        return;
    }
    if SCENE_IDLE.fetch_add(1, Ordering::Relaxed) + 1 == 5 {
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(SCENE_CAP.0) as *mut u64, 0u64);
        }
        if config::get().debug {
            config::llog("scenegc: 밴픽 씬 종료 → 캡처 포인터 무효화(stale read 차단)");
        }
    }
}
static SCENE_LAST_STAMP: AtomicU64 = AtomicU64::new(u64::MAX);
static SCENE_IDLE: AtomicU64 = AtomicU64::new(0);

/// scene_step 캡처 씬의 양팀 픽 이름(소문자). 밴픽 중에만 유효(활성 씬 라이브).
/// 형태 이상/씬 무효면 None. score_pick 의 ctx.ally_pick 불완전 보정용 —
/// ctx.ally_pick 과 가장 겹치는 팀 = ally 로 보고 그 팀의 "완전한" 픽 목록을 되돌린다.
pub fn scene_pick_names() -> Option<(Vec<String>, Vec<String>)> {
    let (scene, _stamp) = scene_cap();
    if !ptr_ok(scene) {
        return None;
    }
    unsafe {
        let p1 = read_scene_vec(scene, O_PICK1)?;
        let p2 = read_scene_vec(scene, O_PICK2)?;
        Some((p1, p2))
    }
}

// ── 설치 공통 ──────────────────────────────────────────────────────────────
unsafe fn write_entry_patch(fn_addr: usize, repl: usize) -> Result<(), &'static str> {
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&repl.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 {
        return Err("VirtualProtect");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

/// 트램폴린 스텁만 생성(진입 패치는 안 함). 진입부가 이미 외부 모드 훅
/// (48 b8 <tgt> ff e0)이면 체인: 원본 12B 대신 그 외부 점프를 트램폴린에 담아
/// 외부 스텁→원본 순으로 발화(§3 규약).
unsafe fn build_tramp(rva: usize, prologue: &[u8; 12]) -> Result<(usize, usize), &'static str> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("base 0");
    }
    let fn_addr = base + rva;
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && &cur != prologue {
        return Err("prologue mismatch");
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&cur); // 원본 12B 또는 외부훅 점프 12B
    if !chained {
        // 원본 프롤로그(전부 push — r11 미사용) 후 복귀
        s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, fn+12
        s.extend_from_slice(&(fn_addr + 12).to_le_bytes());
        s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    Ok((stub, fn_addr))
}

/// post_update 1회 호출 — MASKS 캡처 후에만 부른다(설치 전 detour 발화 없어도 안전하지만
/// 교정이 무의미). lock 이 하나도 없으면 설치 자체를 하지 않는다(원본 무손상).
pub fn install_once() {
    if INSTALL_STATE.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_assign_mask || !config::any_restricted() {
        INSTALL_STATE.store(2, Ordering::Relaxed);
        return;
    }
    unsafe {
        BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        // ★트램폴린 주소를 진입 패치 "전에" 게시 — 패치 직후 타 스레드 발화 레이스 방지.
        let r = build_tramp(RVA_POS_MASK, &PROL_POS_MASK).and_then(|(stub, fn_addr)| {
            TRAMP_POS_MASK.store(stub, Ordering::SeqCst);
            write_entry_patch(fn_addr, pos_mask_detour as usize)
        });
        match r {
            Ok(()) => {
                INSTALL_STATE.store(1, Ordering::Relaxed);
                config::dlog("hookA(pos_mask) 설치 OK");
            }
            Err(e) => {
                INSTALL_STATE.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookA(pos_mask) 설치 실패: {e}"));
            }
        }
    }
}

// ==========================================================================
// Hook SC (스왑 확정 버튼 핸들러) — RE 2026-08-22
//   FUN_141d7bc10 (game-view/src/ui/match_ui.rs) = "스왑 확정" 클릭 핸들러.
//   내부에서 controller+0x198(T1) / +0x1b0(T2) 의 order Vec 을 복사해
//   ClientPacket::SwapDone.order 로 보낸다. 여기를 잡으면:
//     (1) 배정이 우리 설정보다 나쁘면 **원본 미호출 = 클릭 무효**(숨기지 않고 클릭만 막음)
//     (2) 통과 시 **상대 팀 order 도 우리 배정으로** 맞춘 뒤 원본 호출
//   프롤로그 12B = push rbp,r15,r14,r13,r12,rsi,rdi,rbx (실측, 클린 경계).
// ==========================================================================
const RVA_SWAP_CONFIRM: usize = 0x1c18a80;  // 0.5.7 재핀 UNIQUE size1476 동일 (0.5.6=0x1d7bc10)
const PROL_SWAP_CONFIRM: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
static TRAMP_SWAP_CONFIRM: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_SC: AtomicUsize = AtomicUsize::new(0);
/// 지금 확정을 막아야 하는가(내 팀 배정이 최적보다 나쁨).
static SWAP_BLOCK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn set_swap_block(v: bool) {
    SWAP_BLOCK.store(v, Ordering::Relaxed);
}

extern "C" fn swap_confirm_detour(p1: usize, p2: usize, p3: usize, p4: usize) -> usize {
    if SWAP_BLOCK.load(Ordering::Relaxed) {
        // 클릭 무효 — 원본을 부르지 않는다(부작용 0: 패킷도 안 나감).
        crate::config::llog("swapgate: 확정 클릭 무효(배정이 최적보다 나쁨)");
        return 0;
    }
    // 통과 -> 상대 팀 order 를 우리 배정으로 맞춘 뒤 원본 실행.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::apply_opponent_swap();
    }));
    let t = TRAMP_SWAP_CONFIRM.load(Ordering::Relaxed);
    if t == 0 {
        return 0;
    }
    let orig: extern "C" fn(usize, usize, usize, usize) -> usize =
        unsafe { core::mem::transmute(t) };
    orig(p1, p2, p3, p4)
}

pub fn install_once_sc() {
    if INSTALL_STATE_SC.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || cfg.swap_force == 0 {
        INSTALL_STATE_SC.store(2, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let r = build_tramp(RVA_SWAP_CONFIRM, &PROL_SWAP_CONFIRM).and_then(|(stub, fn_addr)| {
            TRAMP_SWAP_CONFIRM.store(stub, Ordering::SeqCst);
            write_entry_patch(fn_addr, swap_confirm_detour as usize)
        });
        match r {
            Ok(()) => {
                INSTALL_STATE_SC.store(1, Ordering::Relaxed);
                config::llog("hookSC(스왑 확정) 설치 OK");
            }
            Err(e) => {
                INSTALL_STATE_SC.store(2, Ordering::Relaxed);
                config::llog(&format!("hookSC(스왑 확정) 설치 실패: {e}"));
            }
        }
    }
}

/// Hook C 설치 (post_update 1회) — 유저 픽 차단용 콜사이트 교체.
pub fn install_once_c() {
    if INSTALL_STATE_C.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block {
        // ★★[2026-09-03] `any_restricted()` 는 **세이브 로드 전엔 false** 다(포지션 설정이
        //   세이브에 딸려 온다). 구 코드는 이때 state=2(영구실패)로 못박아, 이후 세이브를
        //   불러 제한이 생겨도 훅이 영원히 안 붙었다(실사고: CM=2 → 코치 위임 픽 무차단).
        //   ⟹ 설정으로 끈 경우만 영구실패, "아직 제한 없음"은 재시도(state 0 유지).
        INSTALL_STATE_C.store(2, Ordering::Relaxed);
        return;
    }
    if !config::any_restricted() {
        return; // ★제한이 아직 없음(세이브 로드 전) — state 0 유지 = 다음 프레임 재시도
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        match install_contains_sites() {
            Ok(()) => {
                INSTALL_STATE_C.store(1, Ordering::Relaxed);
                config::dlog("hookC(contains 콜사이트 2) 설치 OK");
            }
            Err(e) => {
                INSTALL_STATE_C.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookC 설치 실패: {e}"));
            }
        }
    }
}

/// Hook D' 설치 (scene_step rcx=씬 캡처) — 매 post_update 호출(외부훅 대기 재시도).
/// banpick_order 가 scene_step 을 "전체 대체"하므로 **먼저 설치하면 그 모드가 설치 포기**한다.
/// → 진입부가 이미 외부훅(movabs+jmp)일 때 체인 설치. 부재 대비: 유예(~5s) 후 원본에 설치.
static DP_WAIT: AtomicUsize = AtomicUsize::new(0);
pub fn install_once_dp() {
    if INSTALL_STATE_D.load(Ordering::Relaxed) != 0 {
        return;
    }
    // C(회색화)가 성공해야 차단이 의미 있음. C 실패면 D'도 스킵.
    if INSTALL_STATE_C.load(Ordering::Relaxed) != 1 {
        return;
    }
    unsafe {
        let base = BASE.load(Ordering::Relaxed);
        if base == 0 {
            return;
        }
        let fn_addr = base + RVA_SCENE_STEP;
        let mut cur = [0u8; 12];
        core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
        let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
        let waited = DP_WAIT.fetch_add(1, Ordering::Relaxed);
        // 외부훅(banpick_order 등)이 아직이면 ~300프레임 대기 후 원본에 설치(부재 대비).
        if !chained && waited < 300 {
            return;
        }
        match install_scene_step_capture() {
            Ok(()) => {
                INSTALL_STATE_D.store(1, Ordering::Relaxed);
                config::dlog(&format!(
                    "hookD'(scene_step 씬 캡처) 설치 OK (chained={chained})"
                ));
            }
            Err(e) => {
                INSTALL_STATE_D.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookD' 설치 실패: {e}  // — 유저 픽 차단 비활성"));
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 술어 f36410 후킹 — 유니버설 AI 포지션 중복 차단 (2026-08-22 5축 RE 확정)
//   FUN_141f36410 = 밴픽 결정확정 3경로(f338a0/f5d0b0/1418d01b0)가 전부 수렴하는
//   후보 유효성 술어. match_info 4벡터(밴×2·픽×2 = Vec<String>)를 보고 후보가 이미
//   있으면 0(배제)/없으면 1(유효) 반환. 여기에 "포지션 이미 참" 조건을 더해 0으로
//   강등하면 병렬 후보필터가 후보풀에서 제거 → recommend 가 남은 합법후보 중 점수순
//   최고 선택(품질 유지) + 모든 경기·모든 결정(위임/수동/상대AI/백그라운드) 적용.
// 계약: rcx=name.ptr rdx=name.len r8=match_info r9=sideA.ptr(밴) [+5]=sideA.len
//       [+6]=sideB.ptr(픽) [+7]=sideB.len [+8]=is_pick / 반환 eax: 0=배제 1=유효
//   match_info 벡터: ptr@+0x38/len@+0x40, +0x50/+0x58, +0x68/+0x70, +0x80/+0x88
// ══════════════════════════════════════════════════════════════════════════
const RVA_PRED: usize = 0x2020600;  // 0.5.7 재핀 UNIQUE size537 동일 (0.5.6=0x1f36410)
const PROL_PRED: [u8; 12] =
    [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];
static TRAMP_PRED: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_PRED: AtomicUsize = AtomicUsize::new(0);
static PRED_COOLDOWN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_PRED_SEEN: AtomicU64 = AtomicU64::new(0);
pub static CNT_PRED_VETO: AtomicU64 = AtomicU64::new(0);
static DBG_PRED: AtomicU64 = AtomicU64::new(0);
/// 진단: (mi,is_pick,total) 결정키 dedup — 결정 단위 1줄 로그.
static PRED_KEYS: std::sync::Mutex<Vec<u64>> = std::sync::Mutex::new(Vec::new());

thread_local! {
    /// 술어(pred_hook)가 캡처한 "현재 결정 팀의 픽 마스크(pinned)". 같은 rayon 워커의
    /// orchestrator(orch_demote)가 읽어 out 후보 감점에 사용. 술어가 점수화보다 먼저 발화.
    static TL_PINNED: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

type PredFn =
    unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> u32;

/// match_info+off 의 Vec<String>(원소 24B {cap,ptr@8,len@0x10}) → 소문자 이름들. fault-safe.
unsafe fn read_mi_vec(mi: usize, off: usize) -> Option<Vec<String>> {
    let p = safe_rd_u64(mi + off)? as usize;
    let n = safe_rd_u64(mi + off + 8)? as usize;
    if n == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(p) || n > 16 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let e = p + i * 0x18;
        let sp = safe_rd_u64(e + 8)? as usize;
        let sl = safe_rd_u64(e + 0x10)? as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            return None;
        }
        let bytes = safe_bytes(sp, sl)?;
        out.push(core::str::from_utf8(&bytes).ok()?.to_ascii_lowercase());
    }
    Some(out)
}

unsafe fn read_str_at(ptr: usize, len: usize) -> Option<String> {
    let bytes = safe_bytes(ptr, len)?;
    core::str::from_utf8(&bytes).ok().map(|s| s.to_ascii_lowercase())
}

/// (ptr,len) 직접 지정 Vec<String>(원소 24B {cap,ptr@8,len@0x10}) → 소문자. fault-safe.
unsafe fn read_str_list(ptr: usize, len: usize) -> Option<Vec<String>> {
    if len == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(ptr) || len > 16 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let e = ptr + i * 0x18;
        let sp = safe_rd_u64(e + 8)? as usize;
        let sl = safe_rd_u64(e + 0x10)? as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            return None;
        }
        let bytes = safe_bytes(sp, sl)?;
        out.push(core::str::from_utf8(&bytes).ok()?.to_ascii_lowercase());
    }
    Some(out)
}

/// 술어 detour. 원본 먼저 실행 → 유효(1)일 때만 포지션 중복 추가 검사.
unsafe extern "C" fn pred_hook(
    name_ptr: usize,
    name_len: usize,
    mi: usize,
    sa_ptr: usize,
    sa_len: usize,
    sb_ptr: usize,
    sb_len: usize,
    is_pick: usize,
) -> u32 {
    let stub = TRAMP_PRED.load(Ordering::Relaxed);
    if stub == 0 {
        return 1; // 트램폴린 없음 — 유효로 통과(무손상)
    }
    let orig: PredFn = core::mem::transmute(stub);
    let orig_ret = orig(name_ptr, name_len, mi, sa_ptr, sa_len, sb_ptr, sb_len, is_pick);
    if orig_ret == 0 {
        return 0; // 이미 배제 — 관여 안 함
    }
    CNT_PRED_SEEN.fetch_add(1, Ordering::Relaxed);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pred_extra(
            name_ptr,
            name_len,
            mi,
            sa_ptr,
            sa_len,
            sb_ptr,
            sb_len,
            (is_pick & 0xff) != 0,
        )
    }));
    match r {
        Ok(true) => {
            CNT_PRED_VETO.fetch_add(1, Ordering::Relaxed);
            0 // 포지션 중복 → 배제
        }
        _ => orig_ret, // 유지
    }
}

/// true=이 후보를 포지션 중복으로 배제해야 함. (진단 2단계: mi 그룹핑 + sideA/sideB 덤프)
#[allow(clippy::too_many_arguments)]
unsafe fn pred_extra(
    name_ptr: usize,
    name_len: usize,
    mi: usize,
    sa_ptr: usize,
    sa_len: usize,
    sb_ptr: usize,
    sb_len: usize,
    is_pick: bool,
) -> bool {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return false;
    }
    let _ = (name_ptr, name_len, sa_ptr, sa_len, sb_ptr, sb_len);
    // 밴 결정은 포지션 무관 → TL 갱신 안 함(픽만).
    if !is_pick {
        return false;
    }
    // ★현재 결정 팀 픽 = match_info v0(+0x38, Vec<String> 이름 — 실측 검증됨). 이름→우리
    //   마스크 pinned → thread-local 저장. orchestrator(같은 워커·같은 결정, 점수화 직전)가
    //   이걸 읽어 out 후보 감점. 술어는 후보필터라 orchestrator 보다 먼저 발화 → TL 최신.
    //   ⚠배제 안 함(false 반환)=hang 없음. (orchestrator agent+0xf10 이 런타임에서 픽 아님이
    //   확정돼, 데이터 확실한 match_info 경유로 전환. 2026-08-22.)
    let (Some(names), Some(masks), Some(v0)) =
        (crate::names(), crate::masks(), read_mi_vec(mi, 0x38))
    else {
        return false;
    };
    let name_to_mask = |nm: &str| -> u8 {
        names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(nm))
            .and_then(|i| masks.get(i).copied())
            .unwrap_or(config::MASK_ALL)
    };
    let pinned: Vec<u8> = v0
        .iter()
        .map(|nm| name_to_mask(nm))
        .filter(|&m| m != config::MASK_ALL)
        .collect();
    TL_PINNED.with(|t| *t.borrow_mut() = pinned.clone());
    if cfg.debug && !pinned.is_empty() {
        let n = DBG_PRED.fetch_add(1, Ordering::Relaxed);
        if n < 60 {
            config::dlog(&format!(
                "pred_cap#{n}: v0=[{}] pinned={:?}",
                v0.join(","),
                pinned.iter().map(|m| format!("{m:05b}")).collect::<Vec<_>>()
            ));
        }
    }
    false // 관찰 전용(배제 안 함) — TL 저장만
}

/// 술어 훅 설치 (post_update — 스레드 안전 패치·쿨다운 재시도).
pub fn install_once_pred() {
    let st = INSTALL_STATE_PRED.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_PRED.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    let cd = PRED_COOLDOWN.load(Ordering::Relaxed);
    if cd > 0 {
        PRED_COOLDOWN.store(cd - 1, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        if TRAMP_PRED.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_PRED, &PROL_PRED) {
                Ok((s, _)) => TRAMP_PRED.store(s, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_PRED.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookP(술어 f36410) 트램폴린 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_PRED;
        match patch_entry_thread_safe(fn_addr, pred_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_PRED.store(1, Ordering::Relaxed);
                config::dlog("hookP(술어 f36410 포지션중복) 설치 OK");
            }
            Err("in-prologue-retry") => {
                PRED_COOLDOWN.store(120, Ordering::Relaxed);
            }
            Err(e) => {
                INSTALL_STATE_PRED.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookP 설치 실패: {e}"));
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 술어 orchestrator FUN_1420f1f60 후킹 — AI 포지션중복 감점 (2026-08-22 확정)
//   recommend 스코어 orchestrator. RCX=out({ptr@0,cap@8,len@0x10}, 원소 stride 0x10=
//   {champ_id u64@0, score f32@8}), RDX=agent. agent+0xf10=픽 champ_id 배열 ptr,
//   +0xf18=개수. orig 실행 → out 완성 → 현재팀 픽(agent+0xf10)이 이미 찬 라인의 후보
//   score 를 -1e9 감점(제거 아님=hang 없음). 라인 판정 = 우리 state.txt 마스크만.
//   비재귀(리프 헬퍼만 호출) → 스택오버플로 없음. 반환후 콜러 가산후처리는 -1e9가 흡수.
// ══════════════════════════════════════════════════════════════════════════
const RVA_ORCH: usize = 0x1a069a0;  // 0.5.7 재핀 HEAD_UNIQUE size3215 동일 (0.5.6=0x20f1f60)
const PROL_ORCH: [u8; 12] =
    [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
static TRAMP_ORCH: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_ORCH: AtomicUsize = AtomicUsize::new(0);
static ORCH_COOLDOWN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_ORCH_SEEN: AtomicU64 = AtomicU64::new(0);
pub static CNT_ORCH_DEMOTE: AtomicU64 = AtomicU64::new(0);
static DBG_ORCH: AtomicU64 = AtomicU64::new(0);
static DBG_ORCH2: AtomicU64 = AtomicU64::new(0);
static DBG_ORCHPRE: AtomicU64 = AtomicU64::new(0);

type OrchFn = unsafe extern "C" fn(
    usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
) -> usize;

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn orch_hook(
    out: usize,
    agent: usize,
    p3: usize,
    p4: usize,
    p5: usize,
    p6: usize,
    p7: usize,
    p8: usize,
    p9: usize,
    p10: usize,
    p11: usize,
    p12: usize,
) -> usize {
    // ★세 인자(rdx/r8/r9) 프로브: 어느 것이 "픽 배열(f10=ptr, f18=개수)"을 가진 진짜 agent인지.
    //   (rdx 실측 f10=작은정수·활성가드 불일치 → rdx≠agent 가능성 → r8/r9 확인.)
    {
        let cfg = config::get();
        if cfg.enabled && cfg.debug {
            let n = DBG_ORCHPRE.fetch_add(1, Ordering::Relaxed);
            if n < 20 {
                let nm = |i: usize| {
                    crate::names().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?")
                };
                let probe = |base: usize| -> String {
                    if !ptr_ok(base) {
                        return "badbase".into();
                    }
                    let f10 = safe_rd_u64(base + 0xf10);
                    let f18 = safe_rd_u64(base + 0xf18);
                    match f10 {
                        Some(p) if ptr_ok(p as usize) => {
                            let cnt = f18.unwrap_or(0).min(6) as usize;
                            let pk = (0..cnt)
                                .map(|i| {
                                    safe_rd_u64(p as usize + i * 8)
                                        .map(|v| nm(v as usize).to_string())
                                        .unwrap_or_else(|| "?".into())
                                })
                                .collect::<Vec<_>>()
                                .join(",");
                            format!("f18={:?} pk=[{pk}]", f18)
                        }
                        _ => format!("f10={:?} f18={:?}", f10, f18),
                    }
                };
                config::dlog(&format!(
                    "orchpre#{n}: rdx{{{}}} r8{{{}}} r9{{{}}}",
                    probe(agent),
                    probe(p3),
                    probe(p4),
                ));
            }
        }
    }
    let stub = TRAMP_ORCH.load(Ordering::Relaxed);
    let ret = if stub != 0 {
        let orig: OrchFn = core::mem::transmute(stub);
        orig(out, agent, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
    } else {
        0
    };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        orch_demote(out, agent);
    }));
    ret
}

/// out 후보점수 배열에서 현재팀(agent+0xf10) 픽이 이미 찬 라인의 후보 score 를 감점.
unsafe fn orch_demote(out: usize, agent: usize) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return;
    }
    let _ = agent; // agent+0xf10 은 런타임에서 픽 아님 확정 → 술어 캡처(TL_PINNED) 사용.
    let Some(masks) = crate::masks() else {
        return;
    };
    let mask_of = |i: usize| masks.get(i).copied().unwrap_or(config::MASK_ALL);
    // ★현재팀 픽 마스크 = 술어(pred_hook)가 같은 rayon 워커에 캡처한 TL_PINNED(match_info v0).
    let pinned: Vec<u8> = TL_PINNED.with(|t| t.borrow().clone());
    if pinned.is_empty() {
        return; // 술어 미캡처 or 제한 픽 없음
    }
    if !crate::feasible(&mut pinned.clone()) {
        return; // pinned 이미 불가 → 개입 포기(전부 감점 방지)
    }
    // out 후보 배열: out+0=len, out+8=ptr, 원소 {champ_id@0, score(f32)@8} stride 0x10.
    let len = match safe_rd_u64(out) {
        Some(v) => v as usize,
        None => return,
    };
    let ptr = match safe_rd_u64(out + 8) {
        Some(v) => v as usize,
        None => return,
    };
    if len == 0 || len > 256 || !ptr_ok(ptr) {
        return;
    }
    CNT_ORCH_SEEN.fetch_add(1, Ordering::Relaxed);
    let mut demoted = 0u32;
    for i in 0..len {
        let elem = ptr + i * 0x10;
        let cid = match safe_rd_u64(elem) {
            Some(v) => v as usize,
            None => continue,
        };
        let cm = mask_of(cid);
        if cm == config::MASK_ALL {
            continue; // 무제한 후보 = 감점 안 함(fail-open 겸용)
        }
        let mut pins = pinned.clone();
        pins.push(cm);
        if crate::feasible(&mut pins) {
            continue; // 라인 충돌 없음
        }
        // 라인 충돌 → score(f32 @ elem+8) 감점(제거 아님 = hang 없음).
        let saddr = elem + 8;
        if ptr_ok(saddr) {
            core::ptr::write(saddr as *mut f32, -1.0e9);
            demoted += 1;
        }
    }
    if demoted > 0 {
        CNT_ORCH_DEMOTE.fetch_add(demoted as u64, Ordering::Relaxed);
    }
    if cfg.debug {
        let n = DBG_ORCH2.fetch_add(1, Ordering::Relaxed);
        if n < 40 {
            config::dlog(&format!(
                "orch#{n}: pinned={:?} out_len={len} demoted={demoted}",
                pinned.iter().map(|m| format!("{m:05b}")).collect::<Vec<_>>()
            ));
        }
    }
}

/// orchestrator 훅 설치 (post_update — 스레드 안전 패치·쿨다운 재시도).
pub fn install_once_orch() {
    let st = INSTALL_STATE_ORCH.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_ORCH.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    let cd = ORCH_COOLDOWN.load(Ordering::Relaxed);
    if cd > 0 {
        ORCH_COOLDOWN.store(cd - 1, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        if TRAMP_ORCH.load(Ordering::Relaxed) == 0 {
            match build_tramp(RVA_ORCH, &PROL_ORCH) {
                Ok((s, _)) => TRAMP_ORCH.store(s, Ordering::SeqCst),
                Err(e) => {
                    INSTALL_STATE_ORCH.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookO(orchestrator) 트램폴린 실패: {e}"));
                    return;
                }
            }
        }
        let fn_addr = BASE.load(Ordering::Relaxed) + RVA_ORCH;
        match patch_entry_thread_safe(fn_addr, orch_hook as usize) {
            Ok(()) => {
                INSTALL_STATE_ORCH.store(1, Ordering::Relaxed);
                config::dlog("hookO(orchestrator 0x20f1f60 포지션감점) 설치 OK");
            }
            Err("in-prologue-retry") => {
                ORCH_COOLDOWN.store(120, Ordering::Relaxed);
            }
            Err(e) => {
                INSTALL_STATE_ORCH.store(2, Ordering::Relaxed);
                config::dlog(&format!("hookO 설치 실패: {e}"));
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Hook AM — 코치 최종픽 argmax 스코어러 콜사이트 후킹 (2026-08-22 확정 정답)
//   FUN_140f73340(0xf73340) = 코치 최종 picK argmax 셀렉터(v3 독립, 실측 확정).
//   각 후보를 "기존픽+후보" 조합으로 FUN_140f848f0(0xf848f0)에 채점→최고점 선택.
//   ⟹ f73340 내 pick 콜사이트(0xf73510, CALL f848f0)를 우리 wrap 으로 rel32 리다이렉트.
//   wrap: orig 채점 → 조합(champ_ptr[0..count])이 우리 마스크(state.txt)로 포지션 불가면
//   -1e30 반환 → argmax 가 그 후보 자연 회피(제거 아님=hang 없음). champ_ptr 가 조합 전체라
//   현재팀 픽 별도 추적 불요. 콜사이트 rel32 패치(진입 suspend 없음=검은화면 위험 적음).
//   f848f0 프롤로그는 15B(경계 안 맞음)라 진입후킹 불가 → 콜사이트 방식 채택.
// ══════════════════════════════════════════════════════════════════════════
/// f848f0 을 호출하는 **전체 콜사이트**(exe 스캔 실측 9곳). f73340(밴 셀렉터) 외에
/// 픽 셀렉터가 어디인지 미상이라 전부 후킹 — 감점은 "조합 판정"이라 어느 경로든 안전.
const F848_CALLSITES: [usize; 9] = [
    0x1204e80, 0x1204edb, 0x120a710, 0x120e892, 0x120e8b6, 0x120e8dc, 0x142f1b6, 0x142fc20,
    0x1431190,
]; // ★0.5.7 재핀(2026-08-27): RVA_F848F0 콜러 구/신 각 9건, owner+오프셋 9/9 일치. 0.5.6=[0xf73510..0x12b7ff0]
const RVA_F848F0: usize = 0x12164a0;  // 0.5.7 재핀 UNIQUE size617 동일 (0.5.6=0xf848f0)
static F848_ORIG: AtomicUsize = AtomicUsize::new(0);
static AM_STUB: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_AM: AtomicUsize = AtomicUsize::new(0);
pub static CNT_ARGMAX_PEN: AtomicU64 = AtomicU64::new(0);
static DBG_ARGMAX: AtomicU64 = AtomicU64::new(0);

type F848Fn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize) -> f32;

/// f848f0 콜사이트 wrap: orig 채점 → 조합이 포지션 불가면 -1e30(argmax 회피).
unsafe extern "C" fn f848f0_wrap(
    p1: usize,
    champ_ptr: usize,
    count: usize,
    p4: usize,
    s5: usize,
    s6: usize,
) -> f32 {
    let orig_addr = F848_ORIG.load(Ordering::Relaxed);
    let score = if orig_addr != 0 {
        let orig: F848Fn = core::mem::transmute(orig_addr);
        orig(p1, champ_ptr, count, p4, s5, s6)
    } else {
        0.0
    };
    let pen = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f848_penalty(champ_ptr, count)
    }));
    match pen {
        Ok(true) => {
            CNT_ARGMAX_PEN.fetch_add(1, Ordering::Relaxed);
            -1.0e30
        }
        _ => score,
    }
}

/// champ_ptr[0..count](기존픽+후보 조합)이 우리 state.txt 마스크로 포지션 배정 불가면 true(감점).
unsafe fn f848_penalty(champ_ptr: usize, count: usize) -> bool {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return false;
    }
    if !(2..=6).contains(&count) || !ptr_ok(champ_ptr) {
        return false;
    }
    let Some(masks) = crate::masks() else {
        return false;
    };
    let mut pins: Vec<u8> = Vec::with_capacity(count);
    for i in 0..count {
        let cid = match safe_rd_u64(champ_ptr + i * 8) {
            Some(v) => v as usize,
            None => return false,
        };
        let m = masks.get(cid).copied().unwrap_or(config::MASK_ALL);
        if m != config::MASK_ALL {
            pins.push(m);
        }
    }
    // ★진단: count>=3(픽 페이즈 누적 세트 추정)만 로그 — count=2 밴 호출이 상한을 삼키는 것 방지.
    if cfg.debug && count >= 3 {
        let n = DBG_ARGMAX.fetch_add(1, Ordering::Relaxed);
        if n < 150 {
            let nm = |i: usize| {
                crate::names().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?")
            };
            let names: Vec<String> = (0..count.min(8))
                .map(|i| {
                    safe_rd_u64(champ_ptr + i * 8)
                        .map(|v| nm(v as usize).to_string())
                        .unwrap_or_else(|| "?".into())
                })
                .collect();
            config::dlog(&format!("am#{n}: count={count} set=[{}]", names.join(",")));
        }
    }
    // count 별 호출수 집계(로그 상한과 무관) — 픽 페이즈 호출이 실제로 오는지 확인용.
    let slot = count.min(7);
    AM_COUNT_HIST[slot].fetch_add(1, Ordering::Relaxed);
    if pins.len() < 2 {
        return false; // 제한 픽 2개 미만 = 포지션 충돌 불가
    }
    let infeas = !crate::feasible(&mut pins);
    if infeas {
        CNT_ARGMAX_PEN.fetch_add(1, Ordering::Relaxed);
    }
    infeas
}

/// count(0..7) 별 f848f0 wrap 호출 횟수 — 픽 페이즈 도달 여부 진단.
pub static AM_COUNT_HIST: [AtomicU64; 8] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

// ══════════════════════════════════════════════════════════════════════════
// Hook CP — 커밋 producer FUN_141f16ea0 진입에서 확정 픽 교체 (2026-08-22 최종)
//   RE 2건(축D + 커밋조사)이 독립적으로 지목한 **유일 안전 개입점**.
//   계약: rcx=?, rdx=ctx, r8=?, r9=?, [rsp+0x28]=5th, [rsp+0x30]=6th=DecisionRecord(0xd18)
//     rec+0x00 tag(6=밴/픽 커밋), rec+0x10 match_id, rec+0x18 champ String{cap,ptr@+8,len@+0x10},
//     rec+0x30 acting_team_id.
//   여기서 rec+0x18 String 을 합법 대체 챔프로 바꾸면 **커밋 인자와 사후 브로드캐스트가
//   같은 이름을 쓴다** → 과거 REDIRECT freeze(권위/표시 desync) 원천 제거.
//   합법성 보장: 대체가 (그 매치 4버킷 ∪ 피어리스목록) 밖이면 커밋 반드시 통과(hang 불가).
//   ⚠REJECT 금지(재시도 없는 터미널 종료=영구 정지). 우린 교체만 하고 orig 그대로 실행.
//   String 소유권: cap=0 으로 두면 게임 drop 경로가 free skip(정적/누수 버퍼 안전).
//   실측: 커밋 1430회/세션 = 결정당 1회(lookahead 아님).
// ══════════════════════════════════════════════════════════════════════════
const RVA_CPROD: usize = 0x2000b90;
/// f16ea0 콜사이트 3곳(실행기 Q1/Q2/Q3 드레인 — RE 확정). 진입 패치 대신 여기를 리다이렉트.
const CPROD_CALLSITES: [usize; 3] = [0x1f096b3, 0x1f097a3, 0x1f09883];  // 0.5.7 재핀 — RVA_CPROD 콜러 구/신 각 3건 순서대응 (0.5.6=[0x2129c2d,0x2129d2d,0x2129e1d])
static CP_STUB: AtomicUsize = AtomicUsize::new(0);
static TRAMP_CPROD: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_CPROD: AtomicUsize = AtomicUsize::new(0);
static CPROD_COOLDOWN: AtomicUsize = AtomicUsize::new(0);
pub static CNT_CP_SEEN: AtomicU64 = AtomicU64::new(0);
pub static CNT_CP_SWAP: AtomicU64 = AtomicU64::new(0);
/// 스왑 결정(tag 7) 교체 횟수.
pub static CNT_SWAPORDER: AtomicU64 = AtomicU64::new(0);
/// tag 7 을 본 총 횟수.
pub static SO_SEEN: AtomicU64 = AtomicU64::new(0);
/// 조기 반환 사유별 카운터.
///  0=식별자없음 1=order길이이상 2=순열아님 3=rmi/스냅샷없음 4=팀매칭실패 5=버킷불일치
///  6=제한챔프0명 7=게임배정이이미최적
pub static SO_SKIP: [AtomicU64; 8] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
];
static DBG_CP: AtomicU64 = AtomicU64::new(0);
static DBG_CPF: AtomicU64 = AtomicU64::new(0);
static DBG_CPS: AtomicU64 = AtomicU64::new(0);
static DBG_CPH: AtomicU64 = AtomicU64::new(0);
/// 교체용 이름 버퍼 풀(정적 수명 — cap=0 으로 넘겨 게임이 free 안 하게).
static CP_NAMEBUF: std::sync::Mutex<Vec<&'static [u8]>> = std::sync::Mutex::new(Vec::new());

type CProdFn = unsafe extern "C" fn(
    usize, usize, usize, usize, usize, usize, usize, usize,
) -> usize;

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn cprod_hook(
    p1: usize,
    ctx: usize,
    p3: usize,
    p4: usize,
    p5: usize,
    rec: usize,
    p7: usize,
    p8: usize,
) -> usize {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ★진단 재료 2종(A 백그라운드 스왑 규명용):
        //   ① 실행기 ctx 보관 — post_update 가 전 매치 스냅샷을 훑을 수 있게.
        //   ② 결정레코드 tag 히스토그램 — 스왑 결정이 같은 큐로 오는지(=개입 가능한지).
        if ptr_ok(ctx) {
            CP_CTX.store(ctx, Ordering::Relaxed);
        }
        if let Some(t) = safe_rd_u64(rec) {
            let ti = (t as usize).min(15);
            CP_TAGS[ti].fetch_add(1, Ordering::Relaxed);
        }
        cprod_swap(ctx, rec);
        cprod_swap_order(ctx, rec);
        // ★교체가 끝난 뒤(=최종 확정 이름)에 라인업을 집계한다. 백그라운드 매치 포함 전 매치.
        lineup_note(ctx, rec);
    }));
    let stub = TRAMP_CPROD.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let orig: CProdFn = core::mem::transmute(stub);
    orig(p1, ctx, p3, p4, p5, rec, p7, p8)
}

/// 이미 기록한 (match_id, team) — 라인업 1회만 남기기 위해.
static LINEUP_SEEN: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());
/// ★커밋 훅에서 마지막으로 본 실행기 ctx — post_update 의 전 매치 스왑order 스캔용.
///   (훅 밖에서 쓰므로 stale 가능 → 읽기는 전부 safe_rd_* 경유.)
pub static CP_CTX: AtomicUsize = AtomicUsize::new(0);
/// 이미 order 를 고친 (mid, 스냅샷수, 양팀픽) 조합 — **상태당 1회만** 쓴다(워커와의 경합 최소화).
static FIXED_SEEN: Mutex<Vec<u64>> = Mutex::new(Vec::new());
/// 결정레코드 tag(rec+0x00) 히스토그램 — tag 6(밴/픽 커밋) 말고 **스왑 결정**이
/// 같은 큐로 오는지 찾기 위한 진단(`queue_ai_swap_select` 존재 = 가능성 높음).
pub static CP_TAGS: [AtomicU64; 16] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
];

/// UI 가 읽은 현재 차례 종류(0=미상 1=밴 2=픽). post_update 가 매 프레임 게시.
///   ★밴픽 순서를 바꾸는 모드가 있어도 맞는 유일한 근거(게임의 `in_turn` 표시).
pub static UI_TURN: AtomicUsize = AtomicUsize::new(0);
/// 팀당 픽 수. 이 게임의 일반 경기는 5V5 고정(GameRule::Game5V5).
///   ⚠백그라운드 매치엔 씬이 없어 포맷을 못 읽는다 → 상수. 씬이 있는 경로는 `2 + fmt` 를 쓴다.
pub const PICKS_PER_TEAM: usize = 5;

pub fn set_ui_turn(v: usize) {
    UI_TURN.store(v, Ordering::Relaxed);
}

/// ★모든 매치(유저 경기 + 백그라운드)의 **완성된 5인 라인업**을
///   `champ_pos_lock_lineups.txt` 에 한 팀당 한 줄씩 기록한다.
///   포지션이 겹치지 않는지(완전 매칭) 여기서 판정해 OK / 중복 을 함께 찍는다.
///   커밋 훅에서 **교체 후** 호출되므로 로그의 이름 = 실제 확정된 픽.
unsafe fn lineup_note(ctx: usize, rec: usize) {
    if !crate::config::get().log_lineups {
        return;
    }
    if safe_rd_u64(rec).unwrap_or(0) != 6 {
        return; // tag 6 = 밴/픽 커밋만
    }
    let Some(cur) = rec_name(rec) else {
        return;
    };
    let mid = safe_rd_u64(rec + 0x10).unwrap_or(u64::MAX);
    let acting = safe_rd_u64(rec + 0x30).unwrap_or(u64::MAX);
    let Some(rmi) = find_rmi(ctx, mid).or_else(|| find_rmi_at(ctx, 0x320, mid)) else {
        return;
    };
    let snap_ptr = safe_rd_u64(rmi + 8).unwrap_or(0) as usize;
    let snap_cnt = safe_rd_u64(rmi + 0x10).unwrap_or(0) as usize;
    // ⚠snap_cnt 상한 — `ptr_ok` 는 범위만 본다. 과도기에 쓰레기 count 를 읽으면
    //   r 주소가 엉뚱해지고(raw read 대상), 이전세트 제외 루프가 사실상 무한이 된다.
    if snap_cnt == 0 || snap_cnt > 4096 || !ptr_ok(snap_ptr) {
        return;
    }
    let r = snap_ptr + (snap_cnt - 1) * 0x100;
    let rule = safe_rd_u8(r + 0xf9).unwrap_or(0xff) as usize;
    if rule >= 4 {
        return;
    }
    let picks_n = 4 + rule * 2;
    let per_team = picks_n / 2; // fmt0=2, 1=3, 2=4, 3=5
    let ban_limit = safe_rd_u64(r + 0xf0).unwrap_or(0) as usize;
    let total = (safe_rd_u64(r + 0x40).unwrap_or(0) + safe_rd_u64(r + 0x58).unwrap_or(0) + safe_rd_u64(r + 0x70).unwrap_or(0) + safe_rd_u64(r + 0x88).unwrap_or(0)) as usize;
    // ★밴/픽 판정을 **순서 가정 없이** 한다:
    //   - 픽 목록은 스냅샷의 픽 버킷에서만 읽는다(밴은 절대 섞이지 않는다).
    //   - 스냅샷은 이번 커밋 **직전** 상태라 마지막 1건이 빠지는데,
    //     드래프트의 **마지막 행동은 항상 픽**이므로 그때만 현재 이름을 붙인다.
    //     (밴픽 순서를 바꾸는 모드도 마지막 토큰은 픽으로 강제한다.)
    let is_final = total + 1 == ban_limit * 2 + picks_n;
    let side = safe_rd_u8(r + 0xf8).unwrap_or(0xff) as usize & 1;
    let team_a = safe_rd_u64(rmi + 0x140 + ((side ^ 1) * 8)).unwrap_or(u64::MAX);
    let team_b = safe_rd_u64(rmi + 0x140 + (side * 8)).unwrap_or(u64::MAX);
    for (off, team) in [(0x60usize, team_a), (0x78usize, team_b)] {
        let Some(mut picks) = read_bucket(r + off) else {
            continue;
        };
        if is_final && team == acting && !picks.iter().any(|n| n.eq_ignore_ascii_case(&cur)) {
            picks.push(cur.clone());
        }
        if picks.len() < per_team {
            continue;
        }
        {
            let mut g = LINEUP_SEEN.lock().unwrap_or_else(|e| e.into_inner());
            if g.iter().any(|&(m, t)| m == mid && t == team) {
                continue; // 이미 기록함
            }
            if g.len() > 4096 {
                g.clear();
            }
            g.push((mid, team));
        }
        let masks: Vec<u8> = picks.iter().map(|n| crate::config::mask_of(n)).collect();
        let asg = crate::assign_positions(&masks);
        let body = match &asg {
            Some(ps) => picks
                .iter()
                .zip(ps.iter())
                .map(|(n, &p)| format!("{}({})", crate::kr_name(n), crate::POS_NAMES_KR[p]))
                .collect::<Vec<_>>()
                .join(" "),
            None => picks
                .iter()
                .zip(masks.iter())
                .map(|(n, &m)| {
                    let ok = (0..5)
                        .filter(|p| m & (1 << p) != 0)
                        .map(|p| crate::POS_NAMES_KR[p])
                        .collect::<Vec<_>>()
                        .join("/");
                    format!("{}[{}]", crate::kr_name(n), ok)
                })
                .collect::<Vec<_>>()
                .join(" "),
        };
        let verdict = match &asg {
            Some(_) => format!("OK {}/{}", picks.len(), per_team),
            None => format!("★중복 매칭 {}/{}", crate::max_match(&masks), picks.len()),
        };
        crate::config::llog(&format!("mid={mid} team={team} : {body} => {verdict}"));
    }
}

/// ★★스왑 결정(tag 7) 교체 — **워커 스레드에서, 배정이 정해지는 그 순간 1회**.
///   실측 확정(2026-08-23 `tagrec#7`): tag 7 = 스왑 결정, 팀당 1건.
///     rec+0x00 tag(=7) / rec+0x10 match_id / rec+0x18 order Vec{cap,ptr@+8,len@+0x10} / rec+0x30 팀 id
///   예) `mid=316 f30=34 +18=[2,1,3,0,4]`, `mid=316 f30=37 +18=[2,3,1,4,0]`
///   ⟹ 백그라운드를 폴링하며 스냅샷을 덮어쓸 필요가 없다(유저 제안 2026-08-23).
///      같은 스레드·같은 시점이라 경합이 원천적으로 없다.
unsafe fn cprod_swap_order(ctx: usize, rec: usize) {
    let cfg = config::get();
    if !cfg.enabled || cfg.swap_force == 0 || !config::any_restricted() {
        return;
    }
    if !ptr_ok(rec) || safe_rd_u64(rec).unwrap_or(0) != 7 {
        return;
    }
    SO_SEEN.fetch_add(1, Ordering::Relaxed);
    let mid = safe_rd_u64(rec + 0x10).unwrap_or(u64::MAX);
    let acting = safe_rd_u64(rec + 0x30).unwrap_or(u64::MAX);
    if mid == u64::MAX || acting == u64::MAX {
        SO_SKIP[0].fetch_add(1, Ordering::Relaxed);
        return;
    }
    // order Vec (rec+0x18)
    let Some(optr) = safe_rd_u64(rec + 0x20) else { return };
    let Some(olen) = safe_rd_u64(rec + 0x28) else { return };
    let (optr, olen) = (optr as usize, olen as usize);
    if olen < 2 || olen > 5 || !ptr_ok(optr) {
        SO_SKIP[1].fetch_add(1, Ordering::Relaxed);
        return;
    }
    let mut cur: Vec<u64> = Vec::with_capacity(olen);
    let mut seen = [false; 8];
    for k in 0..olen {
        let Some(v) = safe_rd_u64(optr + k * 8) else { return };
        if v as usize >= olen || seen[v as usize] {
            SO_SKIP[2].fetch_add(1, Ordering::Relaxed); // 순열 아님 = 다른 variant
            return;
        }
        seen[v as usize] = true;
        cur.push(v);
    }
    // 그 팀의 픽 목록: 스냅샷 버킷 중 team id 가 acting 인 쪽.
    let Some(rmi) = find_rmi(ctx, mid).or_else(|| find_rmi_at(ctx, 0x320, mid)) else {
        SO_SKIP[3].fetch_add(1, Ordering::Relaxed);
        return;
    };
    let snap_ptr = safe_rd_u64(rmi + 8).unwrap_or(0) as usize;
    let snap_cnt = safe_rd_u64(rmi + 0x10).unwrap_or(0) as usize;
    if snap_cnt == 0 || snap_cnt > 4096 || !ptr_ok(snap_ptr) {
        SO_SKIP[3].fetch_add(1, Ordering::Relaxed);
        return;
    }
    let r = snap_ptr + (snap_cnt - 1) * 0x100;
    let side = safe_rd_u8(r + 0xf8).unwrap_or(0xff) as usize & 1;
    let team_a = safe_rd_u64(rmi + 0x140 + ((side ^ 1) * 8)).unwrap_or(u64::MAX);
    let team_b = safe_rd_u64(rmi + 0x140 + (side * 8)).unwrap_or(u64::MAX);
    let bucket = if acting == team_a {
        0x60usize
    } else if acting == team_b {
        0x78
    } else {
        SO_SKIP[4].fetch_add(1, Ordering::Relaxed); // 팀 id 매칭 실패
        return;
    };
    let Some(picks) = read_bucket(r + bucket) else {
        SO_SKIP[5].fetch_add(1, Ordering::Relaxed);
        return;
    };
    if picks.len() != olen {
        SO_SKIP[5].fetch_add(1, Ordering::Relaxed); // 버킷 길이 불일치
        return;
    }
    let masks: Vec<u8> = picks.iter().map(|n| config::mask_of(n)).collect();
    let (want, best_n) = crate::best_order(&masks, &cur);
    let restricted = masks.iter().filter(|&&m| m != config::MASK_ALL).count();
    let cur_n = crate::order_matched(&masks, &cur);
    // 교체가 필요하면 여기서 실제로 쓴다.
    let changed = restricted > 0 && cur_n < best_n && want != cur;
    if changed {
        for (k, &w) in want.iter().enumerate() {
            let a = optr + k * 8;
            if ptr_ok(a) {
                core::ptr::write(a as *mut u64, w);
            }
        }
        CNT_SWAPORDER.fetch_add(1, Ordering::Relaxed);
    } else if restricted == 0 {
        SO_SKIP[6].fetch_add(1, Ordering::Relaxed);
    } else {
        SO_SKIP[7].fetch_add(1, Ordering::Relaxed);
    }
    // ★★최종 배정을 **그대로 출력**한다(유저 요청 2026-08-23) —
    //   이게 "게임이 실제로 쓸 배정"이다. 우리가 계산한 값이 아니라 order 의 최종 상태.
    //   `mid=… team=… : 챔프(포지션)` 라인업 줄은 **우리 계산**이라 검증 근거가 못 된다.
    let fin: &Vec<u64> = if changed { &want } else { &cur };
    let fin_n = crate::order_matched(&masks, fin);
    let body: Vec<String> = (0..fin.len())
        .map(|p| {
            let i = fin[p] as usize;
            let m = masks.get(i).copied().unwrap_or(config::MASK_ALL);
            let bad = m != config::MASK_ALL && m & (1 << p) == 0;
            format!(
                "{}{}({})",
                if bad { "★" } else { "" },
                crate::kr_name(&picks[i]),
                crate::POS_NAMES_KR[p]
            )
        })
        .collect();
    if !cfg.log_lineups {
        return;
    }
    crate::config::llog(&format!(
        "배정: mid={mid} team={acting} {} | 적합={fin_n}/{} 제한={restricted} {}",
        body.join(" "),
        best_n,
        if changed { "교체함" } else { "게임값유지" }
    ));
}

/// rec+0x18 String 을 읽어 소문자 이름으로.
unsafe fn rec_name(rec: usize) -> Option<String> {
    let ptr = safe_rd_u64(rec + 0x20)? as usize;
    let len = safe_rd_u64(rec + 0x28)? as usize;
    read_str_at(ptr, len)
}

/// 결정레코드의 확정 픽이 포지션 중복이면 합법 대체로 교체.
unsafe fn cprod_swap(ctx: usize, rec: usize) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return;
    }
    if !ptr_ok(rec) {
        return;
    }
    // tag 6 = 밴/픽 커밋 variant 만.
    if safe_rd_u64(rec).unwrap_or(0) != 6 {
        return;
    }
    CNT_CP_SEEN.fetch_add(1, Ordering::Relaxed);
    let Some(cur) = rec_name(rec) else {
        return;
    };
    // ★진단: tag6 도달 + 이름·team. 훅이 실제 확정을 보고 있는지 확인.
    if cfg.debug {
        let n = DBG_CP.fetch_add(1, Ordering::Relaxed);
        if n < 80 {
            config::dlog(&format!(
                "cp#{n}: cur={cur} team={:?} mid={:?}",
                safe_rd_u64(rec + 0x30),
                safe_rd_u64(rec + 0x10)
            ));
        }
    }
    let (Some(names), Some(masks)) = (crate::names(), crate::masks()) else {
        return;
    };
    let name_to_mask = |nm: &str| -> u8 {
        names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(nm))
            .and_then(|i| masks.get(i).copied())
            .unwrap_or(config::MASK_ALL)
    };
    // rmi 획득: ctx+0x320/+0x350 SwissTable 선형 스캔으로 match_id 일치 엔트리.
    let match_id = safe_rd_u64(rec + 0x10).unwrap_or(u64::MAX);
    let rmi_opt = find_rmi(ctx, match_id).or_else(|| find_rmi_at(ctx, 0x320, match_id));
    let Some(rmi) = rmi_opt else {
        if cfg.debug {
            let n = DBG_CPF.fetch_add(1, Ordering::Relaxed);
            if n < 10 {
                config::dlog(&format!("cpfail#{n}: find_rmi 실패 mid={match_id}"));
            }
        }
        return;
    };
    // 현재 스냅샷(마지막 레코드)의 4버킷 읽기.
    let snap_ptr = safe_rd_u64(rmi + 8).unwrap_or(0) as usize;
    // (가드는 버킷을 읽은 뒤 total 로 판정 — 아래 human-pick guard)
    let snap_cnt = safe_rd_u64(rmi + 0x10).unwrap_or(0) as usize;
    // ⚠snap_cnt 상한 — `ptr_ok` 는 범위만 본다. 과도기에 쓰레기 count 를 읽으면
    //   r 주소가 엉뚱해지고(raw read 대상), 이전세트 제외 루프가 사실상 무한이 된다.
    if snap_cnt == 0 || snap_cnt > 4096 || !ptr_ok(snap_ptr) {
        return;
    }
    let r = snap_ptr + (snap_cnt - 1) * 0x100;
    let mut used: Vec<String> = Vec::new();
    // ★★2026-08-23 피어리스 대응 — **이전 세트에서 쓴 챔피언은 다시 못 쓴다.**
    //   구버전은 현재 레코드(=이번 세트)의 4버킷만 제외했다. 그래서 Bo3 2세트 첫 픽에
    //   **1세트에 쓴 챔프를 대체로 골라 넣었고**, 그 커밋을 게임이 받을 수 없어
    //   **드래프트가 그 자리에서 멈췄다 = 일정 진행 정지**.
    //   실측 3회 재현(2개 세션 독립) — 전부 `mid=412 acting=48 -> alchemist`,
    //   그 팀 1세트 라인업에 연금술사가 있었다.
    //   ⟹ 스냅샷 배열의 **이전 레코드 전부**(밴+픽)를 제외 집합에 넣는다.
    //   ⚠과다 제외는 "합법 대체 없음 → fail-open(원본 유지)"으로 끝나 무해하지만,
    //     과소 제외는 **정지**다. 안전 방향으로 넉넉히 제외한다.
    for i in 0..snap_cnt.saturating_sub(1) {
        let pr = snap_ptr + i * 0x100;
        for off in [0x30usize, 0x48, 0x60, 0x78] {
            if let Some(v) = read_bucket(pr + off) {
                for s in &v {
                    used.push(s.clone());
                }
            }
        }
    }
    let prev_n = used.len();
    let mut t1_pick: Vec<String> = Vec::new();
    let mut t2_pick: Vec<String> = Vec::new();
    for (off, which) in [(0x30usize, 0u8), (0x48, 0), (0x60, 1), (0x78, 2)] {
        if let Some(v) = read_bucket(r + off) {
            for s in &v {
                used.push(s.clone());
            }
            match which {
                1 => t1_pick = v,
                2 => t2_pick = v,
                _ => {}
            }
        }
    }
    // ★사람 픽 가드: 유저 팀(밴픽 씬 +0x3d0)의 픽인데 AI 결정 기록이 없으면 = 수동 픽.
    //   → 절대 교체하지 않는다(회색화로 애초에 못 고르게 하는 것이 올바른 처리).
    {
        let acting_now = safe_rd_u64(rec + 0x30).unwrap_or(u64::MAX);
        // ★★2026-08-23: 여기는 **워커 스레드**다. 밴픽 씬 포인터를 raw 로 읽던 것이
        //   크래시 원인(해제된 메모리 read). 내 팀 id 는 이제 SDK `player_team_id()` 로
        //   알 수 있으므로 **씬을 아예 안 본다.**
        {
            // ★★2026-08-22 정정: `scene+0x3d0`(sel_team)은 **T1 팀 id** 지 내 팀이 아니다.
            //   Bo3 처럼 세트마다 진영이 바뀌면 2세트부터 **상대 픽을 내 픽으로 보호**해 버린다.
            //   ⟹ 유저 팀 id 의 진짜 출처(`*(*(scene+0x388)+0xe3b8)`)를 1순위로 쓴다.
            // 내 팀 id = SDK `db.player_team_id()`(관리화면에서 캡처, 세이브 내내 불변).
            //   미확보면 판정 보류(= 보호 안 함) — 씬 raw read 폴백은 폐기했다.
            let my_team = crate::player_team().unwrap_or(u64::MAX);
            // ★유저가 직접 클릭한 픽만 보호(자동픽/위임은 교정 대상).
            //   진단: 가드가 왜 안 걸렸는지 알 수 있게 재료를 항상 남긴다.
            let is_my_team = acting_now == my_team;
            let clicked = human_take(&cur); // ⚠단락평가 금지 — 항상 소비/판정
            // ★로그 축소(2026-08-23): 이 줄이 전체 로그의 84%(11333/13500)를 차지했다.
            //   판정 근거는 확보됐으니 **내 팀 커밋일 때만** 남긴다(가드가 의미 있는 경우).
            if is_my_team || cfg.debug {
                crate::config::llog(&format!(
                    "guard: cur={cur} acting={acting_now} me={my_team} my={is_my_team} clicked={clicked} ck={}",
                    INSTALL_STATE_CK.load(Ordering::Relaxed)
                ));
            }
            if is_my_team && clicked {
                if cfg.debug {
                    let n = DBG_CPH.fetch_add(1, Ordering::Relaxed);
                    if n < 20 {
                        config::dlog(&format!(
                            "cp_human#{n}: 수동픽 보호  // — {cur} 유지(team={acting_now})"
                        ));
                    }
                }
                return;
            }
        }
    }
    // ★밴 차례면 절대 교체하지 않는다.
    //   밴은 포지션을 차지하지 않으므로 라인업 판정 대상이 아니고, 교체하면 **유저가 밴한
    //   챔프가 아닌 엉뚱한 챔프가 밴된다**(2026-08-22 실사고).
    //   근거 = UI 의 `in_turn` 표시(순서를 바꾸는 모드가 있어도 맞음). 유저가 보고 있는
    //   그 매치일 때만 유효하므로, 매치 대조를 통과할 때만 적용한다.
    {
        let (is_my_match, _) = my_scene_matches(r);
        if is_my_match && UI_TURN.load(Ordering::Relaxed) == 1 {
            return;
        }
    }
    // 이번 확정이 어느 팀 픽인지: acting_team 과 rmi+0x140/+0x148 대조.
    let acting = safe_rd_u64(rec + 0x30).unwrap_or(u64::MAX);
    let side = safe_rd_u8(r + 0xf8).unwrap_or(0xff) as usize & 1;
    let team_a = safe_rd_u64(rmi + 0x140 + ((side ^ 1) * 8)).unwrap_or(u64::MAX);
    let my_picks: &Vec<String> = if acting == team_a { &t1_pick } else { &t2_pick };
    // 픽 단계가 아니면(밴) 개입 안 함 — 우리 팀 픽 목록이 곧 pinned.
    let pinned: Vec<u8> = my_picks
        .iter()
        .map(|n| name_to_mask(n))
        .filter(|&m| m != config::MASK_ALL)
        .collect();
    let cur_mask = name_to_mask(&cur);
    if cur_mask == config::MASK_ALL {
        return; // 무제한 챔프 = 충돌 불가
    }
    // ★진단: rmi 성공 후 판정 재료(내 픽·pinned·현재 마스크).
    if cfg.debug {
        let n = DBG_CPS.fetch_add(1, Ordering::Relaxed);
        if n < 40 {
            config::dlog(&format!(
                "cpst#{n}: cur={cur}(m={cur_mask:05b}) mypicks=[{}] t1=[{}] t2=[{}] side={side} acting={acting} teamA={team_a}",
                my_picks.join(","),
                t1_pick.join(","),
                t2_pick.join(",")
            ));
        }
    }
    if crate::helps(&pinned, cur_mask) {
        return; // 현재 픽이 새 라인을 채움 = 합법
    }
    // 합법 대체 탐색: used(4버킷) 밖 + 우리 마스크로 feasible.
    let used_low: Vec<String> = used.iter().map(|s| s.to_ascii_lowercase()).collect();
    // ★★자유 슬롯이 남아 있으면 **교체하지 않는다**(2026-08-23 유저 제보: 격투가를 눌렀는데
    //   바람술사가 픽됨). 어떤 포지션 풀이 말라 정배치가 불가능해지면 그 자리는 아무나 앉아도 되고,
    //   **몇 번째 픽으로 채울지도 순서와 무관**하다(스왑 배정은 픽이 다 끝난 뒤 결정).
    //   ⚠유저 UI 게이트(lib.rs)가 이미 같은 규칙으로 클릭을 허용하므로, 여기서 안 맞추면
    //     "고를 수는 있는데 확정되면 다른 챔프로 바뀌는" 최악의 불일치가 된다.
    {
        let pinned_all: Vec<u8> = my_picks.iter().map(|n| name_to_mask(n)).collect();
        let pool: Vec<u8> = names
            .iter()
            .enumerate()
            .filter(|(_, n)| {
                let l = n.to_ascii_lowercase();
                !used_low.iter().any(|u| *u == l)
            })
            .map(|(i, _)| masks.get(i).copied().unwrap_or(config::MASK_ALL))
            .collect();
        if crate::free_left(&pinned_all, &pool, PICKS_PER_TEAM) > 0 {
            return;
        }
    }
    let mut chosen: Option<&str> = None;
    // ★1순위: 그 매치의 "게임이 매긴 점수순 차순위"(DQ 캐시) — 품질 보존.
    let ru = runnerup_for(match_id);
    let mut ru_pick: Option<String> = None;
    for cand in ru.iter() {
        let low = cand.to_ascii_lowercase();
        if used_low.iter().any(|u| *u == low) {
            continue;
        }
        let m = names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(cand))
            .and_then(|i| masks.get(i).copied())
            .unwrap_or(config::MASK_ALL);
        if crate::helps(&pinned, m) {
            ru_pick = Some(cand.clone());
            break;
        }
    }
    // 2순위(폴백): NAMES 순회.
    for (i, nm) in names.iter().enumerate() {
        if ru_pick.is_some() {
            break;
        }
        let low = nm.to_ascii_lowercase();
        if used_low.iter().any(|u| *u == low) {
            continue; // 이미 밴/픽됨 → 커밋 거부 대상
        }
        let m = masks.get(i).copied().unwrap_or(config::MASK_ALL);
        if m == config::MASK_ALL {
            continue; // 무제한은 라인 판정 불가 → 보수적으로 스킵
        }
        if crate::helps(&pinned, m) {
            chosen = Some(nm.as_str());
            break;
        }
    }
    let ru_used = ru_pick.is_some();
    let newname: &str = match ru_pick.as_deref().or(chosen) {
        Some(v) => v,
        None => return, // 합법 대체 없음 → fail-open(원본 유지)
    };
    if cfg.log_lineups {
    crate::config::llog(&format!(
        "swap: mid={match_id} acting={acting} cur={cur}(m={cur_mask:05b}) -> {newname} | mypicks=[{}] pinned={} 이전세트제외={prev_n}",
        my_picks.join(","),
        pinned.len()
    ));
    }
    // rec+0x18 String 교체: cap=0(게임이 free 안 함) + 정적 버퍼.
    let buf: &'static [u8] = {
        let leaked: &'static [u8] = Box::leak(newname.as_bytes().to_vec().into_boxed_slice());
        let mut g = CP_NAMEBUF.lock().unwrap_or_else(|e| e.into_inner());
        if g.len() < 4096 {
            g.push(leaked);
        }
        leaked
    };
    core::ptr::write((rec + 0x18) as *mut u64, 0); // cap = 0 → free skip
    core::ptr::write((rec + 0x20) as *mut u64, buf.as_ptr() as u64);
    core::ptr::write((rec + 0x28) as *mut u64, buf.len() as u64);
    CNT_CP_SWAP.fetch_add(1, Ordering::Relaxed);
    if cfg.debug {
        let n = DBG_CP.fetch_add(1, Ordering::Relaxed);
        if n < 60 {
            config::dlog(&format!(
                "cpswap#{n}: {cur} -> {newname} [{}] mid={match_id} | mypicks=[{}] pinned={:?}",
                if ru_used { "차순위" } else { "폴백" },
                my_picks.join(","),
                pinned.iter().map(|m| format!("{m:05b}")).collect::<Vec<_>>()
            ));
        }
    }
}

/// Vec<String>{cap@0, ptr@8, len@0x10} → 소문자 이름들.
unsafe fn read_bucket(v: usize) -> Option<Vec<String>> {
    let ptr = safe_rd_u64(v + 8)? as usize;
    let len = safe_rd_u64(v + 0x10)? as usize;
    if len == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(ptr) || len > 16 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let e = ptr + i * 0x18;
        let sp = safe_rd_u64(e + 8)? as usize;
        let sl = safe_rd_u64(e + 0x10)? as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            return None;
        }
        let b = safe_bytes(sp, sl)?;
        out.push(core::str::from_utf8(&b).ok()?.to_ascii_lowercase());
    }
    Some(out)
}

/// ctx+0x350 SwissTable 선형 스캔 → match_id 일치 엔트리의 rmi(=entry+8).
///   ★0.5.7(2026-08-27): 고정 오프셋 0x350/0x320 이 둘 다 빗나가면 **런타임 스캔**으로 찾는다.
///   RunningMatchInfo 의 테이블 위치·엔트리 stride 는 구조체 값이라 RVA 스캔·git 대조로
///   잡히지 않는다(실사고: `cpfail: find_rmi 실패 mid=1177` 반복 → 픽 제한 전부 무효).
unsafe fn find_rmi(ctx: usize, match_id: u64) -> Option<usize> {
    // ★0.5.7 실측(2026-08-27 런타임 스캔): ctx+0x358 stride=0x160. 구 0x350 은 빗나간다.
    if let Some(r) = find_rmi_at(ctx, 0x358, match_id) { return Some(r); }
    if let Some(r) = find_rmi_at(ctx, 0x350, match_id) { return Some(r); }
    // 이미 스캔으로 찾아둔 조합이 있으면 그걸 먼저
    let (o, s) = (RMI_OFF_DYN.load(Ordering::Relaxed), RMI_STRIDE_DYN.load(Ordering::Relaxed));
    if o != 0 && s != 0 {
        if let Some(r) = find_rmi_stride(ctx, o, s, match_id) { return Some(r); }
    }
    scan_rmi(ctx, match_id)
}

static RMI_OFF_DYN: AtomicUsize = AtomicUsize::new(0);
static RMI_STRIDE_DYN: AtomicUsize = AtomicUsize::new(0);
static RMI_SCAN_N: AtomicU64 = AtomicU64::new(0);

/// stride 를 지정해 SwissTable 을 훑는다(고정 0x160 가정을 뺀 일반형).
unsafe fn find_rmi_stride(ctx: usize, off: usize, stride: usize, match_id: u64) -> Option<usize> {
    if !ptr_ok(ctx) || match_id == u64::MAX { return None; }
    let ctl = safe_rd_u64(ctx + off)? as usize;
    let mask = safe_rd_u64(ctx + off + 8)? as usize;
    if !ptr_ok(ctl) || mask == 0 || mask > 0xffff { return None; }
    for i in 0..=mask {
        let c = safe_rd_u8(ctl + i)?;
        if c & 0x80 != 0 { continue; }
        let entry = ctl.wrapping_sub((i + 1) * stride);
        if !ptr_ok(entry) { continue; }
        if safe_rd_u64(entry) == Some(match_id) { return Some(entry + 8); }
    }
    None
}

/// ★런타임 스캔: ctx 상대변위 × stride 후보를 훑어 match_id 를 담은 조합을 찾는다.
///   찾으면 RMI_OFF_DYN/RMI_STRIDE_DYN 에 캐시하고 로그로 알린다(다음 패치 자동 적응).
unsafe fn scan_rmi(ctx: usize, match_id: u64) -> Option<usize> {
    if !ptr_ok(ctx) || match_id == u64::MAX { return None; }
    if RMI_SCAN_N.fetch_add(1, Ordering::Relaxed) >= 40 { return None; } // 폭주 방지
    const STRIDES: [usize; 8] = [0x160, 0x168, 0x158, 0x150, 0x170, 0x178, 0x148, 0x180];
    let mut off = 0x100usize;
    while off < 0x600 {
        if let (Some(ctl), Some(mask)) = (safe_rd_u64(ctx + off), safe_rd_u64(ctx + off + 8)) {
            let (ctl, mask) = (ctl as usize, mask as usize);
            if ptr_ok(ctl) && mask != 0 && mask <= 0xffff {
                for &st in STRIDES.iter() {
                    if let Some(r) = find_rmi_stride(ctx, off, st, match_id) {
                        RMI_OFF_DYN.store(off, Ordering::Relaxed);
                        RMI_STRIDE_DYN.store(st, Ordering::Relaxed);
                        config::dlog(&format!(
                            "★★find_rmi 스캔 적중: ctx+{off:#x} stride={st:#x} (소스 고정값 0x350/0x320·0x160 은 0.5.7 에서 빗나감  // — 재핀 필요)"));
                        return Some(r);
                    }
                }
            }
        }
        off += 8;
    }
    if RMI_SCAN_N.load(Ordering::Relaxed) <= 2 {
        config::dlog(&format!("★find_rmi 스캔 0건 (ctx={ctx:#x} mid={match_id})  // — ctx 자체 또는 match_id 축 의심"));
    }
    None
}

/// 지정 오프셋의 SwissTable 에서 match_id 일치 엔트리 → rmi(=entry+8).
unsafe fn find_rmi_at(ctx: usize, off: usize, match_id: u64) -> Option<usize> {
    if !ptr_ok(ctx) || match_id == u64::MAX {
        return None;
    }
    let ctl = safe_rd_u64(ctx + off)? as usize;
    let mask = safe_rd_u64(ctx + off + 8)? as usize;
    if !ptr_ok(ctl) || mask == 0 || mask > 0xffff {
        return None;
    }
    for i in 0..=mask {
        let c = core::ptr::read((ctl + i) as *const u8);
        if c & 0x80 != 0 {
            continue; // empty/deleted
        }
        let entry = ctl.wrapping_sub((i + 1) * 0x160);
        if !ptr_ok(entry) {
            continue;
        }
        if safe_rd_u64(entry) == Some(match_id) {
            return Some(entry + 8);
        }
    }
    None
}

/// 1바이트 fault-safe 읽기(정렬 qword 읽고 잘라냄 — VEH 경유라 stale 포인터에도 안전).
unsafe fn safe_rd_u8(a: usize) -> Option<u8> {
    safe_rd_u64(a & !7).map(|v| ((v >> ((a & 7) * 8)) & 0xff) as u8)
}

/// fault-safe 바이트열 읽기. `from_raw_parts` 는 stale 포인터에서 그대로 세그폴트라
/// (2026-08-23 유저 제보: `user_pick_block=1` 일 때만 튕김 — 셀/툴팁 경로가 전부 raw 였다)
/// 게임 소유 메모리에서 문자열을 읽을 땐 **반드시 이걸 쓴다**.
unsafe fn safe_bytes(ptr: usize, len: usize) -> Option<Vec<u8>> {
    if len == 0 || !ptr_ok(ptr) || len > 64 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    let mut k = 0usize;
    while k < len {
        let w = safe_rd_u64(ptr + k)?;
        for b in 0..8 {
            if k + b < len {
                out.push(((w >> (b * 8)) & 0xff) as u8);
            }
        }
        k += 8;
    }
    Some(out)
}

/// ★A(백그라운드 스왑) 규명용 진단 — **모든 매치**의 마지막 드래프트 스냅샷에서
///   vecC(+0x90)/vecD(+0xa8) 를 훑어 문자열로 돌려준다.
///   왜: 2026-08-22 커밋 시점 관측은 6매치 전부 `[0,1,2,3,4]`(identity) 였다.
///   유저 경기의 진짜 order(클라 씬 +0x198/+0x1b0)는 `[4,3,1,0,2]` 같은 비-identity 가
///   나오므로, **커밋 시점 identity = 아직 안 채워졌거나 스왑 order 가 아니다**.
///   드래프트가 끝난 뒤(=post_update 주기 스캔)에도 identity 면 후자로 확정.
pub unsafe fn scan_orders() -> Vec<String> {
    let mut out = Vec::new();
    let ctx = CP_CTX.load(Ordering::Relaxed);
    if !ptr_ok(ctx) {
        return out;
    }
    for off in [0x320usize, 0x350] {
        let Some(ctl) = safe_rd_u64(ctx + off) else { continue };
        let Some(mask) = safe_rd_u64(ctx + off + 8) else { continue };
        let (ctl, mask) = (ctl as usize, mask as usize);
        if !ptr_ok(ctl) || mask == 0 || mask > 0xffff {
            continue;
        }
        for i in 0..=mask {
            let Some(c) = safe_rd_u8(ctl + i) else { continue };
            if c & 0x80 != 0 {
                continue; // empty/deleted
            }
            let entry = ctl.wrapping_sub((i + 1) * 0x160);
            if !ptr_ok(entry) {
                continue;
            }
            let Some(mid) = safe_rd_u64(entry) else { continue };
            if mid == u64::MAX {
                continue;
            }
            if out.len() >= 64 {
                return out; // 폭주 가드(테이블 용량이 커도 로그·비용을 묶는다)
            }
            let rmi = entry + 8;
            let Some(sp) = safe_rd_u64(rmi + 8) else { continue };
            let Some(sc) = safe_rd_u64(rmi + 0x10) else { continue };
            let (sp, sc) = (sp as usize, sc as usize);
            if sc == 0 || sc > 4096 || !ptr_ok(sp) {
                continue;
            }
            let r = sp + (sc - 1) * 0x100;
            let vec_at = |o: usize| -> String {
                let ptr = safe_rd_u64(r + o + 8).unwrap_or(0) as usize;
                let len = safe_rd_u64(r + o + 0x10).unwrap_or(0) as usize;
                if len == 0 {
                    return String::new();
                }
                if !ptr_ok(ptr) || len > 16 {
                    return "?".to_string();
                }
                (0..len)
                    .map(|k| {
                        safe_rd_u64(ptr + k * 8)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "?".into())
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let n_at = |o: usize| safe_rd_u64(r + o + 0x10).unwrap_or(u64::MAX);
            out.push(format!(
                "mid={mid} snaps={sc} 밴={}/{} 픽={}/{} | 00=[{}] 18=[{}] 90=[{}] a8=[{}]",
                n_at(0x30),
                n_at(0x48),
                n_at(0x60),
                n_at(0x78),
                vec_at(0x00),
                vec_at(0x18),
                vec_at(0x90),
                vec_at(0xa8)
            ));
        }
    }
    out
}

/// Hook CP 설치 (post_update — 스레드 안전 패치·쿨다운 재시도).
pub fn install_once_cprod() {
    let st = INSTALL_STATE_CPROD.load(Ordering::Relaxed);
    if st == 1 || st == 2 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_CPROD.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    let cd = CPROD_COOLDOWN.load(Ordering::Relaxed);
    if cd > 0 {
        CPROD_COOLDOWN.store(cd - 1, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        // ★진입 패치(suspend) 대신 **콜사이트 rel32 리다이렉트** — f16ea0 는 매치 실행
        //   스레드에서 빈번 호출돼 진입 suspend 패치가 초기화 중 정지를 유발(실측 검은화면).
        //   콜사이트 3곳(Q1/Q2/Q3 드레인, RE 확정)만 우리 stub 으로 돌린다.
        let base = BASE.load(Ordering::Relaxed);
        let target = base + RVA_CPROD;
        let stub = match CP_STUB.load(Ordering::Relaxed) {
            0 => {
                let Some(s) = alloc_near(base + CPROD_CALLSITES[0]) else {
                    INSTALL_STATE_CPROD.store(2, Ordering::Relaxed);
                    config::dlog("hookCP: alloc_near 실패");
                    return;
                };
                let w = cprod_hook as usize;
                let mut sb = [0u8; 12];
                sb[0] = 0x48;
                sb[1] = 0xb8;
                sb[2..10].copy_from_slice(&w.to_le_bytes());
                sb[10] = 0xff;
                sb[11] = 0xe0;
                core::ptr::copy_nonoverlapping(sb.as_ptr(), s as *mut u8, 12);
                CP_STUB.store(s, Ordering::SeqCst);
                s
            }
            s => s,
        };
        let mut done = 0usize;
        for &rva in CPROD_CALLSITES.iter() {
            let addr = base + rva;
            let mut c = [0u8; 5];
            core::ptr::copy_nonoverlapping(addr as *const u8, c.as_mut_ptr(), 5);
            if c[0] != 0xe8 {
                continue;
            }
            let r = i32::from_le_bytes([c[1], c[2], c[3], c[4]]);
            if ((addr as i64) + 5 + r as i64) as usize != target {
                continue; // 이 콜사이트는 f16ea0 대상이 아님
            }
            let nr = (stub as i64) - ((addr as i64) + 5);
            if nr.abs() > 0x7f00_0000 {
                continue;
            }
            let mut patch = [0u8; 5];
            patch[0] = 0xe8;
            patch[1..5].copy_from_slice(&(nr as i32).to_le_bytes());
            let mut old: u32 = 0;
            if VirtualProtect(addr, 5, RWX, &mut old) == 0 {
                continue;
            }
            core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, 5);
            VirtualProtect(addr, 5, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), addr, 5);
            done += 1;
        }
        if done == 0 {
            INSTALL_STATE_CPROD.store(2, Ordering::Relaxed);
            config::dlog("hookCP: 콜사이트 패치 0건(RVA 확인 필요)");
            return;
        }
        // orig 는 트램폴린이 아니라 실제 함수 주소로 직접 호출(콜사이트만 바꿨으므로 안전).
        TRAMP_CPROD.store(target, Ordering::SeqCst);
        INSTALL_STATE_CPROD.store(1, Ordering::Relaxed);
        config::dlog(&format!("hookCP(f16ea0 콜사이트 {done}/3 픽교체) 설치 OK"));
    }
}

/// Hook AM 설치 (post_update 1회) — f73340 pick 콜사이트 rel32 → wrap.
pub fn install_once_argmax() {
    if INSTALL_STATE_AM.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_AM.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let base = BASE.load(Ordering::Relaxed);
        F848_ORIG.store(base + RVA_F848F0, Ordering::SeqCst);
        // 검증: 전 콜사이트가 CALL f848f0 인지(하나라도 다르면 RVA 표 갱신 필요 → 중단).
        let mut valid: Vec<usize> = Vec::new();
        for &rva in F848_CALLSITES.iter() {
            let addr = base + rva;
            let mut c = [0u8; 5];
            core::ptr::copy_nonoverlapping(addr as *const u8, c.as_mut_ptr(), 5);
            if c[0] != 0xe8 {
                continue;
            }
            let r = i32::from_le_bytes([c[1], c[2], c[3], c[4]]);
            if ((addr as i64) + 5 + r as i64) as usize == base + RVA_F848F0 {
                valid.push(addr);
            }
        }
        if valid.is_empty() {
            INSTALL_STATE_AM.store(2, Ordering::Relaxed);
            config::dlog("hookAM: 유효 콜사이트 0 (RVA 표 확인 필요)");
            return;
        }
        let stub = match AM_STUB.load(Ordering::Relaxed) {
            0 => {
                let Some(s) = alloc_near(valid[0]) else {
                    INSTALL_STATE_AM.store(2, Ordering::Relaxed);
                    config::dlog("hookAM: alloc_near 실패");
                    return;
                };
                // stub: movabs rax, wrap ; jmp rax
                let wrap = f848f0_wrap as usize;
                let mut sb = [0u8; 12];
                sb[0] = 0x48;
                sb[1] = 0xb8;
                sb[2..10].copy_from_slice(&wrap.to_le_bytes());
                sb[10] = 0xff;
                sb[11] = 0xe0;
                core::ptr::copy_nonoverlapping(sb.as_ptr(), s as *mut u8, 12);
                AM_STUB.store(s, Ordering::SeqCst);
                s
            }
            s => s,
        };
        let mut done = 0usize;
        for &addr in &valid {
            let new_rel = (stub as i64) - ((addr as i64) + 5);
            if new_rel.abs() > 0x7f00_0000 {
                continue;
            }
            let mut patch = [0u8; 5];
            patch[0] = 0xe8;
            patch[1..5].copy_from_slice(&(new_rel as i32).to_le_bytes());
            let mut old: u32 = 0;
            if VirtualProtect(addr, 5, RWX, &mut old) == 0 {
                continue;
            }
            core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, 5);
            VirtualProtect(addr, 5, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), addr, 5);
            done += 1;
        }
        if done == 0 {
            INSTALL_STATE_AM.store(2, Ordering::Relaxed);
            config::dlog("hookAM: 패치 0건");
            return;
        }
        INSTALL_STATE_AM.store(1, Ordering::Relaxed);
        config::dlog(&format!("hookAM(f848f0 콜사이트 {done}/{}) 설치 OK", valid.len()));
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Hook DQ — 결정 디스패처 FUN_142079730 출력 품질보존 교정 (2026-08-22)
//   축B/축E 확정: 디스패처 sret(0x58B) +0x10 = Vec<String>, [0]=실제 커밋될 챔프,
//   [1..]=점수 내림차순 차순위(0x2090c90 이 유한필터·내림차순·중복제거·최대8로 조립).
//   ⟹ [0]이 포지션 위반이면 **첫 합법 차순위와 24B 엔트리끼리 자리만 교환**.
//   Vec 내부 스왑이라 할당·free 없음(소유권 불변) = 메모리 안전. 품질 보존
//   (커밋 훅의 "NAMES 순서 첫 합법"은 alchemist 같은 약챔이 뽑히는 문제가 있었음).
//   호출자 1곳(producer 0x2038056)만 rel32 리다이렉트 — 진입 suspend 없음.
//   계약: rcx=sret, rdx=MS, r8=ctx, r9=kind, [+0x28]=match_id, [+0x30]=team_id
//   sret: +0x00 kind(-1=결정없음) +0x08 match_id +0x10 VecA +0x40 team_id +0x50 marker
// ══════════════════════════════════════════════════════════════════════════
const DISP_CALLSITE: usize = 0x1a374ae; // producer 내 유일 call 0x2079730
  // ← 0.5.7 재핀 — RVA_DISP 콜러 구/신 각 1건 (0.5.6=0x2038056)
const RVA_DISP: usize = 0x1a78ea0;
static DQ_STUB: AtomicUsize = AtomicUsize::new(0);
static DISP_ORIG: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_DQ: AtomicUsize = AtomicUsize::new(0);
pub static CNT_DQ_MISS: AtomicU64 = AtomicU64::new(0);
pub static CNT_DQ_FIX: AtomicU64 = AtomicU64::new(0);
static DBG_DQ: AtomicU64 = AtomicU64::new(0);
static DBG_DQE: AtomicU64 = AtomicU64::new(0);
/// ★매치별 "게임이 매긴 점수순 차순위" 캐시 — CP 안전망이 알파벳순 대신 이걸 우선 사용.
///   디스패처(DQ)가 매 결정마다 갱신. (match_id, 이름들) 최대 16매치.
static RUNNERUP: std::sync::Mutex<Vec<(u64, Vec<String>)>> = std::sync::Mutex::new(Vec::new());

/// 그 매치의 최근 차순위 목록(점수 내림차순) 조회.
pub fn runnerup_for(match_id: u64) -> Vec<String> {
    let g = RUNNERUP.lock().unwrap_or_else(|e| e.into_inner());
    g.iter()
        .find(|(m, _)| *m == match_id)
        .map(|(_, v)| v.clone())
        .unwrap_or_default()
}

/// ★누적 저장(덮어쓰기 아님): 드래프트 내내 "코치가 점수를 매긴 챔프" 풀을 키운다.
///   최근 결정의 차순위를 **앞에 붙여** 최신 선호가 우선되게 하고, 중복 제거 후 48개로 캡.
///   (8개만 갖고 있으면 후반에 전부 포지션 충돌 시 알파벳순 폴백으로 떨어져 품질이 급락.)
fn runnerup_store(match_id: u64, names: Vec<String>) {
    let mut g = RUNNERUP.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(e) = g.iter_mut().find(|(m, _)| *m == match_id) {
        let mut merged: Vec<String> = names;
        for old in e.1.iter() {
            if !merged.iter().any(|n| n.eq_ignore_ascii_case(old)) {
                merged.push(old.clone());
            }
        }
        merged.truncate(48);
        e.1 = merged;
    } else {
        if g.len() >= 16 {
            g.remove(0);
        }
        g.push((match_id, names));
    }
}
pub static CNT_DQ_ENTER: AtomicU64 = AtomicU64::new(0);
pub static CNT_DQ_NOALT: AtomicU64 = AtomicU64::new(0);

type DispFn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize) -> usize;

unsafe extern "C" fn disp_hook(
    sret: usize,
    ms: usize,
    ctx: usize,
    kind: usize,
    match_id: usize,
    team_id: usize,
) -> usize {
    let o = DISP_ORIG.load(Ordering::Relaxed);
    let ret = if o != 0 {
        let f: DispFn = core::mem::transmute(o);
        f(sret, ms, ctx, kind, match_id, team_id)
    } else {
        0
    };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        disp_fix(sret, ctx, match_id as u64, team_id as u64);
    }));
    ret
}

/// sret+0x10 Vec<String> 의 [0]이 포지션 위반이면 첫 합법 차순위와 스왑.
unsafe fn disp_fix(sret: usize, ctx: usize, match_id: u64, team_id: u64) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return;
    }
    if !ptr_ok(sret) {
        return;
    }
    CNT_DQ_ENTER.fetch_add(1, Ordering::Relaxed);
    // ★진단: sret 실제 레이아웃 확인(kind/marker/Vec). DQ 미발화 원인 규명용.
    if cfg.debug {
        let n = DBG_DQE.fetch_add(1, Ordering::Relaxed);
        if n < 30 {
            let nm0 = {
                let p = safe_rd_u64(sret + 0x18).unwrap_or(0) as usize;
                let l = safe_rd_u64(sret + 0x20).unwrap_or(0) as usize;
                if l > 0 && ptr_ok(p) {
                    let sp = safe_rd_u64(p + 8).unwrap_or(0) as usize;
                    let sl = safe_rd_u64(p + 0x10).unwrap_or(0) as usize;
                    read_str_at(sp, sl).unwrap_or_else(|| "?".into())
                } else {
                    "-".into()
                }
            };
            config::dlog(&format!(
                "dqe#{n}: kind={:?} arg_mid={match_id} arg_team={team_id} vlen={:?} [0]={nm0}",
                safe_rd_u64(sret),
                safe_rd_u64(sret + 0x20),
            ));
        }
    }
    // kind == -1 이면 결정 없음.
    if safe_rd_u64(sret).unwrap_or(u64::MAX) == u64::MAX {
        return;
    }
    // ⚠sret+0x40(team)·+0x50(marker) 는 RE 추정이 실측과 불일치(team=0, marker=거대값).
    //   ⟹ 팀/매치는 **디스패처 인자**를 쓰고, 밴/픽 구분은 스냅샷에서 계산한다.
    let vptr = safe_rd_u64(sret + 0x18).unwrap_or(0) as usize;
    let vlen = safe_rd_u64(sret + 0x20).unwrap_or(0) as usize;
    if vlen < 2 || !ptr_ok(vptr) || vlen > 16 {
        CNT_DQ_NOALT.fetch_add(1, Ordering::Relaxed);
        return; // 차순위 없으면 개입 불가(커밋 훅이 안전망)
    }
    let Some((pinned, used, is_ban)) = team_state(ctx, match_id, team_id) else {
        return;
    };
    ai_decided_mark(match_id, used.len()); // ★AI 결정(디스패처 경유) 마킹
    if is_ban {
        return; // 밴 결정은 포지션 무관
    }
    if pinned.is_empty() {
        return;
    }
    let (Some(names), Some(masks)) = (crate::names(), crate::masks()) else {
        return;
    };
    let m_of = |nm: &str| -> u8 {
        names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(nm))
            .and_then(|i| masks.get(i).copied())
            .unwrap_or(config::MASK_ALL)
    };
    let ent = |i: usize| -> Option<String> {
        let e = vptr + i * 0x18;
        let p = safe_rd_u64(e + 8)? as usize;
        let l = safe_rd_u64(e + 0x10)? as usize;
        read_str_at(p, l)
    };
    // ★차순위 전체를 매치 캐시에 저장(CP 안전망 품질용). 교체 여부와 무관.
    {
        let mut rl: Vec<String> = Vec::with_capacity(vlen);
        for i in 0..vlen {
            if let Some(nm) = ent(i) {
                rl.push(nm);
            }
        }
        if rl.len() > 1 {
            runnerup_store(match_id, rl);
        }
    }
    let Some(cur) = ent(0) else { return };
    let cm = m_of(&cur);
    if cm == config::MASK_ALL {
        return;
    }
    if crate::helps(&pinned, cm) {
        return; // 이미 합법(새 라인을 채움)
    }
    // ★자유 슬롯이 남아 있으면 교체하지 않는다(위 CP 훅과 같은 규칙).
    {
        let pinned_all: Vec<u8> = pinned.clone();
        let pool: Vec<u8> = names
            .iter()
            .enumerate()
            .filter(|(_, n)| !used.iter().any(|u| u.eq_ignore_ascii_case(n)))
            .map(|(i, _)| masks.get(i).copied().unwrap_or(config::MASK_ALL))
            .collect();
        if crate::free_left(&pinned_all, &pool, PICKS_PER_TEAM) > 0 {
            return;
        }
    }
    // 차순위에서 첫 합법(그 매치의 밴/픽에도 없어야 커밋 통과) 탐색.
    //   ★무제한(MASK_ALL) 챔프는 어느 포지션에나 가므로 **스킵하지 않는다**(항상 안전).
    //     이전 구현이 스킵해서 상위 차순위가 불필요하게 밀렸음(2026-08-22 수정).
    let mut skipped: Vec<String> = Vec::new();
    for i in 1..vlen {
        let Some(cand) = ent(i) else { continue };
        if used.iter().any(|u| u.eq_ignore_ascii_case(&cand)) {
            skipped.push(format!("{i}:{cand}=밴픽됨"));
            continue;
        }
        let m = m_of(&cand);
        if !crate::helps(&pinned, m) {
            skipped.push(format!("{i}:{cand}=포지션충돌"));
            continue;
        }
        // 24B 엔트리 스왑(할당/free 없음 = 소유권 불변).
        let a = vptr;
        let b = vptr + i * 0x18;
        let mut tmp = [0u8; 0x18];
        core::ptr::copy_nonoverlapping(a as *const u8, tmp.as_mut_ptr(), 0x18);
        core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 0x18);
        core::ptr::copy_nonoverlapping(tmp.as_ptr(), b as *mut u8, 0x18);
        CNT_DQ_FIX.fetch_add(1, Ordering::Relaxed);
        if cfg.debug {
            let n = DBG_DQ.fetch_add(1, Ordering::Relaxed);
            if n < 60 {
                config::dlog(&format!(
                    "dqfix#{n}: {cur} -> {cand} (차순위 {i}/{vlen}) 건너뜀=[{}] pinned={:?}",
                    skipped.join(" "),
                    pinned.iter().map(|m| format!("{m:05b}")).collect::<Vec<_>>()
                ));
            }
        }
        return;
    }
    CNT_DQ_MISS.fetch_add(1, Ordering::Relaxed); // 차순위에 합법 없음 → 커밋 훅이 처리
}

/// (그 매치의) 해당 팀 pinned 마스크 + 이미 사용된(밴/픽) 이름들.
unsafe fn team_state(
    ctx: usize,
    match_id: u64,
    team_id: u64,
) -> Option<(Vec<u8>, Vec<String>, bool)> {
    let rmi = find_rmi(ctx, match_id).or_else(|| find_rmi_at(ctx, 0x320, match_id))?;
    let sp = safe_rd_u64(rmi + 8)? as usize;
    let sc = safe_rd_u64(rmi + 0x10)? as usize;
    if sc == 0 || !ptr_ok(sp) {
        return None;
    }
    let r = sp + (sc - 1) * 0x100;
    let mut used: Vec<String> = Vec::new();
    let mut p_a: Vec<String> = Vec::new();
    let mut p_b: Vec<String> = Vec::new();
    for (off, w) in [(0x30usize, 0u8), (0x48, 0), (0x60, 1), (0x78, 2)] {
        if let Some(v) = read_bucket(r + off) {
            used.extend(v.iter().cloned());
            match w {
                1 => p_a = v,
                2 => p_b = v,
                _ => {}
            }
        }
    }
    // 밴/픽 페이즈: total(4버킷 합) < 2*ban_limit(r+0xf0) 이면 밴 (커밋 RE 확정식).
    let total = used.len();
    let ban_limit = safe_rd_u64(r + 0xf0).unwrap_or(0) as usize;
    let is_ban = total < ban_limit * 2;
    let side = safe_rd_u8(r + 0xf8).unwrap_or(0xff) as usize & 1;
    let team_a = safe_rd_u64(rmi + 0x140 + ((side ^ 1) * 8)).unwrap_or(u64::MAX);
    let mine = if team_id == team_a { &p_a } else { &p_b };
    let (Some(names), Some(masks)) = (crate::names(), crate::masks()) else {
        return None;
    };
    let pinned: Vec<u8> = mine
        .iter()
        .map(|nm| {
            names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(nm))
                .and_then(|i| masks.get(i).copied())
                .unwrap_or(config::MASK_ALL)
        })
        .filter(|&m| m != config::MASK_ALL)
        .collect();
    Some((pinned, used, is_ban))
}

/// Hook DQ 설치 — 디스패처 콜사이트 rel32 리다이렉트(진입 suspend 없음).
pub fn install_once_dq() {
    if INSTALL_STATE_DQ.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let base = BASE.load(Ordering::Relaxed);
        let target = base + RVA_DISP;
        DISP_ORIG.store(target, Ordering::SeqCst);
        let cs = base + DISP_CALLSITE;
        let mut c = [0u8; 5];
        core::ptr::copy_nonoverlapping(cs as *const u8, c.as_mut_ptr(), 5);
        if c[0] != 0xe8 {
            INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
            config::dlog(&format!("hookDQ: 콜사이트 non-CALL({:02x})", c[0]));
            return;
        }
        let rel = i32::from_le_bytes([c[1], c[2], c[3], c[4]]);
        if ((cs as i64) + 5 + rel as i64) as usize != target {
            INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
            config::dlog("hookDQ: 콜사이트 tgt 불일치");
            return;
        }
        let stub = match DQ_STUB.load(Ordering::Relaxed) {
            0 => {
                let Some(s) = alloc_near(cs) else {
                    INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
                    config::dlog("hookDQ: alloc_near 실패");
                    return;
                };
                let w = disp_hook as usize;
                let mut sb = [0u8; 12];
                sb[0] = 0x48;
                sb[1] = 0xb8;
                sb[2..10].copy_from_slice(&w.to_le_bytes());
                sb[10] = 0xff;
                sb[11] = 0xe0;
                core::ptr::copy_nonoverlapping(sb.as_ptr(), s as *mut u8, 12);
                DQ_STUB.store(s, Ordering::SeqCst);
                s
            }
            s => s,
        };
        let nr = (stub as i64) - ((cs as i64) + 5);
        if nr.abs() > 0x7f00_0000 {
            INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
            config::dlog("hookDQ: rel32 범위초과");
            return;
        }
        let mut patch = [0u8; 5];
        patch[0] = 0xe8;
        patch[1..5].copy_from_slice(&(nr as i32).to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(cs, 5, RWX, &mut old) == 0 {
            INSTALL_STATE_DQ.store(2, Ordering::Relaxed);
            return;
        }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), cs as *mut u8, 5);
        VirtualProtect(cs, 5, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), cs, 5);
        INSTALL_STATE_DQ.store(1, Ordering::Relaxed);
        config::dlog("hookDQ(디스패처 0x232a950 품질보존 교정) 설치 OK");
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Hook CB — 후보 빌더 FUN_1418d01b0 출력 필터 (2026-08-22, 축B 1순위 권고)
//   디스패처가 점수를 매기기 **전에** 후보 Vec<u64>에서 포지션 충돌 챔프를 제거하면,
//   게임이 합법 후보만 8순위로 매기고 [0]부터 정답이라 사후 교체가 불필요해진다.
//   콜사이트 0x207a191(= call 0x18d01b0, 정적 검증 완료) rel32 리다이렉트.
//   계약(축B): FUN_1418d01b0(sret /*rcx*/, argpack /*rdx*/) → sret = Vec<u64>{cap,ptr@8,len@0x10}
//     argpack = {MS, &snap, ?, ?, &is_ban, …}
//   ⚠1단계는 **진단 전용**: 인덱스 축이 우리 NAMES 와 같은지 확인만(필터 안 함).
//     축이 다르면 엉뚱한 챔프를 걸러 AI 오작동·후보 고갈(hang) 위험.
// ══════════════════════════════════════════════════════════════════════════
const CB_CALLSITE: usize = 0x1a79952;  // 0.5.7 재핀 — RVA_CANDB 콜러 구/신 각 1건 (0.5.6=0x207a191)
const RVA_CANDB: usize = 0x191bfb0;  // 0.5.7 재핀 UNIQUE size806 동일 (0.5.6=0x18d01b0)
static CB_STUB: AtomicUsize = AtomicUsize::new(0);
static CANDB_ORIG: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_CB: AtomicUsize = AtomicUsize::new(0);
pub static CNT_CB_SEEN: AtomicU64 = AtomicU64::new(0);
pub static CNT_CB_CUT: AtomicU64 = AtomicU64::new(0);
static DBG_CB: AtomicU64 = AtomicU64::new(0);
static DBG_CBF: AtomicU64 = AtomicU64::new(0);

/// ★유저가 **실제로 클릭한** 챔프 이름(최근 N개). CK(클릭 커밋 훅)만 기록한다.
///   타임아웃 자동픽·코치 위임은 클릭이 없으므로 여기 안 들어간다 ⟹ 교정 대상.
static HUMAN_PICKED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn human_mark(name: &str) {
    let mut g = HUMAN_PICKED.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() >= 32 {
        g.remove(0);
    }
    g.push(name.to_ascii_lowercase());
}

fn human_take(name: &str) -> bool {
    let mut g = HUMAN_PICKED.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(i) = g.iter().position(|n| n.eq_ignore_ascii_case(name)) {
        g.remove(i);
        return true;
    }
    false
}

/// ★AI 결정 마킹 — (match_id, total) 키. CB/DQ(=AI 결정 경로)가 발화할 때 기록하고,
///   CP(커밋 교체)는 **유저 팀 픽인데 이 기록이 없으면 = 사람이 직접 고른 픽**으로 보고
///   절대 교체하지 않는다(유저 지시: "무효픽 눌렀을 때 멋대로 바꾸면 안 된다").
///   백그라운드 매치는 유저 팀이 아니므로 이 가드에 걸리지 않는다.
static AI_DECIDED: std::sync::Mutex<Vec<(u64, usize)>> = std::sync::Mutex::new(Vec::new());

fn ai_decided_mark(match_id: u64, total: usize) {
    let mut g = AI_DECIDED.lock().unwrap_or_else(|e| e.into_inner());
    if g.iter().any(|&(m, t)| m == match_id && t == total) {
        return;
    }
    if g.len() >= 256 {
        g.remove(0);
    }
    g.push((match_id, total));
}

fn ai_decided_has(match_id: u64, total: usize) -> bool {
    let g = AI_DECIDED.lock().unwrap_or_else(|e| e.into_inner());
    g.iter().any(|&(m, t)| m == match_id && t == total)
}

type CandFn = unsafe extern "C" fn(usize, usize, usize, usize) -> usize;

unsafe extern "C" fn cand_hook(sret: usize, argpack: usize, p3: usize, p4: usize) -> usize {
    let o = CANDB_ORIG.load(Ordering::Relaxed);
    let ret = if o != 0 {
        let f: CandFn = core::mem::transmute(o);
        f(sret, argpack, p3, p4)
    } else {
        0
    };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cand_probe(sret, argpack);
        cand_filter(sret, argpack);
    }));
    ret
}

/// 픽 순서 테이블(축B/축E 실측): 값 0=side0, 1=side1.
const PICK_TBL: [&[u8]; 4] = [
    &[0, 1, 0, 1],
    &[0, 1, 1, 0, 0, 1],
    &[0, 1, 1, 0, 1, 0, 0, 1],
    &[0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
];

/// ★후보 Vec<u64> 에서 "현재 턴 팀의 이미 찬 포지션과 겹치는" 챔프를 제거.
///   게임이 **점수를 매기기 전** 단계라, 이후 8순위 랭킹이 전부 합법 후보로 채워진다
///   ⟹ [0]부터 정답 = 사후 교체 불필요·품질 최상.
///   안전: 남는 후보가 0이면 필터하지 않음(고갈 = f338a0 폴백/hang 위험 회피).
unsafe fn cand_filter(sret: usize, argpack: usize) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate || !config::any_restricted() {
        return;
    }
    if !ptr_ok(sret) || !ptr_ok(argpack) {
        return;
    }
    // argpack+8 = &snap(0x100B 드래프트 스냅샷).
    let snap = match safe_rd_u64(argpack + 8) {
        Some(v) if ptr_ok(v as usize) => v as usize,
        _ => return,
    };
    // 4버킷: +0x30 side0밴 / +0x48 side1밴 / +0x60 side0픽 / +0x78 side1픽 (축B 확정).
    let (Some(b0), Some(b1), Some(p0), Some(p1)) = (
        read_bucket(snap + 0x30),
        read_bucket(snap + 0x48),
        read_bucket(snap + 0x60),
        read_bucket(snap + 0x78),
    ) else {
        return; // 버킷 못 읽으면 개입 안 함
    };
    let total = b0.len() + b1.len() + p0.len() + p1.len();
    let ban = safe_rd_u64(snap + 0xf0).unwrap_or(0) as usize;
    let fmt = ru8(snap + 0xf9) as usize;
    if fmt >= 4 {
        return;
    }
    let base = ban * 2;
    if total < base {
        return; // 밴 페이즈 — 포지션 무관
    }
    let tbl = PICK_TBL[fmt];
    let idx = total - base;
    if idx >= tbl.len() {
        return; // 드래프트 종료
    }
    let side = tbl[idx];
    let mine: &Vec<String> = if side == 0 { &p0 } else { &p1 };
    if mine.is_empty() {
        return;
    }
    let (Some(names), Some(masks)) = (crate::names(), crate::masks()) else {
        return;
    };
    let pinned: Vec<u8> = mine
        .iter()
        .map(|nm| {
            names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(nm))
                .and_then(|i| masks.get(i).copied())
                .unwrap_or(config::MASK_ALL)
        })
        .filter(|&m| m != config::MASK_ALL)
        .collect();
    if pinned.is_empty() {
        return;
    }
    // 후보 Vec 순회 → 합법만 앞으로 압축.
    let ptr = match safe_rd_u64(sret + 8) {
        Some(v) if ptr_ok(v as usize) => v as usize,
        _ => return,
    };
    let len = safe_rd_u64(sret + 0x10).unwrap_or(0) as usize;
    if len == 0 || len > 512 {
        return;
    }
    // ★자유 슬롯이 남아 있으면 후보를 하나도 자르지 않는다(위 CP/DQ 훅과 같은 규칙).
    //   여기선 후보 리스트 자체가 곧 남은 풀이다.
    {
        let mut pool: Vec<u8> = Vec::with_capacity(len);
        for i in 0..len {
            let Some(v) = safe_rd_u64(ptr + i * 8) else { return };
            pool.push(masks.get(v as usize).copied().unwrap_or(config::MASK_ALL));
        }
        if crate::free_left(&pinned, &pool, 2 + fmt) > 0 {
            return;
        }
    }
    let mut keep: Vec<u64> = Vec::with_capacity(len);
    for i in 0..len {
        let v = match safe_rd_u64(ptr + i * 8) {
            Some(v) => v,
            None => return, // 읽기 실패 시 전체 포기(부분 수정 금지)
        };
        let m = masks.get(v as usize).copied().unwrap_or(config::MASK_ALL);
        if crate::helps(&pinned, m) {
            keep.push(v);
        }
    }
    if keep.is_empty() || keep.len() == len {
        return; // 고갈(fail-open) 또는 변화 없음
    }
    for (i, v) in keep.iter().enumerate() {
        core::ptr::write((ptr + i * 8) as *mut u64, *v);
    }
    core::ptr::write((sret + 0x10) as *mut u64, keep.len() as u64);
    CNT_CB_CUT.fetch_add((len - keep.len()) as u64, Ordering::Relaxed);
    if cfg.debug {
        let n = DBG_CBF.fetch_add(1, Ordering::Relaxed);
        if n < 40 {
            config::dlog(&format!(
                "cbcut#{n}: {len} -> {} (side={side} mine=[{}] pinned={:?})",
                keep.len(),
                mine.join(","),
                pinned.iter().map(|m| format!("{m:05b}")).collect::<Vec<_>>()
            ));
        }
    }
}

/// 진단: 후보 Vec 의 인덱스를 우리 NAMES 로 해석해 축 일치 여부 확인 + argpack 덤프.
unsafe fn cand_probe(sret: usize, argpack: usize) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.debug {
        return;
    }
    if !ptr_ok(sret) {
        return;
    }
    CNT_CB_SEEN.fetch_add(1, Ordering::Relaxed);
    let n = DBG_CB.fetch_add(1, Ordering::Relaxed);
    if n >= 12 {
        return;
    }
    let ptr = safe_rd_u64(sret + 8).unwrap_or(0) as usize;
    let len = safe_rd_u64(sret + 0x10).unwrap_or(0) as usize;
    let nm = |i: usize| {
        crate::names().and_then(|v| v.get(i)).map(|s| s.as_str()).unwrap_or("?")
    };
    let mut head: Vec<String> = Vec::new();
    if ptr_ok(ptr) && len > 0 && len < 512 {
        for i in 0..len.min(10) {
            let v = safe_rd_u64(ptr + i * 8).unwrap_or(u64::MAX);
            head.push(format!("{v}:{}", nm(v as usize)));
        }
    }
    // argpack 앞 6워드(= MS, &snap, ?, ?, &is_ban, ?) 덤프 — snap 위치 확인용.
    let mut ap: Vec<String> = Vec::new();
    for i in 0..6 {
        ap.push(format!(
            "{:x}",
            safe_rd_u64(argpack + i * 8).unwrap_or(0) & 0xffffff
        ));
    }
    config::dlog(&format!(
        "cb#{n}: len={len} head=[{}] ap=[{}]",
        head.join(" "),
        ap.join(",")
    ));
}

/// Hook CB 설치 — 후보 빌더 콜사이트 rel32 리다이렉트.
pub fn install_once_cb() {
    if INSTALL_STATE_CB.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_pick_gate {
        INSTALL_STATE_CB.store(2, Ordering::Relaxed);
        return;
    }
    // ★[2026-09-03] "제한이 아직 없음"은 세이브 로드 전일 수 있다 ⟹ 영구실패로
    //   못박지 말고 재시도(state 0 유지). 구 코드는 여기서 state=2 로 박제해
    //   세이브를 불러 제한이 생겨도 훅이 안 붙었다(코치 위임 픽 무차단 실사고).
    if !config::any_restricted() {
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let base = BASE.load(Ordering::Relaxed);
        let target = base + RVA_CANDB;
        CANDB_ORIG.store(target, Ordering::SeqCst);
        let cs = base + CB_CALLSITE;
        let mut c = [0u8; 5];
        core::ptr::copy_nonoverlapping(cs as *const u8, c.as_mut_ptr(), 5);
        if c[0] != 0xe8 {
            INSTALL_STATE_CB.store(2, Ordering::Relaxed);
            config::dlog(&format!("hookCB: 콜사이트 non-CALL({:02x})", c[0]));
            return;
        }
        let rel = i32::from_le_bytes([c[1], c[2], c[3], c[4]]);
        if ((cs as i64) + 5 + rel as i64) as usize != target {
            INSTALL_STATE_CB.store(2, Ordering::Relaxed);
            config::dlog("hookCB: 콜사이트 tgt 불일치");
            return;
        }
        let stub = match CB_STUB.load(Ordering::Relaxed) {
            0 => {
                let Some(s) = alloc_near(cs) else {
                    INSTALL_STATE_CB.store(2, Ordering::Relaxed);
                    config::dlog("hookCB: alloc_near 실패");
                    return;
                };
                let w = cand_hook as usize;
                let mut sb = [0u8; 12];
                sb[0] = 0x48;
                sb[1] = 0xb8;
                sb[2..10].copy_from_slice(&w.to_le_bytes());
                sb[10] = 0xff;
                sb[11] = 0xe0;
                core::ptr::copy_nonoverlapping(sb.as_ptr(), s as *mut u8, 12);
                CB_STUB.store(s, Ordering::SeqCst);
                s
            }
            s => s,
        };
        let nr = (stub as i64) - ((cs as i64) + 5);
        if nr.abs() > 0x7f00_0000 {
            INSTALL_STATE_CB.store(2, Ordering::Relaxed);
            config::dlog("hookCB: rel32 범위초과");
            return;
        }
        let mut patch = [0u8; 5];
        patch[0] = 0xe8;
        patch[1..5].copy_from_slice(&(nr as i32).to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(cs, 5, RWX, &mut old) == 0 {
            INSTALL_STATE_CB.store(2, Ordering::Relaxed);
            return;
        }
        core::ptr::copy_nonoverlapping(patch.as_ptr(), cs as *mut u8, 5);
        VirtualProtect(cs, 5, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), cs, 5);
        INSTALL_STATE_CB.store(1, Ordering::Relaxed);
        config::dlog("hookCB(후보빌더 0x18d01b0 진단) 설치 OK");
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Hook GY/CK — 유저 밴픽 UI 회색화 + 클릭 차단 (2026-08-22 RE 확정)
//   ★기존 hookC(0xb31840 contains)는 **밴픽 진입 1회 빌더**(FUN_1424d7640)에서만
//     호출된다(94챔프×2=188 = 실측 정지 카운터와 일치) ⟹ 픽 진행 중 무효. 폐기.
//   (A) 회색화 = 0x2553a16 (call 0x249df50) 콜사이트 리다이렉트.
//       FUN_14254f8f0 의 셀 전수 순회 루프. rcx = 셀 노드(stride 0x268):
//         이름 = [rcx+8](ptr)/[rcx+0x10](len)  ← 챔프 내부이름
//         로직 = *(u64*)(rcx+0x230), 상태 바이트 = 로직+0x1d3
//         상태: 0=정상 3=밴(회색) 4=픽blue 5=픽red 1/6/7=피어리스
//       ⚠원본 반환 al(1/0) 보존 필수(호출자가 test al,al 로 분기).
//       ⚠**현재값이 0일 때만** 3으로 칠하고, 우리가 칠한 것만 0으로 되돌린다
//         (진짜 밴/픽을 지우지 않기 위해).
//   (B) 클릭 차단 = 0x25508de (call 0x24b6c10) 콜사이트 리다이렉트.
//       인자: rcx=controller, rdx=view_root, r8=ctx, r9=side,
//             [rsp+0x20]=champ_name_ptr, [rsp+0x28]=len, [rsp+0x30]=flag
//       blocklist 면 원본 미호출 → 클릭 소멸(밴/픽 벡터 append 없음 = 부작용 0).
//       banpick_order 는 0x24b6c10 **진입부**를 후킹하므로, 통과 시 원래 주소로
//       호출하면 그쪽 detour 도 정상 발화(체인 불요).
// ══════════════════════════════════════════════════════════════════════════
const GY_CALLSITE: usize = 0x21bc336; // call 0x1e1e3c0 (셀 순회) // ★0.5.7 재핀 정정(2026-08-27): owner 0x254f8f0->0x1ed7970 내부 2개 중 구 +0x4126 짝. ~~0x1eda9fa~~ 는 구 0x255297a(+0x308a) 쪽이라 GY 발화 0회였다. ⚠전체 콜러 인덱스 순서가 아니라 **owner 내 오프셋**으로 짚을 것
const RVA_CELLFN: usize = 0x2105f80;  // 0.5.7 재핀 UNIQUE size1477 동일 (0.5.6=0x249df50)
const CK_CALLSITE: usize = 0x21b91de; // call 0x24b6c10 (클릭 커밋)
  // ← 0.5.7 재핀 — RVA_COMMITTER 콜러 구/신 각 3건 순서대응 #2 (0.5.6=0x25508de)
const RVA_COMMITTER: usize = 0x2140c40;

static GY_STUB: AtomicUsize = AtomicUsize::new(0);
static CELLFN_ORIG: AtomicUsize = AtomicUsize::new(0);
static CK_STUB: AtomicUsize = AtomicUsize::new(0);
static COMMITTER_ORIG: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_GY: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE_CK: AtomicUsize = AtomicUsize::new(0);
pub static CNT_GY_CELL: AtomicU64 = AtomicU64::new(0);
pub static CNT_GY_PAINT: AtomicU64 = AtomicU64::new(0);
pub static CNT_CK_BLOCK: AtomicU64 = AtomicU64::new(0);
static DBG_GY: AtomicU64 = AtomicU64::new(0);
static DBG_GYN: AtomicU64 = AtomicU64::new(0);
static DBG_DISP: AtomicU64 = AtomicU64::new(0);
/// pos_tooltip 의 마지막 실측 폭(f32 비트). rect 는 보이는 프레임에만 채워지므로 캐시한다.
static POS_TIP_W: AtomicU32 = AtomicU32::new(0x4302_0000); // 130.0

static DBG_CK: AtomicU64 = AtomicU64::new(0);
/// 우리가 회색으로 칠한 챔프(해제 시 원복 대상).
/// 우리가 회색으로 칠한 셀: 이름 → **원래 상태값**(복원용).
static GREYED: Mutex<Option<std::collections::HashMap<String, u8>>> = Mutex::new(None);

type CellFn = unsafe extern "C" fn(usize, usize, usize) -> u8;
type UiCommitFn =
    unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize) -> usize;

unsafe extern "C" fn cell_hook(node: usize, edx: usize, r8: usize) -> u8 {
    let o = CELLFN_ORIG.load(Ordering::Relaxed);
    let ret = if o != 0 {
        let f: CellFn = core::mem::transmute(o);
        f(node, edx, r8)
    } else {
        0
    };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        cell_paint(node);
    }));
    ret // ★원본 반환값 보존
}

/// 셀 노드 하나를 blocklist 에 따라 회색(3)/원복(0) 처리.
unsafe fn cell_paint(node: usize) {
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block {
        return;
    }
    if !ptr_ok(node) {
        return;
    }
    CNT_GY_CELL.fetch_add(1, Ordering::Relaxed);
    let np = match safe_rd_u64(node + 8) {
        Some(v) => v as usize,
        None => return,
    };
    let nl = match safe_rd_u64(node + 0x10) {
        Some(v) => v as usize,
        None => return,
    };
    let Some(name) = read_str_at(np, nl) else {
        return;
    };
    let logic = match safe_rd_u64(node + 0x230) {
        Some(v) if ptr_ok(v as usize) => v as usize,
        _ => return,
    };
    let Some(cur) = safe_rd_u8(logic + 0x1d3) else {
        return; // 셀 로직 객체가 이미 해제됨(과도기) — 손대지 않는다
    };
    // ★진단: 로직 객체에서 "표시이름(한글)" 오프셋 탐색 — 8B 스텝으로 (ptr,len) 후보 스캔.
    //   찾으면 밴픽 때 수집해 캐시 → 패치로 챔프가 추가돼도 정렬이 자동으로 따라간다.
    if cfg.debug {
        let n = DBG_DISP.fetch_add(1, Ordering::Relaxed);
        if n < 3 {
            let mut found: Vec<String> = Vec::new();
            let mut off = 0usize;
            while off < 0x1c0 {
                let p = safe_rd_u64(logic + off).unwrap_or(0) as usize;
                let l = safe_rd_u64(logic + off + 8).unwrap_or(0) as usize;
                if ptr_ok(p) && (1..=64).contains(&l) {
                    if let Some(t) = read_str_at(p, l) {
                        if t.chars().any(|c| !c.is_ascii()) || t.len() >= 3 {
                            found.push(format!("+{off:#x}={t}"));
                        }
                    }
                }
                off += 8;
            }
            config::dlog(&format!("dispname#{n}: node={name} {}", found.join(" ")));
        }
    }
    // ★진단: 셀에서 읽은 이름·상태 실제값(블록리스트와 형식이 맞는지).
    if cfg.debug {
        let n = DBG_GYN.fetch_add(1, Ordering::Relaxed);
        if n < 24 {
            let blsz = {
                let g = plock(&BLOCKLIST);
                g.as_ref().map(|b| b.len()).unwrap_or(usize::MAX)
            };
            config::dlog(&format!("gycell#{n}: name='{name}' state={cur} blsz={blsz}"));
        }
    }
    let want = {
        let g = plock(&BLOCKLIST);
        g.as_ref().is_some_and(|b| b.contains(&name))
    };
    // ★실측(2026-08-22): "선택 가능" 상태는 0 이 아니라 **2**. (RE 표의 0=정상은 오기)
    //   0/2 둘 다 정상으로 보고, 원래 값을 기억했다가 해제 시 그대로 복원한다.
    const NORMAL: [u8; 2] = [0, 2];
    if want {
        grey_rect_note(&name, node); // (구)히트테스트용 rect — 게임 툴팁 사용 후엔 참고용
        // ★state 6 = 밴(3)과 **같은 회색**이면서 게임의 fearless_tooltip 이 뜨는 상태.
        //   (update 0x254a5b0 이 state∈{0,1,6,7} 일 때만 툴팁을 처리 — RE 2026-08-22)
        let msg = crate::block_msg(&name);
        set_cell_tooltip(node, &msg);
        if NORMAL.contains(&cur) {
            core::ptr::write((logic + 0x1d3) as *mut u8, 6u8);
            plock(&GREYED)
                .get_or_insert_with(std::collections::HashMap::new)
                .insert(name.clone(), cur);
            CNT_GY_PAINT.fetch_add(1, Ordering::Relaxed);
            if cfg.debug {
                let n = DBG_GY.fetch_add(1, Ordering::Relaxed);
                if n < 30 {
                    config::dlog(&format!("grey#{n}: {name} 회색화(orig={cur})"));
                }
            }
        }
    } else {
        grey_rect_drop(&name);
        // 우리가 칠했던 것만, 그리고 지금도 3일 때만 원복(진짜 밴 보호).
        let orig = {
            let g = plock(&GREYED);
            g.as_ref().and_then(|m| m.get(&name).copied())
        };
        if let Some(o) = orig {
            if cur == 6 {
                core::ptr::write((logic + 0x1d3) as *mut u8, o);
                hide_cell_tooltip(node);
            }
            plock(&GREYED)
                .get_or_insert_with(std::collections::HashMap::new)
                .remove(&name);
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn commit_click_hook(
    ctrl: usize,
    view: usize,
    ctx: usize,
    side: usize,
    name_ptr: usize,
    name_len: usize,
    flag: usize,
) -> usize {
    // ⛔클릭 차단 비활성(2026-08-22 실사고): **코치 위임 픽도 같은 클라이언트 클릭 큐를
    //   통과**한다 — 여기서 막으면 코치가 고른 챔프가 커밋되지 않아 드래프트가 정지한다
    //   (실측: `click_block#0: spirit_caller` 직후 17/20 에서 hang).
    //   ★차단은 불필요하다: 회색(state=3) 셀은 게임의 호버 게이트가 이미 선택을 막는다
    //     (실측: 회색 셀 2회 클릭 → 픽 안 됨, 그때 CK 훅은 발화조차 안 함).
    //   이 훅은 이제 **유저 수동픽 마킹 전용**(CP 가 사람 픽을 안 건드리게).
    let _ = (&BLOCKLIST, &DBG_CK, &CNT_CK_BLOCK);
    // 통과한 클릭 = 유저가 직접 고른 것 → 커밋 훅이 건드리지 않게 마킹.
    if let Some(n) = read_str_at(name_ptr, name_len) {
        human_mark(&n);
    }
    let o = COMMITTER_ORIG.load(Ordering::Relaxed);
    if o == 0 {
        return 0;
    }
    let f: UiCommitFn = core::mem::transmute(o);
    f(ctrl, view, ctx, side, name_ptr, name_len, flag)
}

/// 콜사이트 rel32 리다이렉트 공통 설치.
unsafe fn install_callsite(
    cs_rva: usize,
    target_rva: usize,
    wrap: usize,
    stub_slot: &AtomicUsize,
    tag: &str,
) -> bool {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return false;
    }
    let cs = base + cs_rva;
    let target = base + target_rva;
    let mut c = [0u8; 5];
    core::ptr::copy_nonoverlapping(cs as *const u8, c.as_mut_ptr(), 5);
    if c[0] != 0xe8 {
        config::dlog(&format!("{tag}: 콜사이트 non-CALL({:02x})", c[0]));
        return false;
    }
    let rel = i32::from_le_bytes([c[1], c[2], c[3], c[4]]);
    if ((cs as i64) + 5 + rel as i64) as usize != target {
        config::dlog(&format!("{tag}: 콜사이트 tgt 불일치"));
        return false;
    }
    let stub = match stub_slot.load(Ordering::Relaxed) {
        0 => {
            let Some(s) = alloc_near(cs) else {
                config::dlog(&format!("{tag}: alloc_near 실패"));
                return false;
            };
            let mut sb = [0u8; 12];
            sb[0] = 0x48;
            sb[1] = 0xb8;
            sb[2..10].copy_from_slice(&wrap.to_le_bytes());
            sb[10] = 0xff;
            sb[11] = 0xe0;
            core::ptr::copy_nonoverlapping(sb.as_ptr(), s as *mut u8, 12);
            stub_slot.store(s, Ordering::SeqCst);
            s
        }
        s => s,
    };
    let nr = (stub as i64) - ((cs as i64) + 5);
    if nr.abs() > 0x7f00_0000 {
        config::dlog(&format!("{tag}: rel32 범위초과"));
        return false;
    }
    let mut patch = [0u8; 5];
    patch[0] = 0xe8;
    patch[1..5].copy_from_slice(&(nr as i32).to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(cs, 5, RWX, &mut old) == 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), cs as *mut u8, 5);
    VirtualProtect(cs, 5, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), cs, 5);
    true
}

/// Hook GY(회색화) + CK(클릭차단) 설치.
pub fn install_once_uiblock() {
    if INSTALL_STATE_GY.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block || !config::any_restricted() {
        INSTALL_STATE_GY.store(2, Ordering::Relaxed);
        INSTALL_STATE_CK.store(2, Ordering::Relaxed);
        return;
    }
    unsafe {
        if BASE.load(Ordering::Relaxed) == 0 {
            BASE.store(GetModuleHandleW(core::ptr::null()), Ordering::Relaxed);
        }
        let base = BASE.load(Ordering::Relaxed);
        CELLFN_ORIG.store(base + RVA_CELLFN, Ordering::SeqCst);
        COMMITTER_ORIG.store(base + RVA_COMMITTER, Ordering::SeqCst);
        let a = install_callsite(
            GY_CALLSITE,
            RVA_CELLFN,
            cell_hook as usize,
            &GY_STUB,
            "hookGY",
        );
        let b = install_callsite(
            CK_CALLSITE,
            RVA_COMMITTER,
            commit_click_hook as usize,
            &CK_STUB,
            "hookCK",
        );
        INSTALL_STATE_GY.store(if a { 1 } else { 2 }, Ordering::Relaxed);
        INSTALL_STATE_CK.store(if b { 1 } else { 2 }, Ordering::Relaxed);
        config::dlog(&format!(
            "hookGY(회색화 0x2553a16)={} hookCK(클릭차단 0x25508de)={}",
            if a { "OK" } else { "실패" },
            if b { "OK" } else { "실패" }
        ));
    }
}
