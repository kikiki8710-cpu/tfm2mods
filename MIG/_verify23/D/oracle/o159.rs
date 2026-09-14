#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #159 battle_ally_action (pub · game_ai::battle_ally_action 직접 호출) + 독립 재구현 대조
//! champ.skill_effect(+0x4c8)/skill2_effect(+0x500) 에 조립한 Effect 를 raw write. near_allies 경계(dist² < 150000² 엄격) · 사거리+walk*ms · 태그 16/17 · 원소 +8 target id
//! 사용: o159.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

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
unsafe fn p32(base: *mut u8, off: usize, v: i32) { std::ptr::write_volatile(base.add(off) as *mut i32, v); }
unsafe fn r64(base: *const u8, off: usize) -> i64 { std::ptr::read_volatile(base.add(off) as *const i64) }
unsafe fn r8(base: *const u8, off: usize) -> u8 { std::ptr::read_volatile(base.add(off)) }
fn mk_effect(range: u64, growth: u64, casting: CastingType, target: CastingTarget) -> Effect {
    Effect { range, growth_range: growth, start_timing: 0, casting, target,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, attack_type: AttackType::Skill }
}
fn d2(a: &Entity, b: &Entity) -> u64 {
    let dx = (a.x as i64 - b.x as i64).unsigned_abs(); let dy = (a.y as i64 - b.y as i64).unsigned_abs();
    dx * dx + dy * dy
}

fn main() {
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
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let ver = 57usize;

    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ: &Entity = cache.player_champion[0][2].expect("champ");
    // (skill: Option<(range, target)>, skill2: Option<(range,target)>, level, ally_dx: [i64;5] (0=그대로, 아니면 champ.x+dx / y 같게), champ_ms)
    let mut skill: Option<(u64, CastingTarget)> = None;
    let mut skill2: Option<(u64, CastingTarget)> = None;
    let mut level: i64 = 1;
    let mut adx: [i64; 5] = [0; 5];
    let mut champ_ms: i64 = 0;
    match case {
        0 => {}                                                                       // skill None → 빈 Vec
        1 => { skill = Some((150000, CastingTarget::AllyNotSelf)); }                  // 기본 배치 아군 → 사거리 안이면 Skill
        2 => { skill = Some((20000, CastingTarget::AllyNotSelf)); adx = [5000, 30000, 0, 60000, 120000]; }   // 사거리 20000+walk → 가까운 것만
        3 => { skill = Some((300000, CastingTarget::AllyNotSelf)); adx = [150000, 149999, 0, 150001, 100000]; } // near_allies 경계: 150000/150001 제외 · 149999 포함
        4 => { skill2 = Some((150000, CastingTarget::AllyNotSelf)); level = 3; }      // skill2 만(레벨 3) → 태그 17
        5 => { skill2 = Some((150000, CastingTarget::AllyNotSelf)); level = 1; }      // 레벨 1 → skill2 None → 빈
        6 => { skill = Some((30000, CastingTarget::AllyNotSelf)); skill2 = Some((100000, CastingTarget::AllyNotSelf)); level = 3; adx = [5000, 25000, 0, 50000, 90000]; } // 둘 다
        7 => { skill = Some((10000, CastingTarget::AllyNotSelf)); adx = [10000, 40000, 0, 70000, 100000]; champ_ms = 1000; }  // walk 30*ms=30000 가산 → 40000 까지 포함
        8 => { skill = Some((150000, CastingTarget::Enemy)); }                        // check 실패(아군에게 Enemy) → 빈
        9 => { skill = Some((10000, CastingTarget::AllyNotSelf)); adx = [10000, 40000, 0, 70000, 100000]; champ_ms = 999; }  // walk 29970 → 40000 제외(경계)
        _ => {}
    }
    let chp = champ as *const Entity as *mut u8;
    unsafe {
        p64(chp, 0x5c8, level);
        if champ_ms != 0 { p64(chp, 0x640, champ_ms); }
        match skill { None => p32(chp, 0x4c8 + 0x30, -1), Some((r, t)) => std::ptr::write(chp.add(0x4c8) as *mut Option<Effect>, Some(mk_effect(r, 0, CastingType::Targeting, t))) }
        match skill2 { None => p32(chp, 0x500 + 0x30, -1), Some((r, t)) => std::ptr::write(chp.add(0x500) as *mut Option<Effect>, Some(mk_effect(r, 0, CastingType::Targeting, t))) }
        for p in 0..5 { if adx[p] != 0 { if let Some(e) = cache.player_champion[0][p] {
            let ep = e as *const Entity as *mut u8; p64(ep, 0x660, champ.x as i64 + adx[p]); p64(ep, 0x668, champ.y as i64);
        } } }
    }
    // ── 모델 ──
    let mut near: Vec<&Entity> = Vec::new();
    for p in 0..5 { if let Some(e) = cache.player_champion[0][p] { if e.id != champ.id && d2(e, champ) < 22500000000 { near.push(e); } } }
    let mut model: Vec<(u8, usize)> = Vec::new();
    let mv = champ.stat_cached.move_speed as u64;
    let mut walk_used = 0u64;
    for (which, eff) in [(16u8, champ.skill_effect.as_ref()), (17u8, if champ.level > 2 { champ.skill2_effect.as_ref() } else { None })] {
        let Some(eff) = eff else { continue };
        let can = if which == 16 { champ.can_skill() } else { champ.can_skill2() };
        if !can { continue; }
        let walk: u64 = if EffectType::expected_buff_deep(&*eff.ty, &ctx, champ).is_none() { 30 } else { 90 };
        walk_used = walk;
        for e in near.iter() {
            if !eff.target.check(champ, e) { continue; }
            let maxd = Effect::range(eff, champ) as u64 + walk * mv + eff.range_adjust(champ, e) as u64 + champ.radius() as u64 + e.radius() as u64;
            if d2(e, champ) > maxd * maxd { continue; }
            model.push((which, e.id));
        }
        // 자기 대상: AllyNotSelf/Enemy 면 check(champ,champ) 가 거부 → 모델 밖(should_add_self_* 미재현)
        if eff.target.check(champ, champ) { model.push((which, usize::MAX)); }
    }
    let got: bumpalo::collections::Vec<game_ai::SmallActionPlay> = game_ai::battle_ally_action(ver, &mut rnd, player, &data, 0);
    let mut got_v: Vec<(u8, usize)> = Vec::new();
    unsafe {
        let base = got.as_ptr() as *const u8;
        for i in 0..got.len() { let ep = base.add(i * 184); got_v.push((r8(ep, 0xb1), r64(ep, 8) as usize)); }
    }
    let ok = got_v == model;
    let dists: Vec<u64> = near.iter().map(|e| game_core::utils::isqrt(d2(e, champ) as i64) as u64).collect();
    println!("case={}\tgot={:?}\tmodel={:?}\t{}\t(near={} dists={:?} walk={} ms={} can_skill={} can_skill2={} lvl={})",
             case, got_v, model, if ok { "MATCH" } else { "DIFF" }, near.len(), dists, walk_used, mv, champ.can_skill(), champ.can_skill2(), champ.level);
}
