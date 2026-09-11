#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 B — 오라클: 08 EpicHuntAndPokePlan::is_end
// 목표: (a) L164 objective 게이트  (b) L193 에픽 생존 게이트
//       (c) ★L194 `next_respawn_tick.saturating_sub(tick) > 15*tps` 임계
// ⚠ BRIEF §3① — init_tower/init_nexus 재호출 금지(start_game 이 이미 만든다)
use game_core::*;
use game_core::JungleType;
use rand::SeedableRng;
use std::sync::Arc;
use game_ai::plan_legacy::old::EpicHuntAndPokePlan;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};

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
    // (init_tower / init_nexus 는 부르지 않는다 — 2차 배치 D 오염 원인)
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let tps = setting.tick_per_second;

    // 사전 관찰: 타워 개수 sanity + 초기 epic 상태 + champion id 하나
    let (tick0, live0, nrt0, cid) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let t = (&game as &dyn AbstractGame).tick();
        let id = c.player_champion[0][0].unwrap().id;
        (t, game.mode.jungle_runner.epic.live_list.clone(),
            game.mode.jungle_runner.epic.next_respawn_tick, id)
    };
    println!("# tps={} tick0={} live_list={:?} next_respawn_tick={} champ0_id={}",
             tps, tick0, live0, nrt0, cid);

    let plan = EpicHuntAndPokePlan {
        v46_flee_threats: Vec::new(),
        focus_epic_only: false, vision_only: false, v46_flee: false,
    };
    let mut ok = 0usize; let mut bad = 0usize;

    // ── (a) L164: objective 태그 != 0 이면 즉시 true ────────────────────
    // ── (c) L194: 에픽 죽어 있고 리젠까지 15초 초과 → true
    let cases: Vec<(&str, Option<MainObjective>, Vec<usize>, usize, Option<bool>)> = vec![
        // name, objective, live_list, next_respawn_tick, expected
        ("obj=None",              None, vec![], 0, Some(true)),
        ("obj=Serpen(tag1)",      Some(MainObjective::Serpen{phase:ObjectPhase::None, with_battle:false}), vec![], 0, Some(true)),
        ("obj=Defense(tag2)",     Some(MainObjective::Defense), vec![], 0, Some(true)),
        ("Morgard nrt=0",         Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 0, Some(false)),
        ("Morgard nrt=1",         Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 1, Some(false)),
        ("Morgard nrt=899",       Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 899, Some(false)),
        ("Morgard nrt=900",       Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 900, Some(false)),
        ("Morgard nrt=901",       Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 901, Some(true)),
        ("Morgard nrt=5000",      Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![], 5000, Some(true)),
        ("Morgard epic_alive",    Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![cid], 100000, Some(false)),
        ("Morgard live_badid",    Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false}), vec![999999], 100000, Some(true)),
        ("Morgard Assemble901",   Some(MainObjective::Morgard{phase:ObjectPhase::Assemble, with_battle:false}), vec![], 901, Some(true)),
        ("Morgard Hunt900",       Some(MainObjective::Morgard{phase:ObjectPhase::Hunt, with_battle:false}), vec![], 900, Some(false)),
        // setup 경로(예상 없이 관측만)
        ("Morgard Setup nrt=901", Some(MainObjective::Morgard{phase:ObjectPhase::Setup, with_battle:false}), vec![], 901, None),
        ("Morgard Setup nrt=0",   Some(MainObjective::Morgard{phase:ObjectPhase::Setup, with_battle:false}), vec![], 0, None),
    ];

    for (name, obj, live, nrt, exp) in cases {
        game.mode.jungle_runner.epic.live_list = live.clone();
        game.mode.jungle_runner.epic.next_respawn_tick = nrt;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, poss[1]).unwrap();  // 정글러
        let mut tp: TeamPlan = Default::default();
        tp.objective = obj;
        let mut dbgf: DebugFrameData = Default::default();
        let got = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbgf);
        match exp {
            Some(e) => {
                if got == e { ok += 1; println!("OK  \t{}\tgot={}", name, got); }
                else { bad += 1; println!("MISMATCH\t{}\tgot={}\texp={}", name, got, e); }
            }
            None => println!("OBS \t{}\tgot={}", name, got),
        }
    }
    // version 무영향 확인
    let mut vdiff = 0usize;
    {
        game.mode.jungle_runner.epic.live_list = Vec::new();
        game.mode.jungle_runner.epic.next_respawn_tick = 901;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, poss[1]).unwrap();
        let mut tp: TeamPlan = Default::default();
        tp.objective = Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false});
        let mut dbgf: DebugFrameData = Default::default();
        let base = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbgf);
        for v in [0usize,1,2,3,24,30,46,50,60] {
            let mut d2: DebugFrameData = Default::default();
            if plan.is_end(v, &mut rnd, ps, &data, &tp, &mut d2) != base { vdiff += 1; }
        }
    }
    // -- setup path decomposition: (b) v24 vs (c)/(d) vision/proximity
    {
        game.mode.jungle_runner.epic.live_list = Vec::new();
        game.mode.jungle_runner.epic.next_respawn_tick = 0;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, poss[1]).unwrap();
        let mut tp: TeamPlan = Default::default();
        tp.objective = Some(MainObjective::Morgard{phase:ObjectPhase::Setup, with_battle:false});
        let act = tp.take_active(JungleType::Morgard);
        let sl  = tp.take_setup_like(JungleType::Morgard);
        let v24 = TeamPlan::v24_objective_setup_should_release_to_passive(&tp, 3, ps, &data, JungleType::Morgard);
        let (cx, cy) = map.camp_pos(JungleType::Morgard, true);
        let vis = (&game as &dyn AbstractGame).is_visible_cell(0, (cx / 32000) as usize, (cy / 32000) as usize);
        let mut dbgf: DebugFrameData = Default::default();
        let got = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbgf);
        println!("SETUP	take_active={}	take_setup_like={}	v24_release={}	camp=({},{})	cell=({},{})	is_visible_cell={}	is_end={}",
                 act, sl, v24, cx, cy, cx/32000, cy/32000, vis, got);
        let mut tp2: TeamPlan = Default::default();
        tp2.objective = Some(MainObjective::Morgard{phase:ObjectPhase::None, with_battle:false});
        println!("SETUP2	take_active={}	take_setup_like={}", tp2.take_active(JungleType::Morgard), tp2.take_setup_like(JungleType::Morgard));
    }
    println!("RESULT\tok={}\tbad={}\tversion_diff(non-setup)={}", ok, bad, vdiff);
}
