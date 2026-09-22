#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치G 오라클 — #200 SerpenPokeSubPlan::action_candidates (pub · self ZST)
//! 케이스당 프로세스 1개(check_kill_die_tick / position_score 등 TLS 메모 콜리 있음 — TEMPLATE 함정 ③).
//! 사용: o200.exe <case>
//!   0 = 기준(start_game 직후 · 적 안 보임)                      → 기대 [AroundRegion(7) target_region=2 end_delay=5]
//!   1 = 적 챔프(팀1 Top)를 내 위치+(50000,0)·양방 가시            → near_enemies 1 · L401/403 RunAway 후보 추가 · 결과 = score 최댓값 1개(태그 7|3)
//!   2 = 적 챔프를 내 위치와 동일 좌표·양방 가시                   → L225(적이 나를 때릴 수 있고 자유 · 내 mr==0) → 기대 [RunAway(3) +0x80(with_skill)=0]
//!   3 = 최근접 적 타워(넥서스 제외)의 nearest_enemy=Some((0,champ.id)) → L208~213 → 기대 [RunAway(3) +0x80=1]
//!   4 = 가짜 세르펜(적 타워 엔티티 ty태그→6 · live_list=[id] · 내 위치+(100000,0) · 팀0 가시) → L380 참 · L386 dmg*2<hp → 기대 [Around(5) +0x8=serpen.id]
//!   5 = 4 + champ.hp=0 + 세르펜 거리 20000                        → L386 거짓·range²≥dist² → 기대 [RunAway(3) (RunAway::new)]
//!   6 = 4 + 세르펜 거리 150001(=150000²+... ugt)                   → L380 거짓 → L381 Around(5) (4 와 같은 변형·다른 경로)
//!   7 = 4 + 세르펜 팀0 에 비가시(visible_state[0]=Unknown)          → L380 거짓 → L381 Around(5)
//! 검증 출력: len · 원소 태그(+0xb1) · variant 별 live 바이트(RunAway +0x0..0x38/+0x7d/+0x80..0x84 · Around +0x8 target · AroundRegion +0x8/+0x28/+0x75)
//!   + Option<Input> None 의 태그 워드(open[4]) + rnd 변화 여부 + DebugFrameData 바이트 diff
use game_core::*;
use game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}", ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);

// ★TEMPLATE 함정 ⑦ — 엔티티 세팅은 반드시 raw 포인터 인자로 받아 volatile store
unsafe fn w64(p: *mut u8, off: usize, v: u64) { std::ptr::write_volatile(p.add(off) as *mut u64, v); }
unsafe fn w8(p: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(p.add(off), v); }
unsafe fn r64(p: *const u8, off: usize) -> u64 { std::ptr::read_volatile(p.add(off) as *const u64) }
unsafe fn r8(p: *const u8, off: usize) -> u8 { std::ptr::read_volatile(p.add(off)) }
unsafe fn r32(p: *const u8, off: usize) -> i32 { std::ptr::read_volatile(p.add(off) as *const i32) }

fn dist2(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by); dx * dx + dy * dy }

fn ent_summary(tag: &str, e: *const u8) {
    unsafe {
        println!("  {}: id={} team_tag={} team={} ty_tag={} x={} y={} hp={} level={} stat_range={} radius={} radius_mult={} atk_tag={} atk_range={} atk_growth={} vis0={} vis1={} can_target={} block_target_tick={} act_state={}",
            tag, r64(e, 0x5c0), r64(e, 0), r64(e, 8), r64(e, 0x68), r64(e, 0x660), r64(e, 0x668), r64(e, 0x670), r64(e, 0x5c8), r64(e, 0x438), r64(e, 0x680), r32(e, 0x470),
            r32(e, 0x4c0), r64(e, 0x4a0), r64(e, 0x4a8), r64(e, 0x38), r64(e, 0x38 + 24), r8(e, 0x6b9), r64(e, 0x6a0), r64(e, 0x70));
    }
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(100);
    // open[4]: Option<Input> None 의 태그 워드
    unsafe {
        let none: Option<Input> = None;
        let w: [u64; 4] = std::mem::transmute(none);
        println!("Option<Input>::None words = [{:#x}, {:#x}, {:#x}, {:#x}]  (i64 {})", w[0], w[1], w[2], w[3], w[0] as i64);
        let mv: Option<Input> = Some(Input::Move { x: 7, y: 9 });
        let w: [u64; 4] = std::mem::transmute(mv);
        println!("Option<Input>::Some(Move 7,9) words = [{}, {}, {}, {:#x}]", w[0], w[1], w[2], w[3]);
    }
    // ---- 세계 조작 (cache 생성 전) ----
    {
        let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let champ = cache0.player_champion[0][1].expect("champ") as *const Entity as *mut u8;
        let enemy = cache0.player_champion[1][0].expect("enemy") as *const Entity as *mut u8;
        unsafe {
            let (cx, cy) = (r64(champ, 0x660), r64(champ, 0x668));
            let cid = r64(champ, 0x5c0);
            println!("champ id={} at ({},{}) hp={}", cid, cx, cy, r64(champ, 0x670));
            match case {
                1 | 2 | 8 | 9 => {
                    let dx = if case == 2 { 0 } else { 50000 };
                    w64(enemy, 0x660, cx + dx); w64(enemy, 0x668, cy);
                    // 8/9: 적은 나에게 비가시(near_enemies 제외) · 8: 나는 적에게 가시(L401 참) / 9: 비가시(L401 거짓)
                    w64(enemy, 0x38, if case >= 8 { 2 } else { 0 });   // enemy.visible_state[0]
                    w64(champ, 0x38 + 24, if case == 9 { 2 } else { 0 }); // champ.visible_state[1]
                    let mr = game_ai::plan_legacy::old::max_range_can_use(&*(champ as *const Entity), &*(enemy as *const Entity));
                    let emr = game_ai::plan_legacy::old::max_range_can_use(&*(enemy as *const Entity), &*(champ as *const Entity));
                    let emr40 = game_ai::plan_legacy::old::max_range_nearly_can_use(&*(enemy as *const Entity), &*(champ as *const Entity), 40);
                    let emr60 = game_ai::plan_legacy::old::max_range_nearly_can_use(&*(enemy as *const Entity), &*(champ as *const Entity), 60);
                    println!("ranges: mr(champ->enemy)={} emr(enemy->champ)={} emr_near40={} emr_near60={} dist2={}", mr, emr, emr40, emr60, dist2(cx, cy, cx + dx, cy));
                }
                3 => {
                    // 최근접 적 타워(넥서스 제외) 를 찾아 nearest_enemy = Some((0, champ.id))
                    let mut best: Option<(u64, *mut u8)> = None;
                    for t in cache0.iter_towers_without_nexus(1) {
                        let tp = t as *const Entity as *mut u8;
                        let d = dist2(r64(tp, 0x660), r64(tp, 0x668), cx, cy);
                        if best.map_or(true, |(bd, _)| d < bd) { best = Some((d, tp)); }
                    }
                    let (d, tp) = best.expect("tower");
                    println!("nearest enemy tower id={} dist2={} can_target={} block_target_tick={} ty_tag={} ne_tag_before={}",
                        r64(tp, 0x5c0), d, r8(tp, 0x6b9), r64(tp, 0x6a0), r64(tp, 0x68), r64(tp, 0x88));
                    w64(tp, 0x88, 1); w64(tp, 0x90, 0); w64(tp, 0x98, cid);
                    ent_summary("tower", tp);
                }
                4 | 5 | 6 | 7 => {
                    // 가짜 세르펜: 적 타워 엔티티 하나를 골라 ty 태그 → 6(Serpen), 좌표·가시 세팅, live_list = [id]
                    let mut pick: Option<*mut u8> = None;
                    for t in cache0.iter_towers_without_nexus(1) { pick = Some(t as *const Entity as *mut u8); break; }
                    let sp = pick.expect("tower");
                    let sid = r64(sp, 0x5c0);
                    let dx: u64 = match case { 5 => 20000, 6 => 150001, _ => 100000 };
                    w64(sp, 0x68, 6);
                    w64(sp, 0x660, cx + dx); w64(sp, 0x668, cy);
                    w64(sp, 0x38, if case == 7 { 2 } else { 0 });   // serpen.visible_state[0]
                    if case == 5 { w64(champ, 0x670, 0); }
                    // MobaMode.jungle_runner.serpen.live_list (ptr@0x1d0 len@0x1d8) ← 누수 박스
                    let gm = (&game as &dyn AbstractGame).get_game_mode();
                    let moba = gm.as_moba().expect("moba") as *const MobaMode as *mut u8;
                    let boxed: &'static mut [u64; 1] = Box::leak(Box::new([sid]));
                    println!("live_list before: ptr={:#x} len={} cap={}", r64(moba, 0x1d0), r64(moba, 0x1d8), r64(moba, 0x1c8));
                    w64(moba, 0x1d0, boxed.as_ptr() as u64); w64(moba, 0x1d8, 1);
                    println!("fake serpen id={} dx={} vis0={}", sid, dx, r64(sp, 0x38));
                    ent_summary("serpen", sp);
                }
                _ => {}
            }
            ent_summary("champ", champ);
            ent_summary("enemyTop", enemy);
        }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).expect("player");
    let champ_id = unsafe { r64(cache.player_champion[0][1].unwrap() as *const Entity as *const u8, 0x5c0) };
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let before = format!("{:?}", rnd);
    let mut sp: Buf<5384> = Buf([0u8; 5384]);
    let param: &game_ai::ScoreParameter = unsafe { &*(sp.0.as_ptr() as *const game_ai::ScoreParameter) };
    let team_plan: TeamPlan = Default::default();
    let mut debug: DebugFrameData = Default::default();
    let dbg_before: Vec<u8> = unsafe { std::slice::from_raw_parts(&debug as *const _ as *const u8, std::mem::size_of::<DebugFrameData>()).to_vec() };
    let mut plan: SerpenPokeSubPlan = Default::default();
    println!("sizeof SerpenPokeSubPlan={} TeamPlan={} DebugFrameData={} ScoreParameter={}", std::mem::size_of::<SerpenPokeSubPlan>(), std::mem::size_of::<TeamPlan>(), std::mem::size_of::<DebugFrameData>(), std::mem::size_of::<game_ai::ScoreParameter>());
    let res: bumpalo::collections::Vec<game_ai::SmallActionPlay> = plan.action_candidates(2, &mut rnd, player, &data, param, &team_plan, &mut debug);
    let after = format!("{:?}", rnd);
    let dbg_after: Vec<u8> = unsafe { std::slice::from_raw_parts(&debug as *const _ as *const u8, std::mem::size_of::<DebugFrameData>()).to_vec() };
    let dbg_diff: Vec<usize> = (0..dbg_before.len()).filter(|&i| dbg_before[i] != dbg_after[i]).collect();
    println!("case={} len={} rnd_changed={} debug_diff_bytes={:?} champ_id={}", case, res.len(), before != after, dbg_diff, champ_id);
    for (i, e) in res.iter().enumerate() {
        let p = e as *const game_ai::SmallActionPlay as *const u8;
        unsafe {
            let tag = *p.add(0xb1);
            let name = match tag { 3 => "RunAway", 4 => "Recall", 5 => "Around", 7 => "AroundRegion", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop", _ => "other" };
            print!("  [{}] tag@0xb1={} {}", i, tag, name);
            match tag {
                3 => println!(" start_tick={} goal=({},{}) end_delay={} goal_risk={} pf_tag@0x7d={} with_skill@0x80={} with_ult={} dodge={} committed={}",
                    r64(p, 0), r64(p, 8), r64(p, 0x10), r64(p, 0x18), r64(p, 0x20) as i64, r8(p, 0x7d), r8(p, 0x80), r8(p, 0x81), r8(p, 0x82), r8(p, 0x83)),
                5 => println!(" start_tick={} target@0x8={} goal=({},{}) goal_gain={} range@0x28={} end_delay@0x30={} pf_tag@0x7d={} purpose@0x80={} escape@0x81={}",
                    r64(p, 0), r64(p, 8), r64(p, 0x10), r64(p, 0x18), r64(p, 0x20) as i64, r64(p, 0x28), r64(p, 0x30), r8(p, 0x7d), r8(p, 0x80), r8(p, 0x81)),
                7 => println!(" start_tick={} target_region@0x8={} goal=({},{}) goal_risk@0x20={} end_delay@0x28={} pf_tag@0x75={}",
                    r64(p, 0), r64(p, 8), r64(p, 0x10), r64(p, 0x18), r64(p, 0x20) as i64, r64(p, 0x28), r8(p, 0x75)),
                14 => println!(" emr_tag@0={} pf_tag@0x55={} start_tick@0x58={} target@0x60={} goal=({},{}) margin@0x78={} end_delay@0x80={}",
                    r64(p, 0), r8(p, 0x55), r64(p, 0x58), r64(p, 0x60), r64(p, 0x68), r64(p, 0x70), r64(p, 0x78), r64(p, 0x80)),
                _ => println!(" +0x0={} +0x8={} +0x10={}", r64(p, 0), r64(p, 8), r8(p, 0x10)),
            }
        }
    }
    let vp = &res as *const _ as *const u64;
    unsafe { println!("  vec32: ptr={:#x} bump={:#x} cap={} len={} (pool={:#x})", *vp, *vp.add(1), *vp.add(2), *vp.add(3), &pool as *const _ as u64); }
    std::mem::forget(res);
    // 세계를 raw 로 고쳤으므로(ty 태그·live_list ptr) 소멸자를 돌리지 않는다
    std::process::exit(0);
}
