//! passive_line 콜리 4종(census 밖 = game-core) 순수 재현. 정본 = REPORT\tfm2_ai_adjust\RE\2026-09-06_passive_line-콜리4종-…-0.5.8.md (ghidra-re, capstone).
//!   0x1453260 (606B)  in_lane(G, x, y, lane)                       — 순수 산술(cfg W/H)
//!   0x1323a00 (118B)  recently_seen(TeamState[enemy], world, rec_self, ent)  — visible || (roster_rec && last_seen+120 >= tick)
//!   0xdd9f30  (745B)  predict_unseen_enemies(...) → passive_line 은 **len 만** 쓴다. MOBA(태그 0) = RNG 미소비(splitmix64 해시). 태그 2 = RNG → None(NA)
//!   0xe354d0  (282B)  argmin_near_lane_anchor(slots, len, tx, ty) → 최근접 슬롯(엄격 <)
#![allow(dead_code)]
use crate::*;
use super::super::world::*;
use super::super::layout::*;

// ── 상수(0.5.8 실측) ──
const CFG_W: usize = 0x12b8;
const CFG_H: usize = 0x12c0;
const BASE_R: u64 = 192001;      // 0x2ee01
const BASE_R0: u64 = 192000;     // 0x2ee00
const LANE_BAND: u64 = 64000;    // 0xfa00
const INNER_SPAN: u64 = 704000;  // 0xabe00
const INNER_BAND: u64 = 95999;   // 0x176ff
const REC_JUDGE: usize = 0x218;  // 판단력
const REC_450: usize = 0x450; const REC_464: usize = 0x464; const REC_928: usize = 0x928; const REC_448: usize = 0x448; const REC_208: usize = 0x208; const REC_210: usize = 0x210;
const ENT_SPEED: usize = 0x640;
const MEM_POS: usize = 0x230;    // mem[0x230+i*16] = (px, py)
const MEM_T280: usize = 0x280;   // mem[0x280+i*8]
const MEM_SEEN: usize = 0x2a8;   // mem[0x2a8+i*8] = 마지막 목격 틱
const W_SEED: usize = 0xec90;    // vt+0x20
const GR: u64 = 0x9e3779b97f4a7c15;

#[inline] fn sm(mut z: u64) -> u64 { z ^= z >> 30; z = z.wrapping_mul(0xbf58476d1ce4e5b9); z ^= z >> 27; z = z.wrapping_mul(0x94d049bb133111eb); z ^ (z >> 31) }
#[inline] fn mulhi(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) >> 64) as u64 }
#[inline] fn dist(x0: u64, y0: u64, x1: u64, y1: u64) -> u64 {
    let dx = if x0 < x1 { x1 - x0 } else { x0 - x1 }; let dy = if y0 < y1 { y1 - y0 } else { y0 - y1 };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)).isqrt()
}

/// 0x1453260 — passive_line 은 전부 lane=2 로 부른다.
pub unsafe fn in_region(g: usize, x: u64, y: u64) -> Option<bool> { in_lane(g, x, y, 2) }
pub unsafe fn in_lane(g: usize, x: u64, y: u64, lane: u8) -> Option<bool> {
    let cfg = rd_u64(g + G_CFG)? as usize; if !ptr_ok(cfg) { return None; }
    let h = rd_u64(cfg + CFG_H)?; let w = rd_u64(cfg + CFG_W)?;
    let c = h.wrapping_sub(y);
    if x.max(c) < BASE_R { return Some(true); }
    if x.min(c) >= w.wrapping_sub(BASE_R0) { return Some(true); }
    let ad = if x < c { c - x } else { x - c };
    Some(match lane {
        1 => ad < LANE_BAND,
        _ => {
            // ★inner 띠는 (x, **y**) 로 판정한다 — c(=H−y) 가 아니다. 디스어셈 0x1453332 `cmp r8(y), r9((H−0xabe00)>>1)` 실측.
            //   (ghidra-re 의사코드는 c 로 적혀 있었고, 그대로 옮긴 첫 판에서 미드 카운트 경계 DIFF 2,018건이 났다.)
            let (lo_x, lo_y) = (w.wrapping_sub(INNER_SPAN) >> 1, h.wrapping_sub(INNER_SPAN) >> 1);
            let inner = lo_x <= x && x <= lo_x.wrapping_add(INNER_SPAN) && lo_y <= y && y <= lo_y.wrapping_add(INNER_SPAN);
            let (hi, lo) = if lane == 0 { (c, x) } else { (x, c) };          // 0: c>=x(위) / 2: x>=c(아래)
            let base = hi >= lo && hi >= BASE_R && (hi - lo) >= LANE_BAND;
            if inner { base && (hi - lo) <= INNER_BAND } else { base }
        }
    })
}

/// 0x1323a00 — recently_seen.
pub unsafe fn lane_pred(team_enemy: usize, data: usize, vt: usize, rec_self: usize, ent: usize) -> Option<u8> {
    let w = World { x: 0, data, vt };
    let h = rd_u64(ent + ENT_HANDLE)?; let side = rd_u64(rec_self + P5_SIDE)?;
    if w.visible(side, h)? { return Some(1); }
    let rec = w.roster_rec(h)?; if rec == 0 { return Some(0); }
    let idx = rd_u32(rec + REC_ROLE) as usize;
    let last = rd_u64(team_enemy + LANE_ROSTER + idx * 8)?;
    Some((last.wrapping_add(0x78) >= rd_u64(data + W_TICK)?) as u8)
}

/// 0x16047b0 — 예측 억제 게이트(순수 해시). (A, B)
fn gate(seed: u64, v928: u64, now: u64, tr: u64, a: u64, b: u64, c: u64) -> (bool, bool) {
    let per = (10 * tr).max(1); let r = now % per;
    let h = seed ^ (v928 << 4) ^ ((now / per) << 40) ^ 0x1a75e;
    let ramp = (now.saturating_sub(210 * tr) * 6000 / (810 * tr).max(1)).min(6750);
    let mut w1 = 3 * tr + mulhi(sm(h.wrapping_add(GR)), 3 * tr + 1); w1 = (7 * tr).min(ramp * tr / 6000 + w1);
    let w2 = 3 * tr + mulhi(sm(h.wrapping_add(2u64.wrapping_mul(GR))), 3 * tr + 1);
    let a1 = (100u32.wrapping_sub(a as u32)) as u32; let thr1 = (a1.wrapping_mul(a1).wrapping_mul(ramp as u32) / 10000) as u64;
    let roll1 = mulhi(sm(h.wrapping_add(3u64.wrapping_mul(GR))), 10000);
    let b1 = 100u32.wrapping_sub(b as u32); let c1 = (1000u32.wrapping_sub(c as u32)) as u32;
    let t = b1.wrapping_mul(b1).wrapping_mul(4500) / 10000; let t = t.wrapping_mul(c1) / 1000; let thr2 = (t.wrapping_mul(c1) / 1000) as u64;
    let roll2 = mulhi(sm(h.wrapping_add(4u64.wrapping_mul(GR))), 10000);
    (roll1 < thr1 && r < w1, roll2 < thr2 && r < w2)
}

/// 0xdd9f30 — 미시야 적 예측 목록의 **개수**. 태그 2(RNG 소비) 경로는 None.
pub unsafe fn near_target_count(mem: usize, _p3: usize, _rng: usize, rec: usize, p6: usize, sx: u64, sy: u64, radius: u64, _p8: usize) -> Option<u64> {
    let hd = Holder::new(p6)?; let w = hd.world()?; let g = hd.g()?;
    let cfg = rd_u64(g.0 + G_CFG)? as usize; if !ptr_ok(cfg) || !ptr_ok(mem) || !ptr_ok(rec) { return None; }
    let tr = rd_u64(cfg + CFG_TPS)?;
    let seed = rd_u64(w.data + W_SEED)?; let now = rd_u64(w.data + W_TICK)?;
    let (tag, _) = w.mode()?;
    if tag == 2 { return None; }                                           // RNG 경로(비-MOBA) — 재현 안 함
    if rd_u8(rec + REC_464) == 1 {
        let (a, b) = gate(seed, rd_u64(rec + REC_928)?, now, tr, rd_u64(rec + REC_208)?.min(100), rd_u64(rec + REC_210)?.min(100), rd_u64(rec + REC_448)?);
        if a || b { return Some(0); }
    }
    let my = rd_u64(rec + P5_SIDE)?; if my > 1 { return None; }
    let enemy = 1 - my;
    let judge = rd_u64(rec + REC_JUDGE)?;
    let mut n: u64 = 0;
    for i in 0..5u32 {
        let e = w.roster(enemy, i)?; if e == 0 { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        if w.visible(my, h)? { continue; }
        let spd = rd_u64(e + ENT_SPEED)?;
        let (px, py) = (rd_u64(mem + MEM_POS + (i as usize) * 16)?, rd_u64(mem + MEM_POS + (i as usize) * 16 + 8)?);
        let seen = rd_u64(mem + MEM_SEEN + (i as usize) * 8)?;
        let elapsed = now.saturating_sub(seen);
        if elapsed <= 3 * tr {
            if elapsed.wrapping_mul(spd) < dist(sx, sy, px, py).saturating_sub(radius) { continue; }
            n += 1;
        } else {
            let p = 100u64.saturating_sub(judge.min(100));
            let k = 3000u64.wrapping_add(((p * p) as u128 * 0x2d99999a4718u128 >> 43) as u64);
            let spread = elapsed.wrapping_mul(k) / tr.max(1);
            if spread + 40000 > 300000 { continue; }
            let hh = seed ^ ((i as u64) << 8) ^ rd_u64(rec + REC_928)? ^ ((now / (6 * tr).max(1)) << 40);
            let u1 = mulhi(sm(hh.wrapping_add(GR)), 2001) as i64 - 1000;
            let u2 = mulhi(sm(hh.wrapping_add(2u64.wrapping_mul(GR))), 2001) as i64 - 1000;
            let nn = (((u1 * u1 + u2 * u2) as u64).isqrt() as i64).max(1);
            let rr = mulhi(sm(hh.wrapping_add(3u64.wrapping_mul(GR))), spread + 40001) as i64;
            let (ox, oy) = ((u1 * rr) / nn, (u2 * rr) / nn);
            let (ex, ey) = (rd_u64(e + ENT_X)? as i64, rd_u64(e + ENT_Y)? as i64);
            let qx = (ex.wrapping_add(ox)).max(0) as u64; let qy = (ey.wrapping_add(oy)).max(0) as u64;
            if dist(qx, qy, sx, sy) <= spread + 40000 + 3 * spd * tr + radius { n += 1; }
        }
    }
    Some(n)
}

/// 0xe354d0 — 슬롯 배열(ptr, len)에서 앵커 (ax, ay) 최근접 슬롯 주소. 엄격 <(동점 미갱신). len==0 → Some(None).
pub unsafe fn nearest_to_anchor(ptr: usize, len: u64, ax: u64, ay: u64) -> Option<Option<usize>> {
    if len == 0 { return Some(None); }
    if !ptr_ok(ptr) { return None; }
    let mut best_slot = ptr; let mut best = u64::MAX;
    for i in 0..len.min(4096) as usize {
        let slot = ptr + i * 8; let e = rd_u64(slot)? as usize; if !ptr_ok(e) { return None; }
        let (x, y) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let dx = if x < ax { ax - x } else { x - ax }; let dy = if y < ay { ay - y } else { y - ay };
        let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
        if i == 0 || d2 < best { best = d2; best_slot = slot; }
    }
    Some(Some(best_slot))
}
