//! combat_score — `0xd5bbf0`(action_score.rs:982~1517, 19,586B) 전투행동 점수의 **순수 포팅**(2026-09-07 01:0x~).
//!   정본 = `RE\2026-09-06_action_score-전투행동점수-0xd5bbf0-구조RE-0.5.8.md`(S0~S15 · 콜리 30종 판정표).
//!   계약: `(mode, prof, rec, ctx{W,sim,agents}, bb, sp(data,vt), slot, tgt, p9) -> i64`
//!         = `tower_support + risk_neg + pos_term + main` (조기반환 3종: 특수형 / −9,999,999 하드리젝트 / 처형 100+v).
//!   미포팅 콜리는 `na(tag)` 로 집계만 하고 None(=NA) — 리플레이 1판이면 어느 경로가 실제로 쓰이는지 빈도로 나온다.
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::passive_jungle::estimate_damage;
use super::as_callees::{isqrt_fast, pct_c, kind_pred};
use super::action_score::threat_sum;
use super::dn_cache;

const REC_SIDE: usize = 0x930; const REC_ROLE_O: usize = 0x9c0; const REC_SEED2: usize = 0x928;
const REC_464: usize = 0x464; const REC_208: usize = 0x208; const REC_210: usize = 0x210; const REC_448: usize = 0x448;
const BB_R: usize = 0x918; const BB_998: usize = 0x998; const BB_9A0: usize = 0x9a0; const BB_988: usize = 0x988; const BB_9B0: usize = 0x9b0;
const BB_1500: usize = 0x1500;
const BB_ALLY_PTR: usize = 0x14b8; const BB_ALLY_LEN: usize = 0x14d0;
const BB_ENEMY_PTR: usize = 0x14d8; const BB_ENEMY_LEN: usize = 0x14f0;
const RT_KILL: usize = 0x158; const RT_170: usize = 0x170; const RT_HANDLE: usize = 0x58;
const ENT_KIND: usize = 0x68; const ENT_488: usize = 0x488; const ENT_4C0: usize = 0x4c0;
const SLOT_BASE: usize = 0x10; const SLOT_PERLV: usize = 0x18; const SLOT_FLAG: usize = 0x30;
const CFG_8A8: usize = 0x8a8; const CFG_13F8: usize = 0x13f8;
const STRUCT_OFFS: [usize; 6] = [0x180, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0];

// ── 미포팅 콜리 집계 ─────────────────────────────────────────────────────────────────────
use std::sync::atomic::{AtomicU64, Ordering};
static NA_KEYS: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
static NA_CNTS: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
fn na(tag: u64) -> Option<i64> {
    for i in 0..32 {
        let k = NA_KEYS[i].load(Ordering::Relaxed);
        if k == tag { NA_CNTS[i].fetch_add(1, Ordering::Relaxed); return None; }
        if k == 0 && NA_KEYS[i].compare_exchange(0, tag, Ordering::Relaxed, Ordering::Relaxed).is_ok() { NA_CNTS[i].fetch_add(1, Ordering::Relaxed); return None; }
    }
    None
}
pub fn na_report() -> String {
    let mut v: Vec<(u64, u64)> = (0..32).filter_map(|i| { let k = NA_KEYS[i].load(Ordering::Relaxed); if k == 0 { None } else { Some((k, NA_CNTS[i].load(Ordering::Relaxed))) } }).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    if v.is_empty() { return String::new(); }
    let mut s = String::from("=== combat_score 미포팅 콜리(경로별 도달 수) ===\n");
    for (k, c) in v { let t: String = k.to_le_bytes().iter().take_while(|b| **b != 0).map(|b| *b as char).collect(); s += &format!("{:<10} x{}\n", t, c); }
    s
}
thread_local! { pub static LAST: std::cell::Cell<[i64; 14]> = const { std::cell::Cell::new([0; 14]) }; }
/// DIFF 로그용 단계 값
pub unsafe fn diag(_p1: usize, _p3: usize, _p4: usize) -> String {
    let v = LAST.with(|c| c.get());
    format!("risk_neg={} tower={} pos={} main={} urgent={} C={} thr_s={} chase={} bb998={} b9b0={} cast={} hp={} thr={} thrlen={}",
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13])
}
#[inline] fn tag8(s: &str) -> u64 { let mut b = [0u8; 8]; for (i, c) in s.bytes().take(8).enumerate() { b[i] = c; } u64::from_le_bytes(b) }

// ── 0x16047b0 오판 플래그(결정적 해시, splitmix64 4회) ─────────────────────────────────
//   인자 (seed, rec.928, now, tps, min(100,rec.208), min(100,rec.210), rec.448) → (al, dl); 호출부는 둘을 OR.
//   capstone 전수(2026-09-07 01:25, 0x16047b0~0x1604a76): 나눗셈 매직 5종 = /6000 · /10000 · /10000 · /1000 · /1000.
#[inline] fn splitmix(x: u64) -> u64 {
    let z = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
#[inline] fn mulhi(a: u64, b: u64) -> u64 { ((a as u128).wrapping_mul(b as u128) >> 64) as u64 }
#[allow(clippy::too_many_arguments)]
pub fn misjudge(seed: u64, key: u64, now: u64, tps: u64, s1: u64, s2: u64, v7: u64) -> (bool, bool) {
    let step = (10u64.wrapping_mul(tps)).max(1);
    let (q, rem) = (now / step, now % step);
    let h0 = (seed ^ (key << 4) ^ (q << 40)) ^ 0x1a75e;
    let den = (810u64.wrapping_mul(tps)).max(1);
    let over = now.saturating_sub(210u64.wrapping_mul(tps));
    let ramp = ((over.wrapping_mul(6000)) / den).min(0x1a5e);
    let (t3, t3p) = (3u64.wrapping_mul(tps), 3u64.wrapping_mul(tps).wrapping_add(1));
    let a1 = { let h = splitmix(h0.wrapping_add(0x9e37_79b9_7f4a_7c15)); if t3p != 0 { mulhi(h, t3p).wrapping_add(t3) } else { h } };
    let win_a = (7u64.wrapping_mul(tps)).min((ramp.wrapping_mul(tps) / 6000).wrapping_add(a1));
    let win_b = { let h = splitmix(h0.wrapping_add(0x3c6e_f372_fe94_f82a)); if t3p != 0 { t3.wrapping_add(mulhi(h, t3p)) } else { h } };
    // 첫 판정: (해시 0..9999) < (100−s1)² · ramp / 10000  이면 rem < win_a
    let p1 = 100u64.saturating_sub(s1) as u32;
    let thr_a = ((0xd1b7_1759u64).wrapping_mul((ramp as u32).wrapping_mul(p1.wrapping_mul(p1)) as u64)) >> 45;
    let roll_a = mulhi(splitmix(h0.wrapping_add(0xdaa6_6d2c_7ddf_743f)), 10000);
    let al = if (roll_a as u32) < (thr_a as u32) { rem < win_a } else { false };
    // 둘째 판정: (해시 0..9999) < ((100−s2)²·4500/10000 · (1000−v7)/1000 · (1000−v7)/1000) ∧ rem < win_b
    let p2 = 100u64.saturating_sub(s2) as u32; let p3 = 1000u64.saturating_sub(v7) as u32;
    let e1 = (p2.wrapping_mul(p2)).wrapping_mul(4500) as u64;
    let e2 = (e1.wrapping_mul(0x68db_8bb)) >> 40;
    let e3 = ((e2 as u32).wrapping_mul(p3)) as u64;
    let e4 = (e3.wrapping_mul(0x83_126f)) >> 33;
    let e5 = ((e4 as u32).wrapping_mul(p3)) as u64;
    let thr_b = (e5.wrapping_mul(0x1062_4dd3)) >> 38;
    let roll_b = mulhi(splitmix(h0.wrapping_add(0x78dd_e6e5_fd29_f054)), 10000);
    let dl = (rem < win_b) && ((roll_b as u32) < (thr_b as u32));
    (al, dl)
}

// ── 헬퍼 ────────────────────────────────────────────────────────────────────────────────
#[inline] fn absd(a: u64, b: u64) -> u64 { if a < b { b - a } else { a - b } }
#[inline] fn sq(x: u64) -> u64 { x.wrapping_mul(x) }
#[inline] fn d2_xy(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = absd(ax, bx); let dy = absd(ay, by); dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) }
#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] unsafe fn d2_ee(a: usize, b: usize) -> Option<u64> { let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?; Some(d2_xy(ax, ay, bx, by)) }
/// 0x12a07d0: 두 점 거리(isqrt)
#[inline] unsafe fn dist(a: usize, b: usize) -> Option<u64> { Some(isqrt_fast(d2_ee(a, b)?)) }
#[inline] unsafe fn rng_of(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_F470)? as i64; let r = rd_u64(e + ENT_F680)?;
    Some(if p == 0 { r } else { ((p + 100).wrapping_mul(r as i64)) as u64 / 100 })
}
/// slot vt+0xe8 (사거리 기여) — dn_reach 디스패처 재사용
#[inline] unsafe fn slot_e8(slot: usize, e: usize, other: usize) -> Option<u64> {
    super::dn_reach::eff_e8(rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize, e, other, 0)
}
/// reach(e, slot, other) = slot.base + (slot.flag==0 ? rng(e) : 0) + e.438 + (lv−1)*slot.perlv + rng(other) + slot.vt_e8
unsafe fn reach(e: usize, slot: usize, other: usize) -> Option<u64> {
    let base = rd_u64(slot + SLOT_BASE)?; let per = rd_u64(slot + SLOT_PERLV)?;
    let own = if rd_i32(slot + SLOT_FLAG)? == 0 { rng_of(e)? } else { 0 };
    Some(base.wrapping_add(own).wrapping_add(rd_u64(e + ENT_F438)?)
        .wrapping_add(rd_u64(e + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(per))
        .wrapping_add(rng_of(other)?).wrapping_add(slot_e8(slot, e, other)?))
}
#[inline] unsafe fn est(slot: usize, e: usize, t: usize) -> Option<u64> { estimate_damage(slot, e, t, false) }
/// bb 의 Record Vec 에서 핸들이 일치하는 원소(0xd31bb0 계열 매칭)
unsafe fn find_rec(bb: usize, ptr_off: usize, len_off: usize, handle: u64) -> Option<Option<usize>> {
    let n = rd_u64(bb + len_off)?; if n == 0 { return Some(None); }
    let p = rd_u64(bb + ptr_off)? as usize; if !ptr_ok(p) { return None; }
    for i in 0..n.min(64) as usize { let r = p + i * AS_REC_STRIDE; if rd_u64(r + RT_HANDLE)? == handle { return Some(Some(r)); } }
    Some(None)
}
/// 적 로스터 중 dist² < lim 이고 (보임 ∨ last_seen+120 ≥ now) 인 원소가 있는가 / 목록
unsafe fn enemy_visible_near(w: &World, agents: usize, side: u64, from: usize, lim: u64, now: u64) -> Option<bool> {
    let other = 1 - side;
    for i in 0..5usize {
        let e = rd_u64(w.x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if e == 0 { continue; }
        if d2_ee(e, from)? >= lim { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(side, h)? { return Some(true); }
        let rec = w.roster_rec(h)?;
        if rec != 0 { let role = rd_u32(rec + REC_ROLE_O) as usize;
            if rd_u64(agents + (other as usize) * LANE_STRIDE + LANE_ROSTER + role * 8)?.wrapping_add(120) >= now { return Some(true); } }
    }
    Some(false)
}
/// 0xd729b0 형: 체인(적/아군 구조물 6 + 미니언 Vec)에서 self 에 가장 가까운 원소
unsafe fn nearest_in_chain(w: &World, side: u64, me: usize) -> Option<Option<(usize, u64)>> {
    let (mx, my) = xy(me)?;
    let mut best: Option<(usize, u64)> = None;
    let mut consider = |e: usize, best: &mut Option<(usize, u64)>| -> Option<()> {
        let (ex, ey) = xy(e)?; let d = d2_xy(ex, ey, mx, my);
        if best.map_or(true, |(_, bd)| d < bd) { *best = Some((e, d)); }
        Some(())
    };
    for off in STRUCT_OFFS { let e = rd_u64(w.x + off + (side as usize) * 8)? as usize; if e != 0 { consider(e, &mut best)?; } }
    let n = rd_u64(w.x + X_MINION_LEN + (side as usize) * 0x20)?; let p = rd_u64(w.x + X_MINION_PTR + (side as usize) * 0x20)? as usize;
    if n != 0 { if !ptr_ok(p) { return None; }
        for i in 0..n.min(256) as usize { let e = rd_u64(p + i * 8)? as usize; if e != 0 { consider(e, &mut best)?; } } }
    Some(best)
}

/// 본체. 반환 None = 미재현(NA).
#[allow(clippy::too_many_arguments)]
pub unsafe fn combat_score(mode: usize, _prof: usize, rec: usize, ctx: usize, bb: usize, sp: usize, slot: usize, tgt: usize, _p9: usize) -> Option<i64> {
    if !ptr_ok(rec) || !ptr_ok(ctx) || !ptr_ok(bb) || !ptr_ok(sp) || !ptr_ok(slot) || !ptr_ok(tgt) { return None; }
    // ── S0 프롤로그 ──
    let side = rd_u64(rec + REC_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(rec + REC_ROLE_O);
    let wroot = rd_u64(ctx)? as usize; let sim = rd_u64(ctx + 8)? as usize; let agents = rd_u64(ctx + 0x10)? as usize;
    if !ptr_ok(wroot) || !ptr_ok(sim) || !ptr_ok(agents) { return None; }
    let w = World { x: wroot, data: rd_u64(wroot)? as usize, vt: rd_u64(wroot + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let me = rd_u64(w.x + X_ROSTER + (side as usize) * ROSTER_SIDE_STRIDE + (role as usize) * 8)? as usize;
    if me == 0 { return None; }
    let cfg = rd_u64(sim + 8)? as usize; if !ptr_ok(cfg) { return None; }
    let tps = rd_u64(cfg + CFG_TPS)?;
    let now = rd_u64(w.data + W_TICK)?;
    let (hp, maxhp) = (rd_u64(me + ENT_HP)?, rd_u64(me + ENT_MAXHP)?);
    let urgent = if rd_u8(me + ENT_488) != 0 { true }
        else if mode < 2 { false }
        else {
            let b = dn_cache::bits(rec, ctx)?;   // 0xc87850 순수 재현(bit0|8|16)
            if b & 0x100 != 0 { true }
            else if hp.wrapping_mul(100) <= maxhp.max(1).wrapping_mul(35) { false }
            else { b & 0x10001 != 0 }
        };
    // ── S1 특수형 조기반환(0xe04400) ──
    if let Some(v) = special_early(ctx, rec, me, sp, tgt)? { return Some(v); }
    let tgt_kind = rd_i32(tgt + ENT_KIND)?;
    let self_is_tgt = me == tgt;
    // ── S2 사거리 게이트 ──
    if mode > 1 && tgt_kind == 13 && !self_is_tgt {
        let (mtag, _) = w.mode()?;
        if mtag != 2 {
            let d = dist(me, tgt)?; let r = reach(me, slot, tgt)?;
            if r < d {
                if rd_u8(bb + BB_1500) != 0 { return Some(-9_999_999); }
                if !urgent {
                    if let Some((_, _, t)) = approach(ctx, me, tgt, slot)? {
                        if t != 0 {
                            // 0xca21c0: 적 로스터 중 150,000 이내 · (보임 ∨ 최근 120틱 목격)
                            let mut vis: [usize; 8] = [0; 8]; let mut n = 0usize;
                            for i in 0..5usize {
                                let e = rd_u64(w.x + X_ROSTER + ((1 - side) as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
                                if e == 0 || n >= 8 { continue; }
                                if d2_ee(e, me)? >= 0x53d1ac100 { continue; }
                                let h = rd_u64(e + ENT_HANDLE)?;
                                let ok = w.visible(side, h)? || {
                                    let rc = w.roster_rec(h)?;
                                    rc != 0 && rd_u64(agents + ((1 - side) as usize) * LANE_STRIDE + LANE_ROSTER + (rd_u32(rc + REC_ROLE_O) as usize) * 8)?.wrapping_add(120) >= now
                                };
                                if ok { vis[n] = e; n += 1; }
                            }
                            if n > 0 {
                                let mut hdr = [0u64; 4]; hdr[0] = vis.as_ptr() as u64; hdr[3] = n as u64;
                                let mut emp = [0u64; 4]; emp[0] = vis.as_ptr() as u64;
                                let fc = super::fight_check::fight_check_memo(mode as u64, ctx, rec, me, hdr.as_ptr() as usize, emp.as_ptr() as usize)?;
                                if (fc as i64) <= (cast_delay0(sp)?.wrapping_add(t) as i64) { return Some(-9_999_999); }
                            }
                        }
                    }
                }
            }
        }
    }
    // ── S3 위협·적 구조물 스캔 ──
    let cast_delay = cast_delay0(sp)?;
    let thr = threat_sum(bb + BB_R, cast_delay.wrapping_add(30))?;
    let my_handle = rd_u64(me + ENT_HANDLE)?;
    let seen = w.visible(1 - side, my_handle)?;
    let thr_s = if seen { thr } else { thr / 3 };
    let mut bonus9b0 = if tgt_kind == 13 { rd_i64(bb + BB_9B0)? } else { 0 };
    let mut safe = true;
    if let Some((near, _)) = nearest_in_chain(&w, 1 - side, me)? {
        if rd_i32(near + ENT_KIND)? == 2 && rd_u64(near + 0x88)? == 0 {
            let extra = { let d = dist(tgt, me)? as i64; let r = reach(me, slot, tgt)? as i64; (d - r).max(0) as u64 };
            let r_t = rd_u64(near + ENT_F438)?.wrapping_add(rd_u64(near + 0x4a0)?)
                .wrapping_add(rd_u64(near + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(near + 0x4a8)?))
                .wrapping_add(rng_of(near)?).wrapping_add(rng_of(me)?)
                .wrapping_add(slot_e8(near + 0x490, near, me)?).wrapping_add(15000).wrapping_add(extra);
            safe = dist(me, near)? < r_t || tgt_kind == 13;
            if !safe { bonus9b0 = 0; }
        }
    }
    // ── S4 추격 위험 ──
    let phase = rd_u8(sim + 0x38);
    let mut chase: i64 = 0;
    if matches!(phase, 0 | 5 | 7 | 8) && rd_u64(cfg + CFG_8A8)?.saturating_sub(30u64.wrapping_mul(tps)) <= now && !self_is_tgt && safe {
        if let Some((ax, ay, t)) = approach(ctx, me, tgt, slot)? {
            let h = (cast_delay.wrapping_add(t)).clamp(tps / 2, 2 * tps);
            let (mx, my) = xy(me)?;
            let a = super::position_eval::exposure(w.x, me, mx, my, h, tps)?;
            let b = super::position_eval::exposure(w.x, me, ax as u64, ay as u64, h, tps)?;
            let hp_pct = hp.wrapping_mul(100) / maxhp.max(1);
            let b_pct = b.wrapping_mul(100) / hp.max(1);
            let keep = b != 0 && (hp <= b || b_pct > 49 || (hp_pct < 66 && b_pct > 29) || (hp_pct < 41 && b_pct > 17) || (hp_pct < 26 && b_pct > 9));
            chase = ((if keep { b } else { 0 }) as i64).wrapping_sub((a / 2) as i64).max(0);
        }
    }
    // ── S5 위험항 ──
    let c = pct_c(bb, bb + BB_R)?; let c_val = c;
    if hp == 0 { return None; }
    let risk_neg: i64 = if urgent { -1 } else {
        let inner = (rd_i64(bb + BB_998)?).wrapping_add(chase).wrapping_add(bonus9b0).wrapping_add(thr_s);
        !(inner.wrapping_mul(c) / hp as i64)
    };
    // ── S6 아군 구조물·적 근접 ──
    let near_ally = nearest_in_chain(&w, side, me)?;
    let enemy_near = enemy_visible_near(&w, agents, side, me, 0x37e11d600, now)?;
    // ── S7 타워 지원 ──
    let mut tower_support: i64 = 0;
    if let Some((na_ent, _)) = near_ally {
        let rt = rd_u64(na_ent + ENT_F438)?.wrapping_add(rd_u64(na_ent + 0x4a0)?)
            .wrapping_add(rd_u64(na_ent + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(na_ent + 0x4a8)?))
            .wrapping_add(if rd_i32(na_ent + ENT_4C0)? == 0 { rng_of(na_ent)? } else { 0 })
            .wrapping_add(rng_of(tgt)?).wrapping_add(slot_e8(na_ent + 0x490, na_ent, tgt)?);
        if enemy_near && d2_ee(tgt, na_ent)? <= sq(rt) && now < rd_u64(cfg + CFG_13F8)? {
            let thp = rd_u64(tgt + ENT_HP)?; if thp == 0 { return None; }
            tower_support = (est(na_ent + 0x490, na_ent, tgt)? as i64).wrapping_mul(100) / thp as i64;
            tower_support = tower_support.min(100);
        }
    }
    // ── S8 위치항 ──
    let mut pos_term: i64 = 0;
    if slot_pos_gate(slot)? && d2_ee(me, tgt)? >= 0x49040441 {
        let (tx, ty) = xy(tgt)?;
        let (cx, cy) = ((tx / 32000).min(29), (ty / 32000).min(29));
        let o = super::position_eval::position_eval(mode as u64, rec, ctx, cx * 32000 + 16000, cy * 32000 + 16000, 0xc)?;
        pos_term = o.a.wrapping_mul(c) / 100;
    }
    // ── S9 오판 플래그 ──
    let mis = if rd_i32(rec + REC_464)? == 1 {
        let (a, b) = misjudge(rd_u64(w.data + 0xec90)?, rd_u64(rec + REC_SEED2)?, now, tps,
                              rd_u64(rec + REC_208)?.min(100), rd_u64(rec + REC_210)?.min(100), rd_u64(rec + REC_448)?);
        a || b
    } else { false };
    // ── S10 처형 경로 ──
    if sp_vt_b8(sp)? {
        if d2_ee(me, tgt)? <= sq(reach(me, slot, tgt)?) {
            let th = rd_u64(tgt + ENT_HANDLE)?;
            let rec_t = find_rec(bb, BB_ENEMY_PTR, BB_ENEMY_LEN, th)?;
            let dv = dist(me, tgt)? as i64;
            return Some(match rec_t {
                Some(rt) if !mis => {
                    let d = est(slot, me, tgt)? as i64;
                    let x = threat_sum(rt, cast_delay.wrapping_add(30))?.wrapping_add(rd_i64(rt + RT_170)?);
                    let ct = pct_c(bb, rt)?;
                    let mut v = x / 4 + rd_i64(rt + RT_KILL)? / 2 + d;
                    if rd_u8(tgt + ENT_488) == 1 { v = v.min((rd_i64(tgt + ENT_HP)? - 1).max(0)); }
                    let thp = rd_i64(tgt + ENT_HP)?; if thp == 0 { return None; }
                    100 + v.wrapping_mul(ct) / thp
                }
                _ => 100 + (150000 - dv).max(0) / 1500,
            });
        }
    }
    // ── S11~S14 ──
    let th = rd_u64(tgt + ENT_HANDLE)?;
    let rec_t = find_rec(bb, BB_ENEMY_PTR, BB_ENEMY_LEN, th)?;
    let rec_a = find_rec(bb, BB_ALLY_PTR, BB_ALLY_LEN, th)?;
    if rec_t.is_some() { return na(tag8("S12")); }
    if rec_a.is_some() { return na(tag8("S14")); }
    if self_is_tgt { return na(tag8("S13")); }
    let main: i64 = 0;
    let _ = (bonus9b0, thr_s, sp, my_handle, seen);
    LAST.with(|c| c.set([risk_neg, tower_support, pos_term, main, urgent as i64, c_val, thr_s, chase, rd_i64(bb + BB_998).unwrap_or(-1), bonus9b0, cast_delay as i64, hp as i64, thr, rd_u64(bb + BB_R + AS_REC_THR_LEN).unwrap_or(0) as i64]));
    Some(risk_neg + tower_support + pos_term + main)
}

// ── 아직 못 옮긴 dyn 게터(도달 빈도만 집계) ─────────────────────────────────────────────
/// sp.vt+0x80 (시전 지연, 추정)
#[inline] unsafe fn cast_delay0(sp: usize) -> Option<u64> { sp_vt80(sp) }
unsafe fn sp_vt80(sp: usize) -> Option<u64> {
    let (d, v) = (rd_u64(sp)? as usize, rd_u64(sp + 8)? as usize);
    let f = rd_u64(v + 0x80)? as usize;
    if let Some(x) = super::as_callees::decode_getter(f, d) { return Some(x); }
    if let Some(r) = super::dyn_eff::impl_rva(v, 0x80) { super::dyn_eff::unseen(0x480, r); }
    None
}
/// sp.vt+0xb8 → 처형 경로 bool
unsafe fn sp_vt_b8(sp: usize) -> Option<bool> {
    let (d, v) = (rd_u64(sp)? as usize, rd_u64(sp + 8)? as usize);
    let f = rd_u64(v + 0xb8)? as usize;
    if let Some(x) = super::as_callees::decode_getter(f, d) { return Some(x & 1 == 1); }
    if let Some(r) = super::dyn_eff::impl_rva(v, 0xb8) { super::dyn_eff::unseen(0x4b8, r); }
    None
}
/// slot.vt+0x60 / +0x68 (위치항 게이트)
unsafe fn slot_pos_gate(slot: usize) -> Option<bool> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    let a = super::as_callees::eff_bool(d, v, 0x68, 0)?;
    if a { return Some(true); }
    super::as_callees::eff_bool(d, v, 0x60, 0)
}
// ── 0xe04400 특수형 조기반환 ─────────────────────────────────────────────────────────────
//   ⬜순수 경계(임시): 판정이 `sp.vt+0x68` 이 돌려주는 dyn Any 의 **type_id** 로 갈리는데, 그 게터 impl 이 수십 종이라
//   (judge_dyn 실측: slot+0x468 에 0x115c7c0 68만 · 0x17c4f00 7.7만 …) 전부 옮기기 전까지는 게임 반환을 캡처해 쓴다.
//   호출자는 combat_score(0xd5bde8) 하나뿐이므로 "직전 1건" 이 곧 이번 호출의 결과다(exe 전역 call 사이트 전수 확인).
thread_local! { static E04400: std::cell::Cell<(usize, u64, u64)> = const { std::cell::Cell::new((0, 0, 0)) }; }
pub fn e04400_record(sp: usize, tag: u64, val: u64) { E04400.with(|c| c.set((sp, tag, val))); }
/// `sp.vt+0x68` 이 돌려주는 dyn Any 의 TypeId RVA. 두 단계 모두 **정적 게터**라 기계어를 디코드해 재현한다.
///   ① impl: `48 89 c8`(mov rax,rcx) + `48 8d 15 d32`(lea rdx,[rip+d]) + ret → vt = f+10+d
///   ② vt+0x18: `48 89 c8` + `0f 10 05 d32`(movups xmm0,[rip+d]) + `0f 11 01` + ret → TypeId 주소 = g+10+d
unsafe fn sp_type_id(sp: usize) -> Option<usize> {
    let v = rd_u64(sp + 8)? as usize; if !ptr_ok(v) { return None; }
    let f = rd_u64(v + 0x68)? as usize; if !ptr_ok(f) { return None; }
    if !(rd_u8(f) == 0x48 && rd_u8(f + 1) == 0x89 && rd_u8(f + 2) == 0xc8 && rd_u8(f + 3) == 0x48 && rd_u8(f + 4) == 0x8d && rd_u8(f + 5) == 0x15) {
        if let Some(r) = super::dyn_eff::impl_rva(v, 0x68) { super::dyn_eff::unseen(0x468, r); } return None;
    }
    let vt = (f as isize + 10 + rd_i32(f + 6)? as isize) as usize; if !ptr_ok(vt) { return None; }
    let g = rd_u64(vt + 0x18)? as usize; if !ptr_ok(g) { return None; }
    if !(rd_u8(g) == 0x48 && rd_u8(g + 1) == 0x89 && rd_u8(g + 2) == 0xc8 && rd_u8(g + 3) == 0x0f && rd_u8(g + 4) == 0x10 && rd_u8(g + 5) == 0x05) {
        super::dyn_eff::unseen(0x418, g.wrapping_sub(crate::exe_base())); return None;
    }
    let tid = (g as isize + 10 + rd_i32(g + 6)? as isize) as usize;
    let b = crate::exe_base(); if b == 0 || tid <= b { return None; }
    Some(tid - b)
}
/// 0xe279f0 접근점: 사거리 안이면 (self.xy, t=0) · 밖이면 tgt 에서 self 쪽으로 (reach−15000) 지점을 그리드 보정한 좌표와 도달 틱.
///   tgt 가 내 진영에 안 보이면 None(게임 tag=0). capstone 전수(2026-09-07 01:30, 0xe279f0~0xe27c5c).
unsafe fn approach(ctx: usize, me: usize, tgt: usize, slot: usize) -> Option<Option<(i64, i64, u64)>> {
    let own = if rd_i32(slot + SLOT_FLAG)? != 0 { 0 } else { rng_of(me)? };
    let reach = rd_u64(slot + SLOT_BASE)?.wrapping_add(own).wrapping_add(rd_u64(me + ENT_F438)?)
        .wrapping_add(rd_u64(me + ENT_LEVEL)?.wrapping_sub(1).wrapping_mul(rd_u64(slot + SLOT_PERLV)?))
        .wrapping_add(rng_of(tgt)?).wrapping_add(slot_e8(slot, me, tgt)?);
    let (mx, my) = xy(me)?; let (tx, ty) = xy(tgt)?;
    let d = isqrt_fast(d2_xy(mx, my, tx, ty));
    if d <= reach { return Some(Some((mx as i64, my as i64, 0))); }
    if rd_u8(me) == 0 {
        let side = rd_u64(me + 8)?; if side > 1 { return None; }
        if rd_u64(tgt + 0x38 + (side as usize) * 0x18)? != 0 { return Some(None); }
    }
    let (dx, dy) = ((mx as i64).wrapping_sub(tx as i64), (my as i64).wrapping_sub(ty as i64));
    let len = isqrt_fast((dx.wrapping_mul(dx) as u64).wrapping_add(dy.wrapping_mul(dy) as u64));
    if len == 0 { return Some(Some((mx as i64, my as i64, 0))); }
    let back = reach.saturating_sub(15000) as i64;
    let px = dx.wrapping_mul(back) / len as i64 + tx as i64;
    let py = dy.wrapping_mul(back) / len as i64 + ty as i64;
    let sim = rd_u64(ctx + 8)? as usize; if !ptr_ok(sim) { return None; }
    let cfg = rd_u64(sim + 8)? as usize; let map = rd_u64(sim + 0x20)? as usize;
    if !ptr_ok(cfg) || !ptr_ok(map) { return None; }
    let (ax, ay) = super::as_callees::adjust_target(map, cfg, px, py)?;
    let sp640 = rd_u64(me + 0x640)?.max(1);
    let t = d.saturating_sub(reach) / sp640;
    Some(Some((ax, ay, t)))
}
/// 0xdef530: 적 로스터 중 (x,y) 에서 200,000 이내이고 (보임 ∨ 최근 120틱 목격) 인 적이 있는가
unsafe fn enemy_near_xy(w: &World, agents: usize, side: u64, x: u64, y: u64, now: u64) -> Option<bool> {
    let other = 1 - side;
    for i in 0..5usize {
        let e = rd_u64(w.x + X_ROSTER + (other as usize) * ROSTER_SIDE_STRIDE + i * 8)? as usize;
        if e == 0 { continue; }
        let (ex, ey) = xy(e)?;
        if d2_xy(ex, ey, x, y) >= 0x9502f9001 { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(side, h)? { return Some(true); }
        let rec = w.roster_rec(h)?;
        if rec != 0 { let role = rd_u32(rec + REC_ROLE_O) as usize;
            if rd_u64(agents + (other as usize) * LANE_STRIDE + LANE_ROSTER + role * 8)?.wrapping_add(120) >= now { return Some(true); } }
    }
    Some(false)
}
/// 0xe04400 특수형 조기반환. TypeId 4종만 tag=1(조기반환), 그 외는 통과.
unsafe fn special_early(ctx: usize, _rec: usize, me: usize, sp: usize, tgt: usize) -> Option<Option<i64>> {
    let tid = match sp_type_id(sp) { Some(t) => t, None => return na(tag8("e04400")).map(|_| None) };
    if !matches!(tid, 0x33e1d30 | 0x33e1d40 | 0x33e1d50 | 0x33e1d60) { return Some(None); }
    let wroot = rd_u64(ctx)? as usize; let agents = rd_u64(ctx + 0x10)? as usize;
    let w = World { x: wroot, data: rd_u64(wroot)? as usize, vt: rd_u64(wroot + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let side = rd_u64(me + 8)?; if side > 1 { return None; }
    let now = rd_u64(w.data + W_TICK)?;
    let (tx, ty) = xy(tgt)?;
    let v: i64 = match tid {
        // A: 자기 대상일 때만 의미. slot0 없으면 10, 있으면 clamp(est(slot0)*25 / max(3, sp.vt90*100/max(1, self.3fc+100)), 10, 90),
        //    단 주변에 적이 없으면 5. ⬜sp.vt+0x90(시전 계수) 미포팅이라 그 가지는 NA 로 남긴다.
        0x33e1d30 => {
            if me != tgt { 0 } else {
                let (mx, my) = xy(me)?;
                if !enemy_near_xy(&w, agents, side, mx, my, now)? { 5 }
                else if rd_i32(me + ENT_4C0)? == -1 { 10 }
                else { return na(tag8("e044_A")).map(|_| None); }
            }
        }
        0x33e1d40 => if enemy_near_xy(&w, agents, side, tx, ty, now)? { 25 } else { 8 },
        0x33e1d50 => {
            let n = rd_u64(w.x + 0x108 + (side as usize) * 0x20)?; let p = rd_u64(w.x + 0xf0 + (side as usize) * 0x20)? as usize;
            let mut cnt = 0i64;
            if n != 0 { if !ptr_ok(p) { return None; }
                for i in 0..n.min(256) as usize { let e = rd_u64(p + i * 8)? as usize; if e != 0 && rd_i32(e + ENT_KIND)? == 7 { cnt += 1; } } }
            if cnt == 0 { -100 } else if me == tgt { 0 } else { (18 * cnt).min(60) }
        }
        _ => if enemy_near_xy(&w, agents, side, tx, ty, now)? { 90 } else { 30 },
    };
    Some(Some(v))
}
