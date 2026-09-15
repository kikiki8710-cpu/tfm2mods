#![allow(unused, dead_code, non_snake_case)]
//! 24차 A · 180 position_eval_at_uncached 오라클 — internal fastcc 라 심볼이 없으므로 **pub 호출자 `game_ai::position_eval_at`(187) 경유**.
//! POS_EVAL_CACHE(TLS) 가 끼어 있으므로 **한 프로세스 = 한 케이스(argv[1])**. 케이스 12·13 만 의도적으로 같은 프로세스에서
//! 2~3회 호출해 캐시 hit/miss 조건(tick·seed) 자체를 잰다.
//! 예측 = 명세 logic(v3 specs[180]) 을 손으로 합성: 381 벽=9999 · 375 None=0 · 397/689~690 우물 9999+pct_q32(well_damage) ·
//!   1142 purpose 보정 · 1148~1176 노이즈 해시(K=0x9E3779B97F4A7C15 · h^=h>>31 · spread=(2000-2acc)/3 · 6초 버킷).
use game_core::*;
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
pub fn mkgame(seed: u64, setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(seed, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

const K: u64 = 0x9E3779B97F4A7C15;
/// position_eval.rs:386~387 pct_q32 — Q32 고정소수(2^32/max(hp,1) 를 먼저 내림) — i128 곱 후 >>32, 150 캡
fn pct_q32(hp: u64, dmg: u64) -> i64 {
    let inv: u128 = (1u128 << 32) / (hp.max(1) as u128);
    let v = (inv * 100 * dmg as u128) >> 32;
    v.min(150) as i64
}
/// 1142 apply_position_eval_purpose + 1148~1176 노이즈 (모델). 반환 (risk, tower_risk, gain)
fn model_purpose_noise(purpose: u8 /*메모리태그: 0/1 LineStyle, 2 General, 3 RunAway, 4 Recall, 10 LaneSafe*/, version: usize,
                       risk0: i64, tower_risk0: i64, gain0: i64, acc: i64, id: u64, tick: u64, tps: u64, x: u64, y: u64, deathmatch: bool) -> (i64, i64, i64) {
    let idx: u8 = if purpose > 1 { purpose - 2 } else { 7 };
    let (mut risk, mut tower_risk, mut gain) = (risk0, tower_risk0, gain0);
    match idx {
        7 => { if purpose == 1 { risk = risk * 120 / 100; } else { gain = gain * 120 / 100; } }  // 33 Defensive / 36 Aggressive
        8 => { risk = risk * 3 / 2; tower_risk = tower_risk * 3 / 2; }                              // 39~40 LaneSafe
        _ => {}
    }
    let noisy = if version > 1 { !(idx == 1 || idx == 2) && acc < 1000 } else { acc < 1000 };
    if !noisy { return (risk, tower_risk, gain); }
    let base = risk - tower_risk;
    if base == 0 { return (risk, tower_risk, gain); }
    let spread: i64 = if deathmatch { 1000 - acc } else { (2000 - 2 * acc) / 3 };
    let t_salt: u64 = if version > 1 { tick / (tps * 6).max(1) } else { tick };
    let mut h: u64 = id;
    h = (h ^ t_salt).wrapping_mul(K);
    h = (h ^ (x / 32000)).wrapping_mul(K);
    h = (h ^ (y / 32000)).wrapping_mul(K);
    h ^= h >> 31;
    let factor: i64 = (h % ((2 * spread + 1) as u64)) as i64 - spread + 1000;
    risk = tower_risk + base * factor / 1000;
    (risk, tower_risk, gain)
}

fn fmt(s: &PositioningScore) -> String {
    format!("({},{},{},{},{},{},{},{})", s.risk, s.tower_risk, s.gain, s.gain_me, s.adjust, s.unseen_champ_threat, s.on_trajectory as u8, s.on_periodic_trajectory as u8)
}
fn tup(r: i64, t: i64, g: i64) -> String { format!("({},{},{},0,0,0,0,0)", r, t, g) }

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(1234, &setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let ps = game.get_player_by_position(0, Position::Top).unwrap();
    let e = cache.player_champion[0][0].unwrap();
    let acc = ps.info.parameter.positioning_accuracy() as i64;
    let pid = ps.info.id as u64;
    let tps = setting.tick_per_second as u64;
    println!("setup\tcase={}\tseed={}\ttick={}\tacc={}\tpid={}\tchamp_id={}\thp={}\twell_damage={}", n, game.seed(), game.tick(), acc, pid, e.id, e.hp, setting.well_damage);
    use game_ai::PositionEvalPurpose as P;
    let well = (900000u64, 30000u64);          // 적(팀1) 우물 사각 안: is_enemy_well_danger=true · is_in_well_damage(1)=true
    let gp = &game as *const Game as *mut Game;
    let mp = &map as *const MapDef as *mut MapDef;
    let cp = &cache as *const AbstractGameWithCache as *mut AbstractGameWithCache;
    let ep = e as *const Entity as *mut Entity;
    let mut report = |case: usize, got: String, pred: String, note: &str| {
        println!("o180\tcase={}\t{}\tgot={}\tpred={}\t{}", case, if got == pred { "MATCH" } else { "MISMATCH" }, got, pred, note);
    };
    match n {
        0 => { // 381 벽 셀 → risk 9999, 나머지 0 (memset 42B @8)
            let (xi, yi) = (15u64, 3u64); assert!(map.walls[yi as usize][xi as usize] != 0);
            let s = game_ai::position_eval_at(2, ps, &data, xi * 32000 + 16000, yi * 32000 + 16000, P::Recall);
            report(0, fmt(&s), tup(9999, 0, 0), "wall(15,3) Recall v2");
        }
        1 => { // 378 xi = umin(x/32000, 29) 클램프: walls[3][29] 를 벽으로 세운 뒤 x=5_000_000(=156셀) → 클램프 29 → 9999. 대조군 x=928000(=29셀) 도 9999, x=864000(=27셀·우물 밖) 은 0 (28셀=896000 은 우물 2사각 안이라 10149 — 첫 실행에서 확인)
            unsafe { std::ptr::write_volatile(&mut (*mp).walls[3][29], 1); }
            let a = game_ai::position_eval_at(2, ps, &data, 5_000_000, 3 * 32000 + 16000, P::Recall);
            let b = game_ai::position_eval_at(2, ps, &data, 928000, 3 * 32000 + 16000, P::Recall);
            let c = game_ai::position_eval_at(2, ps, &data, 864000, 3 * 32000 + 16000, P::Recall);
            report(1, format!("{} | {} | {}", fmt(&a), fmt(&b), fmt(&c)), format!("{} | {} | {}", tup(9999, 0, 0), tup(9999, 0, 0), tup(0, 0, 0)),
                   "walls[3][29]=1: x=5000000(클램프 29) · x=928000(29) · x=864000(27, 우물 밖)");
        }
        2 => { // 375 player_champion None → default() memset 50B
            unsafe { std::ptr::write_volatile(&mut (*cp).player_champion[0][0], None); }
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(2, fmt(&s), tup(0, 0, 0), "player_champion[0][0]=None");
        }
        3 => { // 397 우물 9999 + 690 pct_q32(well_damage) · hp=1 → 150 캡 · Recall v2 = 노이즈 면제
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(3, fmt(&s), tup(9999 + pct_q32(1, 600), 9999, 0), "well(900000,30000) hp=1 Recall v2");
        }
        4 => { // ★Q32 절단: hp=1000 → (floor(2^32/1000)*100*600)>>32 = 59 (소박한 100*600/1000 = 60 이 아님)
            unsafe { std::ptr::write_volatile(&mut (*ep).hp, 1000); }
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(4, fmt(&s), tup(9999 + pct_q32(1000, 600), 9999, 0), &format!("hp=1000 pct_q32={} naive={}", pct_q32(1000, 600), 60000 / 1000));
        }
        5 => { // hp=600 → 99 (naive 100)
            unsafe { std::ptr::write_volatile(&mut (*ep).hp, 600); }
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(5, fmt(&s), tup(9999 + pct_q32(600, 600), 9999, 0), &format!("hp=600 pct_q32={} naive={}", pct_q32(600, 600), 60000 / 600));
        }
        6 => { // General v2 tick0 → 노이즈 (base 150)
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::General);
            let (r, t, g) = model_purpose_noise(2, 2, 10149, 9999, 0, acc, pid, 0, tps, well.0, well.1, false);
            report(6, fmt(&s), tup(r, t, g), "General v2 tick0 noisy");
        }
        7 => { // General v2 tick 400 → t_salt = 400/360 = 1
            unsafe { (*gp).set_tick(400); }
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::General);
            let (r, t, g) = model_purpose_noise(2, 2, 10149, 9999, 0, acc, pid, 400, tps, well.0, well.1, false);
            report(7, fmt(&s), tup(r, t, g), &format!("General v2 tick400 (t_salt=1) game.tick={}", game.tick()));
        }
        8 => { // Recall v1 tick 400 → v1 은 면제 없음, t_salt = tick
            unsafe { (*gp).set_tick(400); }
            let s = game_ai::position_eval_at(1, ps, &data, well.0, well.1, P::Recall);
            let (r, t, g) = model_purpose_noise(4, 1, 10149, 9999, 0, acc, pid, 400, tps, well.0, well.1, false);
            report(8, fmt(&s), tup(r, t, g), "Recall v1 tick400 (t_salt=400)");
        }
        9 => { // LineStyle(Defensive) v2 → risk*120/100 후 노이즈
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::LineStyle(LineStyle::Defensive));
            let (r, t, g) = model_purpose_noise(1, 2, 10149, 9999, 0, acc, pid, 0, tps, well.0, well.1, false);
            report(9, fmt(&s), tup(r, t, g), "LineStyle(Defensive) v2 tick0");
        }
        10 => { // LaneSafe v2 → risk*3/2, tower_risk*3/2 후 노이즈
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::LaneSafe);
            let (r, t, g) = model_purpose_noise(10, 2, 10149, 9999, 0, acc, pid, 0, tps, well.0, well.1, false);
            report(10, fmt(&s), tup(r, t, g), "LaneSafe v2 tick0");
        }
        11 => { // LineStyle(Aggressive) v2 → gain 만 120% (gain 0) · risk 는 General 과 동일 노이즈
            let s = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::LineStyle(LineStyle::Aggressive));
            let (r, t, g) = model_purpose_noise(0, 2, 10149, 9999, 0, acc, pid, 0, tps, well.0, well.1, false);
            report(11, fmt(&s), tup(r, t, g), "LineStyle(Aggressive) v2 tick0");
        }
        12 => { // ★POS_EVAL_CACHE hit: 같은 (key,tick,seed) 2회째는 uncached 가 안 불린다 → 벽을 세워도 옛 값 · tick 바꾸면 재계산
            let s1 = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            unsafe { std::ptr::write_volatile(&mut (*mp).walls[0][28], 1); } // (900000/32000=28, 30000/32000=0)
            let s2 = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            unsafe { (*gp).set_tick(1); }
            let s3 = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(12, format!("{} | {} | {}", fmt(&s1), fmt(&s2), fmt(&s3)),
                   format!("{} | {} | {}", tup(10149, 9999, 0), tup(10149, 9999, 0), tup(9999, 0, 0)),
                   "call1 miss → call2(벽 세움, 같은 tick) hit=옛값 → call3(tick=1) miss=벽 9999");
        }
        13 => { // ★seed 불일치 → 512 슬롯 리셋 후 miss: 게임 B(seed 999, 벽 있는 맵) 같은 tick0·같은 key
            let s1 = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            let map2 = MapDef::moba(&setting);
            unsafe { std::ptr::write_volatile(&mut (*(&map2 as *const MapDef as *mut MapDef)).walls[0][28], 1); }
            let pool2 = bumpalo::Bump::new();
            let ctx2 = GameContext { pool: &pool2, setting: &setting, macro_weights: &mw, map_setting: &ms,
                map: &map2, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
                tutorial: TutorialType::None, trace_level: TraceLevel::Off };
            let game2 = mkgame(999, &setting, &ms, &map2, &ctx2);
            let cache2 = AbstractGameWithCache::new(&game2 as &dyn AbstractGame, &ctx2);
            let bb2: [Blackboard; 2] = [Default::default(), Default::default()];
            let data2 = OperationData::new(&cache2, &ctx2, &bb2);
            let ps2 = game2.get_player_by_position(0, Position::Top).unwrap();
            let s2 = game_ai::position_eval_at(2, ps2, &data2, well.0, well.1, P::Recall);
            // 같은 게임 B 에서 한 번 더(hit 확인) 후, 게임 A 로 돌아오면(seed 1234) 다시 miss → 벽 없는 맵 → 10149
            let s3 = game_ai::position_eval_at(2, ps2, &data2, well.0, well.1, P::Recall);
            let s4 = game_ai::position_eval_at(2, ps, &data, well.0, well.1, P::Recall);
            report(13, format!("{} | {} | {} | {}", fmt(&s1), fmt(&s2), fmt(&s3), fmt(&s4)),
                   format!("{} | {} | {} | {}", tup(10149, 9999, 0), tup(9999, 0, 0), tup(9999, 0, 0), tup(10149, 9999, 0)),
                   &format!("A(seed {}) miss → B(seed {} 벽) miss=9999 → B hit → A miss(리셋됨)=10149  pid2={}", game.seed(), game2.seed(), ps2.info.id));
        }
        14 => { // 689 우물 사각 경계: x=800000 포함 / 799999 제외 (y=64000 포함)
            let a = game_ai::position_eval_at(2, ps, &data, 800000, 64000, P::Recall);
            let b = game_ai::position_eval_at(2, ps, &data, 799999, 64000, P::Recall);
            let c = game_ai::position_eval_at(2, ps, &data, 800000, 64001, P::Recall);
            let wd = (game_ai::is_enemy_well_danger(2, ps, 800000, 64000), game_ai::is_enemy_well_danger(2, ps, 799999, 64000), game_ai::is_enemy_well_danger(2, ps, 800000, 64001));
            report(14, format!("{} | {} | {}", fmt(&a), fmt(&b), fmt(&c)), format!("{} | {} | {}", tup(10149, 9999, 0), tup(0, 0, 0), tup(0, 0, 0)),
                   &format!("(800000,64000) in · (799999,64000) out · (800000,64001) out  well_danger={:?}", wd));
        }
        15 => { // 689 두 번째 사각: (896000,160000) 포함 · (895999,160000) 제외 · (896000,160001) 제외
            let a = game_ai::position_eval_at(2, ps, &data, 896000, 160000, P::Recall);
            let b = game_ai::position_eval_at(2, ps, &data, 895999, 160000, P::Recall);
            let c = game_ai::position_eval_at(2, ps, &data, 896000, 160001, P::Recall);
            let wd = (game_ai::is_enemy_well_danger(2, ps, 896000, 160000), game_ai::is_enemy_well_danger(2, ps, 895999, 160000), game_ai::is_enemy_well_danger(2, ps, 896000, 160001));
            report(15, format!("{} | {} | {}", fmt(&a), fmt(&b), fmt(&c)), format!("{} | {} | {}", tup(10149, 9999, 0), tup(0, 0, 0), tup(0, 0, 0)),
                   &format!("2nd rect: (896000,160000) in · (895999,160000) out · (896000,160001) out  well_danger={:?}", wd));
        }
        _ => println!("o180\tcase={}\tSKIP\tunknown", n),
    }
}
