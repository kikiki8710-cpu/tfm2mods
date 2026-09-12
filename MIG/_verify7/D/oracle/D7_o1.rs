#![allow(unused, dead_code, non_snake_case)]
//! 7차 배치 D 오라클 #1 — `_verify3\TEMPLATE.rs` 를 그대로 따른다(real_setting · init_tower 안 부름).
//!
//! 재는 것 (전부 배치 D 의 `ev>=4` 행을 실행으로 내리기 위한 것):
//!  A. 니치 실측  — 15 consts[4] / 15 mem[16] / 15 mem[17] / 19 consts[4]
//!     `Option<TowerType>::None` · `Option<JungleType>::None` · `Option<LineType>::None`
//!     `Option<SinglePlanBattle>::None` 의 **+0x0 8바이트** 와 크기
//!  B. 타워 극성 — 15 consts[2] (`1 - player.info.team` 의 1)
//!     `cache.iter_towers_without_nexus(1 - team)` 이 정말 **적팀** 타워만 주는지
//!     + 팀당 개수(TEMPLATE ② 오염 검산: towers=16 · twin=2/2 여야 한다)
//!  C. 정글 클리어 여유 — 19 knobs[5](tps) · 19 knobs[6](is_side_cleared tps*5)
//!     ★TLS 메모(`CAMP_POS_MEMO`) 때문에 **한 프로세스 = 한 tps** 다. tps 는 argv[1].
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting(tps: usize) -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000;
    s.height = 960000;
    s.respawn_tick = 300;
    s.respawn_growth = 30;
    s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180;
    s.respawn_max = 2400;
    s.visible_distance = 130000;
    s.tick_per_second = tps;
    s.champion_radius = 10000;
    s.nexus_heal = 10;
    s.nexus_heal_2v2 = 10;
    s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100;
    s.kill_exp = 30;
    s.kill_exp_growth = 30;
    s.assist_exp_ratio = 40;
    s.kill_gold = 300;
    s.assist_gold = 100;
    s.start_gold = 500;
    s.gold_per_second = 7;
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400;
    s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200;
    s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600;
    s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700;
    s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15;
    s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

fn bytes_of<T>(v: &T) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()).to_vec() }
}

fn main() {
    let tps: usize = std::env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(60);
    let setting = real_setting(tps);
    println!("setting_ok\twidth={}\theight={}\ttps={}\tchamp_radius={}",
             setting.width, setting.height, setting.tick_per_second, setting.champion_radius);

    // ---------- A. 니치 실측 ----------
    let nt: Option<TowerType> = None;
    let nj: Option<JungleType> = None;
    let nl: Option<LineType> = None;
    println!("A\tOption<TowerType>\tsize={}\tbytes={:?}", std::mem::size_of::<Option<TowerType>>(), bytes_of(&nt));
    println!("A\tOption<JungleType>\tsize={}\tbytes={:?}", std::mem::size_of::<Option<JungleType>>(), bytes_of(&nj));
    println!("A\tOption<LineType>\tsize={}\tbytes={:?}", std::mem::size_of::<Option<LineType>>(), bytes_of(&nl));
    let nb: Option<game_ai::plan_legacy::old::SinglePlanBattle> = None;
    let bb = bytes_of(&nb);
    let head = i64::from_le_bytes([bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], bb[6], bb[7]]);
    println!("A\tOption<SinglePlanBattle>\tsize={}\tinner={}\thead_i64={}",
             std::mem::size_of::<Option<game_ai::plan_legacy::old::SinglePlanBattle>>(),
             std::mem::size_of::<game_ai::plan_legacy::old::SinglePlanBattle>(), head);
    // TowerType 태그 전표(배열 순서 교차검증용)
    for (i, t) in [TowerType::Top, TowerType::Mid, TowerType::Bottom].iter().enumerate() {
        let s: Option<TowerType> = Some(*t);
        println!("A\tTowerType[{}]\tbytes={:?}", i, bytes_of(&s));
    }
    for (i, j) in [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump].iter().enumerate() {
        let s: Option<JungleType> = Some(*j);
        println!("A\tJungleType {:?}\tbytes={:?}", j, bytes_of(&s));
    }

    // ---------- 세계 구성 ----------
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
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80;
            st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, &ctx);
    // ⛔ init_tower / init_nexus 는 부르지 않는다 (TEMPLATE ②)
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bbd: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bbd);
    println!("INTEG\ttowers={}\ttwin0={}\ttwin1={}\t(기대 16/2/2)",
             game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());

    // ---------- B. 타워 극성 ----------
    for team in 0..2usize {
        let enemy = 1 - team;
        let mut n = 0usize;
        let mut same = 0usize;
        let mut other = 0usize;
        let mut coords: Vec<(u64, u64)> = Vec::new();
        for t in cache.iter_towers_without_nexus(enemy) {
            n += 1;
            coords.push((t.x, t.y));
            let tb = bytes_of(&t.team);
            // TeamType 은 16B. 첫 8B 를 팀 번호로 읽는다(값만 찍고 판정은 아래 문자열로).
            let v = i64::from_le_bytes([tb[0], tb[1], tb[2], tb[3], tb[4], tb[5], tb[6], tb[7]]);
            if format!("{:?}", t.team).contains(&format!("{}", enemy)) { same += 1; } else { other += 1; }
            if n <= 2 { println!("B\tteam={} enemy={} tower.team={:?} head={}", team, enemy, t.team, v); }
        }
        coords.sort();
        let uniq = { let mut c = coords.clone(); c.dedup(); c.len() };
        println!("B\tmy_team={}\titer_towers_without_nexus({})\tn={}\t좌표유일={}\t적팀표기일치={}/{}",
                 team, enemy, n, uniq, same, n);
    }

    // ---------- C. 정글 클리어 (tps 의존) ----------
    let tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(11);
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    for camp in [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump] {
        let a = game_ai::plan_legacy::old::is_cleared(
            camp, 0, 0, &mut rnd2, player, &data, &tp, 0, &mut dbg);
        let b = game_ai::plan_legacy::old::is_side_cleared(
            camp, 0, 0, &mut rnd2, player, &data, &tp, 0, &mut dbg);
        println!("C\ttps={}\tcamp={:?}\tis_cleared={}\tis_side_cleared={}", tps, camp, a, b);
    }
    let g = game_ai::plan_legacy::old::best_jungle_goal(
        0, &mut rnd2, player, &data, &tp, None, &mut dbg);
    println!("C\ttps={}\tbest_jungle_goal={:?}", tps, g);
}
