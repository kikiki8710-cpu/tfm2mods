#![allow(unused, dead_code, non_snake_case)]
//! 26차 L · 241 RecallSubPlan::action_candidates 오라클(pub 직접 호출).
//! 같은 시드 rnd 로 battle_action / attack_summon_action 을 따로 돌려 [Recall] ++ battle(retain) ++ summon 순서를 대조한다.
//! argv[1]: 0 = 게임 시작 상태 · 1 = 상대 5명을 내 정글러 옆(사거리 안)으로 끌어옴 · 2 = 아군 4명도 옆으로
use game_core::*;
use game_ai::*;
use game_ai::plan_legacy::sub_plan::RecallSubPlan;
use rand::SeedableRng;
use std::sync::Arc;
include!("common.rs");

fn set_xy(game: &mut Game, id: usize, x: u64, y: u64) {
    let e = game.get_entity_by_id_mut(id).unwrap() as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*e).x, x); std::ptr::write_volatile(&mut (*e).y, y); }
}
fn dump(tag: &str, v: &[SmallActionPlay]) {
    let mut s = String::new();
    for a in v {
        let p = a as *const SmallActionPlay as *const u8;
        let t = unsafe { *p.add(0xb1) };
        let tgt = unsafe { *(p.add(8) as *const u64) };
        s.push_str(&format!("[tag={} +8={}] ", t, tgt));
    }
    println!("{}\tlen={}\t{}", tag, v.len(), s);
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(100);
    let ids: Vec<(usize, usize, usize)> = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut v = vec![];
        for t in 0..2 { for p in 0..5 { v.push((t, p, c.player_champion[t][p].unwrap().id)); } }
        v
    };
    let my = ids[1].2;
    set_xy(&mut game, my, 400000, 400000);
    if case >= 1 { for &(t, p, id) in &ids { if t == 1 { set_xy(&mut game, id, 400000 + 12000 * (p as u64 + 1), 400000); } } }
    if case >= 2 { for &(t, p, id) in &ids { if t == 0 && p != 1 { set_xy(&mut game, id, 400000, 400000 + 12000 * (p as u64 + 1)); } } }
    for &(t, p, id) in &ids { let e = game.get_entity_by_id(id).unwrap(); for tt in 0..2 { unsafe { std::ptr::write_volatile(&mut (*(e as *const Entity as *mut Entity)).visible_state[tt], VisibleState::Visible); } } }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let zp: std::mem::MaybeUninit<ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let parameter: ScoreParameter = unsafe { zp.assume_init() };
    let mut sp: RecallSubPlan = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let res = RecallSubPlan::action_candidates(&mut sp, 2, &mut rnd, player, &data, &parameter);
    dump("action_candidates", &res);
    if let Some(a0) = res.first() {
        let p = a0 as *const SmallActionPlay as *const u8;
        let end_delay = unsafe { *(p.add(0x60) as *const u64) };
        let start = unsafe { *(p.add(0x48) as *const u64) };
        println!("first: tag={} start_tick(+0x48)={} end_delay(+0x60)={}", unsafe { *p.add(0xb1) }, start, end_delay);
    }
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(11);
    let battle = battle_action(2, &mut rnd2, player, &data, 5);
    dump("battle_action(raw)", &battle);
    let summon = attack_summon_action(player, &data);
    dump("attack_summon_action", &summon);
    let champ = cache.player_champion[0][1].unwrap();
    let mut kept = 0;
    for a in battle.iter() {
        let p = a as *const SmallActionPlay as *const u8;
        let t = unsafe { *p.add(0xb1) };
        let tgt = unsafe { *(p.add(8) as *const u64) } as usize;
        let keep = if (16..19).contains(&t) { match game.get_entity_by_id(tgt) { None => true, Some(e) => e.team != champ.team } } else { true };
        if keep { kept += 1; }
    }
    println!("expected len = 1 + {} + {} = {} · actual {}", kept, summon.len(), 1 + kept + summon.len(), res.len());
    std::mem::forget(res); std::mem::forget(battle); std::mem::forget(summon);
}
