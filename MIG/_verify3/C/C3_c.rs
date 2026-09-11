#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 C — probe C (pub 술어 전수 오라클)
//  10: is_enemy_well_danger / objective_entity_id_for_main_objective / is_ignored_well_enemy
//      / Blackboard::is_recent_visible(120틱) / PlayerState::strategy(object_finish)
//  11: TutorialType::player_count / rule_scope::goal_allowed 전수 / get_game_mode 2회 동일성
//  12: rule_scope::chat_allowed 전수 / position_exists 전수
//  14: hp_ratio<41 임계 (stat_cached.hp 를 실제 값으로 올려서)
use game_core::*;
use game_ai::plan_legacy::old::{can_enemy_hit_objective, is_ignored_well_enemy, objective_entity_id_for_main_objective, LineGankerPlan};
use game_ai::plan_legacy::rule_scope;
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
use game_ai::GoalData;
use rand::SeedableRng;
use std::sync::Arc;

fn tuts() -> Vec<(TutorialType, &'static str)> {
    vec![(TutorialType::None, "None"), (TutorialType::First, "First"),
         (TutorialType::TopSolo, "TopSolo"), (TutorialType::Bottom, "Bottom"),
         (TutorialType::MidSolo, "MidSolo"), (TutorialType::MidBottom, "MidBottom"),
         (TutorialType::JungleOnly, "JungleOnly"), (TutorialType::Line, "Line"),
         (TutorialType::Total, "Total")]
}

fn chats() -> Vec<(&'static str, Chat)> {
    vec![
        ("Start", Chat::Start(0)),
        ("Mia", Chat::Mia(Position::Top, 0)),
        ("JungleCheck", Chat::JungleCheck(LineType::Top, 0)),
        ("Battle", Chat::Battle(0, 0)),
        ("BattleDive", Chat::BattleDive(0, 0)),
        ("BattleLine", Chat::BattleLine(LineType::Top, 0)),
        ("BattleHelp", Chat::BattleHelp(0, 0)),
        ("BattleStop", Chat::BattleStop(StopReason::LowHp)),
        ("GankRequest", Chat::GankRequest(LineType::Top, 0)),
        ("CoverLine", Chat::CoverLine(LineType::Top, 0)),
        ("GankLineCover", Chat::GankLineCover(LineType::Top, 0)),
        ("HideLine", Chat::HideLine(LineType::Top, 0)),
        ("LineCover", Chat::LineCover(LineType::Top, 0)),
        ("DefenseLine", Chat::DefenseLine(LineType::Top, 0)),
        ("Ok", Chat::Ok(0)),
        ("Reject", Chat::Reject(0)),
        ("Cancel", Chat::Cancel(CancelReason::TargetMissing)),
        ("CounterJungle", Chat::CounterJungle(JungleType::Rhino, 0)),
        ("Lead", Chat::Lead(0)),
        ("Split", Chat::Split(LineType::Top, 0)),
        ("Press", Chat::Press(LineType::Top, 0)),
        ("Repair", Chat::Repair(0)),
        ("SerpenPrepare", Chat::SerpenPrepare(0, 0)),
        ("MorgardPrepare", Chat::MorgardPrepare(0, 0)),
        ("AttackNexus", Chat::AttackNexus(LineType::Top, 0)),
        ("DefenseNexus", Chat::DefenseNexus(0)),
        ("GankDive", Chat::GankDive(LineType::Top, 0)),
        ("PressTower", Chat::PressTower(LineType::Top, 0)),
        ("ComebackPick", Chat::ComebackPick(LineType::Top, 0)),
        ("PlayCall", Chat::PlayCall(0, 0, 0)),
        ("GankPlan", Chat::GankPlan(LineType::Top, 0, 0)),
        ("EarlyPlan", Chat::EarlyPlan(0, 0)),
        ("ReadySignal", Chat::ReadySignal(0, 0)),
    ]
}

fn main() {
    // ---- player_count (인자 없는 순수 술어) ----
    for (t, n) in tuts().iter() {
        println!("pcount\ttut={}\tplayer_count={}\tfull_match={}\tspawn_jungle={}\tspawn_epic={}\tspawn_serpen={}",
            n, t.player_count(), t.full_match(), t.spawn_jungle_camps(), t.spawn_epic(), t.spawn_serpen());
    }

    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    setting.width = 960000;
    setting.height = 960000;
    setting.champion_radius = 10000;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let pn = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top", "Mid", "Bottom"];

    // ---- goal_allowed / chat_allowed / position_exists 전수 (튜토리얼만 바뀌므로 ctx 만 새로) ----
    for (tut, tn) in tuts().iter() {
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: *tut, trace_level: TraceLevel::Off };
        // goal_allowed 전수
        let mut goals: Vec<(String, BigGoal)> = Vec::new();
        for li in 0..3usize { goals.push((format!("Line({})", ln[li]), BigGoal::Line { line: lines[li] })); }
        for jt in [JungleType::Rhino] { for team in 0..2usize {
            goals.push((format!("Jungle({:?},{})", jt, team), BigGoal::Jungle { camp: jt, team }));
        }}
        goals.push(("Epic".into(), BigGoal::Epic));
        goals.push(("Serpen".into(), BigGoal::Serpen));
        for team in 0..2usize { goals.push((format!("Nexus({})", team), BigGoal::Nexus { team })); }
        goals.push(("Battle(None)".into(), BigGoal::Battle { focus: None }));
        goals.push(("Battle(Some0)".into(), BigGoal::Battle { focus: Some(0) }));
        goals.push(("Recall".into(), BigGoal::Recall));
        let ga: Vec<String> = goals.iter().map(|(n, g)| format!("{}={}", n, rule_scope::goal_allowed(&ctx, *g))).collect();
        println!("goal_allowed\ttut={}\t{}", tn, ga.join(" "));
        // position_exists 전수
        let pe: Vec<String> = (0..5).map(|i| format!("{}={}", pn[i], rule_scope::position_exists(&ctx, poss[i]))).collect();
        println!("pos_exists\ttut={}\t{}", tn, pe.join(" "));
        // line_exists / morgard / serpen
        let le: Vec<String> = (0..3).map(|i| format!("{}={}", ln[i], rule_scope::line_exists(&ctx, lines[i]))).collect();
        println!("line_exists\ttut={}\t{}\tmorgard={}\tserpen={}", tn, le.join(" "),
            rule_scope::morgard_exists(&ctx), rule_scope::serpen_exists(&ctx));
        // chat_allowed 전수
        let ca: Vec<String> = chats().iter().map(|(n, c)| format!("{}={}", n, rule_scope::chat_allowed(&ctx, c))).collect();
        println!("chat_allowed\ttut={}\t{}", tn, ca.join(" "));
        // main_objective_allowed 전수 (10 의 MainObjective 카디널리티 확인용)
        let mut mos: Vec<(String, MainObjective)> = Vec::new();
        for ph in [ObjectPhase::None, ObjectPhase::Setup, ObjectPhase::Assemble, ObjectPhase::Hunt] {
            for wb in [false, true] {
                mos.push((format!("Morgard({:?},{})", ph, wb), MainObjective::Morgard { phase: ph, with_battle: wb }));
                mos.push((format!("Serpen({:?},{})", ph, wb), MainObjective::Serpen { phase: ph, with_battle: wb }));
            }
        }
        mos.push(("Defense".into(), MainObjective::Defense));
        mos.push(("Repair".into(), MainObjective::Repair));
        for li in 0..3usize {
            mos.push((format!("DefenseLine({})", ln[li]), MainObjective::DefenseLine(lines[li])));
            mos.push((format!("Nexus({})", ln[li]), MainObjective::Nexus(lines[li])));
            mos.push((format!("PressEpic({})", ln[li]), MainObjective::PressEpic(lines[li])));
            mos.push((format!("SplitEpic({})", ln[li]), MainObjective::SplitEpic(lines[li])));
            mos.push((format!("Gank({})", ln[li]), MainObjective::Gank { line: lines[li] }));
            mos.push((format!("Dive({})", ln[li]), MainObjective::Dive { line: lines[li] }));
            mos.push((format!("PressTower({})", ln[li]), MainObjective::PressTower { line: lines[li] }));
            mos.push((format!("Comeback({})", ln[li]), MainObjective::ComebackPick { line: lines[li], ready: false }));
        }
        if *tn == "None" {
            let ma: Vec<String> = mos.iter().map(|(n, m)| format!("{}={}", n, rule_scope::main_objective_allowed(&ctx, *m))).collect();
            println!("mo_allowed\ttut={}\t{}", tn, ma.join(" "));
        }
    }

    // ---- 게임을 만들어 엔티티가 필요한 것들 ----
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);

    // ---- get_game_mode 2회 동일성 (11 open[3]) ----
    {
        let g: &dyn AbstractGame = &game;
        let m1 = format!("{:?}", g.get_game_mode().as_moba().map(|m| m as *const MobaMode));
        let m2 = format!("{:?}", g.get_game_mode().as_moba().map(|m| m as *const MobaMode));
        println!("gamemode\tcall1={}\tcall2={}\tsame={}", m1, m2, m1 == m2);
    }

    // ---- objective_entity_id_for_main_objective 전수 (10 open[2]) ----
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut mos: Vec<(String, MainObjective)> = Vec::new();
        for ph in [ObjectPhase::None, ObjectPhase::Setup, ObjectPhase::Assemble, ObjectPhase::Hunt] {
            for wb in [false, true] {
                mos.push((format!("Morgard/{:?}/wb={}", ph, wb), MainObjective::Morgard { phase: ph, with_battle: wb }));
                mos.push((format!("Serpen/{:?}/wb={}", ph, wb), MainObjective::Serpen { phase: ph, with_battle: wb }));
            }
        }
        mos.push(("Defense".into(), MainObjective::Defense));
        mos.push(("Repair".into(), MainObjective::Repair));
        mos.push(("DefenseLine(Top)".into(), MainObjective::DefenseLine(LineType::Top)));
        mos.push(("Nexus(Top)".into(), MainObjective::Nexus(LineType::Top)));
        mos.push(("PressEpic(Top)".into(), MainObjective::PressEpic(LineType::Top)));
        mos.push(("SplitEpic(Top)".into(), MainObjective::SplitEpic(LineType::Top)));
        mos.push(("Gank(Top)".into(), MainObjective::Gank { line: LineType::Top }));
        mos.push(("Dive(Top)".into(), MainObjective::Dive { line: LineType::Top }));
        mos.push(("PressTower(Top)".into(), MainObjective::PressTower { line: LineType::Top }));
        mos.push(("Comeback(Top)".into(), MainObjective::ComebackPick { line: LineType::Top, ready: false }));
        for (n, m) in mos.iter() {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                format!("{:?}", objective_entity_id_for_main_objective(&data, *m))
            })).unwrap_or_else(|_| "PANIC".to_string());
            println!("objid\t{}\t=>{}", n, r);
        }
    }

    // ---- strategy(object_finish) : 10 의 전략 게이트 ----
    for t in 0..2usize {
        let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
        let g: &dyn AbstractGame = &game;
        let st = ps.strategy(&mut rnd, g);
        println!("strategy\tteam={}\t{:?}", t, st);
    }

    // ---- is_enemy_well_danger 진리표 (10 open[1]) ----
    {
        let step = 60000u64;
        for ver in 0..6usize {
            for t in 0..2usize {
                let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
                let mut trues: Vec<(u64, u64)> = Vec::new();
                let mut yy = 0u64;
                while yy <= setting.height { let mut xx = 0u64;
                    while xx <= setting.width {
                        if game_ai::is_enemy_well_danger(ver, ps, xx, yy) { trues.push((xx, yy)); }
                        xx += step; }
                    yy += step; }
                println!("well\tver={}\tteam={}\tn_true={}\tpts={:?}", ver, t, trues.len(), trues);
            }
        }
        // 경계 정밀화: team0/ver3 기준으로 x 축·y 축 경계
        for t in 0..2usize {
            let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
            for &(fx, fy) in [(0u64, 960000u64), (960000u64, 0u64)].iter() {
                // 우물 중심 후보에서 반경을 이분탐색
                let f = |d: u64| -> bool {
                    let x = if fx == 0 { d } else { fx.saturating_sub(d) };
                    let y = if fy == 0 { d } else { fy.saturating_sub(d) };
                    game_ai::is_enemy_well_danger(3, ps, x, y)
                };
                if f(0) {
                    let mut lo = 0u64; let mut hi = 960000u64;
                    while lo + 1 < hi { let m = lo + (hi - lo) / 2; if f(m) { lo = m } else { hi = m } }
                    println!("well_edge\tteam={}\tcorner=({},{})\tlast_true_diag={}\tfirst_false={}", t, fx, fy, lo, hi);
                } else {
                    println!("well_edge\tteam={}\tcorner=({},{})\tf(0)=false", t, fx, fy);
                }
            }
        }
    }

    // ---- is_recent_visible 120틱 (10 knobs) ----
    {
        for _ in 0..300 { let mut ff: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd, &mut ff); }
        let tick = game.world.tick;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        let enemy = cache.player_champion[1][1].unwrap();
        let g: &dyn AbstractGame = &game;
        for &back in [0usize, 100, 119, 120, 121, 200, 600, 601].iter() {
            let mut bb: Blackboard = Default::default();
            for i in 0..5 { bb.last_visible[i] = tick.saturating_sub(back); }
            println!("recentvis\ttick={}\tlast_visible={}\tback={}\tres={}",
                tick, tick.saturating_sub(back), back, bb.is_recent_visible(g, ps, enemy));
        }
    }

    // ---- hp_ratio<41 임계 (14) : stat_cached.hp 를 1000 으로 ----
    {
        let jid0 = { let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][1].unwrap().id };
        { let e = game.world.entity.get_mut(jid0).unwrap(); e.stat_cached.hp = 1000; }
        let gd: GoalData = Default::default();
        let ps_score: PositioningScoreData = Default::default();
        let mut dbg: DebugFrameData = Default::default();
        for &hp in [390usize, 400, 405, 409, 410, 411, 420, 1000].iter() {
            { let e = game.world.entity.get_mut(jid0).unwrap(); e.hp = hp; }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
            let mut gk = LineGankerPlan::new(LineType::Top, 100, 100);
            let mut r = rand::rngs::StdRng::seed_from_u64(5);
            let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                gk.update(3, &mut r, ps, &data, &gd, &ps_score, &mut dbg);
            })).is_ok();
            println!("hpgate\tmaxhp=1000\thp={}\tratio={}\tok={}\tlowhp={}", hp, hp * 100 / 1000, ok,
                format!("{:?}", gk).contains("LowHpSelf"));
        }
    }
}
