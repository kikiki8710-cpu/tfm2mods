#![allow(unused, dead_code, non_snake_case)]
// 2차 반증검증 배치 C — #12 handle_chat 실행 오라클
// 1차는 "오라클 인자 구성 불가"로 기록했다. 그 판정을 실행으로 반증한다.
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

const FF: [(usize, &str); 8] = [
    (0x15b8, "recv"), (0x15c0, "ignored"), (0x15c8, "in_battle"), (0x15d0, "no_help"),
    (0x15d8, "too_far"), (0x15e0, "low_hp"), (0x15e8, "bail"), (0x15f0, "join"),
];

fn main() {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let tuts = [
        (TutorialType::None, "None"), (TutorialType::First, "First"),
        (TutorialType::TopSolo, "TopSolo"), (TutorialType::Bottom, "Bottom"),
        (TutorialType::MidSolo, "MidSolo"), (TutorialType::MidBottom, "MidBottom"),
        (TutorialType::JungleOnly, "JungleOnly"), (TutorialType::Line, "Line"),
        (TutorialType::Total, "Total"),
    ];
    let traces = [(TraceLevel::Off, "Off"), (TraceLevel::Summary, "Summary"), (TraceLevel::Detailed, "Detailed")];

    for (trace, trn) in traces.iter() {
    for (tut, tn) in tuts.iter() {
        let pool = bumpalo::Bump::new();
        let mut setting: GameSetting = Default::default();
        setting.tick_per_second = 60;
        let mw: MacroWeights = Default::default();
        let ms: MapSetting = Default::default();
        let map = MapDef::moba(&setting);
        let champs: Vec<String> = Vec::new();
        let items: Vec<Box<dyn ItemInfo>> = Vec::new();
        let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
            tutorial: *tut, trace_level: *trace };
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
        game.init_tower(&ctx);
        game.init_nexus(&setting, &map);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();

        for rp in 0..5usize {            // 수신자 포지션
            let ps = game.get_player_by_position(0, poss[rp]).unwrap();
            for fp in 0..5usize {        // 발화자 포지션
                for (cn, ch) in chats().iter() {
                    for mis in [false, true] {
                        // Off 이외 트레이스는 채팅 1종만(출력 폭발 방지)
                        if *trn != "Off" && *cn != "BattleHelp" { continue; }
                        if *trn != "Off" && *tn != "None" { continue; }
                        let mut h = LegacyPlanHandler::new(0, &mut rnd, 3, poss[rp]);
                        let before = snap(&h);
                        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            h.handle_chat(3, &mut rnd, ps, &data, poss[fp], *ch, mis, &mut dbg);
                        }));
                        let after = snap(&h);
                        let d = diff_ranges(&before, &after);
                        let ffs: Vec<String> = FF.iter()
                            .filter(|(o, _)| rd_u64(&after, *o) != rd_u64(&before, *o))
                            .map(|(o, n)| format!("{}={}", n, rd_u64(&after, *o))).collect();
                        let rs: Vec<String> = d.iter().map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect();
                        println!("r12\ttrace={}\ttut={}\trecv={:?}\tfrom={:?}\tchat={}\tmis={}\tok={}\tndiff={}\ttrace_len={}\tult530={}\tff[{}]\tdiff={}",
                            trn, tn, poss[rp], poss[fp], cn, mis, r.is_ok(), d.len(),
                            rd_u64(&after, 0x868), rd_u64(&after, 0x530),
                            ffs.join(","), rs.join(","));
                    }
                }
            }
        }
    }}
}
