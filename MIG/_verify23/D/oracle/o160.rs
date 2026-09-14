#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #160 get_die_tick_player (pub, m09.ll:52061) — `#[link_name]` 직접 호출 + 독립 재구현(명세 logic) 대조
//! 판별 축: dps(ChampionCache raw 쓰기) · 거리/이속(도달틱) · hp · RunAway(이속 차감) · 경계 `total+add_max >= hp*100`(케이스 5) · 타워
//! 사용: o160.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai9goal_data19get_die_tick_player"]
    fn gdt(data: *const OperationData, champ_team: usize, champ: usize, enemies: *const bumpalo::collections::Vec<usize>,
           tower: Option<&Entity>, action: Option<SmallAction>) -> u64;
}

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn r64(base: *const u8, off: usize) -> i64 { std::ptr::read_volatile(base.add(off) as *const i64) }
fn ceil_div(a: u64, b: u64) -> u64 { (a + b - 1) / b }

fn range_of(ef: &Effect, e: &Entity, champ: &Entity) -> u64 {
    // Effect::range 인라인(effect.rs:26) + range_adjust + 두 반지름
    let base = ef.range as u64 + ef.growth_range as u64 * (e.level as u64 - 1) + e.stat_buff_cached.range as u64;
    base + ef.range_adjust(e, champ) as u64 + e.radius() as u64 + champ.radius() as u64
}


fn build_events(cache: &AbstractGameWithCache, ctx: &GameContext, champ: &Entity, enemies: &[usize], enemy: usize, dps: &[(i64, i64, i64); 5],
                runaway: bool, champ_ms_v: i64, tower: Option<&Entity>) -> Vec<(u64, u64)> {
    let tps = ctx.setting.tick_per_second as u64;
    let mut events: Vec<(u64, u64)> = Vec::new();
    for &e in enemies.iter() {
        let ee: &Entity = cache.player_champion[enemy][e].expect("enemy");
        let (a_dps, s_dps, s2_dps) = (dps[e].0 as u64, dps[e].1 as u64, dps[e].2 as u64);
        let a_r = ee.attack_effect.as_ref().map(|f| range_of(f, ee, champ)).unwrap_or(0);
        let s_r = ee.skill_effect.as_ref().map(|f| range_of(f, ee, champ)).unwrap_or(0);
        let s2_r = if ee.level > 2 { ee.skill2_effect.as_ref().map(|f| range_of(f, ee, champ)).unwrap_or(0) } else { 0 };
        let dist = ee.distance(champ) as u64;
        let mut mv = ee.stat_cached.move_speed as u64;
        if runaway { mv = mv.saturating_sub(champ_ms_v as u64); }
        let mv = mv.max(1);
        let cc_tick = ee.cc.iter().filter(|c| c.block_input()).map(|c| c.tick() as u64).max().unwrap_or(0);
        events.push((ceil_div(dist.saturating_sub(a_r), mv) + cc_tick, a_dps));
        events.push((ceil_div(dist.saturating_sub(s_r), mv) + cc_tick, s_dps));
        events.push((ceil_div(dist.saturating_sub(s2_r), mv) + cc_tick, s2_dps));
    }
    if let Some(t) = tower {
        let d = t.attack_effect.as_ref().unwrap().expected_damage_target(ctx, t, champ) as u64 * tps / t.attack_cooltime() as u64;
        events.push((0, d));
    }
    for e in cache.others[enemy].iter() {
        if let Some(atk) = e.attack_effect.as_ref() {
            let d = atk.expected_damage_target(ctx, *e, champ) as u64 * tps / (e.attack_cooltime() as u64).max(1);
            let r = range_of(atk, e, champ);
            let dist = e.distance(champ) as u64;
            events.push((dist.saturating_sub(r) / (e.stat_cached.move_speed as u64).max(1), d));
        }
    }
    events.push((9999999999, 0));
    events.sort_by_key(|x| x.0);
    events
}
fn boundary_hp(events: &[(u64, u64)]) -> Option<u64> {
    let mut t0 = 0u64; let mut d0 = 0u64; let mut tot = 0u64;
    for (t, ad) in events.iter() {
        let dt = t - t0; let am = d0 * 100 * dt / 60;
        if *t > 0 && am > 0 && (tot + am) % 100 == 0 { return Some((tot + am) / 100); }
        tot += am; d0 += ad; t0 = *t;
    }
    None
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    let champ_team = 0usize; let champ_pos = 2usize; let enemy = 1usize;
    let champ: &Entity = cache.player_champion[champ_team][champ_pos].expect("champ");
    // 케이스 파라미터
    let mut enemies: Vec<usize> = vec![0, 1, 2, 3, 4];
    let mut dps: [(i64, i64, i64); 5] = [(0, 0, 0); 5];      // (attack, skill, skill2)_per_sec[champ] for enemy e
    let mut dx: [i64; 5] = [0; 5];                              // 적 e 를 champ.x+dx 로 (0 이면 이동 안 함)
    let mut ms: [i64; 5] = [0; 5];                              // 적 e 이속(0 이면 그대로)
    let mut champ_ms: i64 = 0;
    let mut hp: i64 = 0;                                        // 0 이면 그대로
    let mut runaway = false; let mut use_tower = false; let mut boundary = false;
    match case {
        0 => {}                                                            // dps 0 → 센티넬 9999999999
        1 => { dps[0] = (1000, 0, 0); dx[0] = 50000; ms[0] = 300; hp = 1500; }   // 한 명 접근 후 사망
        2 => { dps[0] = (1000, 0, 0); dx[0] = 50000; ms[0] = 300; hp = 1500; runaway = true; champ_ms = 200; }  // 이속 300-200
        3 => { dps[0] = (500, 300, 0); dps[1] = (700, 0, 200); dx[0] = 40000; dx[1] = 90000; ms[0] = 250; ms[1] = 400; hp = 3000; }
        4 => { dps[0] = (100, 0, 0); dx[0] = 1000; ms[0] = 300; hp = 1500; }   // 사거리 안(도달 0) → hp*100/(100) = 1500... 실제는 모델
        5 => { dps[0] = (100, 0, 0); dps[1] = (50, 0, 0); dx[0] = -1; dx[1] = 60000; ms[1] = 100; hp = 0; boundary = true; } // 경계 == 케이스 (hp 는 모델이 정함)
        6 => { dps[0] = (100, 0, 0); dps[1] = (50, 0, 0); dx[0] = -1; dx[1] = 60000; ms[1] = 100; hp = 0; boundary = true; enemies = vec![1, 0]; } // 순서 뒤집어 정렬 확인
        7 => { enemies = vec![]; dps[0] = (1000, 0, 0); hp = 1500; }        // 적 없음 → 센티넬
        8 => { dps[0] = (1000, 0, 0); dx[0] = 50000; ms[0] = 300; hp = 1500; use_tower = true; }   // 타워 Some — cooltime 0 이면 패닉 예상
        9 => { dps[0] = (0, 0, 3000); dx[0] = 30000; ms[0] = 1; hp = 1000; runaway = true; champ_ms = 5; }  // skill2 만 · 이속 1-5 → sat 0 → max 1
        _ => {}
    }
    let mut champ_ms_v = champ.stat_cached.move_speed as i64;
    unsafe {
        let cp = &cache as *const AbstractGameWithCache as *mut u8;
        for e in 0..5 {
            let ee: &Entity = cache.player_champion[enemy][e].expect("enemy");
            let ep = ee as *const Entity as *mut u8;
            let ccp = cp.add(0x280 + (enemy * 5 + e) * 800);
            p64(ccp, 0x190 + champ_pos * 8, dps[e].0); p64(ccp, 0x1b8 + champ_pos * 8, dps[e].1); p64(ccp, 0x1e0 + champ_pos * 8, dps[e].2);
            if dx[e] == -1 { p64(ep, 0x660, champ.x as i64); p64(ep, 0x668, champ.y as i64); } else if dx[e] != 0 { p64(ep, 0x660, champ.x as i64 + dx[e]); p64(ep, 0x668, champ.y as i64); }
            if ms[e] != 0 { p64(ep, 0x640, ms[e]); }
        }
        let chp = champ as *const Entity as *mut u8;
        if champ_ms != 0 { p64(chp, 0x640, champ_ms); champ_ms_v = champ_ms; }
        if hp != 0 { p64(chp, 0x670, hp); }
    }
    // ── 독립 재구현(명세 logic) ──
    let tower: Option<&Entity> = if use_tower { cache.iter_towers_without_nexus(1).next() } else { None };
    let mut events = build_events(&cache, &ctx, champ, &enemies, enemy, &dps, runaway, champ_ms_v, tower);
    let mut hp_v = champ.hp as u64;
    if boundary {
        // e1 의 x 를 훑어 어떤 도달 시각에서 total+add_max == hp*100 이 정확히 되는 hp 를 찾는다
        let mut found = None;
        for k in 0..200i64 {
            let e1: &Entity = cache.player_champion[enemy][1].expect("e1");
            unsafe { p64(e1 as *const Entity as *mut u8, 0x660, champ.x as i64 + 60000 + k * 37); }
            events = build_events(&cache, &ctx, champ, &enemies, enemy, &dps, runaway, champ_ms_v, tower);
            if let Some(h) = boundary_hp(&events) { found = Some(h); break; }
        }
        hp_v = found.expect("boundary hp");
        unsafe { p64(champ as *const Entity as *mut u8, 0x670, hp_v as i64); }
    }
    let hp100 = hp_v * 100;
    let (mut tick, mut dps_acc, mut total) = (0u64, 0u64, 0u64);
    let mut model = None;
    for (t, ad) in events.iter() {
        let dt = t - tick;
        let add_max = dps_acc * 100 * dt / 60;
        if total + add_max >= hp100 { model = Some(tick + (hp100 - total) / dps_acc.max(1)); break; }
        total += add_max; dps_acc += ad; tick = *t;
    }
    let model = model.unwrap_or(if tick == 0 { 9999999999 } else { tick });
    // 경계 대안 모델(`>` 였다면): 보고용
    let mut alt = None; let (mut tk, mut da, mut to) = (0u64, 0u64, 0u64);
    for (t, ad) in events.iter() { let dt = t - tk; let am = da * 100 * dt / 60;
        if to + am > hp100 { alt = Some(tk + (hp100 - to) / da.max(1)); break; } to += am; da += ad; tk = *t; }
    let alt = alt.unwrap_or(if tk == 0 { 9999999999 } else { tk });

    let ev = bumpalo::collections::Vec::from_iter_in(enemies.iter().copied(), &pool);
    let action: Option<SmallAction> = if runaway { Some(SmallAction::RunAway) } else { None };
    let got = unsafe { gdt(&data as *const OperationData, champ_team, champ_pos, &ev as *const bumpalo::collections::Vec<usize>, tower, action) };
    println!("case={}	got={}	model={}	{}	(alt_if_gt={} hp={} champ_ms={} events={:?} tower={})", case, got, model,
             if got == model { "MATCH" } else { "DIFF" }, alt, hp_v, champ_ms_v, &events[..events.len().min(8)], tower.is_some());
}
