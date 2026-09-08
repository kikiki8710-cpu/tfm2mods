//! fight_model — `game_ai::plan_legacy::old::fight_model::{resolve_fight_full, resolve_fight_uncached}` 순수 포팅.
//!   정본 = SDK 원본 IR `C:\tfm2mods\_gaibc\m10.ll:39915`(full, 1,086줄) · `:43974`(uncached, 3,480줄), 소스 줄번호 = DI(fight_model.rs L324~376 / L378~521).
//!   콜리 = `fight_check::{fight_dps, expected_dps, self_sustain_in_window, available_cc_in_window}`(m15.ll) · `utils::error_ratio_noise`(m04.ll:49631).
//!   호출자 = `tower_discipline::v47_siege_stance`(0xd96d00, C게이트 `line == Disengage`) · `resolve_fight_stake`(tower_dive_is_viable, plan 핸들러 전용).
//!   ★완전 재구현 원칙: 게임 함수 호출 0. 읽기는 전부 `rd_*`.
#![allow(dead_code)]
use crate::*;
use super::super::layout::*;
use super::super::world::World;
use super::combat_score::na_tag;

const E_KIND: usize = 0x68;
const E_LV: usize = 0x5c8;
const SLOT0: usize = 0x490; const SLOT1: usize = 0x4c8; const SLOT2: usize = 0x500; const SLOT3: usize = 0x538;
const CD_S1: usize = 0xb8; const CD_S2: usize = 0xc0; const CD_ULT: usize = 0xc8;
const E_400: usize = 0x400;
const PROV_S1: usize = 0x580; const PROV_S2: usize = 0x590;
const BIG: i64 = 2_305_843_009_213_693_951;   // i64::MAX >> 2 (IR 즉치)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FightPrediction {
    pub focus: Option<u64>, pub soaker: Option<u64>, pub rescue: Option<u64>,
    pub net: i64, pub line: u8, pub line_abs: u8,
}
pub const LINE_COMMIT: u8 = 0; pub const LINE_COMMIT_AFTER_JOIN: u8 = 1; pub const LINE_DISENGAGE: u8 = 2; pub const LINE_HOLD: u8 = 3;

/// 게임 sret(64B) 레이아웃(DI `FightPrediction`): [0]=focus tag [8]=focus id [16]=soaker tag [24]=id [32]=rescue tag [40]=id [48]=net [56]=line [57]=line_absolute
impl FightPrediction {
    pub unsafe fn from_sret(p: usize) -> Option<FightPrediction> {
        let opt = |o: usize| -> Option<Option<u64>> { Some(if rd_u64(p + o)? & 1 == 1 { Some(rd_u64(p + o + 8)?) } else { None }) };
        Some(FightPrediction { focus: opt(0)?, soaker: opt(16)?, rescue: opt(32)?, net: rd_i64(p + 48)?, line: rd_u8(p + 56), line_abs: rd_u8(p + 57) })
    }
}

#[inline] unsafe fn xy(e: usize) -> Option<(u64, u64)> { Some((rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)) }
#[inline] fn sqd(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = if ax < bx { bx - ax } else { ax - bx }; let dy = if ay < by { by - ay } else { ay - by };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] unsafe fn d2(a: usize, b: usize) -> Option<u64> { let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?; Some(sqd(ax, ay, bx, by)) }

// ── utils::error_ratio_noise (m04.ll:49631) ───────────────────────────────────────────────
/// `NoiseRng(u64)` — splitmix64 변형. `(1000-acc)/20 = n`, 범위 `2n+1`, 반환 `100 - n + (mix(state) * range >> 64)`.
pub struct NoiseRng(pub u64);
pub fn error_ratio_noise(rng: &mut NoiseRng, acc: u64) -> i64 {
    let n = 1000u64.wrapping_sub(acc) / 20;
    let range = (n << 1) | 1;
    rng.0 = rng.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = rng.0;
    z ^= z >> 30; z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 27; z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let r = ((z as u128 * range as u128) >> 64) as i64;
    r.wrapping_sub(n as i64).wrapping_add(100)
}

// ── fight_check 콜리 4종 (m15.ll) ─────────────────────────────────────────────────────────
#[inline] unsafe fn tps_of(ctx: usize) -> Option<u64> { let cfg = rd_u64(ctx + G_CFG)? as usize; if !ptr_ok(cfg) { return None; } rd_u64(cfg + CFG_TPS) }
/// 스킬 슬롯 k(1..3) 의 (슬롯 주소 or None(EMPTY 정적 슬롯 = 태그 −1), 쿨다운 오프셋)
#[inline] unsafe fn slot_k(e: usize, k: u32) -> Option<(Option<usize>, usize)> {
    let lv = rd_u64(e + E_LV)?;
    Some(match k {
        1 => (Some(e + SLOT1), CD_S1),
        2 => (if lv > 2 { Some(e + SLOT2) } else { None }, CD_S2),
        _ => (if lv > 4 { Some(e + SLOT3) } else { None }, CD_ULT),
    })
}
/// IR 공통 술어: `can_skill_k(e) || !(kind==13 && cd_k > window)` = "창(window) 안에 스킬 k 를 쓸 수 있다"
#[inline] unsafe fn skill_in_window(e: usize, k: u32, window: u64) -> Option<bool> {
    if super::fight_check::pred_skill(e, k)? { return Some(true); }
    let (_, cd) = slot_k(e, k)?;
    Some(!(rd_u64(e + E_KIND)? == 13 && rd_u64(e + cd)? > window))
}
/// `Entity::skill_cooltime`(g06.ll:66405, max 3) / `skill2_cooltime`(:66747, max 1): prov.vt+0x90(data,e)*100 / max(e.0x400+100,1)
unsafe fn skill_cooltime(e: usize, k: u32) -> Option<u64> {
    let off = if k == 1 { PROV_S1 } else { PROV_S2 };
    let (pd, pv) = (rd_u64(e + off)? as usize, rd_u64(e + off + 8)? as usize);
    if !ptr_ok(pd) || !ptr_ok(pv) { return None; }
    let base = super::dyn_eff::prov90_cooltime(pd, pv, e)?;
    let t = (rd_i32(e + E_400)? as i64).wrapping_add(100).max(1) as u64;
    Some((base.wrapping_mul(100) / t).max(if k == 1 { 3 } else { 1 }))
}
/// `Effect::expected_heal_target(slot, ctx, e, desc, e, desc)` = umin(usub_sat(maxhp−hp), slot.vt+0x40(inline, ctx, e, desc)) (g06.ll:52315)
unsafe fn heal_target(slot: usize, ctx: usize, e: usize) -> Option<u64> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(d) || !ptr_ok(v) { return None; }
    let miss = rd_u64(e + ENT_MAXHP)?.saturating_sub(rd_u64(e + ENT_HP)?);
    // ★[2026-09-08] 합성 arm(0x13409d0 두-Vec 합 ×19.9만 / 0x12b2e90 스택 잎) 은 `dyn_eff::eff40_heal` 이 이미 갖고 있다
    //   (siege 첫 판 NA 19.9만 = 전부 FMheal). 그것부터 쓰고, 못 잡는 형만 slot_sum(specemu) 으로.
    let r = match super::dyn_eff::eff40_heal(d, v, e) { Some(x) => x as i64, None => {
        let old = super::buff_value::leaf_ctx(); super::buff_value::set_leaf_ctx(ctx);
        let r = super::buff_value::slot_sum(d, v, 0x40, e, 0, "FMheal");
        super::buff_value::set_leaf_ctx(old); r? } };
    Some(miss.min(r as u64))
}
/// `Effect::expected_shield_target` = slot.vt+0x48(inline, ctx, e, desc) (g06.ll:52849)
unsafe fn shield_target(slot: usize, ctx: usize, e: usize) -> Option<u64> {
    let (d, v) = (rd_u64(slot)? as usize, rd_u64(slot + 8)? as usize);
    if !ptr_ok(d) || !ptr_ok(v) { return None; }
    let r = match super::position_eval::eff48_shield(d, v, e) { Some(x) => x as i64, None => {
        let old = super::buff_value::leaf_ctx(); super::buff_value::set_leaf_ctx(ctx);
        let r = super::buff_value::slot_sum(d, v, 0x48, e, 0, "FMshld");
        super::buff_value::set_leaf_ctx(old); r? } };
    Some(r as u64)
}
/// `fight_check::self_sustain_in_window(version, ctx, e, window)` (m15.ll:30465): 창 안에 쓸 수 있는 슬롯 1·2·3 의 힐+실드 합
pub unsafe fn self_sustain_in_window(_version: u64, ctx: usize, e: usize, window: u64) -> Option<u64> {
    let mut acc = 0u64;
    for k in 1..=3u32 {
        let (slot, _) = slot_k(e, k)?;
        let Some(s) = slot else { continue };
        if rd_i32(s + 0x30)? == -1 { continue; }
        if !skill_in_window(e, k, window)? { continue; }
        acc = acc.wrapping_add(heal_target(s, ctx, e)?).wrapping_add(shield_target(s, ctx, e)?);
    }
    Some(acc)
}
/// `fight_check::available_cc_in_window(version, e, window)` (m15.ll:28877): 창 안 슬롯의 `vt+0x88`(Option<i64> cc 시간) 합
pub unsafe fn available_cc_in_window(_version: u64, e: usize, window: u64) -> Option<u64> {
    let mut acc = 0u64;
    for k in 1..=3u32 {
        let (slot, _) = slot_k(e, k)?;
        let Some(s) = slot else { continue };
        if rd_i32(s + 0x30)? == -1 { continue; }
        if !skill_in_window(e, k, window)? { continue; }
        let (d, v) = (rd_u64(s)? as usize, rd_u64(s + 8)? as usize);
        if !ptr_ok(d) || !ptr_ok(v) { return None; }
        let (ok, t) = super::combat_score::slot_88(d, v, 0)?;
        if ok { acc = acc.wrapping_add(t); }
    }
    Some(acc)
}
/// `fight_check::expected_dps(ctx, e, tgt)` (m15.ll:23520): Σ_slot0..2 expected_damage_target*1000 / cooltime
pub unsafe fn expected_dps(ctx: usize, e: usize, tgt: usize) -> Option<u64> {
    let _ = ctx;
    let mut v = 0u64;
    if rd_i32(e + SLOT0 + 0x30)? != -1 {
        let dmg = super::passive_jungle::estimate_damage(e + SLOT0, e, tgt, false)?;
        let ct = super::fight_check::atk_interval(e)?; if ct == 0 { return None; }
        v = v.wrapping_add(dmg.wrapping_mul(1000) / ct);
    }
    if rd_i32(e + SLOT1 + 0x30)? != -1 {
        let dmg = super::passive_jungle::estimate_damage(e + SLOT1, e, tgt, false)?;
        let ct = skill_cooltime(e, 1)?; if ct == 0 { return None; }
        v = v.wrapping_add(dmg.wrapping_mul(1000) / ct);
    }
    if let (Some(s2), _) = slot_k(e, 2)? {
        if rd_i32(s2 + 0x30)? != -1 {
            let dmg = super::passive_jungle::estimate_damage(s2, e, tgt, false)?;
            let ct = skill_cooltime(e, 2)?; if ct == 0 { return None; }
            v = v.wrapping_add(dmg.wrapping_mul(1000) / ct);
        }
    }
    Some(v)
}
/// `fight_check::fight_dps(version, ctx, e, tgt)` (m15.ll:35853): expected_dps + 궁(6초 창) + 효과 vt+0x80 Σ*1000/tps
pub unsafe fn fight_dps(_version: u64, ctx: usize, e: usize, tgt: usize) -> Option<u64> {
    let mut v = expected_dps(ctx, e, tgt)?;
    let tps = tps_of(ctx)?;
    let w6 = tps.wrapping_mul(6).max(1);
    // 궁: can_ult || !(kind==13 && cd_ult > 6·tps)
    if skill_in_window(e, 3, w6)? {
        if let (Some(s3), _) = slot_k(e, 3)? {
            if rd_i32(s3 + 0x30)? != -1 && super::combat_score::st_valid_target(rd_u32(s3 + 0x28), e, tgt)? {
                let dmg = super::passive_jungle::estimate_damage(s3, e, tgt, false)?;
                v = v.wrapping_add(dmg.wrapping_mul(1000) / w6);
            }
        }
    }
    // 효과 리스트 vt+0x80(eff, ctx, e, tgt) 원값 *1000 / max(tps,1)
    let n = rd_u64(e + ENT_EFFS_LEN)?;
    if n != 0 {
        let p = rd_u64(e + ENT_EFFS_PTR)? as usize; if !ptr_ok(p) { return None; }
        let t1 = tps.max(1);
        for i in 0..n.min(64) as usize {
            let (d, vt) = (rd_u64(p + i * 16)? as usize, rd_u64(p + i * 16 + 8)? as usize);
            let r = super::dyn_eff::impl_rva(vt, 0x80)?;
            let raw = super::fight_check::eff80_dps(r, d, e, tgt, tps)?;
            v = v.wrapping_add(raw.wrapping_mul(1000) / t1);
        }
    }
    Some(v)
}

// ── resolve_fight_uncached (m10.ll:43974, fight_model.rs L378~521) ────────────────────────
/// `data` = &OperationData(첫 16B = (game, vt) fat ptr) · `ctx` = G(+8 = cfg) · 슬라이스는 엔티티 포인터 배열.
#[allow(clippy::too_many_arguments)]
pub unsafe fn resolve_fight_uncached(version: u64, data: usize, ctx: usize, champ: usize,
                                     allies: &[usize], enemies: &[usize], committed_dir: i8, tower: Option<usize>,
                                     judge_accuracy: u64, arrivals: &[i64], baseline: i64) -> Option<FightPrediction> {
    // L380
    if allies.is_empty() || enemies.is_empty() {
        return Some(FightPrediction { focus: None, soaker: None, rescue: None, net: 0, line: LINE_HOLD, line_abs: LINE_HOLD });
    }
    let w = World { x: data, data: rd_u64(data)? as usize, vt: rd_u64(data + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let tps = tps_of(ctx)?;
    let champ_h = rd_u64(champ + ENT_HANDLE)?;
    // L392~403 jrng
    let seed = if version > 1 {
        let mut set_h = 0u64;
        for &a in allies.iter().take(5) { set_h ^= rd_u64(a + ENT_HANDLE)?.wrapping_mul(0x9E37_79B9_7F4A_7C15); }
        for &e in enemies.iter().take(5) { set_h ^= rd_u64(e + ENT_HANDLE)?.wrapping_mul(0x517C_C1B7_2722_0A95); }
        w.seed()? ^ set_h.rotate_left(17) ^ champ_h
    } else {
        let bucket = w.tick()? / (tps.wrapping_mul(2)).max(1);
        champ_h ^ (bucket << 40)
    };
    let mut jrng = NoiseRng(seed);
    let mut misjudge = |v: i64| -> i64 { if judge_accuracy > 999 { v } else { error_ratio_noise(&mut jrng, judge_accuracy).wrapping_mul(v) / 100 } };
    // L415~
    let h = tps.wrapping_mul(6) as i64;                       // horizon_ticks
    let (mut our_hp, mut our_dps, mut our_alive, mut our_arrive) = ([0i64; 5], [0i64; 5], [false; 5], [0i64; 5]);
    let (mut our_n, mut our_cc_sum) = (0usize, 0u64);
    for (ai, &a) in allies.iter().take(5).enumerate() {
        our_arrive[ai] = arrivals.get(ai).copied().unwrap_or(0);
        let dps = {
            let mut best: Option<(u64, usize)> = None;
            for &e in enemies.iter() { let dd = d2(a, e)?; if best.map_or(true, |(bd, _)| dd < bd) { best = Some((dd, e)); } }
            match best { Some((_, t)) => fight_dps(version, ctx, a, t)? as i64, None => 0 }
        };
        let ehp = rd_u64(a + ENT_HP)?.wrapping_add(self_sustain_in_window(version, ctx, a, h as u64)?) as i64;
        our_cc_sum = our_cc_sum.wrapping_add(available_cc_in_window(version, a, h as u64)?);
        our_hp[ai] = misjudge(ehp.wrapping_mul(1000));
        our_dps[ai] = misjudge(dps);
        our_alive[ai] = true; our_n += 1;
    }
    let (mut their_hp, mut their_dps, mut their_alive, mut their_id) = ([0i64; 5], [0i64; 5], [false; 5], [0u64; 5]);
    let (mut their_n, mut their_cc_sum) = (0usize, 0u64);
    for &e in enemies.iter().take(5) {
        let nearest_a = {
            let mut best: Option<(u64, usize)> = None;
            for &a in allies.iter() { let dd = d2(e, a)?; if best.map_or(true, |(bd, _)| dd < bd) { best = Some((dd, a)); } }
            best.map(|b| b.1).unwrap_or(champ)
        };
        let ehp = rd_u64(e + ENT_HP)?.wrapping_add(self_sustain_in_window(version, ctx, e, h as u64)?) as i64;
        their_cc_sum = their_cc_sum.wrapping_add(available_cc_in_window(version, e, h as u64)?);
        their_hp[their_n] = misjudge(ehp.wrapping_mul(1000));
        their_dps[their_n] = misjudge(fight_dps(version, ctx, e, nearest_a)? as i64);
        their_alive[their_n] = true; their_id[their_n] = rd_u64(e + ENT_HANDLE)?; their_n += 1;
    }
    // L437~443 cc 감쇠
    if h != 0 {
        let their_eff = h.wrapping_sub((h >> 1).min(our_cc_sum as i64)).max(1);
        let our_eff = h.wrapping_sub((h >> 1).min(their_cc_sum as i64)).max(1);
        for i in 0..their_n { their_dps[i] = their_dps[i].wrapping_mul(their_eff) / h; }
        for i in 0..our_n { our_dps[i] = our_dps[i].wrapping_mul(our_eff) / h; }
    }
    // L449~451 tower_dps (max_by_key = 동점 시 뒤쪽)
    let mut tower_dps = 0i64;
    if let Some(t) = tower {
        if our_n > 0 {
            let mut bi = 0usize; let mut bk = rd_u64(allies[0] + ENT_MAXHP)?;
            for i in 1..our_n { let k = rd_u64(allies[i] + ENT_MAXHP)?; if k >= bk { bk = k; bi = i; } }
            tower_dps = expected_dps(ctx, t, allies[bi])? as i64;
        }
    }
    // L454~455 soaker
    let pick_soaker = |alive: &[bool; 5]| -> Option<Option<usize>> {
        let mut best: Option<(u64, usize)> = None;
        for i in 0..our_n { if !alive[i] { continue; } let k = rd_u64(allies[i] + ENT_MAXHP)?; if best.map_or(true, |(bk, _)| k >= bk) { best = Some((k, i)); } }
        Some(best.map(|b| b.1))
    };
    let mut soaker = pick_soaker(&our_alive)?;
    let soaker_id = match soaker { Some(i) => Some(rd_u64(allies[i] + ENT_HANDLE)?), None => None };
    // L458~505 틱 루프
    let horizon = h;
    let mut t = 0i64; let (mut our_dead_v, mut their_dead_v) = (0i64, 0i64);
    let mut first_focus: Option<u64> = None;
    loop {
        let our_total: i64 = (0..our_n).filter(|&i| our_alive[i] && our_arrive[i] <= t).map(|i| our_dps[i]).fold(0i64, |a, b| a.wrapping_add(b));
        let their_total: i64 = (0..their_n).filter(|&i| their_alive[i]).map(|i| their_dps[i]).fold(0i64, |a, b| a.wrapping_add(b));
        // te = 살아있는 적 중 hp 최소(min_by_key: 동점 시 앞쪽)
        let mut te: Option<usize> = None;
        for i in 0..their_n { if their_alive[i] && te.map_or(true, |j| their_hp[i] < their_hp[j]) { te = Some(i); } }
        let Some(te) = te else { break };
        // ta = 살아있고 도착한 아군 중 hp 최소
        let mut ta: Option<usize> = None;
        for i in 0..our_n { if our_alive[i] && our_arrive[i] <= t && ta.map_or(true, |j| our_hp[i] < our_hp[j]) { ta = Some(i); } }
        let Some(ta) = ta else {
            // L471: 아직 안 온 아군의 최소 도착 시각으로 점프
            let mut next: Option<i64> = None;
            for i in 0..our_n { if our_alive[i] && our_arrive[i] > t && next.map_or(true, |n| our_arrive[i] < n) { next = Some(our_arrive[i]); } }
            match next { Some(n) if n < horizon => { t = n; continue; } _ => break }
        };
        if t >= horizon || (their_total | our_total) == 0 { break; }
        if first_focus.is_none() { first_focus = Some(their_id[te]); }
        let soak = soaker.filter(|&s| our_alive[s] && our_arrive[s] <= t);
        let ta_incoming = their_total.wrapping_add(if soak == Some(ta) { tower_dps } else { 0 });
        let t_te = if our_total > 0 { their_hp[te].wrapping_add(our_total).wrapping_sub(1) / our_total } else { BIG };
        let t_ta = if ta_incoming > 0 { our_hp[ta].wrapping_add(ta_incoming).wrapping_sub(1) / ta_incoming } else { BIG };
        let t_soak = match soak { Some(s) if tower_dps > 0 && s != ta => our_hp[s].wrapping_add(tower_dps.wrapping_sub(1)) / tower_dps, _ => BIG };
        let dt = horizon.wrapping_sub(t).min(t_soak.min(t_ta.min(t_te))).max(1);
        their_hp[te] = their_hp[te].wrapping_sub(dt.wrapping_mul(our_total));
        our_hp[ta] = our_hp[ta].wrapping_sub(dt.wrapping_mul(ta_incoming));
        if let Some(s) = soak { if s != ta { our_hp[s] = our_hp[s].wrapping_sub(dt.wrapping_mul(tower_dps)); } }
        t = t.wrapping_add(dt);
        if their_hp[te] < 1 { their_alive[te] = false; their_dead_v = their_dead_v.wrapping_add(their_dps[te]); }
        if our_hp[ta] < 1 {
            our_alive[ta] = false; our_dead_v = our_dead_v.wrapping_add(our_dps[ta]);
            if soaker == Some(ta) { soaker = pick_soaker(&our_alive)?; }
        }
        if let Some(s) = soak { if s != ta && our_hp[s] < 1 {
            our_alive[s] = false; our_dead_v = our_dead_v.wrapping_add(our_dps[s]);
            if soaker == Some(s) { soaker = pick_soaker(&our_alive)?; }
        } }
    }
    // L508~520
    let net = their_dead_v.wrapping_sub(our_dead_v.wrapping_add(baseline));
    let our_unit = if our_n > 0 { (0..our_n).fold(0i64, |a, i| a.wrapping_add(our_dps[i])) / our_n as i64 } else { 0 };
    let line = match committed_dir {
        1 => if net < 0i64.wrapping_sub(our_unit) { LINE_DISENGAGE } else if net > -1 { LINE_COMMIT } else { LINE_HOLD },
        -1 => if net > our_unit { LINE_COMMIT } else if net < 1 { LINE_DISENGAGE } else { LINE_HOLD },
        _ => if net > our_unit { LINE_COMMIT } else if net < 0i64.wrapping_sub(our_unit) { LINE_DISENGAGE } else { LINE_HOLD },
    };
    Some(FightPrediction { focus: first_focus, soaker: soaker_id, rescue: None, net, line, line_abs: line })
}

// ── resolve_fight_full (m10.ll:39915, L324~376): (seed, tick) 에포크 TLS 메모 ─────────────
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct RfKey { a: [u64; 8], b: [u64; 8], version: u64, champ: u64, tower: u64, acc: u64, arr: u64, baseline: i64, dir: i8, la: u8, lb: u8 }
thread_local! { static RF_MEMO: std::cell::RefCell<((u64, u64), std::collections::HashMap<RfKey, FightPrediction>)> = std::cell::RefCell::new(((0, 0), std::collections::HashMap::new())); }
pub fn rf_memo_reset() { RF_MEMO.with(|c| { let mut m = c.borrow_mut(); m.0 = (0, 0); m.1.clear(); }); }
/// L336: 도착 틱 8칸을 바이트로 압축 — `arr/15` 를 0..255 로 clamp, `arr < -14` 면 0. i 번째는 `<< (56 - 8i)`.
fn pack_arrivals(arrivals: &[i64]) -> u64 {
    let mut k = 0u64;
    for (i, &v) in arrivals.iter().take(8).enumerate() {
        let b = if v < -14 { 0u64 } else { ((v / 15) as u64).min(255) };
        k |= b << (56 - 8 * i);
    }
    k
}
#[allow(clippy::too_many_arguments)]
pub unsafe fn resolve_fight_full(version: u64, data: usize, champ: usize, allies: &[usize], enemies: &[usize], committed_dir: i8,
                                 tower: Option<usize>, judge_accuracy: u64, arrivals: &[i64], baseline: i64) -> Option<FightPrediction> {
    // `data` = &OperationData {cache: &AGWC(x), context: G, blackboard} (IR full: %37=[data]=x · %39=[data+8]=ctx → uncached(x, ctx))
    let x = rd_u64(data)? as usize; let ctx = rd_u64(data + 8)? as usize; if !ptr_ok(x) || !ptr_ok(ctx) { return None; }
    if allies.len() > 8 || enemies.len() > 8 {
        return resolve_fight_uncached(version, x, ctx, champ, allies, enemies, committed_dir, tower, judge_accuracy, arrivals, baseline);
    }
    let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let epoch = (w.seed()?, w.tick()?);
    let mut key = RfKey { a: [0; 8], b: [0; 8], version, champ: rd_u64(champ + ENT_HANDLE)?, tower: match tower { Some(t) => rd_u64(t + ENT_HANDLE)?, None => u64::MAX },
                          acc: judge_accuracy, arr: pack_arrivals(arrivals), baseline, dir: committed_dir, la: allies.len() as u8, lb: enemies.len() as u8 };
    for (i, &a) in allies.iter().enumerate() { key.a[i] = rd_u64(a + ENT_HANDLE)?; }
    for (i, &e) in enemies.iter().enumerate() { key.b[i] = rd_u64(e + ENT_HANDLE)?; }
    let hit = RF_MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != epoch { m.0 = epoch; m.1.clear(); } m.1.get(&key).copied() });
    if let Some(v) = hit { return Some(v); }
    let v = resolve_fight_uncached(version, x, ctx, champ, allies, enemies, committed_dir, tower, judge_accuracy, arrivals, baseline)?;
    RF_MEMO.with(|c| { let mut m = c.borrow_mut(); m.1.insert(key, v); });
    Some(v)
}

// ── tower_discipline::v47_siege_stance (0xd96d00, m07.ll:48393, L509~539) 순수 재현 ─────────
/// 반환 = 게임의 `Option<usize>` 페어 `(tag, 다이버 핸들)`. `data` = &OperationData · `player` = 자기 PlayerState(sim/rec).
///   G0 kind==2·0x6b9==1·0x6a0==0 → 캐시(SiegeStanceCache, (seed,tick)·(side,tower)) → G1 `tick > cfg[5112|5120|5128 by phase]` → 슬롯0 태그
///   → A 아군(kind13&&0x70==1 제외 · ≤250000²) 2명 미만이면 None → B 적(`Blackboard::is_recent_visible` · ≤250000²)
///   → C 적이 있으면 `resolve_fight(version, data, allies[0], allies, enemies, 0, Some(tower), judge_accuracy(player))` → `line==Disengage` 면 None
///   → D `hp ≥ 3·max(expected_damage_target(tower.slot0, tower, a), 1)` 인 아군 중 (maxhp, handle) 최대(max_by_key = 동점 시 뒤쪽).
pub unsafe fn siege_stance(version: u64, data: usize, player: usize, tower: usize) -> Option<(u64, u64)> {
    if !ptr_ok(data) || !ptr_ok(player) || !ptr_ok(tower) { return None; }
    if rd_u64(tower + E_KIND)? != 2 { return Some((0, 0)); }
    if !(rd_u8(tower + 0x6b9) == 1 && rd_u64(tower + 0x6a0)? == 0) { return Some((0, 0)); }
    let x = rd_u64(data)? as usize; let ctx = rd_u64(data + 8)? as usize; let bb = rd_u64(data + 16)? as usize;
    if !ptr_ok(x) || !ptr_ok(ctx) || !ptr_ok(bb) { return None; }
    let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize };
    if !ptr_ok(w.data) || !ptr_ok(w.vt) { return None; }
    let tick = w.tick()?;
    // ★[2026-09-08] SIEGE_STANCE_CACHE 미러(L513~516 / L531~532): TLS {seed, tick, HashMap<(side, tower.handle), Option<usize>>}.
    //   같은 (seed,tick) 안에서는 **첫 호출의 결과를 그대로 돌려준다** — presim 오버레이로 hp/시야가 바뀐 뒤의 재호출도
    //   첫 값이라 매번 새로 계산하면 DIFF(첫 판 diff 215 중 "game=None/mine=Some(22)" 25연속 패턴).
    let seed = w.seed()?;
    let ckey = (rd_u64(player + P5_SIDE)?, rd_u64(tower + ENT_HANDLE)?);
    let hit = SIEGE_CACHE.with(|c| { let mut m = c.borrow_mut(); if m.0 != (seed, tick) { m.0 = (seed, tick); m.1.clear(); } m.1.get(&ckey).copied() });
    if let Some(v) = hit { SIEGE_LAST.with(|c| c.set((0, 0, 8, 0))); return Some(v); }
    let r = siege_stance_uncached(version, data, player, tower, x, ctx, bb, &w, tick)?;
    SIEGE_CACHE.with(|c| { c.borrow_mut().1.insert(ckey, r); });
    Some(r)
}
thread_local! { static SIEGE_CACHE: std::cell::RefCell<((u64, u64), std::collections::HashMap<(u64, u64), (u64, u64)>)> = std::cell::RefCell::new(((0, 0), std::collections::HashMap::new())); }
#[allow(clippy::too_many_arguments)]
unsafe fn siege_stance_uncached(version: u64, data: usize, player: usize, tower: usize, x: usize, ctx: usize, bb: usize, w: &World, tick: u64) -> Option<(u64, u64)> {
    let w = *w;
    // G1: 국면별 타워 다이브 허용 마감 틱 (L542~548)
    let phase = rd_u8(ctx + G_PHASE);
    let off = match phase { 5 => 5128usize, 1 | 3 => 5120, _ => 5112 };
    let cfg = rd_u64(ctx + G_CFG)? as usize; if !ptr_ok(cfg) { return None; }
    if tick > rd_u64(cfg + off)? { return Some((0, 0)); }
    let side = rd_u64(player + P5_SIDE)?; if side > 1 { return None; }
    let opp = 1 - side;
    if rd_i32(tower + SLOT0 + 0x30)? == -1 { return Some((0, 0)); }
    const D2: u64 = 62_500_000_000;   // 250000² (< D2+1)
    let (tx, ty) = xy(tower)?;
    // A
    let mut allies: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
        if rd_u64(e + E_KIND)? == 13 && rd_u64(e + 0x70)? == 1 { continue; }
        let (ex, ey) = xy(e)?; if sqd(ex, ey, tx, ty) > D2 { continue; }
        allies.push(e);
    }
    if allies.len() < 2 { return Some((0, 0)); }
    // B: Blackboard::is_recent_visible(bb[opp], game, vt, player, e) && ≤250000²  (g07.ll:157005)
    let mut enemies: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (opp as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
        let h = rd_u64(e + ENT_HANDLE)?;
        let recent = if w.visible(side, h)? { true } else {
            let rec = w.roster_rec(h)?;
            if rec == 0 { false } else {
                let role = rd_u32(rec + P5_ROLE) as usize;
                rd_u64(bb + (opp as usize) * BB_STRIDE + 0x1e0 + role * 8)?.wrapping_add(120) >= tick
            }
        };
        if !recent { continue; }
        let (ex, ey) = xy(e)?; if sqd(ex, ey, tx, ty) > D2 { continue; }
        enemies.push(e);
    }
    // C
    if !enemies.is_empty() {
        let acc = judge_accuracy(player + 384)?;
        let pred = resolve_fight_full(version, data, allies[0], &allies, &enemies, 0, Some(tower), acc, &[], 0)?;
        SIEGE_LAST.with(|c| c.set((allies.len() as u64, enemies.len() as u64, pred.line as u64, pred.net)));
        if pred.line == LINE_DISENGAGE { return Some((0, 0)); }
    } else {
        SIEGE_LAST.with(|c| c.set((allies.len() as u64, 0, 9, 0)));
    }
    // D
    let (mut best, mut bkey) = (0usize, (0u64, 0u64));
    for &a in &allies {
        let shot = super::passive_jungle::estimate_damage(tower + SLOT0, tower, a, false)?.max(1);
        if rd_u64(a + ENT_HP)? < shot.wrapping_mul(3) { continue; }
        let key = (rd_u64(a + ENT_MAXHP)?, rd_u64(a + ENT_HANDLE)?);
        if best == 0 || key >= bkey { best = a; bkey = key; }
    }
    Some(if best == 0 { (0, 0) } else { (1, rd_u64(best + ENT_HANDLE)?) })
}
/// Blackboard 1팀 분 크기(`lanes+other*0x2e8` 와 동일 구조체)
const BB_STRIDE: usize = 0x2e8;
thread_local! { pub static SIEGE_LAST: std::cell::Cell<(u64, u64, u64, i64)> = const { std::cell::Cell::new((0, 0, 0, 0)) }; }

/// `AthleteParameter::judge_accuracy(p)` (g15.ll:125794): `min(p.152 * p.720 / 1000, 100) * 9 + 100`
pub unsafe fn judge_accuracy(param: usize) -> Option<u64> {
    let v = rd_u64(param + 152)?.wrapping_mul(rd_u64(param + 720)?) / 1000;
    Some(v.min(100).wrapping_mul(9).wrapping_add(100))
}
#[inline] pub fn na(tag: &str) -> Option<FightPrediction> { let _ = na_tag(tag); None }
