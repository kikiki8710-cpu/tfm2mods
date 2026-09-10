#![allow(unused, dead_code, non_snake_case)]
// D 배치 3단계 — 담당 pub 함수 실측 진리표
//   16 max_range_nearly_can_use / 19 best_jungle_goal · is_cleared / 15 engage_requires_dive
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn row(f: &str, inp: &str, out: &str) { println!("{}\t{}\t{}", f, inp, out); }

fn main() {
    let pool = bumpalo::Bump::new();
    let setting: GameSetting = Default::default();
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
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let st: AthleteStat = Default::default();
            let gp = GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new());
            game.add_player(gp);
            pid += 1;
        }
    }
    game.start_game(&mut rnd, &ctx);
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tps = setting.tick_per_second;
    println!("meta\ttick_per_second\t{}", tps);

    // ── 16. max_range_nearly_can_use ──────────────────────────────
    let a = cache.player_champion[0][0].unwrap();
    let b = cache.player_champion[1][0].unwrap();
    println!("meta\tlevel_a\t{}", a.level);
    for tk in [0usize, 1, 20, 39, 40, 49, 50, 59, 60, 61, 100, 200, 100000] {
        row("max_range_nearly_can_use", &format!("tick={}", tk),
            &format!("{}", old::max_range_nearly_can_use(a, b, tk)));
    }
    // 자기 자신을 대상으로
    row("max_range_nearly_can_use", "self;tick=60",
        &format!("{}", old::max_range_nearly_can_use(a, a, 60)));

    // ── 15. engage_requires_dive ──────────────────────────────────
    for t in 0..2usize {
        for p in 0..5usize {
            let ps = game.get_player_by_position(t, poss[p]).unwrap();
            for tt in 0..2usize {
                let e = cache.player_champion[tt][p].unwrap();
                row("engage_requires_dive",
                    &format!("player=t{}{};target=t{}{}", t, pnames[p], tt, pnames[p]),
                    &format!("{}", game_ai::engage_requires_dive(ps, &data, e)));
            }
        }
    }

    // ── 19. best_jungle_goal / is_cleared ─────────────────────────
    let tp: TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let camps = [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee];
    let cnames = ["Rhino", "Mushroom", "Stump", "Bee"];
    for t in 0..2usize {
        for p in 0..5usize {
            let ps = game.get_player_by_position(t, poss[p]).unwrap();
            let g = old::best_jungle_goal(3, &mut rnd, ps, &data, &tp, None, &mut dbg);
            row("best_jungle_goal", &format!("t{}{};now=None", t, pnames[p]), &format!("{:?}", g));
            let g2 = old::best_jungle_goal(3, &mut rnd, ps, &data, &tp, Some(JungleType::Bee), &mut dbg);
            row("best_jungle_goal", &format!("t{}{};now=Bee", t, pnames[p]), &format!("{:?}", g2));
        }
    }
    let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
    for ti in 0..2usize {
        for ci in 0..4usize {
            for off in [0usize, 300, 100000] {
                row("is_cleared",
                    &format!("team={};camp={};offset={}", ti, cnames[ci], off),
                    &format!("{}", old::is_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, off, &mut dbg)));
            }
        }
    }
    for ti in 0..2usize {
        for ci in 0..4usize {
            row("is_side_cleared", &format!("team={};camp={}", ti, cnames[ci]),
                &format!("{}", old::is_side_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, 0, &mut dbg)));
        }
    }
}
