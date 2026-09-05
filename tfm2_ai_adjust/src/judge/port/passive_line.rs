//! passive_line — game-ai\src\plan_legacy\old\passive_line.rs
//!   게임 0.5.8 · RVA 0xd2c5d0 (5182B / 1194명령) · 원본 행 235,889,918,939,953,958,1037,1038,1483 · 역할 plan_handler
//!   콜리 0xdd9f30, 0xe354d0, 0x1323a00, 0x1453260, 0x31a37c0, 0x31a3863, 0x31a3b40 · vtable 슬롯 0x28, 0x40, 0x98, 0xf8, 0x150
//!   필드 오프셋 0x3×1 0x8×20 0x9×1 0xa×1 0x10×9 0x18×5 0x20×14 0x28×8 0x30×1 0x38×3 0x40×4 0x48×1 0x50×1 0x5f×4 0x60×25 0x68×21 0x70×16 0x78×7 0x80×10 0x88×8 0x90×4 0x98×2 0xa0×3 0xa8×4 0xb0×2 0xb8×2 0xc0×2 0xc8×1 0xd0×2 0xd8×2 0xe0×2 0xf8×6 0x110×1 0x112×1 0x113×1 0x115×1 0x116×3 0x130×1 0x148×1 0x150×14 0x158×11 0x160×5 0x168×1 0x170×2 0x180×1 0x190×1 0x1a0×1 0x1a8×2 0x1d0×1 0x1d8×2 0x1e0×8 0x1f0×1 0x41f×3 0x420×2 0x5c0×6 0x628×1 0x660×29 0x668×29 0x670×1 0x8a8×1 0x930×3 0x9c0×8 0x12f8×1 0x1c98×1 0x6d70×1 0x6d78×1 0x6d80×1 0x6d88×1
//! 포팅 규약(CLAUDE.md §3): 게임 함수 호출 0 · 메모리 읽기는 judge::world(safe read) 경유 · 반환 None = 판단 불가(가드) → 호출부가 게임 원본으로 passthrough.
//! 스켈레톤 생성 = python MIG\aiport.py skeleton passive_line (디컴 원문은 아래 주석). 이 파일은 사람이 채운다 — 재생성은 --force 뿐.
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut, tr, tr_get};
use super::passive_line_callees as cal;

// ── 계약(디컴 0xd2c5d0, passive_line.rs:889~1038 · 0.5.8) ──
//   p1=out · p2=payload(+0x110/+0x112/+0x113/+0x115 플래그, +0x116 레인 f) · p3/p4 → 콜리 near_target 전달(p4 = RNG 추정)
//   p5=선수 sim(+0x930 side·+0x9c0 role) · p6=&Holder(X, G, lanes) · p7=오더(+0x41f plan, +0x420 sf) · p8 → near_target
//   출력 write-set: code 5 / code 3|4(+8=f) / code 3|5(+8=2) / code 4(+8=lane) / code 2(+8=(role==1), +9=lane, +0xa=c15)
// ⚠콜리 4종은 passive_line_callees.rs — 미규명 경로는 None(NA).

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx };
    let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] fn lane_sub(lane: u8) -> usize { if lane == 0 { 0 } else if lane == 2 { LANE_SUB_MID } else { LANE_SUB_SIDE } }

pub unsafe fn passive_line(a: &Args8) -> Option<MpOut> {
    let (payload, p5, p6, p7) = (a.p2, a.p5, a.p6, a.p7);
    if !ptr_ok(payload) || !ptr_ok(p5) || !ptr_ok(p7) { return None; }
    let mut o = MpOut::default();
    // ── 1. 조기 종단 ──
    if rd_u8(payload + PL_F110) != 0 { o.code(5); return Some(o); }
    if rd_u8(payload + PL_F112) == 1 && rd_u8(payload + PL_F115) == 0 {
        o.push(0x8, 1, rd_u8(payload + PL_LANE) as u64);
        o.code(if rd_u8(payload + PL_F113) != 0 { 4 } else { 3 });
        return Some(o);
    }
    let h = Holder::new(p6)?;
    let w = h.world()?;
    let g = h.g()?;
    let lanes = rd_u64(p6 + HOLDER_LANES)? as usize;
    if !ptr_ok(lanes) { return None; }
    let plan = rd_u8(p7 + ORDER_PLAN);
    let sf = rd_u8(p7 + ORDER_SF);
    let f0 = rd_u8(payload + PL_LANE);
    tr(0, 0x100 | f0 as u64); tr(1, 0x100 | plan as u64); tr(2, 0x100 | sf as u64);
    let lane: u8 = f0;   // 모든 경로에서 low byte = f0 (미드 블록은 f0==2 에서만 돌고 그 안의 값도 2)
    // ── 2. 미드(f0==2) 특수블록: (plan&0xfe)==8 && sf==f0 이면 건너뛴다 ──
    let skip_mid = (plan & 0xfe) == 8 && sf == f0;
    if !skip_mid && f0 == 2 {
        let tick = rd_u64(w.data + W_TICK)?;                                   // vt+0x28
        let phase = rd_u8(g.0 + G_PHASE);
        let mut to_main = false;
        if phase < 9 && ((0x1a1u32 >> (phase & 0x1f)) & 1) != 0 {
            let cfg = rd_u64(g.0 + G_CFG)? as usize;
            let t8a8 = rd_u64(cfg + CFG_8A8)?;
            let tps = rd_u64(cfg + CFG_TPS)?;
            let thr = if tps.wrapping_mul(0x1e) <= t8a8 { t8a8 - tps * 0x1e } else { 0 };
            if thr <= tick { to_main = true; }
            tr(7, 0x1_0000_0000 | (thr.min(0xffff_ffff) << 1) | to_main as u64);
        }
        tr(5, 0x100 | phase as u64); tr(6, 0x10000 | (rd_u32(p5 + P5_ROLE) as u64));
        if !to_main {
            let role = rd_u32(p5 + P5_ROLE) as u64;
            if role > 2 {
                let side = rd_u64(p5 + P5_SIDE)?;
                if side > 1 { return None; }
                let ent2 = w.roster(side, (3 + (role == 3) as u64) as u32)?;   // roster[side][3 + (role==3)]
                let mut go_count = true;
                if ent2 != 0 {
                    let bx = g.home_box(side)?;
                    let kind = rd_i32(ent2 + ENT_KIND)?; let sub = rd_i32(ent2 + ENT_SUBTYPE)?;
                    if !(kind == 0xd && sub == 1) {
                        let (ex, ey) = (rd_u64(ent2 + ENT_X)?, rd_u64(ent2 + ENT_Y)?);
                        if ex < bx.xlo { go_count = false; }
                        else if !(ex <= bx.xhi && bx.ylo <= ey && ey <= bx.yhi) { go_count = false; }
                    }
                }
                tr(6, 0x10000 | role | (go_count as u64) << 8 | ((ent2 != 0) as u64) << 9);   // t6 = role | go_count<<8 | ent2<<9
                if go_count {
                    let other = 1 - side;
                    let lane_o = lanes + (other as usize) * LANE_STRIDE;
                    let mut cnt: u32 = 0; let mut bits: u64 = 0x100_0000_0000;
                    // 진단(DIFF 원인 분리): W/H·tick·적 3/4 좌표 — t5 상위비트·t2 상위비트·t10/t11
                    { let cfg = rd_u64(g.0 + G_CFG)? as usize; tr(5, 0x100 | phase as u64 | ((rd_u64(cfg + 0x12b8)? >> 10) << 12) | ((rd_u64(cfg + 0x12c0)? >> 10) << 32)); }
                    tr(2, 0x100 | sf as u64 | (tick.min(0xf_ffff_ffff) << 12));
                    for r in 0..5u32 {
                        let e = w.roster(other, r)?;
                        if e == 0 { continue; }
                        bits |= 1 << (16 + r);
                        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
                        if r >= 3 { tr(7 + r as usize, 0x1_0000_0000_0000 | (ex.min(0xf_ffff) ) | (ey.min(0xf_ffff) << 24)); }   // t10/t11 = 적3/적4 (x | y<<24)
                        if !cal::in_region(g.0, ex, ey)? { continue; }
                        bits |= 1 << (8 + r);
                        let (lp, last) = cal::lane_pred(lane_o, w.data, w.vt, p5, e)?;
                        if lp != 0 { bits |= 1 << r; } if lp == 1 { bits |= 1 << (24 + r); }          // 24+r = 시야로 참(1) / r 만 = 기록으로 참(2)
                        if r >= 3 && lp == 2 {                                                        // t8 = 적3(low 28b)·적4(high) 의 (last+0x78 − tick) 여유(+0x800000 바이어스, 포화)
                            let m = (last.wrapping_add(0x78) as i64).wrapping_sub(tick as i64).clamp(-0x7f_ffff, 0x7f_ffff) + 0x80_0000;
                            let prev = tr_get(8) & !(0xfff_ffffu64 << (if r == 3 { 0 } else { 28 })) & 0x00ff_ffff_ffff_ffff;
                            tr(8, 0x100_0000_0000_0000 | prev | ((m as u64) << (if r == 3 { 0 } else { 28 })));
                        }
                        cnt += (lp != 0) as u32;
                    }
                    tr(9, bits); tr(4, 0x100 | cnt as u64);
                    tr(3, (if cnt >= 2 { 1u64 } else { 2u64 }) << 56);   // 분기 태그: 1=미드 분기(code 3|5) 채택 / 2=미드 카운트 후 main 으로
                    if cnt >= 2 {
                        let me = w.roster(side, role as u32)?; if me == 0 { return None; }
                        let maxhp = rd_u64(me + ENT_MAXHP)?; if maxhp == 0 { return None; }
                        let pct = rd_u64(me + ENT_HP)?.wrapping_mul(100) / maxhp;
                        let code = if pct < 0x33 { if rd_i64(lanes + (side as usize) * LANE_STRIDE + LANE_F60)? > 999 { 5 } else { 3 } } else { 3 };
                        o.push(0x8, 1, 2); o.code(code); return Some(o);
                    }
                }
            }
        }
    }
    // ── 3. LAB_d2ca62 ──
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(p5 + P5_ROLE) as u64;
    let me = w.roster(side, role as u32)?; if me == 0 { return None; }
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let is_role1 = role == 1;
    let other = 1 - side;
    let lane_self = lanes + (side as usize) * LANE_STRIDE;
    let lane_other = lanes + (other as usize) * LANE_STRIDE;
    let c15: u8;
    if plan == 8 && sf == lane {
        // SF 경로
        let mut hit = false;
        let ally1 = w.roster(side, 1)?;
        if ally1 != 0 {
            let (ax, ay) = (rd_u64(ally1 + ENT_X)?, rd_u64(ally1 + ENT_Y)?);
            let d2 = sqd(mx, my, ax, ay);
            let gy = (ay / GRID_CELL).min(GRID_MAX); let gx = (ax / GRID_CELL).min(GRID_MAX);
            let tbl = rd_u64(g.0 + G_BOXES)? as usize; if !ptr_ok(tbl) { return None; }
            let cell = rd_u64(tbl + GRID_TBL + (gy as usize) * 0xf0 + (gx as usize) * 8)?;
            let vis = w.visible(other, rd_u64(ally1 + ENT_HANDLE)?)?;
            if d2 < D2_200K_PLUS1 && cell != 0 && !vis { hit = true; }
        }
        if hit { c15 = 1; }
        else { c15 = (rd_i64(lane_self + lane_sub(lane) + LR_F10)? < 0x7d1) as u8; }
    } else {
        // MAIN: 150000 안 적 중 (보임 || 로스터 레코드 있고 레인 임계+0x78 >= tick) 하나라도 있으면 HIT
        let tick = rd_u64(w.data + W_TICK)?;
        let mut hit = false;
        for r in 0..5u32 {
            let e = w.roster(other, r)?;
            if e == 0 { continue; }
            let d2 = sqd(mx, my, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
            if (d2 >> 8) >= D2_150K_SHR8 { continue; }
            let hh = rd_u64(e + ENT_HANDLE)?;
            if w.visible(side, hh)? { hit = true; break; }
            let rec = w.roster_rec(hh)?;
            if rec != 0 {
                let thr = rd_i64(lane_other + LANE_ROSTER + (rd_u32(rec + REC_ROLE) as usize) * 8)?;
                if tick <= (thr as u64).wrapping_add(LANE_ROSTER_MARGIN) { hit = true; break; }
            }
        }
        c15 = if hit { if rd_i64(lane_self + lane_sub(lane) + LR_F18)? < 0 { 2 } else { 0 } } else { 2 };
    }
    tr(3, (tr_get(3) & (0xffu64 << 56)) | 0x100 | c15 as u64);   // 상위 바이트(분기 태그)는 보존
    // ── 4. LAB_d2d0e4 ──
    let lr = lane_self + lane_sub(lane);
    let code2 = |o: &mut MpOut| { o.push(0x8, 1, is_role1 as u64); o.push(0x9, 1, lane as u64); o.push(0xa, 1, c15 as u64); o.code(2); };
    if rd_i32(lr + LR_STATE)? != 1 { code2(&mut o); return Some(o); }
    let tgt = match w.entity(rd_u64(lr + LR_TARGET_H)?) { Some(t) => t.0, None => { code2(&mut o); return Some(o); } };
    let (tx, ty) = (rd_u64(tgt + ENT_X)?, rd_u64(tgt + ENT_Y)?);
    tr(4, tgt as u64);
    let enemy_cnt = cal::near_target_count(p7, a.p3, a.p4, p5, p6, tx, ty, 150000, a.p8)?;
    // 아군 5명: 나 또는 목표에서 150000 안이면 +1
    let mut ally_cnt: u64 = 0;
    for r in 0..5u32 {
        let e = w.roster(side, r)?; if e == 0 { continue; }
        let (ex, ey) = (rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?);
        let near_me = (sqd(mx, my, ex, ey) >> 8) < D2_150K_SHR8;
        ally_cnt += (near_me || (sqd(tx, ty, ex, ey) >> 8) < D2_150K_SHR8) as u64;
    }
    tr(5, enemy_cnt); tr(6, ally_cnt);
    // 타워: X+0x180(1차) / +0x190(2차) [side*8 + lane*0x20]
    let toff = (side as usize) * 8 + (lane as usize) * 0x20;
    let mut tower = rd_u64(w.x + X_TOWER_A + toff)? as usize;
    if tower == 0 { tower = rd_u64(w.x + X_TOWER_B + toff)? as usize; }
    let mptr = rd_u64(w.x + X_MINION_PTR + (side as usize) * 0x20)? as usize;
    let mlen = rd_u64(w.x + X_MINION_LEN + (side as usize) * 0x20)?;
    let nexus = rd_u64(w.x + X_NEXUS + (side as usize) * 8)? as usize;
    if mlen != 0 {
        let li = lane.min(2) as usize;
        let (ax, ay) = if side == 1 { (LANE_ANCHOR_A[li], LANE_ANCHOR_B[li]) } else { (LANE_ANCHOR_B[li], LANE_ANCHOR_A[li]) };
        if let Some(elem) = cal::nearest_to_anchor(mptr, mlen, ax, ay)? {
            if tower == 0 { tower = rd_u64(elem)? as usize; }
        }
    }
    if tower == 0 { tower = nexus; }
    if tower == 0 || nexus == 0 { return None; }   // 게임: panic(unwrap)
    let (nx, ny) = (rd_u64(nexus + ENT_X)?, rd_u64(nexus + ENT_Y)?);
    let (twx, twy) = (rd_u64(tower + ENT_X)?, rd_u64(tower + ENT_Y)?);
    if sqd(nx, ny, tx, ty) < sqd(nx, ny, twx, twy) { code2(&mut o); return Some(o); }
    if enemy_cnt <= ally_cnt || (sqd(twx, twy, tx, ty) >> 8) < D2_170K_SHR8 { code2(&mut o); return Some(o); }
    let f20 = rd_i32(lr + LR_F20)?;
    // bVar38
    let (tag, mode) = w.mode()?;
    let b38: bool = if plan == 1 { lane == 0 }
        else if plan != 0 {
            if tag != 0 || rd_u64(mode + T0_LEN)? == 0 { (tag == 0 && rd_u64(mode + T1_LEN)? != 0) && lane == 0 }
            else { lane == 2 }
        } else { lane == 2 };
    // d849: 목표 몬스터(세르펜=lane0 / 모르가드=lane2) 해석
    let mut resolved: Option<usize> = None;
    if lane == 0 {
        if tag == 0 && rd_u64(mode + T1_LEN)? != 0 {
            let vp = rd_u64(mode + T1_PTR)? as usize; if !ptr_ok(vp) { return None; }
            resolved = w.entity(rd_u64(vp)?).map(|e| e.0);
        }
    } else if lane == 2 && tag == 0 && rd_u64(mode + T0_LEN)? != 0 {
        let vp = rd_u64(mode + T0_PTR)? as usize; if !ptr_ok(vp) { return None; }
        resolved = w.entity(rd_u64(vp)?).map(|e| e.0);
    }
    let code_45 = |o: &mut MpOut| { if f20 < 3 { o.push(0x8, 1, lane as u64); o.code(4); } else { o.code(5); } };
    let d93f = |o: &mut MpOut| -> Option<()> {
        if rd_i32(tower + ENT_KIND)? != 2 || rd_u64(tower + ENT_F88)? == 0 { code2(o); } else { o.code(5); }
        Some(())
    };
    match resolved {
        None => {
            // d8e2
            if b38 { if enemy_cnt < 4 { code2(&mut o); return Some(o); } code_45(&mut o); return Some(o); }
            d93f(&mut o)?; Some(o)
        }
        Some(ent) => {
            if rd_u8(me) != 0 {
                if !b38 { d93f(&mut o)?; return Some(o); }
                code_45(&mut o); return Some(o);
            }
            let side2 = rd_u64(me + 8)?; if side2 > 1 { return None; }
            if b38 {
                if enemy_cnt < 4 && rd_u64(ent + ENT_VIS_BASE + (side2 as usize) * ENT_VIS_STRIDE)? != 0 { code2(&mut o); return Some(o); }
                code_45(&mut o); return Some(o);
            }
            d93f(&mut o)?; Some(o)
        }
    }
}

// ═══ 원본 디컴(자동 동봉, 참고용 — decomp\0.5.8\plan_legacy\old\passive_line.md) ═══
// ## `0xd2c5d0`  —  원본 행 889~1038
//
// | | |
// |---|---|
// | RVA | `0xd2c5d0` ~ `0xd2da0e` (5182 B) |
// | 명령 수 | 1194 |
// | 원본 행 | 889, 889, 918, 918, 939, 953, 958, 1037, 1038 |
// | 참조 타 모듈 | `map_def.rs`×1, `entity.rs`×1 |
//
// **콜리**: `0x31a37c0`×6 · `0x1453260`×5 · `0x1323a00`×5 · `0x31a3863`×4 · `0xdd9f30`×1 · `0xe354d0`×1 · `0x31a3b40`×1
//
// **상수**(|v|≥16): `0x140d2d045`(5382524997)×10 · `0x53d1ac1`(87890625)×10 · `0x140d2ca62`(5382523490)×9 · `0x140d2d72d`(5382526765)×9 · `0x140d2d744`(5382526788)×6 · `0x2e8`(744)×6 · `0x1431a37c0`(5420758976)×6 · `0x140d2d8e2`(5382527202)×6 · `0x141453260`(5390021216)×5 · `0x141323a00`(5388777984)×5 · `0x53d1ac0`(87890624)×5 · `0x78`(120)×5 · `0x140d2d0e4`(5382525156)×4 · `0x1431a3863`(5420759139)×4 · `0x1e0`(480)×3 · `0x1d`(29)×3 · `0x140d2cd28`(5382524200)×3 · `0x140d2cdef`(5382524399)×3 · `0x140d2ceb1`(5382524593)×3 · `0x140d2cf73`(5382524787)×3 · `0x50`(80)×3 · `0x28`(40)×3 · `0x140d2d8f5`(5382527221)×3 · `0x140d2d93f`(5382527295)×3 · `0xe8`(232)×2 · `0x140d2c630`(5382522416)×2 · `0x140d2c708`(5382522632)×2 · `0x140d2c7c7`(5382522823)×2 · `0x140d2cbe5`(5382523877)×2 · `0x140d2d0b1`(5382525105)×2 · `0x140d2d06f`(5382525039)×2 · `0x140d2d087`(5382525063)×2 · `0x140d2d0d8`(5382525144)×2 · `0x140d2d238`(5382525496)×2 · `0x140d2d2fa`(5382525690)×2 · `0x140d2d682`(5382526594)×2 · `0x140d2d7bc`(5382526908)×2 · `0x140d2d817`(5382526999)×2 · `0x140d2c5fb`(5382522363)×1 · `0x140d2c83a`(5382522938)×1
//
// **필드 오프셋**: `+0x3`×1 · `+0x8`×20 · `+0x9`×1 · `+0xa`×1 · `+0x10`×9 · `+0x18`×5 · `+0x20`×14 · `+0x28`×8 · `+0x30`×1 · `+0x38`×3 · `+0x40`×4 · `+0x48`×1 · `+0x50`×1 · `+0x5f`×4 · `+0x60`×25 · `+0x68`×21 · `+0x70`×16 · `+0x78`×7 · `+0x80`×10 · `+0x88`×8 · `+0x90`×4 · `+0x98`×2 · `+0xa0`×3 · `+0xa8`×4 · `+0xb0`×2 · `+0xb8`×2 · `+0xc0`×2 · `+0xc8`×1 · `+0xd0`×2 · `+0xd8`×2 · `+0xe0`×2 · `+0xf8`×6 · `+0x110`×1 · `+0x112`×1 · `+0x113`×1 · `+0x115`×1 · `+0x116`×3 · `+0x130`×1 · `+0x148`×1 · `+0x150`×14 · `+0x158`×11 · `+0x160`×5 · `+0x168`×1 · `+0x170`×2 · `+0x180`×1 · `+0x190`×1 · `+0x1a0`×1 · `+0x1a8`×2 · `+0x1d0`×1 · `+0x1d8`×2 · `+0x1e0`×8 · `+0x1f0`×1 · `+0x41f`×3 · `+0x420`×2 · `+0x5c0`×6 · `+0x628`×1 · `+0x660`×29 · `+0x668`×29 · `+0x670`×1 · `+0x8a8`×1
//
// **vtable 간접호출**: `+0x28`×6 · `+0x40`×2 · `+0x98`×1 · `+0xf8`×6 · `+0x150`×5
//
// ```c
// // fn @ passive_line.rs:?   [RVA 0xd2c5d0]
//
// /* WARNING: Type propagation algorithm not settling */
//
// longlong *
// FUN_140d2c5d0(longlong *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
//              longlong param_5,longlong *param_6,longlong param_7,undefined8 param_8)
//
// {
//   uint uVar1;
//   int iVar2;
//   ulonglong uVar3;
//   undefined8 uVar4;
//   undefined8 uVar5;
//   code *pcVar6;
//   char *pcVar7;
//   ulonglong uVar8;
//   ulonglong uVar9;
//   ulonglong uVar10;
//   ulonglong uVar11;
//   code *pcVar12;
//   char cVar13;
//   byte bVar14;
//   char cVar15;
//   uint uVar16;
//   longlong lVar17;
//   ulonglong uVar18;
//   int *piVar19;
//   longlong lVar20;
//   longlong lVar21;
//   longlong lVar22;
//   undefined8 *puVar23;
//   ulonglong uVar24;
//   longlong *plVar25;
//   longlong lVar26;
//   longlong lVar27;
//   ulonglong uVar28;
//   ulonglong uVar29;
//   longlong lVar30;
//   ulonglong uVar31;
//   undefined7 uVar34;
//   longlong lVar32;
//   ulonglong uVar33;
//   uint uVar35;
//   ulonglong uVar36;
//   int *piVar37;
//   bool bVar38;
//   bool bStack_c9;
//   uint uStack_a8;
//   ulonglong uStack_a0;
//   longlong *plStack_80;
//   longlong *plStack_78;
//   longlong lStack_70;
//   ulonglong uStack_68;
//   longlong lStack_60;
//   undefined8 uStack_58;
//   undefined8 uStack_50;
//   longlong lStack_48;
//
//   if (*(char *)(param_2 + 0x110) != '\0') {
//     *param_1 = 5;
//     return param_1;
//   }
//   if ((*(char *)(param_2 + 0x112) == '\x01') && (*(char *)(param_2 + 0x115) == '\0')) {
//     cVar13 = *(char *)(param_2 + 0x113);
//     *(undefined1 *)(param_1 + 1) = *(undefined1 *)(param_2 + 0x116);
//     if (cVar13 != '\0') {
//       *param_1 = 4;
//       return param_1;
//     }
//     *param_1 = 3;
//     return param_1;
//   }
//   uStack_58 = param_3;
//   uStack_50 = param_4;
//   lStack_48 = param_2;
//   if ((*(byte *)(param_7 + 0x41f) & 0xfe) == 8) {
//     uVar31 = (ulonglong)*(byte *)(param_7 + 0x420);
//     bVar14 = *(byte *)(param_2 + 0x116);
//     if (*(byte *)(param_7 + 0x420) != bVar14) goto joined_r0x000140d2c69e;
//   }
//   else {
//     bVar14 = *(byte *)(param_2 + 0x116);
// joined_r0x000140d2c69e:
//     uVar31 = (ulonglong)bVar14;
//     if (bVar14 == 2) {
//       uVar31 = 0;
//       puVar23 = (undefined8 *)*param_6;
//       lVar22 = param_6[1];
//       uVar5 = *puVar23;
//       lVar20 = puVar23[1];
//       uVar18 = (**(code **)(lVar20 + 0x28))(uVar5);
//       if ((*(byte *)(lVar22 + 0x38) < 9) && ((0x1a1U >> (*(byte *)(lVar22 + 0x38) & 0x1f) & 1) != 0)
//          ) {
//         uVar24 = *(ulonglong *)(*(longlong *)(lVar22 + 8) + 0x8a8);
//         lVar21 = *(longlong *)(*(longlong *)(lVar22 + 8) + 0x12f8);  // 0x12f8=tick/sec
//         uVar29 = 0;
//         if ((ulonglong)(lVar21 * 0x1e) <= uVar24) {
//           uVar29 = uVar24 + lVar21 * -0x1e;
//         }
//         uVar31 = CONCAT71((int7)(uVar31 >> 8),2);
//         if (uVar29 <= uVar18) goto LAB_140d2ca62;
//       }
//       uVar1 = *(uint *)(param_5 + 0x9c0);  // 0x9c0=role
//       uVar31 = CONCAT71((int7)(uVar31 >> 8),2);
//       if (2 < (ulonglong)uVar1) {
//         uVar18 = *(ulonglong *)(param_5 + 0x930);  // 0x930=side
//         if (1 < uVar18) {
//           FUN_1431a3863(uVar18,2,&PTR_s_game_core_src_simulation_map_def_1433d8c98);
//                     /* WARNING: Does not return */
//           pcVar6 = (code *)invalidInstructionException();
//           (*pcVar6)();
//         }
//         lVar21 = puVar23[uVar18 * 5 + (ulonglong)(uVar1 == 3) + 0x3f];
//         if (lVar21 != 0) {
//           lVar17 = *(longlong *)(lVar22 + 0x20);
//           lVar26 = uVar18 * 0x20;
//           uVar31 = *(ulonglong *)(lVar17 + 0x6d70 + lVar26);
//           if ((*(int *)(lVar21 + 0x68) != 0xd) || (*(int *)(lVar21 + 0x70) != 1)) {
//             uVar34 = (undefined7)(uVar31 >> 8);
//             if (*(ulonglong *)(lVar21 + 0x660) < uVar31) {  // 0x660=x
//               uVar31 = CONCAT71(uVar34,2);
//             }
//             else {
//               uVar31 = CONCAT71(uVar34,2);
//               if (((*(ulonglong *)(lVar21 + 0x660) <= *(ulonglong *)(lVar17 + 0x6d80 + lVar26)) &&  // 0x660=x
//                   (*(ulonglong *)(lVar17 + 0x6d78 + lVar26) <= *(ulonglong *)(lVar21 + 0x668))) &&  // 0x668=y
//                  (*(ulonglong *)(lVar21 + 0x668) <= *(ulonglong *)(lVar17 + 0x6d88 + lVar26)))  // 0x668=y
//               goto LAB_140d2c7c7;
//             }
//             goto LAB_140d2ca62;
//           }
//         }
// LAB_140d2c7c7:
//         lVar26 = 1 - uVar18;
//         lVar21 = param_6[2];
//         lVar27 = lVar26 * 0x2e8 + lVar21;
//         lVar17 = puVar23[lVar26 * 5 + 0x3c];
//         if (lVar17 == 0) {
//           uVar35 = 0;
//         }
//         else {
//           cVar13 = FUN_141453260(lVar22,*(undefined8 *)(lVar17 + 0x660),  // 0x660=x
//                                  *(undefined8 *)(lVar17 + 0x668),CONCAT71((int7)(uVar31 >> 8),2));  // 0x668=y
//           if (cVar13 == '\0') {
//             uVar35 = 0;
//           }
//           else {
//             bVar14 = FUN_141323a00(lVar27,uVar5,lVar20,param_5,lVar17);
//             uVar35 = (uint)bVar14;
//           }
//         }
//         lVar17 = puVar23[lVar26 * 5 + 0x3d];
//         if (lVar17 != 0) {
//           cVar13 = FUN_141453260(lVar22,*(undefined8 *)(lVar17 + 0x660),  // 0x660=x
//                                  *(undefined8 *)(lVar17 + 0x668),2);  // 0x668=y
//           if (cVar13 == '\0') {
//             uVar16 = 0;
//           }
//           else {
//             bVar14 = FUN_141323a00(lVar27,uVar5,lVar20,param_5,lVar17);
//             uVar16 = (uint)bVar14;
//           }
//           uVar35 = uVar35 + uVar16;
//         }
//         lVar17 = puVar23[lVar26 * 5 + 0x3e];
//         if (lVar17 != 0) {
//           cVar13 = FUN_141453260(lVar22,*(undefined8 *)(lVar17 + 0x660),  // 0x660=x
//                                  *(undefined8 *)(lVar17 + 0x668));  // 0x668=y
//           if (cVar13 == '\0') {
//             uVar16 = 0;
//           }
//           else {
//             bVar14 = FUN_141323a00(lVar27,uVar5,lVar20,param_5,lVar17);
//             uVar16 = (uint)bVar14;
//           }
//           uVar35 = uVar35 + uVar16;
//         }
//         lVar17 = puVar23[lVar26 * 5 + 0x3f];
//         if (lVar17 != 0) {
//           cVar13 = FUN_141453260(lVar22,*(undefined8 *)(lVar17 + 0x660),  // 0x660=x
//                                  *(undefined8 *)(lVar17 + 0x668));  // 0x668=y
//           if (cVar13 == '\0') {
//             uVar16 = 0;
//           }
//           else {
//             bVar14 = FUN_141323a00(lVar27,uVar5,lVar20,param_5,lVar17);
//             uVar16 = (uint)bVar14;
//           }
//           uVar35 = uVar35 + uVar16;
//         }
//         lVar17 = puVar23[lVar26 * 5 + 0x40];
//         if (lVar17 != 0) {
//           cVar13 = FUN_141453260(lVar22,*(undefined8 *)(lVar17 + 0x660),  // 0x660=x
//                                  *(undefined8 *)(lVar17 + 0x668));  // 0x668=y
//           if (cVar13 == '\0') {
//             uVar16 = 0;
//           }
//           else {
//             bVar14 = FUN_141323a00(lVar27,uVar5,lVar20,param_5,lVar17);
//             uVar16 = (uint)bVar14;
//           }
//           uVar35 = uVar35 + uVar16;
//         }
//         uVar31 = 2;
//         if (1 < uVar35) {
//           lVar22 = puVar23[uVar18 * 5 + (ulonglong)uVar1 + 0x3c];
//           if (lVar22 == 0) {
//             FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8d98);
//                     /* WARNING: Does not return */
//             pcVar6 = (code *)invalidInstructionException();
//             (*pcVar6)();
//           }
//           uVar31 = *(ulonglong *)(lVar22 + 0x628);  // 0x628=최대HP
//           if (uVar31 != 0) {
//             uVar24 = *(longlong *)(lVar22 + 0x670) * 100;  // 0x670=현재HP
//             if ((uVar24 | uVar31) >> 0x20 == 0) {
//               uVar24 = (uVar24 & 0xffffffff) / (uVar31 & 0xffffffff);
//             }
//             else {
//               uVar24 = uVar24 / uVar31;
//             }
//             lVar22 = 3;
//             if (uVar24 < 0x33) {
//               lVar22 = (ulonglong)(999 < *(longlong *)(lVar21 + 0x60 + uVar18 * 0x2e8)) * 2 + 3;
//             }
//             *param_1 = lVar22;
//             *(undefined1 *)(param_1 + 1) = 2;
//             return param_1;
//           }
//           FUN_1431a3b40(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8db0);
//                     /* WARNING: Does not return */
//           pcVar6 = (code *)invalidInstructionException();
//           (*pcVar6)();
//         }
//       }
//     }
//   }
// LAB_140d2ca62:
//   cVar13 = (char)uVar31;
//   uStack_a8 = (uint)uVar31 & 0xff;
//   if ((*(char *)(param_7 + 0x41f) == '\b') && (*(char *)(param_7 + 0x420) == cVar13)) {
//     uVar31 = *(ulonglong *)(param_5 + 0x930);  // 0x930=side
//     if (1 < uVar31) {
//       FUN_1431a3863(uVar31,2,&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8f30);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//     uVar1 = *(uint *)(param_5 + 0x9c0);  // 0x9c0=role
//     uStack_a0 = (ulonglong)uVar1;
//     puVar23 = (undefined8 *)*param_6;
//     lVar22 = puVar23[uVar31 * 5 + uStack_a0 + 0x3c];
//     if (lVar22 == 0) {
//       FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8f48);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//     lVar20 = puVar23[uVar31 * 5 + 0x3d];
//     if (lVar20 != 0) {
//       uVar18 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//       uVar24 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//       uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//       uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//       lVar22 = uVar29 - uVar18;
//       if (uVar29 < uVar18) {
//         lVar22 = uVar18 - uVar29;
//       }
//       lVar21 = uVar3 - uVar24;
//       if (uVar3 < uVar24) {
//         lVar21 = uVar24 - uVar3;
//       }
//       uVar24 = uVar24 / 32000;
//       if (0x1c < uVar24) {
//         uVar24 = 0x1d;
//       }
//       uVar29 = 0x1d;
//       if (uVar18 / 32000 < 0x1d) {
//         uVar29 = uVar18 / 32000;
//       }
//       lVar17 = *(longlong *)
//                 (uVar24 * 0xf0 + *(longlong *)(param_6[1] + 0x20) + 0x1c98 +
//                 (uVar29 & 0xffffffff) * 8);
//       cVar15 = (**(code **)(puVar23[1] + 0xf8))(*puVar23,1 - uVar31,*(undefined8 *)(lVar20 + 0x5c0))  // 0x5c0=핸들
//       ;
//       bStack_c9 = uVar1 == 1;
//       if ((((ulonglong)(lVar21 * lVar21 + lVar22 * lVar22) < 0x9502f9001) && (lVar17 != 0)) &&  // 0x9502f9001=200000^2+1 · 0x9502f900=200000^2
//          (cVar15 == '\0')) {
//         lVar20 = param_6[2];
//         cVar15 = '\x01';
//         goto LAB_140d2d0e4;
//       }
//     }
//     bStack_c9 = uVar1 == 1;
//     lVar20 = param_6[2];
//     lVar22 = uVar31 * 0x2e8 + lVar20;
//     if (cVar13 != '\0') {
//       if (uStack_a8 == 2) {
//         lVar22 = lVar22 + 0x50;
//       }
//       else {
//         lVar22 = lVar22 + 0x28;
//       }
//     }
//     cVar15 = *(longlong *)(lVar22 + 0x10) < 0x7d1;
//   }
//   else {
//     uStack_a0 = (ulonglong)*(uint *)(param_5 + 0x9c0);  // 0x9c0=role
//     bStack_c9 = uStack_a0 == 1;
//     uVar31 = *(ulonglong *)(param_5 + 0x930);  // 0x930=side
//     if (1 < uVar31) {
//       FUN_1431a3863(uVar31,2,&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8f60);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//     puVar23 = (undefined8 *)*param_6;
//     lVar22 = puVar23[uVar31 * 5 + uStack_a0 + 0x3c];
//     if (lVar22 == 0) {
//       FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8f78);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//     lVar26 = 1 - uVar31;
//     uVar5 = *puVar23;
//     lVar21 = puVar23[1];
//     lVar20 = param_6[2];
//     lVar27 = lVar26 * 0x2e8 + lVar20;
//     lVar17 = puVar23[lVar26 * 5 + 0x3c];
//     if (lVar17 == 0) {
// LAB_140d2cd28:
//       lVar17 = puVar23[lVar26 * 5 + 0x3d];
//       if (lVar17 != 0) {
//         uVar18 = *(ulonglong *)(lVar17 + 0x660);  // 0x660=x
//         uVar24 = *(ulonglong *)(lVar17 + 0x668);  // 0x668=y
//         uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//         uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//         lVar30 = uVar29 - uVar18;
//         if (uVar29 < uVar18) {
//           lVar30 = uVar18 - uVar29;
//         }
//         lVar32 = uVar3 - uVar24;
//         if (uVar3 < uVar24) {
//           lVar32 = uVar24 - uVar3;
//         }
//         if ((ulonglong)(lVar32 * lVar32 + lVar30 * lVar30) >> 8 < 0x53d1ac1) {
//           uVar4 = *(undefined8 *)(lVar17 + 0x5c0);  // 0x5c0=핸들
//           cVar15 = (**(code **)(lVar21 + 0xf8))(uVar5,uVar31,uVar4);
//           if ((cVar15 != '\0') ||
//              ((lVar17 = (**(code **)(lVar21 + 0x150))(uVar5,uVar4), lVar17 != 0 &&
//               (lVar17 = *(longlong *)(lVar27 + 0x1e0 + (ulonglong)*(uint *)(lVar17 + 0x9c0) * 8),  // 0x9c0=role · 0x1e0=로스터 base(+side*0x28+role*8)
//               uVar18 = (**(code **)(lVar21 + 0x28))(uVar5), uVar18 <= lVar17 + 0x78U))))
//           goto LAB_140d2d045;
//         }
//       }
//       lVar17 = puVar23[lVar26 * 5 + 0x3e];
//       if (lVar17 != 0) {
//         uVar18 = *(ulonglong *)(lVar17 + 0x660);  // 0x660=x
//         uVar24 = *(ulonglong *)(lVar17 + 0x668);  // 0x668=y
//         uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//         uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//         lVar30 = uVar29 - uVar18;
//         if (uVar29 < uVar18) {
//           lVar30 = uVar18 - uVar29;
//         }
//         lVar32 = uVar3 - uVar24;
//         if (uVar3 < uVar24) {
//           lVar32 = uVar24 - uVar3;
//         }
//         if ((ulonglong)(lVar32 * lVar32 + lVar30 * lVar30) >> 8 < 0x53d1ac1) {
//           uVar4 = *(undefined8 *)(lVar17 + 0x5c0);  // 0x5c0=핸들
//           cVar15 = (**(code **)(lVar21 + 0xf8))(uVar5,uVar31,uVar4);
//           if ((cVar15 != '\0') ||
//              ((lVar17 = (**(code **)(lVar21 + 0x150))(uVar5,uVar4), lVar17 != 0 &&
//               (lVar17 = *(longlong *)(lVar27 + 0x1e0 + (ulonglong)*(uint *)(lVar17 + 0x9c0) * 8),  // 0x9c0=role · 0x1e0=로스터 base(+side*0x28+role*8)
//               uVar18 = (**(code **)(lVar21 + 0x28))(uVar5), uVar18 <= lVar17 + 0x78U))))
//           goto LAB_140d2d045;
//         }
//       }
//       lVar17 = puVar23[lVar26 * 5 + 0x3f];
//       if (lVar17 != 0) {
//         uVar18 = *(ulonglong *)(lVar17 + 0x660);  // 0x660=x
//         uVar24 = *(ulonglong *)(lVar17 + 0x668);  // 0x668=y
//         uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//         uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//         lVar30 = uVar29 - uVar18;
//         if (uVar29 < uVar18) {
//           lVar30 = uVar18 - uVar29;
//         }
//         lVar32 = uVar3 - uVar24;
//         if (uVar3 < uVar24) {
//           lVar32 = uVar24 - uVar3;
//         }
//         if ((ulonglong)(lVar32 * lVar32 + lVar30 * lVar30) >> 8 < 0x53d1ac1) {
//           uVar4 = *(undefined8 *)(lVar17 + 0x5c0);  // 0x5c0=핸들
//           cVar15 = (**(code **)(lVar21 + 0xf8))(uVar5,uVar31,uVar4);
//           if ((cVar15 != '\0') ||
//              ((lVar17 = (**(code **)(lVar21 + 0x150))(uVar5,uVar4), lVar17 != 0 &&
//               (lVar17 = *(longlong *)(lVar27 + 0x1e0 + (ulonglong)*(uint *)(lVar17 + 0x9c0) * 8),  // 0x9c0=role · 0x1e0=로스터 base(+side*0x28+role*8)
//               uVar18 = (**(code **)(lVar21 + 0x28))(uVar5), uVar18 <= lVar17 + 0x78U))))
//           goto LAB_140d2d045;
//         }
//       }
//       lVar17 = puVar23[lVar26 * 5 + 0x40];
//       cVar15 = '\x02';
//       if (lVar17 != 0) {
//         uVar18 = *(ulonglong *)(lVar17 + 0x660);  // 0x660=x
//         uVar24 = *(ulonglong *)(lVar17 + 0x668);  // 0x668=y
//         uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//         uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//         lVar22 = uVar29 - uVar18;
//         if (uVar29 < uVar18) {
//           lVar22 = uVar18 - uVar29;
//         }
//         lVar26 = uVar3 - uVar24;
//         if (uVar3 < uVar24) {
//           lVar26 = uVar24 - uVar3;
//         }
//         if ((ulonglong)(lVar26 * lVar26 + lVar22 * lVar22) >> 8 < 0x53d1ac1) {
//           uVar4 = *(undefined8 *)(lVar17 + 0x5c0);  // 0x5c0=핸들
//           cVar15 = (**(code **)(lVar21 + 0xf8))(uVar5,uVar31,uVar4);
//           if ((cVar15 != '\0') ||
//              ((lVar22 = (**(code **)(lVar21 + 0x150))(uVar5,uVar4), lVar22 != 0 &&
//               (lVar22 = *(longlong *)(lVar27 + 0x1e0 + (ulonglong)*(uint *)(lVar22 + 0x9c0) * 8),  // 0x9c0=role · 0x1e0=로스터 base(+side*0x28+role*8)
//               uVar18 = (**(code **)(lVar21 + 0x28))(uVar5), uVar18 <= lVar22 + 0x78U))))
//           goto LAB_140d2d045;
//           cVar15 = '\x02';
//         }
//       }
//     }
//     else {
//       uVar18 = *(ulonglong *)(lVar17 + 0x660);  // 0x660=x
//       uVar24 = *(ulonglong *)(lVar17 + 0x668);  // 0x668=y
//       uVar29 = *(ulonglong *)(lVar22 + 0x660);  // 0x660=x
//       uVar3 = *(ulonglong *)(lVar22 + 0x668);  // 0x668=y
//       lVar30 = uVar29 - uVar18;
//       if (uVar29 < uVar18) {
//         lVar30 = uVar18 - uVar29;
//       }
//       lVar32 = uVar3 - uVar24;
//       if (uVar3 < uVar24) {
//         lVar32 = uVar24 - uVar3;
//       }
//       if (0x53d1ac0 < (ulonglong)(lVar32 * lVar32 + lVar30 * lVar30) >> 8) goto LAB_140d2cd28;
//       uVar4 = *(undefined8 *)(lVar17 + 0x5c0);  // 0x5c0=핸들
//       cVar15 = (**(code **)(lVar21 + 0xf8))(uVar5,uVar31,uVar4);
//       if ((cVar15 == '\0') &&
//          ((lVar17 = (**(code **)(lVar21 + 0x150))(uVar5,uVar4), lVar17 == 0 ||
//           (lVar17 = *(longlong *)(lVar27 + 0x1e0 + (ulonglong)*(uint *)(lVar17 + 0x9c0) * 8),  // 0x9c0=role · 0x1e0=로스터 base(+side*0x28+role*8)
//           uVar18 = (**(code **)(lVar21 + 0x28))(uVar5), lVar17 + 0x78U < uVar18))))
//       goto LAB_140d2cd28;
// LAB_140d2d045:
//       lVar22 = uVar31 * 0x2e8 + lVar20;
//       if (cVar13 != '\0') {
//         if (uStack_a8 == 2) {
//           lVar22 = lVar22 + 0x50;
//         }
//         else {
//           lVar22 = lVar22 + 0x28;
//         }
//       }
//       cVar15 = (char)((longlong)*(undefined8 *)(lVar22 + 0x18) >> 0x3f) * -2;
//     }
//   }
// LAB_140d2d0e4:
//   piVar37 = (int *)(uVar31 * 0x2e8 + lVar20);
//   piVar19 = piVar37;
//   if (cVar13 == '\0') {
// LAB_140d2d104:
//     iVar2 = *piVar19;
//   }
//   else {
//     if (uStack_a8 == 2) {
//       piVar19 = piVar37 + 0x14;
//       goto LAB_140d2d104;
//     }
//     piVar19 = piVar37 + 10;
//     iVar2 = *piVar19;
//   }
//   if (iVar2 != 1) goto LAB_140d2d72d;
//   uVar5 = *puVar23;
//   lVar22 = puVar23[1];
//   pcVar6 = *(code **)(lVar22 + 0x1f0);
//   lVar20 = (*pcVar6)(uVar5,*(undefined8 *)(piVar19 + 2));
//   if (lVar20 == 0) goto LAB_140d2d72d;
//   pcVar7 = (char *)puVar23[uVar31 * 5 + uStack_a0 + 0x3c];
//   if (pcVar7 == (char *)0x0) {
//     FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8f90);
//                     /* WARNING: Does not return */
//     pcVar6 = (code *)invalidInstructionException();
//     (*pcVar6)();
//   }
//   plVar25 = puVar23 + uVar31 * 5 + 0x3c;
//   uVar18 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//   uVar24 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//   FUN_140dd9f30(&plStack_80,param_7,uStack_58,uStack_50,param_5,param_6,uVar18,uVar24,150000,param_8
//                );
//   uVar29 = uStack_68;
//   if ((lStack_70 != 0) &&
//      (*(longlong **)(*(longlong *)((longlong)plStack_78 + 0x10) + 0x20) == plStack_80)) {
//     *(longlong **)(*(longlong *)((longlong)plStack_78 + 0x10) + 0x20) = plStack_80 + lStack_70;
//   }
//   uVar3 = *(ulonglong *)(pcVar7 + 0x660);  // 0x660=x
//   uVar33 = *(ulonglong *)(pcVar7 + 0x668);  // 0x668=y
//   uVar8 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//   uVar9 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//   lVar20 = *plVar25;
//   if (lVar20 == 0) {
//     uVar28 = 0;
//   }
//   else {
//     uVar10 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//     uVar11 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//     lVar20 = uVar3 - uVar10;
//     if (uVar3 < uVar10) {
//       lVar20 = uVar10 - uVar3;
//     }
//     lVar21 = uVar33 - uVar11;
//     if (uVar33 < uVar11) {
//       lVar21 = uVar11 - uVar33;
//     }
//     uVar28 = 1;
//     if (0x53d1ac0 < (ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8) {
//       lVar20 = uVar8 - uVar10;
//       if (uVar8 < uVar10) {
//         lVar20 = uVar10 - uVar8;
//       }
//       lVar21 = uVar9 - uVar11;
//       if (uVar9 < uVar11) {
//         lVar21 = uVar11 - uVar9;
//       }
//       uVar28 = (ulonglong)((ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8 < 0x53d1ac1);
//     }
//   }
//   lVar20 = plVar25[1];
//   if (lVar20 != 0) {
//     uVar10 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//     uVar11 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//     lVar20 = uVar3 - uVar10;
//     if (uVar3 < uVar10) {
//       lVar20 = uVar10 - uVar3;
//     }
//     lVar21 = uVar33 - uVar11;
//     if (uVar33 < uVar11) {
//       lVar21 = uVar11 - uVar33;
//     }
//     uVar36 = 1;
//     if (0x53d1ac0 < (ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8) {
//       lVar20 = uVar8 - uVar10;
//       if (uVar8 < uVar10) {
//         lVar20 = uVar10 - uVar8;
//       }
//       lVar21 = uVar9 - uVar11;
//       if (uVar9 < uVar11) {
//         lVar21 = uVar11 - uVar9;
//       }
//       uVar36 = (ulonglong)((ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8 < 0x53d1ac1);
//     }
//     uVar28 = uVar28 + uVar36;
//   }
//   lVar20 = plVar25[2];
//   if (lVar20 != 0) {
//     uVar10 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//     uVar11 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//     lVar20 = uVar3 - uVar10;
//     if (uVar3 < uVar10) {
//       lVar20 = uVar10 - uVar3;
//     }
//     lVar21 = uVar33 - uVar11;
//     if (uVar33 < uVar11) {
//       lVar21 = uVar11 - uVar33;
//     }
//     uVar36 = 1;
//     if (0x53d1ac0 < (ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8) {
//       lVar20 = uVar8 - uVar10;
//       if (uVar8 < uVar10) {
//         lVar20 = uVar10 - uVar8;
//       }
//       lVar21 = uVar9 - uVar11;
//       if (uVar9 < uVar11) {
//         lVar21 = uVar11 - uVar9;
//       }
//       uVar36 = (ulonglong)((ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8 < 0x53d1ac1);
//     }
//     uVar28 = uVar28 + uVar36;
//   }
//   lVar20 = plVar25[3];
//   if (lVar20 != 0) {
//     uVar10 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//     uVar11 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//     lVar20 = uVar3 - uVar10;
//     if (uVar3 < uVar10) {
//       lVar20 = uVar10 - uVar3;
//     }
//     lVar21 = uVar33 - uVar11;
//     if (uVar33 < uVar11) {
//       lVar21 = uVar11 - uVar33;
//     }
//     uVar36 = 1;
//     if (0x53d1ac0 < (ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8) {
//       lVar20 = uVar8 - uVar10;
//       if (uVar8 < uVar10) {
//         lVar20 = uVar10 - uVar8;
//       }
//       lVar21 = uVar9 - uVar11;
//       if (uVar9 < uVar11) {
//         lVar21 = uVar11 - uVar9;
//       }
//       uVar36 = (ulonglong)((ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8 < 0x53d1ac1);
//     }
//     uVar28 = uVar28 + uVar36;
//   }
//   lVar20 = plVar25[4];
//   if (lVar20 != 0) {
//     uVar10 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//     lVar21 = uVar3 - uVar10;
//     if (uVar3 < uVar10) {
//       lVar21 = uVar10 - uVar3;
//     }
//     uVar3 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//     lVar20 = uVar33 - uVar3;
//     if (uVar33 < uVar3) {
//       lVar20 = uVar3 - uVar33;
//     }
//     uVar33 = 1;
//     if (0x53d1ac0 < (ulonglong)(lVar20 * lVar20 + lVar21 * lVar21) >> 8) {
//       lVar20 = uVar8 - uVar10;
//       if (uVar8 < uVar10) {
//         lVar20 = uVar10 - uVar8;
//       }
//       lVar21 = uVar9 - uVar3;
//       if (uVar9 < uVar3) {
//         lVar21 = uVar3 - uVar9;
//       }
//       uVar33 = (ulonglong)((ulonglong)(lVar21 * lVar21 + lVar20 * lVar20) >> 8 < 0x53d1ac1);
//     }
//     uVar28 = uVar28 + uVar33;
//   }
//   lVar20 = *(longlong *)((longlong)puVar23 + uVar31 * 8 + (ulonglong)(uStack_a8 << 5) + 0x180);
//   if (lVar20 == 0) {
//     lVar20 = *(longlong *)((longlong)puVar23 + uVar31 * 8 + (ulonglong)(uStack_a8 << 5) + 400);
//   }
//   plVar25 = (longlong *)puVar23[uVar31 * 4 + 0x26];
//   plStack_78 = plVar25 + puVar23[uVar31 * 4 + 0x29];
//   uStack_68 = *(undefined8 *)(param_6[1] + 8);
//   lStack_70 = lStack_48;
//   lStack_60 = param_5;
//   if (puVar23[uVar31 * 4 + 0x29] == 0) {
// LAB_140d2d67d:
//     if (lVar20 != 0) goto LAB_140d2d682;
//     lVar21 = puVar23[uVar31 + 0x2e];
//     lVar20 = lVar21;
//     if (lVar21 == 0) {
//       FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8fa8);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//   }
//   else {
//     plStack_80 = plVar25 + 1;
//     uVar3 = *(ulonglong *)(&DAT_1433d9eb8 + uStack_a8 * 8);
//     uVar33 = *(ulonglong *)(&DAT_1433d9ea0 + uStack_a8 * 8);
//     if (uVar31 != 1) {
//       uVar33 = uVar3;
//       uVar3 = *(ulonglong *)(&DAT_1433d9ea0 + uStack_a8 * 8);
//     }
//     uVar8 = *(ulonglong *)(*plVar25 + 0x660);  // 0x660=x
//     uVar9 = *(ulonglong *)(*plVar25 + 0x668);  // 0x668=y
//     lVar21 = uVar33 - uVar8;
//     if (uVar33 < uVar8) {
//       lVar21 = uVar8 - uVar33;
//     }
//     lVar17 = uVar3 - uVar9;
//     if (uVar3 < uVar9) {
//       lVar17 = uVar9 - uVar3;
//     }
//     plVar25 = (longlong *)(lVar17 * lVar17 + lVar21 * lVar21);
//     FUN_140e354d0(&plStack_80);
//     if (plVar25 == (longlong *)0x0) goto LAB_140d2d67d;
//     if (lVar20 == 0) {
//       lVar20 = *plVar25;
//     }
// LAB_140d2d682:
//     lVar21 = puVar23[uVar31 + 0x2e];
//     if (lVar21 == 0) {
//       FUN_1431a37c0(&PTR_s_game_ai_src_plan_legacy_old_pass_1433d8fc0);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//   }
//   uVar31 = *(ulonglong *)(lVar21 + 0x660);  // 0x660=x
//   uVar3 = *(ulonglong *)(lVar21 + 0x668);  // 0x668=y
//   lVar21 = uVar31 - uVar18;
//   if (uVar31 < uVar18) {
//     lVar21 = uVar18 - uVar31;
//   }
//   lVar17 = uVar3 - uVar24;
//   if (uVar3 < uVar24) {
//     lVar17 = uVar24 - uVar3;
//   }
//   uVar33 = *(ulonglong *)(lVar20 + 0x660);  // 0x660=x
//   uVar8 = *(ulonglong *)(lVar20 + 0x668);  // 0x668=y
//   lVar26 = uVar31 - uVar33;
//   if (uVar31 < uVar33) {
//     lVar26 = uVar33 - uVar31;
//   }
//   lVar27 = uVar3 - uVar8;
//   if (uVar3 < uVar8) {
//     lVar27 = uVar8 - uVar3;
//   }
//   if ((ulonglong)(lVar17 * lVar17 + lVar21 * lVar21) <
//       (ulonglong)(lVar27 * lVar27 + lVar26 * lVar26)) goto LAB_140d2d72d;
//   lVar21 = uVar33 - uVar18;
//   if (uVar33 < uVar18) {
//     lVar21 = uVar18 - uVar33;
//   }
//   lVar26 = uVar24 - uVar8;
//   lVar17 = uVar8 - uVar24;
//   if (uVar8 < uVar24) {
//     lVar17 = lVar26;
//   }
//   if ((uVar29 <= uVar28) || ((ulonglong)(lVar17 * lVar17 + lVar21 * lVar21) >> 8 < 0x6ba9301))
//   goto LAB_140d2d72d;
//   if (cVar13 != '\0') {
//     if (uStack_a8 == 2) {
//       piVar37 = piVar37 + 0x14;
//     }
//     else {
//       piVar37 = piVar37 + 10;
//     }
//   }
//   iVar2 = piVar37[8];
//   if (*(char *)(param_7 + 0x41f) == '\x01') {
//     bVar38 = cVar13 == '\0';
//   }
//   else {
//     if (*(char *)(param_7 + 0x41f) != '\0') {
//       pcVar12 = *(code **)(lVar22 + 0x40);
//       lVar21 = (*pcVar12)(uVar5);
//       if ((lVar26 == 0 || lVar21 != 0) || (*(longlong *)(lVar26 + 0x1a8) == 0)) {
//         lVar21 = (*pcVar12)(uVar5);
//         if (lVar26 == 0 || lVar21 != 0) {
//           bVar38 = false;
//         }
//         else {
//           bVar38 = *(longlong *)(lVar26 + 0x1d8) != 0;
//         }
//         bVar38 = (bool)(cVar13 == '\0' & bVar38);
//         goto LAB_140d2d849;
//       }
//     }
//     bVar38 = cVar13 == '\x02';
//   }
// LAB_140d2d849:
//   if (cVar13 == '\0') {
//     lVar22 = (**(code **)(lVar22 + 0x40))(uVar5);
//     if ((lVar22 == 0) && (*(longlong *)(lVar26 + 0x1d8) != 0)) {
//       puVar23 = *(undefined8 **)(lVar26 + 0x1d0);
//       goto LAB_140d2d8b2;
//     }
// LAB_140d2d8e2:
//     if (bVar38) {
//       if (uVar29 < 4) goto LAB_140d2d72d;
// LAB_140d2d8f5:
//       if (iVar2 < 3) {
//         *(char *)(param_1 + 1) = cVar13;
//         *param_1 = 4;
//         return param_1;
//       }
//       goto LAB_140d2d95d;
//     }
//   }
//   else {
//     if (((uStack_a8 != 2) || (lVar22 = (**(code **)(lVar22 + 0x40))(uVar5), lVar22 != 0)) ||
//        (*(longlong *)(lVar26 + 0x1a8) == 0)) goto LAB_140d2d8e2;
//     puVar23 = *(undefined8 **)(lVar26 + 0x1a0);
// LAB_140d2d8b2:
//     lVar22 = (*pcVar6)(uVar5,*puVar23);
//     if (lVar22 == 0) goto LAB_140d2d8e2;
//     if (*pcVar7 != '\0') {
//       if (!bVar38) goto LAB_140d2d93f;
//       goto LAB_140d2d8f5;
//     }
//     uVar31 = *(ulonglong *)(pcVar7 + 8);
//     if (1 < uVar31) {
//       FUN_1431a3863(uVar31,2,&PTR_s_game_core_src_simulation_entity__1433d8a80);
//                     /* WARNING: Does not return */
//       pcVar6 = (code *)invalidInstructionException();
//       (*pcVar6)();
//     }
//     if (bVar38) {
//       if ((uVar29 < 4) && (*(longlong *)(lVar22 + 0x38 + uVar31 * 0x18) != 0)) goto LAB_140d2d72d;
//       goto LAB_140d2d8f5;
//     }
//   }
// LAB_140d2d93f:
//   if ((*(int *)(lVar20 + 0x68) != 2) || (*(longlong *)(lVar20 + 0x88) == 0)) {
// LAB_140d2d72d:
//     *(bool *)(param_1 + 1) = bStack_c9;
//     *(char *)((longlong)param_1 + 9) = cVar13;
//     *(char *)((longlong)param_1 + 10) = cVar15;
//     *param_1 = 2;
//     return param_1;
//   }
// LAB_140d2d95d:
//   *param_1 = 5;
//   return param_1;
// }
//
// ```
//
// ---
//
