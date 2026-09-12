#![allow(unused, dead_code, non_snake_case)]
//! 4차 반증검증 배치 B — 07 / 08 의 ev2 표본 재확인 + 3차 미도달 경로 개방
//!  07: hp_ratio 50/51 경계 재확인 · ★Hide(태그9) 경로를 transmute 로 개방(3차 '미도달')
//!  08: 15*tps = 900 경계 재확인 (tick=0 · tick>0 둘 다)
//! ⚠ TEMPLATE 준수: 실전 GameSetting 44줄 · init_tower/init_nexus 재호출 금지
use game_core::*;
use game_core::JungleType;
use rand::SeedableRng;
use std::sync::Arc;
use game_ai::plan_legacy::old::{EpicHuntAndBattlePlan, EpicHuntAndPokePlan};
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};

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
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

fn tag_of(sp: &game_ai::plan_legacy::sub_plan::SubPlan) -> u64 {
    unsafe { *(sp as *const _ as *const u64) }
}
fn word_of(sp: &game_ai::plan_legacy::sub_plan::SubPlan, i: usize) -> u64 {
    unsafe { *((sp as *const _ as *const u64).add(i)) }
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0
                 && setting.champion_radius != 0,
             setting.width, setting.height, setting.tick_per_second,
             setting.champion_radius, setting.visible_distance);
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
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd0, &ctx);
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut top = 0usize;
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                if game_core::is_top_side(&ctx, e.x, e.y) { top += 1; }
            }
        }}
        println!("towers\t{}\ttwin0={}\ttwin1={}",
                 game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
        println!("is_top_side_true\t{}/10\t(height=0 이면 10/10 로 붕괴한다)", top);
        println!("expect\ttowers=16 twin=2/2 is_top_side_true=8/10");
    }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let tps = setting.tick_per_second;
    println!("FOUNTAIN\t0={:?}\t1={:?}", map.fountains[0], map.fountains[1]);
    println!("SZ\tEpicHuntAndBattlePlan={}\tSubPlan={}",
             std::mem::size_of::<EpicHuntAndBattlePlan>(),
             std::mem::size_of::<game_ai::plan_legacy::sub_plan::SubPlan>());

    // ══════════════ 07 ══════════════
    let (epic_id, base_champ) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c.player_champion[1][4].unwrap().id, c.player_champion[0][1].unwrap().clone())
    };
    let plan_none: EpicHuntAndBattlePlan = Default::default();     // target_bush = None
    // ★target_bush 가 private 이라 Default 로만 만들 수 있던 3차 제약을 transmute 로 우회.
    //   Option<usize> = {tag@+0, val@+8}, tag 1 = Some (IR m02.ll:49105 `trunc nuw i64 -> i1`).
    assert_eq!(std::mem::size_of::<EpicHuntAndBattlePlan>(), 16);
    let plan_bush: EpicHuntAndBattlePlan =
        unsafe { std::mem::transmute::<[usize; 2], EpicHuntAndBattlePlan>([1usize, 7usize]) };
    let gd: game_ai::GoalData = Default::default();
    let ins = map.fountains[0];
    let mut ok7 = 0usize; let mut bad7 = 0usize;

    // (hp, max, x, y, epic_full, live_ok, bush?, expected tag)
    let cases7: Vec<(&str, usize, usize, u64, u64, bool, bool, bool, u64)> = vec![
        ("ratio49 out epicfull  ",  49, 100, 500000, 500000, true,  true,  false, 5),
        ("ratio50 out epicfull  ",  50, 100, 500000, 500000, true,  true,  false, 5),
        ("ratio51 out epicfull  ",  51, 100, 500000, 500000, true,  true,  false, 11),
        ("ratio52 out epicfull  ",  52, 100, 500000, 500000, true,  true,  false, 11),
        ("ratio100 out epicfull ", 100, 100, 500000, 500000, true,  true,  false, 11),
        ("ratio51 IN  epicfull  ",  51, 100, ins.0 + 1, ins.1 + 1, true, true, false, 5),
        ("ratio100 IN epicfull  ", 100, 100, ins.0 + 1, ins.1 + 1, true, true, false, 11),
        ("ratio30 out epicHURT  ",  30, 100, 500000, 500000, false, true,  false, 11),
        ("ratio30 out epicNONE  ",  30, 100, 500000, 500000, true,  false, false, 11),
        // ★Hide(9) 경로 — bush=Some(7) + Recall 불성립(live_list 비움)
        ("bush + epicNONE      ",   30, 100, 500000, 500000, true,  false, true,  9),
        ("bush + epicfull r30  ",   30, 100, 500000, 500000, true,  true,  true,  5),
        ("bush + epicfull r100 ",  100, 100, 500000, 500000, true,  true,  true,  9),
    ];
    for (name, hp, mx, x, y, epic_full, live_ok, bush, exp) in cases7 {
        {
            let g: &mut dyn AbstractGame = &mut game;
            let e = g.get_entity_by_id_mut(epic_id).unwrap();
            let m = e.stat_cached.hp;
            e.hp = if epic_full { m } else { m - 1 };
        }
        game.mode.jungle_runner.epic.live_list = if live_ok { vec![epic_id] } else { Vec::new() };
        let mut ce = base_champ.clone();
        ce.hp = hp; ce.stat_cached.hp = mx; ce.x = x; ce.y = y;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][1] = Some(&ce);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbgf: DebugFrameData = Default::default();
        let p: &EpicHuntAndBattlePlan = if bush { &plan_bush } else { &plan_none };
        let sp = p.sub_plan(3, &mut rnd, ps, &data, &gd, &mut dbgf);
        let t = tag_of(&sp);
        let w1 = word_of(&sp, 1);
        let mark = if t == exp { ok7 += 1; "OK  " } else { bad7 += 1; "MISMATCH" };
        println!("{}\t07 {}\tratio={}\ttag={}\texp={}\tw1={}", mark, name, hp * 100 / mx, t, exp, w1);
    }
    // Hide 페이로드 상세 (bush id / out_line / check_move / enemy_spotted_me)
    {
        game.mode.jungle_runner.epic.live_list = Vec::new();
        let mut ce = base_champ.clone();
        ce.hp = 30; ce.stat_cached.hp = 100; ce.x = 500000; ce.y = 500000;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][1] = Some(&ce);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbgf: DebugFrameData = Default::default();
        let sp = plan_bush.sub_plan(3, &mut rnd, ps, &data, &gd, &mut dbgf);
        let raw: [u8; 24] = unsafe { *(&sp as *const _ as *const [u8; 24]) };
        println!("HIDE_RAW\ttag={}\tbush={}\tout_line={}\tcheck_move={}\tspotted={}\tbytes={:?}",
                 tag_of(&sp), word_of(&sp, 1), raw[16], raw[17], raw[18], raw);
        println!("HIDE_DBG\t{:?}", sp);
    }

    // ══════════════ 08 ══════════════
    let plan8 = EpicHuntAndPokePlan {
        v46_flee_threats: Vec::new(),
        focus_epic_only: false, vision_only: false, v46_flee: false,
    };
    let mut ok8 = 0usize; let mut bad8 = 0usize;
    for (label, extra_ticks) in [("tick0", 0usize), ("tick+120", 120usize)] {
        if extra_ticks > 0 {
            let mut r = rand::rngs::StdRng::seed_from_u64(11);
            for _ in 0..extra_ticks {
                let mut fd: Option<&mut GameFrameData> = None;
                game.run_tick(&ctx, &mut r, &mut fd);
            }
        }
        let now = (&game as &dyn AbstractGame).tick();
        for d in [0usize, 1, 899, 900, 901, 5000] {
            let nrt = now + d;
            game.mode.jungle_runner.epic.live_list = Vec::new();
            game.mode.jungle_runner.epic.next_respawn_tick = nrt;
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
            let mut tp: TeamPlan = Default::default();
            tp.objective = Some(MainObjective::Morgard { phase: ObjectPhase::None, with_battle: false });
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbgf: DebugFrameData = Default::default();
            let got = plan8.is_end(3, &mut r, ps, &data, &tp, &mut dbgf);
            let exp = d > 15 * tps;
            let mark = if got == exp { ok8 += 1; "OK  " } else { bad8 += 1; "MISMATCH" };
            println!("{}\t08 {}\ttick={}\tnrt={}\tremain={}\tgot={}\texp={}", mark, label, now, nrt, d, got, exp);
        }
    }
    println!("RESULT\tok7={}\tbad7={}\tok8={}\tbad8={}", ok7, bad7, ok8, bad8);
}
