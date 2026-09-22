#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치K 오라클 — 190 AttackNexusSubPlan::action_candidates · 191 EpicCheckSubPlan::action_candidates (둘 다 pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv) — 콜리에 TLS 메모(POS_EVAL_CACHE·CAMP_POS_MEMO …)가 있어 프로세스를 가른다(TEMPLATE 함정 ③).
//!  관측: sret Vec 원소 태그(+0xb1)·variant 별 live 바이트 · &mut self(move_check) 전후 · rnd 소비(스냅샷 대조) · debug.infos.
//!  predict = 명세 logic 독립 재구현(콜리는 pub 함수 그대로).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/K/oracle/o25k.rs → %TEMP%\tfm2_spanprobe\o25k.exe
//!  실행: o25k.exe <case> [k=v ...]   (드라이버 = run25k.py)
use game_core::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::collections::HashMap;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::sub_plan::{AttackNexusSubPlan, EpicCheckSubPlan};
use game_ai::plan_legacy::team_plan::TeamPlan;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: &Entity) -> *const u8 { e as *const Entity as *const u8 }
fn hex(p: *const u8, a: usize, b: usize) -> String { (a..b).map(|k| format!("{:02x}", rd::<u8>(p, k))).collect::<Vec<_>>().join("") }
fn dsq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by); dx * dx + dy * dy }

fn mkeff(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}
fn tagname(t: u8) -> &'static str {
    match t { 3 => "RunAway", 4 => "Recall", 5 => "Around", 6 => "AroundHide", 7 => "AroundRegion", 8 => "AroundRunAway", 9 => "Positioning",
              11 => "AroundPositionBush", 12 => "AroundBush", 13 => "LaneMinionPosition", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop", 255 => "None", _ => "AroundPosition(untagged)" }
}
/// variant 별 live 바이트만 찍는다(나머지는 alloca 잔재)
fn live_dump(p: *const u8) -> String {
    let t: u8 = rd(p, 177);
    let body = match t {
        3 => format!("[0,56)={} +125={:02x} [128,132)={} | end_delay={} with_skill={} goal=({},{})", hex(p, 0, 56), rd::<u8>(p, 125), hex(p, 128, 132), rd::<usize>(p, 0x18), rd::<u8>(p, 0x80), rd::<u64>(p, 8), rd::<u64>(p, 0x10)),
        5 | 8 => format!("[0,56)={} +125={:02x} [128,130)={} | target={} range={} end_delay={} purpose={} escape={}", hex(p, 0, 56), rd::<u8>(p, 125), hex(p, 128, 130), rd::<usize>(p, 8), rd::<u64>(p, 0x28), rd::<usize>(p, 0x30), rd::<u8>(p, 0x80), rd::<u8>(p, 0x81)),
        14 => format!("+0={:x} +85={:02x} [88,150)={} | target={} goal=({},{}) margin={} end_delay={} +0x91 attack_range_only={}", rd::<u64>(p, 0), rd::<u8>(p, 85), hex(p, 88, 150), rd::<usize>(p, 0x60), rd::<u64>(p, 0x68), rd::<u64>(p, 0x70), rd::<u64>(p, 0x78), rd::<usize>(p, 0x80), rd::<u8>(p, 0x91)),
        15 | 16 | 17 | 18 => format!("[0,17)={} | target_id={}", hex(p, 0, 17), rd::<usize>(p, 8)),
        0 => format!("[0,48)={} [48,104)={} +173={:02x} +176={:02x} | goal=({},{}) target=({},{}) d={} now_goal=({},{}) radius={} end_delay={}", hex(p, 0, 48), hex(p, 48, 104), rd::<u8>(p, 173), rd::<u8>(p, 176), rd::<u64>(p, 8), rd::<u64>(p, 16), rd::<u64>(p, 32), rd::<u64>(p, 40), rd::<u64>(p, 0x40), rd::<u64>(p, 0x48), rd::<u64>(p, 0x50), rd::<u64>(p, 88), rd::<usize>(p, 96)),
        _ => format!("[0,184)={}", hex(p, 0, 184)),
    };
    format!("tag={}({}) {}", t, tagname(t), body)
}

struct Args { m: HashMap<String, i64>, case: String }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } }

fn clone_entity(e: &Entity) -> &'static mut Entity {
    unsafe {
        let b: Box<std::mem::ManuallyDrop<Entity>> = Box::new(std::mem::ManuallyDrop::new(std::ptr::read(e as *const Entity)));
        let p = Box::leak(b);
        &mut **p
    }
}
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); self.0 >> 33 }
    fn pick(&mut self, n: u64) -> u64 { self.next() % n }
}
fn at(cx: u64, cy: u64, d: u64, k: u64) -> (u64, u64) {
    let (sx, sy): (i64, i64) = match k % 8 { 0 => (1, 0), 1 => (0, 1), 2 => (-1, 0), 3 => (0, -1), 4 => (1, 1), 5 => (-1, 1), 6 => (1, -1), _ => (-1, -1) };
    let diag = k % 8 >= 4;
    let dd = if diag { (d as f64 / 1.41421356) as i64 } else { d as i64 };
    let x = (cx as i64 + sx * dd).clamp(20_000, 940_000) as u64;
    let y = (cy as i64 + sy * dd).clamp(20_000, 940_000) as u64;
    (x, y)
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 99 { let _ = game_ai::position_eval_at as *const (); }
    let case = argv.get(1).cloned().unwrap_or("n_base".into());
    let mut m = HashMap::new();
    for a in argv.iter().skip(2) { if let Some((k, v)) = a.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let A = Args { m, case: case.clone() };
    if case == "probe" {
        // Option<SmallActionPlay>::None 의 +177 태그값 · Option<ObjectivePosture>::None 의 +0 태그값(니치) · Option<PathFinder> None 태그 위치
        let none_sap: Option<game_ai::SmallActionPlay> = None;
        let p = &none_sap as *const _ as *const u8;
        println!("probe	size Option<SmallActionPlay>={}	None +177 = {} (0x{:02x})", std::mem::size_of::<Option<game_ai::SmallActionPlay>>(), rd::<u8>(p, 177), rd::<u8>(p, 177));
        let none_op: Option<game_ai::plan_legacy::team_plan::ObjectivePosture> = None;
        let q = &none_op as *const _ as *const u8;
        println!("probe	size Option<ObjectivePosture>={}	None +0 i64 = {}", std::mem::size_of::<Option<game_ai::plan_legacy::team_plan::ObjectivePosture>>(), rd::<i64>(q, 0));
        let none_pf: Option<game_ai::PathFinder> = None;
        let r = &none_pf as *const _ as *const u8;
        println!("probe	size Option<PathFinder>={}	None +69 = {}", std::mem::size_of::<Option<game_ai::PathFinder>>(), rd::<u8>(r, 69));
        println!("probe	size TeamPlan={}	size EpicCheckSubPlan={}	size AttackNexusSubPlan={}", std::mem::size_of::<TeamPlan>(), std::mem::size_of::<EpicCheckSubPlan>(), std::mem::size_of::<AttackNexusSubPlan>());
        return;
    }
    let is190 = case.starts_with("n_");
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let dbg_ctx = A.get("dbg", 0) == 1;
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: dbg_ctx,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick0 = A.get("tick", 1000) as usize;
    game.set_tick(tick0);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let ver = A.get("ver", 2) as usize;
    let seed = A.get("seed", 7) as u64;
    let mut team = A.get("team", 0) as usize;
    let mut posi = A.get("pos", 0) as usize;   // 0 Top 1 Jungle 2 Mid 3 Bottom 4 Support
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];

    // ── v20B 세계 복제(케이스 e_v20_<n>) — Lcg 시퀀스를 그대로 재현해 v25_objective_posture 가 같은 kind 를 내게 한다 ──
    let mut tp: TeamPlan = Default::default();
    let mut v20 = false;
    if let Some(n) = case.strip_prefix("e_v20_") {
        v20 = true;
        let vcase: u64 = n.parse().unwrap();
        let mut g = Lcg(vcase.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(12345));
        team = (g.pick(2)) as usize;
        let enemy = 1 - team;
        posi = 1;
        let target_is_morgard = g.pick(2) == 0;
        let target = if target_is_morgard { JungleType::Morgard } else { JungleType::Serpen };
        let obj_mode = match g.pick(10) { 0 => 1, 1 => 2, _ => 0 };
        let phase_i = g.pick(4) as u8;
        let (cx, cy) = map.camp_pos(target, team == 0);
        let my_d = [80_000u64, 140_000, 149_000, 151_000, 160_000, 179_000, 181_000, 220_000][g.pick(8) as usize];
        let my_k = g.pick(8);
        let (mx, my) = at(cx, cy, my_d, my_k);
        let mut desc = String::new();
        for t in 0..2usize {
            for p in 0..5usize {
                let e0 = cache.player_champion[t][p].expect("champ");
                let c = clone_entity(e0);
                let b = c as *mut Entity as *mut u8;
                let maxhp: u64 = 1000; unsafe { std::ptr::write_unaligned(b.add(0x628) as *mut u64, maxhp); }
                let mut hp_pct = [100u64, 100, 100, 60, 51, 49, 45, 40, 39, 30][g.pick(10) as usize];
                let mut hp = maxhp * hp_pct / 100;
                if hp == 0 { hp = 1; }
                let (x, y, vis): (u64, u64, i64);
                if t == team && p == 1 { x = mx; y = my; vis = 0; }
                else {
                    let kind = g.pick(3);
                    let k = g.pick(8);
                    let (px, py) = match kind {
                        0 => { let d = [40_000u64, 120_000, 139_000, 141_000, 165_000, 171_000, 179_000, 181_000][g.pick(8) as usize]; at(mx, my, d, k) }
                        1 => { let d = [30_000u64, 100_000, 150_000, 179_000, 181_000, 189_000, 191_000, 229_000, 231_000][g.pick(9) as usize]; at(cx, cy, d, k) }
                        _ => { at(cx, cy, 400_000 + 50_000 * g.pick(4), k) }
                    };
                    x = px; y = py;
                    vis = if t == enemy { if g.pick(4) == 0 { 1 } else { 0 } } else { 0 };
                }
                unsafe {
                    std::ptr::write_unaligned(b.add(0x660) as *mut u64, x); std::ptr::write_unaligned(b.add(0x668) as *mut u64, y); std::ptr::write_unaligned(b.add(0x670) as *mut u64, hp);
                    std::ptr::write_unaligned(b.add(0x38) as *mut i64, vis); std::ptr::write_unaligned(b.add(0x38 + 24) as *mut i64, vis);
                    if t == enemy { let msp = [1u64, 1, 500, 5000][g.pick(4) as usize]; std::ptr::write_unaligned(b.add(0x640) as *mut u64, msp); }
                }
                desc.push_str(&format!("[{}{}:({},{}) hp{}% v{}]", t, p, x, y, hp_pct, vis));
                cache.player_champion[t][p] = Some(&*c);
            }
        }
        let tpp = &mut tp as *mut TeamPlan as *mut u8;
        let obj_tag: i8 = match obj_mode { 0 => if target_is_morgard { 0 } else { 1 }, 1 => if target_is_morgard { 1 } else { 0 }, _ => -1 };
        unsafe { std::ptr::write_unaligned(tpp.add(0x41f) as *mut i8, obj_tag); std::ptr::write_unaligned(tpp.add(0x420) as *mut u8, phase_i); std::ptr::write_unaligned(tpp.add(0x421) as *mut u8, 1); }
        println!("v20cfg\tcase={} team={} tgt={} obj_mode={} phase={} my_d={} my=({},{})\t{}", vcase, team, if target_is_morgard { 4 } else { 5 }, obj_mode, phase_i, my_d, mx, my, desc);
    }

    let enemy = 1 - team;
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let champ = cache.player_champion[team][posi].expect("champ");
    let cp = ep(champ);
    println!("game\ttick={}\tteam={}\tpos={}\tchamp_id={}\tchamp_xy=({},{})\tseed={}\tver={}\tctx.debug={}", tick0, team, posi, champ.id, champ.x, champ.y, seed, ver, dbg_ctx);
    let nexus = cache.nexus[enemy].expect("enemy nexus");
    println!("enemy_nexus\tid={}\txy=({},{})\tcan_target={}\tblock={}\tradius={}", nexus.id, nexus.x, nexus.y, rd::<u8>(ep(nexus), 0x6b9), rd::<usize>(ep(nexus), 0x6a0), nexus.radius());
    for (i, t) in cache.iter_towers(enemy).enumerate() {
        println!("iter_towers[{}]\tid={}\txy=({},{})\tty_tag={}\tcan_target={}\tblock={}", i, t.id, t.x, t.y, rd::<i64>(ep(t), 0x68), rd::<u8>(ep(t), 0x6b9), rd::<usize>(ep(t), 0x6a0));
    }
    let (sx_b, sy_b) = map.camp_pos(JungleType::Stump, true); let (sx_r, sy_r) = map.camp_pos(JungleType::Stump, false);
    let (mx_b, my_b) = map.camp_pos(JungleType::Morgard, true); let (mx_r, my_r) = map.camp_pos(JungleType::Morgard, false);
    println!("camps\tStump(blue)=({},{})\tStump(red)=({},{})\tMorgard(blue)=({},{})\tMorgard(red)=({},{})", sx_b, sy_b, sx_r, sy_r, mx_b, my_b, mx_r, my_r);

    // ── 케이스별 세팅 ──
    let x_arg = A.get("x", -1); let y_arg = A.get("y", -1);
    if x_arg >= 0 && y_arg >= 0 { wr(cp, 0x660, x_arg as u64); wr(cp, 0x668, y_arg as u64); }
    if A.get("atk", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(100, A.get("range", 100000) as u64))); } }
    if A.get("towers_off", 0) == 1 {
        for t in cache.iter_towers(enemy) { if t.id != nexus.id { wr(ep(t), 0x6b9, 0u8); } }
    }
    if A.get("nexus_on", 0) == 1 { wr(ep(nexus), 0x6b9, 1u8); }
    if A.get("maxhp", 0) > 0 { wr(cp, 0x628, A.get("maxhp", 0) as u64); }
    if A.get("hp", 0) > 0 { wr(cp, 0x670, A.get("hp", 0) as u64); }
    // n_range_eq/gt: 넥서스 기준 정확히 max_dist(+delta) 떨어진 점에 둔다(중앙 방향)
    if let Some(delta) = A.m.get("range_delta") {
        let atk = champ.attack_effect.as_ref().expect("atk");
        let level: u64 = rd(cp, 0x5c8);
        let sbr: u64 = rd(cp, 0x438);
        let ms_: u64 = rd(cp, 0x640);
        let max_dist = atk.range + sbr + (level - 1) * atk.growth_range + atk.range_adjust(champ, nexus) + champ.radius() as u64 + nexus.radius() as u64 + ms_ * 30;
        let d = (max_dist as i64 + delta) as u64;
        let nx = if nexus.x < 480000 { nexus.x + d } else { nexus.x - d };
        wr(cp, 0x660, nx); wr(cp, 0x668, nexus.y);
        println!("range_setup\tmax_dist={}\tdelta={}\tchamp=({},{})\tdist_sq={}\tmax_sq={}", max_dist, delta, nx, nexus.y, dsq(nx, nexus.y, nexus.x, nexus.y), max_dist * max_dist);
    }
    println!("champ_now\txy=({},{})\thp={}/{}\tlevel={}\tmove_speed={}\tatk={}", champ.x, champ.y, rd::<u64>(cp, 0x670), rd::<u64>(cp, 0x628), rd::<u64>(cp, 0x5c8), rd::<u64>(cp, 0x640), champ.attack_effect.is_some());

    // ScoreParameter: 0 버퍼 + positioning_score(Default) + version — 두 함수 모두 +0x9f0 만 읽는다
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe {
        std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default());
        std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).version), ver);
    }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let rnd_snap = rnd.clone();

    // 공통 predict 재료: has_non_target_action_range(적 챔프 5칸) · position_score
    let mut hnt = false;
    for c in cache.player_champion[enemy].iter().flatten() {
        let w = game_ai::nontarget_windup_perceived(ver, player, &data, c);
        let tyc: i64 = rd(ep(c), 0x68);
        if w && tyc == 13 { println!("hnt_candidate\tid={}", c.id); }
    }
    let purpose = if is190 { game_ai::PositionEvalPurpose::General } else { game_ai::PositionEvalPurpose::Objective };
    let ps = game_ai::position_score_at_position(ver, player, &data, &param.positioning_score, champ.x, champ.y, purpose);
    let on_traj: bool = rd(&ps as *const _ as *const u8, 0x30); let on_ptraj: bool = rd(&ps as *const _ as *const u8, 0x31);
    println!("position_score\ton_trajectory={}\ton_periodic={}\t(별도 프로세스 값 아님: 같은 프로세스에서 먼저 불러 캐시를 채움 — 함수 호출 결과와 같은 값을 재생)", on_traj, on_ptraj);

    if is190 {
        // ── predict(attack_tower_action 부분) ──
        let mut best: Option<&Entity> = None; let mut bd = u64::MAX;
        for t in cache.iter_towers(enemy) {
            if !(t.can_target()) { continue; }
            let d = dsq(t.x, t.y, champ.x, champ.y);
            if d < bd { bd = d; best = Some(t); }
        }
        let mut pred_tower: Option<usize> = None; let mut pred_note = String::new();
        if let Some(t) = best {
            let hp: u64 = rd(cp, 0x670); let mh: u64 = rd(cp, 0x628);
            let ratio = hp * 100 / mh.max(1);
            let mut vetoed = false;
            if ratio < 56 { let v = game_ai::can_tower_focused_when_attack(&ctx, &cache, player, t); pred_note += &format!("hp_ratio={} <56 → can_tower_focused_when_attack={} ", ratio, v); if v { vetoed = true; } } else { pred_note += &format!("hp_ratio={} ", ratio); }
            if !vetoed {
                if let Some(atk) = champ.attack_effect.as_ref() {
                    let level: u64 = rd(cp, 0x5c8); let sbr: u64 = rd(cp, 0x438); let ms_: u64 = rd(cp, 0x640);
                    let max_dist = atk.range + sbr + (level - 1) * atk.growth_range + atk.range_adjust(champ, t) + champ.radius() as u64 + t.radius() as u64 + ms_ * 30;
                    let d = dsq(t.x, t.y, champ.x, champ.y);
                    pred_note += &format!("nearest={} dist_sq={} max_dist={} max_sq={} ", t.id, d, max_dist, max_dist * max_dist);
                    if d <= max_dist * max_dist { pred_tower = Some(t.id); }
                } else { pred_note += "attack_effect None → None "; }
            }
        } else { pred_note += "no targetable tower → None "; }
        println!("predict_tower\t{:?}\t{}", pred_tower, pred_note);
        let danger = on_traj || hnt || on_ptraj;
        println!("predict_head\t{}", if danger { "[RunAway(with_skill=1)] 단독 조기반환".to_string() } else { format!("[Around(target={},end_delay=5,purpose=5), RunAway(with_skill=1)] + battle_action + (미니언 없음) + summon + tower={:?} + structure_skill", nexus.id, pred_tower) });

        let mut sub: AttackNexusSubPlan = Default::default();
        let res = sub.action_candidates(ver, &mut rnd, player, &data, param, &mut dbg);
        println!("rnd_advanced={}\t(기대 false — Around::new·battle_action 모두 rnd readnone)", rnd != rnd_snap);
        println!("sret\tlen={}\tcap={}", res.len(), res.capacity());
        let mut tags = vec![];
        for (i, e) in res.iter().enumerate() {
            let p = e as *const game_ai::SmallActionPlay as *const u8;
            tags.push(rd::<u8>(p, 177));
            println!("elem[{}]\t{}", i, live_dump(p));
        }
        let attack_targets: Vec<usize> = res.iter().filter(|e| rd::<u8>(*e as *const game_ai::SmallActionPlay as *const u8, 177) == 15).map(|e| rd::<usize>(e as *const game_ai::SmallActionPlay as *const u8, 8)).collect();
        let has_nexus_attack = attack_targets.contains(&nexus.id);
        let tower_attack = attack_targets.iter().find(|&&id| cache.iter_towers(enemy).any(|t| t.id == id)).cloned();
        println!("summary\ttags={:?}\tattack_targets={:?}\ttower_attack={:?}\tpredict={:?}\t{}", tags, attack_targets, tower_attack, pred_tower,
                 if danger { "DANGER-PATH" } else if tower_attack == pred_tower { "TOWER MATCH" } else { "TOWER MISMATCH" });
        println!("debug.infos\t{:?}", dbg.infos);
    } else {
        // ── 191 ──
        let mc0 = A.get("mc", 0) as u8;
        let mut sub: EpicCheckSubPlan = Default::default();
        let sp = &mut sub as *mut EpicCheckSubPlan as *const u8;
        wr(sp, 0, mc0);
        let before: u8 = rd(sp, 0);
        // predict (non-posture 경로)
        let (cx, cy) = champ.x.clone().pipe(|x| (x, champ.y));
        let is_es = game_core::is_enemy_side(&ctx, team, champ.x, champ.y);
        let is_blue = game_core::is_blue_side(&ctx, champ.x, champ.y);
        let (stx, sty) = map.camp_pos(JungleType::Stump, team == 0);
        let (mgx, mgy) = map.camp_pos(JungleType::Morgard, team == 0);
        let d_stump = dsq(champ.x, champ.y, stx, sty);
        let d_morg = dsq(champ.x, champ.y, mgx, mgy);
        let mut pred_mc = mc0 == 1;
        let mut pred_write = false;
        // ★1차 실행에서 명세 logic(적 진영이면 Stump 검사)이 반증됨 → IR 분기(%290=xor(team==0, x-y+H>W) 참=Stump검사) = 자기 진영이면 Stump 검사
        if !pred_mc {
            if !is_es { if d_stump < 4900000001 { pred_mc = true; pred_write = true; } }
            else { pred_mc = true; pred_write = true; }
        }
        let raw289 = champ.x.wrapping_sub(champ.y).wrapping_add(setting.height) > setting.width;
        println!("sideprobe	pub is_blue_side={} pub is_enemy_side={} raw(x-y+H>W)={} team==0:{} xor(team==0,raw)={} | probes: is_blue(100000,900000)={} is_blue(900000,100000)={} is_blue(500000,500000)={} is_enemy(t0,900000,100000)={} is_enemy(t1,900000,100000)={}",
            is_blue, is_es, raw289, team == 0, (team == 0) ^ raw289, game_core::is_blue_side(&ctx, 100000, 900000), game_core::is_blue_side(&ctx, 900000, 100000), game_core::is_blue_side(&ctx, 500000, 500000), game_core::is_enemy_side(&ctx, 0, 900000, 100000), game_core::is_enemy_side(&ctx, 1, 900000, 100000));
        let pred_around = if d_morg > 22500000000 { if pred_mc { ("Morgard", mgx, mgy) } else { ("Stump", stx, sty) } } else { ("Morgard(near)", mgx, mgy) };
        let mut enemy_near = false;
        for c in cache.player_champion[enemy].iter().flatten() {
            if bb[enemy].is_recent_visible(&game as &dyn AbstractGame, player, c) && dsq(c.x, c.y, champ.x, champ.y) < 22500000001 { enemy_near = true; }
        }
        if !enemy_near { for x in cache.others[enemy].iter() { if dsq(x.x, x.y, champ.x, champ.y) < 22500000001 { enemy_near = true; } } }
        let vis_to_enemy = game.is_visible(enemy, champ.id);
        println!("predict191\tis_enemy_side={} is_blue_side={} team==0:{} d_stump={} (<=70000²:{}) d_morg={} (>150000²:{}) mc0={} → move_check_after={} self_write={} around={:?} enemy_near={} visible_to_enemy={}",
                 is_es, is_blue, team == 0, d_stump, d_stump < 4900000001, d_morg, d_morg > 22500000000, mc0, pred_mc, pred_write, pred_around, enemy_near, vis_to_enemy);
        let danger = on_traj || hnt || on_ptraj;
        let res = sub.action_candidates(ver, &mut rnd, player, &data, param, &tp, &mut dbg);
        let after: u8 = rd(sp, 0);
        // rnd 소비 대조: AroundPosition::new → wait_around → SliceRandom::choose(12칸) 1회
        let mut r1 = rnd_snap.clone(); let tbl = [0u8; 12]; let _ = tbl.choose(&mut r1);
        let mut r2 = rnd_snap.clone(); let _ = tbl.choose(&mut r2); let _ = tbl.choose(&mut r2);
        println!("rnd\tadvanced={}\teq_one_choose12={}\teq_two_choose12={}", rnd != rnd_snap, rnd == r1, rnd == r2);
        println!("self.move_check\tbefore={}\tafter={}\tpredict_after={}\t{}", before, after, pred_mc as u8, if after == pred_mc as u8 { "SELF MATCH" } else { "SELF MISMATCH" });
        println!("sret\tlen={}\tcap={}", res.len(), res.capacity());
        let mut tags = vec![];
        for (i, e) in res.iter().enumerate() {
            let p = e as *const game_ai::SmallActionPlay as *const u8;
            tags.push(rd::<u8>(p, 177));
            println!("elem[{}]\t{}", i, live_dump(p));
        }
        // AroundPosition 원소의 goal 이 predict 와 같은가
        let ap: Vec<(u64, u64)> = res.iter().filter(|e| rd::<u8>(*e as *const game_ai::SmallActionPlay as *const u8, 177) == 0).map(|e| { let p = e as *const game_ai::SmallActionPlay as *const u8; (rd::<u64>(p, 8), rd::<u64>(p, 16)) }).collect();
        let verdict = if danger { "DANGER-PATH".to_string() } else if v20 { "V20-POSTURE (observe)".to_string() } else if ap.len() == 1 && ap[0] == (pred_around.1, pred_around.2) { format!("AROUND MATCH({})", pred_around.0) } else { format!("AROUND MISMATCH got={:?} pred={:?}", ap, pred_around) };
        println!("summary\ttags={:?}\taround_goals={:?}\t{}", tags, ap, verdict);
        println!("debug.infos\t{:?}", dbg.infos);
    }
}

trait Pipe: Sized { fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R { f(self) } }
impl<T> Pipe for T {}
