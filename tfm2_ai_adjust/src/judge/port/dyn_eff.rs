//! dyn_eff — 이펙트(`Arc<dyn Effect>`, vt 291종)·프로바이더(`Box<dyn DataAction>`, vt 210종) 슬롯의 **구현 RVA 디스패치** 재현.
//!   규칙: 구현체 RVA(= `[vt+slot] − exe_base`)로 갈라 재현한다. 미재현 RVA 는 `None`(NA) 을 돌려주고 `unseen` 표에 (slot, rva, 횟수)를 남긴다
//!   → 리플레이 한 번이면 "실제로 불리는 구현" 목록이 `judge_dyn.txt` 에 나오고, 그것부터 포팅한다(291×5 슬롯 전부를 미리 옮기지 않는다).
//!   구현체 분포(0.5.8 정적 열거, scratchpad dyn_families.txt): +0x28 39종(0 ×152) · +0x30 1종(항상 None) · +0x38 12종(0 ×194) · +0x40 15종(0 ×197) · +0xa0 26종(None ×171) · 프로바이더 +0x90 19종.
//! ⚠페이로드 주소: Arc<dyn> 은 `data + ((align−1) & !0xf) + 0x10`(align=[vt+0x10]), Box<dyn>(프로바이더)은 data 그대로.
#![allow(dead_code)]
use crate::*;
use super::super::layout::*;
use std::sync::atomic::{AtomicU64, Ordering};

// ── 미확인 구현 RVA 표(락 없음: 64칸 선형탐색, 키 = slot<<32 | rva) ──
const N: usize = 64;
static KEYS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
static CNTS: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
pub fn unseen(slot: u32, rva: usize) {
    let key = ((slot as u64) << 32) | (rva as u64 & 0xffff_ffff);
    for i in 0..N {
        let k = KEYS[i].load(Ordering::Relaxed);
        if k == key { CNTS[i].fetch_add(1, Ordering::Relaxed); return; }
        if k == 0 { if KEYS[i].compare_exchange(0, key, Ordering::Relaxed, Ordering::Relaxed).is_ok() { CNTS[i].fetch_add(1, Ordering::Relaxed); return; } }
    }
}
pub fn unseen_report() -> String {
    let mut v: Vec<(u32, usize, u64)> = (0..N).filter_map(|i| { let k = KEYS[i].load(Ordering::Relaxed); if k == 0 { None } else { Some(((k >> 32) as u32, (k & 0xffff_ffff) as usize, CNTS[i].load(Ordering::Relaxed))) } }).collect();
    v.sort_by(|a, b| b.2.cmp(&a.2));
    let mut s = String::from("=== 미재현 dyn 구현체(slot, impl RVA, 호출 수) — 많은 순으로 포팅 ===\n");
    for (slot, rva, c) in v { s.push_str(&format!("slot+{:#x} impl {:#x} x{}\n", slot, rva, c)); }
    s
}

#[inline] pub unsafe fn impl_rva(vt: usize, slot: usize) -> Option<usize> {
    let b = exe_base(); if b == 0 || !ptr_ok(vt) { return None; }
    let t = rd_u64(vt + slot)? as usize; if t <= b || t - b > 0x8000000 { return None; }
    Some(t - b)
}
#[inline] pub unsafe fn arc_payload(data: usize, vt: usize) -> Option<usize> {
    let align = rd_u64(vt + 0x10)?; Some(data.wrapping_add(((align.wrapping_sub(1)) & !0xfu64) as usize).wrapping_add(0x10))
}

/// effect `+0x28` 피해 (rax=물리, rdx=마법). 인자 (payload, W, attacker, ENT_VT) — 재현본은 attacker 만 필요.
pub unsafe fn eff28_damage(data: usize, vt: usize, _att: usize) -> Option<(u64, u64)> {
    let rva = impl_rva(vt, 0x28)?; let me = arc_payload(data, vt)?;
    match rva {
        EFF28_ZERO => Some((0, 0)),
        EFF28_PAIR_RAW => Some((rd_u64(me)?, rd_u64(me + 8)?)),
        EFF28_PAIR_BYKIND => { let v = rd_u64(me)?; if rd_i32(me + 8)? == 1 { Some((0, v)) } else { Some((v, 0)) } }
        _ => { unseen(0x28, rva); None }
    }
}
/// effect `+0x38` 최대체력 비율 피해(%)
pub unsafe fn eff38_pct(data: usize, vt: usize, _att: usize) -> Option<u64> {
    let rva = impl_rva(vt, 0x38)?; let me = arc_payload(data, vt)?;
    match rva {
        EFF38_ZERO => Some(0),
        EFF38_GET28 => rd_u64(me + 0x28),
        _ => { unseen(0x38, rva); None }
    }
}
/// effect `+0x40` 회복량 추정
pub unsafe fn eff40_heal(data: usize, vt: usize, _ent: usize) -> Option<u64> {
    let rva = impl_rva(vt, 0x40)?;
    match rva {
        EFF40_ZERO => Some(0),
        _ => { unseen(0x40, rva); None }
    }
}
/// effect `+0xa0` BuffState sret — 핸들러가 읽는 두 필드만: (+0x48 type i32(−1=None), +0x80 vamp i32)
pub unsafe fn effa0_buff(data: usize, vt: usize, _ent: usize) -> Option<(i32, i32)> {
    let rva = impl_rva(vt, 0xa0)?;
    match rva {
        EFFA0_NONE => Some((-1, 0)),
        _ => { unseen(0xa0, rva); None }
    }
}
/// 프로바이더(Box<dyn DataAction>) `+0x90` cooltime — 구현체 바이트를 해석: `48 8b 41 K c3` / `48 8b 81 K32 c3` = [data+K], `31 c0 c3` = 0.
pub unsafe fn prov90_cooltime(data: usize, vt: usize, _ent: usize) -> Option<u64> {
    let rva = impl_rva(vt, 0x90)?; let b = exe_base(); let t = b + rva;
    let (b0, b1, b2) = (rd_u8(t), rd_u8(t + 1), rd_u8(t + 2));
    if b0 == 0x31 && b1 == 0xc0 && b2 == 0xc3 { return Some(0); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x41 && rd_u8(t + 4) == 0xc3 { return rd_u64(data + rd_u8(t + 3) as usize); }
    if b0 == 0x48 && b1 == 0x8b && b2 == 0x81 && rd_u8(t + 7) == 0xc3 { return rd_u64(data + rd_u32(t + 3) as usize); }
    unseen(0x90, rva); None
}
