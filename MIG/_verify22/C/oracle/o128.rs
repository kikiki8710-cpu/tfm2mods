#![allow(unused, dead_code, non_snake_case)]
//! 22차 C · 128 expected_goal_position 오라클 — ★`in:game_ai`(hidden) 함수를 `#[link_name]` 로 직접 진입하는 실험.
//! `-C lto=fat` 이면 game_ai 의 hidden define 이 같은 LTO 단위에 있어 mangled 심볼 선언으로 링크된다(가설).
//! 한 프로세스 = 한 케이스(argv[1] = 케이스 id).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB2_16SmallActionTrace22expected_goal_position"]
    fn egp(s: &game_ai::SmallActionTrace, p: &PlayerState, d: &OperationData) -> Option<(u64, u64)>;
}

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
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}", ok, s.width, s.height, s.tick_per_second, s.champion_radius);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    let me = cache.player_champion[0][0].unwrap();      // team0 Top
    let tgt = cache.player_champion[1][0].unwrap();     // team1 Top
    let player = game.get_player_by_position(0, Position::Top).unwrap();

    // 케이스별 세계 조작: 엔티티는 pub 필드 — raw 포인터로 고친다(함정 ⑦).
    let mp = me as *const Entity as *mut Entity;
    let tp = tgt as *const Entity as *mut Entity;
    unsafe {
        match case {
            0 => {}
            1 => { (*tp).x = (*mp).x + 50000; (*tp).y = (*mp).y; }            // 가까움, 동일 y
            2 => { (*tp).x = (*mp).x + 50000; (*tp).y = (*mp).y; (*tp).visible_state[0] = VisibleState::Unknown; }
            3 => { (*tp).x = (*mp).x + 300000; (*tp).y = (*mp).y - 100000; }
            4 => { (*tp).x = (*mp).x + 30000; (*tp).y = (*mp).y - 40000; (*tp).visible_state[0] = VisibleState::Visible; }   // dist 50000, fd 5000
            5 => { (*tp).x = (*mp).x + 3; (*tp).y = (*mp).y; (*tp).visible_state[0] = VisibleState::Visible; }               // 아주 가까움 sz=3
            6 => { (*tp).x = (*mp).x; (*tp).y = (*mp).y; (*tp).visible_state[0] = VisibleState::Visible; }                   // 동일 좌표 sz=max(0,1)
            7 => { (*tp).x = (*mp).x + 30000; (*tp).y = (*mp).y - 40000; (*tp).visible_state[0] = VisibleState::Invisible{last_x:1,last_y:2}; }
            8 => { (*mp).team = TeamType::Neutral; (*tp).x = (*mp).x + 30000; (*tp).y = (*mp).y - 40000; }                 // champ Neutral → 가시성 검사 생략
            9 => { let mut found=None; 'o: for ty in 0..30 { for tx in 0..30 { if map.bushes[ty][tx] != 0 { found=Some((ty,tx)); break 'o; } } }
                   let (ty,tx)=found.expect("no bush"); (*tp).x = (tx as u64)*32000+16000; (*tp).y = (ty as u64)*32000+16000; (*tp).visible_state[0] = VisibleState::Visible; println!("bush	ty={} tx={}", ty, tx); }
            10 | 11 | 12 | 13 => { (*tp).x = (*mp).x + 30000; (*tp).y = (*mp).y - 40000; (*tp).visible_state[0] = VisibleState::Visible; }
            _ => {}
        }
    }
    let tr = match case {
        10 => game_ai::SmallActionTrace::new_keep_range(&data, tgt.id, me.id, 40000),
        11 => game_ai::SmallActionTrace::new_attack_range_margin(&data, tgt.id, me.id, 50000),
        12 => game_ai::SmallActionTrace::new_attack_range(&data, tgt.id, me.id),
        13 => game_ai::SmallActionTrace::new_keep_range(&data, tgt.id, me.id, 0),
        _ => game_ai::SmallActionTrace::new(&data, tgt.id, me.id) };
    println!("trace\t{:?}", tr);
    println!("me\tid={} x={} y={} team={:?} lvl={} range={} radius={} mult={}", me.id, me.x, me.y, me.team, me.level, me.stat_buff_cached.range, me.radius, me.stat_buff_cached.radius_mult);
    println!("tgt\tid={} x={} y={} vis0={:?}", tgt.id, tgt.x, tgt.y, tgt.visible_state[0]);
    println!("atk\t{:?}", me.attack_effect.as_ref().map(|e| (e.range, e.growth_range)));
    let r = unsafe { egp(&tr, player, &data) };
    println!("RESULT\tcase={}\t{:?}", case, r);
    // 독립 재구현(명세 logic) — 비교용
    let radius = |e: &Entity| -> u64 { if e.stat_buff_cached.radius_mult == 0 { e.radius as u64 } else { (e.radius as i64 * (e.stat_buff_cached.radius_mult as i64 + 100) / 100) as u64 } };
    let rs = radius(me) + radius(tgt);
    let base = me.stat_buff_cached.range as u64 + rs;
    let atk = me.attack_effect.as_ref();
    let mine: Option<(u64,u64)> = match atk {
        None => None,
        Some(a) => {
            let emr_tag: u64 = unsafe { *((&tr as *const game_ai::SmallActionTrace as *const u8).add(0x0) as *const u64) };
            let emr_val: u64 = unsafe { *((&tr as *const game_ai::SmallActionTrace as *const u8).add(0x8) as *const u64) };
            let m = if emr_tag & 1 == 1 { emr_val } else { base + a.range as u64 + (me.level as u64 - 1) * a.growth_range as u64 + Effect::range_adjust(a, me, tgt) as u64 };
            let vis = match tgt.visible_state[0] { VisibleState::Visible => true, _ => false };
            if !vis { Some((tgt.x, tgt.y)) } else {
                let txi = (tgt.x / 32000).min(29) as usize; let tyi = (tgt.y / 32000).min(29) as usize;
                if map.bushes[tyi][txi] != 0 { Some((tgt.x, tgt.y)) } else {
                    let dx = me.x as i64 - tgt.x as i64; let dy = me.y as i64 - tgt.y as i64;
                    let sz = game_core::utils::isqrt(dx*dx + dy*dy).max(1);
                    let margin: u64 = unsafe { *((&tr as *const game_ai::SmallActionTrace as *const u8).add(0x78) as *const u64) };
                    let fd = m.saturating_sub(margin);
                    let x = tgt.x as i64 + (fd as i64) * dx / sz; let y = tgt.y as i64 + (fd as i64) * dy / sz;
                    Some(Game::adjust_position(&map, &setting, x, y))
                } }
        }
    };
    println!("MINE\tcase={}\t{:?}\tmatch={}", case, mine, mine == r);
}
