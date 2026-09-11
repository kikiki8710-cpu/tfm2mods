#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 3단계 : 15 single_try_engage 체인 (D3_o2 의 패닉 원인 반영 재설계)
//  ★D3_o2 에서 얻은 사실: single_tower_dive_is_viable(single_battle.rs:891)
//      → fight_check.rs:960 check_kill_die_tick → 979 check_kill_die_tick_uncached
//      → 979:59 `Option<&PlayerState>::unwrap()`  ⟹ **target 이 챔피언이어야 한다**
//    (타워/넥서스를 target 으로 주면 그 unwrap 이 None → 패닉. 실전 호출부는 TryKill 대상 = 챔피언)
//  목표: open[1](version) · open[2](single_tower_dive_is_viable) · open[3](update 게이트) · open[0](TryKill.__1)
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
    let pnames = ["Top", "Jungle", "Mid", "Bottom", "Support"];
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
    const A: usize = 18;   // C0Top  (actor)
    const T: usize = 23;   // C1Top  (target)
    let t1_tower0 = (272000u64, 48000u64);   // T1#0
    let home_a = (15000u64, 913000u64);
    let home_t = (913000u64, 15000u64);

    macro_rules! setpos {
        ($id:expr, $x:expr, $y:expr) => {{ let e = game.world.entity.get_mut($id).unwrap(); e.x = $x; e.y = $y; }};
    }
    // single_try_engage 본문 재현. 반환: (in_tower, viable, sub_goal, committed, dive_tower, result_is_some)
    macro_rules! run {
        ($ver:expr, $t:expr, $p:expr, $tid:expr, $goal1:expr) => {{
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let player = game.get_player_by_position($t, poss[$p]).unwrap();
            let g = cache.game;
            let ent = g.get_entity_by_id($tid);
            let in_tower = ent.map(|e| game_ai::engage_requires_dive(player, &data, e)).unwrap_or(false);
            let mut viable: Option<bool> = None;
            let mut sgs = String::from("-");
            let mut com = false; let mut some = false; let mut dt: Option<String> = None;
            if in_tower {
                match g.get_entity_by_id($tid) {
                    None => {}
                    Some(target) => {
                        let v = old::single_tower_dive_is_viable($ver, &mut rnd, player, &data, &tp, target, &mut dbgf);
                        viable = Some(v);
                        if v {
                            let mut b = SinglePlanBattle::new_dive($ver, BattlePlanGoal::TryKill($tid, $goal1), &data, player);
                            b.dive_tower = cache.iter_towers_without_nexus(1 - $t)
                                .min_by_key(|t2| t2.distance_sq(target))
                                .and_then(|e| if let EntityType::Tower { info } = &e.ty { Some(info.ty) } else { None });
                            dt = b.dive_tower.map(|x| format!("{:?}", x));
                            b.update($ver, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
                            sgs = format!("{:?}", b.sub_goal); com = committed(&b.sub_goal); some = com;
                        }
                    }
                }
            } else {
                let mut b = SinglePlanBattle::new($ver, BattlePlanGoal::TryKill($tid, $goal1), &data, player);
                b.update($ver, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
                sgs = format!("{:?}", b.sub_goal); com = committed(&b.sub_goal); some = com;
            }
            (in_tower, viable, sgs, com, dt, some)
        }};
    }

    // ── (Z) 다이브 경로가 켜지는 배치 찾기 ──────────────────────
    // target(C1Top)을 적팀(=team1) 타워 T1#0 옆에 두면 engage_requires_dive(t0*, target)=true
    println!("--- Z: dive path enable check ---");
    for off in [0u64, 5000, 9000, 9999, 10000, 10001, 12000, 20000] {
        setpos!(T, t1_tower0.0 + off, t1_tower0.1);
        setpos!(A, t1_tower0.0 + off + 30000, t1_tower0.1);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player = game.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(T).unwrap();
        println!("Z\toff={}\terd={}", off, game_ai::engage_requires_dive(player, &data, e));
    }

    // ── (A) version 스윕 — 비다이브 경로 / 다이브 경로 둘 다 ────
    println!("--- A: version sweep ---");
    for ver in [0usize, 1, 2, 3, 4, 5, 10, 30, 31, 32, 33, 50, 100] {
        // (a) 비다이브: 양쪽 원위치 (타워 사거리 밖)
        setpos!(A, home_a.0, home_a.1); setpos!(T, home_t.0, home_t.1);
        let r1 = run!(ver, 0, 0, T, 60usize);
        // (b) 다이브: target 을 T1#0 위에, actor 는 30000 뒤
        setpos!(T, t1_tower0.0, t1_tower0.1); setpos!(A, t1_tower0.0 + 30000, t1_tower0.1);
        let r2 = run!(ver, 0, 0, T, 60usize);
        println!("A\tver={}\tNODIVE in_tower={} sg={} some={}\t|\tDIVE in_tower={} viable={:?} sg={} some={} dive_tower={:?}",
                 ver, r1.0, r1.2, r1.5, r2.0, r2.1, r2.2, r2.5, r2.4);
    }

    // ── (B) single_tower_dive_is_viable 진리표 (챔피언 target 만) ─
    println!("--- B: single_tower_dive_is_viable — champion targets only ---");
    for ver in [0usize, 1, 2, 3, 30, 32, 50] {
      for &(tx, ty, lbl) in [(t1_tower0.0, t1_tower0.1, "on_T1#0"),
                             (t1_tower0.0 + 9000, t1_tower0.1, "near_T1#0"),
                             (home_t.0, home_t.1, "home")].iter() {
        setpos!(T, tx, ty);
        for &ad in [10000u64, 30000, 100000, 300000].iter() {
            setpos!(A, tx.saturating_add(ad), ty);
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let player = game.get_player_by_position(0, Position::Top).unwrap();
            let e = cache.game.get_entity_by_id(T).unwrap();
            let v = old::single_tower_dive_is_viable(ver, &mut rnd, player, &data, &tp, e, &mut dbgf);
            println!("B\tver={}\ttgt={}\tactor_d={}\tviable={}", ver, lbl, ad, v);
        }
      }
    }

    // ── (C) update 거리 게이트 스윕 (비다이브 경로) ─────────────
    println!("--- C: distance sweep, non-dive path (target at home) ---");
    setpos!(T, home_t.0, home_t.1);
    let ds: Vec<u64> = vec![0, 1000, 10000, 50000, 100000, 150000, 190000, 199000, 199999,
                            200000, 200001, 210000, 250000, 290000, 299999, 300000, 300001,
                            350000, 400000, 500000, 800000];
    for d in ds.iter() {
        setpos!(A, home_t.0.saturating_sub(*d), home_t.1);
        let r = run!(3usize, 0, 0, T, 60usize);
        println!("C\td={}\tin_tower={}\tsg={}\tsome={}", d, r.0, r.2, r.5);
    }

    // ── (C2) hp 스윕 ──────────────────────────────────────────
    println!("--- C2: actor hp sweep (d=100000) ---");
    setpos!(A, home_t.0.saturating_sub(100000), home_t.1);
    let hp_max = { game.world.entity.get(A).unwrap().stat_cached.hp };
    for frac in [100usize, 75, 50, 30, 20, 10, 5, 1] {
        { let e = game.world.entity.get_mut(A).unwrap(); e.hp = hp_max * frac / 100; }
        let r = run!(3usize, 0, 0, T, 60usize);
        println!("C2\thp%={}\thp={}\tsg={}\tsome={}", frac, hp_max * frac / 100, r.2, r.5);
    }
    // 대상 hp 스윕
    println!("--- C3: TARGET hp sweep (d=100000) ---");
    { let e = game.world.entity.get_mut(A).unwrap(); e.hp = hp_max; }
    let hp_max_t = { game.world.entity.get(T).unwrap().stat_cached.hp };
    for frac in [100usize, 50, 20, 10, 5, 1] {
        { let e = game.world.entity.get_mut(T).unwrap(); e.hp = hp_max_t * frac / 100; }
        let r = run!(3usize, 0, 0, T, 60usize);
        println!("C3\thp%={}\tsg={}\tsome={}", frac, r.2, r.5);
    }
    { let e = game.world.entity.get_mut(T).unwrap(); e.hp = hp_max_t; }

    // ── (D) TryKill.__1 A/B ───────────────────────────────────
    println!("--- D: TryKill.__1 A/B ---");
    setpos!(A, home_t.0.saturating_sub(100000), home_t.1); setpos!(T, home_t.0, home_t.1);
    for g1 in [60usize, 0, 1, 30, 120, 99999] {
        let r = run!(3usize, 0, 0, T, g1);
        println!("D\tgoal1={}\tsg={}\tsome={}", g1, r.2, r.5);
    }
    // 다이브 경로에서도
    setpos!(T, t1_tower0.0, t1_tower0.1); setpos!(A, t1_tower0.0 + 30000, t1_tower0.1);
    for g1 in [60usize, 0, 99999] {
        let r = run!(3usize, 0, 0, T, g1);
        println!("D_dive\tgoal1={}\tin_tower={}\tviable={:?}\tsg={}\tsome={}\tdive_tower={:?}", g1, r.0, r.1, r.2, r.5, r.4);
    }

    // ── (E) 없는 id / 아군 / 타워 (★타워는 in_tower=true 라 패닉 위험 — 확인만) ──
    println!("--- E: target id 변주 (non-champion 은 erd 만 확인) ---");
    setpos!(A, home_a.0, home_a.1); setpos!(T, home_t.0, home_t.1);
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player = game.get_player_by_position(0, Position::Top).unwrap();
        for (nm, id) in [("bogus", 999999usize), ("ally_champ", 19), ("enemy_champ", 23),
                         ("own_tower", 2), ("enemy_tower", 3)].iter() {
            let ent = cache.game.get_entity_by_id(*id);
            let erd = ent.map(|e| game_ai::engage_requires_dive(player, &data, e)).unwrap_or(false);
            println!("E\t{}\tid={}\tentity_found={}\terd={}", nm, id, ent.is_some(), erd);
        }
    }
    // bogus id: in_tower=false → new() 경로가 도는가
    let r = run!(3usize, 0, 0, 999999usize, 60usize);
    println!("E2\tbogus_id\tin_tower={}\tsg={}\tsome={}", r.0, r.2, r.5);
    // 아군 champ target
    let r = run!(3usize, 0, 0, 19usize, 60usize);
    println!("E2\tally_champ\tin_tower={}\tsg={}\tsome={}", r.0, r.2, r.5);
    // 자기 자신 target
    let r = run!(3usize, 0, 0, 18usize, 60usize);
    println!("E2\tself\tin_tower={}\tsg={}\tsome={}", r.0, r.2, r.5);
}
