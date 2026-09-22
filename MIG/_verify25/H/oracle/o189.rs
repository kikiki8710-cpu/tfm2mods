#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치H · 189 DefenseNexusSubPlan::action_candidates 오라클 (pub 직접 호출) — 명세 `logic` 독립 재구현(predict) ↔ 실행 대조.
//!  한 프로세스 = 한 케이스(argv `sc=<n> k=v ...`). 세계 = TEMPLATE mkgame(+미니언 설정, `minions=1` 이면 run_tick N).
//!  대조: ①sret Vec 원소 수·variant 태그(+0xb1) ②variant 별 live 바이트(콜리 define `initializes` 범위) ③&mut self 24B(focus tag/id · last_gate)
//!        ④rnd 소비 동기(복제 rng) ⑤TLS 캐시(LAST_STAND_MEMO/DIE_TICK/POS_EVAL) 는 실행→predict 순서라 두 번째가 hit(값 동일).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/H/oracle/o189.rs
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

/// SmallActionPlay 184B 를 바이트 배열로
fn raw184(p: &game_ai::SmallActionPlay) -> [u8; 184] { unsafe { std::ptr::read(p as *const _ as *const [u8; 184]) } }
fn tag(b: &[u8; 184]) -> u8 { b[0xb1] }
/// variant 별 live 범위(콜리 define initializes + 태그)
fn live_ranges(t: u8) -> Vec<(usize, usize)> {
    match t {
        3 => vec![(0, 56), (125, 126), (128, 132), (177, 178)],        // RunAway: new/new_with_skill initializes((0,56),(125,126),(128,132))
        5 => vec![(0, 56), (125, 126), (128, 130), (177, 178)],        // Around: initializes((0,56),(125,126),(128,130))
        14 => vec![(0, 8), (85, 86), (88, 150), (177, 178)],          // Trace: initializes((0,8),(85,86),(88,150))
        15 | 16 | 17 => vec![(0, 17), (177, 178)],                     // Attack/Skill/Skill2: initializes((0,17))
        _ => vec![(0, 17), (177, 178)],                                // 미상 variant(battle_action 등 콜리 산출): 최소 범위만
    }
}
fn cmp_live(a: &[u8; 184], b: &[u8; 184]) -> (bool, Vec<usize>) {
    let mut bad = vec![];
    for (lo, hi) in live_ranges(tag(a)) { for i in lo..hi { if a[i] != b[i] { bad.push(i); } } }
    (bad.is_empty(), bad)
}
fn cmp_all(a: &[u8; 184], b: &[u8; 184]) -> Vec<usize> { (0..184).filter(|&i| a[i] != b[i]).collect() }

fn wrap<T>(x: T) -> [u8; 184] { unsafe { std::mem::transmute_copy::<T, [u8; 184]>(&x) } }
/// 예측 원소 생성: variant 페이로드 + 태그
fn mk_elem(payload: &[u8], t: u8) -> [u8; 184] {
    let mut b = [0u8; 184];
    b[..payload.len()].copy_from_slice(payload);
    b[0xb1] = t; b
}
fn bytes_of<T>(v: &T) -> Vec<u8> { unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()).to_vec() } }

struct Pred { res: Vec<[u8; 184]>, src: Vec<&'static str>, focus: (u64, u64), last_gate: u8, log: String }

/// ★명세 logic 의 독립 재구현. 콜리는 pub 함수를 그대로 쓴다(합성만 검증).
fn predict<'a>(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &'a OperationData<'a, 'a>, param: &game_ai::ScoreParameter,
               focus0: (u64, u64), bump: &'a bumpalo::Bump, dbg: &mut DebugFrameData) -> Pred {
    let mut P = Pred { res: vec![], src: vec![], focus: focus0, last_gate: 0, log: String::new() };
    let team = player.info.team; let pos = player.info.position.as_index();
    let cache = data.cache;
    // ── commit_chase (L35~95)
    'cc: {
        if version < 2 || !game_ai::plan_legacy::old::base_defense_focus(player, data) { P.focus = (0, P.focus.1); P.log += "cc:bdf=false "; break 'cc; }
        P.log += "cc:bdf=true ";
        let champ = match cache.player_champion[team][pos] { Some(c) => c, None => { P.log += "cc:nochamp "; break 'cc; } };
        let minion = game_ai::plan_legacy::old::base_attacking_minion(player, data);
        P.log += &format!("cc:minion={:?} ", minion);
        let champ_target: Option<usize> = if minion.is_some() { None } else {
            let nexus = cache.nexus[team]; let twins = &cache.twin_towers[team];
            let threats: Vec<&Entity> = cache.iter_champions(1 - team).filter(|e| e.attack_effect.as_ref().map_or(false, |atk|
                nexus.map_or(false, |n| atk.is_in_range(e, n)) || twins.iter().any(|t| atk.is_in_range(e, t)))).collect();
            P.log += &format!("cc:threats={:?} ", threats.iter().map(|e| e.id).collect::<Vec<_>>());
            let mut best: Option<(usize, u64, &Entity)> = None;
            for e in threats.iter() {
                let mut attackers = bumpalo::collections::Vec::new_in(bump);
                for a in cache.iter_champions(team) { if dist_sq(a, e) <= 120000u64 * 120000 { attackers.push(a); } }
                let mut towers = bumpalo::collections::Vec::new_in(bump);
                for t in cache.iter_towers(team) { if t.can_target && t.block_target_tick == 0 && t.attack_effect.as_ref().map_or(false, |atk| atk.is_in_range(t, e)) { towers.push(t); } }
                let die = game_ai::check_kill_die_tick(version, rnd, data, player, e, attackers, towers, dbg);
                let key = (die, dist_sq(e, champ));
                P.log += &format!("cc:key(id={})=({},{}) ", e.id, key.0, key.1);
                if best.map_or(true, |(d, q, _)| key < (d, q)) { best = Some((key.0, key.1, e)); }
            }
            best.map(|(_, _, e)| e.id)
        };
        let want: Option<usize> = minion.or(champ_target);
        let kept: Option<usize> = if P.focus.0 == 1 {
            let id = P.focus.1 as usize;
            let ok = match cache.game.get_entity_by_id(id) {
                None => false,
                Some(e) => if minion.is_some() { game_ai::plan_legacy::old::is_base_attacking_minion(player, data, e) } else {
                    let ety: i64 = rd(ep(e), 0x68);
                    e.team == TeamType::Player(1 - team) && ety == 13 && e.attack_effect.as_ref().map_or(false, |atk| {
                        let nexus = cache.nexus[team]; let twins = &cache.twin_towers[team];
                        nexus.map_or(false, |n| atk.is_in_range(e, n)) || twins.iter().any(|t| atk.is_in_range(e, t)) })
                }
            };
            if ok { Some(id) } else { None }
        } else { None };
        let f = kept.or(want);
        P.log += &format!("cc:kept={:?} want={:?} focus={:?} ", kept, want, f);
        match f { Some(id) => { P.focus = (1, id as u64); let tr = game_ai::SmallActionTrace::new(data, id, 5); P.res.push(mk_elem(&bytes_of(&tr), 14)); P.src.push("Trace"); }
                  None => { P.focus = (0, P.focus.1); } }
    }
    // ── L253~
    let last_stand = version > 1 && game_ai::plan_legacy::old::base_defense_focus(player, data);
    let champ = cache.player_champion[team][pos].expect("champ L256");
    let has_non_target = cache.iter_champions(1 - team).any(|c| {
        let w = game_ai::nontarget_windup_perceived(version, player, data, c);
        let cty: i64 = rd(ep(c), 0x68);
        if !(w && cty == 13) { return false; }
        let st: i64 = rd(ep(c), 0x70);
        let eff: &Effect = match st { 4 => c.skill_effect.as_ref().unwrap(), 5 => c.skill2_effect().as_ref().unwrap(), 6 => c.ult_effect().as_ref().unwrap(), _ => return false };
        matches!(eff.casting, CastingType::Position | CastingType::Direction) && eff.is_in_range(c, champ)
    });
    let ps = game_ai::position_score_at_position(version, player, data, &param.positioning_score, champ.x, champ.y, game_ai::PositionEvalPurpose::General);
    let on_traj = ps.on_trajectory || ps.on_periodic_trajectory;
    P.log += &format!("last_stand={} hnt={} on_traj={}/{} ", last_stand, has_non_target, ps.on_trajectory, ps.on_periodic_trajectory);
    if !last_stand && (on_traj || has_non_target) {
        P.last_gate = 1;
        let r = game_ai::SmallActionRunAway::new_with_skill(data, player, 5, true);
        P.res.push(mk_elem(&bytes_of(&r), 3)); P.src.push("RunAway_skill(L279)"); P.log += "RET:L279 "; return P;
    }
    let tps = data.context.setting.tick_per_second;
    let mwd = game_ai::enemy_minion_wave_danger_damage_at(version, data, champ, champ.x, champ.y, tps * 2);
    let hp_ratio = champ.hp * 100 / std::cmp::max(champ.stat_cached.hp, 1);
    let critical = game_ai::plan_legacy::old::nexus_is_critical(player, data);
    P.log += &format!("mwd={} hp={} ratio={} critical={} ", mwd, champ.hp, hp_ratio, critical);
    if mwd != 0 && (mwd >= champ.hp || hp_ratio < 46) {
        if !last_stand && !(critical && mwd < champ.hp) {
            P.last_gate = 2;
            let r = game_ai::SmallActionRunAway::new(data, player, 5);
            P.res.push(mk_elem(&bytes_of(&r), 3)); P.src.push("RunAway(L294)"); P.log += "RET:L294 "; return P;
        }
    }
    // ── base_positioning (L98~135)
    {
        let champ = cache.player_champion[team][pos].expect("champ L100");
        let enemy_minions = cache.minions(1 - team, bump);
        let twins = &cache.twin_towers[team];
        let attacked_twin: Option<&Entity> = twins.iter().filter(|t| enemy_minions.iter().any(|m| {
            let mp = ep(*m); let mty: i64 = rd(mp, 0x68); let ne: i64 = rd(mp, 0x88); let nid: usize = rd(mp, 0x90);
            mty == 1 && ne == 1 && nid == t.id })).copied().min_by_key(|t| dist_sq(t, champ));
        let nexus = cache.nexus[team].expect("nexus L105");
        let bb = &data.blackboard[1 - team];
        let fm = |st: &BrainMinionParameter| -> Option<&Entity> { st.front_minion.and_then(|id| cache.game.get_entity_by_id(id)) };
        let (top, mid, bot) = (fm(&bb.top_minion_state), fm(&bb.mid_minion_state), fm(&bb.bottom_minion_state));
        let front: Vec<&Entity> = vec![top, mid, bot].into_iter().flatten().collect();
        P.log += &format!("bp:att_twin={:?} front={:?} focus_none={} ", attacked_twin.map(|t| t.id), front.iter().map(|e| e.id).collect::<Vec<_>>(), P.focus.0 == 0);
        if P.focus.0 == 0 {
            let nearest_front = front.iter().copied().min_by_key(|m| dist_sq(m, nexus));
            if let Some(m) = nearest_front { let a = game_ai::SmallActionAround::new(version, rnd, data, player, m.id, 5); P.res.push(mk_elem(&bytes_of(&a), 5)); P.src.push("Around(front L120)"); }
            let twin_tower = twins.iter().copied().min_by_key(|t| dist_sq(t, champ));
            let tid = if let Some(t) = attacked_twin { P.src.push("Around(attacked_twin L125)"); t.id } else if let Some(t) = twin_tower { P.src.push("Around(twin L127)"); t.id } else { P.src.push("Around(nexus L129)"); nexus.id };
            let a = game_ai::SmallActionAround::new(version, rnd, data, player, tid, 5); P.res.push(mk_elem(&bytes_of(&a), 5));
        }
    }
    { let r = game_ai::SmallActionRunAway::new(data, player, 5); P.res.push(mk_elem(&bytes_of(&r), 3)); P.src.push("RunAway(L301)"); }
    { let v = game_ai::battle_action(version, rnd, player, data, 5); for e in v.iter() { P.res.push(raw184(e)); P.src.push("battle_action"); } }
    // ── attack_minion_actions (L137~195)
    {
        let champ = cache.player_champion[team][pos].expect("champ L137");
        let focus_near: Option<usize> = if P.focus.0 == 1 { let id = P.focus.1 as usize; cache.game.get_entity_by_id(id).filter(|e| dist_sq(e, champ) < 80000u64 * 80000).map(|_| id) } else { None };
        let speed = champ.stat_cached.move_speed;
        let mut n = 0;
        for m in cache.iter_minions(1 - team) {
            if !(focus_near.map_or(true, |f| m.id == f) && dist_sq(m, champ) < 80000u64 * 80000) { continue; }
            n += 1;
            if !m.is_visible_from(champ) { P.log += &format!("am:m{} invisible ", m.id); continue; }
            if champ.can_attack() {
                let atk = champ.attack_effect.as_ref().unwrap();
                let range = atk.range(champ) + atk.range_adjust(champ, m) + champ.radius() as u64 + (speed * 30) as u64 + m.radius() as u64;
                P.log += &format!("am:m{} d2={} r={} ", m.id, dist_sq(m, champ), range);
                if dist_sq(m, champ) <= range * range { let a = game_ai::SmallActionAttack::new(data, m.id); P.res.push(mk_elem(&bytes_of(&a), 15)); P.src.push("Attack(minion L164)"); }
            }
            if let Some(sk) = champ.skill_effect.as_ref() {
                if champ.can_skill() && sk.target.check(champ, m) {
                    let range = sk.range(champ) + sk.range_adjust(champ, m) + champ.radius() as u64 + (speed * 30) as u64 + m.radius() as u64;
                    if dist_sq(m, champ) <= range * range { let a = game_ai::SmallActionSkill::new(data, m.id); P.res.push(mk_elem(&bytes_of(&a), 16)); P.src.push("Skill(minion L176)"); }
                }
            }
            if let Some(sk2) = champ.skill2_effect().as_ref() {
                if champ.can_skill2() && sk2.target.check(champ, m) {
                    let range = sk2.range(champ) + sk2.range_adjust(champ, m) + champ.radius() as u64 + (speed * 30) as u64 + m.radius() as u64;
                    if dist_sq(m, champ) <= range * range { let a = game_ai::SmallActionSkill2::new(data, m.id); P.res.push(mk_elem(&bytes_of(&a), 17)); P.src.push("Skill2(minion L189)"); }
                }
            }
        }
        P.log += &format!("am:cands={} ", n);
    }
    { let v = game_ai::attack_summon_action(player, data); for e in v.iter() { P.res.push(raw184(e)); P.src.push("attack_summon_action"); } }
    // ── attack_tower_action (L198~223)
    'at: {
        let champ = match cache.player_champion[team][pos] { Some(c) => c, None => break 'at };
        let tower = match cache.iter_towers(1 - team).filter(|t| t.can_target && t.block_target_tick == 0).min_by_key(|t| dist_sq(t, champ)) { Some(t) => t, None => { P.log += "at:notower "; break 'at } };
        let speed = champ.stat_cached.move_speed;
        let atk = match champ.attack_effect.as_ref() { Some(a) => a, None => { P.log += "at:noatk "; break 'at } };
        let ally_in = cache.iter_minions(team).any(|m| atk.is_in_range(champ, m));
        let d2 = dist_sq(tower, champ);
        let range = atk.range(champ) + atk.range_adjust(champ, tower) + champ.radius() as u64 + (speed * 30) as u64 + tower.radius() as u64;
        P.log += &format!("at:tower={} d2={} r2={} ally_in={} ", tower.id, d2, range * range, ally_in);
        if d2 > range * range { break 'at; }
        let tdmg = tower.attack_effect.as_ref().map_or(0, |e| e.expected_damage_target(data.context, tower as &dyn AbstractEntity, champ));
        let focused = game_ai::can_tower_focused_when_attack(data.context, cache, player, tower);
        P.log += &format!("at:tdmg={} focused={} can_attack={} hp={} ", tdmg, focused, champ.can_attack(), champ.hp);
        if !focused && ally_in && champ.can_attack() && champ.hp > tdmg {
            let a = game_ai::SmallActionAttack::new(data, tower.id); P.res.push(mk_elem(&bytes_of(&a), 15)); P.src.push("Attack(tower L223)");
        }
    }
    { let v = game_ai::attack_structure_skill_action(player, data); for e in v.iter() { P.res.push(raw184(e)); P.src.push("attack_structure_skill_action"); } }
    P
}

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
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let team = a.get("team", 0) as usize;
    let posi = a.get("pos", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let champ = cache.player_champion[team][posi].expect("champ");
    let cp = ep(champ);
    let nexus0 = cache.nexus[team].expect("nexus");
    let twins0 = &cache.twin_towers[team];
    let enemy_twins = &cache.twin_towers[1 - team];
    println!("world\tteam={}\tpos={}\tchamp.id={}\tchamp=({},{})\tnexus.id={}\tnexus=({},{})\ttwins={:?}\tenemy_twins={:?}\tminions0={}\tminions1={}",
        team, posi, champ.id, champ.x, champ.y, nexus0.id, nexus0.x, nexus0.y,
        twins0.iter().map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>(), enemy_twins.iter().map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>(),
        cache.iter_minions(team).count(), cache.iter_minions(1 - team).count());
    // 챔피언 평타 이펙트(L156/L205 unwrap 방지)
    if a.get("catk", 1) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.get("cdmg", 100) as usize, a.get("crange", 30000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
    if a.get("cskill", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, Some(mkeff(50, a.get("srange", 40000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } }
    if a.get("chp", -1) >= 0 { wr(cp, 0x670, a.get("chp", 0) as usize); }
    if a.get("cmax", -1) >= 0 { wr(cp, 0x628, a.get("cmax", 0) as usize); }
    if a.get("nhp", -1) >= 0 { wr(ep(nexus0), 0x670, a.get("nhp", 0) as usize); }
    if a.get("nmax", -1) >= 0 { wr(ep(nexus0), 0x628, a.get("nmax", 0) as usize); }
    // 적 챔피언 넥서스 위협(sc=7 과 결합 가능): en>0 이면 적 챔피언 en 명을 내 넥서스 옆에
    if a.get("en", 0) > 0 { let n = a.get("en", 0) as usize; for i in 0..n { if let Some(e) = cache.player_champion[1 - team][i] { let epp = ep(e);
        wr(epp, 0x660, nexus0.x + 15000 * (i as u64 + 1)); wr(epp, 0x668, nexus0.y);
        unsafe { std::ptr::write(&mut (*(epp as *mut Entity)).attack_effect, Some(mkeff(100, 60000, CastingType::Targeting, CastingTarget::Enemy))); } } } }
    // 시나리오
    let mut sp: game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan = Default::default();
    let spp = &sp as *const _ as *const u8;
    if a.get("focus", -1) >= 0 { wr(spp, 0, 1u64); wr(spp, 8, a.get("focus", 0) as u64); }
    if a.get("focus_tag_only", 0) == 1 { wr(spp, 0, 1u64); wr(spp, 8, 999999u64); }
    if a.get("last_gate0", -1) >= 0 { wr(spp, 16, a.get("last_gate0", 0) as u8); }
    match sc {
        1 => { // 블랙보드 front_minion = 적 챔피언(인덱스) — 넥서스 최근접 선택 검증
            let bp = &bb[1 - team] as *const Blackboard as *const u8;
            for (k, off) in [("bbtop", 0usize), ("bbmid", 0x28), ("bbbot", 0x50)] {
                let v = a.get(k, -1);
                if v >= 0 { let e = cache.player_champion[1 - team][v as usize].expect("enemy champ"); wr(bp, off, 1u64); wr(bp, off + 8, e.id); }
                if v == -2 { wr(bp, off, 1u64); wr(bp, off + 8, 999999usize); } // 존재하지 않는 id → and_then None
            }
        }
        2 => { // 적 미니언 k 마리의 nearest_enemy = 내 twin[twk] → attacked_twin_tower
            let twk = a.get("twk", 1) as usize; let k = a.get("k", 1) as usize;
            let tid = twins0[twk].id;
            for (i, mn) in cache.iter_minions(1 - team).enumerate() { if i < k { wr(ep(mn), 0x88, 1i64); wr(ep(mn), 0x90, tid);
                if a.get("mfar", 0) == 1 { wr(ep(mn), 0x660, 1u64); wr(ep(mn), 0x668, 1u64); }
                if a.get("mfar", 0) == 2 { wr(ep(mn), 0x660, twins0[twk].x + 20000); wr(ep(mn), 0x668, twins0[twk].y); } } }
        }
        3 | 5 => { // 적 미니언 n 마리를 champ 옆으로(dx 간격) + 가시화 → Attack/Skill 후보 · 웨이브 피해
            let n = a.get("n", 3) as usize; let dx = a.get("dx", 5000) as u64;
            for (i, mn) in cache.iter_minions(1 - team).enumerate() {
                let mp = ep(mn);
                if i < n { wr(mp, 0x660, champ.x + dx * (i as u64 + 1)); wr(mp, 0x668, champ.y); if a.get("vis", 1) == 1 { wr(mp, 0x38 + team * 24, 0i64); } else { wr(mp, 0x38 + team * 24, 1i64); } }
                else { wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); }
            }
        }
        4 => { // champ 를 적 타워 옆으로 + 아군 미니언 동행 → attack_tower_action
            let tk = a.get("tk", 0) as usize; // 0 top_tower / 1 twin[0] / 2 nexus 근처 twin[1]
            let t: &Entity = match tk { 0 => cache.top_tower[1 - team].expect("tower"), 1 => enemy_twins[0], _ => enemy_twins[1] };
            let dx = a.get("dx", 20000) as u64;
            wr(cp, 0x660, t.x + dx); wr(cp, 0x668, t.y);
            let n = a.get("n", 2) as usize;
            let mdx = a.get("mdx", 3000) as u64;
            let mut first_ally: Option<usize> = None;
            for (i, mn) in cache.iter_minions(team).enumerate() { let mp = ep(mn); if i < n { wr(mp, 0x660, t.x + mdx); wr(mp, 0x668, t.y); if first_ally.is_none() { first_ally = Some(mn.id); } } else { wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); } }
            if a.get("tne", 0) == 1 { if let Some(id) = first_ally { wr(ep(t), 0x88, 1i64); wr(ep(t), 0x98, id); } }
            if a.get("tne", 0) == 2 { wr(ep(t), 0x88, 1i64); wr(ep(t), 0x98, champ.id); }
            if a.get("tdmg", -1) >= 0 { unsafe { std::ptr::write(&mut (*(ep(t) as *mut Entity)).attack_effect, Some(mkeff(a.get("tdmg", 0) as usize, 100000, CastingType::Targeting, CastingTarget::Enemy))); } }
            for mn in cache.iter_minions(1 - team) { let mp = ep(mn); wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); }
        }
        6 => { // 적 챔피언 e 를 champ 옆에 두고 action_state=Skill + skill_effect(casting Position, 사거리 큼) → L277 게이트
            let ei = a.get("ei", 0) as usize; let e = cache.player_champion[1 - team][ei].expect("enemy");
            let epp = ep(e);
            wr(epp, 0x660, champ.x + a.get("dx", 10000) as u64); wr(epp, 0x668, champ.y);
            let cst = match a.get("cast", 1) { 0 => CastingType::Targeting, 2 => CastingType::Direction, _ => CastingType::Position };
            unsafe { std::ptr::write(&mut (*(epp as *mut Entity)).skill_effect, Some(mkeff(50, a.get("erange", 200000) as u64, cst, CastingTarget::Enemy))); }
            wr(epp, 0x70, a.get("st", 4) as i64);
            wr(epp, 0x78, a.get("sttime", 100000) as usize);   // action_state.time (nontarget_windup_perceived: time >= react_ticks)
            if a.get("bbvis", 1) == 1 { let bp = &bb[1 - team] as *const Blackboard as *const u8; wr(bp, 0x1e0 + ei * 8, tick); }   // last_visible[ei]+120 >= tick
        }
        7 => { // 적 챔피언들을 내 넥서스 옆에(사거리 큰 평타) → base_defense_focus / threats / Trace
            let n = a.get("n", 2) as usize; let dx = a.get("dx", 15000) as u64;
            for i in 0..n { if let Some(e) = cache.player_champion[1 - team][i] { let epp = ep(e);
                wr(epp, 0x660, nexus0.x + dx * (i as u64 + 1)); wr(epp, 0x668, nexus0.y);
                unsafe { std::ptr::write(&mut (*(epp as *mut Entity)).attack_effect, Some(mkeff(100, a.get("erange", 60000) as u64, CastingType::Targeting, CastingTarget::Enemy))); } } }
            if a.get("ehp", -1) >= 0 { for i in 0..n { if let Some(e) = cache.player_champion[1 - team][i] { wr(ep(e), 0x670, a.get("ehp", 0) as usize); } } }
            if a.get("enoatk", 0) == 1 { for i in 0..n { if let Some(e) = cache.player_champion[1 - team][i] { unsafe { std::ptr::write(&mut (*(ep(e) as *mut Entity)).attack_effect, None); } } } }
        }
        _ => {}
    }
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
    let data = OperationData::new(&cache, &ctx, &bb);
    let seed = a.get("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd2 = rnd.clone();
    let focus0: (u64, u64) = (rd(spp, 0), rd(spp, 8));
    let self_before: [u8; 24] = unsafe { std::ptr::read(spp as *const [u8; 24]) };
    let mut dbg1: DebugFrameData = Default::default();
    let mut dbg2: DebugFrameData = Default::default();
    // ★실행(먼저) → 예측(뒤): TLS 메모(LAST_STAND/DIE_TICK/POS_EVAL)는 두 번째가 hit — 값 동일(무효화 키 seed/tick 고정)
    let real = game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::action_candidates(&mut sp, version, &mut rnd, player, &data, param, &mut dbg1);
    let self_after: [u8; 24] = unsafe { std::ptr::read(spp as *const [u8; 24]) };
    let r1: u64 = rnd.gen();
    let real_raw: Vec<[u8; 184]> = real.iter().map(|e| raw184(e)).collect();
    let pred = predict(version, &mut rnd2, player, &data, param, focus0, &pool2, &mut dbg2);
    let r2: u64 = rnd2.gen();
    println!("self_before={:02x?}", self_before);
    println!("self_after ={:02x?}\tfocus=({},{})\tlast_gate={}", self_after, rd::<u64>(spp, 0), rd::<u64>(spp, 8), rd::<u8>(spp, 16));
    println!("pred_self  \tfocus=({},{})\tlast_gate={}", pred.focus.0, pred.focus.1, pred.last_gate);
    println!("log\t{}", pred.log);
    println!("real_n={}\tpred_n={}\trng_sync={}", real_raw.len(), pred.res.len(), r1 == r2);
    let mut all_ok = real_raw.len() == pred.res.len();
    for i in 0..std::cmp::max(real_raw.len(), pred.res.len()) {
        match (real_raw.get(i), pred.res.get(i)) {
            (Some(r), Some(p)) => {
                let (ok, bad) = cmp_live(r, p);
                let full = cmp_all(r, p);
                let key = format!("start_tick={} f8={} f16={}", u64::from_le_bytes(r[0..8].try_into().unwrap()), u64::from_le_bytes(r[8..16].try_into().unwrap()), u64::from_le_bytes(r[16..24].try_into().unwrap()));
                println!("elem[{}]\treal_tag={}\tpred_tag={}\tsrc={}\tlive={}\tlive_bad={:?}\tall_diff_n={}\tall_diff_first={:?}\t{}", i, tag(r), tag(p), pred.src[i], if ok { "MATCH" } else { "MISMATCH" }, bad, full.len(), full.iter().take(12).collect::<Vec<_>>(), key);
                if !ok || tag(r) != tag(p) { all_ok = false; }
            }
            (Some(r), None) => { println!("elem[{}]\treal_tag={}\tpred=NONE", i, tag(r)); all_ok = false; }
            (None, Some(p)) => { println!("elem[{}]\treal=NONE\tpred_tag={}\tsrc={}", i, tag(p), pred.src[i]); all_ok = false; }
            _ => {}
        }
    }
    let self_ok = (rd::<u64>(spp, 0), rd::<u8>(spp, 16)) == (pred.focus.0, pred.last_gate) && (pred.focus.0 == 0 || rd::<u64>(spp, 8) == pred.focus.1);
    println!("RESULT\tsc={}\telems={}\tself={}\trng_sync={}\t{}", sc, if all_ok { "MATCH" } else { "MISMATCH" }, if self_ok { "MATCH" } else { "MISMATCH" }, r1 == r2, if all_ok && self_ok { "MATCH" } else { "MISMATCH" });
}
