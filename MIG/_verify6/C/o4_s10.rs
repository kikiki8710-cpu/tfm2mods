#![allow(unused, dead_code, non_snake_case)]
//! C6 프로브 ④ — `specs[10]` 재검증 (reused: SDK 오라클 실행 / TEMPLATE.rs 정본 세팅)
//!
//! 5차 `o10.out §O10-D` 의 약점을 겨냥한다: 그 스윕은 **162칸이 전부 false** 였다
//! ⟹ "version 0~5 동일" 이 참이긴 해도 **판별력 0 인 vacuous 증거**였다.
//! 여기서는 우물 사각형의 **true 영역을 실제로 밟아** 놓고 버전 불변을 다시 잰다.
//!
//!  (A) `is_enemy_well_danger(version, player, x, y)` 32000 격자 전수 → true 영역 경계상자
//!  (B) 같은 격자 × version 0..7 → 버전 간 불일치 칸 수
//!  (C) `is_ignored_well_enemy(version, player, enemy)` — Entity 를 복제해 team/좌표를 바꿔 먹인다(⑦)
//!  (D) `objective_entity_id_for_main_objective` 태그 0/1 — 에픽/세르펜이 **살아 있는 상태**에서
//!      (5차엔 live_list 가 비어 있어 12/12 전부 None 이었고 "첫 원소 경로 미확인" 으로 남았다)
use game_ai::plan_legacy::old;
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
use game_core::*;
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

const STEP: u64 = 32000;
const N: usize = 30;

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={} height={} tps={} radius={}",
             setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0
                 && setting.champion_radius != 0,
             setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
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

    // ── (A)(B) is_enemy_well_danger ──────────────────────────────────────────
    println!("\n### O6-A is_enemy_well_danger — 32000 격자 30x30, team 별 true 영역 경계상자");
    for t in 0..2usize {
        let ps = game.get_player_by_position(t, Position::Top).unwrap();
        let mut n_true = 0usize;
        let (mut x0, mut y0, mut x1, mut y1) = (u64::MAX, u64::MAX, 0u64, 0u64);
        for iy in 0..N {
            for ix in 0..N {
                let (x, y) = (ix as u64 * STEP, iy as u64 * STEP);
                if game_ai::is_enemy_well_danger(0, ps, x, y) {
                    n_true += 1;
                    if x < x0 { x0 = x } if y < y0 { y0 = y }
                    if x > x1 { x1 = x } if y > y1 { y1 = y }
                }
            }
        }
        println!("player.team={}\ttrue칸={}/900\tbbox=({},{})~({},{})", t, n_true, x0, y0, x1, y1);
    }
    // 명세 history[3] 주장: 적팀0 → (0,800000,64000,960000) ∪ (0,896000,160000,960000)
    println!("\n### O6-A2 주장 사각형 경계 (경계 포함 여부까지)");
    for t in 0..2usize {
        let ps = game.get_player_by_position(t, Position::Top).unwrap();
        let probes: [(u64, u64); 10] = [
            (0, 800000), (0, 799999), (64000, 960000), (64001, 960000), (0, 960000),
            (160000, 896000), (160001, 896000), (960000, 0), (896000, 0), (959999, 63999),
        ];
        for (x, y) in probes {
            println!("team{}\t({}, {})\t{}", t, x, y, game_ai::is_enemy_well_danger(0, ps, x, y));
        }
    }
    println!("\n### O6-B version 0..7 불일치 칸 수 (true 칸이 실제로 존재하는 상태에서)");
    for t in 0..2usize {
        let ps = game.get_player_by_position(t, Position::Top).unwrap();
        let mut diff = 0usize;
        let mut trues = 0usize;
        for iy in 0..N {
            for ix in 0..N {
                let (x, y) = (ix as u64 * STEP, iy as u64 * STEP);
                let base = game_ai::is_enemy_well_danger(0, ps, x, y);
                if base { trues += 1 }
                for v in 1..8usize {
                    if game_ai::is_enemy_well_danger(v, ps, x, y) != base { diff += 1 }
                }
            }
        }
        println!("team{}\tv0 true칸={}\t버전불일치={}\t(판별력 있음={})", t, trues, diff, trues > 0);
    }

    // ── (C) is_ignored_well_enemy ────────────────────────────────────────────
    println!("\n### O6-C is_ignored_well_enemy(version, player, enemy) — Entity 복제 주입(⑦)");
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let ps0 = game.get_player_by_position(0, Position::Top).unwrap();
        let base_e = cache.player_champion[1][0].unwrap();
        for &(x, y, lbl) in [
            (0u64, 800000u64, "team0 우물 안(적팀0 사각형)"),
            (928000u64, 32000u64, "team1 우물 안"),
            (480000u64, 480000u64, "맵 중앙"),
        ]
        .iter()
        {
            unsafe {
                let mut cl: Entity = std::ptr::read(base_e as *const Entity);
                cl.x = x; cl.y = y;
                // player.team=0 기준: enemy 가 team1 일 때만 팀 조건 통과
                for et in 0..2usize {
                    cl.team = TeamType::Player(et);
                    let r = old::is_ignored_well_enemy(0, ps0, &cl);
                    println!("player.team=0\tenemy.team={}\t{}\t({}, {})\t=> {}", et, lbl, x, y, r);
                }
                std::mem::forget(cl);
            }
        }
    }

    // ── (D) objective_entity_id — 에픽/세르펜 생존 상태 ────────────────────────
    println!("\n### O6-D objective_entity_id_for_main_objective — live_list 채우기");
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let mut shown = 0;
    for step in 0..40usize {
        for _ in 0..300 {
            let mut frame: Option<&mut GameFrameData> = None;
            game.run_tick(&ctx, &mut rnd, &mut frame);
        }
        let tick = game.tick();
        let (mut ep, mut se) = (0usize, 0usize);
        if let Some(m) = game.get_game_mode().as_moba() {
            ep = m.jungle_runner().epic.live_list.len();
            se = m.jungle_runner().serpen.live_list.len();
        }
        if ep > 0 || se > 0 || step % 10 == 9 {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let r0 = old::objective_entity_id_for_main_objective(
                &data, MainObjective::Morgard { phase: ObjectPhase::Hunt, with_battle: false });
            let r1 = old::objective_entity_id_for_main_objective(
                &data, MainObjective::Serpen { phase: ObjectPhase::Hunt, with_battle: false });
            println!("tick={}\tepic.live={}\tserpen.live={}\ttag0={:?}\ttag1={:?}", tick, ep, se, r0, r1);
            shown += 1;
            if ep > 0 && se > 0 { break }
        }
    }
    println!("\nDONE");
}
