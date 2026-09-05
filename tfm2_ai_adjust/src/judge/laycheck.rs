//! ★런타임 layout 자기검사 — 첫 훅 진입(유효한 Holder 를 받은 첫 호출)에서 1회.
//!
//! 왜: 주간 패치의 진짜 위험은 RVA 가 아니라 **구조체 오프셋이 밀리는 것**이었다(0.5.8: sim +0x10 → 크래시, RVA 검사는 PASS).
//!     정적 도구(aidiff)는 함수 짝은 맞추지만 vtable(데이터)과 그 슬롯 구현체(leaf·패닉 Location 없음)는 못 짝짓는다.
//!     그런데 게임은 실행 중에 vt 를 손에 쥐고 있다 — 첫 호출에서 `[vt+slot]` 을 따라가 구현체 바이트를 읽으면
//!     "이 버전에서 tick 은 `[w+?]`" 를 게임 자신이 알려 준다. 그 값을 layout.rs 상수와 대조한다.
//!
//! 규칙: 하나라도 FAIL 이면 `live_mode()` 가 0 을 돌려 **모든 live 를 막는다**(검증 모드는 계속 — DIFF 로 어디가 틀렸는지 본다).
//! 산출: `judge_layout.txt` (항목별 PASS/FAIL·실측값·기대값·슬롯 타깃 RVA). 매 패치 첫 리플레이에서 이 파일부터 본다.
//!
//! 패턴 = (구현체 첫 0x60B 안에서 처음 나오는 opcode 접두 → 뒤따르는 disp8/disp32). 접두는 0.5.8 디스어셈에서 땄다(REPORT 02 09-06 절).
//!   패치로 코드젠이 바뀌어 접두가 안 잡히면 `NOPAT` 로 나온다 — 그건 "상수가 틀렸다" 가 아니라 "검사 갱신 필요" 다.
#![allow(dead_code)]
use crate::*;
use super::layout::*;
use std::sync::atomic::{AtomicU8, Ordering};

/// 0 = 미실행 · 1 = PASS · 2 = FAIL(live 차단) · 3 = 실행 불가(홀더/베이스 없음)
pub static STATE: AtomicU8 = AtomicU8::new(0);
#[inline] pub fn ok_for_live() -> bool { STATE.load(Ordering::Relaxed) != 2 }

pub struct Chk { pub slot: usize, pub name: &'static str, pub prefix: &'static [u8], pub disp: u8, pub expect: usize }
const C: &[Chk] = &[
    Chk { slot: 0x28,  name: "W_TICK",            prefix: &[0x48, 0x8b, 0x81],       disp: 4, expect: W_TICK },
    Chk { slot: 0x150, name: "W_ROSTER_REC_CNT",  prefix: &[0x4c, 0x8b, 0x81],       disp: 4, expect: W_ROSTER_REC_CNT },
    Chk { slot: 0x150, name: "W_ROSTER_REC_BASE", prefix: &[0x48, 0x8b, 0x81],       disp: 4, expect: W_ROSTER_REC_BASE },
    Chk { slot: 0x150, name: "REC_STRIDE",        prefix: &[0x49, 0x69, 0xc8],       disp: 4, expect: REC_STRIDE },
    Chk { slot: 0x150, name: "REC_ALIVE",         prefix: &[0x48, 0x83, 0xb8],       disp: 4, expect: REC_ALIVE },
    Chk { slot: 0x150, name: "REC_HANDLE",        prefix: &[0x48, 0x39, 0x90],       disp: 4, expect: REC_HANDLE },
    Chk { slot: 0xf8,  name: "W_L3_CNT",          prefix: &[0x4c, 0x3b, 0x81],       disp: 4, expect: W_L3_CNT },
    Chk { slot: 0xf8,  name: "W_L3_TBL",          prefix: &[0x48, 0x8b, 0x81],       disp: 4, expect: W_L3_TBL },
    Chk { slot: 0xf8,  name: "W_ENT_CNT",         prefix: &[0x48, 0x3b, 0x81],       disp: 4, expect: W_ENT_CNT },
    Chk { slot: 0xf8,  name: "ENT_STRIDE",        prefix: &[0x48, 0x69, 0xc0],       disp: 4, expect: ENT_STRIDE },
    Chk { slot: 0xf8,  name: "W_ENT_BASE",        prefix: &[0x48, 0x03, 0x81],       disp: 4, expect: W_ENT_BASE },
    Chk { slot: 0xf8,  name: "ENT_VIS_BASE",      prefix: &[0x48, 0x83, 0x7c, 0xc8], disp: 1, expect: ENT_VIS_BASE },
    Chk { slot: 0x1f0, name: "W_L3_CNT",          prefix: &[0x48, 0x3b, 0x91],       disp: 4, expect: W_L3_CNT },
    Chk { slot: 0x1f0, name: "W_L3_TBL",          prefix: &[0x48, 0x8b, 0x81],       disp: 4, expect: W_L3_TBL },
    Chk { slot: 0x1f0, name: "W_ENT_CNT",         prefix: &[0x48, 0x3b, 0x81],       disp: 4, expect: W_ENT_CNT },
    Chk { slot: 0x1f0, name: "ENT_STRIDE",        prefix: &[0x48, 0x69, 0xc0],       disp: 4, expect: ENT_STRIDE },
    Chk { slot: 0x1f0, name: "W_ENT_BASE",        prefix: &[0x48, 0x03, 0x81],       disp: 4, expect: W_ENT_BASE },
    Chk { slot: 0x1f0, name: "W_SINGLETON_TAG",   prefix: &[0x48, 0x8d, 0x41],       disp: 1, expect: W_SINGLETON_TAG },
    Chk { slot: 0x1f0, name: "W_SINGLETON_H",     prefix: &[0x48, 0x39, 0x91],       disp: 4, expect: W_SINGLETON_H },
    Chk { slot: 0xe8,  name: "W_CFG_FLAG",        prefix: &[0x0f, 0xb6, 0x81],       disp: 4, expect: W_CFG_FLAG },
];
const SCAN: usize = 0x60;

unsafe fn find_disp(tgt: usize, prefix: &[u8], disp: u8) -> Option<usize> {
    let mut buf = [0u8; SCAN + 8];
    for i in 0..buf.len() { buf[i] = rd_u8(tgt + i); }
    'outer: for i in 0..SCAN {
        for (j, &p) in prefix.iter().enumerate() { if buf[i + j] != p { continue 'outer; } }
        let k = i + prefix.len();
        return Some(if disp == 1 { buf[k] as usize } else { u32::from_le_bytes([buf[k], buf[k + 1], buf[k + 2], buf[k + 3]]) as usize });
    }
    None
}

/// 첫 유효 Holder 에서 1회. 반환 = PASS 여부(이미 실행됐으면 STATE 기준).
pub unsafe fn run_once(w: &super::world::World) -> bool {
    if STATE.load(Ordering::Relaxed) != 0 { return ok_for_live(); }
    let base = exe_base();
    if base == 0 || !ptr_ok(w.vt) { STATE.store(3, Ordering::Relaxed); return true; }
    let rva = |a: usize| if a > base && a - base < 0x8000000 { a - base } else { 0 };
    let mut out = format!("=== judge layout 자기검사 (게임 {}) vt_rva={:#x} ===\n", super::GAME_VER, rva(w.vt));
    let mut fail = 0u32; let mut nopat = 0u32;
    let tgt_of = |slot: usize| rd_u64(w.vt + slot).unwrap_or(0) as usize;
    for c in C {
        let t = tgt_of(c.slot);
        if !ptr_ok(t) { out.push_str(&format!("FAIL  slot+{:#x} {:<18} 타깃 없음\n", c.slot, c.name)); fail += 1; continue; }
        match find_disp(t, c.prefix, c.disp) {
            Some(v) if v == c.expect => out.push_str(&format!("PASS  slot+{:#x} {:<18} = {:#x}  (impl {:#x})\n", c.slot, c.name, v, rva(t))),
            Some(v) => { out.push_str(&format!("FAIL  slot+{:#x} {:<18} 실측 {:#x} ≠ layout {:#x}  (impl {:#x})\n", c.slot, c.name, v, c.expect, rva(t))); fail += 1; }
            None => { out.push_str(&format!("NOPAT slot+{:#x} {:<18} 접두 미검출 — 검사 패턴 갱신 필요 (impl {:#x})\n", c.slot, c.name, rva(t))); nopat += 1; }
        }
    }
    // vt+0x40: 구현 RVA 3종 중 하나여야 하고(포팅이 RVA 로 태그를 판정한다), lea rdx 의 disp 가 그 태그의 모드 데이터 오프셋이어야 한다.
    let t40 = tgt_of(0x40); let r40 = rva(t40);
    let exp = if r40 == VT40_IMPL_MOBA { Some(W_MODE_DATA_MOBA) } else if r40 == VT40_IMPL_TAG1 || r40 == VT40_IMPL_TAG2 { Some(W_MODE_DATA_OTHER) } else { None };
    match exp {
        None => { out.push_str(&format!("FAIL  slot+0x40 VT40_IMPL_*        실측 {:#x} 이 3종({:#x}/{:#x}/{:#x}) 밖 — 포팅의 모드 판정 무효\n", r40, VT40_IMPL_MOBA, VT40_IMPL_TAG1, VT40_IMPL_TAG2)); fail += 1; }
        Some(e) => match find_disp(t40, &[0x48, 0x8d, 0x91], 4) {
            Some(v) if v == e => out.push_str(&format!("PASS  slot+0x40 W_MODE_DATA        = {:#x}  (impl {:#x})\n", v, r40)),
            Some(v) => { out.push_str(&format!("FAIL  slot+0x40 W_MODE_DATA        실측 {:#x} ≠ layout {:#x}  (impl {:#x})\n", v, e, r40)); fail += 1; }
            None => { out.push_str(&format!("NOPAT slot+0x40 W_MODE_DATA        (impl {:#x})\n", r40)); nopat += 1; }
        },
    }
    let st = if fail > 0 { 2 } else { 1 };
    out.push_str(&format!("⟹ {} (FAIL {} · NOPAT {} · 항목 {}) — FAIL>0 이면 judge live 전부 차단(검증은 계속). NOPAT 는 패턴 갱신 대상.\n",
        if st == 1 { "PASS" } else { "FAIL" }, fail, nopat, C.len() + 1));
    if let Some(p) = pth("judge_layout.txt") { let _ = fs::write(p, &out); }
    STATE.store(st, Ordering::Relaxed);
    st == 1
}
