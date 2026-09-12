#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #4 — `/specs[16]` 2차 스윕. o16 에서 **판별력이 없었던 축**을 다시 잡는다.
//!  o16 의 설계 결함: ult 슬롯이 항상 최댓값이라 attack/skill/skill2 의 쿨다운·CastingTarget·None
//!  게이트가 반환값에 나타나지 않았다(전부 5840). ⟹ **슬롯마다 그 슬롯을 유일 최댓값으로 만들어** 다시 잰다.
//!  추가:
//!   (A) `Effect` 4필드 + `Champion` 쿨다운 4필드의 **런타임 오프셋을 주소 산술로 출력**
//!   (B) **실제 비챔프 엔티티**(타워·넥서스·정글)를 champ 로 넘겨 `ty` 14-way switch 팔을 실행으로 확인
//!       (Entity+0x110 Tower / +0xe8 Jungle / tag3 Nexus)
use game_core::*;
use game_ai::plan_legacy::old::max_range_nearly_can_use;
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
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
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
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}
fn mk_effect(range: u64, growth: u64, tgt: CastingTarget) -> Effect {
    let at = AttackEffect { ty: AttackEffectType::Target, damage: 0, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(at) as Arc<dyn EffectType>, range, growth_range: growth,
        start_timing: 0, casting: CastingType::Targeting, target: tgt, attack_type: AttackType::BaseAttack }
}
fn my_radius(e: &Entity) -> u64 {
    let m = e.stat_buff_cached.radius_mult;
    if m == 0 { e.radius as u64 } else { (e.radius as u64) * ((m as u64) + 100) / 100 }
}
fn my_slot(e: &Effect, c: &Entity, t: &Entity) -> u64 {
    e.range + e.growth_range * ((c.level as u64) - 1) + (c.stat_buff_cached.range as u64)
        + e.range_adjust(c, t) + my_radius(c) + my_radius(t)
}
fn my_cd(c: &Entity, slot: usize) -> usize {
    match (&c.ty, slot) {
        (EntityType::Champion(ch), 0) => ch.attack_cooldown,
        (EntityType::Champion(ch), 1) => ch.skill_cooldown,
        (EntityType::Champion(ch), 2) => ch.skill2_cooldown,
        (EntityType::Champion(ch), 3) => ch.ult_cooldown,
        (EntityType::Minion { info: i }, 0) => i.attack_cooldown,
        (EntityType::Tower { info: i }, 0) => i.attack_cooldown,
        (EntityType::Jungle { info: i }, 0) => i.attack_cooldown,
        (EntityType::Epic { info: i }, 0) => i.attack_cooldown,
        (EntityType::Serpen { info: i }, 0) => i.attack_cooldown,
        (EntityType::Ghoul { info: i }, 0) => i.attack_cooldown,
        (EntityType::SmallJiangshi { info: i }, 0) => i.attack_cooldown,
        (EntityType::Bear { info: i }, 0) => i.attack_cooldown,
        (EntityType::Eagle { info: i }, 0) => i.attack_cooldown,
        (EntityType::Revenant { info: i }, 0) => i.attack_cooldown,
        (EntityType::Illusion { info: i }, 0) => i.attack_cooldown,
        _ => 0,
    }
}
fn mine(c: &Entity, t: &Entity, tick: usize) -> u64 {
    let mut range: u64 = 0;
    if let Some(e) = c.attack_effect.as_ref() {
        if e.target.check(c, t) && my_cd(c, 0) <= tick { range = range.max(my_slot(e, c, t)); }
    }
    if let Some(e) = c.skill_effect.as_ref() {
        if e.target.check(c, t) && my_cd(c, 1) <= tick { range = range.max(my_slot(e, c, t)); }
    }
    if c.level >= 3 { if let Some(e) = c.skill2_effect.as_ref() {
        if e.target.check(c, t) && my_cd(c, 2) <= tick { range = range.max(my_slot(e, c, t)); } } }
    if c.level >= 5 { if let Some(e) = c.ult_effect.as_ref() {
        if e.target.check(c, t) && my_cd(c, 3) <= tick { range = range.max(my_slot(e, c, t)); } } }
    range
}
unsafe fn set_cd(c: &mut Entity, a: usize, s: usize, s2: usize, u: usize) {
    if let EntityType::Champion(ref mut ch) = c.ty {
        ch.attack_cooldown = a; ch.skill_cooldown = s; ch.skill2_cooldown = s2; ch.ult_cooldown = u;
    }
}
fn off(base: &Entity, p: *const u8) -> usize { (p as usize) - (base as *const Entity as usize) }

fn main() {
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

    let c0 = cache.player_champion[0][0].unwrap();
    let t0 = cache.player_champion[1][0].unwrap();
    let mut c: Entity = unsafe { std::ptr::read(c0 as *const Entity) };
    let mut t: Entity = unsafe { std::ptr::read(t0 as *const Entity) };
    c.radius = 700; t.radius = 900;
    c.stat_buff_cached.range = 0; c.stat_buff_cached.radius_mult = 0; t.stat_buff_cached.radius_mult = 0;
    unsafe {
        std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 0, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill_effect,  Some(mk_effect(1000, 0, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill2_effect, Some(mk_effect(1000, 0, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.ult_effect,    Some(mk_effect(1000, 0, CastingTarget::EnemyChampion)));
    }
    c.level = 7;

    // ── (A) 런타임 오프셋 — Effect 4필드 + Champion 쿨다운 4필드
    {
        let e = c.attack_effect.as_ref().unwrap();
        println!("eoff_attack\trange=+0x{:x}\tgrowth_range=+0x{:x}\ttarget=+0x{:x}\tcasting=+0x{:x}\tty=+0x{:x}\tstart_timing=+0x{:x}\tattack_type=+0x{:x}",
            off(&c, &e.range as *const _ as *const u8), off(&c, &e.growth_range as *const _ as *const u8),
            off(&c, &e.target as *const _ as *const u8), off(&c, &e.casting as *const _ as *const u8),
            off(&c, &e.ty as *const _ as *const u8), off(&c, &e.start_timing as *const _ as *const u8),
            off(&c, &e.attack_type as *const _ as *const u8));
        let e2 = c.skill_effect.as_ref().unwrap();
        println!("eoff_skill\trange=+0x{:x}\tgrowth_range=+0x{:x}\ttarget=+0x{:x}\tcasting=+0x{:x}",
            off(&c, &e2.range as *const _ as *const u8), off(&c, &e2.growth_range as *const _ as *const u8),
            off(&c, &e2.target as *const _ as *const u8), off(&c, &e2.casting as *const _ as *const u8));
        let e3 = c.skill2_effect.as_ref().unwrap();
        let e4 = c.ult_effect.as_ref().unwrap();
        println!("eoff_s2u\ts2.range=+0x{:x}\ts2.casting=+0x{:x}\tult.range=+0x{:x}\tult.casting=+0x{:x}",
            off(&c, &e3.range as *const _ as *const u8), off(&c, &e3.casting as *const _ as *const u8),
            off(&c, &e4.range as *const _ as *const u8), off(&c, &e4.casting as *const _ as *const u8));
        println!("eoff_rel\tEffect.range=+0x{:x}\tEffect.growth_range=+0x{:x}\tEffect.target=+0x{:x}\tEffect.casting=+0x{:x}",
            (&e.range as *const _ as usize) - (e as *const Effect as usize),
            (&e.growth_range as *const _ as usize) - (e as *const Effect as usize),
            (&e.target as *const _ as usize) - (e as *const Effect as usize),
            (&e.casting as *const _ as usize) - (e as *const Effect as usize));
    }
    if let EntityType::Champion(ref ch) = c.ty {
        println!("cdoff_champion\tattack=+0x{:x}\tskill=+0x{:x}\tskill2=+0x{:x}\tult=+0x{:x}",
            off(&c, &ch.attack_cooldown as *const _ as *const u8),
            off(&c, &ch.skill_cooldown as *const _ as *const u8),
            off(&c, &ch.skill2_cooldown as *const _ as *const u8),
            off(&c, &ch.ult_cooldown as *const _ as *const u8));
    }

    let mut nrun = 0usize; let mut nmatch = 0usize;
    macro_rules! shot { ($tag:expr, $tick:expr) => {{
        let g = max_range_nearly_can_use(&c, &t, $tick);
        let m = mine(&c, &t, $tick);
        nrun += 1; if g == m { nmatch += 1; }
        println!("{}\ttick={}\tgame={}\tmine={}\t{}", $tag, $tick, g, m, if g == m {"MATCH"} else {"**DIFF**"});
    }}; }

    // ── (B) 슬롯별 단독 지배 — 그 슬롯만 크게, 나머지는 작게. 쿨다운 경계 39/40/41
    let big = 50000u64;
    for slot in 0..4usize {
        unsafe {
            std::ptr::write(&mut c.attack_effect, Some(mk_effect(if slot==0 {big} else {1000}, 0, CastingTarget::EnemyChampion)));
            std::ptr::write(&mut c.skill_effect,  Some(mk_effect(if slot==1 {big} else {1000}, 0, CastingTarget::EnemyChampion)));
            std::ptr::write(&mut c.skill2_effect, Some(mk_effect(if slot==2 {big} else {1000}, 0, CastingTarget::EnemyChampion)));
            std::ptr::write(&mut c.ult_effect,    Some(mk_effect(if slot==3 {big} else {1000}, 0, CastingTarget::EnemyChampion)));
        }
        for cd in [0usize, 39, 40, 41, 100000] {
            let (a, s, s2, u) = (if slot==0 {cd} else {0}, if slot==1 {cd} else {0},
                                 if slot==2 {cd} else {0}, if slot==3 {cd} else {0});
            unsafe { set_cd(&mut c, a, s, s2, u); }
            shot!(format!("slot{}_cd{}", slot, cd), 40usize);
        }
        unsafe { set_cd(&mut c, 0, 0, 0, 0); }
        // tick 축 — 이 슬롯 쿨 50 고정, tick 을 쓸어본다(노브0 = 호출부 40/50/60)
        let (a, s, s2, u) = (if slot==0 {50} else {0}, if slot==1 {50} else {0},
                             if slot==2 {50} else {0}, if slot==3 {50} else {0});
        unsafe { set_cd(&mut c, a, s, s2, u); }
        for tk in [40usize, 49, 50, 51, 60] { shot!(format!("slot{}_cd50_tick{}", slot, tk), tk); }
        unsafe { set_cd(&mut c, 0, 0, 0, 0); }
        // CastingTarget 게이트 — 이 슬롯만 Ally 로 바꾼다
        unsafe {
            let e = mk_effect(big, 0, CastingTarget::Ally);
            match slot { 0 => std::ptr::write(&mut c.attack_effect, Some(e)),
                         1 => std::ptr::write(&mut c.skill_effect, Some(e)),
                         2 => std::ptr::write(&mut c.skill2_effect, Some(e)),
                         _ => std::ptr::write(&mut c.ult_effect, Some(e)) }
        }
        shot!(format!("slot{}_targetAlly", slot), 40usize);
        // None 게이트
        unsafe {
            match slot { 0 => std::ptr::write(&mut c.attack_effect, None),
                         1 => std::ptr::write(&mut c.skill_effect, None),
                         2 => std::ptr::write(&mut c.skill2_effect, None),
                         _ => std::ptr::write(&mut c.ult_effect, None) }
        }
        shot!(format!("slot{}_None", slot), 40usize);
    }

    // ── (C) 실제 비챔프 엔티티를 champ 로 — ty 14-way switch 팔 실행 확인
    let mut ids: Vec<(&str, usize)> = Vec::new();
    if let Some(i) = game.world.tower_ids.get(0) { ids.push(("tower", *i)); }
    if let Some(i) = game.world.nexus_ids.get(0) { ids.push(("nexus", *i)); }
    if let Some(i) = game.world.jungle_ids.get(0) { ids.push(("jungle", *i)); }
    if let Some(i) = game.world.minion_ids.get(0) { ids.push(("minion", *i)); }
    for (nm, id) in ids.iter() {
        let src = match cache.game.get_entity_by_id(*id) { Some(e) => e, None => { println!("ent\t{}\tid={}\tNOT_FOUND", nm, id); continue; } };
        let mut x: Entity = unsafe { std::ptr::read(src as *const Entity) };
        let tag = unsafe { ((&x as *const Entity as *const u8).add(0x68) as *const u64).read_unaligned() };
        // info.attack_cooldown 의 런타임 오프셋
        let acoff: i64 = match &x.ty {
            EntityType::Tower { info } => off(&x, &info.attack_cooldown as *const _ as *const u8) as i64,
            EntityType::Jungle { info } => off(&x, &info.attack_cooldown as *const _ as *const u8) as i64,
            EntityType::Minion { info } => off(&x, &info.attack_cooldown as *const _ as *const u8) as i64,
            EntityType::Epic { info } => off(&x, &info.attack_cooldown as *const _ as *const u8) as i64,
            EntityType::Serpen { info } => off(&x, &info.attack_cooldown as *const _ as *const u8) as i64,
            _ => -1,
        };
        println!("ent\t{}\tid={}\tty_tag={}\tattack_cd_off={}\tattack_effect_some={}\tradius={}\tlevel={}",
            nm, id, tag, if acoff >= 0 { format!("+0x{:x}", acoff) } else { "n/a".into() },
            x.attack_effect.is_some(), x.radius, x.level);
        x.radius = 700; x.level = 7;
        x.stat_buff_cached.range = 0; x.stat_buff_cached.radius_mult = 0;
        unsafe {
            std::ptr::write(&mut x.attack_effect, Some(mk_effect(50000, 0, CastingTarget::Both)));
            std::ptr::write(&mut x.skill_effect,  Some(mk_effect(1000, 0, CastingTarget::Both)));
            std::ptr::write(&mut x.skill2_effect, Some(mk_effect(1000, 0, CastingTarget::Both)));
            std::ptr::write(&mut x.ult_effect,    Some(mk_effect(1000, 0, CastingTarget::Both)));
        }
        for cd in [0usize, 40, 41, 100000] {
            match &mut x.ty {
                EntityType::Tower { info } => info.attack_cooldown = cd,
                EntityType::Jungle { info } => info.attack_cooldown = cd,
                EntityType::Minion { info } => info.attack_cooldown = cd,
                EntityType::Epic { info } => info.attack_cooldown = cd,
                EntityType::Serpen { info } => info.attack_cooldown = cd,
                _ => {}
            }
            let g = max_range_nearly_can_use(&x, &t, 40);
            let m = mine(&x, &t, 40);
            nrun += 1; if g == m { nmatch += 1; }
            println!("entcd\t{}\tcd={}\tapi_attack_cooldown={}\tgame={}\tmine={}\t{}",
                nm, cd, x.attack_cooldown(), g, m, if g == m {"MATCH"} else {"**DIFF**"});
        }
        std::mem::forget(x);
    }

    // ── (D) 적 우물 위험 술어 좌표 경계 (4차 R2 의 축 — 17 base_sub_goal 용 보강)
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let me = game.get_player_by_position(0, Position::Top).unwrap();
    for (x, y) in [(913000u64, 15000u64), (900000, 100000), (800000, 200000), (700000, 300000),
                   (600000, 400000), (480000, 480000), (15000, 913000), (959000, 1000)] {
        println!("well\t({},{})\tis_enemy_well_danger={}", x, y, game_ai::is_enemy_well_danger(1, me, x, y));
    }

    println!("TOTAL\t{}/{}\tMATCH", nmatch, nrun);
    println!("DONE\tsetting_ok={}", ok);
    std::mem::forget(c); std::mem::forget(t);
}
