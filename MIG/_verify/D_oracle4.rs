#![allow(unused, dead_code, non_snake_case)]
// D 배치 4단계 — best_jungle_goal 의 "가장 가까운 미클리어 캠프" 규칙을 손계산과 대조
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

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

    let camps = [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump];
    let cn = ["Rhino", "Mushroom", "Bee", "Stump"]; // ★소스 배열 순서(spec 주장)
    for t in 0..2usize {
        println!("camp_pos\tteam={}\t{}", t,
            camps.iter().zip(cn.iter()).map(|(c, n)| {
                let (x, y) = map.camp_pos(*c, t == 0); format!("{}=({},{})", n, x, y)
            }).collect::<Vec<_>>().join(" "));
    }
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let e = cache.player_champion[t][p].unwrap();
        let mut best = (u64::MAX, "?");
        let mut ds = Vec::new();
        for (c, n) in camps.iter().zip(cn.iter()) {
            let (cx, cy) = map.camp_pos(*c, t == 0);
            let dx = if cx > e.x { cx - e.x } else { e.x - cx };
            let dy = if cy > e.y { cy - e.y } else { e.y - cy };
            let d = dx * dx + dy * dy;
            ds.push(format!("{}={}", n, d));
            if d < best.0 { best = (d, n); }   // 첫 최소 유지(min_by first-wins)
        }
        let got = old::best_jungle_goal(3, &mut rnd, ps, &data, &tp, None, &mut dbg);
        println!("check\tt{}{}\tpos=({},{})\t{}\tmine={}\tgame={:?}\t{}",
            t, pnames[p], e.x, e.y, ds.join(" "), best.1, got,
            if format!("{:?}", got) == best.1 { "MATCH" } else { "★MISMATCH" });
    }}
}
