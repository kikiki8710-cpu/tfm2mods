#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ⑦ — `specs[15]/open[0]`(version 축) 을 **마지막 남은 두 하위 함수**로 좁힌다.
//! 5차 배치D 는 `new`/`new_dive`/`new_region` 만 닫고 `update`·`single_tower_dive_is_viable` 를 남겼다.
//!   - `SinglePlanBattle::update` 는 `D6_o6` 에서 9/9 무영향
//!   - 여기서는 **`game_ai::plan_legacy::old::single_tower_dive_is_viable`(pub, 7인자)** 를 직접 호출한다.
//! 덤: `game_ai::engage_requires_dive`(pub) = `specs[15]/knobs[2]` 의 술어.
//! ⚠TLS 메모 대비 — 케이스를 한 프로세스에 몰지만, 이 두 함수는 `tlsscan` 대상이 아니므로
//!   먼저 **순서 반전**(version 오름차순/내림차순)으로 재현성을 확인한다(템플릿 함정③ 징후 검사).
use game_core::*;
use game_ai::plan_legacy::old::single_tower_dive_is_viable;
use game_ai::engage_requires_dive;
use game_ai::plan_legacy::team_plan::TeamPlan;
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
const POS: [Position; 5] = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, POS[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let pool = bumpalo::Bump::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false, tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tp: TeamPlan = Default::default();

    // 표적: 적팀 챔피언 5명 + 적팀 타워(이름 6칸)
    let mut targets: Vec<(String, &Entity)> = Vec::new();
    for x in 0..5usize {
        if let Some(e) = cache.player_champion[1][x] { targets.push((format!("enemy_champ[{}]", x), e)); }
    }
    if let Some(e) = cache.top_tower[1]     { targets.push(("enemy_top_tower".into(), e)); }
    if let Some(e) = cache.mid_tower[1]     { targets.push(("enemy_mid_tower".into(), e)); }
    if let Some(e) = cache.bottom_tower[1]  { targets.push(("enemy_bottom_tower".into(), e)); }
    if let Some(e) = cache.nexus[1]         { targets.push(("enemy_nexus".into(), e)); }
    for x in 0..5usize {
        if let Some(e) = cache.player_champion[0][x] { targets.push((format!("ally_champ[{}]", x), e)); }
    }

    let plr = game.get_player_by_position(0, Position::Top).unwrap();

    // (a) engage_requires_dive — 15/knobs[2]
    let mut erd_true = 0usize;
    for (n, e) in targets.iter() {
        let r = engage_requires_dive(plr, &data, e);
        if r { erd_true += 1 }
        println!("ERD\ttarget={}\tty_tag={}\trequires_dive={}", n,
                 unsafe { *(&e.ty as *const EntityType as *const u8) }, r);
    }
    println!("ERD\tTOTAL_true={}/{}", erd_true, targets.len());

    // (b) single_tower_dive_is_viable — version 축 (오름/내림 양방향으로 재현성 확인)
    let vers_up  = [0usize, 1, 2, 3, 30, 50, 54, 58, 60];
    let vers_dn  = [60usize, 58, 54, 50, 30, 3, 2, 1, 0];
    for (label, vers) in [("up", &vers_up), ("dn", &vers_dn)].iter() {
        for (n, e) in targets.iter() {
            // ⚠타워/넥서스 표적은 fight_check.rs:979 에서 unwrap 패닉 — 챔피언만 잰다(범위 명시)
            if unsafe { *(&e.ty as *const EntityType as *const u8) } != 13 { continue }
            let mut row = String::new();
            for v in vers.iter() {
                let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
                let mut dbg: DebugFrameData = Default::default();
                let r = single_tower_dive_is_viable(*v, &mut rnd, plr, &data, &tp, e, &mut dbg);
                row += &format!(" v{}={}", v, if r { 1 } else { 0 });
            }
            println!("STDV\t{}\ttarget={}\t{}", label, n, row.trim());
        }
    }
}
