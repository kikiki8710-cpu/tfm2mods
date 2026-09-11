#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 11단계 : o7/o8(=true) 와 o9/o10(=false) 사이를 한 축씩 갈라 본다
//  두 세계의 차이 후보: ①target hp(50 vs 2000) ②아군 4명 위치(한 점 적층 vs 스폰 산개)
//  ③타워 hp(100 vs 1) ④적팀 챔프 위치
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let tp: TeamPlan = Default::default();
    let t1t0 = (272000u64, 48000u64);

    // 축: (target_hp, ally_stack, tower_hp, enemy_move)
    for &(thp, ally_stack, twr_hp, enemy_move) in [
        (50usize,   true,  100usize, false),   // = o7/o8 세계  (기대 true)
        (2000,      true,  100,      false),   // target hp 만 바꿈
        (50,        false, 100,      false),   // 아군 산개만 바꿈
        (50,        true,  1,        false),   // 타워 hp 만 바꿈
        (50,        true,  100,      true),    // 적팀 챔프도 이동
        (2000,      false, 1,        false),   // = o10 세계   (기대 false)
    ].iter() {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        let ids: Vec<usize> = g.world.champion_ids.clone();
        let tw: Vec<usize> = g.world.tower_ids.clone();
        for id in ids.iter() {
            let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
            e.stat.attack = 5000; e.stat_cached.attack = 5000;
            e.stat.defence = 0; e.stat_cached.defence = 0;
            e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
            e.radius = 5000;
            if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        }
        { let e = g.world.entity.get_mut(23).unwrap();
          e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp; e.x = t1t0.0; e.y = t1t0.1; }
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                              e.stat.hp = twr_hp; e.stat_cached.hp = twr_hp; e.hp = twr_hp; }
        for (k, id) in ids.iter().enumerate().take(5) {
            let e = g.world.entity.get_mut(*id).unwrap();
            if k == 0 { e.x = t1t0.0 + 20000; e.y = t1t0.1; }
            else if ally_stack { e.x = 15000; e.y = 913000; }
        }
        if enemy_move { for id in ids.iter().skip(5) {
            if *id == 23 { continue; }
            let e = g.world.entity.get_mut(*id).unwrap(); e.x = 15000; e.y = 913000; } }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let erd = game_ai::engage_requires_dive(ps, &data, e);
        let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf);
        println!("15e\tthp={}\tally_stack={}\ttwr_hp={}\tenemy_move={}\terd={}\tviable={}",
                 thp, ally_stack, twr_hp, enemy_move, erd, v);
    }

    // ★target hp 이분탐색 (o7/o8 세계에서)
    println!("--- target hp bisect in the o7/o8 world (ally_stack=true, twr_hp=100) ---");
    let build = |thp: usize| -> bool {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        let ids: Vec<usize> = g.world.champion_ids.clone();
        let tw: Vec<usize> = g.world.tower_ids.clone();
        for id in ids.iter() {
            let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
            e.stat.attack = 5000; e.stat_cached.attack = 5000;
            e.stat.defence = 0; e.stat_cached.defence = 0;
            e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
            e.radius = 5000;
            if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        }
        { let e = g.world.entity.get_mut(23).unwrap();
          e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp; e.x = t1t0.0; e.y = t1t0.1; }
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                              e.stat.hp = 100; e.stat_cached.hp = 100; e.hp = 100; }
        for (k, id) in ids.iter().enumerate().take(5) {
            let e = g.world.entity.get_mut(*id).unwrap();
            if k == 0 { e.x = t1t0.0 + 20000; e.y = t1t0.1; } else { e.x = 15000; e.y = 913000; }
        }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf)
    };
    for thp in [1usize, 10, 50, 100, 200, 400, 500, 600, 800, 1000, 1500, 2000] {
        println!("15e\tbisect thp={}\tviable={}", thp, build(thp));
    }
}
