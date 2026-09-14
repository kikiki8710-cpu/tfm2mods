#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #145 SmallActionPlay::evaluation_position (pub) — 직접 호출.
//! 표적: variant 별 목표점 오프셋(target_position 인라인: RunAway +8/+16 · Recall +0x50/+0x58 · Around +0x10/+0x18 · Trace 폴백 +0x68/+0x70) ·
//!       Stop = 제자리 · 공격계(Attack~Ult)는 착지점 그대로 · 그 외는 move_to(speed = move_speed×tps) 1초 투영 · champ None → None ·
//!       sret 24B live(None = +0 만).
//! 한 프로세스 = 한 케이스(argv[1]).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn rd<T: Copy>(p: *const u8, off: usize) -> T { unsafe { std::ptr::read_unaligned(p.add(off) as *const T) } }

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
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    if case == 9 { cache.player_champion[0][0] = None; }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let V = 55usize;
    let champ_opt = cache.player_champion[0][0];
    let (cx, cy, cid, mspd) = champ_opt.map(|c| (c.x, c.y, c.id, c.stat_cached.move_speed)).unwrap_or((0, 0, 0, 0));
    if let Some(c) = champ_opt { w64(c as *const Entity as *mut Entity, 0x640, 700); }   // move_speed 700 → 1초 42000
    let mspd = champ_opt.map(|c| unsafe { std::ptr::read_volatile(&c.stat_cached.move_speed) }).unwrap_or(0);
    let enemy = cache.player_champion[1][0].unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(3);
    let goal = (cx + 200000, cy.saturating_sub(150000));
    println!("champ\tid={} pos=({},{}) move_speed={} tps={} goal={:?}", cid, cx, cy, mspd, setting.tick_per_second, goal);

    let mut expect_move = |tx: u64, ty: u64| -> (u64, u64) {
        let (mut x, mut y) = (cx, cy);
        let mut fd: Option<&mut GameFrameData> = None;
        Entity::move_to(&setting, &ms, &map, cid, &mut x, &mut y, (mspd * setting.tick_per_second) as u64, tx, ty, &mut fd);
        (x, y)
    };
    let (act, pred): (game_ai::SmallActionPlay, Option<Option<(u64, u64)>>) = match case {
        0 => { let mut a = game_ai::SmallActionRunAway::new(&data, player, 0); a.goal_x = goal.0; a.goal_y = goal.1;
               (game_ai::SmallActionPlay::RunAway(a), Some(Some(expect_move(goal.0, goal.1)))) }
        1 => (game_ai::SmallActionPlay::Stop, Some(Some(expect_move(cx, cy)))),
        2 => { let mut a = game_ai::SmallActionAround::new(V, &mut rnd, &data, player, enemy.id, 0); a.goal_x = goal.0; a.goal_y = goal.1;
               (game_ai::SmallActionPlay::Around(a), Some(Some(expect_move(goal.0, goal.1)))) }
        3 => { let mut a = game_ai::SmallActionTrace::new(&data, enemy.id, 0); a.goal_x = goal.0; a.goal_y = goal.1;
               (game_ai::SmallActionPlay::Trace(a), None) }
        4 => { let a = game_ai::SmallActionAttack::new(&data, enemy.id); (game_ai::SmallActionPlay::Attack(a), None) }
        5 => { let mut a = game_ai::SmallActionRunAway::new(&data, player, 0); a.goal_x = cx + 10; a.goal_y = cy;   // 가까운 목표
               (game_ai::SmallActionPlay::RunAway(a), Some(Some(expect_move(cx + 10, cy)))) }
        6 => { let mut a = game_ai::SmallActionAround::new(V, &mut rnd, &data, player, enemy.id, 0); a.goal_x = goal.0; a.goal_y = goal.1;
               (game_ai::SmallActionPlay::AroundRunAway(a), Some(None)) }   // 명세: AroundRegion/AroundRunAway → None
        9 => (game_ai::SmallActionPlay::Stop, Some(None)),
        _ => (game_ai::SmallActionPlay::Stop, None),
    };
    let tag: u8 = rd(&act as *const _ as *const u8, 0xb1);
    let g = act.evaluation_position(V, player, &data);
    let m = match pred { Some(p) => if p == g { "MATCH" } else { "**MISMATCH**" }, None => "OBS" };
    println!("case{}\ttag={}\tgame={:?}\tpred={:?}\t{}", case, tag, g, pred, m);
    if let Some(Some((gx, gy))) = pred { let d = ((gx as i128 - cx as i128).pow(2) + (gy as i128 - cy as i128).pow(2)) as f64; println!("moved_dist\t{:.0}\t(1초 이동량 {})", d.sqrt(), mspd * setting.tick_per_second); }
}
