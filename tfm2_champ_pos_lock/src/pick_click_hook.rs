//! pick_click_hook — 밴픽 챔피언 카드 클릭 핸들러 진입 detour: 포지션 제한으로 회색 처리된 챔프 클릭을 삼킨다(픽 패킷 미전송).
//! 왜 훅인가: 09-23 유저 제보 "회색으로 된 거 클릭하면 골라짐" — 0.6.x 게임 노드는 런타임 `disabled: true` 를 안 보고(교훈 24),
//!   덮개(pl_block)도 클릭을 흡수하지 못한다. stable API 엔 클릭 소비 수단이 없다 ⟹ 스왑 확정(swap_confirm_hook)과 같은 RVA detour.
//! 근거 RE(0.6.1, 2026-09-24): ClientPacket 변형 순서 `AthleteSelect, BanpickSelect, BanpickDelegate, BanpickTimerPause, StrategySelect, SwapDone`
//!   에서 SwapDone = 0x27(스왑 확정 핸들러 0x2921bc0 에서 확인) ⟹ BanpickSelect = 0x23. 0x740B 패킷 alloc 뒤 `mov qword [rcx], 0x23` 을 쓰는
//!   유일한 함수 = **0x2921300**(pdata 0x2921300..0x2921a12, 진입 12B = push×8 · 스왑 확정 핸들러 바로 앞).
//!   rdx = UI 이벤트 ctx, **[rdx+0x28]/[rdx+0x30] = 클릭 경로 문자열(ptr,len)** — 헬퍼 0x2455780 이 접두 "champions.contents."(0x13B) 뒤
//!   세그먼트를 챔프 id 로 뽑는다. 원본의 가드 거절 경로(이미 픽/밴됨·잠금 등)는 `al = 1` 로 반환 ⟹ 삼킬 때도 1 을 돌려준다.
//! 안전: detour 는 try_read + 문자열 비교만(패닉 경로 없음) · 락 경합이면 통과(fail-open) · 프롤로그 불일치 = 미설치(회색 표시만 유지).
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::RwLock;

pub const RVA_PICK_CLICK: usize = 0x2921300; // 0.6.1
const PROL: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
const PREFIX: &[u8] = b"champions.contents.";

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}

static TRAMP: AtomicUsize = AtomicUsize::new(0);
/// 0 미설치 · 1 설치 · 2 영구 실패
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0);
/// 지금 클릭을 삼킬 챔프 id(소문자). ui_block 이 차단 집합이 바뀔 때 갱신(빈 집합 = 전부 통과).
pub static BLOCK_SET: RwLock<Option<HashSet<String>>> = RwLock::new(None);
pub static CNT_FIRE: AtomicU64 = AtomicU64::new(0);
pub static CNT_BLOCKED: AtomicU64 = AtomicU64::new(0);
/// 마지막으로 삼킨 챔프(툴팁용) — ui_block 이 가져간다.
pub static LAST_BLOCKED: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

pub fn set_block(set: &HashSet<String>) {
    if let Ok(mut g) = BLOCK_SET.write() { *g = if set.is_empty() { None } else { Some(set.clone()) }; }
}

/// 클릭 경로에서 챔프 id(소문자) 추출: "...champions.contents.<id>[.…]"
unsafe fn champ_of(rdx: usize) -> Option<String> {
    if rdx == 0 { return None; }
    let ptr = *((rdx + 0x28) as *const usize);
    let len = *((rdx + 0x30) as *const usize);
    if ptr == 0 || len <= PREFIX.len() || len > 512 { return None; }
    let s = core::slice::from_raw_parts(ptr as *const u8, len);
    let at = s.windows(PREFIX.len()).position(|w| w == PREFIX)? + PREFIX.len();
    let rest = &s[at..];
    let id = &rest[..rest.iter().position(|&c| c == b'.').unwrap_or(rest.len())];
    if id.is_empty() { return None; }
    Some(String::from_utf8_lossy(id).to_ascii_lowercase())
}

extern "C" fn detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    CNT_FIRE.fetch_add(1, Ordering::Relaxed);
    let blocked = match BLOCK_SET.try_read() {
        Ok(g) => match g.as_ref() {
            Some(set) => unsafe { champ_of(rdx) }.filter(|id| set.contains(id)),
            None => None,
        },
        Err(_) => None,
    };
    if let Some(id) = blocked {
        CNT_BLOCKED.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut g) = LAST_BLOCKED.try_lock() { *g = Some(id); }
        return 1; // 원본 가드 거절 경로와 같은 반환(al = 1)
    }
    let tramp = TRAMP.load(Ordering::Relaxed);
    if tramp == 0 { return 0; }
    let orig: extern "C" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(tramp) };
    orig(rcx, rdx, r8, r9)
}

unsafe fn install(fn_addr: usize) -> Result<bool, &'static str> {
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL { return Err("prologue mismatch"); }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&cur);
    if !chained { s.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes()); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::SeqCst);
    let repl = detour as extern "C" fn(usize, usize, usize, usize) -> usize as *const () as usize;
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&repl.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(chained)
}

/// 1회 설치(실패는 영구). 반환 = 로그 문자열.
pub fn install_once() -> Option<String> {
    if INSTALL_STATE.load(Ordering::Relaxed) != 0 { return None; }
    let base = unsafe { GetModuleHandleW(core::ptr::null()) };
    if base == 0 { return None; }
    let fn_addr = base + RVA_PICK_CLICK;
    Some(match unsafe { install(fn_addr) } {
        Ok(chained) => { INSTALL_STATE.store(1, Ordering::Relaxed); format!("pick_click 훅 설치 OK @{:#x} chained={}", fn_addr, chained) }
        Err(e) => { INSTALL_STATE.store(2, Ordering::Relaxed); format!("pick_click 훅 설치 실패: {e} (RVA stale?) — 회색 표시만") }
    })
}
