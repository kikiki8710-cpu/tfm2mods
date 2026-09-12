#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치C 오라클 #2 — specs[11] v3_fall_back_to_passive · specs[12] handle_chat · shared.rule_scope_게이트
//! 정본 템플릿 `_verify3\TEMPLATE.rs` 기반(real_setting, init_tower 미호출).
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
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            let s = i;
            while i < a.len() && a[i] != b[i] { i += 1 }
            out.push((s, i));
        } else { i += 1 }
    }
    out
}
fn fmt_ranges(r: &[(usize, usize)]) -> String {
    r.iter().map(|&(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect::<Vec<_>>().join(",")
}

const TUTS: [TutorialType; 9] = [TutorialType::None, TutorialType::First, TutorialType::TopSolo,
    TutorialType::Bottom, TutorialType::MidSolo, TutorialType::MidBottom,
    TutorialType::JungleOnly, TutorialType::Line, TutorialType::Total];

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={} height={} tps={} radius={}",
        setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0,
        setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
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
    let game = mkgame(&setting, &ms, &map, &ctx0);

    // ================= ① shared.rule_scope_게이트 진리표 =================
    println!("\n### R1 rule_scope 진리표 (tutorial 0..8)");
    println!("tut\tlineTop\tlineMid\tlineBot\tmorgard\tserpen\tposTop\tposJng\tposMid\tposBot\tposSup\tgLine0\tgLine1\tgLine2\tgJungle\tgEpic\tgSerpen\tgNexus\tgBattle\tgRecall");
    let mut rows: Vec<Vec<bool>> = Vec::new();
    for (ti, &tut) in TUTS.iter().enumerate() {
        let c = mkctx(tut, TraceLevel::Off);
        let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
        let goals = [BigGoal::Line { line: LineType::Top }, BigGoal::Line { line: LineType::Mid },
                     BigGoal::Line { line: LineType::Bottom },
                     BigGoal::Jungle { camp: JungleType::Rhino, team: 0 },
                     BigGoal::Epic, BigGoal::Serpen, BigGoal::Nexus { team: 0 },
                     BigGoal::Battle { focus: None }, BigGoal::Recall];
        let mut r: Vec<bool> = Vec::new();
        r.push(rs::line_exists(&c, LineType::Top));
        r.push(rs::line_exists(&c, LineType::Mid));
        r.push(rs::line_exists(&c, LineType::Bottom));
        r.push(rs::morgard_exists(&c));
        r.push(rs::serpen_exists(&c));
        for p in poss { r.push(rs::position_exists(&c, p)); }
        for g in goals { r.push(rs::goal_allowed(&c, g)); }
        print!("{}", ti);
        for v in r.iter() { print!("\t{}", if *v { 1 } else { 0 }); }
        println!();
        rows.push(r);
    }
    // 주장 집합과 대조
    let claim_line = [[0usize, 2, 7, 8].to_vec(), [0, 4, 5, 7, 8].to_vec(), [0, 1, 3, 5, 7, 8].to_vec()];
    let claim_morgard = [0usize, 7, 8].to_vec();
    let claim_serpen = [0usize, 5, 7, 8].to_vec();
    let claim_pos = [[0usize, 2, 7, 8].to_vec(), [0, 6, 8].to_vec(), [0, 4, 5, 7, 8].to_vec(),
                     [0, 1, 3, 5, 7, 8].to_vec(), [0, 1, 3, 5, 7, 8].to_vec()];
    let claim_goal = [[0usize, 2, 7, 8].to_vec(), [0, 4, 5, 7, 8].to_vec(), [0, 1, 3, 5, 7, 8].to_vec(),
                     [0usize, 6, 8].to_vec(), [0, 7, 8].to_vec(), [0, 5, 7, 8].to_vec(),
                     (0..9).collect::<Vec<_>>(), (0..9).collect::<Vec<_>>(), (0..9).collect::<Vec<_>>()];
    let mut names: Vec<String> = Vec::new();
    for n in ["lineTop", "lineMid", "lineBot"] { names.push(n.into()) }
    names.push("morgard".into()); names.push("serpen".into());
    for n in ["posTop", "posJng", "posMid", "posBot", "posSup"] { names.push(n.into()) }
    for n in ["gLineTop", "gLineMid", "gLineBot", "gJungle", "gEpic", "gSerpen", "gNexus", "gBattle", "gRecall"] { names.push(n.into()) }
    let mut claims: Vec<Vec<usize>> = Vec::new();
    for v in claim_line.iter() { claims.push(v.clone()) }
    claims.push(claim_morgard); claims.push(claim_serpen);
    for v in claim_pos.iter() { claims.push(v.clone()) }
    for v in claim_goal.iter() { claims.push(v.clone()) }
    let mut mm = 0usize; let mut nn = 0usize;
    println!("--- 주장 대조 (열 × tutorial)");
    for (ci, nm) in names.iter().enumerate() {
        let mut bad = Vec::new();
        for ti in 0..9 {
            let mine = claims[ci].contains(&ti);
            let got = rows[ti][ci];
            nn += 1;
            if mine == got { mm += 1 } else { bad.push(ti) }
        }
        println!("{}\tclaim={:?}\tgame={:?}\t{}", nm, claims[ci],
            (0..9).filter(|&t| rows[t][ci]).collect::<Vec<_>>(),
            if bad.is_empty() { "OK".to_string() } else { format!("***DIFF tut={:?}***", bad) });
    }
    println!("R1_MATCH\t{}/{}", mm, nn);

    // chat_allowed 표본
    println!("\n### R2 chat_allowed 표본 (tutorial 0..8)");
    println!("chat\t{}", (0..9).map(|i| i.to_string()).collect::<Vec<_>>().join("\t"));
    let chats: Vec<(&str, Chat)> = vec![
        ("MorgardGiveUp", Chat::MorgardGiveUp),
        ("Cancel(TargetMissing)", Chat::Cancel(CancelReason::TargetMissing)),
    ];
    for (nm, ch) in chats.iter() {
        print!("{}", nm);
        for &tut in TUTS.iter() {
            let c = mkctx(tut, TraceLevel::Off);
            print!("\t{}", if rs::chat_allowed(&c, ch) { 1 } else { 0 });
        }
        println!();
    }

    // ================= ② specs[11] v3_fall_back_to_passive (Moba) =================
    println!("\n### S11 v3_fall_back_to_passive — version 게이트 (Moba, tutorial=None)");
    println!("ver\tfallbacks\tmf0\tmf1\tplantag\tdiff_ranges");
    for ver in 0..7usize {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx0);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx0, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(ver, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let before = snap(&h);
        let r = catch_unwind(AssertUnwindSafe(|| {
            let mut rnd2 = rand::rngs::StdRng::seed_from_u64(12);
            h.v3_fall_back_to_passive(ver, &mut rnd2, player, &data, &mut dbg);
        }));
        let after = snap(&h);
        let dr = diff_ranges(&before, &after);
        let ptag = unsafe { *((&h as *const _ as *const u8).add(0x5e8) as *const u64) };
        println!("{}\t{}\t{}\t{}\t{}\t{}{}", ver, h.v3_lapse_passive_fallbacks,
            h.mf_swap.0, h.mf_swap.1, ptag, fmt_ranges(&dr),
            if r.is_err() { "  <PANIC>" } else { "" });
    }

    println!("\n### S11 tutorial 게이트 — version=3, tutorial 0..8 (허용되면 fallbacks=1)");
    println!("tut\tfallbacks\tmf0\tplantag\tplan_goal_tag\tdiff_ranges");
    for &tut in TUTS.iter() {
        let c = mkctx(tut, TraceLevel::Off);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &c);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &c, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let before = snap(&h);
        let r = catch_unwind(AssertUnwindSafe(|| {
            let mut rnd2 = rand::rngs::StdRng::seed_from_u64(12);
            h.v3_fall_back_to_passive(3usize, &mut rnd2, player, &data, &mut dbg);
        }));
        let after = snap(&h);
        let dr = diff_ranges(&before, &after);
        let ptag = unsafe { *((&h as *const _ as *const u8).add(0x5e8) as *const u64) };
        let g = h.plan.goal();
        let gtag = unsafe { *(&g as *const _ as *const u8) };
        println!("{:?}\t{}\t{}\t{}\t{}\t{}{}", tut, h.v3_lapse_passive_fallbacks, h.mf_swap.0,
            ptag, gtag, fmt_ranges(&dr), if r.is_err() { "  <PANIC>" } else { "" });
    }

    // ================= ③ 게임모드 게이트 (SingleLane / DeathMatch) =================
    println!("\n### S11 게임모드 게이트 — Moba / SingleLane / DeathMatch");
    println!("mode\tmodetag\tfallbacks\tmf0\tplantag\tplan+0x20\tplan+0x21\tdiff_ranges");
    let slg = SingleLaneGame::new(1234u64, false, &setting, &ms, &map);
    let dmg = DeathMatchGame::new(1234u64, false, &setting, &ms, &map);
    for (nm, g) in [("Moba", &game as &dyn AbstractGame), ("SingleLane", &slg as &dyn AbstractGame),
                    ("DeathMatch", &dmg as &dyn AbstractGame)].iter() {
        let mut cache = AbstractGameWithCache::new(*g, &ctx0);
        // 챔피언/PlayerState 를 Moba 것으로 이식(비-Moba 게임엔 선수가 없다)
        {
            let mc = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx0);
            // 이식은 수명 때문에 불가 — 대신 존재 여부만 출력
            let _ = mc.player_champion[0][0].is_some();
        }
        let modetag = unsafe { *(&g.get_game_mode() as *const _ as *const u64) };
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx0, &bb);
        let player = game.get_player_by_position(0usize, Position::Top).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let before = snap(&h);
        let r = catch_unwind(AssertUnwindSafe(|| {
            let mut rnd2 = rand::rngs::StdRng::seed_from_u64(12);
            h.v3_fall_back_to_passive(3usize, &mut rnd2, player, &data, &mut dbg);
        }));
        let after = snap(&h);
        let dr = diff_ranges(&before, &after);
        let base = &h as *const _ as *const u8;
        let ptag = unsafe { *(base.add(0x5e8) as *const u64) };
        let p20 = unsafe { *base.add(0x5e8 + 0x20) };
        let p21 = unsafe { *base.add(0x5e8 + 0x21) };
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}{}", nm, modetag, h.v3_lapse_passive_fallbacks,
            h.mf_swap.0, ptag, p20, p21, fmt_ranges(&dr), if r.is_err() { "  <PANIC>" } else { "" });
    }

    // ================= ④ specs[12] handle_chat 게이트 =================
    println!("\n### S12 handle_chat — 게이트 1(자기발화) / 2(position_exists) / 3(chat_allowed) / 트레이스");
    println!("tut\ttrace\tfrom\tselfpos\ttrace_ev\tdiff_ranges\tpred_pass");
    for &tut in TUTS.iter() {
        for tl in [TraceLevel::Off, TraceLevel::Summary] {
            for fromi in 0..5usize {
                let c = mkctx(tut, tl);
                let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &c);
                let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                let data = OperationData::new(&cache, &c, &bb);
                let player = game.get_player_by_position(0usize, Position::Top).unwrap();
                let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
                let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
                let mut h = LegacyPlanHandler::new(3usize, &mut rnd, 0usize, Position::Top);
                let mut dbg: DebugFrameData = Default::default();
                let before = snap(&h);
                let r = catch_unwind(AssertUnwindSafe(|| {
                    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(12);
                    h.handle_chat(3usize, &mut rnd2, player, &data, poss[fromi],
                                  Chat::MorgardGiveUp, false, &mut dbg);
                }));
                let after = snap(&h);
                let dr = diff_ranges(&before, &after);
                // 예측: from != self(Top=0) && position_exists && chat_allowed
                let selfpos = 0usize;
                let pe = rs::position_exists(&c, poss[fromi]);
                let ca = rs::chat_allowed(&c, &Chat::MorgardGiveUp);
                let pred = fromi != selfpos && pe && ca;
                let nev = h.pending_trace_events.len();
                if fromi <= 1 || pred {
                    println!("{:?}\t{:?}\t{:?}\tTop\t{}\t{}\t{}{}", tut, tl, poss[fromi], nev,
                        fmt_ranges(&dr), pred, if r.is_err() { "  <PANIC>" } else { "" });
                }
            }
        }
    }
    println!("\nDONE");
}
