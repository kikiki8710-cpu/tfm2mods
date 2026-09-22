#![allow(unused, dead_code, non_snake_case, invalid_reference_casting)]
#![feature(thread_local)]
//! 25차 배치 I 오라클 — 202 LineGankerPlan::next_plan(pub) 진리표.
//!  한 프로세스 = 한 케이스 = 한 주체(who=game | who=mine). 같은 argv 로 두 프로세스를 돌려 드라이버(run202.py)가 대조한다
//!  (같은 프로세스에서 game→mine 순으로 부르면 TLS 메모(DieTickCache 등)가 mine 쪽에 재생돼 오염 — TEMPLATE 함정 ③).
//!  세계 = TEMPLATE mkgame. 엔티티 변조 = raw write_volatile(함정 ⑦). 플레이어 = team 0 · Jungle(slot 1).
//!  sret 살아있는 바이트 = 384B 버퍼를 0xAA 로 채우고 **심볼 직접 링크(raw sret 포인터)** 로 호출해 변한 범위를 센다.
//!  `mine` = 명세 logic 을 독립 재구현. make_gank_battle(internal fastcc, 링크 불가)은 콜리 계약(r14 #155 · IR 94354~)대로
//!  open_chase_race_hopeless(hidden · link_name) + BattlePlan::new/update(pub) 로 재현.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/I/oracle/o202.rs
//! 실행: o202.exe who=game k=v ...   (드라이버 = run202.py)
use game_core::*;
use rand::SeedableRng;
use rand::RngCore;
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old::{LineGankerPlan, LineGankerPhase, BattlePlan, BattlePlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::plan_legacy::types::BigPlan;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

extern "Rust" {
    // ★raw sret: Rust ABI 에서 sret 포인터는 첫 인자 자리(RCX) — 정의의 %0 와 같은 자리. 0xAA 프리필 버퍼를 넘겨 기록 범위를 잰다.
    #[link_name = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan9next_plan"]
    fn next_plan_raw(out: *mut u8, this: *mut LineGankerPlan, version: usize, rnd: *mut rand::rngs::StdRng,
                     player: *const PlayerState, data: *const OperationData, ps: *const PositioningScoreData,
                     tp: *const TeamPlan, dbg: *mut DebugFrameData);
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model24open_chase_race_hopeless"]
    fn open_chase_race_hopeless(version: usize, data: &OperationData, player: &PlayerState, a: &Entity, b: &Entity) -> bool;
}

struct Args(HashMap<String, String>);
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.0.get(k).and_then(|s| s.parse().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.0.get(k).cloned().unwrap_or(d.to_string()) }
    fn has(&self, k: &str) -> bool { self.0.contains_key(k) }
}

fn set_attack_damage(e: *const Entity, dmg: usize) {
    // Effect.ty(Arc<dyn EffectType>, Entity+0x490 팻포인터 16B) 를 TowerAttackEffect(dmg, 0) 로 교체(22차 A 수법 · 옛 Arc 누수)
    let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
    let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
    wr(ep(e), 0x490, raw[0]); wr(ep(e), 0x498, raw[1]);
}
fn dist2(a: &Entity, b: &Entity) -> u64 { a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) }
fn radius(e: &Entity) -> u64 { let m: i32 = rd(ep(e), 0x470); let r: u64 = rd(ep(e), 0x680); if m == 0 { r } else { r * ((m as i64 + 100) as u64) / 100 } }

#[derive(Debug, Clone, PartialEq)]
enum Out { None(&'static str), Battle { site: &'static str, entry_src: u8, goal_tag: i64, target: usize, min_trace: usize } }

/// make_gank_battle 재현(콜리 계약, IR m08.ll 94354~94516). Some(bp 280B) / None
fn mgb(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData,
       tp: &TeamPlan, target: usize, min_trace: usize, dbg: &mut DebugFrameData, g: &dyn AbstractGame) -> Option<BattlePlan> {
    if version <= 1 {
        let mut bp = BattlePlan::new(version, BattlePlanGoal::TryKill(target, min_trace), data, player);
        wr(&bp as *const _ as *const u8, 0x107, 7u8);
        return Some(bp);
    }
    let team = player.info.team; let pos = player.info.position as usize;
    let champ = data.cache.player_champion[team][pos];
    let te = g.get_entity_by_id(target);
    if let (Some(c), Some(t)) = (champ, te) {
        if unsafe { open_chase_race_hopeless(version, data, player, c, t) } { return None; }
    }
    let mut bp = BattlePlan::new(version, BattlePlanGoal::TryKill(target, min_trace), data, player);
    let bpp = &bp as *const _ as *const u8;
    wr(bpp, 0x107, 7u8);
    let tpp = tp as *const _ as *const u8;
    for k in 0..3usize { wr(bpp, 0xff + k, rd::<u8>(tpp, 0x41f + k)); }
    bp.update(version, rnd, player, data, ps, tp, dbg);
    let sg: i64 = rd(bpp, 0x58);
    if sg == 3 || sg == 4 || sg == 7 { return None; }
    Some(bp)
}

fn bump_vec<'a>(pool: &'a bumpalo::Bump, v: &[usize]) -> bumpalo::collections::Vec<'a, usize> {
    let mut b = bumpalo::collections::Vec::new_in(pool);
    for &x in v { b.push(x); }
    b
}

/// ★명세 logic(ganker.rs:99~246)의 독립 재구현. 반환 = 결과 + make_gank_battle 시도 목록 + chats push 대상
fn mine(version: usize, rnd: &mut rand::rngs::StdRng, plan: &LineGankerPlan, player: &PlayerState, data: &OperationData,
        ps: &PositioningScoreData, tp: &TeamPlan, dbg: &mut DebugFrameData, g: &dyn AbstractGame, ctx: &GameContext,
        bush_from_game: Option<u64>, log: &mut Vec<String>) -> (Out, Vec<(&'static str, usize, usize)>, Option<usize>) {
    let pp = plan as *const _ as *const u8;
    let phase_tag: u8 = rd(pp, 0x29);
    let line_tag: u8 = rd(pp, 0x28);
    let wait_limit: usize = rd(pp, 0x20);
    let mut attempts: Vec<(&'static str, usize, usize)> = Vec::new();
    let mut chat: Option<usize> = None;
    if phase_tag < 6 { return (Out::None("L102"), attempts, chat); }
    let team = player.info.team; let pos = player.info.position as usize;
    let champ = data.cache.player_champion[team][pos].expect("champ");
    let enemy = 1 - team;
    let bb = &data.blackboard[enemy];
    let vis = g.is_visible(enemy, champ.id);
    let jungle = if vis { is_in_jungle_area(ctx, champ.x, champ.y) } else { false };
    log.push(format!("L107 vis={} jungle={}", vis, jungle));
    if vis && jungle {
        let hpok = |x: &Entity| -> bool { let mh: u64 = rd(ep(x), 0x628); if mh == 0 { panic!("div0") } (x.hp as u64) * 100 / mh > 39 };
        let allies = data.cache.player_champion[team].iter().flatten().filter(|x| dist2(x, champ) < 62500000001 && hpok(x)).count();
        let enemies = data.cache.player_champion[enemy].iter().flatten().filter(|x| bb.is_recent_visible(g, player, x) && dist2(x, champ) < 62500000001 && hpok(x)).count();
        let mut best: Option<(u64, &Entity)> = None;
        for x in data.cache.player_champion[enemy].iter().flatten() {
            if !bb.is_recent_visible(g, player, x) { continue; }
            let d = dist2(x, champ);
            if best.map_or(true, |(bd, _)| d < bd) { best = Some((d, x)); }
        }
        log.push(format!("L113 allies={} enemies={} nearest={:?}", allies, enemies, best.map(|b| b.1.id)));
        if allies > enemies && enemies != 0 {
            if let Some((_, e)) = best {
                attempts.push(("L127", e.id, 60));
                if let Some(bp) = mgb(version, rnd, player, data, ps, tp, e.id, 60, dbg, g) {
                    return (Out::Battle { site: "L127", entry_src: 7, goal_tag: 0, target: e.id, min_trace: 60 }, attempts, chat);
                }
            }
        } else if let Some((_, e)) = best {
            chat = Some(e.id);
            return (Out::Battle { site: "L136", entry_src: 3, goal_tag: 2, target: 0, min_trace: 0 }, attempts, chat);
        }
    }
    // L142 target_bush 인라인 — 값은 버려지고 unwrap 패닉 부작용만. 패닉 조건만 재현한다.
    {
        let line = plan_line(line_tag);
        let cache = data.cache;
        let optb = match line_tag { 0 => cache.top_tower[team].or(cache.top_tower2[team]), 1 => cache.mid_tower[team].or(cache.mid_tower2[team]), _ => cache.bottom_tower[team].or(cache.bottom_tower2[team]) };
        let sp = line.get_start_position(ctx.setting, team);
        let t = cache.twin_towers[team].iter().min_by_key(|t| t.x.abs_diff(sp.0).pow(2) + t.y.abs_diff(sp.1).pow(2));
        let nearest_tower = optb.or(t.copied());
        let nexus = cache.nexus[team].expect("L363 nexus unwrap");
        let nearest_tower = nearest_tower.unwrap_or(nexus);
        let ms = data.blackboard[team].minion_state(line);
        let front = ms.front_minion.and_then(|m| g.get_entity_by_id(m));
        let reference = front.filter(|x| dist2(x, nexus) >= dist2(nearest_tower, nexus));
        let r = match reference {
            Some(x) => near_jungle_bush(ctx, champ.x, champ.y, x.x, x.y),
            None => { let c = ctx.map.lane_path(line, team)[3]; near_jungle_bush(ctx, champ.x, champ.y, c.0, c.1) }
        };
        log.push(format!("L142 target_bush={:?} front={:?}", r, front.map(|f| f.id)));
        r.expect("L374 near_jungle_bush unwrap");
    }
    // L144 target_bush_v41 = internal fastcc(링크 불가) → 게임 프로세스의 debug 텍스트에서 받은 값
    let bush = match bush_from_game { Some(b) => b, None => { log.push("L144 bush 미상(게임 debug 텍스트 필요)".into()); return (Out::None("L144?"), attempts, chat); } };
    let champ_bush = ctx.map.bushes[std::cmp::min(champ.y / 32000, 29) as usize][std::cmp::min(champ.x / 32000, 29) as usize] as u64;
    log.push(format!("L148 bush={} champ_bush={}", bush, champ_bush));
    if champ_bush != bush { return (Out::None("L155"), attempts, chat); }
    let cut: u64 = 150000;
    let na: Vec<usize> = (0..5).filter(|&i| data.cache.player_champion[team][i].is_some_and(|x| dist2(x, champ) <= cut * cut)).collect();
    let ne: Vec<usize> = (0..5).filter(|&i| data.cache.player_champion[enemy][i].is_some_and(|c| bb.is_recent_visible(g, player, c) && dist2(c, champ) <= cut * cut)).collect();
    let mut tower: Option<&Entity> = None; let mut td = u64::MAX;
    for t in data.cache.iter_towers_without_nexus(enemy) { let d = dist2(t, champ); if tower.is_none() || d < td { tower = Some(t); td = d; } }
    log.push(format!("L160 na={:?} ne={:?} tower={:?}", na, ne, tower.map(|t| t.id)));
    let tps = ctx.setting.tick_per_second;
    for &e in ne.iter() {
        let e_ent = data.cache.player_champion[enemy][e].expect("L190");
        let na2: Vec<usize> = (0..5).filter(|&i| data.cache.player_champion[team][i].is_some_and(|c| c.id == champ.id || dist2(c, e_ent) <= cut * cut)).collect();
        let ne2: Vec<usize> = (0..5).filter(|&i| data.cache.player_champion[enemy][i].is_some_and(|x| dist2(x, e_ent) <= cut * cut)).collect();
        let bna = bump_vec(ctx.pool, &na2); let bne = bump_vec(ctx.pool, &ne2);
        let edt = game_ai::get_die_tick_player(data, enemy, e, &bna, None, None);
        let mdt = game_ai::get_die_tick_player(data, team, pos, &bne, None, None);
        let mtt = match tower { Some(t) => { let ms: u64 = rd(ep(e_ent), 0x640); if ms == 0 { panic!("div0 L194") } e_ent.distance(t) / ms }, None => 99999999 };
        log.push(format!("L187 e={} na2={:?} ne2={:?} edt={} mdt={} mtt={}", e, na2, ne2, edt, mdt, mtt));
        if edt > 30 && na2.len() <= ne2.len() { log.push(format!("L205 continue e={}", e)); continue; }
        if mtt <= 120 {
            if edt <= 60 {
                attempts.push(("L212", e_ent.id, 60));
                if mgb(version, rnd, player, data, ps, tp, e_ent.id, 60, dbg, g).is_some() { return (Out::Battle { site: "L212", entry_src: 7, goal_tag: 0, target: e_ent.id, min_trace: 60 }, attempts, chat); }
            } else if na2.len() > ne2.len() {
                attempts.push(("L214", e_ent.id, 60));
                if mgb(version, rnd, player, data, ps, tp, e_ent.id, 60, dbg, g).is_some() { return (Out::Battle { site: "L214", entry_src: 7, goal_tag: 0, target: e_ent.id, min_trace: 60 }, attempts, chat); }
            }
        } else if mdt > 120 && na2.len() >= ne2.len() {
            attempts.push(("L217", e_ent.id, 60));
            if mgb(version, rnd, player, data, ps, tp, e_ent.id, 60, dbg, g).is_some() { return (Out::Battle { site: "L217", entry_src: 7, goal_tag: 0, target: e_ent.id, min_trace: 60 }, attempts, chat); }
        } else if wait_limit.saturating_sub(g.tick()) > tps * 3 {
            log.push(format!("L218 wait e={} rem={} > {}", e, wait_limit.saturating_sub(g.tick()), tps * 3));
        } else if na2.len() > ne2.len() {
            attempts.push(("L221", e_ent.id, 60));
            if mgb(version, rnd, player, data, ps, tp, e_ent.id, 60, dbg, g).is_some() { return (Out::Battle { site: "L221", entry_src: 7, goal_tag: 0, target: e_ent.id, min_trace: 60 }, attempts, chat); }
        }
    }
    let ck = game_ai::plan_legacy::handler::check_kill(version, rnd, player, data, ps, dbg);
    log.push(format!("L225 check_kill={:?} na.len={} ne.len={}", ck, na.len(), ne.len()));
    if let Some((e, t)) = ck {
        if na.len() > ne.len() {
            attempts.push(("L227", e, t));
            if mgb(version, rnd, player, data, ps, tp, e, t, dbg, g).is_some() { return (Out::Battle { site: "L227", entry_src: 7, goal_tag: 0, target: e, min_trace: t }, attempts, chat); }
        }
    }
    let mut best: Option<(u64, &Entity)> = None;
    for x in data.cache.player_champion[enemy].iter().flatten() {
        if !bb.is_recent_visible(g, player, x) { continue; }
        let d = dist2(x, champ);
        if best.map_or(true, |(bd, _)| d < bd) { best = Some((d, x)); }
    }
    if let Some((_, ne_)) = best {
        let d = ne_.distance(champ);
        let eff = champ.attack_effect.as_ref().expect("L237 attack_effect");
        let sbr: u64 = rd(ep(champ), 0x438);
        let range = sbr + eff.range + (champ.level as u64 - 1) * eff.growth_range + eff.range_adjust(champ, ne_) + radius(champ) + radius(ne_);
        log.push(format!("L236 ne={} d={} attack_range={} (sbr={} eff.range={} growth={} lvl={} adj={} r1={} r2={})", ne_.id, d, range, sbr, eff.range, eff.growth_range, champ.level, eff.range_adjust(champ, ne_), radius(champ), radius(ne_)));
        if d > range { return (Out::None("L240"), attempts, chat); }
        attempts.push(("L241", ne_.id, 60));
        return match mgb(version, rnd, player, data, ps, tp, ne_.id, 60, dbg, g) {
            Some(_) => (Out::Battle { site: "L241", entry_src: 7, goal_tag: 0, target: ne_.id, min_trace: 60 }, attempts, chat),
            None => (Out::None("L241-None"), attempts, chat),
        };
    }
    (Out::None("L245"), attempts, chat)
}

fn plan_line(tag: u8) -> LineType { match tag { 0 => LineType::Top, 1 => LineType::Mid, _ => LineType::Bottom } }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut m = HashMap::new();
    for s in a.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.to_string()); } }
    let A = Args(m);
    if a.len() > 99 { let _ = game_ai::interaction_score as *const (); }
    let who = A.s("who", "game");
    let version = A.i("version", 2) as usize;
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let debug = A.i("debug", 1) == 1;
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick = A.i("tick", 3000) as usize;
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let g: &dyn AbstractGame = &game;
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let player = game.get_player_by_position(0, Position::Jungle).expect("player");
    let team = player.info.team; let pos = player.info.position as usize;
    let champ = cache.player_champion[team][pos].expect("champ");
    let cp = ep(champ);

    if who == "scan" {
        // 좌표 정찰: 정글 영역 / bushes / 레인 경로 / 타워 / 챔프 초기 위치
        for t in 0..2usize { for p in 0..5usize { if let Some(e) = cache.player_champion[t][p] { println!("champ\tt={} p={} id={} xy=({},{}) hp={} maxhp={} ms={} lvl={} radius={} eff={:?}", t, p, e.id, e.x, e.y, e.hp, rd::<u64>(ep(e), 0x628), rd::<u64>(ep(e), 0x640), e.level, e.radius, e.attack_effect.as_ref().map(|f| (f.range, f.growth_range))); } } }
        for t in 0..2usize { println!("towers\tt={} {:?}\tnexus={:?}\ttop={:?} mid={:?} bot={:?}", t, cache.iter_towers_without_nexus(t).map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>(), cache.nexus[t].map(|n| (n.id, n.x, n.y)), cache.top_tower[t].map(|n| (n.x, n.y)), cache.mid_tower[t].map(|n| (n.x, n.y)), cache.bottom_tower[t].map(|n| (n.x, n.y))); }
        for (li, l) in [LineType::Top, LineType::Mid, LineType::Bottom].iter().enumerate() { println!("lane\tline={} t0={:?}\tstart0={:?}", li, map.lane_path(*l, 0), l.get_start_position(&setting, 0)); }
        let mut rows = Vec::new();
        for gy in 0..30u64 { let mut row = String::new(); for gx in 0..30u64 { let x = gx * 32000 + 16000; let y = gy * 32000 + 16000; let j = is_in_jungle_area(&ctx, x, y); let b = map.bushes[gy as usize][gx as usize]; row.push_str(&format!("{}{:<3}", if j { 'J' } else { '.' }, b)); } rows.push(format!("y{:02} {}", gy, row)); }
        for r in rows { println!("{}", r); }
        return;
    }

    // ---- 세계 조작(★함정 ⑦: raw write 는 별도 #[inline(never)] 함수에서, 판정은 그 뒤 새 함수에서 새 참조로) ----
    mutate(&A, &cache, tick, &mut bb, cp);
    run(&A, &game, &ctx, &bb, &setting, &map);
}

#[inline(never)]
fn mutate(A: &Args, cache: &AbstractGameWithCache, tick: usize, bb: &mut [Blackboard; 2], cp: *const u8) {
    let pos = 1usize;
    let champ_x0: u64 = rd(cp, 0x660); let champ_y0: u64 = rd(cp, 0x668);
    let cx = A.i("cx", champ_x0 as i64) as u64; let cy = A.i("cy", champ_y0 as i64) as u64;
    wr(cp, 0x660, cx); wr(cp, 0x668, cy);
    if A.has("hp") { wr(cp, 0x670, A.i("hp", 0) as usize); }
    if A.has("maxhp") { wr(cp, 0x628, A.i("maxhp", 0) as usize); }
    if A.has("ms") { wr(cp, 0x640, A.i("ms", 0) as usize); }
    if A.has("cdmg") { set_attack_damage(cp as *const Entity, A.i("cdmg", 0) as usize); }
    if A.i("vis", 0) == 1 { unsafe { std::ptr::write_volatile(&mut (*(cp as *mut Entity)).visible_state[1], VisibleState::Visible); } }
    // 적 e0..e4: 기본 멀리(맵 반대 구석) · 아군 a1..a4: 기본 멀리
    for i in 0..5usize {
        if let Some(e) = cache.player_champion[1][i] {
            let k = format!("e{}", i);
            let x = A.i(&format!("{}x", k), 940000) as u64; let y = A.i(&format!("{}y", k), 940000) as u64;
            wr(ep(e), 0x660, x); wr(ep(e), 0x668, y);
            if A.has(&format!("{}hp", k)) { wr(ep(e), 0x670, A.i(&format!("{}hp", k), 0) as usize); }
            if A.has(&format!("{}ms", k)) { wr(ep(e), 0x640, A.i(&format!("{}ms", k), 0) as usize); }
            if A.has(&format!("{}maxhp", k)) { wr(ep(e), 0x628, A.i(&format!("{}maxhp", k), 0) as usize); }
            if A.has(&format!("{}dmg", k)) { set_attack_damage(e, A.i(&format!("{}dmg", k), 0) as usize); }
            if A.has(&format!("{}range", k)) { wr(ep(e), 0x4a0, A.i(&format!("{}range", k), 0) as u64); }
            if A.i(&format!("{}seen", k), 0) == 1 { bb[1].last_visible[i] = tick; }
            if A.i(&format!("{}vis", k), 0) == 1 { unsafe { std::ptr::write_volatile(&mut (*(ep(e) as *mut Entity)).visible_state[0], VisibleState::Visible); } }
        }
        if i != pos { if let Some(e) = cache.player_champion[0][i] {
            let k = format!("a{}", i);
            let x = A.i(&format!("{}x", k), 20000) as u64; let y = A.i(&format!("{}y", k), 20000) as u64;
            wr(ep(e), 0x660, x); wr(ep(e), 0x668, y);
            if A.has(&format!("{}hp", k)) { wr(ep(e), 0x670, A.i(&format!("{}hp", k), 0) as usize); }
            if A.has(&format!("{}maxhp", k)) { wr(ep(e), 0x628, A.i(&format!("{}maxhp", k), 0) as usize); }
            if A.has(&format!("{}dmg", k)) { set_attack_damage(e, A.i(&format!("{}dmg", k), 0) as usize); }
        } }
    }
}

#[inline(never)]
fn run(A: &Args, game: &Game, ctx: &GameContext, bb: &[Blackboard; 2], setting: &GameSetting, map: &MapDef) {
    let who = A.s("who", "game");
    let version = A.i("version", 2) as usize;
    let ok = true;
    let cache = AbstractGameWithCache::new(game as &dyn AbstractGame, ctx);
    let g: &dyn AbstractGame = game;
    let player = game.get_player_by_position(0, Position::Jungle).expect("player");
    let team = player.info.team; let pos = player.info.position as usize;
    let champ = cache.player_champion[team][pos].expect("champ");
    let cp = ep(champ);
    let ctx = ctx;
    let data = OperationData::new(&cache, ctx, bb);
    let ps: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();
    let line = plan_line(A.i("line", 1) as u8);
    let phase = match A.i("phase", 0) { 1 => LineGankerPhase::Setup, 2 => LineGankerPhase::Cancel, 3 => LineGankerPhase::ChangeJungle(JungleType::Rhino), _ => LineGankerPhase::WaitResponse };
    let wait_limit = A.i("wait", 0) as usize;
    let mut plan = LineGankerPlan::new_with_phase(line, A.i("setup", 0) as usize, wait_limit, phase);
    let pp = &plan as *const _ as *const u8;
    let self_before: [u8; 48] = rd(pp, 0);
    let rnd0 = rand::rngs::StdRng::seed_from_u64(A.i("seed", 9) as u64);
    let (rx, ry): (u64, u64) = (rd(cp, 0x660), rd(cp, 0x668));
    println!("world\tok={}\tversion={}\ttick={}\tteam={} pos={} champ={} xy=({},{}) raw=({},{}) hp={} maxhp={} vis={} jungle={} jungle_raw={} jungle_lit={}\tself={:02x?}",
             ok, version, g.tick(), team, pos, champ.id, champ.x, champ.y, rx, ry, champ.hp, rd::<u64>(cp, 0x628), g.is_visible(1 - team, champ.id), is_in_jungle_area(ctx, champ.x, champ.y), is_in_jungle_area(ctx, rx, ry), is_in_jungle_area(ctx, 432000, 656000), &self_before[..]);
    for i in 0..5usize { if let Some(e) = cache.player_champion[1][i] { println!("enemy\ti={} id={} xy=({},{}) hp={} d={} d2={} seen={}", i, e.id, e.x, e.y, e.hp, e.distance(champ), dist2(e, champ), bb[1].is_recent_visible(g, player, e)); } }
    for i in 0..5usize { if let Some(e) = cache.player_champion[0][i] { println!("ally\ti={} id={} xy=({},{}) hp={} d2={}", i, e.id, e.x, e.y, e.hp, dist2(e, champ)); } }

    if who == "mine" {
        let mut rnd = rnd0.clone();
        let mut dbg: DebugFrameData = Default::default();
        let bush = if A.has("bush") { Some(A.i("bush", 0) as u64) } else { None };
        let mut log = Vec::new();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| mine(version, &mut rnd, &plan, player, &data, &ps, &tp, &mut dbg, g, ctx, bush, &mut log)));
        for l in &log { println!("mine.log\t{}", l); }
        match r {
            Ok((out, attempts, chat)) => {
                let rc = rnd.clone().next_u64() != rnd0.clone().next_u64();
                println!("MINE\tout={:?}\tattempts={:?}\tchat={:?}\trnd_changed={}", out, attempts, chat, rc);
            }
            Err(e) => println!("MINE\tPANIC"),
        }
        return;
    }

    // ---- who=game: 실제 함수. raw sret 로 384B 기록 범위까지 잰다 ----
    let mut rnd = rnd0.clone();
    let mut dbg: DebugFrameData = Default::default();
    let mut out = [0xAAu8; 384];
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        next_plan_raw(out.as_mut_ptr(), &mut plan as *mut _, version, &mut rnd as *mut _, player as *const _, &data as *const _, &ps as *const _, &tp as *const _, &mut dbg as *mut _);
    }));
    if r.is_err() { println!("GAME\tPANIC"); return; }
    let tag: i64 = rd(out.as_ptr(), 0);
    // 기록 범위(0xAA 가 아닌 바이트) 런 목록
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0; while i < 384 { if out[i] != 0xAA { let s = i; while i < 384 && out[i] != 0xAA { i += 1; } runs.push((s, i)); } else { i += 1; } }
    let self_after: [u8; 48] = rd(pp, 0);
    let sdiff: Vec<usize> = (0..48).filter(|&k| self_before[k] != self_after[k]).collect();
    let chats_len: usize = rd(pp, 0x10);
    let chat0 = if chats_len > 0 { let p: *const u8 = rd(pp, 0x8); Some((rd::<u8>(p, 0), rd::<usize>(p, 8), rd::<usize>(p, 16))) } else { None };
    let rc = rnd.clone().next_u64() != rnd0.clone().next_u64();
    // debug 텍스트/인포
    let mut bush_txt = String::new();
    for t in dbg.texts.iter() { let s = format!("{:?}", t); if s.contains("bush") { bush_txt = s; } }
    let mut infos = Vec::new();
    for (k, v) in dbg.infos.iter() { for s in v { infos.push(format!("{}:{}", k, s)); } }
    if tag == -1 {
        println!("GAME\ttag=None(-1)\truns={:?}\tself_diff={:?}\tchats_len={} chat0={:?}\trnd_changed={}\tdbg_texts={} infos={}", runs, sdiff, chats_len, chat0, rc, dbg.texts.len(), infos.len());
    } else {
        let bp = unsafe { out.as_ptr().add(8) };
        let entry_src: u8 = rd(bp, 0x107);
        let goal_tag: i64 = rd(bp, 0x40); let target: usize = rd(bp, 0x48); let mt: usize = rd(bp, 0x50);
        let sub_goal: i64 = rd(bp, 0x58);
        let mo: [u8; 3] = rd(bp, 0xff);
        println!("GAME\ttag={}\tentry_src={}\tgoal_tag={}\ttarget={}\tmin_trace={}\tsub_goal={}\tmain_objective={:?}\truns={:?}\tself_diff={:?}\tchats_len={} chat0={:?}\trnd_changed={}\tdbg_texts={} infos={}",
                 tag, entry_src, goal_tag, target, mt, sub_goal, mo, runs, sdiff, chats_len, chat0, rc, dbg.texts.len(), infos.len());
    }
    println!("BUSHTXT\t{}", bush_txt);
    for s in infos { println!("INFO\t{}", s); }
    // 정상 시그니처 호출로 raw sret ABI 교차검증(태그·entry_src 일치해야) — 별도 plan/rnd 사본
    if A.i("xcheck", 1) == 1 {
        let mut plan2 = LineGankerPlan::new_with_phase(line, A.i("setup", 0) as usize, wait_limit, match A.i("phase", 0) { 1 => LineGankerPhase::Setup, 2 => LineGankerPhase::Cancel, 3 => LineGankerPhase::ChangeJungle(JungleType::Rhino), _ => LineGankerPhase::WaitResponse });
        let mut rnd2 = rnd0.clone(); let mut dbg2: DebugFrameData = Default::default();
        let r2 = plan2.next_plan(version, &mut rnd2, player, &data, &ps, &tp, &mut dbg2);
        let p2 = &r2 as *const _ as *const u8;
        let tag2: i64 = rd(p2, 0);
        let es2: u8 = if tag2 == -1 { 0 } else { rd(p2, 8 + 0x107) };
        println!("XCHECK\ttag2={}\tentry_src2={}\tsame_tag={}\tis_battle={}", tag2, es2, tag2 == tag, matches!(r2, Some(BigPlan::Battle(_))));
    }
}
