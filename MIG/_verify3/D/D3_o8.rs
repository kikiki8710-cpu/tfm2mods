#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 8단계 : 남은 두 술어를 확정한다
//  ① 17 base_sub_goal 의 Trace/End 갈림 = world.visible_map(격자 시야) 가설
//     (o7 에서 entity.visible_state·can_target·invisible_tick·hp=0 은 전부 무효로 판명)
//  ② 15 single_tower_dive_is_viable 이 **RNG 의존**인가 (o6 near=1 false vs o7 near=1 true 의 차이)
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{BattlePlanGoal, BattleSubPlanGoal};
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

    // ── ① visible_map / exist_map 스윕 ─────────────────────────
    println!("=== 17-b base_sub_goal vs world.visible_map / exist_map ===");
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let dump = |g: &Game| {
        let mut nz0 = 0; let mut nz1 = 0; let mut ex0 = 0; let mut ex1 = 0;
        for a in 0..30 { for b in 0..30 {
            if g.world.visible_map[0][a][b] != 0 { nz0 += 1 }
            if g.world.visible_map[1][a][b] != 0 { nz1 += 1 }
            if g.world.exist_map[0][a][b] != 0 { ex0 += 1 }
            if g.world.exist_map[1][a][b] != 0 { ex1 += 1 }
        }}
        (nz0, nz1, ex0, ex1)
    };
    let d = dump(&game);
    println!("17b\tbase visible_map nonzero: t0={} t1={} / exist_map nonzero: t0={} t1={}", d.0, d.1, d.2, d.3);
    let probe = |game: &Game, tag: &str| {
        let cache = AbstractGameWithCache::new(game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
        let ps1 = game.get_player_by_position(1, Position::Top).unwrap();
        println!("17b\t{}\tt0Top TryKill(23)={:?}\tt0Top TryKill(3=enemy tower)={:?}\tt1Top TryKill(23)={:?}\tt1Top TryKill(18)={:?}",
                 tag,
                 BattlePlanGoal::TryKill(23, 60).base_sub_goal(3, ps0, &data),
                 BattlePlanGoal::TryKill(3, 60).base_sub_goal(3, ps0, &data),
                 BattlePlanGoal::TryKill(23, 60).base_sub_goal(3, ps1, &data),
                 BattlePlanGoal::TryKill(18, 60).base_sub_goal(3, ps1, &data));
    };
    probe(&game, "base");
    for a in 0..30 { for b in 0..30 { game.world.visible_map[0][a][b] = 1; } }
    probe(&game, "visible_map[0]=1");
    for a in 0..30 { for b in 0..30 { game.world.exist_map[0][a][b] = 1; } }
    probe(&game, "+exist_map[0]=1");
    for a in 0..30 { for b in 0..30 { game.world.visible_map[0][a][b] = 0; game.world.exist_map[0][a][b] = 0; } }
    // 적 챔프를 내 챔프 옆으로 (거리 의존인가)
    { let e = game.world.entity.get_mut(23).unwrap(); e.x = 20000; e.y = 913000; }
    probe(&game, "enemy champ adjacent (no vis)");
    for a in 0..30 { for b in 0..30 { game.world.visible_map[0][a][b] = 1; } }
    probe(&game, "adjacent + visible_map[0]=1");
    // 팀 소속을 바꿔본다 (target 23 을 team0 으로)
    for a in 0..30 { for b in 0..30 { game.world.visible_map[0][a][b] = 0; } }
    { let e = game.world.entity.get_mut(23).unwrap(); e.team = TeamType::Player(0); }
    probe(&game, "E23.team := Player(0)");
    { let e = game.world.entity.get_mut(23).unwrap(); e.team = TeamType::Player(1); e.x = 913000; e.y = 15000; }
    // champion_entity_owner / player_state 쪽?
    println!("17b\tchampion_entity_owner={:?}", {
        let mut v: Vec<(usize, usize)> = game.world.champion_entity_owner.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort(); v });

    // ── ② single_tower_dive_is_viable RNG 의존성 ────────────────
    println!("=== 15-b single_tower_dive_is_viable : RNG 의존성 ===");
    let t1t0 = (272000u64, 48000u64);
    let mut game2 = mkgame(&setting, &ms, &map, &ctx);
    let ids: Vec<usize> = game2.world.champion_ids.clone();
    for id in ids.iter() {
        let e = game2.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 5000; e.stat_cached.attack = 5000;
        e.stat.defence = 0; e.stat_cached.defence = 0;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
    }
    { let e = game2.world.entity.get_mut(23).unwrap();
      e.stat.hp = 50; e.stat_cached.hp = 50; e.hp = 50; e.x = t1t0.0; e.y = t1t0.1; }
    let tw: Vec<usize> = game2.world.tower_ids.clone();
    for id in tw.iter() { let e = game2.world.entity.get_mut(*id).unwrap();
                          e.stat.hp = 100; e.stat_cached.hp = 100; e.hp = 100; }
    for (k, id) in ids.iter().enumerate().take(5) {
        let e = game2.world.entity.get_mut(*id).unwrap();
        if k == 0 { e.x = t1t0.0 + 20000; e.y = t1t0.1; } else { e.x = 15000; e.y = 913000; }
    }
    {
        let cache = AbstractGameWithCache::new(&game2 as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = game2.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        // (a) 같은 rnd 를 계속 진행시키며 20회
        let mut r1 = rand::rngs::StdRng::seed_from_u64(7);
        let mut seq = String::new();
        for _ in 0..20 { seq.push(if old::single_tower_dive_is_viable(3, &mut r1, ps, &data, &tp, e, &mut dbgf) {'T'} else {'F'}); }
        println!("15b\tsame_rng_advancing(seed7) x20 = {}", seq);
        // (b) 매번 새 시드
        let mut seq2 = String::new();
        for s in 0..20u64 {
            let mut r = rand::rngs::StdRng::seed_from_u64(s);
            seq2.push(if old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf) {'T'} else {'F'});
        }
        println!("15b\tfresh_seed 0..19        = {}", seq2);
        // (c) 타워 hp 를 올려 본다 (o6 에서 2000 이면 false 였다)
        drop(data); drop(cache);
    }
    for hp in [1usize, 50, 100, 200, 500, 1000, 2000, 5000] {
        for id in tw.iter() { let e = game2.world.entity.get_mut(*id).unwrap();
                              e.stat.hp = hp; e.stat_cached.hp = hp; e.hp = hp; }
        let cache = AbstractGameWithCache::new(&game2 as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = game2.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        println!("15b\ttower_hp={}\tviable={}", hp,
                 old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf));
    }
    // 대상 hp 스윕
    for id in tw.iter() { let e = game2.world.entity.get_mut(*id).unwrap();
                          e.stat.hp = 100; e.stat_cached.hp = 100; e.hp = 100; }
    for thp in [1usize, 50, 200, 500, 1000, 2000] {
        { let e = game2.world.entity.get_mut(23).unwrap();
          e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp; }
        let cache = AbstractGameWithCache::new(&game2 as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = game2.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        println!("15b\ttarget_hp={}\tviable={}", thp,
                 old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf));
    }
}
