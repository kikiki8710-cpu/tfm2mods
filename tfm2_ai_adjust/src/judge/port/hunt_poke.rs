//! hunt_poke — Plan 핸들러 disc 12 `old\epic\hunt_and_poke.rs:32` (0.5.8 `0xdefcd0`, 800명령) **verify 전용 포팅**(2026-09-06 16:20).
//!   세르펜(disc 14, 0xdf0e90)은 후반 350명령이 달라 별도 포팅 예정.
//!   콜리: ability_pick 0xe7a8c0(RNG → 캡처 `cap_ability_pick`) · recently_seen 0x1323a00(`lane_pred`) · can_attack 0xeca200 · engage_gate 0xdd5db0(`obj_helpers`)
//!         경로 산출 0xd3b2a0 + bumpalo 이동 0xd7bdf0 + drop 0x128f0(★페이로드 쓰기 — verify 는 게임이 쓴 결과를 읽고, 사전 스냅샷으로 "이번 호출에 새로 산출했나" 를 판별)
//!   WorldOps: +0x40 모드(타겟 Vec = 모드+0x1a0/0x1a8) · +0x1f0 리졸버 · +0xe8 cfg_flag · +0x108 side_cfg
//! 계약(디컴 순서):
//!   S0 side<2·me=roster[role]≠0 · payload+0x18/+0x19 ≠0 → (타겟 있음 ? code 0x12{+8=0,+0x10=0,+0x11=f18} : code 10{+8(1B)=0})
//!   pct=hp*100/maxhp · pick=ability_pick.r0 · 타겟 없음 → code 10
//!   홈박스(G+0x20 obj+0x6d70+side*0x20) 밖: (pct>50 && pick==0) ‖ 타겟 hp≠max → 진행, 아니면 5 / 안: 타겟 풀피 && (pick≠0 ‖ pct<51 ‖ (hp<max && y≤yhi)) → 5, 아니면 진행
//!   order+0x41f≠0 → 5 · order+0x420==3 → (cfg_flag? sim+0x4f8 : side_cfg).u32==role && can_attack(4) ? code 2{+8=0,+9=subtype,+10=2} : code 0xb{+8=0}
//!   홈 띠(side별 64000/960000·0xd9c5f/0xdabff) 안 && hp<max → 5
//!   min(p7+0x88, p7+0x98) > tps*5 이면: 경로(payload+0x1a/+8/+0x10) 없으면 산출→성공 시 5 / 있으면 경로 핸들 중 me 200000 안 → 5 · 그리드(G obj+0x38b8, 30×30 u64, 값 7)&&recently_seen 카운트 적>아군 && pct<21 → 5
//!   engage_gate(4)==0 && dist²(me,타겟)<150000²+1 → code 0xc · 그 외 code 10
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut, tr, live_imm8, cap_ability_pick};
use super::passive_line_callees::lane_pred;
use super::obj_helpers::{can_attack, engage_gate};

/// 원본 호출 전 페이로드 경로 상태 스냅샷(+0x1a flag, +0x10 len, +8 ptr) — 원본이 이 호출에서 경로를 새로 쓰거나 지우기 때문.
thread_local! { static PRE: core::cell::Cell<(u8, u8, u64, u64)> = const { core::cell::Cell::new((0, 0, 0, 0)) }; }
pub unsafe fn pre_snapshot(payload: usize) {
    if ptr_ok(payload) { PRE.with(|c| c.set((1, rd_u8(payload + HP_PL_HASPATH), rd_u64(payload + HP_PL_PATH_LEN).unwrap_or(0), rd_u64(payload + HP_PL_PATH_PTR).unwrap_or(0)))); }
    else { PRE.with(|c| c.set((0, 0, 0, 0))); }
}

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] unsafe fn grid7(gobj: usize, x: u64, y: u64) -> Option<bool> {
    let gy = (y / 32000).min(29) as usize; let gx = (x / 32000).min(29) as usize;
    Some(rd_u64(gobj + G_GRID + gy * G_GRID_ROW + gx * 8)? == 7)
}

pub unsafe fn epic_hunt_poke(a: &Args8) -> Option<MpOut> {
    let (payload, p3, p5, p6, p7, p8) = (a.p2, a.p3, a.p5, a.p6, a.p7, a.p8);
    if !ptr_ok(payload) || !ptr_ok(p5) || !ptr_ok(p7) || !ptr_ok(p8) { return None; }
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }                     // 게임: index panic
    let role = rd_u32(p5 + P5_ROLE);
    let h = Holder::new(p6)?; let w = h.world()?; let g = h.g()?;
    let me = w.roster(side, role)?; if me == 0 { return None; }                         // 게임: unwrap panic
    let f18 = rd_u8(payload + HP_PL_F18); let f19 = rd_u8(payload + HP_PL_F19);
    let mut o = MpOut::default();
    let moba = w.moba()?;                                                               // tag≠0 = 게임 panic
    let tgt = match w.first_target(moba, T0_LEN, T0_PTR)? { Some(hh) => w.entity(hh).map(|e| e.0), None => None };
    tr(0, 0x100 | f18 as u64 | (f19 as u64) << 8 | (tgt.is_some() as u64) << 16 | side << 20 | (role as u64) << 24);
    if f18 != 0 || f19 != 0 {
        if tgt.is_some() { o.push(8, 8, 0); o.push(0x10, 1, 0); o.push(0x11, 1, f18 as u64); o.code(0x12); tr(2, 0x100); }
        else { o.push(8, 1, 0); o.code(10); tr(2, 0x101); }
        return Some(o);
    }
    let maxhp = rd_u64(me + ENT_MAXHP)?; if maxhp == 0 { return None; }               // 게임: div0 panic
    let hp = rd_u64(me + ENT_HP)?; let pct = hp.wrapping_mul(100) / maxhp;
    let pick = cap_ability_pick::take()?.0;                                             // 캡처 없음 → NA
    let tgt = match tgt { Some(t) => t, None => { o.push(8, 1, 0); o.code(10); tr(2, 0x102); return Some(o) } };
    let bx = g.home_box(side)?;
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let outside = mx < bx.xlo || bx.xhi < mx || my < bx.ylo;
    let tgt_full = rd_u64(tgt + ENT_HP)? == rd_u64(tgt + ENT_MAXHP)?;
    tr(1, 0x100 | pct.min(0xff) | (pick.min(0xff)) << 8 | (outside as u64) << 16 | (tgt_full as u64) << 17);
    let proceed = if outside { (pct > 0x32 && pick == 0) || !tgt_full }
                  else { !(tgt_full && (pick != 0 || pct < 0x33 || (hp < maxhp && my <= bx.yhi))) };
    if !proceed { o.code(5); tr(2, 0x103); return Some(o); }
    if rd_u8(p8 + ORDER_PLAN) != 0 { o.code(5); tr(2, 0x104); return Some(o); }
    if rd_u8(p8 + ORDER_SF) == 3 {
        let sel = if w.cfg_flag()? == 0 { rd_u64(w.side_cfg(side)?)? } else { rd_u64(p5 + P5_CFG_SELF)? };
        if (sel & 0xffff_ffff) as u32 == role && can_attack(p5, p6, 4)? {
            let mk = rd_u8(g.0 + G_PHASE);
            let sub = if (0x1ebu32 >> mk) & 1 != 0 { 2 } else if mk == 2 { 0 } else { 1 };
            o.push(8, 1, 0); o.push(9, 1, sub); o.push(10, 1, 2); o.code(2); tr(2, 0x105); return Some(o);
        }
        o.push(8, 1, 0); o.code(0xb); tr(2, 0x106); return Some(o);
    }
    // 홈 띠(side 별 사각 영역) 안이고 풀피가 아니면 5
    // 디컴: xhi = side0?64000:960000 · x<=xhi && (x>0xd9c5f ‖ side0) · yhi = side0?960000:64000 · y<=yhi && (y>0xdabff ‖ side1)
    let in_home = if side == 0 { mx <= HP_HOME_LO && my <= HP_HOME_HI && my > HP_HOME_Y1 }
                  else { mx <= HP_HOME_HI && mx > HP_HOME_X1 && my <= HP_HOME_LO };
    if in_home && hp < maxhp { o.code(5); tr(2, 0x107); return Some(o); }
    let pct2 = pct;
    let tps = g.tps()? as u64;
    let tmin = rd_u64(p7 + HP_P7_A)?.min(rd_u64(p7 + HP_P7_B)?);                         // min(p7+0x88, p7+0x98)
    if tps.wrapping_mul(5) < tmin {
        let (ok, pre_flag, pre_len, pre_ptr) = PRE.with(|c| c.get());
        let mut early5 = false;
        if pre_flag == 0 {
            // 게임이 이번 호출에서 경로를 산출: 성공(+0x1a 가 1 이 됨) → 5
            if ok == 1 && rd_u8(payload + HP_PL_HASPATH) == 1 { early5 = true; tr(3, 0x100); }
            else if ok == 0 { return None; } else { tr(3, 0x101); }
        } else {
            tr(3, 0x102 | pre_len.min(0xff) << 16);
            if pre_len != 0 {
                if !ptr_ok(pre_ptr as usize) { return None; }
                for i in 0..pre_len.min(CAP_ITER) as usize {
                    if let Some(e) = w.entity(rd_u64(pre_ptr as usize + i * 8)?) {
                        if sqd(rd_u64(e.0 + ENT_X)?, rd_u64(e.0 + ENT_Y)?, mx, my) < HP_PATH_NEAR_D2 { early5 = true; tr(3, 0x103 | (i as u64) << 8); break; }
                    }
                }
            }
        }
        if early5 { o.code(5); tr(2, 0x108); return Some(o); }
        // 그리드(값 7) && recently_seen 카운트: 적 vs 아군
        let gobj = rd_u64(g.0 + G_BOXES)? as usize; if !ptr_ok(gobj) { return None; }
        let lanes = rd_u64(p6 + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
        let other = 1 - side; let win = live_imm8(SITE_VW_CHECK_IMM, 0x78) as u64;
        let mut en = 0u32; let mut mine_n = 0u32;
        for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + other as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            let g7 = grid7(gobj, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)?;
            let seen = lane_pred(lanes + other as usize * LANE_STRIDE, w.data, w.vt, p5, e, win)?.0 != 0;
            en += (g7 && seen) as u32; }
        for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + side as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            let g7 = grid7(gobj, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)?;
            let seen = lane_pred(lanes + side as usize * LANE_STRIDE, w.data, w.vt, p5, e, win)?.0 != 0;
            mine_n += (g7 && seen) as u32; }
        tr(4, 0x100 | en as u64 | (mine_n as u64) << 8 | pct2.min(0xff) << 16);
        if pct2 < 0x15 && mine_n < en { o.code(5); tr(2, 0x109); return Some(o); }
    } else { tr(3, 0x1ff); }
    let gate = engage_gate(p8, p5, p6, p7, 4)?;
    tr(5, 0x100 | gate as u64);
    if !gate && sqd(rd_u64(tgt + ENT_X)?, rd_u64(tgt + ENT_Y)?, mx, my) < HP_ENGAGE_D2 { o.code(0xc); tr(2, 0x10a); return Some(o); }
    o.push(8, 1, 0); o.code(10); tr(2, 0x10b);
    let _ = p3;
    Some(o)
}

/// disc 14 serpen hunt_and_poke `0xdf0e90`(829명령, capstone 포팅 — 디컴은 JT 오염). 에픽과 다른 점:
///   인터럽트 → code 0x12{+8=0,+0x10=**1**,+0x11=f18} / 타겟 없음 → code **0xd**{+8=0} · order+0x41f **==1** 필수(아니면 5) · 0x420==3 → can_attack(**5**) → code 2 서브타입 = HP_S_SUBTYPE[mk] / 아니면 **0xe**
///   타이머 min(p7+0xc0, p7+0xd0) · 경로 블록 동일 · 그리드 대신 **타겟 150000 안 && recently_seen** 카운트(적 vs 아군) · pct≤20 && 아군<적 → 5
///   engage_gate(5) ? **0xd**{+8=0} : **0xf**(코드만)
pub unsafe fn serpen_hunt_poke(a: &Args8) -> Option<MpOut> {
    let (payload, p5, p6, p7, p8) = (a.p2, a.p5, a.p6, a.p7, a.p8);
    if !ptr_ok(payload) || !ptr_ok(p5) || !ptr_ok(p7) || !ptr_ok(p8) { return None; }
    let side = rd_u64(p5 + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(p5 + P5_ROLE);
    let h = Holder::new(p6)?; let w = h.world()?; let g = h.g()?;
    let me = w.roster(side, role)?; if me == 0 { return None; }
    let f18 = rd_u8(payload + HP_PL_F18); let f19 = rd_u8(payload + HP_PL_F19);
    let mut o = MpOut::default();
    let moba = w.moba()?;
    let tgt = match w.first_target(moba, T1_LEN, T1_PTR)? { Some(hh) => w.entity(hh).map(|e| e.0), None => None };
    tr(0, 0x200 | f18 as u64 | (f19 as u64) << 8 | (tgt.is_some() as u64) << 16 | side << 20 | (role as u64) << 24);
    if f18 != 0 || f19 != 0 {
        if tgt.is_some() { o.push(8, 8, 0); o.push(0x10, 1, 1); o.push(0x11, 1, f18 as u64); o.code(0x12); tr(2, 0x200); }
        else { o.push(8, 1, 0); o.code(0xd); tr(2, 0x201); }
        return Some(o);
    }
    let maxhp = rd_u64(me + ENT_MAXHP)?; if maxhp == 0 { return None; }
    let hp = rd_u64(me + ENT_HP)?; let pct = hp.wrapping_mul(100) / maxhp;
    let tgt = match tgt { Some(t) => t, None => { o.push(8, 1, 0); o.code(0xd); tr(2, 0x202); return Some(o) } };
    let pick = cap_ability_pick::take()?.0;
    let bx = g.home_box(side)?;
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let outside = mx < bx.xlo || bx.xhi < mx || my < bx.ylo;
    let tgt_full = rd_u64(tgt + ENT_HP)? == rd_u64(tgt + ENT_MAXHP)?;
    tr(1, 0x200 | pct.min(0xff) | (pick.min(0xff)) << 8 | (outside as u64) << 16 | (tgt_full as u64) << 17);
    let proceed = if outside { !tgt_full || (pct >= 0x33 && pick == 0) }
                  else { !tgt_full || (!(hp < maxhp && my <= bx.yhi) && pct >= 0x33 && pick == 0) };
    if !proceed { o.code(5); tr(2, 0x203); return Some(o); }
    if rd_u8(p8 + ORDER_PLAN) != 1 { o.code(5); tr(2, 0x204); return Some(o); }
    if rd_u8(p8 + ORDER_SF) == 3 {
        let sel = if w.cfg_flag()? == 0 { rd_u64(w.side_cfg(side)?)? } else { rd_u64(p5 + P5_CFG_SELF)? };
        if (sel & 0xffff_ffff) as u32 == role && can_attack(p5, p6, 5)? {
            let mk = rd_u8(g.0 + G_PHASE); if mk > 8 { return None; }                   // JT 범위 밖 = UB
            o.push(8, 1, 0); o.push(9, 1, HP_S_SUBTYPE[mk as usize]); o.push(10, 1, 2); o.code(2); tr(2, 0x205); return Some(o);
        }
        o.push(8, 1, 0); o.code(0xe); tr(2, 0x206); return Some(o);
    }
    let in_home = if side == 0 { mx <= HP_HOME_LO && my <= HP_HOME_HI && my > HP_HOME_Y1 }
                  else { mx <= HP_HOME_HI && mx > HP_HOME_X1 && my <= HP_HOME_LO };
    if in_home && hp < maxhp { o.code(5); tr(2, 0x207); return Some(o); }
    let pct2 = pct;
    let tps = g.tps()? as u64;
    let tmin = rd_u64(p7 + HP_S_P7_A)?.min(rd_u64(p7 + HP_S_P7_B)?);
    if tps.wrapping_mul(5) < tmin {
        let (ok, pre_flag, pre_len, pre_ptr) = PRE.with(|c| c.get());
        let mut early5 = false;
        if pre_flag == 0 {
            if ok == 1 && rd_u8(payload + HP_PL_HASPATH) == 1 { early5 = true; tr(3, 0x200); }
            else if ok == 0 { return None; } else { tr(3, 0x201); }
        } else {
            tr(3, 0x202 | pre_len.min(0xff) << 16);
            if pre_len != 0 {
                if !ptr_ok(pre_ptr as usize) { return None; }
                for i in 0..pre_len.min(CAP_ITER) as usize {
                    if let Some(e) = w.entity(rd_u64(pre_ptr as usize + i * 8)?) {
                        if sqd(rd_u64(e.0 + ENT_X)?, rd_u64(e.0 + ENT_Y)?, mx, my) < HP_PATH_NEAR_D2 { early5 = true; tr(3, 0x203 | (i as u64) << 8); break; }
                    }
                }
            }
        }
        if early5 { o.code(5); tr(2, 0x208); return Some(o); }
        // vt+0xe0 정글 상태(w+0xed18)+0x1b8/+0x1c0 = 모드+0x1d0/+0x1d8 = 같은 세르펜 타겟 → 없으면 5
        let tgt2 = match w.first_target(moba, T1_LEN, T1_PTR)? { Some(hh) => match w.entity(hh) { Some(e) => e.0, None => { o.code(5); tr(2, 0x209); return Some(o) } }, None => { o.code(5); tr(2, 0x209); return Some(o) } };
        let (tx, ty) = (rd_u64(tgt2 + ENT_X)?, rd_u64(tgt2 + ENT_Y)?);
        let lanes = rd_u64(p6 + HOLDER_LANES)? as usize; if !ptr_ok(lanes) { return None; }
        let other = 1 - side; let win = live_imm8(SITE_VW_CHECK_IMM, 0x78) as u64;
        let mut en = 0u32; let mut mine_n = 0u32;
        for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + other as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            if lane_pred(lanes + other as usize * LANE_STRIDE, w.data, w.vt, p5, e, win)?.0 == 0 { continue; }
            en += (sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, tx, ty) < HP_ENGAGE_D2) as u32; }
        for i in 0..5 { let e = rd_u64(w.x + X_ROSTER + side as usize * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            if lane_pred(lanes + side as usize * LANE_STRIDE, w.data, w.vt, p5, e, win)?.0 == 0 { continue; }
            mine_n += (sqd(rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?, tx, ty) < HP_ENGAGE_D2) as u32; }
        tr(4, 0x200 | en as u64 | (mine_n as u64) << 8 | pct2.min(0xff) << 16);
        if pct2 <= 0x14 && mine_n < en { o.code(5); tr(2, 0x20a); return Some(o); }
    } else { tr(3, 0x2ff); }
    let gate = engage_gate(p8, p5, p6, p7, 5)?;
    tr(5, 0x200 | gate as u64);
    if gate { o.push(8, 1, 0); o.code(0xd); tr(2, 0x20b); } else { o.code(0xf); tr(2, 0x20c); }
    Some(o)
}

