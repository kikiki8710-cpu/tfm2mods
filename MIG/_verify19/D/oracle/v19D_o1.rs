#![allow(unused, dead_code, non_snake_case)]
//! 19차 배치 D 오라클 — #100 TeamPlan::update_steal(&mut self 바이트 diff, 수법 ⓒ) · #99 should_steal_now (pub).
//! TEMPLATE.rs 기반(real_setting/mkgame). 본문 4함수에 TLS 접근 0건(awk 실측)이나 콜리(expected_damage_target 등)는
//! 미확인이라 **케이스당 프로세스 1개**(argv[1]=case).
//!
//! 케이스
//!  1  Top 플레이어(비정글) → L733 게이트: 0x418 ← 255 만 쓰고 return (prev 0x41a 불변)
//!  2  Jungle · tick 1000 · 세션 None · prev=Some(Commit(Serpen)) · plan=ForcePassive · 에픽 없음
//!       → new_action=None(0): 0x418..0x41b ← 0,0,0,0 (Some(None) 이지 None(255) 이 아님)
//!  3  =2 + 가짜 세션(0x1ba=1 Some, target Epic, start/end_tick 500) + snapshot None + 0x128 preset 77
//!       → 세션 종료: 0x1ba←2, 0x3e0[0]←1000, 0x128←0, completed_steal_sessions push(outcome EnemyKilled=1 · 에픽 없음)
//!  4  =3 + snapshot Some((Epic,0)) + mode.epic_minion_buff_time[team]=100 → success: outcome Success(0), 0x3c0 +1
//!  5  =3 + 에픽 실제 스폰(next_respawn_tick=1 → run_tick) → target_alive → outcome EligibilityLost(3)
//!  6  =5 + 세션 없음 + prev None(255) + goal_data.epic {enemy_tick=600,last_seen=tick-10,last_hp=epic.hp} · 에픽 hp 50%
//!       → should_steal_now 가 evaluate 까지 진입 (default 챔프 my_damage=0 → L459 None 예상) · 반환 None
//!  7  =2 + plan=Battle 은 페이로드 구성 불가라 생략(ForcePassive 만). prev None(255)+new None → entries 미증가 확인은 6 에 포함
use game_core::*;
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::plan_legacy::types::BigPlan;
use game_ai::GoalData;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30;
    s.respawn_growth_term = 1800; s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20; s.support_gold_reduction = 15;
    s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
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

// ★함정 ⑦: 포인터 인자 + volatile — 공유참조로 받아 store 하면 LLVM 이 지운다.
unsafe fn w8(p: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(p.add(off), v); }
unsafe fn w64(p: *mut u8, off: usize, v: u64) { std::ptr::write_volatile(p.add(off) as *mut u64, v); }
unsafe fn r8(p: *const u8, off: usize) -> u8 { std::ptr::read_volatile(p.add(off)) }
unsafe fn r64(p: *const u8, off: usize) -> u64 { std::ptr::read_volatile(p.add(off) as *const u64) }
fn snap(p: *const u8, n: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(p, n).to_vec() } }
fn diff(a: &[u8], b: &[u8]) -> Vec<(usize, u8, u8)> {
    (0..a.len()).filter(|&i| a[i] != b[i]).map(|i| (i, a[i], b[i])).collect()
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let team = 0usize;
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(9);
    if case >= 5 {
        game.mode.jungle_runner.epic.next_respawn_tick = 1;
        let mut spawned_at = 0usize;
        for k in 0..3000usize {
            let mut fd: Option<&mut GameFrameData> = None;
            game.run_tick(&ctx, &mut rnd2, &mut fd);
            if !game.mode.jungle_runner.epic.live_list.is_empty() { spawned_at = k + 1; break; }
        }
        println!("epic_spawned_after_ticks\t{}\tlive_list={:?}", spawned_at, game.mode.jungle_runner.epic.live_list);
    }
    if case == 4 { game.mode.epic_minion_buff_time[team] = 100; }
    let tick0 = game.tick();
    let tick = if tick0 < 1000 { 1000 } else { tick0 + 1000 };
    game.set_tick(tick);
    println!("tick\t{}\ttowers={}\tepic_live={}\tserpen_live={}\tbuff_time={:?}\tserpen_count={:?}",
             game.tick(), game.world.tower_ids.len(),
             game.mode.jungle_runner.epic.live_list.len(), game.mode.jungle_runner.serpen.live_list.len(),
             game.mode.epic_minion_buff_time, game.mode.serpen_count);

    // 에픽 hp 를 50% 로 (case 6) — 게임 내부 엔티티에 raw write
    if case == 6 {
        let id = game.mode.jungle_runner.epic.live_list[0];
        let e = game.get_entity_by_id(id).expect("epic entity");
        let ep = e as *const Entity as *mut u8;
        unsafe {
            let maxhp = r64(ep, 0x628);
            let hp = r64(ep, 0x670);
            w64(ep, 0x670, maxhp / 2);
            println!("epic\tid={}\tmax_hp={}\thp {}→{}\tx={}\ty={}", id, maxhp, hp, r64(ep, 0x670), r64(ep, 0x660), r64(ep, 0x668));
        }
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let pos = if case == 1 { Position::Top } else { Position::Jungle };
    let ps = game.get_player_by_position(team, pos).expect("player");
    let mut goal: GoalData = Default::default();
    let mut tp: TeamPlan = Default::default();
    let tpp = &mut tp as *mut TeamPlan as *mut u8;
    unsafe {
        println!("default\t0x418={}\t0x41a={}\t0x1ba={}\t0x120={}\t0xf0(cap)={}\t0x100(len)={}\t0xc0(cap)={}\t0xd0(len)={}",
                 r8(tpp, 0x418), r8(tpp, 0x41a), r8(tpp, 0x1ba), r8(tpp, 0x120), r64(tpp, 0xf0), r64(tpp, 0x100), r64(tpp, 0xc0), r64(tpp, 0xd0));
        // preset
        if case == 1 { w8(tpp, 0x418, 1); w8(tpp, 0x419, 0); w8(tpp, 0x41a, 2); w8(tpp, 0x41b, 1); }
        if case >= 2 && case <= 5 { w8(tpp, 0x418, 1); w8(tpp, 0x419, 1); w8(tpp, 0x41a, 2); w8(tpp, 0x41b, 1); }
        if case >= 3 && case <= 5 {
            w8(tpp, 0x1ba, 1); w8(tpp, 0x1b9, 0); w8(tpp, 0x1b8, 3);
            w64(tpp, 0x150, 500); w64(tpp, 0x158, 500);
            w64(tpp, 0x160, 1234); w64(tpp, 0x168, 1234); w64(tpp, 0x170, 1234);
            w64(tpp, 0x128, 77);
        }
        if case == 4 { w8(tpp, 0x120, 0); w64(tpp, 0x128, 0); }
        if case == 6 {
            let e = game.get_entity_by_id(game.mode.jungle_runner.epic.live_list[0]).unwrap();
            goal.epic.epic_enemy_tick = 600;
            goal.epic.last_epic_seen = tick - 10;
            goal.epic.last_epic_hp = e.hp;
        }
    }
    let before = snap(tpp as *const u8, 1064);
    // ── should_steal_now 직접 (pub) ──
    let act = game_ai::plan_legacy::steal::should_steal_now(4, ps, &goal, &tp, &data);
    println!("should_steal_now\t{:?}", act);
    let mid = snap(tpp as *const u8, 1064);
    println!("ssn_self_diff\t{}", diff(&before, &mid).len());
    // ── update_steal (&mut self) ──
    let plan = BigPlan::ForcePassive;
    tp.update_steal(4, ps, &data, &goal, &plan);
    let after = snap(tpp as *const u8, 1064);
    let d = diff(&before, &after);
    println!("update_steal_diff_bytes\t{}", d.len());
    // 8B 단위로 묶어 보기 좋게
    let mut i = 0;
    while i < d.len() {
        let (off, a, b) = d[i];
        let base = off & !7;
        let mut j = i;
        while j < d.len() && (d[j].0 & !7) == base { j += 1; }
        let bs: Vec<String> = d[i..j].iter().map(|(o, a, b)| format!("+{:x}:{}→{}", o, a, b)).collect();
        unsafe { println!("  qword 0x{:x}\tnow={}\t{}", base, r64(tpp, base), bs.join(" ")); }
        i = j;
    }
    unsafe {
        println!("after\t0x418={}\t0x419={}\t0x41a={}\t0x41b={}\t0x1ba={}\t0x1b9={}\t0x1b8={}\t0x120={}\t0x128={}\t0x3e0[0]={}\t0x3c0={}\t0x3b8={}\t0x100(len)={}\t0xf0(cap)={}\t0xd0(chats.len)={}",
                 r8(tpp, 0x418), r8(tpp, 0x419), r8(tpp, 0x41a), r8(tpp, 0x41b), r8(tpp, 0x1ba), r8(tpp, 0x1b9), r8(tpp, 0x1b8),
                 r8(tpp, 0x120), r64(tpp, 0x128), r64(tpp, 0x3e0), r64(tpp, 0x3c0), r64(tpp, 0x3b8), r64(tpp, 0x100), r64(tpp, 0xf0), r64(tpp, 0xd0));
    }
    let dbg = format!("{:?}", tp);
    // completed_steal_sessions / steal_action 부분만
    for key in ["completed_steal_sessions", "steal_action", "prev_steal_action", "current_steal_session", "steal_commit_snapshot", "last_steal_session_end_tick", "epic_steal_success_count", "epic_steal_attempt_count"] {
        if let Some(p) = dbg.find(key) {
            let s = &dbg[p..];
            let end = s.find(", ").map(|x| x.min(400)).unwrap_or(s.len().min(400));
            // completed_steal_sessions 는 길다 — 400자
            let cut = if key == "completed_steal_sessions" { s.len().min(600) } else { end };
            println!("dbg\t{}", &s[..cut]);
        }
    }
    for (k, _) in dbg.match_indices("outcome:") { println!("dbg_outcome	{}", &dbg[k..(k+40).min(dbg.len())]); }
    for (k, _) in dbg.match_indices("i_attacked_target_ticks:") { println!("dbg_iatt	{}", &dbg[k..(k+60).min(dbg.len())]); }
    std::mem::forget(tp); // 가짜 세션 preset 은 Vec 등 소유물이 아니라 drop 안전이나, 보수적으로 누수
}
