#![allow(unused, dead_code, non_snake_case)]
//! A5-O4 — (a) specs[0] `Entity::radius()` 백분율 기준 100 + `radius_mult as usize`(부호확장)
//!          (b) specs[1] `calculate_jungle_action_score` 계수표·최종식 실행 대조
//! argv: <radius_mult i32>
//! ⚠ `_parameter`(%3, 5384B) · `_action`(%4, 16B) 는 IR 본문에 등장 0회(서명줄에만) 라
//!   제로버퍼 + `assume_init_ref` 로 넘긴다. dereferenceable 만족이 요구사항 전부다.
use game_core::*;
use rand::SeedableRng;
use std::mem::MaybeUninit;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn mk_effect(range: u64, dmg: usize) -> Effect {
    Effect {
        range,
        growth_range: 0,
        start_timing: 0,
        casting: CastingType::Targeting,
        target: CastingTarget::Enemy,
        ty: Arc::new(AttackEffect::new(dmg, 0)) as Arc<dyn EffectType>,
        attack_type: AttackType::Skill,
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let rmult: i32 = a.get(1).map(|s| s.parse().unwrap()).unwrap_or(0);

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

    let my_id = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0].unwrap().id
    };

    // ── (a) radius_mult ──
    {
        let e = game.world.entity.get_mut(my_id).unwrap();
        e.stat_buff_cached.radius_mult = rmult;
    }
    {
        let e = game.world.entity.get(my_id).unwrap();
        let base = e.radius;
        let got = e.radius();
        // sext 가정: (100 + (mult as i64 as u64)) 을 wrapping 으로 곱/나눔
        let sext = (100u64).wrapping_add(rmult as i64 as u64);
        let pred_sext = if rmult == 0 { base as u64 } else { (base as u64).wrapping_mul(sext) / 100 };
        // zext 가정(부호확장 없음)
        let zext = (100u64).wrapping_add(rmult as u32 as u64);
        let pred_zext = if rmult == 0 { base as u64 } else { (base as u64).wrapping_mul(zext) / 100 };
        println!("RADIUS\tmult={}\tradius_field={}\tradius_fn={}\tpred_sext={}\tpred_zext={}\tverdict={}",
                 rmult, base, got, pred_sext, pred_zext,
                 if got as u64 == pred_sext && pred_sext != pred_zext { "SEXT_CONFIRMED" }
                 else if got as u64 == pred_sext { "sext_ok(비판별)" }
                 else if got as u64 == pred_zext { "**ZEXT**" } else { "**MISMATCH**" });
    }

    // ── (b) spec1 계수표 ──
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let champ: &Entity = cache.player_champion[0][0].unwrap();

    println!("WORLD\tchampion_ids={}\ttower_ids={}\tnexus_ids={}\tjungle_ids={}\tminion_ids={}",
             game.world.champion_ids.len(), game.world.tower_ids.len(),
             game.world.nexus_ids.len(), game.world.jungle_ids.len(), game.world.minion_ids.len());

    let zp: MaybeUninit<game_ai::ScoreParameter> = MaybeUninit::zeroed();
    let za: MaybeUninit<Box<dyn Action>> = MaybeUninit::zeroed();
    let parameter: &game_ai::ScoreParameter = unsafe { zp.assume_init_ref() };
    let action: &Box<dyn Action> = unsafe { za.assume_init_ref() };

    let ef = mk_effect(200000, 500);

    let mut cand: Vec<(String, usize)> = Vec::new();
    cand.push(("Champion(enemy Top)".into(), cache.player_champion[1][0].unwrap().id));
    for (i, id) in game.world.tower_ids.iter().enumerate().take(2) { cand.push((format!("Tower#{}", i), *id)); }
    for (i, id) in game.world.nexus_ids.iter().enumerate().take(2) { cand.push((format!("Nexus#{}", i), *id)); }
    for (i, id) in game.world.jungle_ids.iter().enumerate().take(4) { cand.push((format!("Jungle#{}", i), *id)); }
    for (i, id) in game.world.minion_ids.iter().enumerate().take(2) { cand.push((format!("Minion#{}", i), *id)); }

    let dynchamp: &dyn AbstractEntity = champ as &dyn AbstractEntity;

    // ── 정글 캠프를 pub spawn 으로 직접 만든다(월드에 넣지 않고 &Entity 로만 쓴다) ──
    let mut spawned: Vec<(String, Entity)> = Vec::new();
    for team in 0..2usize {
        for jt in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee] {
            let cs = game.mode.jungle_runner.get_camp_state(team, jt);
            let v = cs.spawn(&setting, 0, 5000 + team * 100);
            for (k, e) in v.into_iter().enumerate() {
                spawned.push((format!("Jungle(t{},{:?})#{}", team, jt, k), e));
            }
        }
    }
    // 미니언
    {
        let mut mr = MinionRunner::default();
        let ts = TowerState {
            top_state: [true, true],
            mid_state: [true, true],
            bottom_state: [true, true],
        };
        let v = mr.update(&ctx, ts, 1);
        println!("MINION_SPAWN\tn={}", v.len());
        for (k, e) in v.into_iter().enumerate().take(3) {
            spawned.push((format!("Minion#{}", k), e));
        }
    }
    for (nm, t) in spawned.iter() {
        let tag = unsafe { std::ptr::read_unaligned(((t as *const Entity as usize) + 0x68) as *const i64) };
        let c0 = unsafe { std::ptr::read_unaligned(((t as *const Entity as usize) + 0x98) as *const usize) };
        let hp = t.hp as i64;
        if hp == 0 { println!("SCORE\t{}\ttag={}\thp=0 → skip", nm, tag); continue; }
        let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, t) as i64;
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, t);
        let is_j = tag == 4 && c0 < 2;
        let coef: i64 = match tag {
            3 => 200, 2 => 80,
            4 => if c0 < 2 { if hp <= value { 40 } else { 20 } } else { 0 },
            1 => -10, _ => 0,
        };
        let based: i64 = if is_j { 5 } else { 0 };
        let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp) + based;
        println!("SCORE\t{}\ttag={}\tcamp0={}\thp={}\tvalue={}\tcoef={}\tbased={}\tgame={}\tmine={}\t{}",
                 nm, tag, c0, hp, value, coef, based, got, pred,
                 if got == pred { "MATCH" } else { "**MISMATCH**" });
    }
    // ★hp 를 쓸어 40/20 arm 경계(`hp <= value`) 와 min() 비례 클램프를 실행으로 가른다
    if let Some((_, e0)) = spawned.iter().find(|(n, _)| n.starts_with("Jungle")) {
        for hpv in [1usize, 495, 496, 497, 992, 993, 1000, 10000, 100000] {
            let mut e = e0.clone();
            e.hp = hpv;
            let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, &e) as i64;
            let hp = e.hp as i64;
            let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
            let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, &e);
            let coef: i64 = if hp <= value { 40 } else { 20 };
            let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp) + 5;
            // 반증용 대립가설: `hp < value` (엄격부등호)
            let coef_alt: i64 = if hp < value { 40 } else { 20 };
            let pred_alt = std::cmp::min(coef_alt, coef_alt.wrapping_mul(value) / hp) + 5;
            println!("HPSWEEP\thp={}\tvalue={}\tcoef(le)={}\tgame={}\tmine(hp<=value)={}\talt(hp<value)={}\t{}",
                     hp, value, coef, got, pred, pred_alt,
                     if got == pred && pred != pred_alt { "MATCH(판별)" }
                     else if got == pred { "MATCH" } else { "**MISMATCH**" });
        }
    }

    // ★camp_type.__0 를 2 로 바꿔 '양 팀 캠프 전부' 주장 반증 시도
    if let Some((_, e0)) = spawned.iter().find(|(n, _)| n.starts_with("Jungle")) {
        let mut e = e0.clone();
        unsafe {
            let p = ((&mut e) as *mut Entity as usize + 0x98) as *mut usize;
            std::ptr::write_unaligned(p, 2usize);
        }
        let hp = e.hp as i64;
        let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, &e) as i64;
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, &e);
        println!("SCORE\tJungle(camp0=2 강제)\ttag=4\tcamp0=2\thp={}\tvalue={}\tgame={}\t기대(명세: coef0/based0)=0\t{}",
                 hp, value, got, if got == 0 { "MATCH" } else { "**MISMATCH**" });
        // camp0=1
        unsafe {
            let p = ((&mut e) as *mut Entity as usize + 0x98) as *mut usize;
            std::ptr::write_unaligned(p, 1usize);
        }
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let got1 = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, &e);
        println!("SCORE\tJungle(camp0=1 강제)\tgame={}", got1);
    }

    for (nm, id) in cand {
        let t = match game.world.entity.get(id) { Some(t) => t, None => continue };
        let tag = unsafe { std::ptr::read_unaligned(((t as *const Entity as usize) + 0x68) as *const i64) };
        let hp = t.hp as i64;
        if hp == 0 { println!("SCORE\t{}\thp=0 → skip(div by zero)", nm); continue; }
        let value = Effect::expected_damage_target(&ef, &ctx, dynchamp, t) as i64;
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let got = game_ai::calculate_jungle_action_score(&mut rnd, player, &data, parameter, action, &ef, t);
        // 명세 식 재구현
        let is_jungle = tag == 4;
        let coef: i64 = match tag {
            3 => 200,
            2 => 80,
            4 => if hp <= value { 40 } else { 20 },
            1 => -10,
            _ => 0,
        };
        let based: i64 = if is_jungle { 5 } else { 0 };
        let pred = std::cmp::min(coef, coef.wrapping_mul(value) / hp) + based;
        println!("SCORE\t{}\ttag={}\thp={}\tvalue={}\tcoef={}\tbased={}\tgame={}\tmine={}\t{}",
                 nm, tag, hp, value, coef, based, got, pred,
                 if got == pred { "MATCH" } else { "**MISMATCH**" });
    }
}
