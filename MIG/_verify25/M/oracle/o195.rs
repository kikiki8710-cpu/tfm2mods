#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치M · 195 LineWaitSubPlan::action_candidates 오라클 (pub 직접 호출) — 명세 `logic` 독립 재구현(predict) ↔ 실행 대조.
//!  한 프로세스 = 한 케이스(argv `sc=<n> k=v ...`). 세계 = TEMPLATE mkgame(+미니언 설정, `minions=1` 이면 run_tick N).
//!  대조: ①sret Vec 원소 수·variant 태그(+0xb1) ②variant 별 live 바이트(콜리 define `initializes` + AroundPosition::new store 범위)
//!        ③&mut self 1B ④rnd 소비 동기(복제 rng) ⑤behind() 루프 상계(1..=move_index vs 1..move_index) 를 좌표로 판별
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/M/oracle/o195.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, range: u64, casting: CastingType, target: CastingTarget) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target, attack_type: AttackType::BaseAttack, casting }
}

fn minion_setting(s: &mut GameSetting) {
    s.minion_wave_setting.start_tick = 10; s.minion_wave_setting.tick_per_wave = 660;
    s.minion_wave_setting.melee_count = 2; s.minion_wave_setting.range_count = 1;
    s.minion_wave_setting.tick_per_spawn = 30; s.minion_wave_setting.growth_start_tick = 1800;
    s.minion_wave_setting.growth_tick = 1800; s.minion_wave_setting.growth_tick_2v2 = 400;
    s.minion_wave_setting.growth_tick_3v3 = 800; s.minion_wave_setting.growth_end_tick = 24000;
    s.minion_wave_setting.growth_end_tick_2v2 = 30000; s.minion_wave_setting.growth_end_tick_3v3 = 24000;
    s.minion_wave_setting.exp_range = 150000; s.minion_wave_setting.exp_decay2 = 100;
    s.minion_wave_setting.exp_decay3 = 80; s.minion_wave_setting.exp_decay4 = 60;
    s.melee_minion.stat.attack = 10; s.melee_minion.stat.hp = 400; s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1; s.melee_minion.growth.hp = 30; s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100; s.melee_minion.attack.range = 3000; s.melee_minion.attack.cooltime = 30;
    s.melee_minion.attack.duration = 24; s.melee_minion.attack.start_timing = 16; s.melee_minion.exp = 40; s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15; s.range_minion.stat.hp = 250; s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1; s.range_minion.growth.hp = 20; s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000; s.range_minion.attack.speed = 3000; s.range_minion.attack.cooltime = 40;
    s.range_minion.attack.duration = 24; s.range_minion.attack.start_timing = 16; s.range_minion.exp = 30; s.range_minion.gold = 20;
}

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } }

fn dist_sq(a: &Entity, b: &Entity) -> u64 {
    let dx = a.x.abs_diff(b.x); let dy = a.y.abs_diff(b.y);
    dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx))
}
fn dist_sq_xy(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by);
    dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx))
}

fn raw184(p: &game_ai::SmallActionPlay) -> [u8; 184] { unsafe { std::ptr::read(p as *const _ as *const [u8; 184]) } }
fn tag(b: &[u8; 184]) -> u8 { b[0xb1] }
fn is_known_tag(t: u8) -> bool { matches!(t, 3 | 4 | 5 | 7 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) }
/// variant 별 live 범위(콜리 define initializes / store 범위 + 태그)
fn live_ranges(t: u8) -> Vec<(usize, usize)> {
    match t {
        3 => vec![(0, 56), (125, 126), (128, 132), (177, 178)],        // RunAway::new initializes((0,56),(125,126),(128,132))  m08.ll:92133
        5 => vec![(0, 56), (125, 126), (128, 130), (177, 178)],        // Around::new initializes((0,56),(125,126),(128,130))   m08.ll:92385 (+128 은 caller 가 10 으로 덮음)
        15 | 16 | 17 | 18 => vec![(0, 17), (177, 178)],                // Attack::new initializes((0,17))                        m07.ll:7129
        14 => vec![(0, 8), (85, 86), (88, 150), (177, 178)],
        t if !is_known_tag(t) => vec![(0, 104), (173, 174), (176, 178)], // AroundPosition::new m08.ll:103282~103304 store (0,48)+(48,88 wait_around 전부)+(88,104) + Option<PathFinder> None 니치 104+69 + (176,178)
        _ => vec![(0, 17), (177, 178)],
    }
}
fn cmp_live(a: &[u8; 184], b: &[u8; 184]) -> (bool, Vec<usize>) {
    let mut bad = vec![];
    for (lo, hi) in live_ranges(tag(a)) { for i in lo..hi { if a[i] != b[i] { bad.push(i); } } }
    (bad.is_empty(), bad)
}
fn cmp_all(a: &[u8; 184], b: &[u8; 184]) -> Vec<usize> { (0..184).filter(|&i| a[i] != b[i]).collect() }
fn mk_elem(payload: &[u8], t: u8) -> [u8; 184] {
    let mut b = [0u8; 184];
    b[..payload.len()].copy_from_slice(payload);
    b[0xb1] = t; b
}
fn bytes_of<T>(v: &T) -> Vec<u8> { unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()).to_vec() } }

struct Pred { res: Vec<[u8; 184]>, src: Vec<&'static str>, log: String, behind_incl: Option<(u64, u64)>, behind_excl: Option<(u64, u64)> }

/// behind(fm, d) 재구현. `incl`=true 면 L62 루프를 1..=move_index, false 면 1..move_index 로.
fn behind(data: &OperationData, fm: &Entity, d: u64, incl: bool, log: &mut String) -> (u64, u64) {
    let fp = ep(fm);
    let team_tag: i64 = rd(fp, 0); let team: usize = rd(fp, 8);
    let ety: i64 = rd(fp, 0x68);
    assert!(team_tag == 0, "behind: Neutral team"); assert!(ety == 1, "behind: not minion ty={}", ety);
    let line_tag: u8 = rd(fp, 0x11a);
    let line = match line_tag { 0 => LineType::Top, 1 => LineType::Mid, _ => LineType::Bottom };
    let move_index: usize = rd(fp, 0x108);
    let path = data.context.map.lane_path(line, team);
    let mut moved: u64 = 0;
    let upper = if incl { move_index + 1 } else { move_index };
    for i in 1..upper { moved += game_core::utils::distance(path[i - 1].0, path[i - 1].1, path[i].0, path[i].1); }
    moved += game_core::utils::distance(path[move_index].0, path[move_index].1, fm.x, fm.y);
    *log += &format!("behind[incl={}]:mi={} line={} moved_raw={} ", incl, move_index, line_tag, moved);
    moved = moved.saturating_sub(d);
    let (mut x, mut y) = path[0];
    for i in 1..7 {
        let seg = game_core::utils::distance(x, y, path[i].0, path[i].1);
        if seg > moved {
            let dx = path[i].0.wrapping_sub(x) as i64; let dy = path[i].1.wrapping_sub(y) as i64;
            let sz = game_core::utils::isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
            let nx = (x as i64).wrapping_add(dx.wrapping_mul(moved as i64) / sz);
            let ny = (y as i64).wrapping_add(dy.wrapping_mul(moved as i64) / sz);
            return Game::adjust_position(data.context.map, data.context.setting, nx, ny);
        }
        moved -= seg; x = path[i].0; y = path[i].1;
    }
    path[6]
}

/// ★명세 logic 의 독립 재구현. 콜리는 pub 함수를 그대로 쓴다(합성만 검증).
fn predict<'a>(line: LineType, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &'a OperationData<'a, 'a>, bump: &'a bumpalo::Bump) -> Pred {
    let mut P = Pred { res: vec![], src: vec![], log: String::new(), behind_incl: None, behind_excl: None };
    let team = player.info.team; let pos = player.info.position.as_index();
    let cache = data.cache;
    // ── base_positioning L17~53
    {
        let optb: Option<&Entity> = match line {
            LineType::Top => cache.top_tower[team].or(cache.top_tower2[team]),
            LineType::Mid => cache.mid_tower[team].or(cache.mid_tower2[team]),
            LineType::Bottom => cache.bottom_tower[team].or(cache.bottom_tower2[team]),
        };
        // L20~25: `.or(twin min_by_key)` 는 eager — get_start_position 이 twin 원소마다 호출됨(optb Some 이어도)
        let sp = line.get_start_position(data.context.setting, team);
        let twin = cache.twin_towers[team].iter().copied().min_by_key(|t| dist_sq_xy(t.x, t.y, sp.0, sp.1));
        let optb = optb.or(twin);
        let optb = optb.or(cache.nexus[team]);
        let nearest_tower = optb.expect("L27");
        let nexus = cache.nexus[team].expect("L29");
        let bb = &data.blackboard[team];
        let ms = match line { LineType::Top => &bb.top_minion_state, LineType::Mid => &bb.mid_minion_state, LineType::Bottom => &bb.bottom_minion_state };
        let fm = ms.front_minion.and_then(|id| cache.game.get_entity_by_id(id));
        P.log += &format!("bp:tower={} nexus={} sp=({},{}) twin={:?} fm={:?} ", nearest_tower.id, nexus.id, sp.0, sp.1, twin.map(|t| t.id), fm.map(|e| e.id));
        let around_tower = |P: &mut Pred, rnd: &mut rand::rngs::StdRng, why: &'static str| {
            let a = game_ai::SmallActionAround::new(version, rnd, data, player, nearest_tower.id, 5);
            let mut b = mk_elem(&bytes_of(&a), 5); b[0x80] = 10; // with_position_eval_purpose(LaneSafe) 인라인 store
            P.res.push(b); P.src.push(why);
        };
        match fm {
            None => { around_tower(&mut P, rnd, "Around(tower L35 fm None)"); }
            Some(fm) => {
                let d_fm_nex = dist_sq(fm, nexus); let d_tw_nex = dist_sq(nearest_tower, nexus); let d_fm_tw = dist_sq(fm, nearest_tower);
                P.log += &format!("bp:d(fm,nex)={} d(tw,nex)={} d(fm,tw)={} ", d_fm_nex, d_tw_nex, d_fm_tw);
                if d_fm_nex < d_tw_nex { around_tower(&mut P, rnd, "Around(tower L35 fm behind tower)"); }
                else if d_fm_tw < 32400000001 { around_tower(&mut P, rnd, "Around(tower L42 fm<=180000)"); }
                else {
                    let mut lg = String::new();
                    let bi = behind(data, fm, 180000, true, &mut lg);
                    let be = behind(data, fm, 180000, false, &mut lg);
                    P.log += &lg;
                    P.behind_incl = Some(bi); P.behind_excl = Some(be);
                    let (bx, by) = bi;
                    let a = game_ai::SmallActionAroundPosition::new(rnd, data, bx, by, 5);
                    let mut b = mk_elem(&bytes_of(&a), 0); b[0xb0] = 10; b[0xb1] = a_outline(&bytes_of(&a));
                    P.res.push(b); P.src.push("AroundPosition(behind L47)");
                }
            }
        }
    }
    { let r = game_ai::SmallActionRunAway::new(data, player, 5); P.res.push(mk_elem(&bytes_of(&r), 3)); P.src.push("RunAway(L148)"); }
    { let v = game_ai::battle_action(version, rnd, player, data, 5); for e in v.iter() { P.res.push(raw184(e)); P.src.push("battle_action"); } }
    { let v = game_ai::line_minion_action_candidates(version, data, player, line); for e in v.iter() { P.res.push(raw184(e)); P.src.push("line_minion_action_candidates"); } }
    { let v = game_ai::attack_summon_action(player, data); for e in v.iter() { P.res.push(raw184(e)); P.src.push("attack_summon_action"); } }
    // ── attack_tower_action L104~121
    'at: {
        let champ = match cache.player_champion[team][pos] { Some(c) => c, None => { P.log += "at:nochamp "; break 'at } };
        let tower = match cache.iter_towers(1 - team).filter(|t| t.can_target && t.block_target_tick == 0).min_by_key(|t| dist_sq(t, champ)) { Some(t) => t, None => { P.log += "at:notower "; break 'at } };
        let move_speed = champ.stat_cached.move_speed;
        let atk = match champ.attack_effect.as_ref() { Some(a) => a, None => { P.log += "at:noatk "; break 'at } };
        let d2 = dist_sq(tower, champ);
        let mut max = atk.range.wrapping_add(champ.stat_buff_cached.range as u64).wrapping_add(((champ.level as u64).wrapping_sub(1)).wrapping_mul(atk.growth_range as u64))
            .wrapping_add(atk.range_adjust(champ, tower) as u64).wrapping_add(champ.radius() as u64);
        max = max.wrapping_add((move_speed as u64).wrapping_mul(30)).wrapping_add(tower.radius() as u64);
        P.log += &format!("at:tower={} d2={} max={} max2={} ", tower.id, d2, max, max.wrapping_mul(max));
        if d2 > max.wrapping_mul(max) { P.log += "at:out_of_range "; break 'at; }
        let v22 = game_ai::v22_lane_tower_pressure_attack_allowed(version, data, player, tower);
        P.log += &format!("at:v22={} ", v22);
        if !v22 { break 'at; }
        let a = game_ai::SmallActionAttack::new(data, tower.id); P.res.push(mk_elem(&bytes_of(&a), 15)); P.src.push("Attack(tower L120)");
    }
    { let v = game_ai::attack_structure_skill_action(player, data); for e in v.iter() { P.res.push(raw184(e)); P.src.push("attack_structure_skill_action"); } }
    // ── L154 retain(!v30)
    {
        let mut keep_res = vec![]; let mut keep_src = vec![]; let mut removed = 0;
        for (b, s) in P.res.iter().zip(P.src.iter()) {
            let sap: &game_ai::SmallActionPlay = unsafe { &*(b as *const [u8; 184] as *const game_ai::SmallActionPlay) };
            if game_ai::v30_line_champion_action_tower_aggro_risk(version, data, player, sap) { removed += 1; continue; }
            keep_res.push(*b); keep_src.push(*s);
        }
        P.log += &format!("retain:removed={} ", removed);
        P.res = keep_res; P.src = keep_src;
    }
    P
}
fn a_outline(b: &[u8]) -> u8 { b[0xb1] }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::battle_action as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let sc = a.get("sc", 0);
    let mut setting = real_setting();
    minion_setting(&mut setting);
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let pool2 = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(9);
    let need_minions = a.get("minions", 0) == 1;
    if need_minions { for _ in 0..(a.get("ticks", 150) as usize) { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); } }
    let tick = a.get("tick", 1000) as usize;
    game.set_tick(tick);
    let version = a.get("version", 2) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let cachep = &cache as *const AbstractGameWithCache as *const u8;
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let team = a.get("team", 0) as usize;
    let posi = a.get("pos", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let champ = cache.player_champion[team][posi].expect("champ");
    let cp = ep(champ);
    let nexus0 = cache.nexus[team].expect("nexus");
    let li = a.get("line", 0);
    let line = match li { 0 => LineType::Top, 1 => LineType::Mid, _ => LineType::Bottom };
    let lane_tower: &Entity = match li { 0 => cache.top_tower[team].expect("top_tower"), 1 => cache.mid_tower[team].expect("mid_tower"), _ => cache.bottom_tower[team].expect("bottom_tower") };
    let twins0 = &cache.twin_towers[team];
    let enemy_twins = &cache.twin_towers[1 - team];
    println!("world\tteam={}\tpos={}\tline={}\tchamp.id={}\tchamp=({},{})\tnexus.id={}\tnexus=({},{})\tlane_tower.id={}\tlane_tower=({},{})\ttwins={:?}\tminions0={}\tminions1={}",
        team, posi, li, champ.id, champ.x, champ.y, nexus0.id, nexus0.x, nexus0.y, lane_tower.id, lane_tower.x, lane_tower.y,
        twins0.iter().map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>(), cache.iter_minions(team).count(), cache.iter_minions(1 - team).count());
    if a.get("catk", 1) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.get("cdmg", 100) as usize, a.get("crange", 30000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
    if a.get("chp", -1) >= 0 { wr(cp, 0x670, a.get("chp", 0) as usize); }
    // offset_of 교차검증(ev3 급)
    println!("offs\tEntity.x={:#x} y={:#x} id={:#x} level={:#x} radius={:#x} can_target={:#x} block_target_tick={:#x} attack_effect={:#x} stat_cached={:#x} stat_buff_cached={:#x} PlayerState.info={:#x} Blackboard.mid={:#x} bot={:#x} Cache.twin={:#x} nexus={:#x} top_tower={:#x} player_champion={:#x} GameContext.setting={:#x} map={:#x} OperationData.blackboard={:#x}",
        std::mem::offset_of!(Entity, x), std::mem::offset_of!(Entity, y), std::mem::offset_of!(Entity, id), std::mem::offset_of!(Entity, level), std::mem::offset_of!(Entity, radius),
        std::mem::offset_of!(Entity, can_target), std::mem::offset_of!(Entity, block_target_tick), std::mem::offset_of!(Entity, attack_effect), std::mem::offset_of!(Entity, stat_cached), std::mem::offset_of!(Entity, stat_buff_cached),
        std::mem::offset_of!(PlayerState, info), std::mem::offset_of!(Blackboard, mid_minion_state), std::mem::offset_of!(Blackboard, bottom_minion_state),
        std::mem::offset_of!(AbstractGameWithCache, twin_towers), std::mem::offset_of!(AbstractGameWithCache, nexus), std::mem::offset_of!(AbstractGameWithCache, top_tower), std::mem::offset_of!(AbstractGameWithCache, player_champion),
        std::mem::offset_of!(GameContext, setting), std::mem::offset_of!(GameContext, map), std::mem::offset_of!(OperationData, blackboard));
    let bp = &bb[team] as *const Blackboard as *const u8;
    let bboff = match li { 0 => 0usize, 1 => 0x28, _ => 0x50 };
    // 시나리오
    match sc {
        0 => { /* fm None */ }
        1 => { // fm = 아군 챔피언(pos fmi) 을 넥서스 바로 옆으로 → dist²(fm,nexus) < dist²(tower,nexus) → Around(tower)
            let fmi = a.get("fmi", 1) as usize; let e = cache.player_champion[team][fmi].expect("fm champ");
            wr(ep(e), 0x660, nexus0.x + a.get("dx", 1000) as u64); wr(ep(e), 0x668, nexus0.y);
            wr(bp, bboff, 1u64); wr(bp, bboff + 8, e.id);
        }
        2 => { // fm = 아군 챔피언을 lane_tower 에서 넥서스 반대쪽으로 (dx, dy) 만큼 → 경계 32400000001
            let fmi = a.get("fmi", 1) as usize; let e = cache.player_champion[team][fmi].expect("fm champ");
            let dx = a.get("dx", 180000) as i64; let dy = a.get("dy", 0) as i64;
            let sx: i64 = if lane_tower.x >= nexus0.x { 1 } else { -1 };
            let sy: i64 = if lane_tower.y >= nexus0.y { 1 } else { -1 };
            wr(ep(e), 0x660, (lane_tower.x as i64 + sx * dx) as u64); wr(ep(e), 0x668, (lane_tower.y as i64 + sy * dy) as u64);
            wr(ep(e), 0x68, 13i64); // 챔피언 그대로(behind 미진입 케이스)
            wr(bp, bboff, 1u64); wr(bp, bboff + 8, e.id);
        }
        3 => { // fm = 실제 아군 미니언(minions=1 필요) 을 lane_path[k] (+dx) 로 옮기고 move_index=mi → behind 경로
            let k = a.get("k", 3) as usize; let mi = a.get("mi", 2) as usize; let dx = a.get("dx", 0) as u64;
            let mn = cache.iter_minions(team).nth(a.get("mn", 0) as usize).expect("ally minion");
            let path = map.lane_path(line, team);
            wr(ep(mn), 0x11a, li as u8);
            wr(ep(mn), 0x660, path[k].0 + dx); wr(ep(mn), 0x668, path[k].1);
            wr(ep(mn), 0x108, mi);
            println!("minion\tid={}\tty={}\tteam_tag={}\tteam={}\tline={}\tmi={}\tpos=({},{})\tpath={:?}", mn.id, rd::<i64>(ep(mn), 0x68), rd::<i64>(ep(mn), 0), rd::<usize>(ep(mn), 8), rd::<u8>(ep(mn), 0x11a), rd::<usize>(ep(mn), 0x108), mn.x, mn.y, path);
            wr(bp, bboff, 1u64); wr(bp, bboff + 8, mn.id);
        }
        4 => { // lane tower/tower2 = None → twin_towers min_by_key(get_start_position) · twins=0 이면 nexus
            let toff = match li { 0 => 0x180usize, 1 => 0x1a0, _ => 0x1c0 };
            wr(cachep, toff + team * 8, 0usize); wr(cachep, toff + 0x10 + team * 8, 0usize);
            if a.get("twins", 1) == 0 { wr(cachep, 0x130 + team * 32 + 0x18, 0usize); }
            if a.get("twinswap", 0) == 1 { // 두 twin 좌표 교환 → 동률/선택 검증
                let t0 = twins0[0]; let t1 = twins0[1]; let (x0, y0, x1, y1) = (t0.x, t0.y, t1.x, t1.y);
                wr(ep(t0), 0x660, x1); wr(ep(t0), 0x668, y1); wr(ep(t1), 0x660, x0); wr(ep(t1), 0x668, y0);
            }
        }
        5 => { // champ 를 적 lane tower 옆으로 → attack_tower_action. dx=0 이면 정확히 max 거리(경계), dx=1 이면 +1
            let t: &Entity = match a.get("tk", 0) { 0 => cache.top_tower[1 - team].expect("etower"), 1 => enemy_twins[0], _ => cache.mid_tower[1 - team].expect("emid") };
            let atk = champ.attack_effect.as_ref().unwrap();
            let mut max = atk.range as u64 + champ.stat_buff_cached.range as u64 + (champ.level as u64 - 1) * atk.growth_range as u64 + atk.range_adjust(champ, t) as u64 + champ.radius() as u64;
            max += champ.stat_cached.move_speed as u64 * 30 + t.radius() as u64;
            let d = max as i64 + a.get("dx", 0);
            wr(cp, 0x660, (t.x as i64 + d) as u64); wr(cp, 0x668, t.y);
            println!("at_setup\ttower={}\tmax={}\td={}\tchamp=({},{})\ttower=({},{})", t.id, max, d, rd::<u64>(cp, 0x660), rd::<u64>(cp, 0x668), t.x, t.y);
            for mn in cache.iter_minions(1 - team) { let mp = ep(mn); wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); }
            if a.get("ally", 0) == 1 { for (i, mn) in cache.iter_minions(team).enumerate() { if i < 2 { wr(ep(mn), 0x660, t.x + 3000); wr(ep(mn), 0x668, t.y); } } }
        }
        _ => {}
    }
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
    let data = OperationData::new(&cache, &ctx, &bb);
    let seed = a.get("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd2 = rnd.clone();
    let mut sp = game_ai::plan_legacy::sub_plan::LineWaitSubPlan::new(line);
    let spp = &sp as *const _ as *const u8;
    let self_before: u8 = rd(spp, 0);
    let real = game_ai::plan_legacy::sub_plan::LineWaitSubPlan::action_candidates(&mut sp, version, &mut rnd, player, &data, param);
    let self_after: u8 = rd(spp, 0);
    let r1: u64 = rnd.gen();
    let real_raw: Vec<[u8; 184]> = real.iter().map(|e| raw184(e)).collect();
    let pred = predict(line, version, &mut rnd2, player, &data, &pool2);
    let r2: u64 = rnd2.gen();
    println!("self_before={}\tself_after={}\tsize={}", self_before, self_after, std::mem::size_of::<game_ai::plan_legacy::sub_plan::LineWaitSubPlan>());
    println!("log\t{}", pred.log);
    println!("real_n={}\tpred_n={}\trng_sync={}", real_raw.len(), pred.res.len(), r1 == r2);
    if let (Some(bi), Some(be)) = (pred.behind_incl, pred.behind_excl) {
        let r0 = &real_raw[0];
        let rx = u64::from_le_bytes(r0[8..16].try_into().unwrap()); let ry = u64::from_le_bytes(r0[16..24].try_into().unwrap());
        println!("behind\treal=({},{})\tincl=({},{})\texcl=({},{})\tverdict={}", rx, ry, bi.0, bi.1, be.0, be.1,
            if (rx, ry) == bi && (rx, ry) != be { "INCLUSIVE(1..=move_index)" } else if (rx, ry) == be && (rx, ry) != bi { "EXCLUSIVE(1..move_index)" } else if bi == be { "INDISTINGUISHABLE" } else { "NEITHER" });
    }
    let mut all_ok = real_raw.len() == pred.res.len();
    for i in 0..std::cmp::max(real_raw.len(), pred.res.len()) {
        match (real_raw.get(i), pred.res.get(i)) {
            (Some(r), Some(p)) => {
                let (ok, bad) = cmp_live(r, p);
                let full = cmp_all(r, p);
                let key = format!("f0={} f8={} f16={} f128={} f176={}", u64::from_le_bytes(r[0..8].try_into().unwrap()), u64::from_le_bytes(r[8..16].try_into().unwrap()), u64::from_le_bytes(r[16..24].try_into().unwrap()), r[128], r[176]);
                println!("elem[{}]\treal_tag={}\tpred_tag={}\tsrc={}\tlive={}\tlive_bad={:?}\tall_diff_n={}\tall_diff_first={:?}\t{}", i, tag(r), tag(p), pred.src[i], if ok { "MATCH" } else { "MISMATCH" }, bad, full.len(), full.iter().take(16).collect::<Vec<_>>(), key);
                if !ok || tag(r) != tag(p) { all_ok = false; }
            }
            (Some(r), None) => { println!("elem[{}]\treal_tag={}\tpred=NONE", i, tag(r)); all_ok = false; }
            (None, Some(p)) => { println!("elem[{}]\treal=NONE\tpred_tag={}\tsrc={}", i, tag(p), pred.src[i]); all_ok = false; }
            _ => {}
        }
    }
    println!("RESULT\tsc={}\telems={}\tself={}\trng_sync={}\t{}", sc, if all_ok { "MATCH" } else { "MISMATCH" }, if self_before == self_after { "MATCH" } else { "MISMATCH" }, r1 == r2, if all_ok && self_before == self_after && r1 == r2 { "MATCH" } else { "MISMATCH" });
}
