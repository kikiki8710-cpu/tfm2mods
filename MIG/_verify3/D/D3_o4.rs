#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 4단계 : 15 open[3] `SinglePlanBattle::update` 게이트 실측
//  D3_o3 은 전 조합 RunAway 였다. 원인 후보 = 디폴트 데이터 빈곤(stat_cached.hp == 1 · 이펙트 range 0).
//  ★여기서 노브를 열어 sub_goal 이 갈라지는지 본다: world.tick(pub) · stat/stat_cached(pub) ·
//    attack_effect.range(pub) · battle.start_tick / help_called / with_dive / region (pub 필드)
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal, BattleSubPlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn committed(g: &BattleSubPlanGoal) -> bool {
    !matches!(g, BattleSubPlanGoal::KitingBack { .. } | BattleSubPlanGoal::RunAway | BattleSubPlanGoal::End)
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
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    let pos_score: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();
    const A: usize = 18;    // C0Top
    const T: usize = 23;    // C1Top

    // ── 전 챔피언에 실전풍 스탯을 넣는다 ────────────────────────
    let ids: Vec<usize> = game.world.champion_ids.clone();
    for id in ids.iter() {
        let e = game.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 100; e.stat_cached.attack = 100;
        e.stat.defence = 30; e.stat_cached.defence = 30;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        if let Some(f) = e.skill_effect.as_mut()  { f.range = 60000; }
    }

    println!("meta\tbaseline_stat_cached_hp_before_patch=1 (실측) -> now {}",
             game.world.entity.get(A).unwrap().stat_cached.hp);

    macro_rules! setpos {
        ($id:expr, $x:expr, $y:expr) => {{ let e = game.world.entity.get_mut($id).unwrap(); e.x = $x; e.y = $y; }};
    }
    // 반환: (in_tower, viable, sub_goal, committed)
    macro_rules! run {
        ($ver:expr, $tid:expr, $dive:expr, $start_tick:expr, $help:expr) => {{
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let player = game.get_player_by_position(0, Position::Top).unwrap();
            let g = cache.game;
            let ent = g.get_entity_by_id($tid);
            let in_tower = ent.map(|e| game_ai::engage_requires_dive(player, &data, e)).unwrap_or(false);
            let mut viable: Option<bool> = None;
            let mut sgs = String::from("-"); let mut com = false;
            let use_dive = $dive && in_tower;
            let ok = if in_tower {
                let target = ent.unwrap();
                let v = old::single_tower_dive_is_viable($ver, &mut rnd, player, &data, &tp, target, &mut dbgf);
                viable = Some(v); v
            } else { true };
            if ok {
                let mut b = if use_dive {
                    SinglePlanBattle::new_dive($ver, BattlePlanGoal::TryKill($tid, 60), &data, player)
                } else {
                    SinglePlanBattle::new($ver, BattlePlanGoal::TryKill($tid, 60), &data, player)
                };
                if let Some(s) = $start_tick { b.start_tick = s; }
                if $help { b.help_called = true; }
                b.update($ver, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
                sgs = format!("{:?}", b.sub_goal); com = committed(&b.sub_goal);
            }
            (in_tower, viable, sgs, com)
        }};
    }

    // ── (F) 스탯 개방 후 거리 스윕 ──────────────────────────────
    println!("--- F: distance sweep after stat opening (tick=0) ---");
    setpos!(T, 500000, 500000);
    for d in [0u64, 1000, 30000, 49999, 50000, 50001, 60000, 100000, 199999, 200000,
              200001, 250000, 299999, 300000, 300001, 400000].iter() {
        setpos!(A, 500000u64.saturating_sub(*d), 500000);
        let r = run!(3usize, T, false, None::<usize>, false);
        println!("F\td={}\tsg={}\tcommitted={}", d, r.2, r.3);
    }

    // ── (G) tick / start_tick 스윕 ─────────────────────────────
    println!("--- G: tick & start_tick sweep (d=30000) ---");
    setpos!(A, 470000, 500000);
    for tk in [0usize, 29, 30, 31, 100, 119, 120, 121, 200, 1000].iter() {
        game.world.tick = *tk;
        let r0 = run!(3usize, T, false, None::<usize>, false);       // start_tick = 현재 tick
        let r1 = run!(3usize, T, false, Some(0usize), false);        // start_tick = 0 (경과 = tk)
        let r2 = run!(3usize, T, false, Some(0usize), true);         // + help_called
        println!("G\ttick={}\tst=now:{}\tst=0:{}\tst=0+help:{}", tk, r0.2, r1.2, r2.2);
    }
    game.world.tick = 0;

    // ── (H) hp 스윕 (양쪽) ─────────────────────────────────────
    println!("--- H: hp sweep (d=30000, tick=0) ---");
    for (who, id) in [("actor", A), ("target", T)].iter() {
        for frac in [100usize, 70, 50, 30, 20, 10, 5, 1].iter() {
            { let e = game.world.entity.get_mut(*id).unwrap(); e.hp = 2000 * frac / 100; }
            let r = run!(3usize, T, false, None::<usize>, false);
            println!("H\t{}\thp%={}\tsg={}", who, frac, r.2);
        }
        { let e = game.world.entity.get_mut(*id).unwrap(); e.hp = 2000; }
    }

    // ── (I) 이펙트 사거리 스윕 (505 게이트 = dist_sq > max_range_cached²?) ─
    println!("--- I: attack_effect.range sweep (d=100000, tick=0) ---");
    setpos!(A, 400000, 500000);
    for r in [0u64, 10000, 50000, 99999, 100000, 100001, 150000, 400000].iter() {
        { let e = game.world.entity.get_mut(A).unwrap();
          if let Some(f) = e.attack_effect.as_mut() { f.range = *r; } }
        let rr = run!(3usize, T, false, None::<usize>, false);
        println!("I\trange={}\tsg={}", r, rr.2);
    }
    { let e = game.world.entity.get_mut(A).unwrap();
      if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; } }

    // ── (J) 다이브 경로 (target 을 적 타워 사거리 안에) ─────────
    println!("--- J: dive path with opened stats ---");
    let t1t0 = (272000u64, 48000u64);
    for off in [0u64, 5000, 10000, 14000, 14999, 15000, 15001, 20000].iter() {
        setpos!(T, t1t0.0 + *off, t1t0.1);
        setpos!(A, t1t0.0 + *off + 30000, t1t0.1);
        let r = run!(3usize, T, true, None::<usize>, false);
        println!("J\toff={}\tin_tower={}\tviable={:?}\tsg={}\tcommitted={}", off, r.0, r.1, r.2, r.3);
    }

    // ── (K) version 스윕 재확인 (스탯 개방 후) ─────────────────
    println!("--- K: version sweep after stat opening ---");
    setpos!(T, 500000, 500000); setpos!(A, 470000, 500000);
    for ver in [0usize, 1, 2, 3, 4, 5, 30, 31, 32, 33, 50].iter() {
        let r = run!(*ver, T, false, None::<usize>, false);
        println!("K\tver={}\tsg={}\tcommitted={}", ver, r.2, r.3);
    }
    println!("--- K2: version sweep, dive path ---");
    setpos!(T, t1t0.0, t1t0.1); setpos!(A, t1t0.0 + 30000, t1t0.1);
    for ver in [0usize, 1, 2, 3, 4, 5, 30, 31, 32, 33, 50].iter() {
        let r = run!(*ver, T, true, None::<usize>, false);
        println!("K2\tver={}\tin_tower={}\tviable={:?}\tsg={}\tcommitted={}", ver, r.0, r.1, r.2, r.3);
    }
}
