#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #1 — `/specs[17]` `DeathMatchBattle::new`(death_battle.rs:746) 를 **실행**해서
//! mem/consts 표를 바이트 단위로 대조한다(ev4 → ev2).
//!
//! 왜 가능한가: `DeathMatchBattle` 이 `pub` + `derive(Debug)` 이고 `new`·`base_sub_goal`·
//! `BattlePlanGoal` variant 생성자가 전부 `pub`(tcx `_tcx\game_ai.json`).
//! private 필드(`region`/`flee_die`/…)도 **Debug 가 이름과 값을 찍는다** + 384B 원바이트 덤프로
//! 오프셋까지 직접 확인한다.
//!
//! TLS 메모(브리핑 함정③) 해당 없음 — `new` 는 `check_kill_die_tick` 경로를 타지 않는다.
//! 그래도 **호출 순서 역전 실행**으로 재현성을 같이 찍는다(징후 검사).
use game_core::*;
use game_ai::plan_legacy::old::{BattlePlanGoal, DeathMatchBattle};
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

fn hexdump(label: &str, p: *const u8, n: usize) {
    for row in 0..(n + 15) / 16 {
        let off = row * 16;
        let mut s = String::new();
        for i in 0..16 {
            if off + i < n { s.push_str(&format!("{:02x} ", unsafe { *p.add(off + i) })); }
            else { s.push_str("   "); }
        }
        println!("{}\t+0x{:03x}\t{}", label, off, s);
    }
}
unsafe fn rd_u64(p: *const u8, off: usize) -> u64 { (p.add(off) as *const u64).read_unaligned() }
unsafe fn rd_i64(p: *const u8, off: usize) -> i64 { (p.add(off) as *const i64).read_unaligned() }
unsafe fn rd_u8(p: *const u8, off: usize) -> u8 { *p.add(off) }

fn main() {
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    println!("towers\t{}\ttwin0={}\ttwin1={}",
             game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());

    let player = game.get_player_by_position(0, Position::Top).unwrap();
    println!("size_of_DeathMatchBattle\t{}", std::mem::size_of::<DeathMatchBattle>());
    println!("size_of_BattlePlanGoal\t{}", std::mem::size_of::<BattlePlanGoal>());

    // 게임의 현재 틱 — start_tick 기대값
    let gtick = (&game as &dyn AbstractGame).tick();
    println!("game_tick\t{}", gtick);

    // ── 케이스 A: TryKill(77, 88)  (유일한 분기의 참 쪽)
    let goalA = BattlePlanGoal::TryKill(77usize, 88usize);
    let subA = goalA.clone().base_sub_goal(1, player, &data);
    let bA = DeathMatchBattle::new(1usize, goalA.clone(), &data, player);
    println!("caseA_debug\t{:?}", bA);
    println!("caseA_base_sub_goal_direct\t{:?}", subA);
    println!("caseA_sub_goal_method\t{:?}", bA.sub_goal());
    let pA = &bA as *const DeathMatchBattle as *const u8;
    hexdump("caseA", pA, 384);
    unsafe {
        println!("A/0x000_support_target\t{}", rd_i64(pA, 0x000));
        println!("A/0x010_region_tag\t{}", rd_i64(pA, 0x010));
        println!("A/0x030_well_runaway\t{}", rd_i64(pA, 0x030));
        println!("A/0x040_death_focus\t{}", rd_i64(pA, 0x040));
        println!("A/0x050_seal_basis\t{}", rd_i64(pA, 0x050));
        println!("A/0x070_hold_scene_basis\t{}", rd_i64(pA, 0x070));
        println!("A/0x090_repo_scene_basis\t{}", rd_i64(pA, 0x090));
        println!("A/0x0a8_flee_dir\t{}", rd_i64(pA, 0x0a8));
        println!("A/0x0c0_far_noout_since\t{}", rd_i64(pA, 0x0c0));
        println!("A/0x0d0_main_goal_tag\t{}", rd_i64(pA, 0x0d0));
        println!("A/0x0d8_main_goal_p0\t{}", rd_u64(pA, 0x0d8));
        println!("A/0x0e0_main_goal_p1\t{}", rd_u64(pA, 0x0e0));
        println!("A/0x0e8_sub_goal_lo\t{}", rd_i64(pA, 0x0e8));
        println!("A/0x0f0_sub_goal_hi\t{}", rd_i64(pA, 0x0f0));
        println!("A/0x0f8_chats_w0\t{}", rd_u64(pA, 0x0f8));
        println!("A/0x100_chats_w1\t{}", rd_u64(pA, 0x100));
        println!("A/0x108_chats_w2\t{}", rd_u64(pA, 0x108));
        println!("A/0x110_start_tick\t{}", rd_i64(pA, 0x110));
        println!("A/0x118_flee_die\t{}", rd_i64(pA, 0x118));
        for (o, n) in [(0x120usize, "trade_lean"), (0x128, "lean_last_tick"), (0x130, "scene_change_tick"),
                       (0x138, "last_act_tick"), (0x140, "idle_spec_tick"), (0x148, "last_swing_tick"),
                       (0x150, "ep_follow_until"), (0x158, "idle_prev_pos.0"), (0x160, "idle_prev_pos.1"),
                       (0x168, "last_unseal_tick")] {
            println!("A/0x{:03x}_{}\t{}", o, n, rd_i64(pA, o));
        }
        for (o, n) in [(0x170usize, "with_dive"), (0x171, "help_called"), (0x172, "dive_abandoned"),
                       (0x173, "dodge_commit"), (0x174, "last_stand"), (0x175, "had_ult_ready"),
                       (0x176, "stance"), (0x177, "tactic"), (0x178, "scene"), (0x179, "scene_last_from"),
                       (0x17a, "dive_tower"), (0x17b, "main_objective"), (0x17c, "mo+1"), (0x17d, "mo+2"),
                       (0x17e, "lean_last_sign")] {
            println!("A/0x{:03x}_{}\t{}", o, n, rd_u8(pA, o));
        }
        // chats 내용 — Vec<Chat> 원소 1개를 가정하고 첫 원소 바이트를 찍는다
        let w0 = rd_u64(pA, 0x0f8) as usize;
        let w1 = rd_u64(pA, 0x100) as usize;
        let w2 = rd_u64(pA, 0x108) as usize;
        println!("A_chats_words\t{}\t{}\t{}\tsize_of_Chat={}", w0, w1, w2, std::mem::size_of::<Chat>());
        // 세 워드 중 8보다 크고 정렬된 것이 ptr 후보
        for (i, w) in [w0, w1, w2].iter().enumerate() {
            if *w > 0x10000 {
                let ep = *w as *const u8;
                let cn = std::mem::size_of::<Chat>();
                let mut s = String::new();
                for k in 0..cn { s.push_str(&format!("{:02x} ", *ep.add(k))); }
                println!("A_chat0_via_w{}\t{}", i, s);
            }
        }
    }

    // ── 케이스 B~D: 나머지 세 태그 (분기의 거짓 쪽)
    for (nm, g) in [("B_Support", BattlePlanGoal::Support(55usize)),
                    ("C_Response", BattlePlanGoal::Response),
                    ("D_Avoid", BattlePlanGoal::Avoid)] {
        let b = DeathMatchBattle::new(1usize, g.clone(), &data, player);
        let p = &b as *const DeathMatchBattle as *const u8;
        unsafe {
            println!("{}\tgoal_tag={}\tchats_w0={}\tchats_w1={}\tchats_w2={}\tstart_tick={}\tflee_die={}\tsub_lo={}\tsub_hi={}",
                nm, rd_i64(p, 0xd0), rd_u64(p, 0xf8), rd_u64(p, 0x100), rd_u64(p, 0x108),
                rd_i64(p, 0x110), rd_i64(p, 0x118), rd_i64(p, 0xe8), rd_i64(p, 0xf0));
        }
        println!("{}_debug\t{:?}", nm, b);
    }

    // ── version 축: 본문에 version 분기가 없다는 주장(sig.params[1]) 실측
    for v in [0usize, 1, 2, 3, 10, 50, 60] {
        let b = DeathMatchBattle::new(v, BattlePlanGoal::TryKill(77, 88), &data, player);
        let p = &b as *const DeathMatchBattle as *const u8;
        unsafe {
            println!("ver\t{}\tsub_lo={}\tsub_hi={}\tstart_tick={}\tflee_die={}\tstance={}\ttactic={}\tscene={}",
                v, rd_i64(p, 0xe8), rd_i64(p, 0xf0), rd_i64(p, 0x110), rd_i64(p, 0x118),
                rd_u8(p, 0x176), rd_u8(p, 0x177), rd_u8(p, 0x178));
        }
    }

    // ── player 축: 5 포지션 × 2 팀 (본문에서 player 를 안 읽는다는 주장)
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    for t in 0..2usize {
        for p5 in poss.iter() {
            if let Some(ps) = game.get_player_by_position(t, *p5) {
                let b = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(77, 88), &data, ps);
                let p = &b as *const DeathMatchBattle as *const u8;
                unsafe {
                    println!("plr\tt{}\t{:?}\tsub_lo={}\tsub_hi={}", t, p5, rd_i64(p, 0xe8), rd_i64(p, 0xf0));
                }
            }
        }
    }

    // ── TryKill 두 인자 축: __0(target) 만 chats 에 실리는가 / __1 은 안 쓰이는가
    for (a, b2) in [(0usize, 0usize), (1, 0), (77, 88), (88, 77), (12345, 0), (0, 12345)] {
        let bb2 = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(a, b2), &data, player);
        let p = &bb2 as *const DeathMatchBattle as *const u8;
        unsafe {
            let w = [rd_u64(p, 0xf8), rd_u64(p, 0x100), rd_u64(p, 0x108)];
            let mut chat = String::new();
            for wv in w.iter() {
                if *wv > 0x10000 {
                    let ep = *wv as *const u8;
                    for k in 0..std::mem::size_of::<Chat>() { chat.push_str(&format!("{:02x} ", *ep.add(k))); }
                    break;
                }
            }
            println!("trykill\t__0={}\t__1={}\tgoal_p0={}\tgoal_p1={}\tsub_lo={}\tsub_hi={}\tchat0=[{}]",
                a, b2, rd_u64(p, 0xd8), rd_u64(p, 0xe0), rd_i64(p, 0xe8), rd_i64(p, 0xf0), chat.trim());
        }
    }

    // ── 재현성(호출 순서 역전) — TLS 오염 징후 검사
    let r1 = DeathMatchBattle::new(1, BattlePlanGoal::Avoid, &data, player);
    let r2 = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(77, 88), &data, player);
    unsafe {
        let q1 = &r1 as *const DeathMatchBattle as *const u8;
        let q2 = &r2 as *const DeathMatchBattle as *const u8;
        println!("reorder\tavoid_sub=({},{})\ttrykill_sub=({},{})",
            rd_i64(q1, 0xe8), rd_i64(q1, 0xf0), rd_i64(q2, 0xe8), rd_i64(q2, 0xf0));
    }
    println!("DONE\tsetting_ok={}", ok);
}
