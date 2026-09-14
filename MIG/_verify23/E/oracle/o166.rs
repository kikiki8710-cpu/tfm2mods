#![allow(unused, dead_code, non_snake_case)]
//! 23차 E · 166 utils::can1v1win 오라클 (pub 직접 호출). 한 프로세스 = 한 케이스(argv[1]).
//! 예측식(명세 logic) = my_dps = Σ dmg_i*100000/max(cool_i,1) (attack/skill/skill2(level>2 만)) ;
//!   my_die = my_hp*100000/enemy_dps ; enemy_die = enemy_hp*100000/my_dps ; enemy_hp = 가시면 hp 아니면 stat_cached.hp ;
//!   return my_die > enemy_die + 60.  ult_effect 는 무시.
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
fn mkeff(damage: usize, atk_ty: AttackType) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 100000, growth_range: 0, start_timing: 0,
        target: CastingTarget::Enemy, attack_type: atk_ty, casting: CastingType::Targeting }
}
fn ids(g: &Game, team: usize) -> Vec<usize> {
    g.world.champion_ids.iter().cloned()
        .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(t)) if t == team))
        .collect()
}

// 케이스: (my_atk, my_sk, my_sk2, my_lv, my_hp, en_atk, en_sk, en_sk2, en_lv, en_hp, en_maxhp, en_visible, my_ult, en_ult)
fn case(n: u32) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, bool, usize, usize) {
    match n {
        0 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),      // 대칭 → false (my_die == en_die)
        1 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 50000, 50000, true, 0, 0),        // 내 hp 2배 → true
        2 => (100, 0, 0, 5, 50000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),       // 상대 hp 2배 → false
        3 => (100, 200, 0, 5, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),    // 내 skill 추가 → true
        4 => (100, 0, 300, 3, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),    // 내 skill2, level 3 → 반영 → true
        5 => (100, 0, 300, 2, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),    // 내 skill2, level 2 → 미반영 → false
        6 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 20000, 100000, true, 0, 0),       // 상대 가시 · hp 20000 → true
        7 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 20000, 100000, false, 0, 0),      // 상대 비가시 → max hp 100000 사용 → false
        8 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 100000, 100000, true, 9999, 0),   // 내 ult 거대 → 무시 → false
        9 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 9999),   // 상대 ult 거대 → 무시 → false
        10 => (100, 0, 0, 5, 100000, 0, 100, 0, 5, 100000, 100000, true, 0, 0),     // 상대 attack None·skill 만 → 계산 진행
        11 => (0, 100, 0, 5, 100000, 100, 0, 0, 5, 100000, 100000, true, 0, 0),     // 내 attack None·skill 만
        // 경계: 상대 hp 를 조절해 my_die − en_die 가 60 근처(dps 3333333 이면 die = hp*3/100). 실행 시 cooltime 으로 pred 계산
        12 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 98000, 98000, true, 0, 0),       // diff 60 → false
        13 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 97960, 97960, true, 0, 0),       // diff 62 → true
        14 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 98034, 98034, true, 0, 0),       // diff 59 → false
        15 => (100, 0, 0, 5, 100000, 100, 0, 0, 5, 97967, 97967, true, 0, 0),       // diff 61 → true
        _ => (100, 0, 0, 5, 1000, 100, 0, 0, 5, 1000, 1000, true, 0, 0),
    }
}

fn main() {
    let n: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    println!("setting_ok\t{}\ttps={}", setting.width != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0, setting.tick_per_second);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let g = mkgame(&setting, &ms, &map, &ctx);
    let (ma, msk, msk2, mlv, mhp, ea, esk, esk2, elv, ehp, emax, evis, mult, eult) = case(n);
    // ★캐시는 attack_effect None 인 챔피언이 있으면 new() 에서 unwrap 패닉(simulation.rs:1603) → 캐시를 먼저 만들고 raw ptr 로 세팅
    let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].unwrap();
    let enemy = cache.player_champion[1][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let ep = enemy as *const Entity as *mut Entity;
    unsafe {
        let e = cp;
        std::ptr::write(&mut (*e).attack_effect, if ma > 0 { Some(mkeff(ma, AttackType::BaseAttack)) } else { None });
        std::ptr::write(&mut (*e).skill_effect, if msk > 0 { Some(mkeff(msk, AttackType::Skill)) } else { None });
        std::ptr::write(&mut (*e).skill2_effect, if msk2 > 0 { Some(mkeff(msk2, AttackType::Skill)) } else { None });
        std::ptr::write(&mut (*e).ult_effect, if mult > 0 { Some(mkeff(mult, AttackType::Skill)) } else { None });
        std::ptr::write_volatile(&mut (*e).level, mlv);
        std::ptr::write_volatile(&mut (*e).hp, mhp);
        std::ptr::write_volatile(&mut (*e).stat_cached.hp, mhp);
        let e = ep;
        std::ptr::write(&mut (*e).attack_effect, if ea > 0 { Some(mkeff(ea, AttackType::BaseAttack)) } else { None });
        std::ptr::write(&mut (*e).skill_effect, if esk > 0 { Some(mkeff(esk, AttackType::Skill)) } else { None });
        std::ptr::write(&mut (*e).skill2_effect, if esk2 > 0 { Some(mkeff(esk2, AttackType::Skill)) } else { None });
        std::ptr::write(&mut (*e).ult_effect, if eult > 0 { Some(mkeff(eult, AttackType::Skill)) } else { None });
        std::ptr::write_volatile(&mut (*e).level, elv);
        std::ptr::write_volatile(&mut (*e).hp, ehp);
        std::ptr::write_volatile(&mut (*e).stat_cached.hp, emax);
        std::ptr::write(&mut (*e).visible_state[0], if evis { VisibleState::Visible } else { VisibleState::Unknown });
    }
    let champ: &Entity = unsafe { &*cp };
    let enemy: &Entity = unsafe { &*ep };
    let player = g.get_player_by_position(0, Position::Top).unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);

    // 예측 (콜리 pub 함수를 그대로 써서 합성만 검증)
    let dmg = |c: &Entity, t: &Entity, eff: &Option<Effect>| -> usize {
        eff.as_ref().map(|e| e.expected_damage_target(&ctx, c as &dyn AbstractEntity, t)).unwrap_or(0)
    };
    let sk2 = |e: &Entity| -> Option<Effect> { if e.level > 2 { e.skill2_effect.clone() } else { None } };
    let my_a = dmg(champ, enemy, &champ.attack_effect); let my_s = dmg(champ, enemy, &champ.skill_effect); let my_s2 = dmg(champ, enemy, &sk2(champ));
    let en_a = dmg(enemy, champ, &enemy.attack_effect); let en_s = dmg(enemy, champ, &enemy.skill_effect); let en_s2 = dmg(enemy, champ, &sk2(enemy));
    let (mac, msc, ms2c) = (champ.attack_cooltime().max(1), champ.skill_cooltime().max(1), champ.skill2_cooltime().max(1));
    let (eac, esc, es2c) = (enemy.attack_cooltime().max(1), enemy.skill_cooltime().max(1), enemy.skill2_cooltime().max(1));
    let my_dps = my_a * 100000 / mac + my_s * 100000 / msc + my_s2 * 100000 / ms2c;
    let en_dps = en_a * 100000 / eac + en_s * 100000 / esc + en_s2 * 100000 / es2c;
    let vis = enemy.is_visible_from(champ);
    let enemy_hp = if vis { enemy.hp } else { enemy.stat_cached.hp };
    let my_die = champ.hp * 100000 / en_dps;
    let en_die = enemy_hp * 100000 / my_dps;
    let pred = my_die > en_die + 60;
    let got = game_ai::can1v1win(&mut rnd, player, &data, champ, enemy);
    println!("o166\tcase={}\t{}\tpred={}\tgot={}\tmy_dmg=({},{},{})\ten_dmg=({},{},{})\tmy_cool=({},{},{})\ten_cool=({},{},{})\tmy_dps={}\ten_dps={}\tvis={}\tenemy_hp={}\tmy_die={}\ten_die={}\tdiff={}",
        n, if pred == got { "MATCH" } else { "MISMATCH" }, pred, got, my_a, my_s, my_s2, en_a, en_s, en_s2,
        mac, msc, ms2c, eac, esc, es2c, my_dps, en_dps, vis, enemy_hp, my_die, en_die, my_die as i64 - en_die as i64);
}
