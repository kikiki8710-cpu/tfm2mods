#![allow(unused, dead_code, non_snake_case)]
// 배치 B 2차 반증검증 — 오라클 1: 도달성 스모크 테스트
//  ① MapDef.fountains 실체(07/09 공용 주장)
//  ② BigPlan::get_name 실측(05 end_plan 표 제3 출처)
//  ③ SubPlan / BattleSubPlanGoal 태그 실측
//  ④ 07 sub_plan / 08 is_end / 09 check_favorable_engage_formation 호출 가능성
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::types::BigPlan;
use game_ai::plan_legacy::sub_plan::SubPlan;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn tag64<T>(v: &T) -> i64 { unsafe { *(v as *const T as *const i64) } }

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    println!("tps_default\t{}", setting.tick_per_second);
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

    // ── ① fountains
    println!("fountains_len\t{}", map.fountains.len());
    for (i, f) in map.fountains.iter().enumerate() {
        println!("fountain\t{}\t{:?}", i, f);
    }
    println!("nexus_pos\t{:?}", map.nexus_pos);

    // ── ② BigPlan::get_name (구성 가능한 variant 만)
    let mut names: Vec<(String, String)> = Vec::new();
    names.push(("ForcePassive".into(), BigPlan::ForcePassive.get_name()));
    names.push(("ActiveRecall".into(), BigPlan::ActiveRecall(Default::default()).get_name()));
    names.push(("EpicHuntAndPoke".into(), BigPlan::EpicHuntAndPoke(Default::default()).get_name()));
    names.push(("EpicHuntAndBattle".into(), BigPlan::EpicHuntAndBattle(Default::default()).get_name()));
    names.push(("SerpenHuntAndPoke".into(), BigPlan::SerpenHuntAndPoke(Default::default()).get_name()));
    names.push(("SerpenHuntAndBattle".into(), BigPlan::SerpenHuntAndBattle(Default::default()).get_name()));
    names.push(("AttackNexus".into(), BigPlan::AttackNexus(Default::default()).get_name()));
    names.push(("DefenseNexus".into(), BigPlan::DefenseNexus(Default::default()).get_name()));
    for (v, n) in &names {
        // 05 명세의 end_plan 판정 사슬을 그대로 적용
        let ep = if n.starts_with("PassiveLine") { 1 }
            else if n.starts_with("PassiveJungle") { 2 }
            else if n.starts_with("LineGank") { 3 }
            else if n.contains("Epic") { 4 }
            else if n.contains("Serpen") { 5 }
            else if n.starts_with("ActiveRecall") { 6 }
            else if n.starts_with("Recall") { 6 }
            else if n.starts_with("Battle") { 7 }
            else if n.contains("Nexus") { 8 }
            else { 9 };
        println!("getname\t{}\t{:?}\tend_plan={}", v, n, ep);
    }

    // ── ③ 태그
    println!("subplan_default_tag\t{}", tag64(&SubPlan::default()));
    println!("subplan_size\t{}", std::mem::size_of::<SubPlan>());
    println!("bsg_size\t{}", std::mem::size_of::<old::BattleSubPlanGoal>());

    // ── ④ 게임 구성
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
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tp: TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let gd: game_ai::GoalData = Default::default();

    for t in 0..2usize { for p in 0..5usize {
        let e = cache.player_champion[t][p];
        println!("champ\t{}\t{}\t{:?}", t, p, e.map(|x| (x.id, x.x, x.y, x.hp, x.stat_cached.hp)));
    }}

    let ps = game.get_player_by_position(0, Position::Top).unwrap();

    // 07
    let hb: old::EpicHuntAndBattlePlan = Default::default();
    for v in [0usize, 1, 2, 3, 40, 50, 60] {
        let sp = hb.sub_plan(v, &mut rnd, ps, &data, &gd, &mut dbg);
        println!("o07\tver={}\ttag={}\t{:?}", v, tag64(&sp), sp);
    }

    // 08
    let hp_plan: old::EpicHuntAndPokePlan = Default::default();
    for v in [0usize, 1, 2, 24, 50, 60] {
        let r = hp_plan.is_end(v, &mut rnd, ps, &data, &tp, &mut dbg);
        println!("o08\tver={}\t{}", v, r);
    }

    // 09 — 도달성만 먼저
    let tgt = cache.player_champion[1][0].unwrap();
    for v in [0usize, 1, 2, 3, 50] {
        let r = game_ai::check_favorable_engage_formation(v, ps, &data, tgt, 200000);
        println!("o09\tver={}\t{}", v, r);
    }
}
