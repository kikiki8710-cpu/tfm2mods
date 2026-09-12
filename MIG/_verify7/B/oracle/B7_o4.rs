#![allow(unused, dead_code, non_snake_case)]
//! B7_o4 — 7차 배치B 오라클 ②: **09 의 미니언 위험 조회 창(window) 계수가 정말 `tick_per_second × 2` 인가**
//!
//! ## 무엇을 내리려 하나 (ev4 → ev2)
//! - `09 consts[0]` 값 `2` — `tick_per_second × 2 = 2초 구간`
//! - `09 knobs[5]` 미니언 위험 조회 구간 / `09 knobs[8]` 교전 판정 윈도우 `tps*2`
//!
//! 셋은 **같은 사실 하나**인데 IR 에서 `shl i64 %30, 1` 로 접혀 리터럴 2 가 없다(명세 `notes[0]`).
//! 4차 배치B 는 `danger_damage_at` 을 **직접** `tps*2` 로 불러 봤을 뿐, **09 가 실제로 무엇을 넘기는지**는
//! 안 재봤다 — `×1` 이나 `×3` 이었어도 그 실험은 똑같이 통과한다.
//!
//! ## 어떻게 가르나 — **09 의 게이트 ↔ 직접 호출의 일치**
//! 명세 09 `L1208~1209`:
//! ```text
//! dmg = enemy_minion_line_action_danger_damage_at(ver, data, champ,
//!         target_enemy.x, target_enemy.y, ctx.setting.tick_per_second * K, true, false)
//! if dmg != 0 { return false }
//! ```
//! `tick_per_second` 를 T 로 쓸어가며
//!   `r09 = check_favorable_engage_formation(...)` 과
//!   `gK  = danger_damage_at(..., T*K, true, false) != 0`  (K = 1,2,3,4)
//! 를 같이 찍는다. **`r09 == false` 를 전 구간 설명하는 K 가 곧 계수**다.
//! ⚠단 09 는 대형(formation) 이유로도 false 를 낸다 ⟹ **게이트가 안 걸린 T 에서의 값**을 기준선으로
//!   잡고, 기준선과 달라지는 T 만 게이트 발화로 본다.
//!
//! 미니언 스폰 레시피는 4차 배치B(`_verify4\B\B4_o09m.rs`) 것을 그대로 쓴다 — TEMPLATE ⑧.
//! ⚠TEMPLATE ② `init_tower`/`init_nexus` 재호출 금지. TEMPLATE ① `real_setting()`.
use game_core::*;
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
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    // ★미니언을 실제로 세우는 값 (4차 배치B 레시피)
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
    s.melee_minion.stat.attack = 10; s.melee_minion.stat.hp = 400;
    s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1; s.melee_minion.growth.hp = 30;
    s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100; s.melee_minion.attack.range = 3000;
    s.melee_minion.attack.cooltime = 30; s.melee_minion.attack.duration = 24;
    s.melee_minion.attack.start_timing = 16;
    s.melee_minion.exp = 40; s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15; s.range_minion.stat.hp = 250;
    s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1; s.range_minion.growth.hp = 20;
    s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000; s.range_minion.attack.speed = 3000;
    s.range_minion.attack.cooltime = 40; s.range_minion.attack.duration = 24;
    s.range_minion.attack.start_timing = 16;
    s.range_minion.exp = 30; s.range_minion.gold = 20;
    s
}


fn main() {
    let setting = real_setting();
    println!("setting_ok\ttps={}\theight={}\tchamp_radius={}",
             setting.tick_per_second, setting.height, setting.champion_radius);
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
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(9);
    for _ in 0..600usize {
        let mut fd: Option<&mut GameFrameData> = None;
        game.run_tick(&ctx, &mut rnd2, &mut fd);
    }
    println!("MINION_TOTAL\t{}", game.world.minion_ids.len());
    if game.world.minion_ids.is_empty() { println!("!! 미니언 0 — 중단"); return; }

    let (c0, c1, a0, pts) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut v: Vec<(u64, u64)> = Vec::new();
        for m in cache.iter_minions(1) { v.push((m.x, m.y)); if v.len() >= 3 { break; } }
        (cache.player_champion[0][0].unwrap().clone(),
         cache.player_champion[1][0].unwrap().clone(),
         cache.player_champion[0][1].unwrap().clone(), v)
    };
    println!("FOUNTAINS\t{:?}", map.fountains);
    let (ex, ey) = pts[0];
    let (fx0, fy0, fx1, fy1) = map.fountains[1];
    let (bx, by) = ((fx0 + fx1) / 2, (fy0 + fy1) / 2);
    println!("ENEMY\t({},{})\tENEMY_BASE\t({},{})", ex, ey, bx, by);
    // 적 → 적 분수 방향으로 200000 떨어진 지점 = 퇴로 차단(rear) 아군 자리
    let (rdx, rdy) = (bx as f64 - ex as f64, by as f64 - ey as f64);
    let rl = (rdx * rdx + rdy * rdy).sqrt();
    let t = 200000.0f64;
    let (ax, ay) = ((ex as f64 + rdx / rl * t) as u64, (ey as f64 + rdy / rl * t) as u64);
    println!("REAR_ALLY\t({},{})\tdist={}", ax, ay, t as u64);

    // ── T 스윕: 유리 대형(rear 아군 1명)을 세워 09 가 true 가 되게 하고,
    //           미니언 게이트가 켜지는 T 에서만 false 로 뒤집히는지 본다
    println!("\n#T\ttps\tr09\tg1\tg2\tg3\tg4");
    let mut rows: Vec<(usize, bool, [bool; 4])> = Vec::new();
    for tps in [1usize, 2, 5, 10, 15, 20, 25, 27, 30, 40, 41, 45, 50, 60, 79, 80, 81, 82, 100, 150] {
        let mut s2 = real_setting();
        s2.tick_per_second = tps;
        let ctx2 = GameContext {
            pool: &pool, setting: &s2, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off,
        };
        let mut ce = c0.clone();  ce.hp = 300; ce.stat_cached.hp = 1000; ce.x = ex + 1000; ce.y = ey;
        let mut al = a0.clone();  al.hp = 1000; al.stat_cached.hp = 1000; al.x = ax; al.y = ay;
        let mut te = c1.clone();  te.x = ex; te.y = ey;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx2);
        cache.player_champion[0][0] = Some(&ce);
        cache.player_champion[0][1] = Some(&al);
        cache.player_champion[1][0] = Some(&te);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx2, &bb);
        let ps = game.get_player_by_position(0, Position::Top).unwrap();
        let r09 = game_ai::check_favorable_engage_formation(0, ps, &data, &te, 200000);
        // ★플래그 4조합 × K=2 — 09 가 넘기는 (champion_action, predict_retarget) 을 가른다
        let gf = |ca: bool, pr: bool| game_ai::enemy_minion_line_action_danger_damage_at(
            0, &data, &ce, te.x, te.y, tps * 2, ca, pr) != 0;
        let gs = [gf(true, false), gf(false, false), gf(true, true), gf(false, true)];
        println!("T\t{}\t{}\t{}\t{}\t{}\t{}", tps, r09, gs[0], gs[1], gs[2], gs[3]);
        rows.push((tps, r09, gs));
    }
    // 계수 판정 — `r09 == !gK` 를 전 구간 만족하는 K 만 남는다
    println!("\n#K\tK\tagree\tdisagree\tverdict");
    for ki in 1..=4usize {
        let mut ag = 0; let mut dis = 0;
        for &(_t, r09, gs) in &rows {
            if r09 == !gs[ki - 1] { ag += 1 } else { dis += 1 }
        }
        println!("K\tK={}\t{}\t{}\t{}", ki, ag, dis, if dis == 0 { "가능" } else { "기각" });
    }
}
