#![allow(unused, dead_code, non_snake_case)]
//! 19차 배치C 오라클 1 — 담당 serpen 4 + SerpenHuntAndPokePlan::sub_plan 의 **조기 반환 경로**를 실행으로 확인한다.
//!  (a) check_serpen_giveup: 게임 시작 직후(세르펜 live_list 비어 있음) → L241 true
//!  (b) check_serpen_hunt : 시작 직후(양팀 5명 생존·캠프 근처 0) → L361 false 경로
//!  (c) check_serpen_setup: Jungle 선수(is_skip_serpen 비대상)·세르펜 미생존·remain ≥ tps*15 → L81 false
//!  (d) serpen_passive_plan: phase Hunt(3)→tag 14 / None(0)·Assemble(2)→tag -1 / Setup(1)→(전략 의존, 값만 기록)
//!  (e) SerpenHuntAndPokePlan::sub_plan(default): 세르펜 없음 → SerpenCheck tag 13 (L48~50)
//! 한 프로세스 = 한 케이스(argv[1]) — TLS 메모 함정 ③ 회피(camp_pos 메모는 같은 맵이라 무해).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify19/C/oracle/v19C_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old::{check_serpen_giveup, check_serpen_hunt, check_serpen_setup, serpen_passive_plan, SerpenHuntAndPokePlan};
use game_ai::plan_legacy::team_plan::{TeamPlan, ObjectPhase};
use game_ai::GoalData;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    println!("towers\t{}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
    // 세르펜/에픽 생존 상태 확인 (MobaMode +0x1d8 serpen.live_list.len / +0x1a8 epic.live_list.len / +0x1e0 serpen.next_respawn_tick)
    let mode = game.get_game_mode();
    let mp: *const u8 = match &mode { GameMode::Moba(m) => (*m as *const MobaMode) as *const u8, _ => std::ptr::null() };
    if !mp.is_null() {
        println!("moba\tserpen_live_len={}\tepic_live_len={}\tserpen_next_respawn={}\ttick={}",
            rd::<u64>(mp, 0x1d8), rd::<u64>(mp, 0x1a8), rd::<u64>(mp, 0x1e0), game.tick());
    }
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut rng = rand::rngs::StdRng::seed_from_u64(11);
    let tp: TeamPlan = Default::default();
    let gd: GoalData = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let version = 0usize;
    match case {
        0 => {
            for t in 0..2usize { for p in 0..5usize {
                let player = game.get_player_by_position(t, poss[p]).expect("player");
                let r = check_serpen_giveup(version, &mut rng, player, &data, &tp, &mut dbg);
                println!("giveup\tt={}\tp={}\t{}", t, p, r);
            } }
        }
        1 => {
            for t in 0..2usize { for p in 0..5usize {
                let player = game.get_player_by_position(t, poss[p]).expect("player");
                let plan = game_ai::plan_legacy::types::BigPlan::SerpenHuntAndPoke(Default::default());
                let r = check_serpen_hunt(version, &mut rng, player, &data, &gd, &plan, &tp, &mut dbg);
                println!("hunt\tt={}\tp={}\t{}", t, p, r);
            } }
        }
        2 => {
            for t in 0..2usize { for p in 1..5usize {
                let player = game.get_player_by_position(t, poss[p]).expect("player");
                let r = check_serpen_setup(version, &mut rng, player, &data, &tp, &mut dbg);
                println!("setup\tt={}\tp={}\t{}", t, p, r);
            } }
        }
        3 => {
            let phases = [ObjectPhase::None, ObjectPhase::Setup, ObjectPhase::Assemble, ObjectPhase::Hunt];
            for (pi, ph) in phases.iter().enumerate() {
                for t in 0..2usize { for p in 1..3usize {
                    let player = game.get_player_by_position(t, poss[p]).expect("player");
                    let r = serpen_passive_plan(version, &mut rng, player, &data, &tp, *ph, None, &mut dbg);
                    let tag: i64 = rd::<i64>(&r as *const _ as *const u8, 0);
                    let line: u8 = rd::<u8>(&r as *const _ as *const u8, 0x11e);
                    println!("passive\tphase={}\tt={}\tp={}\ttag={}\tline_at_0x11e={}\tis_some={}", pi, t, p, tag, line, r.is_some());
                } }
            }
        }
        4 => {
            for t in 0..2usize { for p in 0..5usize {
                let player = game.get_player_by_position(t, poss[p]).expect("player");
                let mut plan: SerpenHuntAndPokePlan = Default::default();
                let r = plan.sub_plan(version, &mut rng, player, &data, &gd, &tp, &mut dbg);
                let tag: i64 = rd::<i64>(&r as *const _ as *const u8, 0);
                println!("subplan\tt={}\tp={}\ttag={}\t{:?}", t, p, tag, r);
            } }
        }
        _ => {}
    }
}
