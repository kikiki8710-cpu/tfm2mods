#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치 B 오라클 — `LegacyPlanHandler::get_small_action`(specs[205], pub) 직접 진입.
//! 케이스당 프로세스 1개(TLS 메모 콜리: calculate_score_parameter → CHAMP_POWERS_MEMO 등). `o26B.exe <case>`.
//!  c0  version=2 · Moba · 기본(update 1회 후 get_small_action) — 정상 경로 · self 6168B 바이트 diff
//!  c1  version=1 — L29 블록(회두홀드·생존규칙·0x1809 store) 비실행 확인
//!  c2  version=2 · champ.hp=0 (write_volatile) — L50 생존 절대규칙 → (99999, RunAway) 선반환
//!  c3  version=2 · self+0x530 에 pending_global_ult_target=Some((적id, 999999)) 주입 — L61 can_ult 실패 → L88 소거(0x530←0)
//!  c4  c0 + ignore_action=[(0, <c0 가 고른 SmallAction>)] — 동률 없으면 L322 경로로 같은 액션
//!  c5  c0 + ctx.debug=true — debug.infos[champ.id].len()==with_score.len() · logs
//!  c6  c0 를 같은 프로세스에서 2회 — judge_noise_ratio(0x17a0) 는 2회째 재롤 없음(플랜 판별자 동일)
//!  c7  version=2 · update 없이 new() 직후 호출 — 초기 sub_plan 에서 후보 0 이면 println+패닉(L261/L291) 관측
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::SmallActionPlay;
use rand::SeedableRng;
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
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}", ok, s.width, s.height, s.tick_per_second, s.champion_radius);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    let n = std::mem::size_of::<LegacyPlanHandler>();
    let p = h as *const LegacyPlanHandler as *const u8;
    let mut v = vec![0u8; n];
    unsafe { std::ptr::copy_nonoverlapping(p, v.as_mut_ptr(), n); }
    v
}
fn diff(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            let s = i;
            while i < a.len() && a[i] != b[i] { i += 1; }
            out.push((s, i));
        } else { i += 1; }
    }
    out
}
fn rd_i64(v: &[u8], o: usize) -> i64 { i64::from_le_bytes(v[o..o+8].try_into().unwrap()) }
fn rd_u8(v: &[u8], o: usize) -> u8 { v[o] }

fn main() {
    let case = std::env::args().nth(1).unwrap_or("c0".into());
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let dbg_on = case == "c5";
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: dbg_on,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    println!("towers\t{}\ttick={}", game.world.tower_ids.len(), cache.game.tick());
    let team = 0usize; let pos = Position::Top;
    let me = game.get_player_by_position(team, pos).unwrap();
    let champ = cache.player_champion[team][pos as usize].unwrap();
    println!("champ\tid={}\thp={}\tmax_hp={}\tlevel={}\tx={}\ty={}", champ.id, champ.hp, champ.stat_cached.hp, champ.level, champ.x, champ.y);
    println!("size_of_LegacyPlanHandler\t{}\tsize_of_ret\t{}", std::mem::size_of::<LegacyPlanHandler>(),
             std::mem::size_of::<(game_ai::ScoreParameter, i64, SmallActionPlay)>());

    let version: usize = if case == "c1" { 1 } else { 2 };
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let mut h = LegacyPlanHandler::new(version, &mut rnd, team, pos);
    let mut dbg: DebugFrameData = Default::default();
    if case != "c7" {
        h.update(version, &mut rnd, me, &data, &mut dbg, false);
    }
    let pd = format!("{:?}", h.plan); let sd = format!("{:?}", h.sub_plan);
    println!("plan\t{}\nsub_plan\t{}", &pd[..pd.len().min(160)], &sd[..sd.len().min(160)]);

    if case == "c2" {
        unsafe {
            let p = champ as *const Entity as *mut Entity;
            std::ptr::write_volatile(&mut (*p).hp, 0usize);
        }
        println!("champ.hp_after_write\t{}", unsafe { std::ptr::read_volatile(&champ.hp) });
    }
    if case == "c3" {
        let tid = cache.player_champion[1][0].unwrap().id;
        unsafe {
            let p = (&mut h as *mut LegacyPlanHandler as *mut u8);
            std::ptr::write_volatile(p.add(0x530) as *mut i64, 1);
            std::ptr::write_volatile(p.add(0x538) as *mut usize, tid);
            std::ptr::write_volatile(p.add(0x540) as *mut usize, 999999);
        }
        println!("inject_pending_global_ult\ttid={}\tcan_ult={}\tlevel={}", tid, champ.can_ult(), champ.level);
    }

    let pre = SmallActionPlay::Stop;
    let mut ignore: Vec<(usize, SmallAction)> = Vec::new();
    let before = snap(&h);
    let rnd_before = { let p = &rnd as *const _ as *const u8; let mut v = vec![0u8; 320]; unsafe { std::ptr::copy_nonoverlapping(p, v.as_mut_ptr(), 320); } v };
    let (param, score, play) = h.get_small_action(version, &mut rnd, me, &data, &pre, &ignore, &mut dbg);
    let after = snap(&h);
    let rnd_after = { let p = &rnd as *const _ as *const u8; let mut v = vec![0u8; 320]; unsafe { std::ptr::copy_nonoverlapping(p, v.as_mut_ptr(), 320); } v };
    println!("RESULT\tscore={}\taction={:?}\tis_ult_escape={}", score, play.get_action(), play.is_ult_escape());
    println!("rnd_changed\t{}", rnd_before != rnd_after);
    let d = diff(&before, &after);
    println!("self_diff_ranges\t{}", d.iter().map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect::<Vec<_>>().join(" "));
    // 핵심 필드 판독
    println!("f/0x530 pending_global_ult tag\t{} -> {}", rd_i64(&before, 0x530), rd_i64(&after, 0x530));
    println!("f/0x560 judge_noise_plan tag\t{} -> {}\t0x568 val {} -> {}", rd_i64(&before, 0x560), rd_i64(&after, 0x560), rd_i64(&before, 0x568), rd_i64(&after, 0x568));
    let ratios: Vec<i64> = (0..11).map(|i| rd_i64(&after, 0x17a0 + 8 * i)).collect();
    println!("f/0x17a0 judge_noise_ratio\t{:?}", ratios);
    println!("f/0x1804 v3_last_stand {}->{}\t0x1805 v3_final_stand {}->{}\t0x1809 turnback_prev {}->{}\t0x1812 cand_src {}->{}\t0x1813 bail_goal {}->{}",
        rd_u8(&before,0x1804), rd_u8(&after,0x1804), rd_u8(&before,0x1805), rd_u8(&after,0x1805), rd_u8(&before,0x1809), rd_u8(&after,0x1809),
        rd_u8(&before,0x1812), rd_u8(&after,0x1812), rd_u8(&before,0x1813), rd_u8(&after,0x1813));
    println!("f/0x1540 step_picks {}->{}\t0x1548 flee_picks {}->{}", rd_i64(&before,0x1540), rd_i64(&after,0x1540), rd_i64(&before,0x1548), rd_i64(&after,0x1548));
    println!("f/0x868 pending_trace_events.len {}->{}", rd_i64(&before,0x868), rd_i64(&after,0x868));
    let ps_eq = &after[0x990..0x990+2760] == unsafe { std::slice::from_raw_parts((&param as *const game_ai::ScoreParameter as *const u8).add(0x9f0), 2760) };
    println!("f/0x990 positioning_score == param.positioning_score(+0x9f0)\t{}", ps_eq);
    println!("param/v3_turnback_hold(+0x1500)\t{}\twave_snapshot tag(+0)\t{}", unsafe { *((&param as *const _ as *const u8).add(0x1500)) }, unsafe { *((&param as *const _ as *const i64)) });
    println!("debug.infos\t{}\tdebug.logs\t{}", dbg.infos.len(), dbg.logs.len());
    if dbg_on {
        for (k, v) in dbg.infos.iter() { println!("infos[{}]\tn={}\t{:?}", k, v.len(), v); }
    }
    let ret_play_tag = unsafe { *((&play as *const SmallActionPlay as *const u8).add(0xb1)) };
    println!("ret_play_tag(+0xb1)\t{}", ret_play_tag);

    if case == "c4" {
        // c0 와 같은 세계·같은 rnd 로 다시 만들어 ignore 를 넣는다(프로세스는 1개지만 self/rnd 를 새로 만든다)
        let mut rnd2 = rand::rngs::StdRng::seed_from_u64(11);
        let mut h2 = LegacyPlanHandler::new(version, &mut rnd2, team, pos);
        let mut dbg2: DebugFrameData = Default::default();
        h2.update(version, &mut rnd2, me, &data, &mut dbg2, false);
        let ig = vec![(0usize, play.get_action())];
        let (_p2, score2, play2) = h2.get_small_action(version, &mut rnd2, me, &data, &pre, &ig, &mut dbg2);
        println!("C4\tignore={:?}\tscore2={}\taction2={:?}\tsame={}", ig[0].1, score2, play2.get_action(), play2.get_action() == play.get_action());
    }
    if case == "c6" {
        let before2 = snap(&h);
        let (_p2, score2, play2) = h.get_small_action(version, &mut rnd, me, &data, &pre, &ignore, &mut dbg);
        let after2 = snap(&h);
        let d2 = diff(&before2, &after2);
        let ratios2: Vec<i64> = (0..11).map(|i| rd_i64(&after2, 0x17a0 + 8 * i)).collect();
        println!("C6\tscore2={}\taction2={:?}\tdiff2={}\tratio_same={}", score2, play2.get_action(),
            d2.iter().map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect::<Vec<_>>().join(" "), ratios == ratios2);
    }
    if std::env::args().count() > 99 { let _ = game_ai::calculate_score_parameter as *const (); }
}
