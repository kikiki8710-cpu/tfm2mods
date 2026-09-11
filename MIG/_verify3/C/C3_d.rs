#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 C — probe D
//  11: v3_fall_back_to_passive 실행 (version 게이트 · 튜토리얼 게이트 · writes 오프셋)
//  12: handle_chat 4단 게이트 표본 재확인
//  10: objective_entity_id_for_main_objective — live_list 를 채워 태그 0/1 과 그 외를 분리
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::plan_legacy::old::objective_entity_id_for_main_objective;
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
use rand::SeedableRng;
use std::sync::Arc;

fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(h as *const LegacyPlanHandler as *const u8,
        std::mem::size_of::<LegacyPlanHandler>()).to_vec() }
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

fn tuts() -> Vec<(TutorialType, &'static str)> {
    vec![(TutorialType::None, "None"), (TutorialType::First, "First"),
         (TutorialType::TopSolo, "TopSolo"), (TutorialType::Bottom, "Bottom"),
         (TutorialType::MidSolo, "MidSolo"), (TutorialType::MidBottom, "MidBottom"),
         (TutorialType::JungleOnly, "JungleOnly"), (TutorialType::Line, "Line"),
         (TutorialType::Total, "Total")]
}

fn main() {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let pn = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let pool = bumpalo::Bump::new();

    for (tut, tn) in tuts().iter() {
        let mut setting: GameSetting = Default::default();
        setting.tick_per_second = 60; setting.width = 960000; setting.height = 960000; setting.champion_radius = 10000;
        let mw: MacroWeights = Default::default();
        let ms: MapSetting = Default::default();
        let map = MapDef::moba(&setting);
        let champs: Vec<String> = Vec::new();
        let items: Vec<Box<dyn ItemInfo>> = Vec::new();
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: *tut, trace_level: TraceLevel::Off };
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
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();

        // ---- 11: v3_fall_back_to_passive ----
        for rp in 0..5usize {
            let ps = game.get_player_by_position(0, poss[rp]).unwrap();
            for ver in [0usize, 1, 2, 3, 5] {
                let mut h = LegacyPlanHandler::new(0, &mut rnd, ver, poss[rp]);
                let before = snap(&h);
                let plan_before = format!("{:?}", h.plan);
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    h.v3_fall_back_to_passive(ver, &mut rand::rngs::StdRng::seed_from_u64(9), ps, &data, &mut dbg);
                }));
                let after = snap(&h);
                let d = diff_ranges(&before, &after);
                let rs: Vec<String> = d.iter().map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect();
                println!("r11\ttut={}\tpos={}\tver={}\tok={}\tndiff={}\tmf_swap=({},{})\tfallbacks={}\tplan_changed={}\tdiff={}",
                    tn, pn[rp], ver, r.is_ok(), d.len(),
                    after[0x1610], rd_u64(&after, 0x1618), rd_u64(&after, 0x1628),
                    plan_before != format!("{:?}", h.plan), rs.join(","));
            }
        }

        // ---- 12: handle_chat 4단 게이트 표본 ----
        let sample_chats: Vec<(&str, Chat)> = vec![
            ("BattleHelp", Chat::BattleHelp(0, 0)),
            ("GankRequest", Chat::GankRequest(LineType::Top, 0)),
            ("Start", Chat::Start(0)),
        ];
        for rp in [0usize, 1, 2] {
            let ps = game.get_player_by_position(0, poss[rp]).unwrap();
            for fp in 0..5usize {
                for (cn, ch) in sample_chats.iter() {
                    let mut h = LegacyPlanHandler::new(0, &mut rnd, 3, poss[rp]);
                    let before = snap(&h);
                    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        h.handle_chat(3, &mut rand::rngs::StdRng::seed_from_u64(9), ps, &data, poss[fp], *ch, false, &mut dbg);
                    }));
                    let after = snap(&h);
                    let d = diff_ranges(&before, &after);
                    println!("r12\ttut={}\trecv={}\tfrom={}\tchat={}\tok={}\tndiff={}\tff_recv={}\ttrace_len={}",
                        tn, pn[rp], pn[fp], cn, r.is_ok(), d.len(), rd_u64(&after, 0x15b8), rd_u64(&after, 0x868));
                }
            }
        }
    }

    // ---- 10: objective_entity_id_for_main_objective, live_list 를 채운 상태 ----
    {
        let mut setting: GameSetting = Default::default();
        setting.tick_per_second = 60; setting.width = 960000; setting.height = 960000; setting.champion_radius = 10000;
        let mw: MacroWeights = Default::default();
        let ms: MapSetting = Default::default();
        let map = MapDef::moba(&setting);
        let champs: Vec<String> = Vec::new();
        let items: Vec<Box<dyn ItemInfo>> = Vec::new();
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
        game.mode.jungle_runner.epic.live_list = vec![777usize, 888];
        game.mode.jungle_runner.serpen.live_list = vec![555usize, 666];
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut mos: Vec<(String, MainObjective)> = Vec::new();
        for ph in [ObjectPhase::None, ObjectPhase::Hunt] { for wb in [false, true] {
            mos.push((format!("Morgard/{:?}/wb={}", ph, wb), MainObjective::Morgard { phase: ph, with_battle: wb }));
            mos.push((format!("Serpen/{:?}/wb={}", ph, wb), MainObjective::Serpen { phase: ph, with_battle: wb }));
        }}
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
            println!("objid2\t{}\t=>{}", n, r);
        }
    }
}
