#![allow(unused, dead_code, non_snake_case)]
//! 22차 C · 125 should_suppress_direct_minion_attack 오라클(pub 직접 호출). 미니언 스폰 = B4_o09m 세팅 복제. 한 프로세스 = 한 케이스.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000;
    s.height = 960000;
    s.respawn_tick = 300;
    s.respawn_growth = 30;
    s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180;
    s.respawn_max = 2400;
    s.visible_distance = 130000;
    s.tick_per_second = 60;
    s.champion_radius = 10000;
    s.nexus_heal = 10;
    s.nexus_heal_2v2 = 10;
    s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100;
    s.kill_exp = 30;
    s.kill_exp_growth = 30;
    s.assist_exp_ratio = 40;
    s.kill_gold = 300;
    s.assist_gold = 100;
    s.start_gold = 500;
    s.gold_per_second = 7;
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400;
    s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200;
    s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600;
    s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700;
    s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15;
    s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    // ★BRIEF §1⑥ 우회 — 실전 minion_wave_setting (game_setting.game_setting 원문)
    s.minion_wave_setting.start_tick = 10;
    s.minion_wave_setting.tick_per_wave = 660;
    s.minion_wave_setting.melee_count = 2;
    s.minion_wave_setting.range_count = 1;
    s.minion_wave_setting.tick_per_spawn = 30;
    s.minion_wave_setting.growth_start_tick = 1800;
    s.minion_wave_setting.growth_tick = 1800;
    s.minion_wave_setting.growth_tick_2v2 = 400;
    s.minion_wave_setting.growth_tick_3v3 = 800;
    s.minion_wave_setting.growth_end_tick = 24000;
    s.minion_wave_setting.growth_end_tick_2v2 = 30000;
    s.minion_wave_setting.growth_end_tick_3v3 = 24000;
    s.minion_wave_setting.exp_range = 150000;
    s.minion_wave_setting.exp_decay2 = 100;
    s.minion_wave_setting.exp_decay3 = 80;
    s.minion_wave_setting.exp_decay4 = 60;
    // ★미니언 실전 스탯 (없으면 attack_effect 피해가 0 이라 게이트가 안 열린다)
    s.melee_minion.stat.attack = 10;
    s.melee_minion.stat.hp = 400;
    s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1;
    s.melee_minion.growth.hp = 30;
    s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100;
    s.melee_minion.attack.range = 3000;
    s.melee_minion.attack.cooltime = 30;
    s.melee_minion.attack.duration = 24;
    s.melee_minion.attack.start_timing = 16;
    s.melee_minion.exp = 40;
    s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15;
    s.range_minion.stat.hp = 250;
    s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1;
    s.range_minion.growth.hp = 20;
    s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000;
    s.range_minion.attack.speed = 3000;
    s.range_minion.attack.cooltime = 40;
    s.range_minion.attack.duration = 24;
    s.range_minion.attack.start_timing = 16;
    s.range_minion.exp = 30;
    s.range_minion.gold = 20;
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
            st.judgement = 80;
            st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}


fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    for _ in 0..600usize { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd, &mut fd); }
    println!("MINION_TOTAL\t{}", game.world.minion_ids.len());
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].unwrap();
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let cp = champ as *const Entity as *mut Entity;
    // 적(팀1) 미니언 중 Top 라인 첫 번째
    let line_of = |e: &Entity| -> u8 { unsafe { *((e as *const Entity as *const u8).add(0x11a)) } };
    let mut em: Option<&Entity> = None;
    for m in cache.iter_minions(1) { if line_of(m) == 0 { em = Some(m); break; } }
    let mut am: Option<&Entity> = None;
    for m in cache.iter_minions(0) { if line_of(m) == 0 { am = Some(m); break; } }
    let em = em.expect("no enemy top minion"); let am = am.expect("no ally top minion");
    let ec = cache.player_champion[1][0].unwrap();
    println!("em\tid={} x={} y={} line={} vis0={:?} team={:?} ty_tag={}", em.id, em.x, em.y, line_of(em), em.visible_state[0], em.team, unsafe{*((em as *const Entity as *const u8).add(0x68) as *const i64)});
    println!("champ\tid={} x={} y={} lvl={} range={} radius={} atk={:?}", champ.id, champ.x, champ.y, champ.level, champ.stat_buff_cached.range, champ.radius, champ.attack_effect.as_ref().map(|a| (a.range, a.growth_range, a.target)));
    let atk = champ.attack_effect.as_ref().unwrap();
    let radius = |e: &Entity| -> u64 { if e.stat_buff_cached.radius_mult == 0 { e.radius as u64 } else { (e.radius as i64 * (e.stat_buff_cached.radius_mult as i64 + 100) / 100) as u64 } };
    let r_full = champ.stat_buff_cached.range as u64 + atk.range + (champ.level as u64 - 1) * atk.growth_range + Effect::range_adjust(atk, champ, em) + radius(champ) + radius(em);
    println!("attack_range\t{}\tcheck={}", r_full, CastingTarget::check(&atk.target, champ, em));
    let emp = em as *const Entity as *mut Entity;
    if case != 1 && case != 2 && case != 8 { unsafe { std::ptr::write_volatile(&mut (*emp).visible_state[0], VisibleState::Visible); } }
    let (target, line): (&Entity, LineType) = unsafe { match case {
        0 => (em, LineType::Top),                                  // 그대로(멀리) → 콜리 3개 경로
        1 => (am, LineType::Top),                                  // 아군 미니언 → 팀 같음 → false
        2 => (ec, LineType::Top),                                  // 적 챔피언(비미니언) → false
        3 => (em, LineType::Mid),                                  // 라인 불일치 → false
        4 => { std::ptr::write_volatile(&mut (*cp).attack_effect, None); (em, LineType::Top) }   // attack_effect None → false
        5 => { std::ptr::write_volatile(&mut (*cp).x, em.x); std::ptr::write_volatile(&mut (*cp).y, em.y); (em, LineType::Top) }   // dist 0 → false
        6 => { std::ptr::write_volatile(&mut (*cp).x, em.x + r_full); std::ptr::write_volatile(&mut (*cp).y, em.y); (em, LineType::Top) }   // dist² == range² → false(ugt)
        7 => { std::ptr::write_volatile(&mut (*cp).x, em.x + r_full + 1); std::ptr::write_volatile(&mut (*cp).y, em.y); (em, LineType::Top) } // dist² = (range+1)² → 계속
        8 => { std::ptr::write_volatile(&mut (*emp).visible_state[0], VisibleState::Unknown); (em, LineType::Top) }   // 안 보임 → false
        9 => { std::ptr::write_volatile(&mut (*cp).x, em.x + r_full + 20000); std::ptr::write_volatile(&mut (*cp).y, em.y); (em, LineType::Top) }
        _ => (em, LineType::Top),
    } };
    let vis_now = unsafe { std::ptr::read_volatile(&(*emp).visible_state[0]) };
    let r = game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack(55, &data, player, line, target);
    let cx = unsafe { std::ptr::read_volatile(&(*cp).x) }; let cy = unsafe { std::ptr::read_volatile(&(*cp).y) };
    let d2 = cx.abs_diff(target.x).pow(2) + cy.abs_diff(target.y).pow(2);
    println!("RESULT125\tcase={}\tgame={}\tchamp=({},{})\tdist2={}\trange2={}\tvis={:?}", case, r, cx, cy, d2, r_full*r_full, vis_now);
}
