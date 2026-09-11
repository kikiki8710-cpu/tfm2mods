#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 7단계 : o6 에서 갈린 항목의 술어를 좁힌다
//  ① 17 base_sub_goal 의 Trace/End 갈림 = **가시성(visible_state)** 가설 검정
//  ② 15 single_tower_dive_is_viable 의 아군 수 임계
//  ③ 18 v3_epic_group_line 이 Mid 이외/None 을 낼 수 있는가 (TutorialType 스윕)
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal, BattleSubPlanGoal};
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
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let pnames = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let pos_score: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();

    // ── ① base_sub_goal 가시성 검정 ────────────────────────────
    {
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off };
        let mut game = mkgame(&setting, &ms, &map, &ctx);
        println!("=== 17-a base_sub_goal: visible_state 를 손으로 바꿔 본다 (target=23, actor=t0Top) ===");
        // 기본값 덤프
        {
            let e = game.world.entity.get(23).unwrap();
            println!("17a\tbase\tE23.visible_state=[{:?}, {:?}]\tinvisible_tick={}\tcan_target={}",
                     e.visible_state[0], e.visible_state[1], e.invisible_tick, e.can_target);
            let e2 = game.world.entity.get(19).unwrap();
            println!("17a\tbase\tE19(ally).visible_state=[{:?}, {:?}]", e2.visible_state[0], e2.visible_state[1]);
            let e3 = game.world.entity.get(3).unwrap();
            println!("17a\tbase\tE3(enemy tower).visible_state=[{:?}, {:?}]", e3.visible_state[0], e3.visible_state[1]);
        }
        let variants: Vec<(&str, VisibleState)> = vec![
            ("Visible",   VisibleState::Visible),
            ("Unknown",   VisibleState::Unknown),
            ("Invisible", VisibleState::Invisible { last_x: 913000, last_y: 15000 }),
        ];
        for (nm, vs) in variants.iter() {
            { let e = game.world.entity.get_mut(23).unwrap(); e.visible_state[0] = vs.clone(); }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
            let ps1 = game.get_player_by_position(1, Position::Top).unwrap();
            let g = BattlePlanGoal::TryKill(23, 60);
            let s = BattlePlanGoal::Support(23);
            println!("17a\tE23.visible_state[0]={}\tt0Top TryKill={:?}\tSupport={:?}\t|\tt1Top TryKill={:?}",
                     nm, g.base_sub_goal(3, ps0, &data), s.base_sub_goal(3, ps0, &data),
                     g.base_sub_goal(3, ps1, &data));
        }
        // can_target / invisible_tick 도 본다
        { let e = game.world.entity.get_mut(23).unwrap(); e.visible_state[0] = VisibleState::Visible; e.can_target = false; }
        {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
            println!("17a\tvisible+can_target=false\tt0Top TryKill={:?}",
                     BattlePlanGoal::TryKill(23, 60).base_sub_goal(3, ps0, &data));
        }
        { let e = game.world.entity.get_mut(23).unwrap(); e.can_target = true; e.invisible_tick = 999; }
        {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
            println!("17a\tvisible+invisible_tick=999\tt0Top TryKill={:?}",
                     BattlePlanGoal::TryKill(23, 60).base_sub_goal(3, ps0, &data));
        }
        // hp=0 (사망 상태)
        { let e = game.world.entity.get_mut(23).unwrap(); e.invisible_tick = 0; e.hp = 0; }
        {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
            println!("17a\tvisible+hp=0\tt0Top TryKill={:?}",
                     BattlePlanGoal::TryKill(23, 60).base_sub_goal(3, ps0, &data));
        }
    }

    // ── ② single_tower_dive_is_viable 아군 수 임계 ─────────────
    {
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off };
        println!("=== 15-a single_tower_dive_is_viable: 근처 아군 수 임계 ===");
        let t1t0 = (272000u64, 48000u64);
        for allies in 0..5usize {
            let mut game = mkgame(&setting, &ms, &map, &ctx);
            let ids: Vec<usize> = game.world.champion_ids.clone();
            for id in ids.iter() {
                let e = game.world.entity.get_mut(*id).unwrap();
                e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
                e.stat.attack = 5000; e.stat_cached.attack = 5000;
                e.stat.defence = 0; e.stat_cached.defence = 0;
                e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
                e.radius = 5000;
                if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
            }
            { let e = game.world.entity.get_mut(23).unwrap();
              e.stat.hp = 50; e.stat_cached.hp = 50; e.hp = 50; e.x = t1t0.0; e.y = t1t0.1; }
            let tw: Vec<usize> = game.world.tower_ids.clone();
            for id in tw.iter() { let e = game.world.entity.get_mut(*id).unwrap();
                                  e.stat.hp = 100; e.stat_cached.hp = 100; e.hp = 100; }
            // team0 챔프 중 allies+1 명(=actor 포함)을 타워 근처로
            for (k, id) in ids.iter().enumerate().take(5) {
                let e = game.world.entity.get_mut(*id).unwrap();
                if k <= allies { e.x = t1t0.0 + 20000 + (k as u64) * 1000; e.y = t1t0.1; }
                else { e.x = 15000; e.y = 913000; }
            }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let ps = game.get_player_by_position(0, Position::Top).unwrap();
            let e = cache.game.get_entity_by_id(23).unwrap();
            let mut line = format!("15a\tnear_team0_champs={}\terd={}", allies + 1,
                                   game_ai::engage_requires_dive(ps, &data, e));
            for ver in [0usize, 3, 32] {
                line.push_str(&format!("\tv{}={}", ver,
                    old::single_tower_dive_is_viable(ver, &mut rnd, ps, &data, &tp, e, &mut dbgf)));
            }
            println!("{}", line);
        }
    }

    // ── ③ v3_epic_group_line 이 Mid 이외/None 을 낼 수 있는가 ──
    println!("=== 18-a v3_epic_group_line : TutorialType 스윕 ===");
    for tut in [TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
                TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
                TutorialType::Line, TutorialType::Total] {
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: tut, trace_level: TraceLevel::Off };
        let game = mkgame(&setting, &ms, &map, &ctx);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = match game.get_player_by_position(0, Position::Top) { Some(x) => x, None => { println!("18a\ttut={:?}\tno player", tut); continue; } };
        let gl_g = old::v3_epic_group_line(MorgardUseStrategy::Gather, ps, &data);
        let gl_s = old::v3_epic_group_line(MorgardUseStrategy::Split14 { position: Position::Top }, ps, &data);
        println!("18a\ttut={:?}\tgroup_line(Gather)={:?}\tgroup_line(Split14Top)={:?}", tut, gl_g, gl_s);
    }
}
