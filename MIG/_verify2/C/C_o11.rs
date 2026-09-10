#![allow(unused, dead_code, non_snake_case)]
// 2차 반증검증 배치 C — #11 v3_fall_back_to_passive 실행 오라클
// 1차는 "&PlayerState/&OperationData/&mut DebugFrameData 인자 구성 불가"로 기록했다. 그 판정을 실행으로 반증한다.
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use rand::SeedableRng;
use std::sync::Arc;

fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(
            h as *const LegacyPlanHandler as *const u8,
            std::mem::size_of::<LegacyPlanHandler>(),
        ).to_vec()
    }
}

fn diff_ranges(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            let s = i;
            while i < a.len() && a[i] != b[i] { i += 1; }
            out.push((s, i));
        } else { i += 1; }
    }
    out
}

fn rd_u64(v: &[u8], o: usize) -> u64 {
    let mut x = [0u8; 8]; x.copy_from_slice(&v[o..o + 8]); u64::from_le_bytes(x)
}

fn main() {
    let tuts = [
        (TutorialType::None, "None"), (TutorialType::First, "First"),
        (TutorialType::TopSolo, "TopSolo"), (TutorialType::Bottom, "Bottom"),
        (TutorialType::MidSolo, "MidSolo"), (TutorialType::MidBottom, "MidBottom"),
        (TutorialType::JungleOnly, "JungleOnly"), (TutorialType::Line, "Line"),
        (TutorialType::Total, "Total"),
    ];
    println!("handler_size\t{}", std::mem::size_of::<LegacyPlanHandler>());

    for (tut, tn) in tuts.iter() {
        let pool = bumpalo::Bump::new();
        let mut setting: GameSetting = Default::default();
        setting.tick_per_second = 60;                       // ★BRIEF §1① : default 는 0 이다
        let mw: MacroWeights = Default::default();
        let ms: MapSetting = Default::default();
        let map = MapDef::moba(&setting);
        let champs: Vec<String> = Vec::new();
        let items: Vec<Box<dyn ItemInfo>> = Vec::new();
        let ctx = GameContext {
            pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: *tut, trace_level: TraceLevel::Off,
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
        let mut dbg: DebugFrameData = Default::default();

        for p in 0..5usize {
            let ps = game.get_player_by_position(0, poss[p]).unwrap();
            for ver in [0usize, 1, 2, 3, 5] {
                let mut h = LegacyPlanHandler::new(0, &mut rnd, ver, poss[p]);
                let before = snap(&h);
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    h.v3_fall_back_to_passive(ver, &mut rnd, ps, &data, &mut dbg);
                }));
                let after = snap(&h);
                let d = diff_ranges(&before, &after);
                let rs: Vec<String> = d.iter()
                    .map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect();
                println!("r11\ttut={}\tpos={}\tver={}\tok={}\tplan_tag {}->{}\tmf_swap=({},{})\tv3_lapse_passive_fallbacks={}\tnew_version_field={}\tdiff[{}]={}",
                    tn, pnames[p], ver, r.is_ok(),
                    before[0x5e8], after[0x5e8],
                    after[0x1610], rd_u64(&after, 0x1618),
                    rd_u64(&after, 0x1628),
                    rd_u64(&after, 0x1468),
                    d.len(), rs.join(","));
            }
        }
    }
}
