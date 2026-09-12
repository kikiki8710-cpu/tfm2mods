#![allow(unused, dead_code, non_snake_case)]
//! B7_o1 — 7차 배치B 오라클 ①: **08 `EpicHuntAndPokePlan::is_end` 의 `MainObjective` 전수 스윕**
//!
//! ## 무엇을 내리려 하나 (ev4 → ev2)
//! - `08 knobs[3]` **대상 캠프 고정값 4(JungleType::Morgard)** — 「이 플랜은 에픽 전용」
//! - `08 knobs[5]` **MainObjective→JungleType 표 (0→Morgard4 / 1→Serpen5 / 그 외 None)**
//!
//! 2·3차는 `Morgard`·`Serpen`·`Defense`·`Nexus(Mid)` **4 variant 만** 돌렸다(`_verify2\B\B_o3.rs`).
//! `MainObjective` 는 **12 variant**(tcx 실측: 0 Morgard ~ 11 ComebackPick)이므로
//! 「그 외 전부 None」은 **8 variant 가 미실행**이었다. 여기서 전수로 채운다.
//!
//! ## 판정 근거
//! 명세 08 `L164` = `if !team_plan.take_active(JungleType::Morgard) { return true }`.
//! 접힌 실체가 `TeamPlan+0x41f != 0` 이므로 **objective 태그가 0(Morgard) 이 아닌 모든 variant 는
//! 즉시 `true`** 여야 한다. 하나라도 `false` 가 나오면 명세가 틀린 것이다.
//!
//! ⚠TEMPLATE ② — `start_game()` 이 이미 타워·넥서스를 만든다. `init_tower`/`init_nexus` 를
//!   **부르지 않는다**(2차 프로브 `B_o3.rs` 는 불러서 타워가 2배였다).
//! ⚠TEMPLATE ① — `GameSetting::default()` 는 거의 전부 0 이라 `real_setting()` 을 쓴다.
//! ⚠TLS 메모 — `MapDef::camp_pos` 가 `CAMP_POS_MEMO` 를 쓰지만, 이 프로브는 **맵을 하나만** 쓰므로
//!   캐시 오염이 성립하지 않는다(5차 배치D 의 기각 사례와 같은 조건).
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};
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
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
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

fn main() {
    let pool = bumpalo::Bump::new();
    let setting = real_setting();
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    println!("setting_ok\ttps={}\theight={}\tchampion_radius={}",
             setting.tick_per_second, setting.height, setting.champion_radius);

    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);          // ★init_tower/init_nexus 는 부르지 않는다(TEMPLATE ②)
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbg: DebugFrameData = Default::default();
    let plan: old::EpicHuntAndPokePlan = Default::default();
    let ps = game.get_player_by_position(0, Position::Top).unwrap();

    // 캠프 상수 확인 — 08 consts[0] = JungleType::Morgard(4)
    println!("camp\tMorgard_blue\t{:?}", map.camp_pos(JungleType::Morgard, true));
    println!("camp\tMorgard_red\t{:?}", map.camp_pos(JungleType::Morgard, false));
    println!("camp\tSerpen_blue\t{:?}", map.camp_pos(JungleType::Serpen, true));

    let lts = [LineType::Top, LineType::Mid, LineType::Bottom];
    let phs = [ObjectPhase::None, ObjectPhase::Setup, ObjectPhase::Assemble, ObjectPhase::Hunt];
    let pn = ["None", "Setup", "Assemble", "Hunt"];
    let mut tp: TeamPlan = Default::default();

    // ══ #M) MainObjective 12 variant 전수 — 태그 0(Morgard) 외 전부 true 여야 한다 ══
    println!("\n#M\ttag\tvariant\tis_end\texpect\tverdict");
    let mut n = 0usize; let mut bad = 0usize;
    let mut row = |tag: i32, vn: String, r: bool, exp: bool, n: &mut usize, bad: &mut usize| {
        *n += 1;
        if r != exp { *bad += 1; }
        println!("M\t{}\t{}\t{}\t{}\t{}", tag, vn, r, exp,
                 if r == exp { "MATCH" } else { "MISMATCH" });
    };
    // tag 0 Morgard — phase 별로 기대가 다르다(Setup 만 추가 종료경로가 열린다)
    for (i, ph) in phs.iter().enumerate() {
        for wb in [false, true] {
            tp.objective = Some(MainObjective::Morgard { phase: *ph, with_battle: wb });
            let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
            let exp = *ph == ObjectPhase::Setup;   // 3차 실측: Setup 만 true
            row(0, format!("Morgard{{{},wb={}}}", pn[i], wb), r, exp, &mut n, &mut bad);
        }
    }
    // tag 1..11 — 전부 즉시 true 여야 한다
    for (i, ph) in phs.iter().enumerate() {
        for wb in [false, true] {
            tp.objective = Some(MainObjective::Serpen { phase: *ph, with_battle: wb });
            let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
            row(1, format!("Serpen{{{},wb={}}}", pn[i], wb), r, true, &mut n, &mut bad);
        }
    }
    tp.objective = Some(MainObjective::Defense);
    let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
    row(2, "Defense".into(), r, true, &mut n, &mut bad);
    for lt in lts { tp.objective = Some(MainObjective::DefenseLine(lt));
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(3, format!("DefenseLine({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { tp.objective = Some(MainObjective::Nexus(lt));
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(4, format!("Nexus({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { tp.objective = Some(MainObjective::PressEpic(lt));
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(5, format!("PressEpic({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { tp.objective = Some(MainObjective::SplitEpic(lt));
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(6, format!("SplitEpic({:?})", lt), r, true, &mut n, &mut bad); }
    tp.objective = Some(MainObjective::Repair);
    let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
    row(7, "Repair".into(), r, true, &mut n, &mut bad);
    for lt in lts { tp.objective = Some(MainObjective::Gank { line: lt });
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(8, format!("Gank({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { tp.objective = Some(MainObjective::Dive { line: lt });
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(9, format!("Dive({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { tp.objective = Some(MainObjective::PressTower { line: lt });
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(10, format!("PressTower({:?})", lt), r, true, &mut n, &mut bad); }
    for lt in lts { for rdy in [false, true] {
        tp.objective = Some(MainObjective::ComebackPick { line: lt, ready: rdy });
        let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
        row(11, format!("ComebackPick({:?},ready={})", lt, rdy), r, true, &mut n, &mut bad); } }
    tp.objective = None;
    let r = plan.is_end(3, &mut rnd, ps, &data, &tp, &mut dbg);
    row(-1, "None".into(), r, true, &mut n, &mut bad);
    println!("M_SUMMARY\tcases={}\tmismatch={}", n, bad);

    // ══ #V) version 무영향 재확인 (08 params[1]) ══
    println!("\n#V\tversion\tobjective\tis_end");
    tp.objective = Some(MainObjective::Morgard { phase: ObjectPhase::Setup, with_battle: false });
    for v in [0usize, 1, 2, 3, 23, 24, 30, 46, 50, 60] {
        println!("V\t{}\tMorgard{{Setup}}\t{}", v, plan.is_end(v, &mut rnd, ps, &data, &tp, &mut dbg));
    }
}
