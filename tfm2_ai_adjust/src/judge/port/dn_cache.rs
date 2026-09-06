//! dn_cache — 틱 메모 캐시 `0xc87850`(hashbrown TLS, 키 rec+0x928, 에포크 (seed w+0xec90, tick w+0xec98)) 의 **순수 재현 + 메모 미러**(2026-09-06 20:55).
//!   정본 = REPORT RE\2026-09-06_dn_cache-틱메모-0xc87850-비트생산자-0xd3e660-0xd405d0-순수재현용-0.5.8.md
//!   반환 u32 = b0 | b1<<8 | b2<<16 :
//!     b0 (0xd3e660) = ∃적 챔프 e(슬롯0 있음): reach(e, 내 넥서스) ∨ ∃m∈내 미니언 reach(e, m) ; 아니면 (∃e reach(e, 나) ∧ b2)
//!     b1 (0xd3fe50) = dn_reach::reach (기존 순수 포팅)
//!     b2 (0xd405d0) = 넥서스·나 존재 ∧ ∃u∈적 유닛 리스트3(X+0x10/0x50/0x90+other*0x20): kind==1 ∧ +0x88==1 ∧ 공격목표(+0x90) ∈ {넥서스 핸들, 내 미니언 핸들}
//!   reach(e, t) = d2(e,t) <= (e.438 +w slot.10 +w (lv−1)*slot.18 +w (slot.flag==0 ? term(e) : 0) +w term(t) +w vt_e8(slot, e, t))²  (전부 wrapping u64)
//!   호출 caps p2 = [&seed, &tick, rec, holder]. 같은 에포크 안에서 rec.928 당 첫 호출 값이 고정(메모) — 미러로 재현.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::dn_reach;

const W_SEED_EC90: usize = 0xec90;

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] unsafe fn term(t: usize) -> Option<u64> {
    let a = rd_i32(t + ENT_F470)? as i64; let h = rd_u64(t + ENT_F680)?;
    Some(if a == 0 { h } else { ((a + 100).wrapping_mul(h as i64)) as u64 / 100 })
}
/// reach(e, t) — e 의 슬롯0(플래그 −1 아님을 호출자가 확인) 사거리가 t 에 닿는가
unsafe fn reach_et(e: usize, t: usize) -> Option<bool> {
    let slot = e + ENT_SLOT0;
    let (data, vt) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    let bonus = dn_reach::eff_e8(data, vt, e, t, 0)?;
    let ta = if rd_i32(slot + 0x30)? == 0 { term(e)? } else { 0 };
    let tt = term(t)?;
    let range = rd_u64(e + ENT_F438)?.wrapping_add(rd_u64(slot + 0x10)?).wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(slot + 0x18)?))
        .wrapping_add(ta).wrapping_add(tt).wrapping_add(bonus);
    let d2 = sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, rd_u64(t + ENT_X)?, rd_u64(t + ENT_Y)?);
    Some(d2 <= range.wrapping_mul(range))
}
/// 0xd405d0 == 1
pub unsafe fn bit16(rec: usize, x: usize) -> Option<bool> {
    let side = rd_u64(rec + P5_SIDE)?; if side > 1 { return None; }
    let n = rd_u64(x + X_NEXUS + (side as usize) * 8)? as usize; if n == 0 { return Some(false); }
    let lane = rd_u32(rec + P5_ROLE) as usize; if lane > 4 { return None; }
    let me = rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + lane * 8)? as usize; if me == 0 { return Some(false); }
    let (mp, ml) = (rd_u64(x + X_MINION_PTR + (side as usize) * 0x20)? as usize, rd_u64(x + X_MINION_LEN + (side as usize) * 0x20)?);
    if ml != 0 && !ptr_ok(mp) { return None; }
    let nh = rd_u64(n + ENT_HANDLE)?;
    let other = 1 - side;
    for i in 0..3usize {
        let ptr = rd_u64(x + X_LIST3_PTR[i] + (other as usize) * 0x20)? as usize; let len = rd_u64(x + X_LIST3_LEN[i] + (other as usize) * 0x20)?;
        if len == 0 { continue; } if !ptr_ok(ptr) { return None; }
        for j in 0..len.min(4096) as usize {
            let u = rd_u64(ptr + j * 8)? as usize; if !ptr_ok(u) { return None; }
            if rd_i32(u + ENT_KIND)? != 1 || rd_i32(u + ENT_F88)? != 1 { continue; }
            let th = rd_u64(u + ENT_TARGET_H)?;
            if th == nh { return Some(true); }
            for k in 0..ml.min(512) as usize { let m = rd_u64(mp + k * 8)? as usize; if m != 0 && rd_u64(m + ENT_HANDLE)? == th { return Some(true); } }
        }
    }
    Some(false)
}
/// 0xd3e660
pub unsafe fn bit0(rec: usize, x: usize) -> Option<bool> {
    let side = rd_u64(rec + P5_SIDE)?; if side > 1 { return None; }
    let n = rd_u64(x + X_NEXUS + (side as usize) * 8)? as usize; if n == 0 { return Some(false); }
    let (mp, ml) = (rd_u64(x + X_MINION_PTR + (side as usize) * 0x20)? as usize, rd_u64(x + X_MINION_LEN + (side as usize) * 0x20)?);
    if ml != 0 && !ptr_ok(mp) { return None; }
    let other = 1 - side;
    for r in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + r * 8)? as usize; if e == 0 { continue; }
        if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { continue; }
        if reach_et(e, n)? { return Some(true); }
        for k in 0..ml.min(512) as usize { let m = rd_u64(mp + k * 8)? as usize; if m == 0 { return None; } if reach_et(e, m)? { return Some(true); } }
    }
    let lane = rd_u32(rec + P5_ROLE) as usize; if lane > 4 { return None; }
    let me = rd_u64(x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + lane * 8)? as usize; if me == 0 { return Some(false); }
    let mut hit = false;
    for r in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + r * 8)? as usize; if e == 0 { continue; }
        if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { continue; }
        if reach_et(e, me)? { hit = true; break; }
    }
    if !hit { return Some(false); }
    bit16(rec, x)
}
/// 순수 3비트
pub unsafe fn bits(rec: usize, holder: usize) -> Option<u32> {
    let h = Holder::new(holder)?; let w = h.world()?;
    let b0 = bit0(rec, w.x)?; let b1 = dn_reach::reach(rec, holder)?; let b2 = bit16(rec, w.x)?;
    Some((b0 as u32) | ((b1 as u32) << 8) | ((b2 as u32) << 16))
}
thread_local! { static MEMO: std::cell::RefCell<((u64, u64), Vec<(u64, u32)>)> = const { std::cell::RefCell::new(((0, 0), Vec::new())) }; }
pub fn memo_reset() { MEMO.with(|c| { let mut m = c.borrow_mut(); m.0 = (0, 0); m.1.clear(); }); }
/// 0xc87850 계약: p2 = caps [&seed, &tick, rec, holder]
pub unsafe fn bits_memo(p2: usize) -> Option<u64> {
    if !ptr_ok(p2) { return None; }
    let (ps, pt) = (rd_u64(p2)? as usize, rd_u64(p2 + 8)? as usize); if !ptr_ok(ps) || !ptr_ok(pt) { return None; }
    let epoch = (rd_u64(ps)?, rd_u64(pt)?);
    let rec = rd_u64(p2 + 0x10)? as usize; let holder = rd_u64(p2 + 0x18)? as usize; if !ptr_ok(rec) || !ptr_ok(holder) { return None; }
    let key = rd_u64(rec + P5_MEMO_KEY)?;
    let hit = MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != epoch { m.0 = epoch; m.1.clear(); } m.1.iter().find(|e| e.0 == key).map(|e| e.1) });
    if let Some(v) = hit { return Some(v as u64); }
    let v = bits(rec, holder)?;
    MEMO.with(|c| { let mut m = c.borrow_mut(); if m.1.len() < 256 { m.1.push((key, v)); } });
    Some(v as u64)
}
pub unsafe fn diag(p2: usize) -> String {
    if !ptr_ok(p2) { return "bad".into(); }
    let rec = rd_u64(p2 + 0x10).unwrap_or(0) as usize; let holder = rd_u64(p2 + 0x18).unwrap_or(0) as usize;
    if !ptr_ok(rec) || !ptr_ok(holder) { return "bad2".into(); }
    let w = Holder::new(holder).and_then(|h| h.world());
    let hit = crate::judge::cap_dn_cache::HIT.load(std::sync::atomic::Ordering::Relaxed);
    let post = game_lookup(p2); let cell = game_cell();
    let cellinfo = cell.map(|c| format!("cell={:#x} seed={:#x} tick={} items={} mask={}", c, rd_u64(c + 8).unwrap_or(0), rd_u64(c + 0x10).unwrap_or(0), rd_u64(c + 0x30).unwrap_or(0), rd_u64(c + 0x20).unwrap_or(0))).unwrap_or("cell=None".into());
    let mml = w.as_ref().map(|w| rd_u64(w.x + X_MINION_LEN + (rd_u64(rec + P5_SIDE).unwrap_or(0) as usize) * 0x20).unwrap_or(999)).unwrap_or(998);
    let pre = PRE.with(|c| c.get());
    let rawx = rd_u64(holder).unwrap_or(0) as usize;
    let (l0, l1, nx0, nx1) = if ptr_ok(rawx) { (rd_u64(rawx + 0x148).unwrap_or(9), rd_u64(rawx + 0x168).unwrap_or(9), rd_u64(rawx + 0x170).unwrap_or(0), rd_u64(rawx + 0x178).unwrap_or(0)) } else { (8, 8, 0, 0) };
    let pres = format!("pre[found={} items={} epoch_ok={} mml={} cell_tick={} tick={}] rawx={:#x} len0={} len1={} nx0={:#x} nx1={:#x} b1_nogate={:?} rec930={:?}", pre[0], pre[1], pre[2], pre[3], pre[4], pre[5], rawx, l0, l1, nx0, nx1, dn_reach::reach_nogate(rec, holder), rd_u64(rec + 0x930));
    match w { Some(w) => format!("{} side={:?} lane={} b0={:?} b1={:?} b2={:?} key={:#x} mml={} | hits={} post={:?} {}", pres, rd_u64(rec + P5_SIDE), rd_u32(rec + P5_ROLE), bit0(rec, w.x), dn_reach::reach(rec, holder), bit16(rec, w.x), rd_u64(rec + P5_MEMO_KEY).unwrap_or(0), mml, hit, post, cellinfo), None => "no world".into() }
}

// ── 게임 TLS 메모 표 직접 조회: 접근자 0xd43130 = gs:[0x58][tls_idx*8] + 0x12f0 ; 셀 {+0 borrow, +8 seed, +0x10 tick, +0x18 ctrl, +0x20 mask, +0x28 growth, +0x30 items}
//    버킷 i = ctrl − (i+1)*16 : −0x10 키 u64, −8 b0, −7 b1, −6 b2. ctrl[i] < 0x80 = full.
const DNC_TLS_OFF: usize = 0x12f0;
unsafe fn game_cell() -> Option<usize> {
    let b = exe_base(); if b == 0 { return None; }
    let idx = rd_u32(b + CAMP_MEMO_TLS_IDX) as usize; if idx > 0x1000 { return None; }
    let teb: usize; core::arch::asm!("mov {}, gs:[0x58]", out(reg) teb, options(nostack, readonly, preserves_flags));
    if !ptr_ok(teb) { return None; }
    let blk = rd_u64(teb + idx * 8)? as usize; if !ptr_ok(blk) { return None; }
    Some(blk + DNC_TLS_OFF)
}
/// 게임 표에 (에포크 일치 ∧ 키 존재) 이면 Some(값), 아니면 None. 호출 **전**에 조회해야 "게임이 히트할지" 를 안다.
pub unsafe fn game_lookup(p2: usize) -> Option<u32> {
    if !ptr_ok(p2) { return None; }
    let (ps, pt) = (rd_u64(p2)? as usize, rd_u64(p2 + 8)? as usize); if !ptr_ok(ps) || !ptr_ok(pt) { return None; }
    let rec = rd_u64(p2 + 0x10)? as usize; if !ptr_ok(rec) { return None; }
    let key = rd_u64(rec + P5_MEMO_KEY)?;
    let cell = game_cell()?;
    if rd_u64(cell + 8)? != rd_u64(ps)? || rd_u64(cell + 0x10)? != rd_u64(pt)? { return None; }
    let items = rd_u64(cell + 0x30)?; if items == 0 { return None; }
    let ctrl = rd_u64(cell + 0x18)? as usize; let mask = rd_u64(cell + 0x20)?; if !ptr_ok(ctrl) || mask > 0x10000 { return None; }
    for i in 0..=mask as usize {
        if rd_u8(ctrl + i) >= 0x80 { continue; }
        let bk = ctrl - (i + 1) * 16;
        if rd_u64(bk)? == key { return Some(rd_u8(bk + 8) as u32 | (rd_u8(bk + 9) as u32) << 8 | (rd_u8(bk + 10) as u32) << 16); }
    }
    None
}
/// 검증용: pre = 호출 전 게임 표 조회 결과(0xff_ffff 초과 값 = 없음). 게임이 히트하면 그 값을 미러에도 넣고 그대로 반환(생산자 대조 대상 아님), 미스면 순수 계산(대조 대상).
pub unsafe fn bits_verify(p2: usize, pre: u64) -> Option<u64> {
    if pre <= 0xff_ffff {
        // 게임 히트 → 미러 동기화
        if ptr_ok(p2) { if let (Some(ps), Some(pt), Some(rec)) = (rd_u64(p2), rd_u64(p2 + 8), rd_u64(p2 + 0x10)) {
            let (ps, pt, rec) = (ps as usize, pt as usize, rec as usize);
            if ptr_ok(ps) && ptr_ok(pt) && ptr_ok(rec) { if let (Some(sv), Some(tv), Some(key)) = (rd_u64(ps), rd_u64(pt), rd_u64(rec + P5_MEMO_KEY)) {
                MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != (sv, tv) { m.0 = (sv, tv); m.1.clear(); } if !m.1.iter().any(|e| e.0 == key) && m.1.len() < 256 { m.1.push((key, pre as u32)); } }); } } } }
        return Some(pre);
    }
    bits_memo(p2)
}
thread_local! { pub static PRE: std::cell::Cell<[u64; 6]> = const { std::cell::Cell::new([0; 6]) }; }
/// 호출 전 스냅샷: [0]=found(1/0) [1]=items_pre [2]=epoch_match [3]=mml_pre [4]=cell_tick_pre [5]=tick_now
pub unsafe fn pre_lookup(p2: usize) -> u64 {
    let r = game_lookup(p2);
    let mut snap = [0u64; 6];
    if ptr_ok(p2) {
        if let Some(cell) = game_cell() {
            snap[1] = rd_u64(cell + 0x30).unwrap_or(0); snap[4] = rd_u64(cell + 0x10).unwrap_or(0);
            let (ps, pt) = (rd_u64(p2).unwrap_or(0) as usize, rd_u64(p2 + 8).unwrap_or(0) as usize);
            if ptr_ok(ps) && ptr_ok(pt) { snap[5] = rd_u64(pt).unwrap_or(0); snap[2] = (rd_u64(cell + 8) == rd_u64(ps) && rd_u64(cell + 0x10) == rd_u64(pt)) as u64; }
        }
        let rec = rd_u64(p2 + 0x10).unwrap_or(0) as usize; let holder = rd_u64(p2 + 0x18).unwrap_or(0) as usize;
        if ptr_ok(rec) && ptr_ok(holder) { if let Some(w) = Holder::new(holder).and_then(|h| h.world()) { let side = rd_u64(rec + P5_SIDE).unwrap_or(0) as usize; if side < 2 { snap[3] = rd_u64(w.x + X_MINION_LEN + side * 0x20).unwrap_or(777); } } }
    }
    snap[0] = r.is_some() as u64;
    PRE.with(|c| c.set(snap));
    match r { Some(v) => v as u64, None => u64::MAX }
}

