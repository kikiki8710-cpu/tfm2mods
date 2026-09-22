#![allow(unused, dead_code, non_snake_case, invalid_reference_casting)]
#![feature(thread_local)]
//! 25차 배치F 오라클 — 199 EpicPokeSubPlan::action_candidates(pub) 진리표 + TLS 셀 상태 직독.
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(10 swordman) + 엔티티 raw write_volatile(함정 ⑦).
//!  predict = 명세 `logic` 독립 재구현(콜리는 pub 함수를 그대로 씀 · 합성만 검증) ↔ game 실행 결과 대조.
//!  대조 축: 원소 수 · 태그(+0xb1) · RunAway with_skill(+0x80) · Trace target(+0x60) · AroundRegion region(+8) · rnd 소비(다음 u64).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/F/oracle/o199.rs
//! 실행: o199.exe k=v ...   (드라이버 = run199.py)
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

extern "Rust" {
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai11fight_check14DIE_TICK_CACHE0s_023___RUST_STD_INTERNAL_VAL"]
    static DIE_TICK: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval14POS_EVAL_CACHE0s_023___RUST_STD_INTERNAL_VAL"]
    static POS_EVAL: [u8; 32];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai12action_score9INTER_CTX0s_023___RUST_STD_INTERNAL_VAL"]
    static INTER: [u8; 112];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai5utils13HP_VALUE_MEMO0s_023___RUST_STD_INTERNAL_VAL"]
    static HPV: [u8; 104];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus15LAST_STAND_MEMO0s_023___RUST_STD_INTERNAL_VAL"]
    static LAST_STAND: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai16tower_discipline18SIEGE_STANCE_CACHE0s_023___RUST_STD_INTERNAL_VAL"]
    static SIEGE: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle15MAX_RANGE_CACHE0s_023___RUST_STD_INTERNAL_VAL"]
    static MAXR: [u8; 1632];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle10CAST_BEAMSs_0s_023___RUST_STD_INTERNAL_VAL"]
    static BEAMS: [u8; 336];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai11fight_check14DIE_TICK_CACHE0023___RUST_STD_INTERNAL_VAL"]
    static DIE_TICK_B: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai13position_eval14POS_EVAL_CACHE0023___RUST_STD_INTERNAL_VAL"]
    static POS_EVAL_B: [u8; 32];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai12action_score9INTER_CTX0023___RUST_STD_INTERNAL_VAL"]
    static INTER_B: [u8; 112];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai5utils13HP_VALUE_MEMO0023___RUST_STD_INTERNAL_VAL"]
    static HPV_B: [u8; 104];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus15LAST_STAND_MEMO0023___RUST_STD_INTERNAL_VAL"]
    static LAST_STAND_B: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai16tower_discipline18SIEGE_STANCE_CACHE0023___RUST_STD_INTERNAL_VAL"]
    static SIEGE_B: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle15MAX_RANGE_CACHE0023___RUST_STD_INTERNAL_VAL"]
    static MAXR_B: [u8; 1632];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battle10CAST_BEAMSs_0023___RUST_STD_INTERNAL_VAL"]
    static BEAMS_B: [u8; 336];
}
/// state 바이트(closure#0 판 / closure#1 판) — state 0 = 미초기화(lazy) · 1 = 초기화됨 → 그 프로세스에서 그 TLS 키를 한 번이라도 with() 한 것
fn tls_state() -> String {
    unsafe { format!("DIE_TICK={}/{} POS_EVAL={}/{} INTER={}/{} HPV={}/{} LAST_STAND={}/{} SIEGE={}/{} MAXR={}/{} BEAMS={}/{}", DIE_TICK[88], DIE_TICK_B[88], POS_EVAL[24], POS_EVAL_B[24], INTER[104], INTER_B[104], HPV[96], HPV_B[96], LAST_STAND[88], LAST_STAND_B[88], SIEGE[88], SIEGE_B[88], MAXR[1624], MAXR_B[1624], BEAMS[328], BEAMS_B[328]) }
}

fn mkeff(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}
fn set_attack(e: *const Entity, dmg: usize, range: u64) {
    unsafe { std::ptr::write(&mut (*(e as *mut Entity)).attack_effect, Some(mkeff(dmg, range))); }
}

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } }

fn dist2(a: &Entity, b: &Entity) -> u64 {
    let dx = if a.x < b.x { b.x - a.x } else { a.x - b.x };
    let dy = if a.y < b.y { b.y - a.y } else { a.y - b.y };
    dx * dx + dy * dy
}
fn same_team(a: &Entity, b: &Entity) -> bool {
    let ta: i64 = rd(ep(a), 0); let tb: i64 = rd(ep(b), 0);
    ta == tb && (ta == 1 || rd::<usize>(ep(a), 8) == rd::<usize>(ep(b), 8))
}
fn visible_to(e: &Entity, viewer: &Entity) -> bool {
    let t: i64 = rd(ep(viewer), 0);
    if t == 1 { return true; }
    let idx: usize = rd(ep(viewer), 8);
    rd::<i64>(ep(e), 0x38 + 24 * idx) == 0
}
fn radius_eff(e: &Entity) -> u64 {
    let mult: i32 = rd(ep(e), 0x470); let r: u64 = rd(ep(e), 0x680);
    if mult == 0 { r } else { r * (100 + mult as u64) / 100 }
}
fn tag(a: &game_ai::SmallActionPlay) -> u8 { rd(a as *const _ as *const u8, 0xb1) }

#[derive(Debug, Clone)]
struct PA { tag: u8, with_skill: i8, target: i64, note: &'static str }
fn pa_of(a: &game_ai::SmallActionPlay, note: &'static str) -> PA {
    let p = a as *const _ as *const u8; let t = tag(a);
    let ws: i8 = if t == 3 { rd::<u8>(p, 0x80) as i8 } else { -1 };
    let target: i64 = match t { 14 => rd::<usize>(p, 0x60) as i64, 5 | 7 => rd::<usize>(p, 8) as i64, 15..=18 => rd::<usize>(p, 8) as i64, _ => -1 };
    PA { tag: t, with_skill: ws, target, note }
}

/// ★명세 logic 독립 재구현(predict). 콜리는 pub 함수 그대로. 반환 = (원소 요약 목록, 경로 표시)
fn predict<'a>(pool: &'a bumpalo::Bump, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData,
               param: &game_ai::ScoreParameter, team_plan: &game_ai::plan_legacy::team_plan::TeamPlan, dbg: &mut DebugFrameData,
               log: &mut String) -> Vec<PA> {
    use game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as SP;
    use game_ai::SmallActionPlay as A;
    let g: &dyn AbstractGame = data.cache.game;
    let mut sp: SP = Default::default();
    // L15
    if let Some(a) = team_plan.v27_objective_discipline_action(version, rnd, player, data, JungleType::Morgard) {
        *log += "L15 "; return vec![pa_of(&a, "v27")];
    }
    // L19
    let old = sp.action_candidates_old(version, rnd, player, data, param);
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = data.cache.player_champion[team][pos].unwrap();
    *log += &format!("old=[{}] ", old.iter().map(|a| tag(a).to_string()).collect::<Vec<_>>().join(","));
    // L23
    let mut best_t: Option<&Entity> = None; let mut best_d = u64::MAX;
    for t in data.cache.iter_towers_without_nexus(1 - team) {
        if t.can_target && t.block_target_tick == 0 { let d = dist2(t, champ); if d < best_d { best_d = d; best_t = Some(t); } }
    }
    let tower = best_t;
    // L28 closure#2 (Attack 가지만 완전 재현 · Skill 계열은 대상 존재+사거리+타워 규칙까지)
    let mut act: Vec<A> = Vec::new();
    for a in old.iter() {
        let t = tag(a);
        if !(15..=18).contains(&t) { continue; }
        let tid: usize = rd(a as *const _ as *const u8, 8);
        let te = match g.get_entity_by_id(tid) { Some(e) => e, None => continue };
        let tety: i64 = rd(ep(te), 0x68);
        if tety != 13 { act.push(a.clone()); continue; }
        let eff: Option<&Effect> = match t { 15 => champ.attack_effect.as_ref(), 16 => champ.skill_effect.as_ref(),
            17 => if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }, _ => if champ.level > 4 { champ.ult_effect.as_ref() } else { None } };
        let eff = eff.expect("effect unwrap (게임도 여기서 패닉)");
        if !eff.is_in_range(champ, te) { continue; }
        if t == 15 {
            let keep = match tower { None => true, Some(tw) => {
                let twa = tw.attack_effect.as_ref().unwrap();
                if twa.is_in_range(tw, champ) && rd::<i64>(ep(te), 0x68) != 2 { same_team(te, champ) } else { true } } };
            if keep { act.push(a.clone()); }
        } else {
            if let Some(tw) = tower { let twa = tw.attack_effect.as_ref().unwrap();
                if twa.is_in_range(tw, champ) && tety != 2 && !same_team(te, champ) { continue; } }
            *log += "SKILL-ARM(근사) "; act.push(a.clone());
        }
    }
    // L93 retain#4: Skill 계열·아군 대상만 — 이 세계엔 아군 대상 스킬이 없다(swordman default) → 통과
    // L199 retain#5
    act.retain(|a| { let s = sp.score(version, param, rnd, player, data, a, dbg); s >= -30 });
    // L204 get_move_action
    let ps_data = &param.positioning_score;
    let mut res: Vec<A> = Vec::new();
    let enemies: Vec<&Entity> = (0..5).filter_map(|i| data.cache.player_champion[1 - team][i]).collect();
    let has_nt = enemies.iter().any(|c| { let c: &Entity = *c;
        game_ai::nontarget_windup_perceived(version, player, data, c) && rd::<i64>(ep(c), 0x68) == 13 && {
            let st: i64 = rd(ep(c), 0x70);
            let eff = match st { 4 => c.skill_effect.as_ref(), 5 => if c.level > 2 { c.skill2_effect.as_ref() } else { None },
                                 6 => if c.level > 4 { c.ult_effect.as_ref() } else { None }, _ => return false };
            eff.unwrap().is_in_range(c, champ) }
    });
    let ps = game_ai::position_score_at_position(version, player, data, ps_data, champ.x, champ.y, game_ai::PositionEvalPurpose::Objective);
    let ps_p = &ps as *const _ as *const u8;
    let on_tr: bool = rd::<u8>(ps_p, 0x30) != 0; let on_ptr: bool = rd::<u8>(ps_p, 0x31) != 0;
    let tps = data.context.setting.tick_per_second;
    let mut path = String::new();
    if on_tr || has_nt || on_ptr {
        res.push(A::RunAway(game_ai::SmallActionRunAway::new_with_skill(data, player, 5, true))); path += "L299 ";
    } else {
        let acc = player.info.parameter.positioning_accuracy() as u64; let maxv = 2000 - acc;
        let near: Vec<&Entity> = data.cache.iter_champions(1 - team).filter(|c| visible_to(*c, champ) && dist2(*c, champ) < 160000u64 * 160000).collect();
        let mut nv = bumpalo::collections::Vec::new_in(pool); for c in near.iter() { nv.push(*c); }
        let me_die = game_ai::check_kill_die_tick(version, rnd, data, player, champ, nv, bumpalo::collections::Vec::new_in(pool), dbg);
        let mode = match g.get_game_mode() { GameMode::Moba(m) => m, _ => panic!("not moba") };
        let mp = mode as *const MobaMode as *const u8;
        let ll_ptr: *const usize = rd(mp, 0x1a0); let ll_len: usize = rd(mp, 0x1a8);
        let obj: Option<&Entity> = if ll_len > 0 { g.get_entity_by_id(unsafe { *ll_ptr }) } else { None };
        *log += &format!("me_die={} near={} obj={} ", me_die, near.len(), obj.map(|e| e.id as i64).unwrap_or(-1));
        let mut in_range = false;
        for e in near.iter() {
            let mut jrng = game_ai::range_misjudge_rng(version, data, player, e.id);
            let mr = game_ai::plan_legacy::old::max_range_can_use(champ, e) * game_ai::range_misjudge_roll(rnd, &mut jrng, acc, maxv) / 1000;
            let emr = game_ai::plan_legacy::old::max_range_can_use(e, champ) * game_ai::range_misjudge_roll(rnd, &mut jrng, acc, maxv) / 1000;
            let emr_near = game_ai::plan_legacy::old::max_range_nearly_can_use(e, champ, 40) * game_ai::range_misjudge_roll(rnd, &mut jrng, acc, maxv) / 1000;
            let d = dist2(champ, e);
            let near_obj = obj.map_or(true, |o| dist2(e, o) < 200000u64 * 200000 + 1);
            *log += &format!("[e{} mr={} emr={} emrn={} d={} ", e.id, mr, emr, emr_near, d);
            if me_die < tps {
                let emr2 = game_ai::plan_legacy::old::max_range_nearly_can_use(e, champ, 60) * game_ai::range_misjudge_roll(rnd, &mut jrng, acc, maxv) / 1000;
                if d > mr * mr && emr2 < mr { if near_obj { res.push(A::Trace(game_ai::SmallActionTrace::new(data, e.id, 5))); path += "L338 "; } }
                else if d <= emr2 * emr2 { in_range = true; path += "L341 "; }
            } else if e.remain_action_time() > 10 && mr >= 1 {
                if d > mr * mr && near_obj { res.push(A::Trace(game_ai::SmallActionTrace::new(data, e.id, 5))); path += "L343 "; }
            } else if emr < mr && d > mr * mr {
                if near_obj { res.push(A::Trace(game_ai::SmallActionTrace::new(data, e.id, 5))); path += "L355 "; }
            } else if d <= emr_near * emr_near { in_range = true; path += "L359 "; }
            *log += "] ";
        }
        for e in data.cache.others[1 - team].iter() {
            if let Some(atk) = e.attack_effect.as_ref() {
                let rng = (e.stat_buff_cached.range as u64) + (atk.range as u64) + (e.level as u64 - 1) * (atk.growth_range as u64) + (atk.range_adjust(e, champ) as u64) + radius_eff(e) + radius_eff(champ);
                if dist2(champ, e) <= rng * rng { in_range = true; path += "L364 "; break; }
            }
        }
        if res.is_empty() {
            let map = data.context.map; let mapp = map as *const MapDef as *const u8;
            let ry = std::cmp::min(champ.y / 32000, 29) as usize; let rx = std::cmp::min(champ.x / 32000, 29) as usize;
            let region: u64 = rd(mapp, 0x38b8 + (ry * 30 + rx) * 8);
            match obj {
                Some(o) if rd::<i64>(ep(o), 0x68) == 5 => {
                    if region == 7 {
                        if visible_to(o, champ) {
                            let atk = o.attack_effect.as_ref().unwrap();
                            let rng = (atk.range as u64) + 20000 + (champ.stat_buff_cached.range as u64) + (champ.level as u64 - 1) * (atk.growth_range as u64) + radius_eff(champ) + radius_eff(o);
                            let dmg = atk.expected_damage_target(data.context, o, champ);
                            if dmg * 2 < champ.hp || rng * rng < dist2(champ, o) { res.push(A::AroundRegion(game_ai::SmallActionAroundRegion::new(version, rnd, data, player, 7, 5))); path += "L387 "; }
                            else { res.push(A::RunAway(game_ai::SmallActionRunAway::new(data, player, 5))); path += "L393 "; }
                        } else { res.push(A::Around(game_ai::SmallActionAround::new(version, rnd, data, player, o.id, 5))); path += "L384 "; }
                    } else { res.push(A::Around(game_ai::SmallActionAround::new(version, rnd, data, player, o.id, 5))); path += "L395 "; }
                }
                _ => { res.push(A::AroundRegion(game_ai::SmallActionAroundRegion::new(version, rnd, data, player, 7, 5))); path += "L403/406 "; }
            }
            // ★L409~416 은 `if res.is_empty()` 블록 **안**이다(IR: %800 의 preds 가 전부 match arm 끝 · len!=0 이면 %655(L419)로 직행) — 명세 logic 의 최상위 배치는 실오류
            if g.is_visible(1 - team, champ.id) && (enemies.iter().any(|c| !same_team(*c, champ) && dist2(*c, champ) < 150000u64 * 150000 + 1)
                || data.cache.others[1 - team].iter().any(|e| dist2(e, champ) < 150000u64 * 150000 + 1)) {
                res.push(A::RunAway(game_ai::SmallActionRunAway::new(data, player, 5))); path += "L409 ";
            }
        }
        if in_range { res.push(A::RunAway(game_ai::SmallActionRunAway::new_with_skill(data, player, 5, false))); path += "L419 "; }
    }
    let mut move_actions = res;
    // L207
    if let Some(t) = tower {
        if rd::<i64>(ep(t), 0x68) == 2 && rd::<i64>(ep(t), 0x88) & 1 == 1 && rd::<usize>(ep(t), 0x98) == champ.id {
            act.clear(); move_actions.clear();
            move_actions.push(A::RunAway(game_ai::SmallActionRunAway::new_with_skill(data, player, 5, true))); path += "L207 ";
        }
    }
    *log += &format!("path={} act={} move=[{}] ", path, act.len(), move_actions.iter().map(|a| tag(a).to_string()).collect::<Vec<_>>().join(","));
    if act.is_empty() {
        let acc = player.info.parameter.positioning_accuracy() as u64; let maxv = 2000 - acc;
        let bb = &data.blackboard[team];
        let flee = enemies.iter().any(|c| { let c: &Entity = *c;
            let mut jrng = game_ai::range_misjudge_rng(version, data, player, c.id);
            let emr = game_ai::plan_legacy::old::max_range_can_use(c, champ) * game_ai::range_misjudge_roll(rnd, &mut jrng, acc, maxv) / 1000 + 10000;
            let mr = game_ai::plan_legacy::old::max_range_can_use(champ, c);
            let rv = bb.is_recent_visible(g, player, c);
            let st: i64 = rd(ep(c), 0x70); let ty: i64 = rd(ep(c), 0x68);
            *log += &format!("{{L224 e{} emr={} mr={} rv={} d2={} st={}}} ", c.id, emr, mr, rv, dist2(c, champ), st);
            rv && dist2(c, champ) <= emr * emr && (ty != 13 || st < 3) && mr == 0 && !c.block_input()
        });
        if flee { *log += "L234 "; return vec![pa_of(&A::RunAway(game_ai::SmallActionRunAway::new_with_skill(data, player, 5, false)), "L234")]; }
        // L237 max_by_key(마지막 최대)
        let mut best_i = 0usize; let mut best_s = i64::MIN;
        for (i, a) in move_actions.iter().enumerate() { let s = sp.score(version, param, rnd, player, data, a, dbg); *log += &format!("s{}={} ", tag(a), s); if s >= best_s { best_s = s; best_i = i; } }
        let best = &move_actions[best_i];
        let traj = g.iter_projectile().any(|p| { let pp = p as *const _ as *const u8; !(rd::<i64>(pp, 0) == rd::<i64>(ep(champ), 0) && (rd::<i64>(pp, 0) == 1 || rd::<usize>(pp, 8) == rd::<usize>(ep(champ), 8)))
            && { let mt: i64 = rd(pp, 0x40); mt != 6 && mt != 7 && !(mt == 1 /*BouncingTarget Some*/) }
            && { let dx = champ.x.abs_diff(rd::<u64>(pp, 0x100)); let dy = champ.y.abs_diff(rd::<u64>(pp, 0x108)); dx * dx + dy * dy < 420000u64 * 420000 } });
        let rushing = enemies.iter().any(|c| { let c: &Entity = *c; let r: i64 = rd(ep(c), 0x308); r >= 0 || r == -9223372036854775805 });
        if traj || rushing {
            *log += "L244 ";
            let mut b = best.clone();
            let input = b.get_input(version, rnd, player, data, ps_data, dbg);
            let ip = &input as *const _ as *const u8; let itag: i64 = rd(ip, 0);
            *log += &format!("input_tag={} ", itag);
            if itag == 0 {
                let x: u64 = rd(ip, 8); let y: u64 = rd(ip, 16);
                let ps2 = game_ai::position_score_at_position(version, player, data, ps_data, x, y, game_ai::PositionEvalPurpose::Objective);
                let p2 = &ps2 as *const _ as *const u8;
                if rd::<u8>(p2, 0x30) != 0 || rd::<u8>(p2, 0x31) != 0 { *log += "L260 "; return vec![pa_of(&A::RunAway(game_ai::SmallActionRunAway::new_with_skill(data, player, 5, false)), "L260")]; }
                *log += "L262 "; return vec![pa_of(&best.clone(), "L262")];
            }
            *log += "L265 "; return vec![pa_of(&best.clone(), "L265")];
        }
        *log += "L268 "; return vec![pa_of(&best.clone(), "L268")];
    }
    *log += "L273 ";
    act.iter().map(|a| pa_of(a, "L273")).collect()
}

pub fn mkgame2(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext, jud: usize) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60; st.positioning = jud;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 99 { let _ = game_ai::interaction_score as *const (); }
    let mut m = HashMap::new();
    for s in argv.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.parse::<i64>().unwrap_or(0)); } }
    let a = Args { m };
    let version = a.get("version", 2) as usize;
    let tick = a.get("tick", 1000) as usize;
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame2(&setting, &ms, &map, &ctx, a.get("jud", 80) as usize);
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let cpos = a.get("cpos", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[cpos]).expect("player");
    let champ = cache.player_champion[0][cpos].expect("champ");
    let cp = ep(champ);
    let cx = a.get("cx", 480000) as u64; let cy = a.get("cy", 480000) as u64;
    wr(cp, 0x660, cx); wr(cp, 0x668, cy);
    if a.get("chp", -1) >= 0 { wr(cp, 0x670, a.get("chp", 0) as usize); }
    // 아군 나머지 멀리
    for s in 0..5usize { if s != cpos { if let Some(e) = cache.player_champion[0][s] { wr(ep(e), 0x660, 900000u64); wr(ep(e), 0x668, 900000u64); } } }
    // 적: e0 = slot 0 at champ + (e0dx, e0dy) · 나머지 멀리(또는 e1 도)
    let e0 = cache.player_champion[1][0].expect("e0");
    wr(ep(e0), 0x660, (cx as i64 + a.get("e0dx", 900000)) as u64); wr(ep(e0), 0x668, (cy as i64 + a.get("e0dy", 0)) as u64);
    let e1 = cache.player_champion[1][1].expect("e1");
    wr(ep(e1), 0x660, (cx as i64 + a.get("e1dx", 900000)) as u64); wr(ep(e1), 0x668, (cy as i64 + a.get("e1dy", 0)) as u64);
    for s in 2..5usize { if let Some(e) = cache.player_champion[1][s] { wr(ep(e), 0x660, 900000u64); wr(ep(e), 0x668, 900000u64); } }
    // 미니언·소환물 없음(others 는 타워 등) — 타워를 멀리 보낼지: twfar=1 이면 적 타워 전부 (1,1)
    if a.get("twfar", 0) == 1 { for t in 0..2usize { for tw in cache.iter_towers_without_nexus(t) { wr(ep(tw), 0x660, 1u64); wr(ep(tw), 0x668, 1u64); } } }
    // 공격 이펙트
    if a.get("catk", -1) >= 0 { set_attack(champ, 50, a.get("catk", 0) as u64); }
    if a.get("catk", -1) == -2 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); } }
    if a.get("cnoskill", 0) == 1 { unsafe { let e = cp as *mut Entity; std::ptr::write(&mut (*e).skill_effect, None); std::ptr::write(&mut (*e).skill2_effect, None); std::ptr::write(&mut (*e).ult_effect, None); } }
    if a.get("eatk", -1) == -2 { unsafe { std::ptr::write(&mut (*(ep(e0) as *mut Entity)).attack_effect, None); } }
    if a.get("eatk", -1) >= 0 { set_attack(e0, 50, a.get("eatk", 0) as u64); }
    if a.get("e1atk", -1) >= 0 { set_attack(e1, 50, a.get("e1atk", 0) as u64); }
    // 가시성
    if a.get("evis", 1) == 1 { unsafe { std::ptr::write_volatile(&mut (*(ep(e0) as *mut Entity)).visible_state[0], VisibleState::Visible);
                                       std::ptr::write_volatile(&mut (*(ep(e1) as *mut Entity)).visible_state[0], VisibleState::Visible); } }
    if a.get("cvis", 0) == 1 { unsafe { std::ptr::write_volatile(&mut (*(cp as *mut Entity)).visible_state[1], VisibleState::Visible); } }
    if a.get("seen", 1) == 1 { for i in 0..5 { bb[0].last_visible[i] = tick; bb[1].last_visible[i] = tick; } }
    // 적 action_state / rush
    if a.get("e0st", -1) >= 0 { wr(ep(e0), 0x70, a.get("e0st", 0)); }
    if a.get("e0rush", 0) == 1 { wr(ep(e0), 0x308, -9223372036854775805i64); }
    // 타워가 나를 조준(L207)
    let tw_mode = a.get("twr", 0);
    if tw_mode >= 1 {
        let mut best: Option<&Entity> = None; let mut bd = u64::MAX;
        for tw in cache.iter_towers_without_nexus(1) { if tw.can_target && tw.block_target_tick == 0 { let d = dist2(tw, champ); if d < bd { bd = d; best = Some(tw); } } }
        if let Some(tw) = best { wr(ep(tw), 0x88, 1i64); wr(ep(tw), 0x98, if tw_mode == 1 { champ.id } else { 999999usize }); }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let g: &dyn AbstractGame = &game;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(a.get("seed", 9) as u64);
    let mut dbg: DebugFrameData = Default::default();
    let mut sp = game_ai::calculate_score_parameter(version, &mut rnd, player, &data, &mut dbg);
    let tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
    // 세계 요약
    let twn: Vec<(usize, u64, u64, bool, usize)> = cache.iter_towers_without_nexus(1).map(|t| (t.id, t.x, t.y, t.can_target, t.block_target_tick)).collect();
    println!("world\ttick={} champ=({},{}) id={} hp={} lvl={} atk={} e0=({},{}) id={} atk={} e1=({},{}) e1atk={} others1={} towers1={:?}",
        g.tick(), champ.x, champ.y, champ.id, champ.hp, champ.level, champ.attack_effect.as_ref().map(|e| e.range as i64).unwrap_or(-1),
        e0.x, e0.y, e0.id, e0.attack_effect.as_ref().map(|e| e.range as i64).unwrap_or(-1), e1.x, e1.y, e1.attack_effect.as_ref().map(|e| e.range as i64).unwrap_or(-1),
        cache.others[1].len(), twn);
    println!("mr\tchamp->e0={} e0->champ={} nearly40={} nearly60={} rv={} vis_to_enemy={} evis={} d2={}",
        game_ai::plan_legacy::old::max_range_can_use(champ, e0), game_ai::plan_legacy::old::max_range_can_use(e0, champ),
        game_ai::plan_legacy::old::max_range_nearly_can_use(e0, champ, 40), game_ai::plan_legacy::old::max_range_nearly_can_use(e0, champ, 60),
        data.blackboard[0].is_recent_visible(g, player, e0), g.is_visible(1, champ.id), visible_to(e0, champ), dist2(champ, e0));
    println!("tls_before\t{}", tls_state());
    // 오라클(게임) — 명세 대상 함수 직접 호출. order=1 이면 predict 를 먼저 돌린다(TLS 메모 재생 여부 판별)
    let mut rnd_g = rnd.clone(); let mut rnd_m = rnd.clone();
    let mut dbg_g: DebugFrameData = Default::default(); let mut dbg_m: DebugFrameData = Default::default();
    let mut self_g: game_ai::plan_legacy::sub_plan::EpicPokeSubPlan = Default::default();
    let mut log = String::new();
    let order = a.get("order", 0);
    let mut mv: Vec<PA> = Vec::new();
    if order == 1 { mv = predict(&pool, version, &mut rnd_m, player, &data, &sp, &tp, &mut dbg_m, &mut log); println!("tls_mid	{}", tls_state()); }
    let out = self_g.action_candidates(version, &mut rnd_g, player, &data, &sp, &tp, &mut dbg_g);
    println!("tls_after	{}", tls_state());
    let gv: Vec<PA> = out.iter().map(|a| pa_of(a, "game")).collect();
    if let Some(a0) = out.iter().next() {
        let p = a0 as *const _ as *const u8;
        println!("elem0	tag={} +0={} +8={} +0x10={} +0x18={} +0x20={} +0x28={} +0x30={} +0x55={} +0x58={} +0x60={} +0x68={} +0x70={} +0x75={} +0x78={} +0x7d={} +0x80={} +0x81={} +0x82={} +0x83={} +0x88={} +0x90..94={},{},{},{},{} +0x95={}",
            rd::<u8>(p, 0xb1), rd::<u64>(p, 0), rd::<u64>(p, 8), rd::<u64>(p, 0x10), rd::<u64>(p, 0x18), rd::<i64>(p, 0x20), rd::<u64>(p, 0x28), rd::<u64>(p, 0x30),
            rd::<u8>(p, 0x55), rd::<u64>(p, 0x58), rd::<u64>(p, 0x60), rd::<u64>(p, 0x68), rd::<u64>(p, 0x70), rd::<u8>(p, 0x75), rd::<u64>(p, 0x78), rd::<u8>(p, 0x7d), rd::<u8>(p, 0x80), rd::<u8>(p, 0x81), rd::<u8>(p, 0x82), rd::<u8>(p, 0x83),
            rd::<u64>(p, 0x88), rd::<u8>(p, 0x90), rd::<u8>(p, 0x91), rd::<u8>(p, 0x92), rd::<u8>(p, 0x93), rd::<u8>(p, 0x94), rd::<u8>(p, 0x95));
    }
    if order != 1 { mv = predict(&pool, version, &mut rnd_m, player, &data, &sp, &tp, &mut dbg_m, &mut log); }
    let g_next = rnd_g.next_u64(); let m_next = rnd_m.next_u64();
    let gs: Vec<String> = gv.iter().map(|p| format!("{}:{}:{}", p.tag, p.with_skill, p.target)).collect();
    let msv: Vec<String> = mv.iter().map(|p| format!("{}:{}:{}", p.tag, p.with_skill, p.target)).collect();
    let verdict = if gs == msv && g_next == m_next { "MATCH" } else if gs == msv { "MATCH_RND_DIFF" } else { "MISMATCH" };
    println!("result\t{}\tgame=[{}]\tmine=[{}]\trnd_g={} rnd_m={}\tlog={}", verdict, gs.join(" "), msv.join(" "), g_next, m_next, log);
}
