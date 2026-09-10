#![allow(unused, dead_code, non_snake_case)]
// 배치 B 2차 반증검증 — 오라클 3: 08 EpicHuntAndPokePlan::is_end 진리표
//  L164 objective 게이트 / L172 setup_like / L194 `15 * tick_per_second` 임계
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};
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
    let mkctx = |st: &'static GameSetting| ();
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
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbg: DebugFrameData = Default::default();
    let plan: old::EpicHuntAndPokePlan = Default::default();
    let ps = game.get_player_by_position(0, Position::Top).unwrap();

    println!("# camp_morgard_blue\t{:?}", map.camp_pos(JungleType::Morgard, true));
    println!("# camp_morgard_red\t{:?}", map.camp_pos(JungleType::Morgard, false));

    // ── L164 / L172 : objective 조합
    let phases = [ObjectPhase::None, ObjectPhase::Setup, ObjectPhase::Assemble, ObjectPhase::Hunt];
    let pn = ["None", "Setup", "Assemble", "Hunt"];
    let mut tp: TeamPlan = Default::default();
    println!("obj\tNone(default)\t{}", plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg));
    for (i, ph) in phases.iter().enumerate() {
        for wb in [false, true] {
            tp.objective = Some(MainObjective::Morgard { phase: *ph, with_battle: wb });
            let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
            println!("obj\tMorgard{{{}, wb={}}}\t{}", pn[i], wb, r);
        }
    }
    for (i, ph) in phases.iter().enumerate() {
        tp.objective = Some(MainObjective::Serpen { phase: *ph, with_battle: false });
        println!("obj\tSerpen{{{}}}\t{}", pn[i], plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg));
    }
    tp.objective = Some(MainObjective::Defense);
    println!("obj\tDefense\t{}", plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg));
    tp.objective = Some(MainObjective::Nexus(LineType::Mid));
    println!("obj\tNexus(Mid)\t{}", plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg));

    // ── L194 : tick_per_second 를 스윕해 `15 * tps` 임계를 실측
    tp.objective = Some(MainObjective::Morgard { phase: ObjectPhase::Assemble, with_battle: false });
    let mut flip: Option<usize> = None;
    let mut prev: Option<bool> = None;
    for tps in 1..=400usize {
        let mut s2: GameSetting = Default::default();
        s2.tick_per_second = tps;
        let ctx2 = GameContext {
            pool: &pool, setting: &s2, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off,
        };
        let c2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx2);
        let d2 = OperationData::new(&c2, &ctx2, &bb);
        let r = plan.is_end(3, &mut rnd, ps, &d2, &tp, &mut dbg);
        if let Some(p) = prev { if p != r && flip.is_none() { flip = Some(tps); println!("FLIP\ttps={}\t{}->{}", tps, p, r); } }
        prev = Some(r);
        if tps <= 3 || tps == 60 { println!("tps\t{}\t{}", tps, r); }
    }
    println!("flip\t{:?}", flip);
    // 15*tps < X <= 15*(tps_flip) 관계에서 X(=next_respawn_tick - tick) 를 역산
    if let Some(f) = flip { println!("infer\tnext_respawn_minus_tick in (15*{}, 15*{}]", f - 1, f); }
}
