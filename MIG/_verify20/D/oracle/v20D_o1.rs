#![allow(unused, dead_code, non_snake_case)]
//! 20차 배치D 오라클 1 — #109 BattlePlan::update_v32 를 pub 래퍼 `BattlePlan::update`(battle.rs:427→745 tail call) 경유로 실행.
//! TLS 메모(check_kill_die_tick 등) 때문에 **케이스당 프로세스 1개**: `v20D_o1.exe <case>`.
//!   case 0 = 기본(적 챔프 전원 자기 진영 · 가시). 기대: near_enemies 비고 chase 250000 안에도 없음 → exit_src=1 · ff_exit1_cls=0 · sub_goal=RunAway(4)
//!   case 1 = 적 챔프 1명(idx0)을 내 챔프 옆(+20000)으로 이동 · tick 그대로(가시). 기대: near_enemies≥1 → 교전 판정 경로
//!   case 2 = case1 + set_tick(1000) → Blackboard.last_visible(0)+120 < tick → 비가시 → exit_src=1 · ff_exit1_cls=1(invis)
//!   case 3 = 적 5명 전원 근접(+20000·+40000…) · 가시
//!   case 4 = case0 + main_objective:=Nexus(4) → 998 End(7) 기대 / 5 = sub_goal:=RunAway 프리셋 → 750 조기 return / 6 = KitingBack 프리셋(committed_dir=-1) / 7 = Protect(0) / 8 = End(0)
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify20/D/oracle/v20D_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old::{BattlePlan, BattlePlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_unaligned(base.add(off) as *const T) } }
#[inline(never)]
fn wr<T: Copy>(base: *mut u8, off: usize, v: T) { unsafe { std::ptr::write_unaligned(base.add(off) as *mut T, v) } }

#[inline(never)]
fn move_entity(p: *mut Entity, x: u64, y: u64) {
    unsafe { (*p).x = x; (*p).y = y; }
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tps = setting.tick_per_second as u64;
    if case == 2 { game.set_tick(1000); }
    // 적 이동 (case 1/2: idx0 만, case 3: 전원)
    {
        let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let me = cache0.player_champion[0][0].expect("me");
        let (mx, my) = (me.x, me.y);
        println!("me\tid={}\tx={}\ty={}\thp={}\tmaxhp={}\tms={}", me.id, mx, my, me.hp, me.stat_cached.hp, me.stat_cached.move_speed);
        let n = match case { 1 | 2 => 1, 3 => 5, _ => 0 };
        for k in 0..n {
            if let Some(e) = cache0.player_champion[1][k] {
                let p = e as *const Entity as *mut Entity;
                move_entity(p, mx + 20000 * (k as u64 + 1), my);
            }
        }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    for k in 0..5 { if let Some(e) = cache.player_champion[1][k] { println!("enemy\tk={}\tid={}\tx={}\ty={}\thp={}", k, e.id, e.x, e.y, e.hp); } }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tick = data.cache.game.tick() as u64;
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let mut tp: TeamPlan = Default::default();
    // case 9: TeamPlan.objective(+0x41f, Option<MainObjective> 3B) := Some(Nexus(4)) → 래퍼가 self.main_objective(+0xff) 에 복사 → 998 End(7) 기대
    if case == 9 { let tpp = &mut tp as *mut TeamPlan as *mut u8; wr::<u8>(tpp, 0x41f, 4); wr::<u8>(tpp, 0x420, 0); wr::<u8>(tpp, 0x421, 0); }
    let ps: PositioningScoreData = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(1);

    let mut bp = BattlePlan::new(58, BattlePlanGoal::TryKill(0, 0), &data, player);
    let pm = &mut bp as *mut BattlePlan as *mut u8;
    // case 4: main_objective=Nexus(4) → 998 End 기대 / 5: sub_goal=RunAway 프리셋 → 750 조기 return(0xa8 미기록) 기대
    // case 6: sub_goal=KitingBack(3) → committed_dir=-1 → die_tick.saturating_sub(tps) / 7: Protect(1) → 0 → 그대로 / 8: End(7) → 0
    match case { 4 => wr::<i8>(pm, 0xff, 4), 5 => wr::<i64>(pm, 0x58, 4), 6 => wr::<i64>(pm, 0x58, 3), 7 => wr::<i64>(pm, 0x58, 1), 8 => wr::<i64>(pm, 0x58, 7), 10 => wr::<i64>(pm, 0x58, 5), 11 => wr::<i64>(pm, 0x58, 6), 12 => wr::<i64>(pm, 0x58, 2), _ => {} }
    let p = &bp as *const BattlePlan as *const u8;
    let before: Vec<u8> = (0..280).map(|i| rd::<u8>(p, i)).collect();
    println!("before\tsub_goal_tag={}\tfocus={}\tstart_tick={}\ttick={}\tmain_goal_tag={}\tmo_tag={}\twith_dive={}\ttactic={}\texit_src={}\tff_exit1_cls={}",
             rd::<i64>(p,0x58), rd::<u64>(p,0x60), rd::<u64>(p,0xc0), tick, rd::<i64>(p,0x40), rd::<i8>(p,0xff), rd::<u8>(p,0xf6), rd::<u8>(p,0xfd), rd::<u8>(p,0x108), rd::<u8>(p,0x110));
    bp.update(58, &mut rnd, player, &data, &ps, &tp, &mut dbg);
    let after: Vec<u8> = (0..280).map(|i| rd::<u8>(p, i)).collect();
    let mut diffs = Vec::new();
    for i in 0..280 { if before[i] != after[i] { diffs.push(format!("0x{:x}:{}->{}", i, before[i], after[i])); } }
    println!("DIFF\t{}", diffs.join(" "));
    println!("after\tsub_goal_tag={}\tfocus={}\tprev_die_eval={}\tdie_fall_evals={}\texit_src={}\texit_sub={}\tff_exit1_cls={}\ttactic={}\tsupport_tag={}\tchats_len={}",
             rd::<i64>(p,0x58), rd::<u64>(p,0x60), rd::<u64>(p,0xa8), rd::<u8>(p,0x102), rd::<u8>(p,0x108), rd::<u8>(p,0x111), rd::<u8>(p,0x110), rd::<u8>(p,0xfd), rd::<i64>(p,0x0), rd::<u64>(p,0x78));
    println!("SUMMARY\tcase={}\tsetting_ok={}\ttps={}", case, ok, tps);
}
