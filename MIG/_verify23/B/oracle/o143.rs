#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #143 LineDefenseSubPlan::score (pub) — 직접 호출.
//! 표적: ① evaluate_action(Lane 앵커) 이 Around 계열에 항상 Some 을 돌려주므로 L978~988(Around 아군 원거리 50/100/5·200k 게이트)이
//!         도달 불가인지 — game 을 evaluate_action 직접값 / base+econ+50 두 가설과 대조 ② Attack 대상 None → base+econ−99999
//!       ③ RunAway/Stop → base+econ+0 ④ Attack 적 챔피언 → base+econ+calculate_action_score(같은 rng 순서).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn r64(p: *const Entity, off: usize) -> u64 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const u64) } }
fn r32(p: *const Entity, off: usize) -> i32 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const i32) } }

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let V = 55usize;
    let champ = cache.player_champion[0][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let ally = cache.player_champion[0][1].unwrap();
    let ap = ally as *const Entity as *mut Entity;
    let enemy = cache.player_champion[1][0].unwrap();
    // 아군 넥서스 / 아군 타워
    let mut nexus_id = None; let mut ally_tower = None;
    for id in game.world.tower_ids.iter() { if let Some(e) = game.get_entity_by_id(*id) { if e.team == TeamType::Player(0) && ally_tower.is_none() { ally_tower = Some(*id) } } }
    for id in 0..200usize { if let Some(e) = game.get_entity_by_id(id) { if e.team == TeamType::Player(0) && matches!(e.ty, EntityType::Nexus) { nexus_id = Some(id); break; } } }
    println!("ids\tchamp={} ally={} enemy={} nexus={:?} ally_tower={:?} champ=({},{}) ally=({},{})", champ.id, ally.id, enemy.id, nexus_id, ally_tower, champ.x, champ.y, ally.x, ally.y);
    w64(cp, 0x670, 500); w64(cp, 0x628, 1000);
    w64(ap, 0x660, champ.x + 300000); w64(ap, 0x668, champ.y);   // 아군을 300k 밖으로

    let sub = game_ai::plan_legacy::sub_plan::LineDefenseSubPlan { style: LineStyle::Defensive, line: LineType::Top, minion_action_type: game_ai::MinionActionType::Normal };
    let zp: std::mem::MaybeUninit<game_ai::ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let parameter: game_ai::ScoreParameter = unsafe { zp.assume_init() };
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    let mut rnd2 = rnd.clone();
    let mut dbg: DebugFrameData = Default::default();
    let mut dbg2: DebugFrameData = Default::default();

    let (act, bonus_if_live, desc): (game_ai::SmallActionPlay, Option<i64>, &str) = match case {
        0 => (game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, ally.id, 0)), Some(5), "Around 아군 챔피언 300k → (live 면) +5"),
        1 => (game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, nexus_id.unwrap_or(0), 0)), Some(100), "Around 아군 넥서스 → (live 면) +100"),
        2 => (game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, ally_tower.unwrap_or(0), 0)), Some(50), "Around 아군 타워 → (live 면) +50"),
        3 => (game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, enemy.id, 0)), Some(0), "Around 적 챔피언 → 0"),
        4 => (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, 999_999)), Some(-99999), "Attack 대상 없음 → -99999"),
        5 => (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), Some(0), "RunAway → 0"),
        6 => (game_ai::SmallActionPlay::Stop, Some(0), "Stop → 0"),
        7 => (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)), None, "Attack 적 챔피언 → calculate_action_score"),
        8 => (game_ai::SmallActionPlay::Skill2(game_ai::SmallActionSkill2::new(&data, 999_999)), Some(-99999), "Skill2 대상 없음 → -99999 (level 게이트 전에 반환)"),
        9 => (game_ai::SmallActionPlay::Ult(game_ai::SmallActionUlt::new(&data, enemy.id)), Some(0), "Ult → 0 (match 팔 없음)"),
        _ => (game_ai::SmallActionPlay::Stop, Some(0), "-"),
    };
    let tag: u8 = unsafe { *((&act as *const _ as *const u8).add(0xb1)) };
    let actx = game_ai::plan_legacy::action_eval::ActionContext { priority: game_ai::plan_legacy::action_eval::PriorityProfile::Lane, anchor: game_ai::plan_legacy::action_eval::Anchor::Lane { line: LineType::Top } };
    let ev = game_ai::plan_legacy::action_eval::evaluate_action(V, &actx, &parameter, &mut rnd2, player, &data, &act, &mut dbg2);
    let base = game_ai::interaction_score(V, &mut rnd2, player, &data, &parameter, &act, &mut dbg2);
    let econ = game_ai::line_action_economy_adjustment(V, player, &data, &parameter, &act, game_ai::MinionActionType::Normal);
    let cas = if case == 7 { let eff = champ.attack_effect.as_ref().unwrap(); Some(game_ai::calculate_action_score(V, &mut rnd2, player, &data, &parameter, &champ.attack, eff, champ.attack_speed_mult(), enemy, game_ai::MinionActionType::Normal, &mut dbg2)) } else { None };
    let g = sub.score(V, &parameter, &mut rnd, player, &data, &act, &mut dbg);
    let pred_dead = match ev { Some(v) => v, None => base + econ + bonus_if_live.or(cas).unwrap_or(0) };
    let pred_live = base + econ + bonus_if_live.or(cas).unwrap_or(0);
    let m = if g == pred_dead && Some(g) == ev { "MATCH(evaluate_action Some 그대로 · L978 팔 미도달)" }
            else if g == pred_dead { "MATCH(None 경로 base+econ+bonus)" } else if g == pred_live { "MATCH_LIVE(L978 팔 도달)" } else { "**MISMATCH**" };
    println!("case{}\ttag={}\tevaluate_action={:?}\tbase={}\tecon={}\tcas={:?}\tgame={}\tpred_dead={}\tpred_live={}\t{}\t// {}", case, tag, ev, base, econ, cas, g, pred_dead, pred_live, m, desc);
}
