#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치D 오라클 — 257 v30_wave_danger_chase_guard(define hidden m10.ll:52377 → `#[link_name]` 직접 진입).
//!  명세 `logic` 의 독립 재구현(predict) ↔ 실행 대조. 세계 = TEMPLATE mkgame(real_setting)+minion_setting, `ticks=N` 이면 run_tick N(600 이면 미니언이 선다).
//!  출력 3개(&mut u8/&mut u8/&mut bool)는 센티널 0xAA 로 채워 두고 호출 뒤 값을 찍는다 → 「조기 None 3경로 미기록」 검증.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify27/D/oracle/o27D_v30.rs → %TEMP%\tfm2_spanprobe\o27D_v30.exe
//!  실행: o27D_v30.exe k=v ...  (드라이버 = run27D.py · 케이스당 프로세스 1개)
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle27v30_wave_danger_chase_guard"]
    fn v30(version: usize, data: &OperationData, champ: &Entity, focused: &Entity, max_range: u64,
           focused_die_tick: usize, focused_is_in_range: bool, my_die_tick: usize, target_hp_ratio: usize,
           can_runaway: bool, return_to_objective: bool, open_eval: bool,
           wave_obs: &mut u8, wave_pct_out: &mut u8, wave_danger_out: &mut bool)
           -> Option<game_ai::plan_legacy::old::BattleSubPlanGoal>;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

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

/// 명세 logic 독립 재구현. 반환 = (tag, payload, obs, pct, danger, log) — 미기록은 None.
fn predict(version: usize, data: &OperationData, champ: &Entity, focused: &Entity, max_range: u64,
           fdt: usize, inr: bool, mdt: usize, thr: usize, cr: bool, rto: bool, oe: bool) -> (i64, Option<u64>, Option<u8>, Option<u8>, Option<bool>, String) {
    let mut log = String::new();
    let cp = ep(champ); let fp = ep(focused);
    let ct: i64 = rd(cp, 0x0); let ft: i64 = rd(fp, 0x0);
    let c0: i64 = rd(cp, 0x8); let f0: i64 = rd(fp, 0x8);
    // L2083 TeamType::eq — 태그 같고 (Neutral 둘다 | Player 번호 같음)
    let same_team = ct == ft && (ct == 1 || c0 == f0);
    if same_team { log += "same_team "; return (-1, None, None, None, None, log); }
    // battle_chase_stance 인라인
    if max_range == 0 { log += "max_range0 "; return (-1, None, None, None, None, log); }
    let ctx = data.context; let setting = ctx.setting; let map = ctx.map;
    let dist = champ.distance(focused);
    let cx: i64 = rd(cp, 0x660); let cy: i64 = rd(cp, 0x668);
    let fx: i64 = rd(fp, 0x660); let fy: i64 = rd(fp, 0x668);
    let (mut sx, mut sy, mut walk_tick): (i64, i64, u64) = (cx, cy, 0);
    if dist > max_range {
        // is_visible_from: champ Neutral 이면 통과, Player(t) 면 focused.visible_state[t] 태그 0
        let vis = if ct == 1 { true } else { let t = c0 as usize; assert!(t < 2); let tag: i64 = rd(fp, 0x38 + 24 * t); tag == 0 };
        if !vis { log += "invisible "; return (-1, None, None, None, None, log); }
        let dx = cx - fx; let dy = cy - fy;
        let sz = game_core::utils::isqrt(dx * dx + dy * dy);
        if sz < 1 { /* stance = 내 위치 */ }
        else {
            let fd = max_range.saturating_sub(15000) as i64;
            let (x, y) = Game::adjust_position(map, setting, fx + dx * fd / sz, fy + dy * fd / sz);
            let ms: u64 = rd(cp, 0x640); let ms = ms.max(1);
            let walk = dist.saturating_sub(max_range);
            sx = x as i64; sy = y as i64; walk_tick = walk / ms;
        }
    }
    let tps = setting.tick_per_second as u64;
    let window = (tps * 3).min(tps.max(walk_tick + tps * 2)) as usize;
    log += &format!("stance=({},{}) walk={} window={} ", sx, sy, walk_tick, window);
    let mut wave_damage = if version > 1 { game_ai::enemy_minion_line_action_damage_at(version, data, champ, sx as u64, sy as u64, window, true, true) }
                          else { game_ai::enemy_minion_wave_risk_damage_at(version, data, champ, sx as u64, sy as u64, window) };
    let mut projected: Option<(i64, i64)> = None;
    if version > 1 && oe && ft == 0 {
        let et = f0 as usize; assert!(et < 2);
        if let Some(nexus) = data.cache.nexus[et] {
            let np = ep(nexus);
            let nx: i64 = rd(np, 0x660); let ny: i64 = rd(np, 0x668);
            let dx = nx - fx; let dy = ny - fy;
            let sz = game_core::utils::isqrt(dx * dx + dy * dy).max(1);
            let fms: i64 = rd(fp, 0x640);
            let flee = fms * tps as i64;
            let (px, py) = Game::adjust_position(map, setting, fx + flee * dx / sz, fy + flee * dy / sz);
            let (px, py) = (px as i64, py as i64);
            let ddx = cx - px; let ddy = cy - py;
            let dsz = game_core::utils::isqrt(ddx * ddx + ddy * ddy).max(1);
            let fd = max_range.saturating_sub(15000) as i64;
            let (sx2, sy2) = Game::adjust_position(map, setting, px + ddx * fd / dsz, py + ddy * fd / dsz);
            projected = Some((sx2 as i64, sy2 as i64));
            let wd2 = game_ai::enemy_minion_line_action_damage_at(version, data, champ, sx2, sy2, window, true, true);
            wave_damage = wave_damage.max(wd2);
            log += &format!("proj=({},{}) wd2={} ", sx2, sy2, wd2);
        }
    }
    let hp: u64 = rd(cp, 0x670); let mhp: u64 = rd(cp, 0x628);
    let wave_pct = (wave_damage as u64 * 100) / hp.max(1);
    let pct_out = wave_pct.min(254) as u8;
    log += &format!("wave_damage={} wave_pct={} hp={} mhp={} ", wave_damage, wave_pct, hp, mhp);
    if wave_damage == 0 { return (-1, None, Some(1), Some(pct_out), None, log); }
    let hp_pct = hp * 100 / mhp.max(1);
    let mut danger = if version > 1 { game_ai::enemy_minion_line_action_danger_damage_at(version, data, champ, sx as u64, sy as u64, window, true, true) }
                     else { game_ai::enemy_minion_wave_danger_damage_at(version, data, champ, sx as u64, sy as u64, window) };
    if let Some((sx2, sy2)) = projected {
        danger = danger.max(game_ai::enemy_minion_line_action_danger_damage_at(version, data, champ, sx2 as u64, sy2 as u64, window, true, true));
    }
    let danger_out = Some(danger != 0);
    log += &format!("danger={} hp_pct={} ", danger, hp_pct);
    if danger == 0 && !(wave_pct > 11 && hp_pct < 51) && !((wave_pct > 4 && hp_pct < 36) || hp_pct < 26) {
        return (-1, None, Some(2), Some(pct_out), danger_out, log);
    }
    let needs_to_move = if inr && !(version > 1 && oe) { walk_tick >= tps / 3 } else { true };
    let ikw = if fdt as u64 > tps / 2 { inr && thr < 26 && fdt <= mdt.saturating_add((tps / 2) as usize) } else { true };
    let wd = wave_damage as u64;
    if ikw && wd < hp && !(wave_pct > 14 && hp_pct < 26) { return (-1, None, Some(3), Some(pct_out), danger_out, log); }
    if version > 1 && wd < hp {
        let wave_die_tick = hp.saturating_mul(window as u64) / wd;
        let fms: u64 = rd(fp, 0x640); let cms: u64 = rd(cp, 0x640);
        let kill_reachable = inr || cms > fms;
        if kill_reachable && (fdt as u64).saturating_add(walk_tick) < wave_die_tick.min(mdt as u64) {
            return (-1, None, Some(3), Some(pct_out), danger_out, log);
        }
    }
    if !(hp_pct < 46 || needs_to_move || wd >= hp) { return (-1, None, Some(4), Some(pct_out), danger_out, log); }
    let obs = Some(5u8);
    if cr && (wd >= hp || hp_pct < 46) { return (4, None, obs, Some(pct_out), danger_out, log); }
    if inr { let id: u64 = rd(fp, 0x5c0); return (3, Some(id), obs, Some(pct_out), danger_out, log); }
    if cr { return (if rto { 7 } else { 4 }, None, obs, Some(pct_out), danger_out, log); }
    (7, None, obs, Some(pct_out), danger_out, log)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::battle_action as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let mut setting = real_setting();
    minion_setting(&mut setting);
    if a.has("matk") { setting.melee_minion.stat.attack = a.get("matk", 10) as usize; setting.range_minion.stat.attack = a.get("ratk", a.get("matk", 15)) as usize; }
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(9);
    let ticks = a.get("ticks", 600) as usize;
    for _ in 0..ticks { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    let version = a.get("version", 2) as usize;
    let team = a.get("team", 0) as usize; let et = 1 - team;
    let posi = a.get("pos", 2) as usize;
    let fk = a.get("fk", 2) as usize;
    {
        let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let champ = cache0.player_champion[team][posi].expect("champ");
        let cp = ep(champ);
        // 챔프를 적 미니언 k 번째 옆(+dx,+dy) 으로
        if a.has("nearm") {
            let k = a.get("nearm", 0) as usize;
            let mut ms_: Vec<&Entity> = cache0.iter_minions(et).collect();
            ms_.sort_by_key(|e| e.id);
            if let Some(mn) = ms_.get(k) {
                wr(cp, 0x660, mn.x.wrapping_add(a.get("mdx", 0) as u64)); wr(cp, 0x668, mn.y.wrapping_add(a.get("mdy", 0) as u64));
                println!("nearm: minion{} id={} xy=({},{}) count={}", k, mn.id, mn.x, mn.y, ms_.len());
            } else { println!("WARN no enemy minion k={} (count={})", k, ms_.len()); }
        }
        if a.has("cx") { wr(cp, 0x660, a.get("cx", 0) as u64); wr(cp, 0x668, a.get("cy", 0) as u64); }
        if a.has("chp") { wr(cp, 0x670, a.get("chp", 0) as i64); }
        if a.has("cmhp") { wr(cp, 0x628, a.get("cmhp", 1) as i64); }
        if a.has("cms") { wr(cp, 0x640, a.get("cms", 1) as u64); }
        let f = cache0.player_champion[et][fk].expect("focused");
        let fp = ep(f);
        // focused 를 챔프 기준 (fdx, fdy) 로
        if a.has("fdx") { let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668); wr(fp, 0x660, cx.wrapping_add(a.get("fdx", 0) as u64)); wr(fp, 0x668, cy.wrapping_add(a.get("fdy", 0) as u64)); }
        if a.has("fms") { wr(fp, 0x640, a.get("fms", 1) as u64); }
        if !a.has("invis") { wr(fp, 0x38 + team * 24, 0i64); }   // visible_state[my_team] = Visible(0)
        else { wr(fp, 0x38 + team * 24, a.get("invis", 2) as i64); }
        if a.has("ftm") { // 같은팀 테스트: focused.team = champ.team (Player(team))
            wr(fp, 0x0, 0i64); wr(fp, 0x8, team as i64);
        }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[team][posi].expect("champ");
    let focused = cache.player_champion[et][fk].expect("focused");
    let max_range = a.get("mr", 30000) as u64;
    let fdt = a.get("fdt", 100) as usize; let inr = a.get("inr", 1) != 0; let mdt = a.get("mdt", 100) as usize;
    let thr = a.get("thr", 50) as usize; let cr = a.get("cr", 1) != 0; let rto = a.get("rto", 0) != 0; let oe = a.get("oe", 0) != 0;
    println!("world tick={} champ.id={} xy=({},{}) hp={}/{} ms={} | focused.id={} xy=({},{}) ms={} vis_tag={} | dist={} minions_et={} nexus_et={}",
        game.tick(), champ.id, champ.x, champ.y, champ.hp, champ.stat_cached.hp, champ.stat_cached.move_speed,
        focused.id, focused.x, focused.y, focused.stat_cached.move_speed, rd::<i64>(ep(focused), 0x38 + team * 24),
        champ.distance(focused), cache.iter_minions(et).count(), cache.nexus[et].is_some());
    // 오프셋 교차검증(pub 필드 ↔ raw)
    let cp = ep(champ);
    let off_ok = rd::<u64>(cp, 0x660) == champ.x && rd::<u64>(cp, 0x668) == champ.y && rd::<i64>(cp, 0x670) == champ.hp as i64
        && rd::<i64>(cp, 0x628) == champ.stat_cached.hp as i64 && rd::<u64>(cp, 0x640) == champ.stat_cached.move_speed as u64 && rd::<u64>(cp, 0x5c0) == champ.id as u64;
    println!("offsets_ok\t{}\tis_visible_from={}", off_ok, focused.is_visible_from(champ));

    let mut obs: u8 = 0xAA; let mut pct: u8 = 0xAA; let mut dang_raw: u8 = 0xAA;
    let ret = unsafe { v30(version, &data, champ, focused, max_range, fdt, inr, mdt, thr, cr, rto, oe, &mut obs, &mut pct, &mut *( &mut dang_raw as *mut u8 as *mut bool)) };
    let (tag, pay): (i64, u64) = unsafe { std::mem::transmute_copy(&ret) };
    std::mem::forget(ret);
    let (ptag, ppay, pobs, ppct, pdang, log) = predict(version, &data, champ, focused, max_range, fdt, inr, mdt, thr, cr, rto, oe);
    let pobs_b = pobs.unwrap_or(0xAA); let ppct_b = ppct.unwrap_or(0xAA); let pdang_b = pdang.map(|b| b as u8).unwrap_or(0xAA);
    let pay_ok = ptag != 3 || ppay == Some(pay);
    let ok = tag == ptag && pay_ok && obs == pobs_b && pct == ppct_b && dang_raw == pdang_b;
    println!("got\ttag={}\tpay={}\tobs={:#x}\tpct={:#x}\tdanger={:#x}", tag, pay, obs, pct, dang_raw);
    println!("pred\ttag={}\tpay={:?}\tobs={:#x}\tpct={:#x}\tdanger={:#x}\t{}", ptag, ppay, pobs_b, ppct_b, pdang_b, log);
    println!("{}", if ok { "MATCH" } else { "DIFF" });
}
