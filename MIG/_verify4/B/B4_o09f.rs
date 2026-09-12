#![allow(unused, dead_code, non_snake_case)]
//! 4차 반증검증 배치 B — 09 `check_favorable_engage_formation` 진리표 **독립 재구현 대조**
//! `specs20_v3.json /specs[9]/logic` 을 그대로 Rust 로 옮겨 놓고 실제 함수와 N 케이스 대조한다.
//! (2차의 400/400 은 "같은 프로브가 만든 진리표"였고, 여기서는 **명세 → 재구현**을 대조한다)
//! ⚠ TEMPLATE 준수: 실전 GameSetting · init_tower/init_nexus 재호출 금지
use game_core::*;
use rand::{Rng, SeedableRng};
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
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

fn dist_sq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    dx * dx + dy * dy
}

/// `/specs[9]/logic` 의 1203~1348 을 그대로 옮긴 재구현.
/// 미니언 게이트(1208~1209)는 명세대로 피호출자를 그대로 호출한다.
fn mine(version: usize, team: usize, champ: &Entity, allies: &[Option<&Entity>; 5],
        target_enemy: &Entity, engage_range: u64,
        data: &OperationData, map: &MapDef, tps: usize) -> bool {
    let enemy_team = 1 - team;
    // 1208~1209
    let dmg = game_ai::enemy_minion_line_action_danger_damage_at(
        version, data, champ, target_enemy.x, target_enemy.y, tps * 2, true, false);
    if dmg != 0 { return false; }
    // 1214~1216
    let (elx, ely, erx, ery) = map.fountains[enemy_team];
    let ebx = (elx + erx) / 2;
    let eby = (ely + ery) / 2;
    // 1219~1223
    let rdx = ebx as i128 - target_enemy.x as i128;
    let rdy = eby as i128 - target_enemy.y as i128;
    let rlen = rdx * rdx + rdy * rdy;
    if rlen < 1 { return true; }
    // 1229~1231
    let mut front = 0usize; let mut flank = 0usize; let mut rear = 0usize;
    let e2b = dist_sq(target_enemy.x, target_enemy.y, ebx, eby);
    let maxd = engage_range + 100000;
    for ap in 0..5usize {
        let Some(ally) = allies[ap] else { continue };
        if ally.id == champ.id { continue }
        if ally.hp * 100 / ally.stat_cached.hp < 40 { continue }
        let a2e = dist_sq(ally.x, ally.y, target_enemy.x, target_enemy.y);
        if a2e > maxd * maxd { continue }
        let adx = target_enemy.x as i128 - ally.x as i128;
        let ady = target_enemy.y as i128 - ally.y as i128;
        let alen = adx * adx + ady * ady;
        if alen < 1 { front += 1; continue }
        let dot = adx * rdx + ady * rdy;
        let cross = adx * rdy - ady * rdx;
        let lp = alen * rlen;
        let dot_sq = dot * dot;
        let cross_sq = cross * cross;
        let is_front = dot > 0 && dot_sq * 4 > lp;
        let is_rear_dir = dot < 0 && dot_sq * 100 > lp * 9;
        let is_flank = cross_sq * 100 > lp * 9;
        if is_front { front += 1; }
        else if is_rear_dir {
            let t2b = dist_sq(ally.x, ally.y, ebx, eby);
            if t2b < e2b { rear += 1; } else { flank += 1; }
        } else if is_flank { flank += 1; }
        else { front += 1; }
    }
    if rear > 0 { return true }
    if flank > 1 || (flank > 0 && front > 0) { return true }
    if front > 1 {
        let c2b = dist_sq(champ.x, champ.y, ebx, eby);
        return c2b * 5 <= e2b * 6;
    }
    false
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             true, setting.width, setting.height, setting.tick_per_second,
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
    let (base0, base1, ally_ids) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut ids = [0usize; 5];
        for k in 0..5usize { ids[k] = c.player_champion[0][k].unwrap().id; }
        (c.player_champion[0][0].unwrap().clone(), c.player_champion[1][0].unwrap().clone(), ids)
    };
    println!("ALLY_IDS	{:?}", ally_ids);
    println!("BASE\tchamp hp={}/{}\tenemy hp={}/{}", base0.hp, base0.stat_cached.hp, base1.hp, base1.stat_cached.hp);

    let mut rng = rand::rngs::StdRng::seed_from_u64(20260911);
    let mut n = 0usize; let mut ok = 0usize; let mut bad = 0usize;
    let mut tcnt = 0usize;
    let ps = game.get_player_by_position(0, Position::Top).unwrap();
    for _ in 0..600usize {
        let ex: u64 = rng.gen_range(40000u64..920000);
        let ey: u64 = rng.gen_range(40000u64..920000);
        let mut en = base1.clone(); en.x = ex; en.y = ey;
        let mut ce = base0.clone();
        ce.x = rng.gen_range(40000u64..920000);
        ce.y = rng.gen_range(40000u64..920000);
        ce.hp = 1_000_000; ce.stat_cached.hp = 1_000_000;
        // 아군 4명 (Jungle..Support) 을 적 주변에 흩뿌린다
        let mut al: Vec<Entity> = Vec::new();
        for k in 1..5usize {
            let mut a = base0.clone();
            a.id = ally_ids[k];
            let r: u64 = rng.gen_range(1000u64..340000);
            let th: u64 = rng.gen_range(0u64..360);
            let (dx, dy) = match th / 90 {
                0 => (r, r / 2), 1 => (r / 2, r), 2 => (r, r), _ => (r / 3, r),
            };
            a.x = if th % 2 == 0 { ex.saturating_add(dx) } else { ex.saturating_sub(dx) };
            a.y = if th % 3 == 0 { ey.saturating_add(dy) } else { ey.saturating_sub(dy) };
            a.hp = rng.gen_range(1usize..=1000);
            a.stat_cached.hp = 1000;
            al.push(a);
        }
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        for k in 1..5usize { cache.player_champion[0][k] = Some(&al[k - 1]); }
        let arr = cache.player_champion[0];
        let data = OperationData::new(&cache, &ctx, &bb);
        for er in [100000u64, 200000u64, 400000u64] {
            let got = game_ai::check_favorable_engage_formation(0, ps, &data, &en, er);
            let exp = mine(0, 0, &ce, &arr, &en, er, &data, &map, tps);
            n += 1;
            if got { tcnt += 1; }
            if got == exp { ok += 1; } else {
                bad += 1;
                if bad <= 8 {
                    println!("MISMATCH\tenemy=({},{})\tchamp=({},{})\ter={}\tgot={}\texp={}",
                             ex, ey, ce.x, ce.y, er, got, exp);
                }
            }
        }
    }
    println!("RESULT\tn={}\tok={}\tbad={}\ttrue_count={}", n, ok, bad, tcnt);

    // version 12종 무영향 재확인 (2차 ev2 표본)
    {
        let mut ce = base0.clone(); ce.x = 480000; ce.y = 480000;
        ce.hp = 1_000_000; ce.stat_cached.hp = 1_000_000;
        let mut al: Vec<Entity> = Vec::new();
        for k in 0..4usize {
            let mut a = base0.clone();
            a.id = ally_ids[k + 1];
            a.x = 500000 + (k as u64) * 30000; a.y = 460000 - (k as u64) * 20000;
            a.hp = 900; a.stat_cached.hp = 1000;
            al.push(a);
        }
        let mut en = base1.clone(); en.x = 520000; en.y = 500000;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0] = Some(&ce);
        for k in 1..5usize { cache.player_champion[0][k] = Some(&al[k - 1]); }
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut vs = Vec::new();
        for v in [0usize, 1, 2, 3, 12, 24, 30, 40, 46, 50, 60, 99] {
            vs.push(game_ai::check_favorable_engage_formation(v, ps, &data, &en, 200000));
        }
        let diff = vs.iter().filter(|&&b| b != vs[0]).count();
        println!("VERSION\tbase={}\tdiff={}\tall={:?}", vs[0], diff, vs);
    }
}
