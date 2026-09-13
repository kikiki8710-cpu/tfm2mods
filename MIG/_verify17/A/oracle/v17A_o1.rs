#![allow(unused, dead_code, non_snake_case)]
//! 17차 배치A 오라클 1 — #42 SinglePlanBattle::with_runaway · #43 BattlePlan::with_runaway
//! 둘 다 `pub` · TLS 메모 없음(순수 읽기 함수, IR 에 LocalKey 호출 0) → 한 프로세스 전수 진리표.
//! 축: main_goal 태그 {0 TryKill,1 Support,2 Response,3 Avoid} × elapsed {0, tps-1, tps, tps+1, 10*tps}
//!     × main_objective {None(-1), 0..11} × with_dive {false,true}  = 4×5×13×2 = 520 케이스/함수
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify17/A/oracle/o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old::{BattlePlan, SinglePlanBattle, BattlePlanGoal};
use game_ai::plan_legacy::team_plan::MainObjective;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *mut u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut T, v) }
}

/// 명세 logic 의 독립 재구현 (42/43 공통 — 오프셋만 다르다)
fn spec_with_runaway(main_goal_tag: i64, tick: u64, start_tick: u64, tps: u64, mo_tag: i8, with_dive: bool) -> bool {
    if main_goal_tag == 0 {
        let elapsed = tick.saturating_sub(start_tick);
        if elapsed <= tps { return false; }
    }
    match mo_tag {
        5 => !with_dive,
        0 | 1 | 2 | 4 => false,
        _ => true,
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tps = setting.tick_per_second as u64;
    println!("tps\t{}", tps);

    let mo_tags: Vec<i8> = vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let elapsed_set: Vec<u64> = vec![0, tps - 1, tps, tps + 1, 10 * tps];
    let goal_tags: Vec<i64> = vec![0, 1, 2, 3];
    let start_tick: u64 = 1000;

    let mut n_match = 0usize; let mut n_mis = 0usize;
    for &el in elapsed_set.iter() {
        // ★tick 은 프로세스 안에서 set_tick(pub 트레이트 메서드)으로 조정
        game.set_tick((start_tick + el) as usize);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let got_tick = data.cache.game.tick() as u64;
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        for &g in goal_tags.iter() {
            for &mo in mo_tags.iter() {
                for wd in [false, true] {
                    // ── #43 BattlePlan ──
                    let mut bp = BattlePlan::new(58, BattlePlanGoal::TryKill(0, 0), &data, player);
                    let p = &mut bp as *mut BattlePlan as *mut u8;
                    wr::<i64>(p, 0x40, g);
                    wr::<u64>(p, 0xc0, start_tick);
                    wr::<u8>(p, 0xf6, wd as u8);
                    wr::<i8>(p, 0xff, mo);
                    let r43 = bp.with_runaway(58, &data);
                    // ── #42 SinglePlanBattle ──
                    let mut sp = SinglePlanBattle::new(58, BattlePlanGoal::TryKill(0, 0), &data, player);
                    let q = &mut sp as *mut SinglePlanBattle as *mut u8;
                    wr::<i64>(q, 0x40, g);
                    wr::<u64>(q, 0x80, start_tick);
                    wr::<u8>(q, 0x88, wd as u8);
                    wr::<i8>(q, 0x8d, mo);
                    let r42 = sp.with_runaway(58, &data);
                    let pred = spec_with_runaway(g, got_tick, start_tick, tps, mo, wd);
                    let m = (r42 == pred) && (r43 == pred);
                    if m { n_match += 1; } else { n_mis += 1; }
                    println!("CASE\tgoal={}\telapsed={}\ttick={}\tmo={}\twith_dive={}\tr42={}\tr43={}\tpred={}\t{}",
                             g, el, got_tick, mo, wd, r42, r43, pred, if m { "MATCH" } else { "MISMATCH" });
                }
            }
        }
    }
    // 읽기 확인: new() 가 세팅한 start_tick / main_objective 초기값도 찍는다(명세 밖 참고)
    println!("SUMMARY\tmatch={}\tmismatch={}\tsetting_ok={}", n_match, n_mis, ok);
}
