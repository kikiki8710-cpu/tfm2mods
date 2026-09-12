#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치C 오라클 #3b — specs[10] can_enemy_hit_objective 의 25000(선형 여유치)·19600000000(140000² 하드컷)
//! o10.rs 에서 전부 false 가 나온 원인 = 캐스터/오브젝트를 **같은 팀 챔피언**으로 둬서
//! `CastingTarget::check`(m10.ll:47509) 가 먼저 거부한 것. 여기서는 팀·target·casting 을 손으로 맞춘다.
use game_core::*;
use game_ai::plan_legacy::old as old;
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
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
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

fn main() {
    let setting = real_setting();
    println!("setting_ok\ttrue\theight={} tps={} radius={}", setting.height, setting.tick_per_second, setting.champion_radius);
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

    let (mut caster, mut obj) = {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let a = c0.player_champion[0][0].expect("champ0").clone();
        let b = c0.player_champion[1][0].expect("champ1").clone();
        (a, b)
    };
    println!("caster.level={} radius={} eff_range(0x4a0)={:?}", caster.level, caster.radius,
        caster.attack_effect.as_ref().map(|e| e.range));
    println!("caster.attack_effect.target={:?} casting={:?}",
        caster.attack_effect.as_ref().map(|e| e.target),
        caster.attack_effect.as_ref().map(|e| e.casting));

    // 스킬/궁/스킬2 슬롯은 모두 죽이고 attack 슬롯만 남긴다 (한 축만 움직이게)
    caster.skill_effect = None; caster.skill2_effect = None; caster.ult_effect = None;
    caster.level = 1; // skill2(level>2)·ult(level>4) 슬롯 비활성
    caster.x = 0; caster.y = 0;
    if let Some(ref mut e) = caster.attack_effect {
        e.target = CastingTarget::Enemy;
        e.casting = CastingType::Targeting;
        e.growth_range = 0;
    }

    println!("\n### O10b-A attack 슬롯만, range 고정 → margin 을 키우면 경계가 1:1 로 따라가는가");
    println!("eff_range\tmargin\tboundary_d\tdelta_vs_prev");
    for &r in [0u64, 50000, 100000].iter() {
        if let Some(ref mut e) = caster.attack_effect { e.range = r }
        let mut prev: Option<u64> = None;
        for &margin in [0u64, 1000, 25000, 50000, 100000, 200000, 400000, 1000000].iter() {
            let f = |d: u64| -> bool {
                let mut o = obj.clone(); o.x = d; o.y = 0;
                old::can_enemy_hit_objective(&caster, &o, margin)
            };
            if !f(0) { println!("{}\t{}\tNONE(d=0 에서도 false)", r, margin); continue }
            let mut lo = 0u64; let mut hi = 2_000_000u64;
            if f(hi) { println!("{}\t{}\t>=2000000", r, margin); continue }
            while lo + 1 < hi { let m = (lo + hi) / 2; if f(m) { lo = m } else { hi = m } }
            let d = match prev { Some(p) => format!("{}", lo as i64 - p as i64), None => "-".into() };
            println!("{}\t{}\t{}\t{}", r, margin, lo, d);
            prev = Some(lo);
        }
        println!("---");
    }

    println!("\n### O10b-B 140000² 하드컷 — margin 을 아주 크게 주면 경계가 140000 에 고정되는가");
    println!("margin\td\tresult");
    if let Some(ref mut e) = caster.attack_effect { e.range = 900000 }
    for &d in [139998u64, 139999, 140000, 140001, 140002].iter() {
        let mut o = obj.clone(); o.x = d; o.y = 0;
        println!("{}\t{}\t{}", 900000, d, old::can_enemy_hit_objective(&caster, &o, 900000));
    }
    // 대각선으로도 같은 제곱합인지(dx²+dy² 인지 max 인지 구분)
    println!("--- 대각선 (dx=dy=k): 제곱합이면 경계 k = floor(140000/sqrt(2)) = 98994");
    for &k in [98994u64, 98995, 98996].iter() {
        let mut o = obj.clone(); o.x = k; o.y = k;
        println!("k={}\tdx2+dy2={}\t{}", k, 2 * k * k, old::can_enemy_hit_objective(&caster, &o, 900000));
    }

    println!("\n### O10b-C 레벨 게이트 — skill2(level>2) / ult(level>4)");
    println!("level\tonly_skill2\tonly_ult");
    {
        let c2 = {
            let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            c0.player_champion[0][0].unwrap().clone()
        };
        for lv in 1..7usize {
            // skill2 만 살린 캐스터
            let mut a = c2.clone();
            a.x = 0; a.y = 0; a.level = lv;
            a.attack_effect = None; a.skill_effect = None; a.ult_effect = None;
            if let Some(ref mut e) = a.skill2_effect {
                e.target = CastingTarget::Enemy; e.casting = CastingType::Targeting;
                e.range = 100000; e.growth_range = 0;
            }
            // ult 만 살린 캐스터
            let mut b = c2.clone();
            b.x = 0; b.y = 0; b.level = lv;
            b.attack_effect = None; b.skill_effect = None; b.skill2_effect = None;
            if let Some(ref mut e) = b.ult_effect {
                e.target = CastingTarget::Enemy; e.casting = CastingType::Targeting;
                e.range = 100000; e.growth_range = 0;
            }
            let mut o = obj.clone(); o.x = 50000; o.y = 0;
            println!("{}\t{}\t{}", lv, old::can_enemy_hit_objective(&a, &o, 0), old::can_enemy_hit_objective(&b, &o, 0));
        }
    }

    println!("\n### O10b-D is_enemy_well_danger 사각형 — team0 player / team1 player");
    {
        let p0 = game.get_player_by_position(0usize, Position::Top).unwrap();
        let p1 = game.get_player_by_position(1usize, Position::Top).unwrap();
        println!("player_team\tx\ty\tresult");
        for (nm, p) in [("t0", p0), ("t1", p1)].iter() {
            for &(x, y) in [(0u64, 800000u64), (0, 799999), (64000, 960000), (64001, 960000),
                            (63999, 959999), (0, 896000), (159999, 959999), (160000, 896000),
                            (960000, 0), (896000, 0), (896000, 159999), (960000, 160000),
                            (960000, 160001), (895999, 0)].iter() {
                println!("{}\t{}\t{}\t{}", nm, x, y, game_ai::is_enemy_well_danger(0, p, x, y));
            }
        }
    }
    println!("\n### O10b-E 상수항 20000 의 정체 — 반지름인가 (range=0, margin=0)");
    println!("caster.radius\tobj.radius\tboundary_d");
    if let Some(ref mut e) = caster.attack_effect { e.range = 0 }
    for &(cr, orr) in [(10000u64, 10000u64), (0, 10000), (10000, 0), (0, 0),
                       (30000, 10000), (10000, 30000)].iter() {
        let mut c2 = caster.clone(); c2.radius = cr as usize;
        let f = |d: u64| -> bool {
            let mut o = obj.clone(); o.x = d; o.y = 0; o.radius = orr as usize;
            old::can_enemy_hit_objective(&c2, &o, 0)
        };
        if !f(0) { println!("{}\t{}\tNONE", cr, orr); continue }
        let mut lo = 0u64; let mut hi = 2_000_000u64;
        if f(hi) { println!("{}\t{}\t>=2000000", cr, orr); continue }
        while lo + 1 < hi { let m = (lo + hi) / 2; if f(m) { lo = m } else { hi = m } }
        println!("{}\t{}\t{}", cr, orr, lo);
    }

    println!("\nDONE");
}
