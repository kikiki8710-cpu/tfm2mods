#![allow(unused, dead_code, non_snake_case)]
//! 22차 배치D 오라클 — #131 noncombat_steroid_value (hidden define, m10.ll:34945) — `#[link_name]` 직접 호출(o132 에서 실증한 수법)
//! 실전 게임(TEMPLATE mkgame) 의 cache/data 를 그대로 쓰고, target Entity·ChampionCache 는 raw ptr 로 조립한다(수법 ⑦).
//! 사용: o131.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23noncombat_steroid_value"]
    fn nsv(data: *const OperationData, buff: *const u8, target: *const Entity, window: *const u8) -> i64;
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

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);
fn w32(b: &mut [u8], off: usize, v: i32) { b[off..off + 4].copy_from_slice(&v.to_le_bytes()); }
fn w64(b: &mut [u8], off: usize, v: i64) { b[off..off + 8].copy_from_slice(&v.to_le_bytes()); }
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_unaligned(base.add(off) as *mut i64, v); }

#[derive(Default, Clone, Copy)]
struct B { attack: i32, attack_mult: i32, magic_power: i32, magic_power_mult: i32, vamp: i32, attack_speed_mult: i32,
           skill_cooldown_mult: i32, crit_chance: i32, def_pen: i64, mr_pen: i64, range: i64 }
fn buf_of(b: &B) -> Box<Buf<288>> {
    let mut o = Box::new(Buf([0u8; 288]));
    w32(&mut o.0, 0x58, b.attack); w32(&mut o.0, 0x5c, b.attack_mult); w32(&mut o.0, 0x60, b.magic_power);
    w32(&mut o.0, 0x64, b.magic_power_mult); w32(&mut o.0, 0x80, b.vamp); w32(&mut o.0, 0x8c, b.attack_speed_mult);
    w32(&mut o.0, 0x90, b.skill_cooldown_mult); w32(&mut o.0, 0x104, b.crit_chance);
    w64(&mut o.0, 0xa8, b.def_pen); w64(&mut o.0, 0xb0, b.mr_pen); w64(&mut o.0, 0xc8, b.range);
    o
}

/// 독립 재구현(명세 logic) — share·n(60000 내 아군 수)·hp 는 실측값을 넣는다
fn model(b: &B, atk: i64, mp: i64, share: i64, neutral: bool, n_near: usize, fights_back: bool, hp: i64, maxhp: i64) -> i64 {
    let mut v = 0i64;
    if b.attack_mult > 0 { v += (atk * b.attack_mult as i64 / 100).min(40) }
    if b.attack_speed_mult > 0 { v += ((atk * b.attack_speed_mult as i64 / 200) * share / 500).min(40) }
    v += (b.attack as i64).clamp(0, 30);
    v += (b.magic_power as i64).clamp(0, 30);
    if b.magic_power_mult > 0 { v += (mp * b.magic_power_mult as i64 / 100).min(40) }
    if b.crit_chance > 0 { v += (share * (b.crit_chance as i64 / 5) / 500).min(20) }
    if b.def_pen != 0 { v += (b.def_pen / 3).min(20) }
    if b.mr_pen != 0 { v += (b.mr_pen / 3).min(20) }
    if b.range != 0 { v += ((b.range / 3000).max(1) * (atk / 50).max(1)).min(25) }
    if b.skill_cooldown_mult >= 1 && !neutral { if n_near > 1 { v += ((1000 - share) * (b.skill_cooldown_mult as i64 / 5) / 500).min(15) } }
    if b.vamp > 0 && (fights_back || hp < maxhp) { v += (share * (b.vamp as i64 / 5) / 500).min(15) }
    (v / 2).min(40)
}

fn main() {
    // ★game_ai 크레이트를 링크 그래프에 넣기 위한 더미 참조(없으면 LNK2019 — o132 1차 시도 실측)
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

    // 케이스 정의
    let mut b = B::default();
    let (mut atk, mut mp, mut neutral, mut fights_back, mut hp_minus, mut cache_a, mut cache_s) = (200i64, 100i64, false, false, 0i64, 0i64, 0i64);
    let (mut scatter, mut keep_one) = (false, false);
    match case {
        0 => {}
        1 => { b.attack_mult = 50 }                       // 100 → cap 40 → 20
        2 => { b.attack_mult = 10 }                       // 20 → 10
        3 => { b.attack_speed_mult = 100 }                // 200*100/200=100 *500/500=100 → 40 → 20
        4 => { b.attack = 50; b.magic_power = -5 }        // 30 + 0 → 15
        5 => { b.magic_power_mult = 20 }                  // 20 → 10
        6 => { b.crit_chance = 50 }                       // 500*10/500=10 → 5
        7 => { b.def_pen = 30; b.mr_pen = 90 }            // 10 + 20 → 15
        8 => { b.range = 6000 }                           // 2*4=8 → 4
        9 => { b.range = 100000; atk = 1000 }             // 33*20 → 25 → 12
        10 => { b.skill_cooldown_mult = 50 }              // n>1 이면 (1000-500)*10/500=10 → 5
        11 => { b.skill_cooldown_mult = 50; neutral = true }   // Neutral → 0
        12 => { b.skill_cooldown_mult = 0 }               // <1 → 0
        13 => { b.vamp = 50; fights_back = true }         // 10 → 5
        14 => { b.vamp = 50 }                             // hp==max, fights_back=false → 0
        15 => { b.vamp = 50; hp_minus = 1 }               // hp<max → 5
        16 => { b.attack_speed_mult = 100; b.crit_chance = 50; b.skill_cooldown_mult = 50; cache_a = 300; cache_s = 100 } // share=750
        17 => { b.attack_mult = 100; b.magic_power_mult = 100; b.attack = 100; b.magic_power = 100 } // 140 → 70 → 40 cap
        18 => { b.attack_mult = 1; atk = 50 }             // 0 (50*1/100=0)
        19 => { b.crit_chance = 4 }                       // 4/5=0 → 0
        20 => { b.vamp = 50; hp_minus = -1 }              // hp>max, fights_back=false → 0 (ult 비교)
        21 => { b.skill_cooldown_mult = 50; scatter = true } // 아군 4명을 멀리 → n_near=1 → 쿨감 항 0
        22 => { b.skill_cooldown_mult = 50; scatter = true; keep_one = true } // 1명만 60000 안 → n_near=2 → 5
        _ => {}
    }
    let target: &Entity = cache.player_champion[0][2].expect("champ");
    let tp = cache.player_by_champion_id(target.id).expect("player");
    let (tt, tpos) = (tp.info.team, tp.info.position as usize);
    unsafe {
        let ep = target as *const Entity as *mut u8;
        p64(ep, 0x618, atk); p64(ep, 0x620, mp);
        let maxhp = std::ptr::read_unaligned(ep.add(0x628) as *const i64);
        p64(ep, 0x670, maxhp - hp_minus);
        if neutral { p64(ep, 0x0, 1) }
        if scatter { let mut kept = false; for p in 0..5 { if p == 2 { continue } if let Some(e) = cache.player_champion[0][p] {
            if keep_one && !kept { kept = true; continue }
            let q = e as *const Entity as *mut u8; let x = std::ptr::read_unaligned(q.add(0x660) as *const i64); p64(q, 0x660, x + 100000 * (p as i64 + 1)); } } }
        // ChampionCache[team][pos] attack_per_sec(+400) / skill_per_sec(+440)
        let cp = (&cache as *const AbstractGameWithCache as *mut u8).add(0x280 + (tt * 5 + tpos) * 800);
        for i in 0..5 { p64(cp, 400 + i * 8, cache_a); p64(cp, 440 + i * 8, cache_s); }
    }
    // 실측 보조값
    let maxhp = target.stat_cached.hp as i64;
    let hp = target.hp as i64;
    let share = if cache_a + cache_s == 0 { 500 } else { cache_a * 5 * 1000 / ((cache_a + cache_s) * 5) };
    let mut n_near = 0usize;
    for p in 0..5 { if let Some(e) = cache.player_champion[0][p] {
        let dx = (e.x as i64 - target.x as i64).abs() as u64; let dy = (e.y as i64 - target.y as i64).abs() as u64;
        if dx * dx + dy * dy < 3600000001 { n_near += 1 } } }
    let bufb = buf_of(&b);
    let win: [u8; 2] = [fights_back as u8, 0];
    let got = unsafe { nsv(&data as *const OperationData, bufb.0.as_ptr(), target as *const Entity, win.as_ptr()) };
    let exp = model(&b, atk, mp, share, neutral, n_near, fights_back, hp, maxhp);
    println!("case={}\tgot={}\tmodel={}\t{}\t(share={} n_near={} hp={} maxhp={} team={} pos={})",
             case, got, exp, if got == exp { "MATCH" } else { "DIFF" }, share, n_near, hp, maxhp, tt, tpos);
}
