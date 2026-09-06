//! passive_jungle — game-ai\src\plan_legacy\old\passive_jungle.rs:143 (Plan 7 정글 핸들러)
//!   게임 0.5.8 · RVA 0xd2e500 (2993B / 699명령) · capstone 디스어셈 포팅(scratchpad pj_d2e500.txt) + RE = REPORT\RE\2026-09-06_passive_jungle-…
//!   콜리: estimate_damage 0x12857f0(아래 `estimate_damage`) · camp_pos 0xffa3e0(아래 `camp_pos`, 메모는 의미 없음 → 재계산) · HeapAlloc/memcpy(Vec 복사 = 재현 불요)
//!   dyn: 프로바이더 +0x90 cooltime · effect +0x28/+0x38(몬스터 평타 피해) · effect +0x40 heal / +0xa0 BuffState(내 스킬1·2) → `dyn_eff.rs` RVA 디스패치(미재현 = NA + 로그)
//!
//! 계약: p1=out(tag u64: 5=불가 / 6=Some(+8 side u64, +0x10 camp u8, +0x11 0)) · p2=플랜(+0x48 side, +0x60 camp u8) · p5=선수 sim · p6=&Holder(X, W(G), …)
//! 흐름: S0 홈존&&hp<max → 5 │ A 캠프 Vec(모드데이터+off)·캠프 위치 120000 안이면 몬스터 dps→ttk>tps → 6 │ S 지속회복(sus) 판정 │ P2 몬스터가 나를 치는 중이면 P3, 아니면 hp% ≥ (sus?21:41) → 6 │ P3 dps 재계산(거리게이트 없음) ttk>tps → 6
//! 노브: d7_hp_selfheal(21)·d7_hp_normal(41) = 바이트패치 사이트(0xd2ed57/0xd2ed6d) → 검증판은 라이브 즉치 · d7_wp_dist2(120000²) 는 사이트 없음(원본 상수).
use crate::*;
use super::super::world::*;
use super::super::layout::*;
use super::super::{Args8, MpOut, tr, live_imm8};
use super::dyn_eff as dy;

#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[derive(Clone, Copy)]
pub struct Knobs { pub hp_selfheal: u64, pub hp_normal: u64, pub wp_d2: u64 }
impl Knobs {
    pub unsafe fn game_equiv() -> Knobs { Knobs { hp_selfheal: live_imm8(SITE_PJ_HP_SELFHEAL_IMM, 0x15) as u64, hp_normal: live_imm8(SITE_PJ_HP_NORMAL_IMM, 0x29) as u64, wp_d2: PJ_WP_D2 } }
    pub unsafe fn from_cfg() -> Knobs {
        let k = |key: &str, orig: i64| { let v = tune(key, -1); if v < 0 { orig } else { v } };
        Knobs { hp_selfheal: k("d7_hp_selfheal", 21) as u64, hp_normal: k("d7_hp_normal", 41) as u64, wp_d2: k("d7_wp_dist2", PJ_WP_D2 as i64) as u64 }
    }
}
pub unsafe fn passive_jungle(a: &Args8) -> Option<MpOut> { passive_jungle_k(a, &Knobs::game_equiv()) }
pub unsafe fn passive_jungle_live(a: &Args8) -> Option<MpOut> { passive_jungle_k(a, &Knobs::from_cfg()) }

/// 0xffa3e0 — 맵 정의 캠프 표(`map_def+0x68` ptr / `+0x70` len, stride 0x28: {pos_side0 @0, pos_side1 @0x10, kind u8 @0x20})에서 kind==camp 첫 항목의 side 위치. 없으면 (0,0). 스레드로컬 메모는 결과에 영향 없음.
pub unsafe fn camp_pos(map_def: usize, camp: u8, side: usize) -> Option<(u64, u64)> {
    if camp as u64 >= 8 || side >= 2 { return None; }                           // 게임: index panic
    let ptr = rd_u64(map_def + MAPDEF_CAMPS_PTR)? as usize; let n = rd_u64(map_def + MAPDEF_CAMPS_LEN)?;
    if n == 0 { return Some((0, 0)); } if !ptr_ok(ptr) { return None; }
    for i in 0..n.min(64) as usize {
        let e = ptr + i * 0x28;
        if rd_u8(e + 0x20) == camp { return Some((rd_u64(e + side * 0x10)?, rd_u64(e + side * 0x10 + 8)?)); }
    }
    Some((0, 0))
}

/// 0x12857f0 — 공격자 슬롯(slot=att+0x490: [0] Arc data·[8] vt·+0x2c atk_type u32)의 대상(target) 예상 피해.
///   (p,m)=eff28 · p += eff38*target.maxhp/100 · 공격자 버프블록(att+0x370)의 +0xd0/+0xd8/+0xe0/+0xf0 보정 · 관통 +0xa8/+0xb0 · 방어 target+0x630/+0x638 · 결과 max(p,1)+max(m,1)
pub unsafe fn estimate_damage(slot: usize, att: usize, target: usize, dbg: bool) -> Option<u64> {
    let (data, vt) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    let (mut p, mut m) = dy::eff28_damage(data, vt, att)?;                    // +0x30 은 단일 구현(항상 None) → kind 무관하게 +0x28
    let x38 = dy::eff38_pct(data, vt, att)?;
    if dbg {
        let me_ = dy::arc_payload(data, vt).unwrap_or(0);
        let g = |o: usize| if me_ != 0 { rd_u64(me_ + o).unwrap_or(0xffff).min(0xffff) } else { 0xffff };
        tr(6, g(0x18) | g(0x20) << 16 | rd_u64(att + ENT_STATS).unwrap_or(0).min(0xffff) << 32 | g(0x10) << 48);
        tr(11, rd_u64(target + ENT_DEF_P).unwrap_or(0).min(0xffff) | rd_u64(att + ENT_BUFF_BLOCK + 0xa8).unwrap_or(0).min(0xff) << 16 | (rd_u32(slot + 0x2c) as u64 & 0xff) << 24 | p.min(0xffff) << 32 | m.min(0xffff) << 48);
    }
    let tmax = rd_u64(target + ENT_MAXHP)?;
    if x38 != 0 { p = p.wrapping_add(x38.wrapping_mul(tmax) / 100); }
    if p == 0 && m == 0 { return Some(0); }
    let atk = rd_u32(slot + 0x2c);
    let stats = att + ENT_STATS; let buffs = att + ENT_BUFF_BLOCK;
    let apply = |x: u64| -> Option<u64> {
        let mut x = x;
        if atk.wrapping_sub(2) < 2 {                                              // 2,3
            let f0 = rd_u64(buffs + 0xf0)?; if f0 != 0 { x = (f0.wrapping_add(100)).wrapping_mul(x) / 100; }
        } else {
            if atk == 0 { let d0 = rd_u64(buffs + 0xd0)?; if d0 != 0 { x = x.wrapping_add(d0.wrapping_mul(tmax) / 100); } }
            else { let e0 = rd_u64(buffs + 0xe0)?; if e0 != 0 { x = x.wrapping_add(e0.wrapping_mul(tmax) / 100); } }
            let d8 = rd_u64(buffs + 0xd8)?; if d8 != 0 { x = x.wrapping_add(d8.wrapping_mul(rd_u64(stats + 0x10)?) / 100); }
        }
        Some(x)
    };
    p = apply(p)?;
    let mut defp = rd_u64(target + ENT_DEF_P)?; let penp = rd_u64(buffs + 0xa8)?;
    if penp != 0 { defp = (if penp < 0x65 { 100 - penp } else { 0 }).wrapping_mul(defp) / 100; }
    defp = defp.wrapping_add(100); if defp == 0 { return None; }               // 게임: div0 panic
    p = p.wrapping_mul(100) / defp;
    m = apply(m)?;
    let mut defm = rd_u64(target + ENT_DEF_M)?; let penm = rd_u64(buffs + 0xb0)?;
    if penm != 0 { defm = (if penm < 0x65 { 100 - penm } else { 0 }).wrapping_mul(defm) / 100; }
    defm = defm.wrapping_add(100); if defm == 0 { return None; }
    m = m.wrapping_mul(100) / defm;
    Some(p.max(1).wrapping_add(m.max(1)))
}

/// 캠프 인덱스·사이드 → 모드데이터(MobaData) 안 Vec 오프셋(점프테이블 0x33da0dc/0x33da0f4/0x33da10c 공통)
#[inline] fn camp_off(camp: u8, side: bool) -> Option<usize> {
    Some(match camp { 0 => if side { 0xc0 } else { 0 }, 1 => if side { 0xf0 } else { 0x30 }, 2 => if side { 0x120 } else { 0x60 }, 3 => if side { 0x150 } else { 0x90 }, 4 => 0x180, 5 => 0x1b0, _ => return None })
}
/// 몬스터 e 의 초당 피해×1000: dmg*1000 / per, per = max(cd*100/max(1, spd+100), 3 if <4)
unsafe fn monster_dps(e: usize, me: usize, slot_tr: usize) -> Option<u64> {
    tr(9, 0x100 | 0x20);
    let vt28 = rd_u64(e + ENT_SLOT0 + 8).unwrap_or(0) as usize;
    let impl28 = dy::impl_rva(vt28, 0x28).unwrap_or(0xffff) as u64;
    let dmg = match estimate_damage(e + ENT_SLOT0, e, me, slot_tr == 7) { Some(v) => v, None => { tr(9, 0x100 | 0x21); tr(slot_tr, 0x8000_0000_0000_0000 | (impl28 & 0xffff) << 48); return None; } };
    tr(9, 0x100 | 0x22);
    let cd = dy::prov90_cooltime(rd_u64(e + ENT_PROV0_DATA)? as usize, rd_u64(e + ENT_PROV0_VT)? as usize, e)?;
    tr(9, 0x100 | 0x23);
    let mut spd = (rd_i32(e + ENT_F3FC)? as i64).wrapping_add(100); if spd < 2 { spd = 1; }
    let mut per = cd.wrapping_mul(100) / (spd as u64); if per < 4 { per = 3; }
    if slot_tr < 12 { tr(slot_tr, dmg.min(0xffff) | cd.min(0xffff) << 16 | ((spd as u64) & 0xffff) << 32 | (impl28 & 0xffff) << 48); }
    Some(dmg.wrapping_mul(1000) / per)
}

pub unsafe fn passive_jungle_k(a: &Args8, k: &Knobs) -> Option<MpOut> {
    let (p2, p5, p6) = (a.p2, a.p5, a.p6);
    if !ptr_ok(p2) || !ptr_ok(p5) { return None; }
    let team = rd_u64(p5 + P5_SIDE)?; if team > 1 { return None; }
    let slot = rd_u32(p5 + P5_ROLE);
    let h = Holder::new(p6)?; let w = h.world()?; let g = h.g()?;
    let me = w.roster(team, slot)?; if me == 0 { return None; }                 // 게임: panic
    let side_flag = rd_u64(p2 + PLAN_PJ_SIDE)?; let camp = rd_u8(p2 + PLAN_PJ_CAMP);
    let side = side_flag != 0;
    let (mx, my) = (rd_u64(me + ENT_X)?, rd_u64(me + ENT_Y)?);
    let (hp, maxhp) = (rd_u64(me + ENT_HP)?, rd_u64(me + ENT_MAXHP)?);
    tr(0, 0x100 | camp as u64 | (side as u64) << 8); tr(1, 0x100 | team | (slot as u64) << 4);
    let mut o = MpOut::default();
    let tag5 = |o: &mut MpOut| { o.code(5); };
    let tag6 = |o: &mut MpOut| { o.push(8, 8, side_flag); o.push(0x10, 1, camp as u64); o.push(0x11, 1, 0); o.code(6); };
    // S0
    if g.home_box(team)?.contains(mx, my) && hp < maxhp { tag5(&mut o); tr(2, 0x100); return Some(o); }
    tr(9, 0x100 | 1);
    let (tag, moba) = w.mode()?; if tag != 0 { tr(9, 0x100 | 2); return None; }                   // 게임: unwrap None panic
    tr(9, 0x100 | 3);
    let off = camp_off(camp, side)?;                                            // 테이블 밖 = UB
    tr(9, 0x100 | 4);
    let vptr = rd_u64(moba + off + 0x20)? as usize; let vlen = rd_u64(moba + off + 0x28)?;
    if vlen > 0 && !ptr_ok(vptr) { return None; }
    let tps = rd_u64(rd_u64(g.0 + G_CFG)? as usize + CFG_TPS)?;
    let map_def = rd_u64(g.0 + G_BOXES)? as usize;
    tr(9, 0x100 | 5);
    // A
    if vlen != 0 {
        let (cx, cy) = camp_pos(map_def, camp, side as usize)?;
        tr(9, 0x100 | 6);
        let d2 = sqd(mx, my, cx, cy); tr(3, d2.min(0xffff_ffff_ffff));
        if d2 <= k.wp_d2 {
            let mut dps: u64 = 0; let (mut nres, mut nskip) = (0u64, 0u64);
            for i in 0..vlen.min(64) as usize {
                let e = match w.entity(rd_u64(vptr + i * 8)?) { Some(e) => e.0, None => continue };
                nres += 1;
                if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { nskip += 1; continue; }
                dps = dps.wrapping_add(monster_dps(e, me, 7 + i.min(1) * 4)?);
            }
            tr(10, 0x100_0000 | vlen.min(0xff) | nres << 8 | nskip << 16);
            tr(4, 0x1_0000_0000 | dps.min(0xffff_ffff));
            if dps != 0 { if hp.wrapping_mul(1000) / dps > tps { tag6(&mut o); tr(2, 0x101); return Some(o); } }
            else { tag6(&mut o); tr(2, 0x102); return Some(o); }
        }
    }
    // S — 지속 회복(sus)
    tr(9, 0x100 | 7);
    let sus: bool = if rd_i32(me + ENT_F3F0)? > 0 { true } else {
        let mut r: Option<bool> = None;
        if rd_i32(me + ENT_SLOT1_FLAG)? != -1 {
            let (d1, v1) = (rd_u64(me + ENT_SLOT1)? as usize, rd_u64(me + ENT_SLOT1 + 8)? as usize);
            tr(9, 0x100 | 0x71); tr(8, v1 as u64);
            let heal = dy::eff40_heal(d1, v1, me)?;
            tr(9, 0x100 | 0x72);
            if maxhp.saturating_sub(hp).min(heal) != 0 { r = Some(true); }
            else { let (ty, vamp) = dy::effa0_buff(d1, v1, me)?; tr(9, 0x100 | 0x73); if ty != -1 && vamp > 0 { r = Some(true); } }
        }
        match r { Some(b) => b, None => {
            if rd_u64(me + ENT_LEVEL)? < 3 || rd_i32(me + ENT_SLOT2_FLAG)? == -1 { false } else {
                let (d2_, v2) = (rd_u64(me + ENT_SLOT2)? as usize, rd_u64(me + ENT_SLOT2 + 8)? as usize);
                tr(9, 0x100 | 0x74); tr(8, v2 as u64);
                let heal = dy::eff40_heal(d2_, v2, me)?;
                tr(9, 0x100 | 0x75);
                if maxhp.saturating_sub(hp).min(heal) != 0 { true } else { let (ty, vamp) = dy::effa0_buff(d2_, v2, me)?; tr(9, 0x100 | 0x76); ty != -1 && vamp > 0 }
            }
        } }
    };
    tr(5, 0x100 | sus as u64); tr(9, 0x100 | 8);
    // P2 — 몬스터(kind 4)가 나를 치는 중인가
    let myh = rd_u64(me + ENT_HANDLE)?;
    let mut engaged = false;
    for i in 0..vlen.min(64) as usize {
        let e = match w.entity(rd_u64(vptr + i * 8)?) { Some(e) => e.0, None => continue };
        if rd_i32(e + ENT_KIND)? == 4 && rd_i32(e + ENT_F88)? == 1 && rd_u64(e + ENT_TARGET_H)? == myh { engaged = true; break; }
    }
    tr(6, 0x100 | engaged as u64);
    if !engaged {
        if maxhp == 0 { return None; }                                          // 게임: div0 panic
        let pct = hp.wrapping_mul(100) / maxhp; tr(7, 0x100 | pct.min(0xff));
        if pct >= (if sus { k.hp_selfheal } else { k.hp_normal }) { tag6(&mut o); tr(2, 0x103); } else { tag5(&mut o); tr(2, 0x104); }
        return Some(o);
    }
    // P3 — dps 재계산(거리 게이트 없음, 슬롯 flag −1 이면 게임 panic)
    let mut dps: u64 = 0;
    for i in 0..vlen.min(64) as usize {
        let e = match w.entity(rd_u64(vptr + i * 8)?) { Some(e) => e.0, None => continue };
        if rd_i32(e + ENT_SLOT0_FLAG)? == -1 { tr(9, 0x100 | 9); return None; }
        dps = dps.wrapping_add(monster_dps(e, me, 7 + i.min(1) * 4)?);
    }
    let dps = if vlen == 0 { 1 } else { dps.max(1) };
    tr(4, 0x2_0000_0000 | dps.min(0xffff_ffff));
    if hp.wrapping_mul(1000) / dps > tps { tag6(&mut o); tr(2, 0x105); } else { tag5(&mut o); tr(2, 0x106); }
    Some(o)
}
