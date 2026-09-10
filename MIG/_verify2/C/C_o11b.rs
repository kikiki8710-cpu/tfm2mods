#![allow(unused, dead_code, non_snake_case)]
// #11 v3_fall_back_to_passive — GameMode 별 경로(Moba / SingleLane / DeathMatch) 실행 오라클
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use rand::SeedableRng;
use std::sync::Arc;

fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(h as *const LegacyPlanHandler as *const u8,
            std::mem::size_of::<LegacyPlanHandler>()).to_vec()
    }
}
fn diff_ranges(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new(); let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] { let s = i; while i < a.len() && a[i] != b[i] { i += 1; } out.push((s, i)); }
        else { i += 1; }
    }
    out
}
fn rd_u64(v: &[u8], o: usize) -> u64 { let mut x = [0u8; 8]; x.copy_from_slice(&v[o..o + 8]); u64::from_le_bytes(x) }

fn run(tag: &str, game: &mut dyn AbstractGame, ctx: &GameContext, setting: &GameSetting, rnd: &mut rand::rngs::StdRng) {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(rnd, ctx);
    let cache = AbstractGameWithCache::new(&*game, ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, ctx, &bb);
    let mut dbg: DebugFrameData = Default::default();
    for p in 0..5usize {
        let ps = match game.get_player_by_position(0, poss[p]) { Some(x) => x, None => { println!("r11b\t{}\tpos={:?}\tNO_PLAYER", tag, poss[p]); continue; } };
        let mut h = LegacyPlanHandler::new(0, rnd, 3, poss[p]);
        let before = snap(&h);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            h.v3_fall_back_to_passive(3, rnd, ps, &data, &mut dbg);
        }));
        let after = snap(&h);
        let d = diff_ranges(&before, &after);
        // plan 영역 0x5e8..0x768 의 앞 40바이트 덤프 (tag + 페이로드 시작)
        let head: Vec<String> = (0x5e8..0x5e8 + 40).map(|i| format!("{:02x}", after[i])).collect();
        println!("r11b\t{}\tpos={:?}\tok={}\tplan_tag {}->{}\tmf=({},{})\tcnt={}\tndiff={}\tplanhead={}",
            tag, poss[p], r.is_ok(), before[0x5e8], after[0x5e8],
            after[0x1610], rd_u64(&after, 0x1618), rd_u64(&after, 0x1628), d.len(), head.join(""));
    }
}

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();

    // --- Moba
    {
        let map = MapDef::moba(&setting);
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off };
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
        let mut g = Game::new(1234u64, false, &setting, &ms, &map);
        run("Moba", &mut g, &ctx, &setting, &mut rnd);
    }
    // --- SingleLane
    {
        let map = MapDef::single_lane(&setting);
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off };
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
        let mut g = SingleLaneGame::new(1234u64, false, &setting, &ms, &map);
        run("SingleLane", &mut g, &ctx, &setting, &mut rnd);
    }
    // --- DeathMatch
    {
        let map = MapDef::death_match(&setting);
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off };
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
        let mut g = DeathMatchGame::new(1234u64, false, &setting, &ms, &map);
        run("DeathMatch", &mut g, &ctx, &setting, &mut rnd);
    }
}
