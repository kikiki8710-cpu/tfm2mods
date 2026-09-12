#![allow(unused, dead_code, non_snake_case)]
//! 4차 반증검증 배치 B — 오라클: 09 의 미니언 게이트 (minion_wave_risk.rs 130~231)
//! 목표
//!  ① BRIEF §1⑥ 의 `minion_wave_setting`=0 함정을 우회해 **실제로 미니언을 스폰**시킨다
//!  ② `enemy_minion_line_action_damage_at` 의 damage>0 을 얻어 IR 독해(1차 재구성)를 실행으로 검증
//!  ③ ★`epic_minion_buff_time[적팀]` 이 **모델 자체를 바꾸는지**(wave_risk 로 위임) 확인
//!  ④ ★`champion_action=true` 인 09 경로에서 buff 값이 `..._danger_...` 의 표 선택에 영향이 없는지
//!  ⑤ 반환 상한 `damage.min(max(target.hp*2,1))`
//! ⚠ BRIEF §3① — init_tower/init_nexus 재호출 금지 (TEMPLATE 그대로)
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
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        println!("towers\t{}\ttwin0={}\ttwin1={}",
                 game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
        let mut top = 0usize;
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                if game_core::is_top_side(&ctx, e.x, e.y) { top += 1; }
            }
        }}
        println!("is_top_side_true\t{}/10\t(height=0 이면 10/10 로 붕괴한다)", top);
        println!("expect\ttowers=16 twin=2/2 is_top_side_true=8/10");
    }

    // ── 미니언 스폰까지 시뮬 ───────────────────────────────────────────
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    for step in 0..600usize {
        let mut fd: Option<&mut GameFrameData> = None;
        game.run_tick(&ctx, &mut rnd, &mut fd);
        if step == 99 || step == 299 || step == 599 {
            println!("TICKS\t{}\tminions={}", step + 1, game.world.minion_ids.len());
        }
    }
    let nm = game.world.minion_ids.len();
    println!("MINION_TOTAL\t{}", nm);
    if nm == 0 { println!("!! 미니언 0 — minion_wave_setting 주입 실패"); return; }

    // 미니언 목록 덤프 (팀·좌표·line·nearest_enemy·attack_effect 유무)
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for t in 0..2usize {
            let mut n = 0;
            for m in cache.iter_minions(t) {
                if n < 6 {
                    println!("MINION\tteam={}\tid={}\tx={}\ty={}\thp={}\tmax={}\tmspd={}\teff={}",
                             t, m.id, m.x, m.y, m.hp, m.stat_cached.hp, m.stat_cached.move_speed,
                             m.attack_effect.is_some());
                }
                n += 1;
            }
            println!("MINION_COUNT\tteam={}\t{}", t, n);
        }
    }

    // ── 고HP 대역 제어군 (반환 상한에 안 물리게) ──
    let base = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0].unwrap().clone()
    };
    println!("BASE\thp={}\tmax={}\tx={}\ty={}", base.hp, base.stat_cached.hp, base.x, base.y);

    let probe_pts: Vec<(u64, u64)> = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut v = Vec::new();
        for m in cache.iter_minions(1) { v.push((m.x, m.y)); if v.len() >= 4 { break; } }
        v
    };
    println!("PROBE_PTS\t{:?}", probe_pts);
    let tps = setting.tick_per_second;
    const BIG: usize = 1_000_000;

    // (1) damage_at / danger / wave_risk
    for (i, &(px, py)) in probe_pts.iter().enumerate() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let d_raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
        let d_dng = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
        let d_wav = game_ai::enemy_minion_wave_risk_damage_at(0, &data, &ce, px, py, tps * 2);
        println!("PT{}\t({},{})\tdamage_at={}\tdanger={}\twave_risk={}", i, px, py, d_raw, d_dng, d_wav);
    }

    // (2) epic_minion_buff_time != 0 -> damage_at delegates to wave_risk ?
    if let Some(&(px, py)) = probe_pts.first() {
        for buff in [0usize, 1usize, 600usize] {
            game.mode.epic_minion_buff_time = [buff, buff];
            let mut ce = base.clone();
            ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let d_raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_wav = game_ai::enemy_minion_wave_risk_damage_at(0, &data, &ce, px, py, tps * 2);
            let t_t = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let t_f = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, false, false);
            println!("BUFF\t{}\tdamage_at={}\twave_risk={}\tequal={}\tdangerT={}\tdangerF={}",
                     buff, d_raw, d_wav, d_raw == d_wav, t_t, t_f);
        }
        game.mode.epic_minion_buff_time = [0, 0];
    }

    // (3) window_tick clamp
    if let Some(&(px, py)) = probe_pts.first() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        for w in [0usize, 1, 15, 29, 30, 31, 45, 59, 60, 61, 120, 600] {
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, w, true, false);
            println!("WIN\tw={}\tdamage_at={}", w, d);
        }
    }

    // (4) champion_action / predict_retarget 4 combos
    if let Some(&(px, py)) = probe_pts.first() {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        for ca in [false, true] { for pr in [false, true] {
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, ca, pr);
            println!("FLAG\tca={}\tpr={}\tdamage_at={}", ca, pr, d);
        }}
    }

    // (5) return cap = damage.min(max(hp*2,1))
    if let Some(&(px, py)) = probe_pts.first() {
        let raw = {
            let mut ce = base.clone();
            ce.hp = BIG; ce.stat_cached.hp = BIG; ce.x = px; ce.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false)
        };
        println!("RAW\t{}", raw);
        for hp in [0usize, 1, 2, 3, 10, 50, 100, raw / 2, raw, raw * 2] {
            let mut ce = base.clone();
            ce.stat_cached.hp = BIG; ce.x = px; ce.y = py; ce.hp = hp;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let d = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let cap = std::cmp::max(hp.saturating_mul(2), 1);
            println!("CAP\thp={}\tcap={}\tdamage_at={}\tpred={}\tmatch={}",
                     hp, cap, d, std::cmp::min(raw, cap), d == std::cmp::min(raw, cap));
        }
    }

    // (6) danger/critical threshold tables
    if let Some(&(px, py)) = probe_pts.first() {
        let cases: [(usize, usize); 9] = [(BIG, BIG), (1000, 1000), (400, 1000), (300, 1000), (200, 1000),
                                          (100, 1000), (60, 1000), (30, 1000), (10, 1000)];
        for (hp, mx) in cases {
            let mut ce = base.clone();
            ce.x = px; ce.y = py; ce.hp = hp; ce.stat_cached.hp = mx;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_t = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let d_f = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, false, false);
            let hp_pct = hp * 100 / std::cmp::max(mx, 1);
            let dmg_pct = raw * 100 / std::cmp::max(hp, 1);
            let dg = raw != 0 && (raw >= hp || dmg_pct > 49 || (hp_pct < 66 && dmg_pct > 29)
                     || (hp_pct < 41 && dmg_pct > 17) || (hp_pct < 26 && dmg_pct > 9));
            let cr = raw != 0 && (raw >= hp || (hp_pct < 26 && dmg_pct > 34) || (hp_pct < 16 && dmg_pct > 19));
            let pdg = if dg { raw } else { 0 };
            let pcr = if cr { raw } else { 0 };
            println!("TBL\thp={}/{}\thp_pct={}\traw={}\tdmg_pct={}\tdangerT={}\tpredDG={}\tdangerF={}\tpredCR={}\tokT={}\tokF={}",
                     hp, mx, hp_pct, raw, dmg_pct, d_t, pdg, d_f, pcr, d_t == pdg, d_f == pcr);
        }
    }

    // (7) 09 body with enemy placed on a minion line
    {
        let mut ce = base.clone();
        ce.hp = BIG; ce.stat_cached.hp = BIG;
        let enemy_base = {
            let c2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            c2.player_champion[1][0].unwrap().clone()
        };
        for &(px, py) in probe_pts.iter() {
            let mut en = enemy_base.clone(); en.x = px; en.y = py;
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&ce);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Top).unwrap();
            let dmg = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
            let r = game_ai::check_favorable_engage_formation(0, ps, &data, &en, 200000);
            println!("FORM\tenemy=({},{})\tgate_dmg={}\tresult={}", px, py, dmg, r);
        }
    }
    // (8) buff x champion_action : 표 선택이 `or(ca, buff!=0)` 인가
    if let Some(&(px, py)) = probe_pts.first() {
        for buff in [0usize, 600usize] {
            game.mode.epic_minion_buff_time = [buff, buff];
            for (hp, mx) in [(400usize, 1000usize), (300usize, 1000usize)] {
                for ca in [false, true] {
                    let mut ce = base.clone();
                    ce.x = px; ce.y = py; ce.hp = hp; ce.stat_cached.hp = mx;
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    cache.player_champion[0][0] = Some(&ce);
                    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let raw = game_ai::enemy_minion_line_action_damage_at(0, &data, &ce, px, py, tps * 2, ca, false);
                    let d = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, ca, false);
                    let hp_pct = hp * 100 / mx;
                    let dmg_pct = raw * 100 / std::cmp::max(hp, 1);
                    let dg = raw != 0 && (raw >= hp || dmg_pct > 49 || (hp_pct < 66 && dmg_pct > 29)
                             || (hp_pct < 41 && dmg_pct > 17) || (hp_pct < 26 && dmg_pct > 9));
                    let cr = raw != 0 && (raw >= hp || (hp_pct < 26 && dmg_pct > 34) || (hp_pct < 16 && dmg_pct > 19));
                    let strict = ca || buff != 0;
                    let pred = if strict { if dg { raw } else { 0 } } else { if cr { raw } else { 0 } };
                    println!("OR\tbuff={}\tca={}\thp={}/{}\traw={}\tdmg_pct={}\tdanger={}\tpred={}\tmatch={}",
                             buff, ca, hp, mx, raw, dmg_pct, d, pred, d == pred);
                }
            }
        }
        game.mode.epic_minion_buff_time = [0, 0];
    }

    // (9) 09 본진 게이트 발화 : champ hp 를 낮추면 dmg!=0 -> 직행 false
    {
        let enemy_base = {
            let c2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            c2.player_champion[1][0].unwrap().clone()
        };
        if let Some(&(px, py)) = probe_pts.first() {
            for (hp, mx) in [(1_000_000usize, 1_000_000usize), (400usize, 1000usize), (100usize, 1000usize)] {
                let mut ce = base.clone();
                ce.hp = hp; ce.stat_cached.hp = mx;
                // 자기 위치도 적 옆에 둔다(front 전개 유도)
                ce.x = px; ce.y = py;
                let mut en = enemy_base.clone(); en.x = px; en.y = py;
                let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                cache.player_champion[0][0] = Some(&ce);
                let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                let data = OperationData::new(&cache, &ctx, &bb);
                let ps = game.get_player_by_position(0, Position::Top).unwrap();
                let dmg = game_ai::enemy_minion_line_action_danger_damage_at(0, &data, &ce, px, py, tps * 2, true, false);
                let r = game_ai::check_favorable_engage_formation(0, ps, &data, &en, 200000);
                println!("GATE\thp={}/{}\tgate_dmg={}\tformation={}", hp, mx, dmg, r);
            }
        }
    }
    println!("DONE\tsetting_ok={}", ok);
}
