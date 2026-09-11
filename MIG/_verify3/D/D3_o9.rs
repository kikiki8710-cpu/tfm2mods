#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 9단계 : 상수 확정
//  ① 17 base_sub_goal(TryKill/Support) 의 Trace↔End 거리 임계 이분탐색 (챔피언 target)
//     + 비챔피언 target(타워/없는 id)은 거리 무관 Trace 인지 재확인
//  ② 15 single_tower_dive_is_viable 이 false 가 되는 축 찾기 (깨끗한 세계에서 스윕)
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);

    // 액터 t0Top(18) 을 (500000,500000) 에, target 23 을 x 축으로 d 만큼
    macro_rules! bsg {
        ($id:expr, $t:expr) => {{
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position($t, Position::Top).unwrap();
            let r = BattlePlanGoal::TryKill($id, 60).base_sub_goal(3, ps, &data);
            match r { BattleSubPlanGoal::Trace{..} => 1u8, BattleSubPlanGoal::End => 0,
                      BattleSubPlanGoal::RunAway => 2, _ => 3 }
        }};
    }
    { let e = game.world.entity.get_mut(18).unwrap(); e.x = 500000; e.y = 500000; }
    println!("=== 17-c base_sub_goal(TryKill(champion)) 거리 임계 ===");
    let mut lo = 0u64; let mut hi = 1_000_000u64;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + mid; e.y = 500000; }
        if bsg!(23usize, 0usize) == 1 { lo = mid } else { hi = mid }
    }
    { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + lo; e.y = 500000; }
    let a = bsg!(23usize, 0usize);
    { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + hi; e.y = 500000; }
    let b = bsg!(23usize, 0usize);
    println!("17c\tTrace_max_d={} (code {})\tnext_d={} (code {})   [1=Trace 0=End]", lo, a, hi, b);
    // y 축·대각으로도 같은가
    for d in [lo, hi].iter() {
        { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000; e.y = 500000 + *d; }
        let cy = bsg!(23usize, 0usize);
        let dd = (*d as f64 / 2f64.sqrt()) as u64;
        { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + dd; e.y = 500000 + dd; }
        let cd = bsg!(23usize, 0usize);
        println!("17c\td={}\ty축={}\t대각={}", d, cy, cd);
    }
    // 비챔피언 target 은 거리 무관인가
    { let e = game.world.entity.get_mut(23).unwrap(); e.x = 913000; e.y = 15000; }
    for (nm, id) in [("enemy_tower(3)", 3usize), ("own_tower(2)", 2), ("bogus", 999999)].iter() {
        println!("17c\tnon-champion target {}\tcode={}", nm, bsg!(*id, 0usize));
    }
    // Support 도 같은 임계인가
    {
        { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + lo; e.y = 500000; }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Top).unwrap();
        let s1 = BattlePlanGoal::Support(23).base_sub_goal(3, ps, &data);
        drop(data); drop(cache);
        { let e = game.world.entity.get_mut(23).unwrap(); e.x = 500000 + hi; e.y = 500000; }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Top).unwrap();
        let s2 = BattlePlanGoal::Support(23).base_sub_goal(3, ps, &data);
        println!("17c\tSupport(23)\td={} -> {:?}\td={} -> {:?}", lo, s1, hi, s2);
    }

    // ── ② single_tower_dive_is_viable false 축 찾기 ─────────────
    println!("=== 15-c single_tower_dive_is_viable : false 가 되는 축 ===");
    let t1t0 = (272000u64, 48000u64);
    let mut g2 = mkgame(&setting, &ms, &map, &ctx);
    let ids: Vec<usize> = g2.world.champion_ids.clone();
    let tw: Vec<usize> = g2.world.tower_ids.clone();
    // 기준 배치: 모든 챔프 기본 스탯 그대로(=stat_cached.hp 1), target 을 T1#0 위에
    let mut reset = |g: &mut Game, atk: usize, def: usize, hp: usize, rad: usize, rng: u64, actor_d: u64| {
        for id in ids.iter() {
            let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = hp; e.stat_cached.hp = hp; e.hp = hp;
            e.stat.attack = atk; e.stat_cached.attack = atk;
            e.stat.defence = def; e.stat_cached.defence = def;
            e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
            e.radius = rad;
            if let Some(f) = e.attack_effect.as_mut() { f.range = rng; }
            e.x = 15000; e.y = 913000;
        }
        { let e = g.world.entity.get_mut(23).unwrap(); e.x = t1t0.0; e.y = t1t0.1; }
        { let e = g.world.entity.get_mut(18).unwrap(); e.x = t1t0.0 + actor_d; e.y = t1t0.1; }
    };
    macro_rules! viable {
        () => {{
            let cache = AbstractGameWithCache::new(&g2 as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let ps = g2.get_player_by_position(0, Position::Top).unwrap();
            let e = cache.game.get_entity_by_id(23).unwrap();
            let erd = game_ai::engage_requires_dive(ps, &data, e);
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            (erd, old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf))
        }};
    }
    // (a) 디폴트 스탯(hp=1) + 액터 거리 스윕
    for ad in [10000u64, 20000, 50000, 100000, 200000, 500000].iter() {
        reset(&mut g2, 1, 0, 1, 5000, 0, *ad);
        let (erd, v) = viable!();
        println!("15c\tdefault_stat hp=1 rng=0 actor_d={}\terd={}\tviable={}", ad, erd, v);
    }
    // (b) 스탯 개방 + 액터 거리 스윕
    for ad in [10000u64, 20000, 50000, 100000, 200000, 500000].iter() {
        reset(&mut g2, 5000, 0, 2000, 5000, 50000, *ad);
        let (erd, v) = viable!();
        println!("15c\topened atk=5000 hp=2000 rng=50000 actor_d={}\terd={}\tviable={}", ad, erd, v);
    }
    // (c) 액터 공격력 스윕
    for atk in [0usize, 1, 10, 100, 1000, 5000].iter() {
        reset(&mut g2, *atk, 0, 2000, 5000, 50000, 20000);
        let (erd, v) = viable!();
        println!("15c\tatk={}\terd={}\tviable={}", atk, erd, v);
    }
    // (d) 이펙트 사거리 스윕
    for rg in [0u64, 1000, 10000, 50000, 200000].iter() {
        reset(&mut g2, 5000, 0, 2000, 5000, *rg, 20000);
        let (erd, v) = viable!();
        println!("15c\teff_range={}\terd={}\tviable={}", rg, erd, v);
    }
    // (e) 방어력 스윕
    for df in [0usize, 30, 300, 3000].iter() {
        reset(&mut g2, 5000, *df, 2000, 5000, 50000, 20000);
        let (erd, v) = viable!();
        println!("15c\tdefence={}\terd={}\tviable={}", df, erd, v);
    }
}
