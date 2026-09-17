//! legacy_assign — (선택 축) AI 포지션 배정 마스크 detour = 클래식 hookA 이식 (cfg `ai_assign_mask`, 기본 1).
//! 왜 남기나: stable 엔 "AI 팀이 챔프를 어느 포지션에 앉히는가"(SGD 스코어러 마스크)에 닿는 API 가 없다.
//!   decide_pick 은 **무엇을 고르는가**만 지배 → 탑 전용으로 지정한 챔프가 AI 팀에서 미드로 갈 수 있다.
//!   hookA = champ→eligible-positions 비트마스크 산출기(`RVA_POS_MASK`, 0.6.0 재핀 ✅ 매니페스트)의 반환값 하위 5비트에
//!   우리 마스크를 AND(공집합이면 우리 마스크로 강제). 인자 rdx = 모델 인덱스 → `draft.rs` 가 ctx.champion_briefs() 로 게시한
//!   모델 이름표로 조회(클래식 v0.6.1 의 finalize 캡처 대체 — 인덱스 공간 동일).
//! ⚠클래식 cprod(서버 스왑 order 재작성)는 이식하지 않았다 — DecisionRecord/rmi 스냅샷 오프셋(0.5.8 채록)이 0.6.0 미검증이라
//!   raw write 위험. 필요하면 ghidra-re 로 재규명 후 별도 축.
//! 안전: 프롤로그 12B 일치 또는 외부훅(48 b8 … ff e0) 체인일 때만 설치, detour 본문 catch_unwind, raw read 없음.
use crate::config::{self, MASK_ALL};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const RVA_POS_MASK: usize = 0x13796a0; // 0.6.0 (MIG\manifest\tfm2_champ_pos_lock.json)
const PROL_POS_MASK: [u8; 12] = [0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53];

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

static TRAMP: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0); // 0 미설치 1 설치 2 영구실패
pub static CNT_FIRE: AtomicU64 = AtomicU64::new(0);
pub static CNT_ADJ: AtomicU64 = AtomicU64::new(0);
static MODEL_NAMES: Mutex<Option<Arc<Vec<String>>>> = Mutex::new(None);

/// draft.rs 가 ctx.champion_briefs() 이름(소문자, 모델 인덱스 순)을 게시.
pub fn publish_model_names(names: Vec<String>) {
    let mut g = MODEL_NAMES.lock().unwrap_or_else(|e| e.into_inner());
    if g.as_ref().map(|v| **v == names).unwrap_or(false) { return; }
    config::dlog(&format!("모델 이름표 게시: {}종", names.len()));
    *g = Some(Arc::new(names));
}
pub fn model_len() -> usize { MODEL_NAMES.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|v| v.len()).unwrap_or(0) }

extern "C" fn pos_mask_detour(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let tramp = TRAMP.load(Ordering::Relaxed);
    if tramp == 0 { return 0; }
    let orig: extern "C" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(tramp) };
    let ret = orig(rcx, rdx, r8, r9);
    CNT_FIRE.fetch_add(1, Ordering::Relaxed);
    let adj = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let cfg = config::get();
        if !cfg.enabled || !cfg.ai_assign_mask { return None; }
        let names = MODEL_NAMES.lock().unwrap_or_else(|e| e.into_inner()).clone()?;
        let name = names.get(rdx)?;
        let m = crate::mask_of(name);
        if m == MASK_ALL || m == 0 { return None; }
        let game = (ret & 0x1f) as u8;
        let and = game & m;
        let newm = if and != 0 { and } else { m };
        if newm == game { return None; }
        CNT_ADJ.fetch_add(1, Ordering::Relaxed);
        Some((ret & !0x1fusize) | newm as usize)
    }));
    match adj { Ok(Some(v)) => v, _ => ret }
}

unsafe fn build_tramp(fn_addr: usize) -> Result<usize, &'static str> {
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL_POS_MASK { return Err("prologue mismatch"); }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&cur);
    if !chained { s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes()); s.extend_from_slice(&[0x41, 0xff, 0xe3]); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    Ok(stub)
}
unsafe fn write_entry_patch(fn_addr: usize, repl: usize) -> Result<(), &'static str> {
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&repl.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

/// post_update 매 프레임 호출(설치 후 no-op). 제한이 하나도 없으면 설치 보류(세이브 로드 대기 — 영구실패 아님).
pub fn install_once() {
    if INSTALL_STATE.load(Ordering::Relaxed) != 0 { return; }
    let cfg = config::get();
    if !cfg.enabled || !cfg.ai_assign_mask { INSTALL_STATE.store(2, Ordering::Relaxed); return; }
    if !config::any_restricted() || model_len() == 0 { return; }
    unsafe {
        let base = GetModuleHandleW(core::ptr::null());
        if base == 0 { INSTALL_STATE.store(2, Ordering::Relaxed); return; }
        let fn_addr = base + RVA_POS_MASK;
        let r = build_tramp(fn_addr).and_then(|stub| { TRAMP.store(stub, Ordering::SeqCst); write_entry_patch(fn_addr, pos_mask_detour as *const () as usize) });
        match r {
            Ok(()) => { INSTALL_STATE.store(1, Ordering::Relaxed); config::dlog(&format!("hookA(pos_mask) 설치 OK @{:#x}", fn_addr)); }
            Err(e) => { INSTALL_STATE.store(2, Ordering::Relaxed); config::dlog(&format!("hookA(pos_mask) 설치 실패: {e} (RVA stale?) — AI 배정 마스크 없이 진행")); }
        }
    }
}
