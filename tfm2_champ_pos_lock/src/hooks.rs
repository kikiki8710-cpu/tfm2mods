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

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::config;
use crate::config::MASK_ALL;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}
const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;

// ── 0.5.6 RVA (패치 시 재핀 대상 — 이 파일이 단일 수정점) ──────────────────
/// champ→eligible-positions 비트마스크 산출기 (0.5.5 0x1294180 → 0.5.6 재핀·프롤로그 동일)
const RVA_POS_MASK: usize = 0x2e739e0;
/// 프롤로그 12B = push r15/r14/r13/r12/rsi/rdi/rbp/rbx … (전부 1~2B push — 12B 경계 클린)
const PROL_POS_MASK: [u8; 12] = [
    0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53,
];

// Hook C (유저 픽 차단 — 피어리스 픽불가 차용, RE 2026-08-20 4차):
/// contains 헬퍼 `bool al = contains(rcx=&String{cap,ptr,len}, rdx=list.ptr, r8=list.len)`
/// (0.5.5 0x943440 → 0.5.6 재핀·프롤로그 동일)
const RVA_CONTAINS: usize = 0xb31840;
/// contains 프롤로그 14B(사이트 검증용 — 함수 자체는 패치하지 않음)
const PROL_CONTAINS: [u8; 14] = [
    0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xEC, 0x20, 0x4D, 0x85, 0xC0,
];
/// slot_widget(0x1971b00) 내부 contains E8 콜사이트 2곳 — 피어리스 회색+클릭게이트 지배점.
/// ⚠커밋기(0x12156b0) 내 4콜은 일부러 안 건드림: AI가 fail-open으로 위반 픽을 냈을 때
/// 서버 커밋이 거부되면 턴이 막힐 수 있다 — 차단은 UI(클릭 전)에서만.
const SITES_CONTAINS: [usize; 2] = [0x24dd7ae, 0x24dd7c7];

// Hook D (밴픽 씬 ptr 캡처 — RE 2026-08-20 5차):
/// slot_widget = banpick_champion_slot 렌더러. 씬 ptr = 스택 인자 arg15 = 진입 시 [rsp+0x78].
const RVA_SLOT_WIDGET: usize = 0x24d7640;
/// 프롤로그 12B push열(그 뒤 mov eax+call chkstk는 원위치 실행 — resume = fn+12)
const PROL_SLOT_WIDGET: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP_POS_MASK: AtomicUsize = AtomicUsize::new(0);
static CONTAINS_FN: AtomicUsize = AtomicUsize::new(0);

pub static CNT_MASK_CALL: AtomicU64 = AtomicU64::new(0);
pub static CNT_MASK_ADJ: AtomicU64 = AtomicU64::new(0);
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
    // 교정 — detour 본문은 절대 unwind 금지
    let adj = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_assign_mask {
            return None;
        }
        let masks = crate::masks()?;
        let m = *masks.get(rdx)?;
        if m == MASK_ALL {
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
    let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
    let s = core::str::from_utf8(bytes).ok()?;
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

#[inline]
fn ptr_ok(a: usize) -> bool {
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
    let p = ru64(scene + off) as usize;
    let n = ru64(scene + off + 8) as usize;
    if n == 0 {
        return Some(Vec::new());
    }
    if !ptr_ok(p) || n > 16 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let e = p + i * 0x18;
        let sp = ru64(e + 8) as usize;
        let sl = ru64(e + 0x10) as usize;
        if !ptr_ok(sp) || sl == 0 || sl > 64 {
            return None;
        }
        let bytes = core::slice::from_raw_parts(sp as *const u8, sl);
        out.push(core::str::from_utf8(bytes).ok()?.to_ascii_lowercase());
    }
    Some(out)
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

/// Hook C 설치 (post_update 1회) — 유저 픽 차단용 콜사이트 교체.
pub fn install_once_c() {
    if INSTALL_STATE_C.load(Ordering::Relaxed) != 0 {
        return;
    }
    let cfg = config::get();
    if !cfg.enabled || !cfg.user_pick_block || !config::any_restricted() {
        INSTALL_STATE_C.store(2, Ordering::Relaxed);
        return;
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
        // Hook D는 C가 성공했을 때만 의미 있음(C의 blocklist에 상태 공급)
        if INSTALL_STATE_C.load(Ordering::Relaxed) == 1 {
            match install_scene_capture() {
                Ok(()) => {
                    INSTALL_STATE_D.store(1, Ordering::Relaxed);
                    config::dlog("hookD(scene capture) 설치 OK");
                }
                Err(e) => {
                    INSTALL_STATE_D.store(2, Ordering::Relaxed);
                    config::dlog(&format!("hookD 설치 실패: {e} — 유저 픽 차단 비활성(blocklist 미게시)"));
                }
            }
        } else {
            INSTALL_STATE_D.store(2, Ordering::Relaxed);
        }
    }
}
