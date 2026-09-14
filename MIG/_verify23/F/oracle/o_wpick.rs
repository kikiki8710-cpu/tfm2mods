#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치F 오라클 — `positioning_window_pick<(i32,i32,i32,i32,i64)>` (define hidden, m11.ll:11351)
//! 172/173/177/178/179 의 「창 안에서 하나 고르기」 규칙을 실행으로 확정한다.
//! IR 독해(m11.ll:11359~11439): version>1 이면 rnd 를 쓰지 않고
//!   bucket = game.tick() / max(setting.tick_per_second*6, 1)
//!   h = (bucket<<32) ^ player.info.id ; h ^= h>>33 ; h ^= 20942 ; h = h.wrapping_mul(0x9E3779B97F4A7C15) ; h ^= h>>29
//!   idx = h % len   (len==0 → rem_by_zero 패닉)
//! version<=1 이면 rand::seq::SliceRandom::choose(rnd).unwrap()
//! 사용: o_wpick.exe <case>   (케이스당 프로세스 1개 · TLS 메모 없음이지만 규약대로)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

type T5 = (i32, i32, i32, i32, i64);
extern "Rust" {
    #[link_name = "_RINvNtCshdEBA0ozCnw_7game_ai12small_action23positioning_window_pickTllllxEEB4_"]
    fn wpick<'a>(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, c: &'a [T5]) -> &'a T5;
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

fn expect_idx(tick: u64, tps: u64, id: u64, len: u64) -> u64 {
    let bucket = tick / std::cmp::max(tps.wrapping_mul(6), 1);
    let mut h = (bucket << 32) ^ id;
    h ^= h >> 33;
    h ^= 20942;
    h = h.wrapping_mul(0x9E3779B97F4A7C15u64);
    h ^= h >> 29;
    h % len
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut setting = real_setting();
    // case 축: (version, tick, len, team, pos, tps)
    let (version, tick, len, team, pos, tps): (usize, usize, usize, usize, usize, usize) = match case {
        0 => (2, 0, 7, 0, 0, 60),
        1 => (2, 359, 7, 0, 0, 60),     // 같은 버킷(0..359)
        2 => (2, 360, 7, 0, 0, 60),     // 버킷 1
        3 => (2, 12345, 49, 1, 3, 60),
        4 => (2, 12345, 49, 1, 4, 60),  // id 만 다름
        5 => (2, 12345, 1, 1, 4, 60),   // len 1 → 항상 0
        6 => (2, 777777, 13, 0, 2, 60),
        7 => (2, 12345, 49, 1, 3, 0),   // tps=0 → max(0,1)=1 → bucket=tick
        8 => (3, 12345, 49, 1, 3, 60),  // version 3 도 같은 경로(>1)
        9 => (1, 12345, 49, 1, 3, 60),  // version 1 → SliceRandom::choose (rnd 소비)
        10 => (0, 12345, 49, 1, 3, 60),
        _ => (2, 0, 7, 0, 0, 60),
    };
    setting.tick_per_second = tps;
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[pos]).expect("player");
    let id = player.info.id as u64;
    let v: Vec<T5> = (0..len).map(|i| (i as i32, 0, 1000 * i as i32, 0, -(i as i64))).collect();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let before = format!("{:?}", rnd);
    let got = unsafe { wpick(version, &mut rnd, player, &data, &v) };
    let after = format!("{:?}", rnd);
    let got_idx = got.0 as u64;
    let exp = if version > 1 { expect_idx(tick as u64, tps as u64, id, len as u64) } else { u64::MAX };
    println!("case={} version={} tick={} tps={} team={} pos={} id={} len={} got_idx={} expect_idx={} rnd_changed={} verdict={}",
        case, version, tick, tps, team, pos, id, len, got_idx, exp, before != after,
        if version > 1 { if got_idx == exp { "MATCH" } else { "MISMATCH" } } else { "RND-PATH" });
}
