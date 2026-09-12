#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #3 — `/specs[16]` `max_range_nearly_can_use`(battle.rs:2397) 를 **실행**해서
//! consts/knobs/mem 을 진리표로 대조한다(ev4 → ev2). `game` = SDK 함수 반환, `mine` = 내 재현.
//!
//! 3차·4차가 못 한 이유 = 「`SwordmanChampionInfo::default()` 는 이펙트가 비어 전 구간 0」.
//! 해법(TEMPLATE 함정④) = **`AttackEffect`(72B, 전 필드 pub) + `Effect`(7필드 전부 pub) 직접 조립.**
//! ★추가로 확인된 사실: `game_core::Entity` 의 **41필드 전부 `pub`**(tcx) 이라
//!   `ptr::read` 로 복제한 Entity 의 이펙트·레벨·반경·쿨다운을 **그대로 대입**할 수 있다.
//!   (복제본은 Arc 를 증가시키지 않으므로 ①덮어쓸 때 `ptr::write`(옛 값 drop 안 함) ②끝에 `mem::forget`)
//!
//! TLS 메모(브리핑 함정③) 해당 없음 — 이 함수는 `check_kill_die_tick` 경로를 안 탄다.
//! 그래도 **정방향/역방향 2회 스윕**으로 순서 의존을 같이 찍는다.
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
    let at = AttackEffect {
        ty: AttackEffectType::Target,
        damage: 0, attack_ratio: 0, hp_ratio: 0, target_hp_ratio: 0,
        cc_damage: 0, cc_damage_attack_ratio: 0, shared: false,
    };
    Effect {
        ty: Arc::new(at) as Arc<dyn EffectType>,
        range, growth_range: growth, start_timing: 0,
        casting: CastingType::Targeting,
        target: tgt,
        attack_type: AttackType::BaseAttack,
    }
}

// ───────── 내 재현 (명세 `logic` 대로. 게임 헬퍼는 callee 로만 호출) ─────────
fn my_radius(e: &Entity) -> u64 {
    let m = e.stat_buff_cached.radius_mult;
    if m == 0 { e.radius as u64 } else { (e.radius as u64) * ((m as u64) + 100) / 100 }
}
fn my_erange(e: &Effect, c: &Entity) -> u64 {
    e.range + e.growth_range * ((c.level as u64) - 1) + (c.stat_buff_cached.range as u64)
}
fn my_slot(e: &Effect, c: &Entity, t: &Entity) -> u64 {
    my_erange(e, c) + e.range_adjust(c, t) + my_radius(c) + my_radius(t)
}
fn is_champ(c: &Entity) -> bool { matches!(c.ty, EntityType::Champion(_)) }
fn my_cd(c: &Entity, slot: usize) -> usize {
    // Entity::{attack,skill,skill2,ult}_cooldown 의 재현:
    //  attack = ty 별 14-way (Champion 이면 Champion.0.attack_cooldown)
    //  skill/skill2/ult = ty==Champion 일 때만 각 필드, 아니면 0
    match (&c.ty, slot) {
        (EntityType::Champion(ch), 0) => ch.attack_cooldown,
        (EntityType::Champion(ch), 1) => ch.skill_cooldown,
        (EntityType::Champion(ch), 2) => ch.skill2_cooldown,
        (EntityType::Champion(ch), 3) => ch.ult_cooldown,
        (EntityType::Minion { info: i }, 0) => i.attack_cooldown,
        (EntityType::Tower { info: i }, 0) => i.attack_cooldown,
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
    // skill2 는 level >= 3 일 때만 Some (내가 직접 게이트한다 — 게임 getter 를 안 쓴다)
    if c.level >= 3 {
        if let Some(e) = c.skill2_effect.as_ref() {
            if e.target.check(c, t) && my_cd(c, 2) <= tick { range = range.max(my_slot(e, c, t)); }
        }
    }
    // ult 는 level >= 5 일 때만 Some
    if c.level >= 5 {
        if let Some(e) = c.ult_effect.as_ref() {
            if e.target.check(c, t) && my_cd(c, 3) <= tick { range = range.max(my_slot(e, c, t)); }
        }
    }
    range
}

unsafe fn set_cd(c: &mut Entity, a: usize, s: usize, s2: usize, u: usize) {
    if let EntityType::Champion(ref mut ch) = c.ty {
        ch.attack_cooldown = a; ch.skill_cooldown = s;
        ch.skill2_cooldown = s2; ch.ult_cooldown = u;
    }
}

fn main() {
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    println!("towers\t{}\ttwin0={}\ttwin1={}",
             game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());

    println!("size\tEffect={}\tOption<Effect>={}\tEntity={}\tEntityType={}\tChampion={}",
        std::mem::size_of::<Effect>(), std::mem::size_of::<Option<Effect>>(),
        std::mem::size_of::<Entity>(), std::mem::size_of::<EntityType>(),
        std::mem::size_of::<Champion>());

    // Option<Effect> 니치 실측 — None 일 때 +0x30(casting) 바이트
    {
        let n: Option<Effect> = None;
        let p = &n as *const Option<Effect> as *const u8;
        let v = unsafe { (p.add(0x30) as *const i32).read_unaligned() };
        let s: Option<Effect> = Some(mk_effect(1, 0, CastingTarget::EnemyChampion));
        let p2 = &s as *const Option<Effect> as *const u8;
        let v2 = unsafe { (p2.add(0x30) as *const i32).read_unaligned() };
        println!("niche\tNone_casting_i32={}\tSome(Targeting)_casting_i32={}", v, v2);
    }

    let c0 = cache.player_champion[0][0].unwrap();
    let t0 = cache.player_champion[1][0].unwrap();
    // ★Entity 복제: 전 필드 pub 이지만 Arc 를 증가시키지 않으므로 덮어쓸 때 ptr::write, 끝에 forget
    let mut c: Entity = unsafe { std::ptr::read(c0 as *const Entity) };
    let mut t: Entity = unsafe { std::ptr::read(t0 as *const Entity) };
    let cp = &c as *const Entity as *const u8;
    println!("base\tc.id={}\tc.level={}\tc.radius={}\tc.ty_tag={}\tt.radius={}\tc.sbc.range={}\tc.sbc.radius_mult={}",
        c.id, c.level, c.radius,
        unsafe { (cp.add(0x68) as *const u64).read_unaligned() }, t.radius,
        c.stat_buff_cached.range, c.stat_buff_cached.radius_mult);
    println!("base_cd\tattack={}\tskill={}\tskill2={}\tult={}\tattack_effect_is_some={}\tskill2_getter_is_some={}",
        c.attack_cooldown(), c.skill_cooldown(), c.skill2_cooldown(), c.ult_cooldown(),
        c.attack_effect.is_some(), c.skill2_effect().is_some());

    // 네 슬롯에 구분 가능한 사거리를 심는다 (attack 1000/g10 · skill 2000/g20 · skill2 3000/g30 · ult 4000/g40)
    unsafe {
        std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 10, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill_effect,  Some(mk_effect(2000, 20, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill2_effect, Some(mk_effect(3000, 30, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.ult_effect,    Some(mk_effect(4000, 40, CastingTarget::EnemyChampion)));
    }
    c.radius = 700; t.radius = 900;
    c.stat_buff_cached.range = 0;
    c.stat_buff_cached.radius_mult = 0;
    t.stat_buff_cached.radius_mult = 0;
    unsafe { set_cd(&mut c, 0, 0, 0, 0); }

    println!("effect_offsets\tattack_effect@+0x{:x}\tskill_effect@+0x{:x}\tskill2_effect@+0x{:x}\tult_effect@+0x{:x}\tlevel@+0x{:x}\tradius@+0x{:x}\tsbc.range@+0x{:x}\tsbc.radius_mult@+0x{:x}",
        (&c.attack_effect as *const _ as usize) - (&c as *const _ as usize),
        (&c.skill_effect  as *const _ as usize) - (&c as *const _ as usize),
        (&c.skill2_effect as *const _ as usize) - (&c as *const _ as usize),
        (&c.ult_effect    as *const _ as usize) - (&c as *const _ as usize),
        (&c.level         as *const _ as usize) - (&c as *const _ as usize),
        (&c.radius        as *const _ as usize) - (&c as *const _ as usize),
        (&c.stat_buff_cached.range as *const _ as usize) - (&c as *const _ as usize),
        (&c.stat_buff_cached.radius_mult as *const _ as usize) - (&c as *const _ as usize));
    println!("range_adjust\tattack={}\tEffect::range(level{})={}",
        c.attack_effect.as_ref().unwrap().range_adjust(&c, &t), c.level,
        c.attack_effect.as_ref().unwrap().range(&c));

    let mut nrun = 0usize; let mut nmatch = 0usize;
    macro_rules! shot {
        ($tag:expr, $tick:expr) => {{
            let g = max_range_nearly_can_use(&c, &t, $tick);
            let m = mine(&c, &t, $tick);
            nrun += 1; if g == m { nmatch += 1; }
            println!("{}\ttick={}\tgame={}\tmine={}\t{}\tlvl={}\tcd=({},{},{})\tsbcR={}\trmC={}\trmT={}",
                $tag, $tick, g, m, if g == m { "MATCH" } else { "**DIFF**" },
                c.level, c.attack_cooldown(), c.skill_cooldown(), c.ult_cooldown(),
                c.stat_buff_cached.range, c.stat_buff_cached.radius_mult, t.stat_buff_cached.radius_mult);
        }};
    }

    // ── (1) 레벨 축: skill2(level>=3)·ult(level>=5) 개방 게이트
    for lv in [1usize, 2, 3, 4, 5, 6, 7, 18] {
        c.level = lv;
        println!("lvlgate\tlevel={}\tskill2_getter_some={}\tult_getter_some={}",
            lv, c.skill2_effect().is_some(), c.ult_effect().is_some());
        shot!("L", 40usize);
    }

    // ── (2) 쿨다운 경계 (tick=40). 레벨 7 = 네 슬롯 전부 개방
    c.level = 7;
    for cd in [0usize, 39, 40, 41, 1000] {
        unsafe { set_cd(&mut c, cd, 0, 0, 0); } shot!(format!("cdA{}", cd), 40usize);
        unsafe { set_cd(&mut c, 0, cd, 0, 0); } shot!(format!("cdS{}", cd), 40usize);
        unsafe { set_cd(&mut c, 0, 0, cd, 0); } shot!(format!("cdS2_{}", cd), 40usize);
        unsafe { set_cd(&mut c, 0, 0, 0, cd); } shot!(format!("cdU{}", cd), 40usize);
    }
    unsafe { set_cd(&mut c, 0, 0, 0, 0); }

    // ── (3) tick 축(노브0: 호출부 리터럴 40/50/60)
    unsafe { set_cd(&mut c, 45, 55, 0, 0); }
    for tk in [0usize, 39, 40, 44, 45, 50, 54, 55, 60] { shot!(format!("tk{}", tk), tk); }
    unsafe { set_cd(&mut c, 0, 0, 0, 0); }

    // ── (4) radius_mult / 100 기준
    for (rc, rt) in [(0i32, 0i32), (50, 0), (0, 50), (100, 100), (-50, 0), (1, 0)] {
        c.stat_buff_cached.radius_mult = rc; t.stat_buff_cached.radius_mult = rt;
        shot!(format!("rm{}_{}", rc, rt), 40usize);
    }
    c.stat_buff_cached.radius_mult = 0; t.stat_buff_cached.radius_mult = 0;

    // ── (5) stat_buff_cached.range 축
    for sr in [0usize, 1, 777, 100000] { c.stat_buff_cached.range = sr; shot!(format!("sbc{}", sr), 40usize); }
    c.stat_buff_cached.range = 0;

    // ── (6) CastingTarget 축 (attack 슬롯만 바꾼다)
    for tg in [CastingTarget::EnemyChampion, CastingTarget::Enemy, CastingTarget::Ally,
               CastingTarget::AllyChampion, CastingTarget::None, CastingTarget::Both,
               CastingTarget::EnemyChampionInCC, CastingTarget::EnemyChampionRecentlyAttacked] {
        unsafe { std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 10, tg))); }
        let chk = c.attack_effect.as_ref().unwrap().target.check(&c, &t);
        println!("ctgt\t{:?}\tcheck={}", tg, chk);
        shot!(format!("ct{:?}", tg), 40usize);
    }
    unsafe { std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 10, CastingTarget::EnemyChampion))); }

    // ── (7) None 슬롯 축 — 각 슬롯을 None 으로 지워본다
    let variants: [(&str, usize); 4] = [("noA", 0), ("noS", 1), ("noS2", 2), ("noU", 3)];
    for (nm, which) in variants {
        unsafe {
            std::ptr::write(&mut c.attack_effect, if which == 0 { None } else { Some(mk_effect(1000, 10, CastingTarget::EnemyChampion)) });
            std::ptr::write(&mut c.skill_effect,  if which == 1 { None } else { Some(mk_effect(2000, 20, CastingTarget::EnemyChampion)) });
            std::ptr::write(&mut c.skill2_effect, if which == 2 { None } else { Some(mk_effect(3000, 30, CastingTarget::EnemyChampion)) });
            std::ptr::write(&mut c.ult_effect,    if which == 3 { None } else { Some(mk_effect(4000, 40, CastingTarget::EnemyChampion)) });
        }
        shot!(nm, 40usize);
    }
    // 전부 None
    unsafe {
        std::ptr::write(&mut c.attack_effect, None); std::ptr::write(&mut c.skill_effect, None);
        std::ptr::write(&mut c.skill2_effect, None); std::ptr::write(&mut c.ult_effect, None);
    }
    shot!("allNone", 40usize);
    // 복구
    unsafe {
        std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 10, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill_effect,  Some(mk_effect(2000, 20, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.skill2_effect, Some(mk_effect(3000, 30, CastingTarget::EnemyChampion)));
        std::ptr::write(&mut c.ult_effect,    Some(mk_effect(4000, 40, CastingTarget::EnemyChampion)));
    }

    // ── (8) max 의 방향 — attack 을 최대로 두면 뒤 슬롯이 덮지 않는가
    unsafe { std::ptr::write(&mut c.attack_effect, Some(mk_effect(90000, 0, CastingTarget::EnemyChampion))); }
    shot!("maxA", 40usize);
    unsafe { set_cd(&mut c, 1000, 0, 0, 0); }
    shot!("maxA_cdblocked", 40usize);   // 주의2) 앞 슬롯이 막혀도 뒤 슬롯 값은 남는다
    unsafe { set_cd(&mut c, 0, 1000, 1000, 1000); }
    shot!("maxA_others_blocked", 40usize);
    unsafe { set_cd(&mut c, 0, 0, 0, 0); }
    unsafe { std::ptr::write(&mut c.attack_effect, Some(mk_effect(1000, 10, CastingTarget::EnemyChampion))); }

    // ── (9) ty 축 — Champion(13) 이 아니면 스킬 쿨 게이트가 사라지는가 (consts[1]=13 / consts[6]=3)
    let saved_ty: EntityType = unsafe { std::ptr::read(&c.ty as *const EntityType) };
    unsafe { set_cd(&mut c, 9999, 9999, 9999, 9999); }
    shot!("tyChampion_cd9999", 40usize);
    for (nm, ty) in [("Nexus", EntityType::Nexus), ("None", EntityType::None)] {
        unsafe { std::ptr::write(&mut c.ty, ty); }
        println!("tycd\t{}\tattack_cd={}\tskill_cd={}\tskill2_cd={}\tult_cd={}",
            nm, c.attack_cooldown(), c.skill_cooldown(), c.skill2_cooldown(), c.ult_cooldown());
        shot!(format!("ty{}", nm), 40usize);
    }
    unsafe { std::ptr::write(&mut c.ty, saved_ty); }
    unsafe { set_cd(&mut c, 0, 0, 0, 0); }

    // ── (10) 순서 역전 재현성 (TLS 오염 징후)
    let mut fw: Vec<u64> = Vec::new();
    for lv in 1..=8usize { c.level = lv; fw.push(max_range_nearly_can_use(&c, &t, 40)); }
    let mut bw: Vec<u64> = Vec::new();
    for lv in (1..=8usize).rev() { c.level = lv; bw.push(max_range_nearly_can_use(&c, &t, 40)); }
    bw.reverse();
    println!("reorder\tfw={:?}\tbw={:?}\tsame={}", fw, bw, fw == bw);

    println!("TOTAL\t{}/{}\tMATCH", nmatch, nrun);
    println!("DONE\tsetting_ok={}", ok);
    std::mem::forget(c);
    std::mem::forget(t);
}
