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

// ── 0.5.5 RVA (패치 시 재핀 대상 — 이 파일이 단일 수정점) ──────────────────
/// champ→eligible-positions 비트마스크 산출기 (RE 2026-08-19 확정·"강한 추정" 후계)
const RVA_POS_MASK: usize = 0x1294180;
/// 프롤로그 12B = push r15/r14/r13/r12/rsi/rdi/rbp/rbx … (전부 1~2B push — 12B 경계 클린)
const PROL_POS_MASK: [u8; 12] = [
    0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53,
];

static BASE: AtomicUsize = AtomicUsize::new(0);
static TRAMP_POS_MASK: AtomicUsize = AtomicUsize::new(0);

pub static CNT_MASK_CALL: AtomicU64 = AtomicU64::new(0);
pub static CNT_MASK_ADJ: AtomicU64 = AtomicU64::new(0);

/// 설치 결과(1회 기록): 0=미시도 1=OK 2=실패
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0);

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
        let masks = crate::MASKS.get()?;
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
    if !cfg.enabled || !cfg.ai_assign_mask || cfg.locks.is_empty() {
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
