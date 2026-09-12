#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치C 오라클 #2b — specs[11] mf_swap.__1(+0x1618) 틱 기록 · specs[12] 트레이스 3단계 · S12 전수 집계
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::plan_legacy::rule_scope as rs;
use rand::SeedableRng;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000;
    s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24; s.nexus_heal_decay = 100;
    s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}
const HSZ: usize = 6168;
fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(h as *const _ as *const u8, HSZ).to_vec() }
}
fn diff_ranges(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new(); let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] { let s = i; while i < a.len() && a[i] != b[i] { i += 1 } out.push((s, i)); } else { i += 1 }
    }
    out
}
fn fr(r: &[(usize, usize)]) -> String {
    r.iter().map(|&(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect::<Vec<_>>().join(",")
}
const TUTS: [TutorialType; 9] = [TutorialType::None, TutorialType::First, TutorialType::TopSolo,
    TutorialType::Bottom, TutorialType::MidSolo, TutorialType::MidBottom,
    TutorialType::JungleOnly, TutorialType::Line, TutorialType::Total];

fn main() {
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let mkctx = |tut: TutorialType, tl: TraceLevel| GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false, tutorial: tut, trace_level: tl,
    };
    let ctx0 = mkctx(TutorialType::None, TraceLevel::Off);
    let mut game = mkgame(&setting, &ms, &map, &ctx0);
    println!("setting_ok\ttrue\theight={} tps={}", setting.height, setting.tick_per_second);

    // ① mf_swap.__1 = tick (handler +0x1618) — 틱을 0 이 아니게 두고 확인
    println!("\n### S11b mf_swap = (29, tick) — tick 을 바꿔 +0x1618 이 기록되는지");
    println!("tick\tmf0\tmf1\t0x1618포함?\tdiff_ranges");
    for &tk in [0usize, 1, 777, 100000].iter() {
        game.set_tick(tk);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx0);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx0, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let before = snap(&h);
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut r2 = rand::rngs::StdRng::seed_from_u64(12);
            h.v3_fall_back_to_passive(3usize, &mut r2, player, &data, &mut dbg);
        }));
        let after = snap(&h);
        let dr = diff_ranges(&before, &after);
        let has = dr.iter().any(|&(s, e)| s < 0x1620 && e > 0x1618);
        println!("{}\t{}\t{}\t{}\t{}", tk, h.mf_swap.0, h.mf_swap.1, has, fr(&dr));
    }
    game.set_tick(0);

    // ② specs[12] 트레이스 3단계 + 전수 집계
    println!("\n### S12b handle_chat 전수 — tut 9 × trace 3 × from 5 = 135");
    let mut m = 0usize; let mut n = 0usize; let mut bad: Vec<String> = Vec::new();
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let chatset: Vec<(&str, Chat)> = vec![
        ("MorgardGiveUp", Chat::MorgardGiveUp),
        ("Cancel(TargetMissing)", Chat::Cancel(CancelReason::TargetMissing)),
    ];
    for (cnm, ch) in chatset.iter() {
        for &tut in TUTS.iter() {
            for tl in [TraceLevel::Off, TraceLevel::Summary, TraceLevel::Detailed] {
                for fromi in 0..5usize {
                    let c = mkctx(tut, tl);
                    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &c);
                    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    let data = OperationData::new(&cache, &c, &bb);
                    let player = game.get_player_by_position(0usize, Position::Top).unwrap();
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
                    let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
                    let mut dbg: DebugFrameData = Default::default();
                    let _ = catch_unwind(AssertUnwindSafe(|| {
                        let mut r2 = rand::rngs::StdRng::seed_from_u64(12);
                        h.handle_chat(3usize, &mut r2, player, &data, poss[fromi], *ch, false, &mut dbg);
                    }));
                    let pass = fromi != 0 && rs::position_exists(&c, poss[fromi]) && rs::chat_allowed(&c, ch);
                    let want = if pass && tl != TraceLevel::Off { 1usize } else { 0 };
                    let got = h.pending_trace_events.len();
                    n += 1;
                    if want == got { m += 1 } else {
                        bad.push(format!("{} tut={:?} tl={:?} from={:?} want={} got={}", cnm, tut, tl, poss[fromi], want, got));
                    }
                }
            }
        }
    }
    println!("S12_TRACE_MATCH\t{}/{}", m, n);
    for b in bad.iter() { println!("  ***DIFF*** {}", b) }

    // ③ handle_chat 이 게이트에 걸릴 때 정말 아무것도 안 바꾸는가(6168B 전량 diff)
    println!("\n### S12c 게이트 차단 시 handler 6168B 무변경 확인");
    println!("case\tdiff_ranges");
    for (nm, tut, tl, fromi) in [("자기발화(Top→Top)", TutorialType::None, TraceLevel::Detailed, 0usize),
                                  ("포지션없음(JungleOnly,Mid)", TutorialType::JungleOnly, TraceLevel::Detailed, 2usize),
                                  ("chat_allowed 거부(First,Jungle)", TutorialType::First, TraceLevel::Detailed, 1usize)].iter() {
        let c = mkctx(*tut, *tl);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &c);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &c, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let before = snap(&h);
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut r2 = rand::rngs::StdRng::seed_from_u64(12);
            h.handle_chat(3usize, &mut r2, player, &data, poss[*fromi], Chat::MorgardGiveUp, false, &mut dbg);
        }));
        let after = snap(&h);
        let dr = diff_ranges(&before, &after);
        println!("{}\t{}", nm, if dr.is_empty() { "(무변경)".to_string() } else { fr(&dr) });
    }

    // ④ specs[12] p8 misunderstood 가 이 본문에서 분기하지 않는가 (트레이스 이벤트에만 기록)
    println!("\n### S12d misunderstood=false/true — 게이트 동작 동일한가");
    for mis in [false, true] {
        let c = mkctx(TutorialType::None, TraceLevel::Summary);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &c);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &c, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut r2 = rand::rngs::StdRng::seed_from_u64(12);
            h.handle_chat(3usize, &mut r2, player, &data, Position::Mid, Chat::MorgardGiveUp, mis, &mut dbg);
        }));
        // PendingTraceEvent(184B) 의 +0x84 = misunderstood, +0x80 = from
        let ev = &h.pending_trace_events;
        if ev.len() == 1 {
            let p = &ev[0] as *const _ as *const u8;
            unsafe {
                println!("mis={}\tevents={}\tev+0x0(tag i64)={}\tev+0x80(from i32)={}\tev+0x84(mis u8)={}\tev+0xb0(tick)={}",
                    mis, ev.len(), *(p as *const i64), *(p.add(0x80) as *const i32), *p.add(0x84),
                    *(p.add(0xb0) as *const u64));
            }
        } else { println!("mis={}\tevents={}", mis, ev.len()) }
    }
    println!("\nDONE");
}
