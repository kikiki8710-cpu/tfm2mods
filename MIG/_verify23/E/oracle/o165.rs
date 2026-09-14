#![allow(unused, dead_code, non_snake_case)]
//! 23차 E · 165 action_eval::evaluate_action 오라클 (pub 직접 호출). 한 프로세스 = 한 케이스(argv[1]).
//! 예측 = 명세 logic 을 pub 콜리(position_eval_at·line_phase_position_eval_purpose·champion_hp_value·possible_risk·
//!   SmallActionPlay::evaluation_position)로 합성. 미니언 0 마리 → last_hit_gain = 0 (LAG_MEMO n=0).
//! ScoreParameter(5384B) 는 MaybeUninit::zeroed 후 pub 필드만 세팅(o132 방식).
use game_core::*;
use game_ai::plan_legacy::action_eval::{ActionContext, Anchor, PriorityProfile};
use rand::SeedableRng;
use std::sync::Arc;
use std::mem::MaybeUninit;

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

// 케이스: (anchor: 0 Free/1 Lane(Top)/2 Lane(Mid)/3 Objective, action kind: 0 AroundBush/1 Attack/2 LaneMinionPosition/3 Around/4 Stop,
//          version, undying, risk_damage, risk_possible_tower, hp)
fn case(n: u32) -> (u8, u8, usize, bool, i64, i64, usize) {
    match n {
        0 => (0, 0, 2, false, 0, 0, 1000),        // anchor Free → None
        1 => (1, 1, 2, false, 0, 0, 1000),        // Attack → None
        2 => (1, 0, 2, false, 0, 0, 1000),        // Lane(Top) + AroundBush → Some
        3 => (2, 0, 2, false, 0, 0, 1000),        // Lane(Mid)
        4 => (1, 0, 2, true, 0, 0, 1000),         // undying → danger 0
        5 => (1, 0, 2, false, 777, 555, 1000),    // risk_damage/tower 폴백값 — dest Some 이면 무시
        6 => (1, 2, 2, false, 0, 0, 1000),        // LaneMinionPosition
        7 => (1, 3, 2, false, 0, 0, 1000),        // Around
        8 => (1, 4, 2, false, 0, 0, 1000),        // Stop → None
        9 => (1, 0, 1, false, 0, 0, 1000),        // version 1
        10 => (3, 0, 2, false, 0, 0, 1000),       // Objective anchor → None
        11 => (1, 0, 2, false, 0, 0, 1),          // hp 1
        12 => (1, 0, 2, false, 0, 0, 100000),     // hp 큼
        13 => (1, 0, 2, false, 0, 0, 1000),       // risk_possible 비움 → possible 0
        _ => (1, 0, 2, false, 0, 0, 1000),
    }
}

#[repr(C)] struct PG { from: usize, tick: usize, value: usize }
static RP: [PG; 2] = [PG { from: 1, tick: 100, value: 400 }, PG { from: 2, tick: 200, value: 300 }];

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
    let (anc, kind, version, undying, rd, rpt, hp) = case(n);
    let mut cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
    // ★판별력: 적 챔프 DPS 캐시 주입(o124 방식) — position_eval 의 risk 가 0 이 아니게
    for team in 0..2 { for pos in 0..5 { for i in 0..5 {
        let c = &mut cache.player_champion_cache[team][pos];
        c.attack_per_sec[i] = 300 + 100 * pos + 1000 * team;
        c.skill_per_sec[i] = 200; c.skill2_per_sec[i] = 100; c.ult_per_sec[i] = 400;
    } } }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].unwrap();      // team0 Top
    let enemy = cache.player_champion[1][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let ep = enemy as *const Entity as *mut Entity;
    unsafe {
        std::ptr::write_volatile(&mut (*cp).hp, hp);
        std::ptr::write_volatile(&mut (*cp).stat_buff_cached.undying, undying);
        // 적 Top 을 내 스폰 근처(dest 주변)로 옮기고 가시 처리 → risk/gain 이 0 이 아니게
        std::ptr::write_volatile(&mut (*ep).x, champ.x + 40000);
        std::ptr::write_volatile(&mut (*ep).y, champ.y);
        std::ptr::write(&mut (*ep).visible_state[0], VisibleState::Visible);
        std::ptr::write_volatile(&mut (*ep).hp, 3000);
        std::ptr::write_volatile(&mut (*ep).stat_cached.hp, 3000);
    }
    let champ: &Entity = unsafe { &*cp };
    let player = g.get_player_by_position(0, Position::Top).unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let mut debug: DebugFrameData = Default::default();

    let mut zp: MaybeUninit<game_ai::ScoreParameter> = MaybeUninit::zeroed();
    let pp = zp.as_mut_ptr();
    unsafe {
        std::ptr::write_volatile(&mut (*pp).player.risk_damage, rd as usize);
        std::ptr::write_volatile(&mut (*pp).player.risk_possible_tower, rpt as usize);
        std::ptr::write_volatile(&mut (*pp).version, version);
        // champion_hp_value 가 읽는 칸(o132: 0x60/0xa8~0xd0) — hp_value 가 0 이 아니게
        std::ptr::write_volatile(&mut (*pp).player.team, 0);
        std::ptr::write_volatile(&mut (*pp).player.attack_value, 500);
        std::ptr::write_volatile(&mut (*pp).player.util_value, 300);
        std::ptr::write_volatile(&mut (*pp).player.attack_power, 1000);
        std::ptr::write_volatile(&mut (*pp).player.util_power_base, 200);
        std::ptr::write_volatile(&mut (*pp).player.cc_time_x_inv_cd, 50);
        std::ptr::write_volatile(&mut (*pp).player.buff_inv_cd_count, 2);
        // bumpalo Vec(near_allies/near_enemies) 의 ptr 을 null 이 아닌 정렬 값으로(len 0 유지)
        let base = pp as *mut u8;
        std::ptr::write_volatile(base.add(0x14b8) as *mut usize, 8usize);
        std::ptr::write_volatile(base.add(0x14d8) as *mut usize, 8usize);
        // player.risk_possible(bumpalo Vec @0x918+0x18: ptr +0 · len +0x18) 에 PossibleGain 2개 → possible_risk(…,9999) ≠ 0
        std::ptr::write_volatile(base.add(0x930) as *mut *const u8, RP.as_ptr() as *const u8);
        std::ptr::write_volatile(base.add(0x948) as *mut usize, if n == 13 { 0 } else { 2 });
        std::ptr::write_volatile(base.add(0x950) as *mut *const u8, RP.as_ptr() as *const u8);   // gain_possible ptr 도 non-null
    }
    let parameter: &game_ai::ScoreParameter = unsafe { zp.assume_init_ref() };

    let anchor = match anc { 0 => Anchor::Free, 1 => Anchor::Lane { line: LineType::Top }, 2 => Anchor::Lane { line: LineType::Mid }, _ => Anchor::Objective { camp_x: 1, camp_y: 2 } };
    let actx = ActionContext { priority: PriorityProfile::Lane, anchor };
    let action: game_ai::SmallActionPlay = match kind {
        0 => game_ai::SmallActionPlay::AroundBush(game_ai::SmallActionAroundBush::new_with_target(&data, enemy, version, game_ai::AroundBushOutlineType::None)),
        1 => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)),
        2 => game_ai::SmallActionPlay::LaneMinionPosition(game_ai::SmallActionLaneMinionPosition::new(&data, version, enemy.id, 0, game_ai::PositionEvalPurpose::Lane)),
        3 => game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(version, &mut rnd, &data, player, enemy.id, 100000)),
        _ => game_ai::SmallActionPlay::Stop,
    };

    // ── 예측 ──
    let pred: Option<i64> = (|| {
        let line = match actx.anchor { Anchor::Lane { line } => line, _ => return None };
        let ok = matches!(action, game_ai::SmallActionPlay::Around(_) | game_ai::SmallActionPlay::AroundHide(_) | game_ai::SmallActionPlay::AroundRegion(_)
            | game_ai::SmallActionPlay::AroundPosition(_) | game_ai::SmallActionPlay::AroundPositionBush(_) | game_ai::SmallActionPlay::AroundBush(_) | game_ai::SmallActionPlay::LaneMinionPosition(_));
        if !ok { return None; }
        let purpose = game_ai::line_phase_position_eval_purpose(player, &data);
        let dest = action.evaluation_position(version, player, &data);
        let dest_score = dest.map(|(x, y)| game_ai::position_eval_at(version, player, &data, x, y, purpose));
        let hp_value = game_ai::champion_hp_value(&data, parameter, &parameter.player);
        let hpv = (champ.hp as i64).max(1);
        let base_damage = dest_score.as_ref().map(|s| hpv * s.risk.max(0) / 100).unwrap_or(parameter.player.risk_damage as i64);
        let possible_damage = parameter.player.possible_risk(&data, 9999) + dest_score.as_ref().map(|s| hpv * s.tower_risk.max(0) / 100).unwrap_or(parameter.player.risk_possible_tower as i64) / 3;
        let danger = if champ.stat_buff_cached.undying { 0 } else { base_damage * hp_value / hpv + possible_damage * hp_value / hpv };
        let positional_gain = dest_score.as_ref().map(|s| (s.gain - s.gain_me).max(0) * hp_value / 200).unwrap_or(0);
        let last_hit_gain = 0i64; // 미니언 0
        println!("pred_detail\tpurpose={:?}\tdest={:?}\tdest_score={:?}\thp_value={}\thp={}\tbase={}\tpossible={}\tdanger={}\tpos_gain={}",
            purpose, dest, dest_score.as_ref().map(|s| (s.risk, s.tower_risk, s.gain, s.gain_me)), hp_value, hpv, base_damage, possible_damage, danger, positional_gain);
        Some(positional_gain - danger + last_hit_gain)
    })();
    let got = game_ai::plan_legacy::action_eval::evaluate_action(version, &actx, parameter, &mut rnd, player, &data, &action, &mut debug);
    println!("o165\tcase={}\t{}\tpred={:?}\tgot={:?}\tanchor={}\tkind={}\tversion={}\tundying={}\thp={}", n,
        if pred == got { "MATCH" } else { "MISMATCH" }, pred, got, anc, kind, version, undying, hp);
}
