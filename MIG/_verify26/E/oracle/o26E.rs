#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치E 오라클 — 209 build_minion_wave_snapshot(pub) · 210 StealSubPlan::action_candidates(pub) · 211 check_kill_die_tick_uncached(internal → pub 래퍼 game_ai::check_kill_die_tick 경유 · 케이스당 프로세스 1개 = TLS DieTickCache 비어 있음).
//!  명세 `logic` 의 독립 재구현(predict) ↔ 실행 대조. 세계 = TEMPLATE mkgame(real_setting)+minion_setting, `ticks=N` 이면 run_tick N.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/E/oracle/o26E.rs → %TEMP%\tfm2_spanprobe\o26E.exe
//!  실행: o26E.exe fn=209|210|211 k=v ...   (드라이버 = run26E.py)
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
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
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10, target, attack_type: AttackType::BaseAttack, casting }
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
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

fn dist_sq_xy(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by);
    dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx))
}
fn bytes_of<T>(v: &T) -> Vec<u8> { unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()).to_vec() } }
fn team_eq(a: *const u8, b: *const u8) -> bool {
    // TeamType derive PartialEq: tag(+0) 비교 → tag==0(Player) 이면 payload(+8) 비교
    let ta: i64 = rd(a, 0); let tb: i64 = rd(b, 0);
    if ta != tb { return false; }
    if ta == 0 { let pa: usize = rd(a, 8); let pb: usize = rd(b, 8); return pa == pb; }
    true
}
fn ent_radius(e: &Entity) -> u64 { // entity.rs:1511~1515 인라인 재구현
    let p = ep(e); let mult: i32 = rd(p, 0x470); let r: u64 = rd(p, 0x680);
    if mult == 0 { r } else { r.wrapping_mul((mult as i64 + 100) as u64) / 100 }
}

// ═══════════════════════════════ 209 ═══════════════════════════════
fn predict209(player: &PlayerState, data: &OperationData, depth: usize, sq: usize, log: &mut String) -> [u8; 2320] {
    let mut snap = [0u8; 2320];
    let sp = snap.as_mut_ptr() as *const u8;
    let cache = data.cache;
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = cache.player_champion[team][pos].expect("L617 champ");
    let cp = ep(champ);
    // L618 default: 12 슬롯 expected_death_tick = MAX
    for s in 0..12 { wr(sp, s * 192 + 0xb8, usize::MAX); }
    wr(sp, 0x908, cache.game.tick());                                              // L619
    let range_sq: u64 = 6400000000;                                                // L622
    let mut targets: Vec<&Entity> = Vec::new();
    let mut target_ids = [usize::MAX; 12];                                          // L628
    let mut count = 0usize;
    let mut it = cache.game.iter_entity();
    loop {                                                                          // L629~647
        let e = match it.next() { Some(e) => e, None => break };
        if count > 11 { break; }
        let p = ep(e);
        let ty: i64 = rd(p, 0x68);
        if ty != 1 { continue; }
        if team_eq(p, cp) { continue; }
        let ct: u8 = rd(p, 0x6b9); if ct == 0 { continue; }
        let d2 = dist_sq_xy(champ.x, champ.y, e.x, e.y);
        if d2 > range_sq { continue; }
        let base = count * 192;
        wr(sp, base + 0x90, e.id); wr(sp, base + 0x98, e.hp); wr(sp, base + 0xa0, e.stat_cached.hp);
        target_ids[count] = e.id; targets.push(e); count += 1;
    }
    wr(sp, 0x900, count);
    if count == 0 { *log += "count0 "; return snap; }
    let find_slot = |id: usize| target_ids[..count].iter().position(|&t| t == id);
    let mut dps = [0i64; 12];
    let mut one: [[(usize, usize); 16]; 12] = [[(0, 0); 16]; 12];
    let mut onec = [0usize; 12];
    for src in cache.game.iter_entity() {                                            // L665~687
        let p = ep(src);
        let ty: i64 = rd(p, 0x68);
        let same = team_eq(p, cp);
        let tid: Option<usize> = match ty {
            1 | 7 | 9 => { if !same { continue; } let t: i64 = rd(p, 0x88); if t & 1 == 1 { Some(rd(p, 0x90)) } else { None } }
            2 => { if sq == 0 || !same { continue; } let t: i64 = rd(p, 0x88); if t & 1 == 1 { Some(rd(p, 0x98)) } else { None } }
            10 => { if !same { continue; } let t: i64 = rd(p, 0x70); if t & 1 == 1 { Some(rd(p, 0x78)) } else { None } }
            8 => { if !same { continue; } Some(rd(p, 0x100)) }
            _ => continue,
        };
        let Some(tid) = tid else { continue };
        let Some(slot) = find_slot(tid) else { continue };
        let Some(atk) = src.attack_effect.as_ref() else { continue };
        let dmg = atk.expected_damage_target(data.context, src as &dyn AbstractEntity, targets[slot]) as i64;
        let ct = src.attack_cooltime().max(1) as i64;
        dps[slot] += dmg / ct;
        *log += &format!("src(ty{} id{}→slot{} dmg{} ct{}) ", ty, src.id, slot, dmg, ct);
    }
    if sq > 1 {                                                                      // L688~721
        for pj in cache.game.iter_projectile() {
            let pp = pj as *const Projectile as *const u8;
            let mt: i64 = rd(pp, 0x40);
            let caster_id: usize = rd(pp, 0xf8);
            if mt == 6 {
                let speed: u64 = rd(pp, 0x48); let tgt: usize = rd(pp, 0x50);
                let Some(slot) = find_slot(tgt) else { continue };
                let e = targets[slot];
                let Some(caster) = cache.game.get_entity_by_id(caster_id) else { continue };
                let px: u64 = rd(pp, 0x100); let py: u64 = rd(pp, 0x108);
                let dist = game_core::utils::distance(px, py, e.x, e.y);
                let arrival = dist / speed.max(1) + 5;
                let dmg = pj.expected_damage_target(data.context, caster, e);
                *log += &format!("pjT(c{}→slot{} arr{} dmg{}) ", caster_id, slot, arrival, dmg);
                if arrival as usize <= depth && onec[slot] < 16 { one[slot][onec[slot]] = (arrival as usize, dmg); onec[slot] += 1; }
            } else if sq != 2 {
                let Some(caster) = cache.game.get_entity_by_id(caster_id) else { continue };
                if !team_eq(ep(caster), cp) { continue; }
                let at: &CastingTarget = unsafe { &*(pp.add(0x12c) as *const CastingTarget) };
                for slot in 0..count {
                    let e = targets[slot];
                    if at.check_projectile(pj, e) && pj.is_in_orbit(e.x, e.y, ent_radius(e)) {
                        let dmg = pj.expected_damage_target(data.context, caster, e);
                        *log += &format!("pjA(c{}→slot{} dmg{}) ", caster_id, slot, dmg);
                        if onec[slot] < 16 { one[slot][onec[slot]] = (1, dmg); onec[slot] += 1; }
                    }
                }
            }
        }
    }
    for slot in 0..count {                                                           // L724~761
        let base = slot * 192;
        wr(sp, base + 0xa8, dps[slot]);
        let ncp = (depth / 5).min(18);
        let cur: i64 = rd(sp, base + 0x98);
        let mut hp = cur;
        let mut death = usize::MAX;
        let mut prev_cp: i64 = 0;
        for i in 0..ncp {
            let tick_off = (i + 1) * 5;
            hp = hp.wrapping_sub(dps[slot].wrapping_mul(5));
            let prev_tick = i * 5;
            for j in 0..onec[slot] {
                let (arr, dmg) = one[slot][j];
                if arr > prev_tick && arr <= tick_off { hp = hp.wrapping_sub(dmg as i64); }
            }
            wr(sp, base + i * 8, hp);
            wr(sp, base + 0xb0, i + 1);
            if hp < 1 && death == usize::MAX {
                let prev_hp = if i == 0 { cur } else { rd::<i64>(sp, base + (i - 1) * 8) };
                if prev_hp > 0 { let frac = prev_hp.wrapping_mul(5) / (prev_hp - hp); death = prev_tick + frac as usize; }
                else { death = tick_off; }
            }
        }
        wr(sp, base + 0xb8, death);
    }
    snap
}

fn dump209(b: &[u8; 2320]) -> String {
    let p = b.as_ptr();
    let count: usize = rd(p, 0x900); let ct: usize = rd(p, 0x908);
    let mut s = format!("count={} tick={} ", count, ct);
    for sl in 0..count.min(12) {
        let base = sl * 192;
        let id: usize = rd(p, base + 0x90); let hp: i64 = rd(p, base + 0x98); let thp: i64 = rd(p, base + 0xa0);
        let dps: i64 = rd(p, base + 0xa8); let cc: usize = rd(p, base + 0xb0); let dt: usize = rd(p, base + 0xb8);
        let cps: Vec<i64> = (0..cc.min(18)).map(|i| rd::<i64>(p, base + i * 8)).collect();
        s += &format!("| slot{} id={} hp={} max={} dps={} ncp={} death={} cps={:?} ", sl, id, hp, thp, dps, cc, if dt == usize::MAX { -1 } else { dt as i64 }, cps);
    }
    s
}

// ═══════════════════════════════ 210 ═══════════════════════════════
fn raw184(p: &game_ai::SmallActionPlay) -> [u8; 184] { unsafe { std::ptr::read(p as *const _ as *const [u8; 184]) } }
fn tag(b: &[u8; 184]) -> u8 { b[0xb1] }
fn is_known_tag(t: u8) -> bool { matches!(t, 3 | 4 | 5 | 6 | 7 | 8 | 9 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19) }
fn live_ranges(t: u8) -> Vec<(usize, usize)> {
    match t {
        3 => vec![(0, 56), (125, 126), (128, 132), (177, 178)],        // RunAway::new_with_skill initializes((0,56),(125,126),(128,132)) m08.ll:92086
        4 => vec![(69, 70), (72, 129), (177, 178)],                     // Recall::new initializes((69,70),(72,129)) m08.ll:98371
        12 => vec![(0, 40), (109, 110), (112, 113), (177, 178)],        // AroundBush::new_with_target store 전수 m08.ll:104917 (+109 Option<PathFinder> None 니치 2 · +112 out_line)
        15 | 16 | 17 | 18 => vec![(0, 17), (177, 178)],                // Attack/Skill/Skill2/Ult ::new initializes((0,17))
        t if !is_known_tag(t) => vec![(0, 104), (173, 174), (176, 178)], // AroundPosition::new / new_with_out_line store 전수 m08.ll:103238/103172
        _ => vec![(0, 17), (177, 178)],
    }
}
fn cmp_live(a: &[u8; 184], b: &[u8; 184]) -> Vec<usize> {
    let mut bad = vec![];
    for (lo, hi) in live_ranges(tag(a)) { for i in lo..hi { if a[i] != b[i] { bad.push(i); } } }
    bad
}
fn mk_elem(payload: &[u8], t: u8) -> [u8; 184] { let mut b = [0u8; 184]; b[..payload.len()].copy_from_slice(payload); b[0xb1] = t; b }
fn tagname(t: u8) -> &'static str {
    match t { 3 => "RunAway", 4 => "Recall", 5 => "Around", 12 => "AroundBush", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop",
        0 | 1 | 2 => "AroundPosition(untagged)", _ => "?" }
}
/// steal.rs:27/28/30 헬퍼 재구현: get_game_mode → Moba → jungle_runner.{epic,serpen}.live_list.first → get_entity_by_id
fn steal_target<'a>(data: &'a OperationData, target: Option<StealTarget>, log: &mut String) -> Option<&'a Entity> {
    let t = target?;
    let gm = data.cache.game.get_game_mode();
    let (disc, mp): (i64, *const u8) = unsafe { std::mem::transmute_copy(&gm) };
    std::mem::forget(gm);
    if disc != 0 { panic!("get_game_mode not Moba (unwrap panic path)"); }
    let off = match t { StealTarget::Epic => 0x198usize, StealTarget::Serpen => 0x1c8 };
    let len: usize = rd(mp, off + 0x10);
    *log += &format!("live_list[{:?}].len={} ", t, len);
    if len == 0 { return None; }
    let ptr: *const usize = rd(mp, off + 0x8);
    let id = unsafe { *ptr };
    data.cache.game.get_entity_by_id(id)
}
fn predict210(target: Option<StealTarget>, commit: bool, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, log: &mut String) -> Vec<[u8; 184]> {
    let mut res: Vec<[u8; 184]> = vec![];
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = match data.cache.player_champion[team][pos] {
        Some(c) => c,
        None => { let r = game_ai::SmallActionRecall::new(data, player, 5); res.push(mk_elem(&bytes_of(&r), 4)); return res; }
    };
    let cp = ep(champ);
    let tgt = steal_target(data, target, log);
    let Some(tgt) = tgt else {
        let jt = if target == Some(StealTarget::Serpen) { JungleType::Serpen } else { JungleType::Morgard };
        let (x, y) = data.context.map.camp_pos(jt, team == 0);
        *log += &format!("fallback camp {:?} ({},{}) ", jt, x, y);
        let ap = game_ai::SmallActionAroundPosition::new(rnd, data, x, y, 5);
        let mut b = [0u8; 184]; b.copy_from_slice(&bytes_of(&ap)); res.push(b); return res;
    };
    let tp = ep(tgt);
    if commit {
        let ms = champ.stat_cached.move_speed as u64;
        let level: u64 = rd(cp, 0x5c8);
        let radii = ent_radius(champ).wrapping_add(ent_radius(tgt));
        let d2 = dist_sq_xy(tgt.x, tgt.y, champ.x, champ.y);
        let gate = |eff: &Effect, name: &str, log: &mut String| -> bool {
            let range = eff.range.wrapping_add(champ.stat_buff_cached.range as u64).wrapping_add(eff.growth_range.wrapping_mul(level.wrapping_sub(1)));
            let mut mx = range.wrapping_add(eff.range_adjust(champ, tgt)).wrapping_add(radii);
            mx = mx.wrapping_add(ms.wrapping_mul(30));
            let ok = d2 <= mx.wrapping_mul(mx);
            *log += &format!("{}:range={} max={} d2={} ok={} ", name, range, mx, d2, ok);
            ok
        };
        if champ.can_attack() { if let Some(atk) = champ.attack_effect.as_ref() {
            if gate(atk, "atk", log) { let a = game_ai::SmallActionAttack::new(data, tgt.id); res.push(mk_elem(&bytes_of(&a), 15)); }
        } }
        if let Some(sk) = champ.skill_effect.as_ref() { if champ.can_skill() && sk.target.check(champ, tgt) {
            if gate(sk, "sk", log) { let a = game_ai::SmallActionSkill::new(data, tgt.id); res.push(mk_elem(&bytes_of(&a), 16)); }
        } }
        if let Some(sk) = champ.skill2_effect().as_ref() { if champ.can_skill2() && sk.target.check(champ, tgt) {
            if gate(sk, "sk2", log) { let a = game_ai::SmallActionSkill2::new(data, tgt.id); res.push(mk_elem(&bytes_of(&a), 17)); }
        } }
        if let Some(u) = champ.ult_effect().as_ref() { if champ.can_ult() && u.target.check(champ, tgt) {
            if gate(u, "ult", log) { let a = game_ai::SmallActionUlt::new(data, tgt.id); res.push(mk_elem(&bytes_of(&a), 18)); }
        } }
        let tps = data.context.setting.tick_per_second;
        let (x, y) = game_ai::plan_legacy::steal::steal_damage_entry_pos(data.context, champ, tgt, tps * 5);
        *log += &format!("entry=({},{}) ", x, y);
        let ap = game_ai::SmallActionAroundPosition::new_with_out_line(rnd, data, x, y, 5, game_ai::AroundBushOutlineType::Outline);
        let mut b = [0u8; 184]; b.copy_from_slice(&bytes_of(&ap)); res.push(b);
    } else {
        let et = 1 - team;
        let mut near = false;
        for e in data.cache.player_champion[et].iter().flatten() {
            let vis = e.is_visible_from(champ);
            let d2 = dist_sq_xy(e.x, e.y, champ.x, champ.y);
            *log += &format!("enemy{} vis={} d2={} ", e.id, vis, d2);
            if vis && d2 < 16900000001 { near = true; break; }
        }
        if near {
            let r = game_ai::SmallActionRunAway::new_with_skill(data, player, 5, true); res.push(mk_elem(&bytes_of(&r), 3));
        } else if let Some(t) = target {
            let bush = game_ai::plan_legacy::steal::steal_wait_bush(t, team, champ, data.context.map);
            let tgt2 = steal_target(data, target, log).expect("L113 unwrap");
            *log += &format!("bush={} ", bush);
            let ab = game_ai::SmallActionAroundBush::new_with_target(data, tgt2, bush, game_ai::AroundBushOutlineType::Outline);
            res.push(mk_elem(&bytes_of(&ab), 12));
        }
    }
    res
}

// ═══════════════════════════════ 211 ═══════════════════════════════
struct Noise { st: u64, lo: u64, w: u64, n: usize }
impl Noise {
    fn next(&mut self) -> u64 {
        self.st = self.st.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.st;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        self.n += 1;
        (((z as u128) * (self.w as u128)) >> 64) as u64 + self.lo
    }
}
fn predict211(version: usize, data: &OperationData, judger: &PlayerState, focus: &Entity, enemy: &[&Entity], towers: &[&Entity], log: &mut String) -> usize {
    let fp = ep(focus);
    let und: u8 = rd(fp, 0x488);
    if und != 0 { return i64::MAX as usize; }
    let cache = data.cache; let ctx = data.context; let setting = ctx.setting;
    let player = cache.player_by_champion_id(focus.id).expect("L979");
    let tps_raw = setting.tick_per_second; let tps = tps_raw.max(1);
    let bucket = cache.game.tick() / (tps * 2);
    let jp = judger as *const PlayerState as *const u8;
    let jid: u64 = rd(jp, 0x928);
    let seed = (jid.wrapping_mul(0x9E3779B97F4A7C15) ^ ((focus.id as u64) << 24)) ^ bucket as u64;
    let ja = unsafe { (&*(jp.add(0x180) as *const AthleteParameter)).judge_accuracy() } as u64;
    let d = (1000 - ja) >> 1;
    let mut rng = Noise { st: seed, lo: 1000 - d, w: (1000 - ja) | 1, n: 0 };
    *log += &format!("ja={} d={} bucket={} seed={:#x} ", ja, d, bucket, seed);
    let mut edps: u64 = 0; let mut enuke: u64 = 0;
    let fpos = player.info.position.as_index(); let fteam = player.info.team;
    let cachep = cache as *const AbstractGameWithCache as *const u8;
    let cc = |p: &PlayerState, field: usize| -> u64 {
        let t = p.info.team; let ps = p.info.position.as_index();
        rd(cachep, 0x280 + t * 4000 + ps * 800 + field + fpos * 8)
    };
    for pc in enemy {
        let p = cache.player_by_champion_id(pc.id).expect("L1002");
        let mut nuke: u64 = 0;
        if pc.can_attack() || pc.attack_cooldown() <= tps_raw { nuke = cc(p, 0x0).wrapping_mul(rng.next()) / 1000; }
        if pc.can_skill() || pc.skill_cooldown() <= tps_raw { nuke = nuke.max(cc(p, 0x28).wrapping_mul(rng.next()) / 1000); }
        if pc.can_skill2() || pc.skill2_cooldown() <= tps_raw { nuke = nuke.max(cc(p, 0x50).wrapping_mul(rng.next()) / 1000); }
        if pc.can_ult() || pc.ult_cooldown() <= tps_raw { nuke = nuke.max(cc(p, 0x78).wrapping_mul(rng.next()) / 1000); }
        if !pc.is_block_attack() { edps = edps.wrapping_add(cc(p, 0x190).wrapping_mul(rng.next()) / 1000); }
        if !pc.is_block_skill() && !(pc.is_block_move_skill() && pc.skill_effect.as_ref().map_or(false, |e| e.ty.can_move())) {
            edps = edps.wrapping_add(cc(p, 0x1b8).wrapping_mul(rng.next()) / 1000);
        }
        if !pc.is_block_skill() && !(pc.is_block_move_skill() && pc.skill2_effect().as_ref().map_or(false, |e| e.ty.can_move())) {
            edps = edps.wrapping_add(cc(p, 0x1e0).wrapping_mul(rng.next()) / 1000);
        }
        enuke = enuke.wrapping_add(nuke);
        *log += &format!("enemy{}(nuke={} dps→{} draws={}) ", pc.id, nuke, edps, rng.n);
    }
    for tw in towers {
        let dmg = tw.attack_effect.as_ref().expect("L1050 tower attack_effect").expected_damage_target(ctx, *tw as &dyn AbstractEntity, focus) as u64;
        let a = rng.next().wrapping_mul(tps_raw as u64).wrapping_mul(dmg) / 1000;
        let ct = tw.attack_cooltime() as u64; if ct == 0 { panic!("div by zero cooltime"); }
        edps = edps.wrapping_add(a / ct);
        enuke = enuke.wrapping_add(dmg.wrapping_mul(rng.next()) / 1000);
        *log += &format!("tower{}(dmg={} ct={} dps→{} nuke→{}) ", tw.id, dmg, ct, edps, enuke);
    }
    for e in cache.others[1 - fteam].iter() {
        if dist_sq_xy(e.x, e.y, focus.x, focus.y) > 22500000000 { continue; }
        if !cache.game.is_visible(judger.info.team, e.id) { *log += &format!("other{} invisible ", e.id); continue; }
        if let Some(atk) = e.attack_effect.as_ref() {
            let dmg = atk.expected_damage_target(ctx, *e as &dyn AbstractEntity, focus) as u64;
            edps = edps.wrapping_add(dmg.wrapping_mul(tps_raw as u64) / (e.attack_cooltime() as u64).max(1));
            enuke = enuke.wrapping_add(dmg);
            *log += &format!("other{}(dmg={} dps→{} nuke→{}) ", e.id, dmg, edps, enuke);
        }
    }
    for pc in enemy { for b in pc.effect_buffs.iter() { edps = edps.wrapping_add(b.expected_aura_dps_at(ctx, pc, focus) as u64); } }
    let mut heal: u64 = 0;
    for al in cache.iter_champions(fteam) { for b in al.effect_buffs.iter() { heal = heal.wrapping_add(b.expected_aura_heal_at(ctx, al, focus) as u64); } }
    edps = edps.saturating_sub(heal);
    // L1095~1099
    let gm = cache.game.get_game_mode();
    let (disc, mp): (i64, *const u8) = unsafe { std::mem::transmute_copy(&gm) }; std::mem::forget(gm);
    let enemy_epic_buff = disc == 0 && !mp.is_null() && rd::<u64>(mp, 0x240 + (1 - fteam) * 8) != 0;
    let low_enough = (focus.hp as u64).wrapping_mul(100) <= (focus.stat_cached.hp as u64).max(1).wrapping_mul(75);
    let line_phase = if matches!(ctx.tutorial, TutorialType::None | TutorialType::MidBottom | TutorialType::Line | TutorialType::Total) {
        let fst: u64 = rd(setting as *const GameSetting as *const u8, 0x8a8);
        (cache.game.tick() as u64) < fst.saturating_sub((tps_raw as u64).wrapping_mul(30))
    } else { true };
    *log += &format!("epic_buff={} low_enough={} line_phase={} heal={} ", enemy_epic_buff, low_enough, line_phase, heal);
    if enemy_epic_buff || (low_enough && !line_phase) {
        let mr = game_ai::enemy_minion_wave_risk_dps_at(version, data, focus, focus.x, focus.y) as u64;
        edps = edps.wrapping_add(mr); *log += &format!("minion_risk={} ", mr);
    }
    let mut revive: u64 = 0;
    for b in focus.effect_buffs.iter() { revive = revive.wrapping_add(b.revive_bonus_hp(ctx, focus) as u64); }
    *log += &format!("edps={} enuke={} revive={} draws={} ", edps, enuke, revive, rng.n);
    ((revive.wrapping_add(focus.hp as u64).saturating_sub(enuke)).wrapping_mul(60) / edps.max(1)) as usize
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::battle_action as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let f = a.get("fn", 209);
    let mut setting = real_setting();
    minion_setting(&mut setting);
    if a.has("matk") { setting.melee_minion.stat.attack = a.get("matk", 10) as usize; setting.range_minion.stat.attack = a.get("ratk", a.get("matk", 15)) as usize; }
    if a.has("fst") { wr(&setting as *const GameSetting as *const u8, 0x8a8, a.get("fst", 0) as u64); }
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tut = match a.get("tut", 0) { 0 => TutorialType::None, 1 => TutorialType::First, 5 => TutorialType::MidBottom, 7 => TutorialType::Line, 8 => TutorialType::Total, _ => TutorialType::Bottom };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(9);
    let ticks = a.get("ticks", 0) as usize;
    for _ in 0..ticks { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    if a.has("tick") { game.set_tick(a.get("tick", 1000) as usize); }
    let version = a.get("version", 2) as usize;
    let team = a.get("team", 0) as usize; let et = 1 - team;
    let posi = a.get("pos", 1) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    // 엔티티 조작은 cache 생성 전에(ChampionCache 가 이펙트를 읽는다)
    {
        let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let champ = cache0.player_champion[team][posi].expect("champ");
        let cp = ep(champ);
        if a.has("cdmg") { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.get("cdmg", 100) as usize, a.get("crange", 30000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
        if a.has("sdmg") { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, Some(mkeff(a.get("sdmg", 100) as usize, a.get("srange", 50000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
        if a.has("s2dmg") { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill2_effect, Some(mkeff(a.get("s2dmg", 100) as usize, a.get("s2range", 50000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
        if a.has("udmg") { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).ult_effect, Some(mkeff(a.get("udmg", 100) as usize, a.get("urange", 50000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
        if a.has("lvl") { wr(cp, 0x5c8, a.get("lvl", 1) as u64); }
        if a.has("chp") { wr(cp, 0x670, a.get("chp", 0) as i64); }
        if a.has("cmhp") { wr(cp, 0x628, a.get("cmhp", 1) as i64); }
        if a.has("und") { wr(cp, 0x488, a.get("und", 0) as u8); }
        if a.has("cx") { wr(cp, 0x660, a.get("cx", 0) as u64); wr(cp, 0x668, a.get("cy", 0) as u64); }
        if a.has("cms") { wr(cp, 0x640, a.get("cms", 1) as u64); }
        // 적 챔피언 조작: e<k>= 슬롯 인덱스 (edmg/ecool/ex,ey/evis)
        for k in 0..5usize {
            let key = format!("e{}", k);
            if !a.has(&key) { continue; }
            let e = cache0.player_champion[et][k].expect("enemy champ");
            let ptr = ep(e);
            if a.has("edmg") { unsafe { std::ptr::write(&mut (*(ptr as *mut Entity)).attack_effect, Some(mkeff(a.get("edmg", 100) as usize, a.get("erange", 30000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
            if a.has("esdmg") { unsafe { std::ptr::write(&mut (*(ptr as *mut Entity)).skill_effect, Some(mkeff(a.get("esdmg", 100) as usize, 50000, CastingType::Targeting, CastingTarget::Enemy))); } }
            if a.has("ecool") { wr(ptr, 0xb0, a.get("ecool", 0) as u64); }
            if a.has("escool") { wr(ptr, 0xb8, a.get("escool", 0) as u64); }
            if a.has("ex") { wr(ptr, 0x660, a.get("ex", 0) as u64); wr(ptr, 0x668, a.get("ey", 0) as u64); }
            if a.has("enear") { let d = a.get("enear", 0) as u64; wr(ptr, 0x660, champ.x + d); wr(ptr, 0x668, champ.y); }
            if a.has("evis") { wr(ptr, 0x38 + team * 24, 0i64); } // visible_state[my_team] = Visible(0)
        }
        // 타워 이펙트 주입(적 타워 전부)
        if a.has("tdmg") {
            for t in [cache0.top_tower[et], cache0.mid_tower[et], cache0.bottom_tower[et], cache0.top_tower2[et], cache0.mid_tower2[et], cache0.bottom_tower2[et]].into_iter().flatten() {
                unsafe { std::ptr::write(&mut (*(ep(t) as *mut Entity)).attack_effect, Some(mkeff(a.get("tdmg", 100) as usize, 60000, CastingType::Targeting, CastingTarget::Enemy))); }
            }
        }
        // 적 에픽버프 시간
        if a.has("ebuff") {
            let gm = cache0.game.get_game_mode();
            let (disc, mp): (i64, *const u8) = unsafe { std::mem::transmute_copy(&gm) }; std::mem::forget(gm);
            if disc == 0 { wr(mp, 0x240 + et * 8, a.get("ebuff", 0) as u64); }
        }
        // 209: 챔프를 적 미니언 k 번째 위치(+dx) 로
        if a.has("nearm") {
            let k = a.get("nearm", 0) as usize;
            let mut ms_: Vec<&Entity> = cache0.iter_minions(et).collect();
            ms_.sort_by_key(|e| e.id);
            if let Some(mn) = ms_.get(k) { wr(cp, 0x660, mn.x + a.get("dx", 0) as u64); wr(cp, 0x668, mn.y + a.get("dy", 0) as u64); }
            else { println!("WARN no enemy minion k={} (count={})", k, ms_.len()); }
        }
        // 209: 적 미니언 n 마리를 챔프 위치로 모음(12 슬롯 상한·순회 순서) / mhp0=k → 적 미니언 k 의 hp=0
        if a.has("gather") {
            let n = a.get("gather", 0) as usize;
            let mut ms_: Vec<&Entity> = cache0.iter_minions(et).collect();
            ms_.sort_by_key(|e| e.id);
            for (i, mn) in ms_.iter().take(n).enumerate() { wr(ep(*mn), 0x660, champ.x + (i as u64) * 100); wr(ep(*mn), 0x668, champ.y); }
            println!("gather {} of {}", n.min(ms_.len()), ms_.len());
        }
        if a.has("gathera") {
            let n = a.get("gathera", 0) as usize;
            let mut ms_: Vec<&Entity> = cache0.iter_minions(team).collect();
            ms_.sort_by_key(|e| e.id);
            for (i, mn) in ms_.iter().take(n).enumerate() { wr(ep(*mn), 0x660, champ.x + 2000 + (i as u64) * 100); wr(ep(*mn), 0x668, champ.y + 500); }
            println!("gathera {} of {}", n.min(ms_.len()), ms_.len());
        }
        if a.has("mhp0") {
            let k = a.get("mhp0", 0) as usize;
            let mut ms_: Vec<&Entity> = cache0.iter_minions(et).collect();
            ms_.sort_by_key(|e| e.id);
            if let Some(mn) = ms_.get(k) { wr(ep(*mn), 0x670, a.get("mhpv", 0) as i64); println!("mhp0 id={} hp→{}", mn.id, a.get("mhpv", 0)); }
        }
        // 210: 챔프를 스틸 대상 위치(+dx)로
        if a.has("attgt") {
            let (mut lg, tsel) = (String::new(), a.get("target", 0));
            let tt = match tsel { 0 => Some(StealTarget::Epic), 1 => Some(StealTarget::Serpen), _ => None };
            let bb0: [Blackboard; 2] = [Default::default(), Default::default()];
            let d0 = OperationData::new(&cache0, &ctx, &bb0);
            if let Some(t) = steal_target(&d0, tt, &mut lg) { wr(cp, 0x660, t.x + a.get("dx", 0) as u64); wr(cp, 0x668, t.y + a.get("dy", 0) as u64); println!("attgt id={} ({},{}) {}", t.id, t.x, t.y, lg); }
            else { println!("attgt: target None {}", lg); }
        }
        if a.has("atcamp") {
            let jt = if a.get("atcamp", 4) == 5 { JungleType::Serpen } else { JungleType::Morgard };
            let (x, y) = map.camp_pos(jt, team == 0); wr(cp, 0x660, x + a.get("dx", 0) as u64); wr(cp, 0x668, y + a.get("dy", 0) as u64);
        }
    }
    if a.has("vtick") { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); if a.has("tick") { game.set_tick(a.get("tick", 1000) as usize); } }
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    if a.has("cnone") { cache.player_champion[team][posi] = None; println!("cnone: player_champion[{}][{}] = None", team, posi); }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let champ = cache.player_champion[team][posi].unwrap_or_else(|| cache.player_champion[et][0].expect("any champ"));
    println!("world team={} pos={} champ.id={} xy=({},{}) hp={}/{} lvl={} ms={} tick={} minions0={} minions1={} pj={} others1={} jungles={}",
        team, posi, champ.id, champ.x, champ.y, champ.hp, champ.stat_cached.hp, champ.level, champ.stat_cached.move_speed, game.tick(),
        cache.iter_minions(team).count(), cache.iter_minions(et).count(), game.iter_projectile().count(), cache.others[et].len(), cache.jungles.len());
    let mut log = String::new();
    match f {
        209 => {
            let depth = a.get("depth", 30) as usize; let sq = a.get("sq", 3) as usize;
            let got = game_ai::build_minion_wave_snapshot(player, &data, depth, sq);
            let gb: [u8; 2320] = unsafe { std::mem::transmute_copy(&got) };
            let pb = predict209(player, &data, depth, sq, &mut log);
            let diff: Vec<usize> = (0..2320).filter(|&i| gb[i] != pb[i]).collect();
            println!("got:  {}", dump209(&gb));
            println!("pred: {}", dump209(&pb));
            println!("log: {}", log);
            println!("RESULT fn=209 depth={} sq={} {} diffbytes={} first={:?}", depth, sq, if diff.is_empty() { "MATCH" } else { "MISMATCH" }, diff.len(), diff.iter().take(8).collect::<Vec<_>>());
        }
        210 => {
            let tsel = a.get("target", 0);
            let tt = match tsel { 0 => Some(StealTarget::Epic), 1 => Some(StealTarget::Serpen), _ => None };
            let commit = a.get("commit", 1) == 1;
            let mut plan = game_ai::plan_legacy::sub_plan::StealSubPlan::new(tt.unwrap_or(StealTarget::Epic), commit);
            if tt.is_none() { wr(&plan as *const _ as *const u8, 8, 2u8); }
            let self_before = bytes_of(&plan);
            let seed = a.get("seed", 777) as u64;
            let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
            let mut rnd2 = rnd.clone();
            let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
            let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
            let got = plan.action_candidates(version, &mut rnd, player, &data, param);
            let self_after = bytes_of(&plan);
            let gv: Vec<[u8; 184]> = got.iter().map(|p| raw184(p)).collect();
            let pv = predict210(tt, commit, &mut rnd2, player, &data, &mut log);
            println!("got tags:  {:?}", gv.iter().map(|b| format!("{}({})", tagname(tag(b)), tag(b))).collect::<Vec<_>>());
            println!("pred tags: {:?}", pv.iter().map(|b| format!("{}({})", tagname(tag(b)), tag(b))).collect::<Vec<_>>());
            println!("log: {}", log);
            let mut ok = gv.len() == pv.len();
            let mut bad = vec![];
            for i in 0..gv.len().min(pv.len()) {
                if tag(&gv[i]) != tag(&pv[i]) { ok = false; bad.push(format!("#{} tag {}≠{}", i, tag(&gv[i]), tag(&pv[i]))); continue; }
                let b = cmp_live(&gv[i], &pv[i]);
                if !b.is_empty() { ok = false; bad.push(format!("#{} live bytes {:?}", i, b.iter().take(10).collect::<Vec<_>>())); }
                let all: Vec<usize> = (0..184).filter(|&j| gv[i][j] != pv[i][j]).collect();
                println!("elem#{} nonlive-diff={:?}", i, all.iter().take(12).collect::<Vec<_>>());
            }
            let rnd_same = rnd.next_u64() == rnd2.next_u64();
            println!("RESULT fn=210 target={} commit={} {} n={} self_diff={} rnd_sync={} bad={:?}", tsel, commit, if ok { "MATCH" } else { "MISMATCH" }, gv.len(), self_before != self_after, rnd_same, bad);
        }
        211 => {
            let focus = champ;
            let jteam = a.get("jteam", team as i64) as usize; let jpos = a.get("jpos", posi as i64) as usize;
            let judger = game.get_player_by_position(jteam, poss[jpos]).expect("judger");
            let ne = a.get("ne", 0) as usize; let nt = a.get("nt", 0) as usize;
            let mut ev: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
            let mut tv: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
            let mut ev2: Vec<&Entity> = vec![]; let mut tv2: Vec<&Entity> = vec![];
            for k in 0..ne { if let Some(e) = cache.player_champion[et][k] { ev.push(e); ev2.push(e); } }
            let tws = [cache.top_tower[et], cache.mid_tower[et], cache.bottom_tower[et], cache.top_tower2[et], cache.mid_tower2[et], cache.bottom_tower2[et]];
            let mut c = 0; for t in tws.into_iter().flatten() { if c >= nt { break; } tv.push(t); tv2.push(t); c += 1; }
            let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
            let mut dbg: DebugFrameData = Default::default();
            let pred = predict211(version, &data, judger, focus, &ev2, &tv2, &mut log);
            let got = game_ai::check_kill_die_tick(version, &mut rnd, &data, judger, focus, ev, tv, &mut dbg);
            println!("log: {}", log);
            println!("RESULT fn=211 ne={} nt={} got={} pred={} {}", ne, nt, got, pred, if got == pred { "MATCH" } else { "MISMATCH" });
        }
        _ => println!("unknown fn"),
    }
}
