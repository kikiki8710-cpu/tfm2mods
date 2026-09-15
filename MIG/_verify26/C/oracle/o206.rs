#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치C · 206 PassiveLinePlan::update 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10, 미니언 없음).
//!  관측: &mut self 280B 전후 diff · in_recall(0x110) · v46_commit(0x111) · chats(len·원소 바이트) · rnd 소비 · debug.logs 수 · 패닉.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/C/oracle/o206.rs
//!  실행: o206.exe line=<0|1|2> me=<0..4> hp=<pct|-1> hpabs=<n|-1> pos=<keep|start|tower|xy> mx= my= obj=<none|gank|dive> oline=<0|1|2> tick= version=2 seed=
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::old::PassiveLinePlan;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
}

fn lt(n: i64) -> LineType { match n { 0 => LineType::Top, 1 => LineType::Mid, _ => LineType::Bottom } }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::check_kill_die_tick as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.to_string()); } }
    let a = Args { m };
    let mut setting = real_setting();
    // 에픽 첫 스폰 틱(0x8a8) — default 0 이면 is_line_phase 가 항상 false. 실전값 미상 → 인자(기본 36000)
    { let sp0 = &mut setting as *mut GameSetting as *const u8; wr(sp0, 0x8a8, a.i("spawn", 36000) as usize); }
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: a.i("dbg", 0) == 1,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick = a.i("tick", 600) as usize;
    game.set_tick(tick);
    let version = a.i("version", 2) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let team = a.i("team", 0) as usize;
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[me]).expect("player");
    let champ = cache.player_champion[team][me].expect("champ");
    let cp = ep(champ);
    let line = lt(a.i("line", 0));
    // 위치 조작
    let pos = a.s("pos", "keep");
    if pos == "start" { let (sx, sy) = line.get_start_position(&setting, team); wr(cp, 0x660, sx); wr(cp, 0x668, sy); }
    else if pos == "tower" { let t = cache.tower(line, team).expect("tower"); wr(cp, 0x660, t.x); wr(cp, 0x668, t.y); }
    else if pos == "xy" { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    // HP 조작: hp=<pct> 이면 hp = ceil(pct*max/100) 후 실제 hp*100/max 를 찍는다
    if a.i("maxhp", -1) >= 0 { wr(cp, 0x628, a.i("maxhp", 0) as usize); wr(cp, 0x670, a.i("maxhp", 0) as usize); }
    let maxhp: usize = rd(cp, 0x628);
    if a.i("hpabs", -1) >= 0 { wr(cp, 0x670, a.i("hpabs", 0) as usize); }
    else if a.i("hp", -1) >= 0 { let pct = a.i("hp", 0) as usize; let h = (pct * maxhp + 99) / 100; wr(cp, 0x670, h); }
    let hp: usize = rd(cp, 0x670);
    // 적 가시(vis=1: bb[1-team].last_visible[p]=tick) · 적 위치 이동(enear=D: 내 위치+D)
    let vis = a.i("vis", 0);
    let enear = a.i("enear", -1);
    for p in 0..5usize {
        let e = cache.player_champion[1 - team][p].expect("enemy");
        let eb = ep(e);
        if vis == 1 { bb[1 - team].last_visible[p] = tick; }
        if enear >= 0 { let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668); wr(eb, 0x660, mx + enear as u64 + (p as u64) * 1000); wr(eb, 0x668, my); }
    }
    // 팀 플랜 objective
    let mut tp: TeamPlan = Default::default();
    let obj = a.s("obj", "none");
    let oline = lt(a.i("oline", 0));
    if obj == "gank" { tp.objective = Some(MainObjective::Gank { line: oline }); }
    else if obj == "dive" { tp.objective = Some(MainObjective::Dive { line: oline }); }
    let tpp = &tp as *const TeamPlan as *const u8;
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let psd: PositioningScoreData = Default::default();
    let seed = a.i("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let rnd0 = rnd.clone();
    let mut sp = PassiveLinePlan::new(line);
    let spp = &sp as *const PassiveLinePlan as *const u8;
    // 선행 상태 조작(있으면): in_recall0 · commit0
    if a.i("in_recall0", -1) >= 0 { wr(spp, 0x110, a.i("in_recall0", 0) as u8); }
    if a.i("commit0", -1) >= 0 { wr(spp, 0x111, a.i("commit0", 0) as u8); }
    // 도주 선행 상태: flee0(0x112) · entry0=<tick>(0x0=1,0x8=tick,0x10=eflags0) · appr0(0x114) · hpe0(0x100) · hpm0(0x108) · threats0=e0,e1(0x48 Vec<usize>)
    if a.i("flee0", -1) >= 0 { wr(spp, 0x112, a.i("flee0", 0) as u8); }
    if a.i("entry0", -1) >= 0 { wr(spp, 0x0, 1i64); wr(spp, 0x8, a.i("entry0", 0) as usize); wr(spp, 0x10, a.i("eflags0", 0) as u8); }
    if a.i("appr0", -1) >= 0 { wr(spp, 0x114, a.i("appr0", 0) as u8); }
    if a.i("hpe0", -1) >= 0 { wr(spp, 0x100, a.i("hpe0", 0) as usize); }
    if a.i("hpm0", -1) >= 0 { wr(spp, 0x108, a.i("hpm0", 0) as usize); }
    {
        let th = a.s("threats0", "");
        if !th.is_empty() {
            let v: Vec<usize> = th.split(',').map(|s| cache.player_champion[1 - team][s[1..].parse::<usize>().unwrap()].unwrap().id).collect();
            unsafe { std::ptr::write((spp as *mut u8).add(0x48) as *mut Vec<usize>, v); }
        }
    }
    // 적 배치: epos=tower → 내 라인 타워 위치(+p*1000 x 오프셋) · epos=etower → 적 라인 타워
    {
        let epos = a.s("epos", "keep");
        if epos == "tower" || epos == "etower" {
            let t = cache.tower(line, if epos == "tower" { team } else { 1 - team }).expect("tower");
            let edx = a.i("edx", 0) as u64;
            for p in 0..5usize { let e = cache.player_champion[1 - team][p].unwrap(); wr(ep(e), 0x660, t.x + edx + (p as u64) * 1000); wr(ep(e), 0x668, t.y); }
        }
    }
    // 적 라인 타워 nearest_enemy 태그(0x88) 세팅(L1255) · 내 챔피언 귀환 상태(0x70=1 Return, 0x78=time)(L1248)
    if a.i("etnear", -1) >= 0 { let t = cache.tower(line, 1 - team).expect("etower"); wr(ep(t), 0x88, a.i("etnear", 0) as i64); }
    if a.i("ret", -1) >= 0 { wr(cp, 0x70, 1i64); wr(cp, 0x78, a.i("ret", 0) as usize); }
    let before: [u8; 280] = unsafe { std::ptr::read(spp as *const [u8; 280]) };
    // 세계 요약
    let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
    let f = map.fountains[team];
    let in_f = f.0 <= mx && mx <= f.2 && f.1 <= my && my <= f.3;
    let ph = tick < setting.epic_jungle.first_spawn_tick.saturating_sub(setting.tick_per_second * 30);
    let tw = cache.tower(line, team).map(|t| (t.x, t.y, t.id));
    let etw = cache.tower(line, 1 - team).map(|t| (t.x, t.y, t.id));
    println!("world\ttick={}\tteam={}\tme={}\tline={:?}\tme=({},{})\thp={}/{}\thp_ratio={}\tin_fountain={}\tfountain={:?}\tline_phase={}\tfirst_spawn={}\ttps={}\tmy_tower={:?}\tenemy_tower={:?}\tobj_tag={}\tobj_line={}",
        tick, team, me, line, mx, my, hp, maxhp, if maxhp > 0 { hp * 100 / maxhp } else { 0 }, in_f, f, ph, setting.epic_jungle.first_spawn_tick, setting.tick_per_second, tw, etw, rd::<u8>(tpp, 0x41f), rd::<u8>(tpp, 0x420));
    println!("self_before\t{:?}", sp);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sp.update(version, &mut rnd, player, &data, &tp, &psd, &mut dbgf)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(()) => {
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen();
            let clen: usize = rd(spp, 0x28); let cptr: *const u8 = rd(spp, 0x20);
            let mut chats = String::new();
            for i in 0..clen {
                let e = unsafe { cptr.add(i * 24) };
                let b: [u8; 16] = unsafe { std::ptr::read(e as *const [u8; 16]) };
                chats += &format!("[tag={} +1={} +8={} raw={:?}] ", b[0], b[1], rd::<u64>(e, 8), &b[..16]);
            }
            println!("RESULT\tin_recall={}\tv46_commit={}\tv46_flee={}\tchats_len={}\t{}\trnd_used={}\tdebug_logs={}",
                rd::<u8>(spp, 0x110), rd::<u8>(spp, 0x111), rd::<u8>(spp, 0x112), clen, chats, r1 != r0, dbgf.logs.len());
        }
    }
    println!("self_after\t{:?}", sp);
    let after: [u8; 280] = unsafe { std::ptr::read(spp as *const [u8; 280]) };
    let diff: Vec<String> = (0..280).filter(|&i| before[i] != after[i]).map(|i| format!("+{:#x}:{}->{}", i, before[i], after[i])).collect();
    println!("self_diff\t{}", diff.join(" "));
}
