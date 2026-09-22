#![allow(unused, dead_code, non_snake_case, invalid_reference_casting)]
#![feature(thread_local)]
//! 25차 배치J 오라클 — 203 PassiveJunglePlan::next_plan(pub) 진리표.
//!  한 프로세스 = 한 케이스(argv `k=v`). 세계 = TEMPLATE mkgame + set_tick + set_strategy.
//!  엔티티 변조 = raw write_volatile(함정 ⑦). sret 384B·self 104B·chats 원소를 바이트로 덤프해
//!  variant 별 live 바이트(IR store 독해)와 &mut self writes 표를 실행으로 대조한다.
//!  CAMP_POS_MEMO(TLS) 는 #[thread_local] extern static 직독(24차 F 선례).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/J/oracle/o25j.rs
//! 실행: o25j.exe k=v ...   (드라이버 = run25j.py)
use game_core::*;
use rand::SeedableRng;
use std::collections::HashMap;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

extern "Rust" {
    // CAMP_POS_MEMO: RefCell<(usize, [[Option<(u64,u64)>;2];8])> 400B + lazy 상태 i8 @400 + pad 7
    #[thread_local]
    #[link_name = "_RNvNCNKNvNvMNtNtCs97f5S1uJLkH_9game_core10simulation7map_defNtBa_6MapDef8camp_pos13CAMP_POS_MEMOs0_0s_023___RUST_STD_INTERNAL_VAL"]
    static CAMP_RAW: [u8; 408];
}

fn camp_memo_dump() -> String {
    let p = unsafe { CAMP_RAW.as_ptr() };
    let borrow: i64 = rd(p, 0); let key: u64 = rd(p, 8); let state: u8 = rd(p, 400);
    let mut s = format!("borrow={} key=0x{:x} state={} entries=[", borrow, key, state);
    for camp in 0..8usize { for team in 0..2usize {
        let off = 16 + camp * 48 + team * 24;
        let tag: u64 = rd(p, off);
        if tag == 1 { let x: u64 = rd(p, off + 8); let y: u64 = rd(p, off + 16); s += &format!("({},{})=({},{}) ", camp, team, x, y); }
    } }
    s + "]"
}

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } }

fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join("") }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::calculate_action_score as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let mut setting = real_setting();
    setting.epic_jungle.first_spawn_tick = a.get("first_spawn", 100_000_000) as usize;
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tut = match a.get("tut", 0) { 1 => TutorialType::First, 2 => TutorialType::TopSolo, 3 => TutorialType::Bottom, 4 => TutorialType::MidSolo,
                                     5 => TutorialType::MidBottom, 6 => TutorialType::JungleOnly, 7 => TutorialType::Line, 8 => TutorialType::Total, _ => TutorialType::None };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick = a.get("tick", 100000) as usize;
    game.set_tick(tick);
    let tps = setting.tick_per_second;
    // 전략(팀 0) — early_jungle 0 GrowthAndCover / 1 Ganking / 2 CounterJungle · focused 0 Top / 1 Bottom / 2 All
    let mut st: Strategy = Default::default();
    st.early_jungle = match a.get("ej", 0) { 1 => EarlyJungleStrategy::Ganking, 2 => EarlyJungleStrategy::CounterJungle, _ => EarlyJungleStrategy::GrowthAndCover };
    st.focused = match a.get("fa", 0) { 1 => FocusedAreaStrategy::Bottom, 2 => FocusedAreaStrategy::All, _ => FocusedAreaStrategy::Top };
    game.set_strategy(0, st);
    let version = a.get("version", 2) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    // 블랙보드 변조
    bb[0].top_minion_state.minion_count = a.get("mc_top", 0) as i32;
    bb[0].mid_minion_state.minion_count = a.get("mc_mid", 0) as i32;
    bb[0].bottom_minion_state.minion_count = a.get("mc_bot", 0) as i32;
    if a.get("bigline_top", 0) == 1 { bb[0].big_goal[0] = (0, Some(BigGoal::Line { line: LineType::Top })); }
    if a.get("bigline_top_tag1", 0) == 1 { bb[0].big_goal[0] = (0, Some(BigGoal::Recall)); }
    let player = game.get_player_by_position(0, Position::Jungle).expect("player");
    let champ = cache.player_champion[0][1].expect("champ");
    let cp = ep(champ);
    // 내 챔프 스탯/위치
    let maxhp = a.get("maxhp", 1000) as usize; let hp = a.get("hp", 1000) as usize;
    wr(cp, 0x628, maxhp); wr(cp, 0x670, hp);
    wr(cp, 0x640, a.get("ms", 1000) as usize);
    let cx = a.get("cx", 100000) as u64; let cy = a.get("cy", 100000) as u64;
    wr(cp, 0x660, cx); wr(cp, 0x668, cy);
    // 적 정글러(팀1 슬롯1) — Battle 분기용
    let enemy = cache.player_champion[1][1].expect("enemy");
    let eptr = ep(enemy);
    let emaxhp = a.get("emaxhp", 1000) as usize; let ehp = a.get("ehp", 1000) as usize;
    wr(eptr, 0x628, emaxhp); wr(eptr, 0x670, ehp);
    if a.get("edx", -1) >= 0 { wr(eptr, 0x660, cx + a.get("edx", 0) as u64); wr(eptr, 0x668, cy + a.get("edy", 0) as u64); }
    let eplayer = game.get_player_by_position(1, Position::Jungle).expect("eplayer");
    if a.get("evis", 0) == 1 { bb[1].last_visible[eplayer.info.position.as_index()] = tick; }
    let data = OperationData::new(&cache, &ctx, &bb);
    // self
    let mut rnd = rand::rngs::StdRng::seed_from_u64(a.get("seed", 7) as u64);
    let mut plan = game_ai::plan_legacy::old::PassiveJunglePlan::new(&mut rnd, 0);
    plan.team = a.get("s_team", 0) as usize;
    plan.player_team = a.get("s_pteam", 0) as usize;
    plan.jungle = match a.get("s_jungle", 0) { 1 => JungleType::Mushroom, 2 => JungleType::Stump, 3 => JungleType::Bee, _ => JungleType::Rhino };
    plan.last_lead_action_tick = a.get("s_last", 0) as usize;
    // team_plan.next_respawn_tick[team][camp_idx]
    let mut tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
    let r0 = a.get("resp0", 100_000_000) as usize; let r1 = a.get("resp1", 100_000_000) as usize;
    for i in 0..4 { tp.next_respawn_tick[0][i] = r0; tp.next_respawn_tick[1][i] = r1; }
    for (k, camp) in [("rS0", JungleType::Stump), ("rM0", JungleType::Mushroom), ("rB0", JungleType::Bee), ("rR0", JungleType::Rhino)] {
        if a.get(k, -1) >= 0 { tp.next_respawn_tick[0][game_ai::plan_legacy::team_plan::camp_idx(camp)] = a.get(k, 0) as usize; } }
    for (k, camp) in [("rS1", JungleType::Stump), ("rM1", JungleType::Mushroom), ("rB1", JungleType::Bee), ("rR1", JungleType::Rhino)] {
        if a.get(k, -1) >= 0 { tp.next_respawn_tick[1][game_ai::plan_legacy::team_plan::camp_idx(camp)] = a.get(k, 0) as usize; } }
    let goal: game_ai::GoalData = Default::default();
    let posd: PositioningScoreData = Default::default();
    let mut dbg: DebugFrameData = Default::default();

    // 진단
    println!("setting_ok={} tick={} tps={} ego={} agg={} order={} champ=({},{}) hp={}/{} enemy=({},{}) hp={}/{} solorank={}",
        ok, tick, tps, player.info.parameter.ego_ratio(), player.info.parameter.aggressive_ratio(), player.info.parameter.order_ratio(),
        champ.x, champ.y, champ.hp, champ.stat_cached.hp, enemy.x, enemy.y, enemy.hp, enemy.stat_cached.hp, (&game as &dyn AbstractGame).is_solorank());
    let stg = player.strategy(&mut rnd, &game as &dyn AbstractGame);
    println!("strategy focused={:?} early_jungle={:?}", stg.focused, stg.early_jungle);
    if a.get("diag", 1) == 1 { for camp in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee] {
        let idx = game_ai::plan_legacy::team_plan::camp_idx(camp);
        let mut live = [0usize; 2];
        if let GameMode::Moba(mm) = (&game as &dyn AbstractGame).get_game_mode() {
            for t in 0..2usize { live[t] = mm.jungle_runner.get_camp_state(t, camp).live_list.len(); }
        }
        let p0 = map.camp_pos(camp, true); let p1 = map.camp_pos(camp, false);
        let reg1 = map.regions[(p1.1 / 32000) as usize][(p1.0 / 32000) as usize];
        let reg0 = map.regions[(p0.1 / 32000) as usize][(p0.0 / 32000) as usize];
        println!("camp {:?} idx={} live=[{},{}] pos_t0={:?} pos_t1={:?} resp=[{},{}] dist_from_champ_t1={} region(t0)={} rp={} region(t1)={} rp={}", camp, idx, live[0], live[1], p0, p1,
            tp.next_respawn_tick[0][idx], tp.next_respawn_tick[1][idx], game_core::utils::distance(champ.x, champ.y, p1.0, p1.1),
            reg0, cache.region_point[reg0], reg1, cache.region_point[reg1]);
    } }
    println!("camp_memo_before {}", camp_memo_dump());
    let before: [u8; 104] = unsafe { std::ptr::read(&plan as *const _ as *const [u8; 104]) };
    let bb_vis = data.blackboard[1].is_recent_visible(&game as &dyn AbstractGame, player, enemy);
    println!("is_recent_visible(bb1,player,enemy)={} dist2={}", bb_vis, champ.x.abs_diff(enemy.x).pow(2) + champ.y.abs_diff(enemy.y).pow(2));
    // ★호출
    let r: Option<game_ai::plan_legacy::types::BigPlan> = plan.next_plan(version, &mut rnd, player, &data, &goal, &posd, &tp, &mut dbg);
    println!("camp_memo_right_after {}", camp_memo_dump());
    let rb: [u8; 384] = unsafe { std::ptr::read(&r as *const _ as *const [u8; 384]) };
    let tag: i64 = unsafe { std::ptr::read(rb.as_ptr() as *const i64) };
    println!("RESULT tag={} bytes0_64={}", tag, hex(&rb[0..64]));
    println!("RESULT bytes64_128={}", hex(&rb[64..128]));
    println!("RESULT byte286={} byte287_300={}", rb[286], hex(&rb[287..300]));
    if tag == 9 { println!("RESULT battle sub_goal_tag={} entry_src={} chats(cap,ptr,len)=({},{:x},{})", rd::<i64>(rb.as_ptr(), 8 + 0x58), rb[8 + 0x107],
        rd::<u64>(rb.as_ptr(), 8 + 0x68), rd::<u64>(rb.as_ptr(), 8 + 0x70), rd::<u64>(rb.as_ptr(), 8 + 0x78)); }
    let after: [u8; 104] = unsafe { std::ptr::read(&plan as *const _ as *const [u8; 104]) };
    let mut diffs = vec![];
    for i in 0..104 { if before[i] != after[i] { diffs.push(i); } }
    println!("SELF team={} player_team={} last={} jungle={:?} chats.len={} diff_bytes={:?}", plan.team, plan.player_team, plan.last_lead_action_tick, plan.jungle, plan.chats.len(), diffs);
    for (i, c) in plan.chats.iter().enumerate() {
        let cb: [u8; 24] = unsafe { std::ptr::read(c as *const _ as *const [u8; 24]) };
        println!("CHAT[{}] {:?} bytes={}", i, c, hex(&cb));
    }
    println!("camp_memo_after {}", camp_memo_dump());
    println!("dbg infos={} logs={} texts={}", dbg.infos.len(), dbg.logs.len(), dbg.texts.len());
    drop(r);
}
