#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #163 abstract_input::skill / #164 skill2 (pub · game_ai::skill / game_ai::skill2 직접 호출) + 독립 재구현 대조
//! Effect 를 직접 조립(range/growth/casting/target)해 champ.skill_effect(+0x4c8) / skill2_effect(+0x500) 에 raw write.
//! 접근 경로 모델 = 명세 L216~L234: margin(Tower|Nexus 2000 / 그외 15000) · effect_range_with_radii · isqrt · adjust_position · safe_move(pub)
//! 사용: o163.exe <case>  (케이스당 프로세스 1개)
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
unsafe fn r32(base: *const u8, off: usize) -> i32 { std::ptr::read_volatile(base.add(off) as *const i32) }
fn show(o: &Option<Input>) -> String {
    match o { None => "None".to_string(), Some(Input::Move { x, y }) => format!("Move({},{})", x, y), Some(other) => format!("{:?}", other) }
}
fn mk_effect(range: u64, growth: u64, casting: CastingType, target: CastingTarget) -> Effect {
    Effect { range, growth_range: growth, start_timing: 0, casting, target,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, attack_type: AttackType::Skill }
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
    let psd: PositioningScoreData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let ver = 57usize;

    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ: &Entity = cache.player_champion[0][2].expect("champ");
    // (which: 1 skill / 2 skill2, effect: None|Some(range,growth,casting,target), target_kind: 0 far enemy champ / 1 ally / 2 enemy tower / 3 near enemy(1000) / 4 far enemy 200000,
    //  visible, neutral_champ, level)
    let (which, eff, tk, visible, neutral, level): (u8, Option<(u64, u64, CastingType, CastingTarget)>, u8, bool, bool, i64) = match case {
        0 => (1, None, 0, false, false, 1),                                                    // skill_effect None → None
        1 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Ally)), 0, false, false, 1),   // check 실패(적에게 Ally) → None
        2 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 0, false, false, 1),  // 먼 적 · 비가시 → Move(target.x,y) via safe_move
        3 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 0, true, false, 1),   // 먼 적 · 가시 → 접근 지점(margin 15000)
        4 => (1, Some((150000, 1000, CastingType::Position, CastingTarget::Enemy)), 0, true, false, 1),    // casting≠Targeting → champ 반지름 미가산
        5 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 2, true, false, 1),   // 적 타워 · 가시 → margin 2000
        6 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 0, false, true, 1),   // 챔프 Neutral → 비가시라도 접근 지점 경로
        7 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 3, true, false, 1),   // 가까운 적 → get_input_target Some → Input::Skill
        8 => (1, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 4, true, false, 3),   // 레벨 3 → growth*(level-1) 반영
        9 => (2, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 0, true, false, 1),   // skill2: 레벨 1 → None
        10 => (2, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 0, true, false, 3),  // skill2: 레벨 3 · 먼 적 가시 → 접근 지점
        11 => (2, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 2, false, false, 3), // skill2: 타워 비가시 → Move(tower.x,y)
        12 => (2, Some((150000, 1000, CastingType::Targeting, CastingTarget::Ally)), 0, true, false, 3),   // skill2: check 실패 → None
        13 => (2, Some((150000, 1000, CastingType::Targeting, CastingTarget::Enemy)), 3, true, false, 3),  // skill2: 가까운 적 → Input::Skill2
        14 => (1, Some((10000, 0, CastingType::Targeting, CastingTarget::Enemy)), 0, true, false, 1),      // 사거리 작음 → from_distance sat_sub(15000) → 0 → x=target.x
        _ => (1, None, 0, false, false, 1),
    };
    let chp = champ as *const Entity as *mut u8;
    unsafe {
        p64(chp, 0x5c8, level);
        if neutral { p64(chp, 0x0, 1); }
        let off = if which == 1 { 0x4c8 } else { 0x500 };
        match eff {
            None => { p32(chp, off + 0x30, -1); }
            Some((r, g, c, t)) => { std::ptr::write(chp.add(off) as *mut Option<Effect>, Some(mk_effect(r, g, c, t))); }
        }
    }
    let target: &Entity = match tk {
        1 => cache.player_champion[0][0].expect("ally"),
        2 => cache.iter_towers_without_nexus(1).next().expect("tower"),
        _ => cache.player_champion[1][0].expect("enemy"),
    };
    let tp = target as *const Entity as *mut u8;
    unsafe {
        if tk == 3 { p64(tp, 0x660, champ.x as i64 + 1000); p64(tp, 0x668, champ.y as i64 + 500); }
        if tk == 4 { p64(tp, 0x660, champ.x as i64 + 200000); p64(tp, 0x668, champ.y as i64 - 100000); }
        if visible { p64(tp, 0x38 + 0 * 24, 0); }   // target.visible_state[champ_team 0] = Visible(0)
    }
    // ── 모델 ──
    let effect_ref: Option<&Effect> = if which == 1 { champ.skill_effect.as_ref() } else if champ.level > 2 { champ.skill2_effect.as_ref() } else { None };
    let can = if which == 1 { champ.can_skill() } else { champ.can_skill2() };
    let model: Option<Input> = (|| {
        let effect = effect_ref?;
        if !can { return None; }
        if !effect.target.check(champ, target) { return None; }
        // get_input_target: 사거리 안이면 Some 이라 모델 밖 → 케이스 7/13 은 태그만 본다
        let tag = unsafe { r64(chp, 0) };
        let team = unsafe { r64(chp, 8) } as usize;
        let vs = unsafe { r64(tp, 0x38 + team * 24) };
        let visible_from = if (tag & 1) == 1 { true } else { vs == 0 };
        if visible_from {
            let dx = champ.x as i64 - target.x as i64; let dy = champ.y as i64 - target.y as i64;
            let sz = game_core::utils::isqrt(dx * dx + dy * dy);
            let ty_tag = unsafe { r64(tp, 0x68) };
            let margin: u64 = if (ty_tag & 14) == 2 { 2000 } else { 15000 };
            let caster_r = if matches!(effect.casting, CastingType::Targeting) { champ.radius() as u64 } else { 0 };
            let er = caster_r + effect.range as u64 + champ.stat_buff_cached.range as u64 + effect.growth_range as u64 * (champ.level as u64 - 1)
                     + effect.range_adjust(champ, target) as u64 + target.radius() as u64;
            let from = er.saturating_sub(margin);
            let x = target.x as i64 + (from as i64 * dx) / sz;
            let y = target.y as i64 + (from as i64 * dy) / sz;
            let (ax, ay) = Game::adjust_position(&map, &setting, x, y);
            game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, ax, ay)
        } else {
            game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, target.x, target.y)
        }
    })();
    let got: Option<Input> = if which == 1 { game_ai::skill(ver, &mut rnd, player, &data, &psd, target) }
                             else { game_ai::skill2(ver, &mut rnd, player, &data, &psd, target) };
    let near = tk == 3;
    let ok = if near { matches!(got, Some(Input::Skill { .. }) | Some(Input::Skill2 { .. })) } else { show(&got) == show(&model) };
    println!("case={}\twhich={}\tgot={}\tmodel={}\t{}\t(can={} eff_some={} lvl={} tgt=({},{}) champ=({},{}) ty_tag={} vis_tag={} team_tag={})",
             case, which, show(&got), if near { "Some(Skill*)".to_string() } else { show(&model) }, if ok { "MATCH" } else { "DIFF" },
             can, effect_ref.is_some(), champ.level, target.x, target.y, champ.x, champ.y,
             unsafe { r64(tp, 0x68) }, unsafe { r64(tp, 0x38) }, unsafe { r64(chp, 0) });
}
