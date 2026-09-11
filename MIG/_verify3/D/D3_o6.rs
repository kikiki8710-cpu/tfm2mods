#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 6단계 : 17/19/18/15 의 open 을 pub 피호출자 오라클로 닫는다
//  17 closed[0]/open : BattlePlanGoal::base_sub_goal (pub, battle.rs:69) 전수 진리표
//  19 open[0]        : JungleRunner::get_camp_state (pub) — ★포인터 동일성으로 team/ty 매핑 확정
//  18 open[0]/[1]    : v3_epic_group_line · v3_epic_formation_role · v3_serpen_contest_clear_win (pub)
//  15 open[2]        : single_tower_dive_is_viable(single_battle::) 가 true 가 되는 조건 탐색
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal, BattleSubPlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

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

    // ── 19 open[0] : get_camp_state 의 team/ty 매핑 (포인터 동일성) ─
    println!("=== 19 get_camp_state mapping (pointer identity) ===");
    {
        let r: &JungleRunner = &game.mode.jungle_runner;
        let named: [(&str, *const JungleCampState); 10] = [
            ("blue_rhino", &r.blue_rhino as *const _), ("blue_mushroom", &r.blue_mushroom as *const _),
            ("blue_stump", &r.blue_stump as *const _), ("blue_bee", &r.blue_bee as *const _),
            ("red_rhino", &r.red_rhino as *const _), ("red_mushroom", &r.red_mushroom as *const _),
            ("red_stump", &r.red_stump as *const _), ("red_bee", &r.red_bee as *const _),
            ("epic", &r.epic as *const _), ("serpen", &r.serpen as *const _)];
        let camps = [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee,
                     JungleType::Morgard, JungleType::Serpen];
        // ★일부 (team,ty) 조합이 unreachable! 로 패닉한다 — 전수를 catch_unwind 로 훑는다
        std::panic::set_hook(Box::new(|_| {}));
        for team in [0usize, 1, 2, 5] {
            for c in camps.iter() {
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let got = r.get_camp_state(team, *c) as *const JungleCampState;
                    let which = named.iter().find(|(_, p)| *p == got).map(|(n, _)| *n).unwrap_or("???");
                    let st = r.get_camp_state(team, *c);
                    format!("field={}\tis_blue_side={}\tty={:?}\tnext_respawn_tick={}\trespawn_count={}\tlive={}",
                            which, st.is_blue_side, st.ty, st.next_respawn_tick, st.respawn_count, st.live_list.len())
                }));
                match res {
                    Ok(s) => println!("19\tteam={}\tcamp={:?}\t-> {}", team, c, s),
                    Err(_) => println!("19\tteam={}\tcamp={:?}\t-> ★PANIC (jungle.rs:713 unreachable!(\"team value must be 0 or 1\"))", team, c),
                }
            }
        }
        let _ = std::panic::take_hook();
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();

    // ── 17 : base_sub_goal 전수 진리표 ─────────────────────────
    println!("=== 17 BattlePlanGoal::base_sub_goal(version, player, data) ===");
    let cids: Vec<usize> = game.world.champion_ids.clone();
    let goals: Vec<(String, BattlePlanGoal)> = vec![
        ("TryKill(C1Top,60)".into(), BattlePlanGoal::TryKill(23, 60)),
        ("TryKill(C1Top,0)".into(),  BattlePlanGoal::TryKill(23, 0)),
        ("TryKill(C0Jgl,60)".into(), BattlePlanGoal::TryKill(19, 60)),
        ("TryKill(self,60)".into(),  BattlePlanGoal::TryKill(18, 60)),
        ("TryKill(T1#0,60)".into(),  BattlePlanGoal::TryKill(3, 60)),
        ("TryKill(bogus,60)".into(), BattlePlanGoal::TryKill(999999, 60)),
        ("Support(C0Jgl)".into(),    BattlePlanGoal::Support(19)),
        ("Support(C1Top)".into(),    BattlePlanGoal::Support(23)),
        ("Support(bogus)".into(),    BattlePlanGoal::Support(999999)),
        ("Response".into(),          BattlePlanGoal::Response),
        ("Avoid".into(),             BattlePlanGoal::Avoid),
    ];
    for ver in [0usize, 1, 2, 3, 30, 32, 50] {
        for (gn, g) in goals.iter() {
            let mut line = format!("17\tver={}\tgoal={}", ver, gn);
            for (t, p) in [(0usize, 0usize), (0, 1), (1, 0)].iter() {
                let ps = game.get_player_by_position(*t, poss[*p]).unwrap();
                let sg = g.base_sub_goal(ver, ps, &data);
                line.push_str(&format!("\tt{}{}={:?}", t, pnames[*p], sg));
            }
            println!("{}", line);
        }
    }

    // ── 18 : epic 헬퍼 3종 ────────────────────────────────────
    println!("=== 18 v3_epic_group_line / v3_epic_formation_role / v3_serpen_contest_clear_win ===");
    let strats: Vec<(String, MorgardUseStrategy)> = vec![
        ("Gather".into(), MorgardUseStrategy::Gather),
        ("Split14(Top)".into(), MorgardUseStrategy::Split14 { position: Position::Top }),
        ("Split14(Jungle)".into(), MorgardUseStrategy::Split14 { position: Position::Jungle }),
        ("Split14(Mid)".into(), MorgardUseStrategy::Split14 { position: Position::Mid }),
        ("Split14(Bottom)".into(), MorgardUseStrategy::Split14 { position: Position::Bottom }),
        ("Split14(Support)".into(), MorgardUseStrategy::Split14 { position: Position::Support }),
        ("Split131(Top,Bottom)".into(), MorgardUseStrategy::Split131 { position1: Position::Top, position2: Position::Bottom }),
        ("Split131(Mid,Support)".into(), MorgardUseStrategy::Split131 { position1: Position::Mid, position2: Position::Support }),
    ];
    for (sn, s) in strats.iter() {
        for t in 0..2usize {
            let ps = game.get_player_by_position(t, Position::Top).unwrap();
            let gl = old::v3_epic_group_line(*s, ps, &data);
            let mut roles = String::new();
            for p in 0..5usize {
                let ps2 = game.get_player_by_position(t, poss[p]).unwrap();
                let r = old::v3_epic_formation_role(*s, poss[p], ps2, &data);
                roles.push_str(&format!(" {}={:?}", pnames[p], r));
            }
            println!("18\tstrat={}\tteam={}\tgroup_line={:?}\troles:{}", sn, t, gl, roles);
        }
    }
    for ver in [0usize, 1, 2, 3, 30, 32, 50] {
        for t in 0..2usize {
            let ps = game.get_player_by_position(t, Position::Top).unwrap();
            println!("18\tserpen_contest_clear_win\tver={}\tteam={}\t{}", ver, t,
                     old::v3_serpen_contest_clear_win(ver, ps, &data, &tp));
        }
    }
    // player.strategy(rnd) 로 morgard_use 실측
    for t in 0..2usize {
        let ps = game.get_player_by_position(t, Position::Top).unwrap();
        // ★실측 시그니처: PlayerState::strategy(&self, &mut StdRng, &dyn AbstractGame) (player.rs:1576)
        let st = ps.strategy(&mut rnd, cache.game);
        println!("18\tstrategy\tteam={}\tmorgard_use={:?}", t, st.morgard_use);
    }
    drop(data); drop(cache);

    // ── 15 open[2] : single_tower_dive_is_viable 가 true 가 되는 조건 탐색 ─
    println!("=== 15 single_tower_dive_is_viable — true 조건 탐색 ===");
    let ids: Vec<usize> = game.world.champion_ids.clone();
    let t1t0 = (272000u64, 48000u64);
    // 파라미터: (내 공격력, 대상 hp, 대상 방어, 타워 hp, 아군 수 근접)
    for &(atk, thp, tdef, twr_hp, allies) in [
        (100usize, 2000usize, 30usize, 2000usize, 0usize),
        (2000, 200, 0, 2000, 0),
        (5000, 50, 0, 100, 0),
        (5000, 50, 0, 100, 4),
        (100, 2000, 30, 100, 4),
        (5000, 50, 0, 1, 4),
    ].iter() {
        // 스탯 세팅
        for id in ids.iter() {
            let e = game.world.entity.get_mut(*id).unwrap();
            e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
            e.stat.attack = atk; e.stat_cached.attack = atk;
            e.stat.defence = 30; e.stat_cached.defence = 30;
            e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
            e.radius = 5000;
            if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        }
        { let e = game.world.entity.get_mut(23).unwrap();
          e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp;
          e.stat.defence = tdef; e.stat_cached.defence = tdef;
          e.x = t1t0.0; e.y = t1t0.1; }
        // 적 타워 hp
        let tw_ids: Vec<usize> = game.world.tower_ids.clone();
        for id in tw_ids.iter() {
            let e = game.world.entity.get_mut(*id).unwrap();
            e.stat.hp = twr_hp; e.stat_cached.hp = twr_hp; e.hp = twr_hp;
        }
        // 아군 배치
        for (k, id) in ids.iter().enumerate() {
            if *id == 23 { continue; }
            let e = game.world.entity.get_mut(*id).unwrap();
            if k < 5 {  // team0
                if k < allies + 1 { e.x = t1t0.0 + 20000 + (k as u64) * 1000; e.y = t1t0.1; }
                else { e.x = 15000; e.y = 913000; }
            }
        }
        let cache2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb2: [Blackboard; 2] = [Default::default(), Default::default()];
        let data2 = OperationData::new(&cache2, &ctx, &bb2);
        let mut dbg2: DebugFrameData = Default::default();
        let ps = game.get_player_by_position(0, Position::Top).unwrap();
        let e = cache2.game.get_entity_by_id(23).unwrap();
        let erd = game_ai::engage_requires_dive(ps, &data2, e);
        let mut out = String::new();
        for ver in [0usize, 3, 32, 50] {
            let v = old::single_tower_dive_is_viable(ver, &mut rnd, ps, &data2, &tp, e, &mut dbg2);
            out.push_str(&format!(" v{}={}", ver, v));
        }
        println!("15\tatk={} thp={} tdef={} twr_hp={} allies={}\terd={}\tviable:{}",
                 atk, thp, tdef, twr_hp, allies, erd, out);
    }
}
