#![allow(unused, dead_code, non_snake_case)]
//! 17차 배치C 오라클 1 — 담당 #50~#54 의 공통 헬퍼·G10 반증 항목을 **실행**으로 확인한다.
//!  (a) `game_core::is_top_side` 극성(#53 open[0]·logic L44 반증) — pub 직접 호출
//!  (b) `macro_judgement_penalty/bonus`(#50 closed[0]·knobs[4] / #51 open[5] / #53 open[4]) —
//!      judgement × judgement_mental_ratio 를 쓸어 값 범위 0..=3 확인
//!  (c) `v23_visible_objective_overload`(#50 open[0]) — 근접 아군 a × 시야 적 e 진리표
//!  (d) `v25_objective_far_split_pressure`(#52) — lead × from_mid × minion_count × far_count 진리표
//! TLS 메모: camp_pos(CAMP_POS_MEMO)만 — 같은 맵이라 무해. 나머지는 순수 읽기(IR 에 LocalKey 없음).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify17/C/oracle/v17C_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::team_plan::{macro_judgement_penalty, macro_judgement_bonus,
                                       v23_visible_objective_overload, v25_objective_far_split_pressure};

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *const u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) }
}

// ── 명세 독립 재구현 ────────────────────────────────────────────
fn spec_penalty(judgement: u64, ratio: u64) -> i32 {
    let acc = 100 + 9 * std::cmp::min(judgement * ratio / 1000, 100);
    let d = (400i64 - acc as i64).max(0) as i32;
    (d + 124) / 125
}
fn spec_bonus(judgement: u64, ratio: u64) -> i32 {
    let acc = 100 + 9 * std::cmp::min(judgement * ratio / 1000, 100);
    ((acc as i64 - 400).max(0) as i32) / 175
}
fn spec_overload(allies: u64, enemies: u64) -> bool { enemies > 2 && enemies >= allies + 2 }
fn spec_far_split(far_count: u64, lead: u64, from_mid: i64, count: i32) -> bool {
    if far_count == 0 { return false; }
    if lead > 1 { return true; }
    if from_mid > 2999 { return true; }
    if from_mid > 999 { return count > 1; }
    count > 3
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
    game.set_tick(1000);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let bbp = &bb as *const [Blackboard; 2] as *const u8;
    let cachep = &cache as *const AbstractGameWithCache as *const u8;
    println!("towers\t{}\ttick={}", game.world.tower_ids.len(), game.tick());

    // ── (a) is_top_side 극성 ──
    let h = setting.height;
    for (x, y) in [(100000u64, 100000u64), (800000, 800000), (100000, 800000), (800000, 100000),
                   (480000, 480000), (480000, 479999), (479999, 480000), (0, h), (h, 0)] {
        let r = game_core::is_top_side(&ctx, x, y);
        println!("is_top_side\tx={}\ty={}\tx+y={}\theight={}\t=> {}\t(spec: x<=height-y => {})",
                 x, y, x + y, h, r, x <= h - y);
    }
    let camp0 = MapDef::camp_pos(&map, JungleType::Morgard, true);
    let camp1 = MapDef::camp_pos(&map, JungleType::Morgard, false);
    println!("camp_pos\tMorgard side0={:?}\tside1={:?}\ttop0={}\ttop1={}", camp0, camp1,
             game_core::is_top_side(&ctx, camp0.0, camp0.1), game_core::is_top_side(&ctx, camp1.0, camp1.1));
    let s0 = MapDef::camp_pos(&map, JungleType::Serpen, true);
    println!("camp_pos\tSerpen side0={:?}\ttop={}", s0, game_core::is_top_side(&ctx, s0.0, s0.1));

    // ── (b) macro_judgement_penalty / bonus ──
    let player = game.get_player_by_position(0, Position::Top).expect("player");
    let pp = player as *const PlayerState as *const u8;
    println!("player\tjudgement@0x218={}\tratio@0x450={}\tteam@0x930={}",
             rd::<u64>(pp, 0x218), rd::<u64>(pp, 0x450), rd::<u64>(pp, 0x930));
    let mut mism = 0;
    for &j in &[0u64, 10, 20, 33, 34, 40, 50, 60, 80, 100, 120, 200] {
        for &r in &[0u64, 500, 1000, 1500] {
            wr::<u64>(pp, 0x218, j);
            wr::<u64>(pp, 0x450, r);
            let p = macro_judgement_penalty(2, player);
            let b = macro_judgement_bonus(2, player);
            let (sp, sb) = (spec_penalty(j, r), spec_bonus(j, r));
            if p != sp || b != sb { mism += 1; }
            println!("judge\tj={}\tratio={}\tpenalty={}\tbonus={}\tspec={}/{}\t{}",
                     j, r, p, b, sp, sb, if p == sp && b == sb { "OK" } else { "MISMATCH" });
        }
    }
    println!("judge_summary\tMISMATCH={}", mism);
    wr::<u64>(pp, 0x218, 80); wr::<u64>(pp, 0x450, 1000);

    // ── 챔프 준비: hp/max_hp 정상화, 전원 자기 넥서스 근처(카메라 밖)로 ──
    let ents: Vec<Vec<*const u8>> = (0..2).map(|t| (0..5).map(|p| {
        cache.player_champion[t][p].expect("champ") as *const Entity as *const u8
    }).collect()).collect();
    for t in 0..2 { for p in 0..5 {
        let e = ents[t][p];
        println!("ent\tt={}\tp={}\tx={}\ty={}\thp={}\tmax={}\tms={}", t, p,
                 rd::<u64>(e, 0x660), rd::<u64>(e, 0x668), rd::<u64>(e, 0x670), rd::<u64>(e, 0x628), rd::<u64>(e, 0x640));
        wr::<u64>(e, 0x628, 1000); wr::<u64>(e, 0x670, 1000);
    }}
    let far = |t: usize| -> (u64, u64) { if t == 0 { (60000, 60000) } else { (900000, 900000) } };
    let park = |ents: &Vec<Vec<*const u8>>| { for t in 0..2 { for p in 0..5 {
        let (x, y) = far(t); wr::<u64>(ents[t][p], 0x660, x); wr::<u64>(ents[t][p], 0x668, y);
    }}};

    // ── (c) v23_visible_objective_overload(player(team0), data, camp0) ──
    // 아군 a 명을 camp0 근처(120000 이내), 적 e 명을 camp0 근처 + blackboard[1].last_visible[pos]=tick 로 '최근 시야'
    let tick = game.tick() as u64;
    let mut mism = 0;
    for a in 0..=4u64 { for e in 0..=5u64 {
        park(&ents);
        for p in 0..5 { wr::<u64>(bbp, 744 + 0x1e0 + 8 * p, 0); }
        for p in 0..a as usize { wr::<u64>(ents[0][p], 0x660, camp0.0 + 20000); wr::<u64>(ents[0][p], 0x668, camp0.1); }
        for p in 0..e as usize {
            wr::<u64>(ents[1][p], 0x660, camp0.0 - 20000); wr::<u64>(ents[1][p], 0x668, camp0.1);
            wr::<u64>(bbp, 744 + 0x1e0 + 8 * p, tick);   // blackboard[1].last_visible[p] = tick
        }
        let r = v23_visible_objective_overload(player, &data, camp0);
        let s = spec_overload(a, e);
        if r != s { mism += 1; }
        println!("overload\ta={}\te={}\t=> {}\tspec={}\t{}", a, e, r, s, if r == s { "OK" } else { "MISMATCH" });
    }}
    // 시야 없는 적은 안 센다 / hp 39% 적은 안 센다 (각 1건)
    park(&ents);
    for p in 0..5 { wr::<u64>(bbp, 744 + 0x1e0 + 8 * p, 0); }
    for p in 0..4 { wr::<u64>(ents[1][p], 0x660, camp0.0 - 20000); wr::<u64>(ents[1][p], 0x668, camp0.1); }
    println!("overload_novis\te=4(last_visible=0)\t=> {}\t(spec false)", v23_visible_objective_overload(player, &data, camp0));
    for p in 0..4 { wr::<u64>(bbp, 744 + 0x1e0 + 8 * p, tick); wr::<u64>(ents[1][p], 0x670, 390); }
    println!("overload_lowhp\te=4(hp39%)\t=> {}\t(spec false)", v23_visible_objective_overload(player, &data, camp0));
    for p in 0..4 { wr::<u64>(ents[1][p], 0x670, 400); }
    println!("overload_hp40\te=4(hp40%)\t=> {}\t(spec true)", v23_visible_objective_overload(player, &data, camp0));
    for p in 0..5 { wr::<u64>(ents[1][p], 0x670, 1000); }
    println!("overload_summary\tMISMATCH={}", mism);

    // ── (d) v25_objective_far_split_pressure(player(team0), data, Morgard) ──
    // Morgard 의 먼 라인 = Bottom. camp0 에서 250000 초과 떨어진 Bottom 라인 점을 찾는다.
    let mut spot = None;
    'o: for yi in 0..30u64 { for xi in 0..30u64 {
        let (x, y) = (xi * 32000 + 16000, yi * 32000 + 16000);
        if game_core::is_near_line(&ctx, x, y, LineType::Bottom) && utils::distance_sq(x, y, camp0.0, camp0.1) > 62499999999 {
            spot = Some((x, y)); break 'o;
        }
    }}
    let spot = spot.expect("bottom spot");
    println!("far_spot\t{:?}\tdist_sq_to_camp0={}", spot, utils::distance_sq(spot.0, spot.1, camp0.0, camp0.1));
    let mut mism = 0;
    for far_count in 0..=2u64 { for lead in 0..=2u64 { for &fm in &[0i64, 999, 1000, 2999, 3000] { for &mc in &[0i32, 1, 2, 3, 4] {
        park(&ents);
        for p in 0..far_count as usize { wr::<u64>(ents[0][p], 0x660, spot.0); wr::<u64>(ents[0][p], 0x668, spot.1); }
        wr::<u64>(cachep, 0x21e0 + 0, lead);          // bottom_lead[0]
        wr::<i64>(bbp, 0x50 + 0x10, fm);             // blackboard[0].bottom_minion_state.from_mid
        wr::<i32>(bbp, 0x50 + 0x20, mc);             // .minion_count
        let r = v25_objective_far_split_pressure(player, &data, JungleType::Morgard);
        let s = spec_far_split(far_count, lead, fm, mc);
        if r != s { mism += 1; }
        println!("farsplit\tfar={}\tlead={}\tfrom_mid={}\tcount={}\t=> {}\tspec={}\t{}", far_count, lead, fm, mc, r, s,
                 if r == s { "OK" } else { "MISMATCH" });
    }}}}
    // top_lead 는 무관해야 한다(먼 라인 = Bottom)
    park(&ents);
    wr::<u64>(ents[0][0], 0x660, spot.0); wr::<u64>(ents[0][0], 0x668, spot.1);
    wr::<u64>(cachep, 0x21e0, 0); wr::<i64>(bbp, 0x60, 0); wr::<i32>(bbp, 0x70, 0);
    wr::<u64>(cachep, 0x21c0, 5);
    println!("farsplit_toplead5\t=> {}\t(spec false: Morgard 는 bottom_lead 만 본다)", v25_objective_far_split_pressure(player, &data, JungleType::Morgard));
    wr::<u64>(cachep, 0x21c0, 0);
    // hp 39% 아군은 far_count 에서 빠진다
    wr::<u64>(cachep, 0x21e0, 2); wr::<u64>(ents[0][0], 0x670, 390);
    println!("farsplit_lowhp\tlead=2 hp39%\t=> {}\t(spec false)", v25_objective_far_split_pressure(player, &data, JungleType::Morgard));
    wr::<u64>(ents[0][0], 0x670, 400);
    println!("farsplit_hp40\tlead=2 hp40%\t=> {}\t(spec true)", v25_objective_far_split_pressure(player, &data, JungleType::Morgard));
    // 일반 정글캠프(Rhino) 는 즉시 false
    println!("farsplit_rhino\t=> {}\t(spec false)", v25_objective_far_split_pressure(player, &data, JungleType::Rhino));
    println!("farsplit_summary\tMISMATCH={}", mism);
}
