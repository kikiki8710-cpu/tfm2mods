//! swap_confirm_hook — 스왑 확정 클릭 핸들러(`main.swap.bottom.confirm`) 진입 detour: 게이트가 "위반" 이면 원본을 호출하지 않고 반환(확정 무효).
//! 근거 RE: `mods_report/tfm2_champ_pos_lock/RE/2026-09-18_0.6.0-스왑단계-씬레이아웃-함수RVA.md` — 0.6.0 RVA 0x1fc5ce0(검증 완료), 진입 12B = push×8,
//!   rcx = 클로저 상태(+0xb8/+0xc0 team id), rdx = UI 이벤트 ctx. 클래식 `swap_confirm_detour`(0.5.8 `_classic_058\src\hooks.rs:3103`) 의 stable 이식.
//! 왜 훅인가: 09-18 실측 — 게임 버튼에 `disable: true` 를 set_properties 해도 **클릭이 그대로 통과**(우리 카드와 달리 게임 핸들러가 disable 을 안 본다).
//! 안전: detour 는 원자 읽기 + 카운터만(패닉 경로 없음) · 프롤로그 검증 실패 = 미설치(fail-safe: 게이트는 툴팁만) · 체인 훅 대응(진입부가 외부 `movabs rax; jmp rax` 면 그 스텁으로 점프).
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

pub const RVA_SWAP_CONFIRM: usize = 0x1fc5ce0; // 0.6.0
const PROL: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];

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
/// true 면 확정 클릭을 삼킨다(swap_gate 가 매프레임 갱신).
pub static BLOCK: AtomicBool = AtomicBool::new(false);
pub static CNT_FIRE: AtomicU64 = AtomicU64::new(0);
pub static CNT_BLOCKED: AtomicU64 = AtomicU64::new(0);

extern "C" fn detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    CNT_FIRE.fetch_add(1, Ordering::Relaxed);
    if BLOCK.load(Ordering::Relaxed) { CNT_BLOCKED.fetch_add(1, Ordering::Relaxed); return 0; }
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
    s.extend_from_slice(&cur); // 원본 12B(push 들) 또는 외부 훅 12B(= 외부 스텁으로 점프)
    if !chained { s.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes()); } // jmp qword [rip+0]
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
    let fn_addr = base + RVA_SWAP_CONFIRM;
    Some(match unsafe { install(fn_addr) } {
        Ok(chained) => { INSTALL_STATE.store(1, Ordering::Relaxed); format!("swap_confirm 훅 설치 OK @{:#x} chained={}", fn_addr, chained) }
        Err(e) => { INSTALL_STATE.store(2, Ordering::Relaxed); format!("swap_confirm 훅 설치 실패: {e} (RVA stale?) — 확정 게이트는 툴팁만") }
    })
}
