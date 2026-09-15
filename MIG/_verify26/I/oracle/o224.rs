#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치I · 224 v21_defensive_cc_score 오라클.
//!  대상은 `internal fastcc` 라 심볼이 없다 → **직접 호출자** `v21_runaway_defensive_cc_bonus`(define hidden · m05.ll:58188 · goal 태그==4(RunAway) 이면
//!  v21_defensive_cc_score 를 그대로 tail-call, 아니면 0) 를 `#[link_name]` 으로 링크해 진입한다(METHOD_MAP ⑥ · 22차 C·D 수법).
//!  mode=score: 래퍼 호출 / mode=expect: 명세 logic 을 콜리 계약(pub)만 써서 독립 재구현(수법 ⓓ) — 케이스당 프로세스 2개(TLS MAX_RANGE_CACHE).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/I/oracle/o224.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::old::{BattleSubPlanGoal, max_range_cached};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common30v21_runaway_defensive_cc_bonus"]
    fn v21_runaway_defensive_cc_bonus(version: usize, player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter,
        goal: &BattleSubPlanGoal, champ: &Entity, target: &Entity, effect: &Effect, cc_time: Option<usize>) -> i64;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, range: u64, growth: u64, ct: CastingTarget) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: growth, start_timing: 10,
        target: ct, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
}
fn radius_of(e: &Entity) -> u64 {
    let mult: i32 = rd(ep(e), 0x470); let r: u64 = rd(ep(e), 0x680);
    if mult == 0 { r } else { r * ((mult as i64 + 100) as u64) / 100 }
}
fn d2(a: &Entity, b: &Entity) -> u64 { let dx = a.x.abs_diff(b.x); let dy = a.y.abs_diff(b.y); dx * dx + dy * dy }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::check_kill_die_tick as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.to_string()); } }
    let a = Args { m };
    let setting = real_setting();
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
    let tick = a.i("tick", 3000) as usize;
    game.set_tick(tick);
    let version = a.i("ver", 55) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = 0usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    // 내 챔피언을 맵 중앙에
    wr(cp, 0x660, 480000u64); wr(cp, 0x668, 480000u64);
    if a.i("hp", -1) >= 0 { wr(cp, 0x670, a.i("hp", 0) as usize); }
    if a.i("maxhp", -1) >= 0 { wr(cp, 0x628, a.i("maxhp", 0) as usize); }
    if a.i("cms", -1) >= 0 { wr(cp, 0x640, a.i("cms", 0) as usize); }
    if a.i("clvl", -1) >= 0 { wr(cp, 0x5c8, a.i("clvl", 1) as usize); }
    if a.i("cbr", -1) >= 0 { wr(cp, 0x438, a.i("cbr", 0) as u64); }
    // 대상: 기본 e0(적) · same=1 이면 a1(아군)
    let target: &Entity = if a.i("same", 0) == 1 { cache.player_champion[0][1].unwrap() } else { cache.player_champion[1][0].unwrap() };
    let tp = ep(target);
    let D = a.i("D", 60000) as u64;
    wr(tp, 0x660, 480000u64 + D); wr(tp, 0x668, 480000u64);
    if a.i("tms", -1) >= 0 { wr(tp, 0x640, a.i("tms", 0) as usize); }
    if a.i("tty", -1) >= 0 { wr(tp, 0x68, a.i("tty", 13) as i64); }
    if a.i("imm", 0) == 1 { wr(tp, 0x468, 1u8); }
    let ccst = a.s("ccst", "none");
    unsafe {
        let tm = tp as *mut Entity;
        match ccst.as_str() {
            "hard" => { (*tm).cc.push(CCState::Airborne { tick: 100 }); }
            "soft" => { (*tm).cc.push(CCState::BlockAttack { tick: 100 }); }
            "both" => { (*tm).cc.push(CCState::BlockAttack { tick: 100 }); (*tm).cc.push(CCState::Stun { tick: 100 }); }
            _ => {}
        }
    }
    // 가시성: 적 팀 블랙보드 last_visible[대상 pos]
    if a.i("vis", 1) == 1 { bb[1].last_visible[0] = tick; }
    // 다른 적/아군을 내 근처에(near_enemies/near_allies) — 적은 가시 처리
    let ne_n = a.i("ne", 0) as usize; let na_n = a.i("na", 0) as usize;
    for p in 1..5usize {
        let e = cache.player_champion[1][p].unwrap();
        if p <= ne_n { wr(ep(e), 0x660, 480000u64 + 30000 + p as u64 * 1000); wr(ep(e), 0x668, 480000u64); bb[1].last_visible[p] = tick; }
        else { wr(ep(e), 0x660, 1u64); wr(ep(e), 0x668, 1u64); }
        let al = cache.player_champion[0][p].unwrap();
        if p <= na_n { wr(ep(al), 0x660, 480000u64 - 30000 - p as u64 * 1000); wr(ep(al), 0x668, 480000u64); }
        else { wr(ep(al), 0x660, 1u64); wr(ep(al), 0x668, 900000u64); }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let ct = match a.s("ct", "enemy").as_str() { "ally" => CastingTarget::Ally, _ => CastingTarget::Enemy };
    let effect = mkeff(80, a.i("erng", 100000) as u64, a.i("egrowth", 0) as u64, ct);
    let cc_time: Option<usize> = match a.s("cc", "90").as_str() { "none" => None, s => Some(s.parse::<usize>().unwrap()) };
    let goal = if a.i("goal", 4) == 4 { BattleSubPlanGoal::RunAway } else { BattleSubPlanGoal::End };
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let pbase = sp_ptr as *const u8;
    if a.i("adm", -1) >= 0 { wr(pbase, 0x988, a.i("adm", 0) as i64); }
    if a.i("rdm", -1) >= 0 { wr(pbase, 0x998, a.i("rdm", 0) as i64); }
    if a.i("rtw", -1) >= 0 { wr(pbase, 0x9b0, a.i("rtw", 0) as i64); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let mode = a.s("mode", "score");
    let game_dyn: &dyn AbstractGame = data.cache.game;
    if mode == "expect" {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> (i64, String) {
            let mut trace = String::new();
            let cc = match cc_time { Some(t) if t != 0 => t, _ => return (0, "cc".into()) };
            if target.team == champ.team { return (0, "team".into()); }
            if rd::<i64>(tp, 0x68) != 13 { return (0, "ty".into()); }
            if rd::<u8>(tp, 0x468) != 0 { return (0, "immune".into()); }
            if target.cc.iter().any(|c| c.is_cc()) { return (0, "cc_state".into()); }
            let et = 1 - player.info.team;
            if !bb[et].is_recent_visible(game_dyn, player, target) { return (0, "vis".into()); }
            if !data.can_target(game_dyn, player, target) { return (0, "can_target".into()); }
            if game_ai::is_enemy_well_danger(version, player, target.x, target.y) { return (0, "well".into()); }
            if effect.ty.expected_rush_effect() { return (0, "rush".into()); }
            if effect.ty.expected_move_on_hit() { return (0, "moveonhit".into()); }
            if effect.ty.expected_move_distance().is_some() { return (0, "movedist".into()); }
            if !effect.target.check(champ, target) { return (0, "ct".into()); }
            let cast_range = effect.range(champ) + effect.range_adjust(champ, target) + radius_of(champ) + radius_of(target);
            let dist = target.distance(champ);
            let cms: usize = rd(cp, 0x640);
            if dist > cast_range + 10000 + (cms as u64) * 8 { return (0, format!("far dist={} cast={}", dist, cast_range)); }
            let adm: i64 = rd(pbase, 0x988); let rdm: i64 = rd(pbase, 0x998); let rtw: i64 = rd(pbase, 0x9b0);
            let pr = param.player.possible_risk(&data, 90);
            let incoming = adm + rdm + pr + rtw / 2;
            let chp: usize = rd(cp, 0x670); let cmax: usize = rd(cp, 0x628);
            let hp = std::cmp::max(chp, 1) as i64;
            let hp_ratio = (chp as u64) * 100 / (std::cmp::max(cmax, 1) as u64);
            let na = cache.player_champion[player.info.team].iter().flatten().filter(|x| x.id != champ.id && d2(x, champ) < 14400000001).count() + 1;
            let ne = cache.player_champion[et].iter().flatten().filter(|e| {
                bb[et].is_recent_visible(game_dyn, player, e) && { let thr = max_range_cached(&data, e, champ) + 40000; let dd = d2(e, champ); dd < 14400000001 || dd <= thr * thr }
            }).count();
            let tms: usize = rd(tp, 0x640);
            let catch_range = max_range_cached(&data, target, champ) + 25000 + (tms as u64) * 18;
            let direct_threat = d2(target, champ) <= catch_range * catch_range;
            trace = format!("dist={} cast={} incoming={} hp={} hp_ratio={} na={} ne={} catch={} direct={}", dist, cast_range, incoming, hp, hp_ratio, na, ne, catch_range, direct_threat);
            if !direct_threat && incoming * 100 < hp * 20 && ne <= na { return (0, format!("gate597 {}", trace)); }
            let mut bonus = ((cc as i64) / 3).clamp(10, 45);
            bonus += if dist > cast_range { 16 } else { 35 };
            if direct_threat { bonus += 22; }
            if incoming * 100 >= hp * 35 { bonus += 24; } else if incoming * 100 >= hp * 15 { bonus += 10; }
            if ne > na { bonus += std::cmp::min(((ne - na) as i64) * 8, 24); }
            bonus += if hp_ratio < 45 { 14 } else if hp_ratio < 65 { 6 } else { 0 };
            let itr = max_range_cached(&data, target, champ) + 30000;
            if d2(target, champ) <= itr * itr { bonus += 12; }
            (std::cmp::min(bonus, 160), trace)
        }));
        match r {
            Ok((v, t)) => println!("RESULT\tmode=expect\tval={}\t{}", v, t),
            Err(_) => println!("RESULT\tmode=expect\tPANIC"),
        }
    } else {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            v21_runaway_defensive_cc_bonus(version, player, &data, param, &goal, champ, target, &effect, cc_time)
        }));
        match r {
            Ok(v) => println!("RESULT\tmode=score\tval={}", v),
            Err(_) => println!("RESULT\tmode=score\tPANIC"),
        }
    }
}
