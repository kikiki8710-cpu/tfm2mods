#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치D 오라클 — 255 end_check(define hidden m14.ll:8724 → `#[link_name]` 직접 진입).
//!  Flexible 경로는 명세 `logic` 독립 재구현(predict) ↔ 실행 대조. Stable/Aggressive 는 콜리 build_game_finish_check_state(internal fastcc · 계약만)의
//!  상태 필드를 예측할 재료가 없어 **관측만**(타워 생존 시 false 기대치만 대조).
//!  세계 제어: game.set_strategy(team, Strategy{game_finish}) · cache.<line>_tower[et]=None · twin_towers[et].clear() · player_champion[et][k]=None ·
//!  아군 좌표/HP raw write · MobaMode.epic_minion_buff_time[team] · bb[team].<line>_minion_state.minion_count · ctx.tutorial · game.set_tick.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify27/D/oracle/o27D_ec.rs → %TEMP%\tfm2_spanprobe\o27D_ec.exe
//!  실행: o27D_ec.exe k=v ... (드라이버 = run27D_ec.py · 케이스당 프로세스 1개 = TLS FINISH_AGG_CACHE 비어 있음)
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvCshdEBA0ozCnw_7game_ai9end_check"]
    fn end_check(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, line: LineType, debug: *mut u8) -> bool;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

fn player_count_of(tut: i64) -> usize { match tut { 0 | 7 | 8 => 4, 1 | 3 => 2, 2 | 4 | 6 => 1, 5 => 3, _ => 4 } }

/// Flexible 독립 재구현(명세 logic L1225~L1271). 반환 (bool, log)
fn predict_flexible(data: &OperationData, team: usize, pos: usize, line: LineType, tut: i64, mc: i32) -> (bool, String) {
    let mut log = String::new();
    let cache = data.cache; let ctx = data.context; let setting = ctx.setting;
    let enemy = 1 - team;
    let tick = cache.game.tick();
    let disable = setting.tower_attack_disable_tick;
    if !(tick > disable) {
        let (t1, t2) = match line {
            LineType::Top => (cache.top_tower[enemy], cache.top_tower2[enemy]),
            LineType::Mid => (cache.mid_tower[enemy], cache.mid_tower2[enemy]),
            LineType::Bottom => (cache.bottom_tower[enemy], cache.bottom_tower2[enemy]),
        };
        if !(t1.is_none() && t2.is_none()) { log += "tower_alive "; return (false, log); }
    }
    let nexus = cache.nexus[enemy].expect("enemy nexus");
    let live_enemy = cache.player_champion[enemy].iter().filter(|e| e.is_some()).count();
    let has_twin = if tick > disable { false } else { cache.twin_towers[enemy].len() != 0 };
    let champ = cache.player_champion[team][pos].expect("champ");
    let allies: Vec<&Entity> = cache.player_champion[team].iter().flatten().copied().collect();
    let near_ally = allies.iter().filter(|a| a.distance_sq(champ) < 1440000000001).count();
    log += &format!("live_enemy={} has_twin={} allies={} near_ally={} ", live_enemy, has_twin, allies.len(), near_ally);
    if near_ally != allies.len() { log += "not_gathered "; return (false, log); }
    let ok_ally = allies.iter().filter(|a| { let mh: u64 = rd(ep(**a), 0x628); let h: u64 = rd(ep(**a), 0x670); h * 100 / mh > 39 }).count();
    let gm = cache.game.get_game_mode();
    let epic = gm.as_moba().map_or(0, |m| m.remain_epic_time(team));
    std::mem::forget(gm);
    let tps = setting.tick_per_second;
    log += &format!("ok_ally={} epic={} tps20={} mc={} ", ok_ally, epic, tps * 20, mc);
    if epic < tps * 20 {
        if live_enemy == 0 { log += "L1260 true "; return (true, log); }
        if has_twin { log += "L1260 twin false "; return (false, log); }
    } else {
        let cond = (ok_ally >= live_enemy.saturating_sub(1) && mc > 5) || live_enemy == 0;
        if cond { log += "L1255 true "; return (true, log); }
        if has_twin { log += "L1255 twin false "; return (false, log); }
    }
    let required_edge = match player_count_of(tut) { 2 => 1, 3 => 2, _ => 3 };
    let near_nexus = allies.iter().filter(|a| a.distance_sq(nexus) < 360000000001).count();
    log += &format!("required_edge={} near_nexus={} ", required_edge, near_nexus);
    (near_nexus >= required_edge + live_enemy, log)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::battle_action as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let mut setting = real_setting();
    if a.has("disable") { setting.tower_attack_disable_tick = a.get("disable", 9999999) as usize; }
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tut = a.get("tut", 0);
    let tutorial = match tut { 0 => TutorialType::None, 1 => TutorialType::First, 2 => TutorialType::TopSolo, 3 => TutorialType::Bottom, 4 => TutorialType::MidSolo,
        5 => TutorialType::MidBottom, 6 => TutorialType::JungleOnly, 7 => TutorialType::Line, _ => TutorialType::Total };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let team = a.get("team", 0) as usize; let et = 1 - team;
    let posi = a.get("pos", 2) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let line = match a.get("line", 1) { 0 => LineType::Top, 2 => LineType::Bottom, _ => LineType::Mid };
    // 전략 세팅
    let mut strat: Strategy = Default::default();
    strat.game_finish = match a.get("strat", 1) { 0 => GameFinishStrategy::Stable, 2 => GameFinishStrategy::Aggressive, _ => GameFinishStrategy::Flexible };
    game.set_strategy(team, strat);
    game.set_tick(a.get("tick", 3000) as usize);
    let version = a.get("version", 2) as usize;
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    // 타워 제거
    if a.get("towers", 1) == 0 {
        match line { LineType::Top => { cache.top_tower[et] = None; cache.top_tower2[et] = None; }
                     LineType::Mid => { cache.mid_tower[et] = None; cache.mid_tower2[et] = None; }
                     LineType::Bottom => { cache.bottom_tower[et] = None; cache.bottom_tower2[et] = None; } }
    }
    if a.has("t1only") { match line { LineType::Top => cache.top_tower2[et] = None, LineType::Mid => cache.mid_tower2[et] = None, LineType::Bottom => cache.bottom_tower2[et] = None } }
    if a.get("twin", 1) == 0 { cache.twin_towers[et].clear(); }
    // 적 생존 수
    let ek = a.get("ek", 5) as usize;
    for k in ek..5 { cache.player_champion[et][k] = None; }
    if a.has("cnone") { cache.player_champion[team][posi] = None; }
    let champ_ptr = cache.player_champion[team][posi].map(|e| ep(e));
    // 아군 좌표·HP
    if let Some(cp) = champ_ptr {
        let (cx, cy): (u64, u64) = (rd(cp, 0x660), rd(cp, 0x668));
        let nexus = cache.nexus[et].expect("nexus");
        let (nx, ny) = (nexus.x, nexus.y);
        for (k, e) in cache.player_champion[team].iter().enumerate() {
            if let Some(e) = e {
                let p = ep(*e);
                if a.has("ahp") { wr(p, 0x628, 1000i64); wr(p, 0x670, a.get("ahp", 1000) as i64); }
                if a.has("gather") { let d = a.get("gather", 0) as u64; wr(p, 0x660, cx.wrapping_add(d * k as u64)); wr(p, 0x668, cy); }
                if a.has("nex") { let d = a.get("nex", 0) as u64; wr(p, 0x660, nx.wrapping_add(d * (k as u64 + 1))); wr(p, 0x668, ny); }
            }
        }
        if a.has("far") { // k 번째 아군 하나를 멀리
            let k = a.get("far", 0) as usize;
            if let Some(e) = cache.player_champion[team][k] { let p = ep(e); wr(p, 0x660, cx.wrapping_add(a.get("fard", 1300000) as u64)); wr(p, 0x668, cy); }
        }
        if a.has("lowk") { // k 번째 아군 hp 를 낮게
            let k = a.get("lowk", 0) as usize;
            if let Some(e) = cache.player_champion[team][k] { let p = ep(e); wr(p, 0x628, 1000i64); wr(p, 0x670, a.get("lowhp", 390) as i64); }
        }
        if a.has("nexk") { // k 번째 아군만 적 넥서스 옆
            let k = a.get("nexk", 0) as usize;
            if let Some(e) = cache.player_champion[team][k] { let p = ep(e); wr(p, 0x660, nx.wrapping_add(a.get("nexkd", 100000) as u64)); wr(p, 0x668, ny); }
        }
    }
    // 에픽 버프
    if a.has("ebuff") {
        let gm = cache.game.get_game_mode();
        let (disc, mp): (i64, *const u8) = unsafe { std::mem::transmute_copy(&gm) }; std::mem::forget(gm);
        if disc == 0 { wr(mp, 0x240 + team * 8, a.get("ebuff", 0) as u64); } else { println!("WARN game mode disc={}", disc); }
    }
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let mc = a.get("mc", 0) as i32;
    match line { LineType::Top => bb[team].top_minion_state.minion_count = mc, LineType::Mid => bb[team].mid_minion_state.minion_count = mc, LineType::Bottom => bb[team].bottom_minion_state.minion_count = mc }
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let rnd_before: [u8; 320] = unsafe { std::mem::transmute_copy(&rnd) };
    let mut dbg = [0u8; 224];
    let st = player.strategy(&mut rnd, cache.game);
    println!("world tick={} strat.game_finish={:?} line={:?} tut={} towers_et=({:?},{:?}) twin_et={} live_enemy={} allies={} nexus_et={} epic={} pc={}",
        cache.game.tick(), st.game_finish, line, tut,
        match line { LineType::Top => cache.top_tower[et].is_some(), LineType::Mid => cache.mid_tower[et].is_some(), LineType::Bottom => cache.bottom_tower[et].is_some() },
        match line { LineType::Top => cache.top_tower2[et].is_some(), LineType::Mid => cache.mid_tower2[et].is_some(), LineType::Bottom => cache.bottom_tower2[et].is_some() },
        cache.twin_towers[et].len(), cache.player_champion[et].iter().filter(|e| e.is_some()).count(), cache.player_champion[team].iter().filter(|e| e.is_some()).count(),
        cache.nexus[et].is_some(), { let gm = cache.game.get_game_mode(); let v = gm.as_moba().map_or(0, |m| m.remain_epic_time(team)); std::mem::forget(gm); v }, ctx.tutorial.player_count());
    let got = unsafe { end_check(version, &mut rnd, player, &data, line, dbg.as_mut_ptr()) };
    let rnd_after: [u8; 320] = unsafe { std::mem::transmute_copy(&rnd) };
    let rnd_same = rnd_before == rnd_after;
    let strat_i = a.get("strat", 1);
    if strat_i == 1 {
        let (pred, log) = predict_flexible(&data, team, posi, line, tut, mc);
        println!("got\t{}\tpred\t{}\trnd_same={}\t{}", got, pred, rnd_same, log);
        println!("{}", if got == pred { "MATCH" } else { "DIFF" });
    } else {
        println!("got\t{}\tpred\tOBS\trnd_same={}\t(Stable/Aggressive 관측만)", got, rnd_same);
        println!("OBS");
    }
}
