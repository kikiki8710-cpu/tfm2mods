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


// ══════════════════════════════════════════════════════════════════════════════════════════════
// ★[2026-09-08] `tower_dive_is_viable`(0xe07430, m10.ll:42838, fight_model.rs L935~1045) 순수 재현
//   + 그 콜리 `resolve_fight_stake`(m10.ll:42357, L571~600) · `ally_is_bound`(m10.ll:39193)
//   + `battle::max_range_cached`(m10.ll:50689 = MaxRangeCache 미러 + 이미 포팅된 `as_callees::max_reach`)
//   + `path_finder::is_enemy_well_danger`(m03.ll:144500) · `AbstractGameWithCache::iter_towers_without_nexus`(g15.ll:108971)
//   ⚠**version ≤ 1 경로는 미포팅(NA)** — 그 갈래에서만 `is_unreasonable_tower_dive_enemy` 서브트리
//     (can_tower_focused 458줄 · Game::adjust_position 552줄 · towers 466줄 …)가 필요한데,
//     0.5.8 리플레이는 version > 1 이라 죽은 코드다. NA 가 실제로 잡히면 그때 포팅한다.
// ══════════════════════════════════════════════════════════════════════════════════════════════

const ENT_SPEED: usize = 0x640;      // 이동속도(=1600)
const ENT_SZ_PCT: usize = 0x470;     // 크기 보정 %(i32)
const ENT_SZ: usize = 0x680;         // 기본 크기
const ENT_R438: usize = 0x438;       // 사거리 가산

/// `Entity::size` = `pct==0 ? sz : (pct+100)*sz/100` (max_range·타워 사거리에서 양쪽 크기를 더한다)
#[inline] unsafe fn esize(e: usize) -> Option<u64> {
    let p = rd_i32(e + ENT_SZ_PCT)? as i64; let r = rd_u64(e + ENT_SZ)?;
    Some(if p == 0 { r } else { ((p + 100) as u64).wrapping_mul(r) / 100 })
}
/// `Entity::distance`(g06.ll:84197) = `utils::distance`(g06.ll:87697) = **정수 √(dx²+dy²)**
#[inline] unsafe fn edist(a: usize, b: usize) -> Option<u64> {
    let (ax, ay) = xy(a)?; let (bx, by) = xy(b)?;
    Some(super::obj_helpers::isqrt(sqd(ax, ay, bx, by)))
}

// ── battle::max_range_cached — TLS `MaxRangeCache` 미러 ────────────────────────────────────────
//   키 = **로스터 평면 인덱스 쌍**(엔티티 포인터가 아니다) · 에포크 = (seed, tick) · 값 = `max_range(a,b)`.
//   둘 중 하나라도 로스터에 없으면(타워 등) **캐시를 안 탄다** = 매번 계산.
//   ⚠presim 오버레이로 같은 틱에 상태가 바뀌어도 게임은 **첫 값**을 돌려준다 — 미러 없이는 조용한 DIFF.
thread_local! {
    static MR_MEMO: std::cell::RefCell<((u64, u64), std::collections::HashMap<(u8, u8), u64>)> =
        std::cell::RefCell::new(((0, 0), std::collections::HashMap::new()));
}
pub fn mr_memo_reset() { MR_MEMO.with(|c| { let mut m = c.borrow_mut(); m.0 = (0, 0); m.1.clear(); }); }
/// `x+0x1e0` 의 10칸(팀0 5 + 팀1 5)에서 핸들 위치를 찾는다. 없으면 `Some(None)`.
unsafe fn roster_flat_idx(x: usize, h: u64) -> Option<Option<u8>> {
    for i in 0..10usize {
        let p = rd_u64(x + X_ROSTER + i * 8)? as usize;
        if p == 0 { continue; }
        if rd_u64(p + ENT_HANDLE)? == h { return Some(Some(i as u8)); }
    }
    Some(None)
}
pub unsafe fn max_range_cached(data: usize, a: usize, b: usize) -> Option<u64> {
    if !ptr_ok(a) || !ptr_ok(b) { return None; }
    let x = rd_u64(data)? as usize; if !ptr_ok(x) { return None; }
    let ia = roster_flat_idx(x, rd_u64(a + ENT_HANDLE)?)?;
    let ib = roster_flat_idx(x, rd_u64(b + ENT_HANDLE)?)?;
    let (ia, ib) = match (ia, ib) { (Some(p), Some(q)) => (p, q), _ => return super::as_callees::max_reach(a, b) };
    let (wd, wv) = (rd_u64(x)? as usize, rd_u64(x + 8)? as usize);
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let epoch = (crate::judge::world::game_seed(wd, wv)?, crate::judge::world::game_tick(wd, wv)?);
    let hit = MR_MEMO.with(|c| { let mut m = c.borrow_mut(); if m.0 != epoch { m.0 = epoch; m.1.clear(); } m.1.get(&(ia, ib)).copied() });
    if let Some(v) = hit { return Some(v); }
    let v = super::as_callees::max_reach(a, b)?;
    MR_MEMO.with(|c| { c.borrow_mut().1.insert((ia, ib), v); });
    Some(v)
}

/// `path_finder::is_enemy_well_danger`(m03.ll:144500) — 적 우물 근처인가(팀별 코너 두 사각형).
unsafe fn is_enemy_well_danger(player: usize, ex: u64, ey: u64) -> Option<bool> {
    let side = rd_u64(player + P5_SIDE)?;
    Some(if side == 1 {
        if ex <= 64_000 && ey.wrapping_sub(800_000) <= 160_000 { true }
        else { ex <= 160_000 && ey.wrapping_sub(896_000) <= 64_000 }
    } else if ex.wrapping_sub(800_000) <= 160_000 && ey <= 64_000 { true }
    else { ex.wrapping_sub(896_000) <= 64_000 && ey <= 160_000 })
}

/// `Blackboard::is_recent_visible(bb[1-side], game, vt, player, e)`(g07.ll:157005) — siege B 와 같은 식.
unsafe fn recent_visible(w: &World, bb: usize, side: u64, tick: u64, e: usize) -> Option<bool> {
    let h = rd_u64(e + ENT_HANDLE)?;
    if w.visible(side, h)? { return Some(true); }
    let rec = w.roster_rec(h)?;
    if rec == 0 { return Some(false); }
    let role = rd_u32(rec + P5_ROLE) as usize; if role >= 5 { return None; }
    let opp = 1 - side;
    Some(rd_u64(bb + (opp as usize) * BB_STRIDE + 0x1e0 + role * 8)?.wrapping_add(120) >= tick)
}

/// `check_kill_die_tick`(0xeb82d0) 호출 — 게임의 bumpalo `Vec<&Entity>`{ptr@0, bump@8, cap@0x10, len@0x18} 를
/// 스택에 흉내 내 이미 포팅된 `fight_check::fight_check_memo`(DieTickCache 미러 포함)에 넘긴다.
unsafe fn ckdt(version: u64, data: usize, player: usize, me: usize, a: &[usize], b: &[usize]) -> Option<u64> {
    let ba: Vec<u64> = a.iter().map(|&e| e as u64).collect();
    let bb: Vec<u64> = b.iter().map(|&e| e as u64).collect();
    let ha: [u64; 4] = [ba.as_ptr() as u64, 0, ba.len() as u64, ba.len() as u64];
    let hb: [u64; 4] = [bb.as_ptr() as u64, 0, bb.len() as u64, bb.len() as u64];
    super::fight_check::fight_check_memo(version, data, player, me, ha.as_ptr() as usize, hb.as_ptr() as usize)
}

/// `fight_model::ally_is_bound`(m10.ll:39193) — 이 아군이 "적에게 묶여 있는가".
///   근접적 = `dist²(e, ally) ≤ (max_range_cached(data, e, ally) + 30000)²` 인 적. 없으면 false.
///   lim = 근접적들의 `usub_sat(range+30000, dist)/max(ally.speed,1)` 최소값. 반환 = `die_tick(ally) ≤ lim`.
thread_local! { pub static BOUND_LAST: std::cell::Cell<(u64, u64, u64)> = const { std::cell::Cell::new((0, 0, 0)) }; }   // (near, lim, die)
unsafe fn ally_is_bound(version: u64, data: usize, player: usize, ally: usize, enemies: &[usize]) -> Option<bool> {
    let mut near: Vec<usize> = Vec::with_capacity(enemies.len());
    for &e in enemies {
        let r = max_range_cached(data, e, ally)?.wrapping_add(30_000);
        if d2(e, ally)? <= r.wrapping_mul(r) { near.push(e); }
    }
    if near.is_empty() { return Some(false); }
    let sp = rd_u64(ally + ENT_SPEED)?.max(1);
    // ★★2026-09-10 정정: 여기는 **max** 다(원본 `.map(..).max()`). ~~min~~ 이었고 그게 tower_dive
    //   잔차 0.49%(125/25,673)의 단일 원인이었다. 근거 3중:
    //     ① IR: DISubprogram `max<Map<Iter<&Entity>, ally_is_bound::closure_env$1>>` (iterator.rs:3250 → max_by)
    //     ② exe `0x140e04e68: CMP RBX,RAX ; CMOVBE RBX,RAX` = if(acc <= new) acc = new  ⟹ max
    //     ③ 디컴 `if (uVar7 <= uVar9) { uVar7 = uVar9; }`
    //   원본 실명 = game_ai::plan_legacy::old::fight_model::ally_is_bound @ RVA 0xe04c60 (fight_model.rs:531~551).
    //   ⚠나머지(near 필터·+30000·제곱비교·speed +0x640·정수내림·포화뺄셈·`die <= lim` 방향·2번째 리스트 빈배열)는
    //     전부 맞았다 — 부호가 뒤집혔을 거라 의심했지만 `SETBE` 로 `die <= lim` 이 확정됐다.
    let mut lim = 0u64;                       // near 는 비어있지 않음이 보장 + 모든 항 ≥ 0 이라 0 시작이 동치
    for &e in &near {
        let r = max_range_cached(data, e, ally)?.wrapping_add(30_000);
        let t = r.saturating_sub(edist(e, ally)?) / sp;
        if t > lim { lim = t; }
    }
    let die = ckdt(version, data, player, ally, &near, &[])?;
    BOUND_LAST.with(|c| c.set((near.len() as u64, lim, die)));
    Some(die <= lim)
}

/// `fight_model::resolve_fight_stake`(m10.ll:42357, L571~600).
///   version<2 → `resolve_fight_full` 그대로. 아니면 ①`bound` 아군(챔프 제외)만 골라 ②전체 예측(base)
///   ③bound 만의 예측 p1(dir=0) ④전체 + `baseline = p1.net` 예측 p2 → `p2.line_absolute = base.line`.
///   `p2.line != base.line` 이면 p2.rescue 를 **bound 중 (거리², 핸들) 최소** 아군으로 교체.
#[allow(clippy::too_many_arguments)]
thread_local! { pub static STAKE_LAST: std::cell::Cell<(u64, u64, u64, u64, i64, u64, u64, i64, u64)> = const { std::cell::Cell::new((0, 0, 0, 0, 0, 0, 0, 0, 0)) }; }   // (al, bd, base_ln, p2_ln, p1net, resc, en, base_net, base_focus)   // (allies, bound, base.line, p2.line, p1.net, rescue)
pub unsafe fn resolve_fight_stake(version: u64, data: usize, player: usize, champ: usize,
                                  allies: &[usize], enemies: &[usize], committed_dir: i8,
                                  tower: Option<usize>, judge_accuracy: u64) -> Option<FightPrediction> {
    if version < 2 {
        return resolve_fight_full(version, data, champ, allies, enemies, committed_dir, tower, judge_accuracy, &[], 0);
    }
    let ch = rd_u64(champ + ENT_HANDLE)?;
    let mut bound: Vec<usize> = Vec::with_capacity(allies.len());
    for &e in allies {
        if rd_u64(e + ENT_HANDLE)? == ch { continue; }
        if ally_is_bound(version, data, player, e, enemies)? { bound.push(e); }
    }
    let base = resolve_fight_full(version, data, champ, allies, enemies, committed_dir, tower, judge_accuracy, &[], 0)?;
    if bound.is_empty() {
        STAKE_LAST.with(|c| c.set((allies.len() as u64, 0, base.line as u64, base.line as u64, 0, base.rescue.unwrap_or(u64::MAX), enemies.len() as u64, base.net, base.focus.unwrap_or(u64::MAX))));
        return Some(base);
    }
    let p1 = resolve_fight_full(version, data, champ, &bound, enemies, 0, tower, judge_accuracy, &[], 0)?;
    let mut p2 = resolve_fight_full(version, data, champ, allies, enemies, committed_dir, tower, judge_accuracy, &[], p1.net)?;
    p2.line_abs = base.line;
    if p2.line != base.line {
        let mut best: Option<((u64, u64), u64)> = None;
        for &e in &bound {
            let k = (d2(e, champ)?, rd_u64(e + ENT_HANDLE)?);
            if best.map_or(true, |b| k < b.0) { best = Some((k, k.1)); }
        }
        p2.rescue = best.map(|b| b.1);
    }
    STAKE_LAST.with(|c| c.set((allies.len() as u64, bound.len() as u64, base.line as u64, p2.line as u64, p1.net, p2.rescue.unwrap_or(u64::MAX), enemies.len() as u64, base.net, base.focus.unwrap_or(u64::MAX))));
    Some(p2)
}

/// `AbstractGameWithCache::iter_towers_without_nexus(x, side)`(g15.ll:108971) → 대상과 가장 가까운 타워
/// (`min_by_key(dist²)`, 동점은 앞쪽) 중 **대상을 때릴 수 있는** 것만. 고정 6칸 순서 = 384·416·448·400·432·464(+8·side),
/// 이어서 `x+304+32·side` 의 Vec(ptr@0, len@24). 사거리식엔 **+15000** 이 붙는다(L971).
unsafe fn pick_tower(x: usize, side: u64, target: usize) -> Option<Option<usize>> {
    let s = side as usize;
    let mut list: Vec<usize> = Vec::with_capacity(16);
    for off in [384usize, 416, 448, 400, 432, 464] {
        let p = rd_u64(x + off + s * 8)? as usize;
        if p != 0 { if !ptr_ok(p) { return None; } list.push(p); }
    }
    let vb = x + 304 + s * 32;
    let n = rd_u64(vb + 24)?; if n > 256 { return None; }
    if n != 0 {
        let p = rd_u64(vb)? as usize; if !ptr_ok(p) { return None; }
        for i in 0..n as usize { let e = rd_u64(p + i * 8)? as usize; if e != 0 { list.push(e); } }
    }
    let mut best: Option<(u64, usize)> = None;
    for &t in &list { let k = d2(t, target)?; if best.map_or(true, |b| k < b.0) { best = Some((k, t)); } }
    let t = match best { Some((_, t)) => t, None => return Some(None) };
    if rd_i32(t + SLOT0 + 0x30)? == -1 { return na_opt("TDunw"); }   // 게임은 unwrap 패닉 — 실측되면 조사
    let range = rd_u64(t + SLOT0 + 0x10)?
        .wrapping_add(15_000)
        .wrapping_add(rd_u64(t + ENT_R438)?)
        .wrapping_add(rd_u64(t + E_LV)?.wrapping_sub(1).wrapping_mul(rd_u64(t + SLOT0 + 0x18)?))
        .wrapping_add(esize(target)?)
        .wrapping_add(esize(t)?);
    Some(if edist(t, target)? > range { None } else { Some(t) })
}
#[inline] unsafe fn na_opt(tag: &str) -> Option<Option<usize>> { let _ = na_tag(tag); None }
#[inline] unsafe fn na_b(tag: &str) -> Option<bool> { let _ = na_tag(tag); None }

/// L951 의 적 필터(closure #0, m10.ll:56215).
///   ①`dist²(e, champ) > (max(max_range(champ,e), max_range(e,champ)) + 30000)²` 면 제외
///   ②최근 시야에 없으면 제외 ③version>1: **적 챔프이면서 적 우물 위험**이면 제외.
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn tdiv_enemy_ok(version: u64, w: &World, data: usize, bb: usize, tick: u64,
                        player: usize, champ: usize, e: usize) -> Option<bool> {
    let r = max_range_cached(data, champ, e)?.max(max_range_cached(data, e, champ)?).wrapping_add(30_000);
    if d2(e, champ)? > r.wrapping_mul(r) { return Some(false); }
    let side = rd_u64(player + P5_SIDE)?; if side > 1 { return None; }
    if !recent_visible(w, bb, side, tick, e)? { return Some(false); }
    if version <= 1 { return na_b("TDv1e"); }        // is_unreasonable_tower_dive_enemy 갈래 미포팅
    let is_enemy_champ = rd_u64(e)? == 0 && rd_u64(e + 8)? == 1 - side;
    let bad = if is_enemy_champ { is_enemy_well_danger(player, rd_u64(e + ENT_X)?, rd_u64(e + ENT_Y)?)? } else { false };
    Some(!bad)
}

/// L1019 의 적 필터(closure s7_0, m10.ll:56457): 대상 자신이거나, `dist² ≤ 150000²` 이면서 최근 시야.
unsafe fn tdiv_s7_ok(w: &World, bb: usize, side: u64, tick: u64, target: usize, e: usize) -> Option<bool> {
    if rd_u64(e + ENT_HANDLE)? == rd_u64(target + ENT_HANDLE)? { return Some(true); }
    if d2(e, target)? >= 22_500_000_001 { return Some(false); }     // < 150000²+1
    recent_visible(w, bb, side, tick, e)
}

/// ★`fight_model::tower_dive_is_viable`(0xe07430) — 반환 = `line != Disengage`.
///   인자(래퍼 기준) `p1`=version · `p2`=rnd · **`p3`=player** · **`p4`=data** · `p5`=team_plan · `p6`=target · `p7`=team_model · `p8`=debug
#[allow(clippy::too_many_arguments)]
pub unsafe fn tower_dive_is_viable(version: u64, player: usize, data: usize, team_plan: usize,
                                   target: usize, team_model: bool) -> Option<bool> {
    if !ptr_ok(player) || !ptr_ok(data) || !ptr_ok(team_plan) || !ptr_ok(target) { return None; }
    let side = rd_u64(player + P5_SIDE)?; if side > 1 { return None; }
    let role = rd_u32(player + P5_ROLE) as usize; if role >= 5 { return None; }
    let x = rd_u64(data)? as usize; if !ptr_ok(x) { return None; }
    let champ = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + role * 8)? as usize;
    if champ == 0 { return Some(false); }                       // L945
    if !ptr_ok(champ) { return None; }
    let ctx = rd_u64(data + 8)? as usize; if !ptr_ok(ctx) { return None; }
    let tps = tps_of(ctx)?;                                     // L948
    let bb = rd_u64(data + 16)? as usize; if !ptr_ok(bb) { return None; }
    let (wd, wv) = (rd_u64(x)? as usize, rd_u64(x + 8)? as usize);
    if !ptr_ok(wd) || !ptr_ok(wv) { return None; }
    let w = World { x, data: wd, vt: wv };
    let tick = w.tick()?;
    let opp = 1 - side;

    // L951 — 챔프 주변의 적
    let mut near_enemies: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (opp as usize) * 0x28 + i * 8)? as usize;
        if e == 0 { continue; } if !ptr_ok(e) { return None; }
        if tdiv_enemy_ok(version, &w, data, bb, tick, player, champ, e)? { near_enemies.push(e); }
    }
    // L959 — team_plan[i] 태그 0 이고 챔프에서 120000 안인 아군
    let mut near_allies: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        if rd_u64(team_plan + i * 16)? != 0 { continue; }
        let e = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + i * 8)? as usize;
        if e == 0 { continue; } if !ptr_ok(e) { return None; }
        if d2(e, champ)? >= 14_400_000_001 { continue; }        // < 120000²+1
        near_allies.push(e);
    }
    // L969~972 — 대상에 가장 가까운(그리고 대상을 때릴 수 있는) 상대 팀 타워
    let tower = pick_tower(x, opp, target)?;
    // L976/978 — 내가 죽는 틱 / 대상이 죽는 틱
    let tv: Vec<usize> = tower.into_iter().collect();
    let my_die = ckdt(version, data, player, champ, &near_enemies, &tv)?;
    let tgt_die = ckdt(version, data, player, target, &near_allies, &[])?;
    if version <= 1 { return na_b("TDv1"); }                    // L989 레거시 갈래 미포팅

    // L990~993 — 대상이 자기 진영으로 도망칠 여유보다 늦게 죽으면 불가
    let (hx, hy) = if side == 1 { (0u64, 960_000u64) } else { (960_000u64, 0u64) };
    let d = super::obj_helpers::dist(rd_u64(target + ENT_X)?, rd_u64(target + ENT_Y)?, hx, hy);
    let sp = rd_u64(target + ENT_SPEED)?.max(1);
    if tgt_die > d.saturating_sub(160_000) / sp { return Some(false); }

    // L1000~1003 — 타워 사거리에서 빠져나오는 데 걸리는 틱(여기 사거리식엔 +15000 이 없다)
    let extra = match tower {
        None => 0u64,
        Some(t) => {
            if rd_i32(t + SLOT0 + 0x30)? == -1 { 0 } else {
                let range = rd_u64(t + ENT_R438)?
                    .wrapping_add(rd_u64(t + SLOT0 + 0x10)?)
                    .wrapping_add(rd_u64(t + E_LV)?.wrapping_sub(1).wrapping_mul(rd_u64(t + SLOT0 + 0x18)?))
                    .wrapping_add(esize(t)?)
                    .wrapping_add(esize(champ)?);
                range.saturating_sub(edist(t, target)?) / rd_u64(champ + ENT_SPEED)?.max(1)
            }
        }
    };
    // L1008/1015
    let viable = tgt_die.saturating_add(extra.wrapping_add(tps >> 1)) < my_die;
    if !team_model || viable { return Some(viable); }

    // L1016~1042 — 팀 모델: stake 예측의 line 으로 판단
    let mut l1: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + i * 8)? as usize;
        if e == 0 { continue; }
        if d2(e, target)? >= 40_000_000_001 { continue; }       // < 200000²+1
        l1.push(e);
    }
    let mut l2: Vec<usize> = Vec::with_capacity(5);
    for i in 0..5usize {
        let e = rd_u64(x + X_ROSTER + (opp as usize) * 0x28 + i * 8)? as usize;
        if e == 0 { continue; }
        if tdiv_s7_ok(&w, bb, side, tick, target, e)? { l2.push(e); }
    }
    let acc = judge_accuracy(player + 384)?;
    let st = resolve_fight_stake(version, data, player, champ, &l1, &l2, 0, tower, acc)?;
    let mut line = st.line;                                     // L1028
    if let Some(h) = st.rescue {                                // L1029~1035
        match w.entity(h) {
            None => line = st.line_abs,
            Some(e2) => {
                let r = max_range_cached(data, target, e2.0)?.max(max_range_cached(data, e2.0, target)?)
                        .wrapping_add(30_000);
                if d2(target, e2.0)? > r.wrapping_mul(r) { line = st.line_abs; }
            }
        }
    }
    Some(line != LINE_DISENGAGE)                                // L1042
}

/// 오라클: 게임 반환(bool) ↔ 재현. `p7` 하위 1비트 = team_model.
pub unsafe fn tower_dive_cmp(p1: u64, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize) -> Option<i64> {
    tower_dive_is_viable(p1, p3, p4, p5, p6, p7 & 1 != 0).map(|b| b as i64)
}

/// DIFF 진단: tower_dive 의 판단 재료를 한 줄로.
pub unsafe fn td_diag(version: u64, player: usize, data: usize, team_plan: usize, target: usize, p7: usize) -> String {
    let f = || -> Option<String> {
        let side = rd_u64(player + P5_SIDE)?; let role = rd_u32(player + P5_ROLE) as usize;
        let x = rd_u64(data)? as usize;
        let champ = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + role * 8)? as usize;
        if champ == 0 { return Some(format!("side={} role={} champ=NULL", side, role)); }
        let ctx = rd_u64(data + 8)? as usize; let tps = tps_of(ctx)?;
        let bb = rd_u64(data + 16)? as usize;
        let w = World { x, data: rd_u64(x)? as usize, vt: rd_u64(x + 8)? as usize };
        let tick = w.tick()?; let opp = 1 - side;
        let mut ne = 0u32; let mut na_e = 0u32;
        for i in 0..5usize {
            let e = rd_u64(x + X_ROSTER + (opp as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            match tdiv_enemy_ok(version, &w, data, bb, tick, player, champ, e) { Some(true) => ne += 1, Some(false) => {}, None => na_e += 1 }
        }
        let mut nl = 0u32;
        for i in 0..5usize {
            if rd_u64(team_plan + i * 16)? != 0 { continue; }
            let e = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + i * 8)? as usize; if e == 0 { continue; }
            if d2(e, champ)? >= 14_400_000_001 { continue; }
            nl += 1;
        }
        let tw = pick_tower(x, opp, target);
        let (hx, hy) = if side == 1 { (0u64, 960_000u64) } else { (960_000u64, 0u64) };
        let d = super::obj_helpers::dist(rd_u64(target + ENT_X)?, rd_u64(target + ENT_Y)?, hx, hy);
        let sp = rd_u64(target + ENT_SPEED)?.max(1);
        // stake 경로면 그 내부까지
        let st = if p7 & 1 == 1 { tower_dive_is_viable(version, player, data, team_plan, target, true).and_then(|_| Some(STAKE_LAST.with(|c| c.get()))) } else { None };
        // l1(아군 200000²) · l2(적 s7) 멤버와 판정 근거
        let th = rd_u64(target + ENT_HANDLE)?;
        let mut l1s = String::new(); let mut l2s = String::new();
        for i in 0..5usize {
            let e = rd_u64(x + X_ROSTER + (side as usize) * 0x28 + i * 8)? as usize;
            if e != 0 { let d = d2(e, target)?; l1s += &format!(" a{}:h{} d2={} {}", i, rd_u64(e + ENT_HANDLE)?, d, if d < 40_000_000_001 { "IN" } else { "out" }); }
            let o = rd_u64(x + X_ROSTER + (opp as usize) * 0x28 + i * 8)? as usize;
            if o != 0 {
                let oh = rd_u64(o + ENT_HANDLE)?; let d = d2(o, target)?;
                let vis = w.visible(side, oh).unwrap_or(false);
                let rec = recent_visible(&w, bb, side, tick, o).unwrap_or(false);
                l2s += &format!(" e{}:h{} d2={} self={} vis={} rec={} {}", i, oh, d, oh == th, vis, rec,
                                if tdiv_s7_ok(&w, bb, side, tick, target, o).unwrap_or(false) { "IN" } else { "out" });
            }
        }
        Some(format!("v={} side={} role={} tps={} ne={}(na{}) nl={} tower={:?} flee_lim={} tm={} tick={} stake(al,bd,base_ln,p2_ln,p1net,resc,en,base_net,focus)={:?} bound_last(near,lim,die)={:?} | L1[{}] | L2[{}] | GAME_RFF[{}]",
                     version, side, role, tps, ne, na_e, nl,
                     tw.map(|o| o.map(|t| rd_u64(t + ENT_HANDLE).unwrap_or(0))), d.saturating_sub(160_000) / sp, p7 & 1, tick, st, BOUND_LAST.with(|c| c.get()), l1s, l2s, rff_ring_fmt()))
    };
    f().unwrap_or_else(|| "diag NA".into())
}

/// 훅 어댑터 — 게임 `resolve_fight_full`(0xe05450) 인자를 그대로 받아 재현값을 만든다.
///   `(version, data, champ, allies_ptr, allies_len, enemies_ptr, enemies_len, dir, tower, acc, arr_ptr, arr_len, baseline)`
#[allow(clippy::too_many_arguments)]
pub unsafe fn rff_cmp(version: usize, data: usize, champ: usize, ap: usize, al: usize, ep: usize, el: usize,
                      dir: usize, tower: usize, acc: usize, arp: usize, arl: usize, base: usize) -> Option<FightPrediction> {
    if !ptr_ok(data) || !ptr_ok(champ) { return None; }
    if al > 8 || el > 8 || arl > 8 { return None; }          // 재현 키가 8칸이라 그 이상은 대조 안 함
    let mut allies: Vec<usize> = Vec::with_capacity(al);
    for i in 0..al { let e = rd_u64(ap + i * 8)? as usize; if !ptr_ok(e) { return None; } allies.push(e); }
    let mut enemies: Vec<usize> = Vec::with_capacity(el);
    for i in 0..el { let e = rd_u64(ep + i * 8)? as usize; if !ptr_ok(e) { return None; } enemies.push(e); }
    let mut arr: Vec<i64> = Vec::with_capacity(arl);
    for i in 0..arl { arr.push(rd_i64(arp + i * 8)?); }
    let tw = if tower == 0 { None } else { if !ptr_ok(tower) { return None; } Some(tower) };
    rff_record(champ, ap, al, ep, el, dir, tower, acc, base);
    resolve_fight_full(version as u64, data, champ, &allies, &enemies, dir as u8 as i8, tw, acc as u64, &arr, base as i64)
}

/// ★게임이 실제로 `resolve_fight_full` 에 넘긴 인자 링(최근 6건). tower_dive DIFF 때 내 l1/l2 와 대조한다.
///   (champ_h, dir, tower_h, acc, baseline, ally handles, enemy handles)
#[derive(Clone, Copy, Default)]
pub struct RffArgs { pub champ: u64, pub dir: i8, pub tower: u64, pub acc: u64, pub base: i64, pub a: [u64; 8], pub la: u8, pub b: [u64; 8], pub lb: u8 }
thread_local! { pub static RFF_RING: std::cell::RefCell<(Vec<RffArgs>, usize)> = const { std::cell::RefCell::new((Vec::new(), 0)) }; }
#[allow(clippy::too_many_arguments)]
pub unsafe fn rff_record(champ: usize, ap: usize, al: usize, ep: usize, el: usize, dir: usize, tower: usize, acc: usize, base: usize) {
    if al > 8 || el > 8 { return; }
    let mut r = RffArgs { champ: rd_u64(champ + ENT_HANDLE).unwrap_or(0), dir: dir as u8 as i8,
                          tower: if tower == 0 { u64::MAX } else { rd_u64(tower + ENT_HANDLE).unwrap_or(0) },
                          acc: acc as u64, base: base as i64, a: [0; 8], la: al as u8, b: [0; 8], lb: el as u8 };
    for i in 0..al { let e = rd_u64(ap + i * 8).unwrap_or(0) as usize; r.a[i] = rd_u64(e + ENT_HANDLE).unwrap_or(0); }
    for i in 0..el { let e = rd_u64(ep + i * 8).unwrap_or(0) as usize; r.b[i] = rd_u64(e + ENT_HANDLE).unwrap_or(0); }
    RFF_RING.with(|c| { let mut v = c.borrow_mut(); if v.0.len() < 6 { v.0.push(r); } else { let n = v.1 % 6; v.0[n] = r; } v.1 += 1; });
}
pub fn rff_ring_fmt() -> String {
    RFF_RING.with(|c| { let v = c.borrow();
        v.0.iter().map(|r| format!("(champ{} dir{} tw{} base{} A{:?} B{:?})", r.champ, r.dir,
            if r.tower == u64::MAX { -1i64 } else { r.tower as i64 }, r.base,
            &r.a[..r.la as usize], &r.b[..r.lb as usize])).collect::<Vec<_>>().join(" ")
    })
}
