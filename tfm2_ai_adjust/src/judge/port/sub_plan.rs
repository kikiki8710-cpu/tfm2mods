//! `plan_legacy::types::BigPlan::sub_plan`(RVA `0xcaf9f0`) — **모든 플랜의 서브플랜 결정이 지나는 깔때기**.
//!   정본 = SDK IR `_gaibc\m02.ll` `BigPlan::sub_plan`(types.rs L233~250) + 인라인된 각 플랜 `sub_plan`.
//!   계약: `sub_plan(sret[72], self, version, p3, player, data, p6, p7, p8, p9)` — Win64 로
//!     `p1=sret · p2=self · p3=version · p4=p3 · p5=player · p6=data · p7.. = 나머지`.
//!   분기 = `idx = if self[0] > 1 { self[0] - 2 } else { 4 }` 의 16-arm 스위치(`self[0] != 6` 가정).
//!
//! ★출력은 **arm 마다 쓰는 바이트가 다르다**(나머지는 미초기화 쓰레기) → 재현도 (값, 기록마스크) 로 돌려주고
//!   대조는 **마스크된 바이트만** 한다. 전 바이트를 비교하면 쓰레기 때문에 가짜 DIFF 가 난다.
#![allow(dead_code)]
use crate::*;
use super::super::layout::*;
use super::combat_score::na_tag;

/// 72B sret + 기록 마스크
#[derive(Clone, Copy)]
pub struct SubPlanOut { pub b: [u8; 72], pub m: [u8; 72] }
impl SubPlanOut {
    fn new() -> SubPlanOut { SubPlanOut { b: [0; 72], m: [0; 72] } }
    #[inline] fn w8(&mut self, o: usize, v: u8) { self.b[o] = v; self.m[o] = 1; }
    #[inline] fn w64(&mut self, o: usize, v: u64) {
        let by = v.to_le_bytes();
        for i in 0..8 { self.b[o + i] = by[i]; self.m[o + i] = 1; }
    }
    /// 게임이 쓴 sret 과 마스크 구간만 비교
    pub unsafe fn eq_game(&self, p: usize) -> Option<bool> {
        for i in 0..72 { if self.m[i] != 0 && rd_u8(p + i) != self.b[i] { return Some(false); } }
        Some(true)
    }
    pub fn tag(&self) -> u64 { u64::from_le_bytes(self.b[0..8].try_into().unwrap()) }
}

const AN_REGION: usize = 28016;      // cfg + 28016 + side*32 = 팀별 (lx, ly, rx, ry)
const TOWERV_LEN: usize = 328;       // x + 328 + side*32 = 그 팀 여분 타워 Vec 의 len

/// arm 14 — `AttackNexusPlan::sub_plan`(m12.ll:34867, attack_nexus.rs L36~52)
unsafe fn arm_attack_nexus(sf: usize, player: usize, data: usize) -> Option<SubPlanOut> {
    let mut o = SubPlanOut::new();
    let side = rd_u64(player + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(player + P5_ROLE) as usize; if role >= 5 { return None; }
    let x = rd_u64(data)? as usize; if !ptr_ok(x) { return None; }
    let me = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + role * 8)? as usize;
    if me == 0 { return None; }                              // 게임은 unwrap 패닉
    let ctx = rd_u64(data + 8)? as usize; if !ptr_ok(ctx) { return None; }
    let cfg = rd_u64(ctx + 32)? as usize; if !ptr_ok(cfg) { return None; }
    let reg = cfg + AN_REGION + (side as usize) * 32;
    let (lx, ly, rx, ry) = (rd_u64(reg)?, rd_u64(reg + 8)?, rd_u64(reg + 16)?, rd_u64(reg + 24)?);
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    if mx >= lx && mx <= rx && my >= ly && my <= ry && rd_u64(me + ENT_HP)? < rd_u64(me + ENT_MAXHP)? {
        o.w64(0, 5); return Some(o);                         // 진영 안 + 체력 부족 → 5
    }
    let opp = 1 - side;
    if rd_u64(x + TOWERV_LEN + (opp as usize) * 32)? == 0 { o.w64(0, 16); return Some(o); }
    o.w8(8, 0); o.w8(9, rd_u8(sf + 8)); o.w8(10, 2); o.w64(0, 2);
    Some(o)
}

/// arm 3 — 인라인(types.rs L237, 콜리 L880): 지원 목표·목표점·전술을 그대로 옮겨 담고 태그 7
unsafe fn arm_support(sf: usize) -> Option<SubPlanOut> {
    let mut o = SubPlanOut::new();
    let st0 = rd_u64(sf + 8)?; let st1 = rd_u64(sf + 16)?;
    let g0 = rd_u64(sf + 96)?; let g1 = rd_u64(sf + 104)?;
    let tactic = rd_u8(sf + 147); let with_dive = rd_u8(sf + 144);
    o.w64(8, st0); o.w64(16, st1); o.w64(24, g0); o.w64(32, g1); o.w64(40, 0);
    o.w8(48, ((tactic != 1) && (with_dive & 1 == 1)) as u8);
    o.w8(49, with_dive); o.w8(50, 0); o.w8(51, 0); o.w8(52, tactic); o.w8(53, 0);
    o.w64(0, 7);
    Some(o)
}

/// arm 4 — 인라인(types.rs L238, 콜리 L1669) = **라인전(plan 0·1, 실측 최다)**. 태그 = `self[0]` 그대로.
unsafe fn arm_line(sf: usize, variant: u64) -> Option<SubPlanOut> {
    let mut o = SubPlanOut::new();
    o.w64(0, variant);
    o.w64(8, rd_u64(sf + 8)?);
    for i in 0..24 { o.w8(16 + i, rd_u8(sf + 168 + i)); }     // memcpy 24B
    o.w64(40, rd_u64(sf + 232)?);
    o.w64(48, rd_u64(sf + 240)?);
    o.w64(56, rd_u64(sf + 288)?);
    o.w8(64, rd_u8(sf + 371)); o.w8(65, rd_u8(sf + 368)); o.w8(66, rd_u8(sf + 372));
    o.w8(67, rd_u8(sf + 374)); o.w8(68, rd_u8(sf + 375)); o.w8(69, rd_u8(sf + 376));
    Some(o)
}

/// ★`BigPlan::sub_plan` 본체. 미포팅 arm 은 `na_tag("SPn")` 으로 남긴다(어느 플랜이 실제로 뜨는지 측정용).
pub unsafe fn big_plan_sub_plan(sf: usize, _version: u64, player: usize, data: usize) -> Option<SubPlanOut> {
    if !ptr_ok(sf) { return None; }
    let v = rd_u64(sf)?;
    let idx = if v > 1 { v.wrapping_sub(2) } else { 4 };
    match idx {
        0 => { let mut o = SubPlanOut::new(); o.w64(0, 5); Some(o) }
        6 => { let mut o = SubPlanOut::new(); o.w64(0, 5); Some(o) }
        3 => arm_support(sf),
        4 => arm_line(sf, v),
        14 => arm_attack_nexus(sf + 8, player, data),
        1 => { let _ = na_tag("SP_passive_line"); None }
        2 => { let _ = na_tag("SP_single_line"); None }
        5 => { let _ = na_tag("SP_passive_jungle"); None }
        7 => { let _ = na_tag("SP_battle"); None }
        8 => { let _ = na_tag("SP_line_ganker"); None }
        9 => { let _ = na_tag("SP_gank_cover"); None }
        10 => { let _ = na_tag("SP_epic_poke"); None }
        11 => { let _ = na_tag("SP_epic_battle"); None }
        12 => { let _ = na_tag("SP_serpen_poke"); None }
        13 => { let _ = na_tag("SP_serpen_battle"); None }
        15 => { let _ = na_tag("SP_defense_nexus"); None }
        _ => { let _ = na_tag("SP_unreachable"); None }       // 게임은 unreachable
    }
}

/// 진단 — arm 번호와 태그를 남긴다.
pub unsafe fn sp_diag(sf: usize, player: usize, data: usize) -> String {
    let f = || -> Option<String> {
        let v = rd_u64(sf)?;
        let idx = if v > 1 { v.wrapping_sub(2) } else { 4 };
        let side = if ptr_ok(player) { rd_u64(player + P5_SIDE)? } else { 9 };
        let mine = big_plan_sub_plan(sf, 0, player, data);
        Some(format!("variant={} arm={} side={} mine_tag={:?}", v, idx, side, mine.map(|m| m.tag())))
    };
    f().unwrap_or_else(|| "sp diag NA".into())
}
