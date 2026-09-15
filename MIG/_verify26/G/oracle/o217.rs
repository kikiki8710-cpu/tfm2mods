#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치G · 217 v17_runaway_counterattack_bonus 오라클 (define hidden → link_name 직접 진입).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame.
//!  독립 재구현: lethal_now 는 expected_damage_target(pub) 로, range 항은 Entity::distance/max_range_cached(pub) 로,
//!  incoming/lowhp/aliies 항은 필드 직독으로 계산. ready_damage(internal)·escape_is_costly(internal) 는 관측 불가 →
//!  escape=0/10 두 가정을 다 내고(pred0/pred1) 어느 쪽에 맞는지 기록. combo/heavy 는 champ.attack_effect 주입으로 유도.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/G/oracle/o217.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::old::BattleSubPlanGoal;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus"]
    fn v17(version: usize, player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter, goal: &BattleSubPlanGoal,
           champ: &Entity, target: &Entity, effect: &Effect) -> i64;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff_atk(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
}

fn d2(a: &Entity, b: &Entity) -> u64 {
    let dx = if a.x < b.x { b.x - a.x } else { a.x - b.x };
    let dy = if a.y < b.y { b.y - a.y } else { a.y - b.y };
    dx * dx + dy * dy
}

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
    let version = a.i("version", 55) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    let tgt = a.s("tgt", "e0");
    let target: &Entity = match tgt.chars().next() {
        Some('e') => cache.player_champion[1][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('a') => cache.player_champion[0][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('t') => cache.top_tower[1].unwrap(),
        _ => panic!("tgt"),
    };
    let tp = ep(target);
    // 내 위치: target 에서 mdist 만큼(x 방향) — 기본 50000
    let mdist = a.i("mdist", 50000);
    if mdist >= 0 { wr(cp, 0x660, target.x + mdist as u64); wr(cp, 0x668, target.y); }
    if a.i("thp", -1) >= 0 { wr(tp, 0x670, a.i("thp", 0) as usize); }
    if a.i("tmax", -1) >= 0 { wr(tp, 0x628, a.i("tmax", 0) as usize); }
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    if a.i("mspd", -1) >= 0 { wr(cp, 0x640, a.i("mspd", 0) as usize); }
    if a.i("undying", 0) == 1 { wr(tp, 0x488, 1u8); }
    // 내 attack_effect 주입(ready_damage 유도)
    if a.i("catk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff_atk(a.i("catk", 0) as usize, a.i("crng", 100000) as u64))); } }
    if a.i("catk", -2) == -2 { /* keep */ }
    // 아군(나 제외) 배치: ally=D → champ 에서 D 만큼
    let ally = a.i("ally", -1);
    let nally = a.i("nally", 4) as usize;
    if ally >= 0 {
        let mut k = 0usize;
        for p in 0..5usize { if p == me { continue; } if k >= nally { break; }
            let e = cache.player_champion[0][p].unwrap(); let eb = ep(e);
            wr(eb, 0x660, champ.x + ally as u64); wr(eb, 0x668, champ.y + (k as u64) * 1000); k += 1; }
    }
    // 적(target 제외) 배치: enemy=D → champ 에서 D 만큼 · vis=1(기본) → bb[1].last_visible[p]=tick
    let enemy = a.i("enemy", -1);
    let nenemy = a.i("nenemy", 4) as usize;
    let vis = a.i("vis", 1);
    let mut k = 0usize;
    for p in 0..5usize {
        let e = cache.player_champion[1][p].unwrap(); let eb = ep(e);
        if vis == 1 { bb[1].last_visible[p] = tick; }
        if e.id == target.id { continue; }
        if enemy >= 0 && k < nenemy { wr(eb, 0x660, champ.x + enemy as u64); wr(eb, 0x668, champ.y + (k as u64) * 1000); k += 1; }
    }
    let effect = mkeff_atk(a.i("adm", 50) as usize, a.i("erng", 100000) as u64);
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let pbase = sp_ptr as *const u8;
    // parameter.player(+0x918): 빈 bumpalo Vec 2개(risk_possible +0x930 / gain_possible +0x950) · 필드 spf
    wr(pbase, 0x918 + 0x18, 8usize); wr(pbase, 0x918 + 0x20, &pool as *const _ as usize); wr(pbase, 0x918 + 0x38, 8usize); wr(pbase, 0x918 + 0x40, &pool as *const _ as usize);
    for kv in a.s("spf", "").split(',').filter(|x| !x.is_empty()) { let (o, v) = kv.split_once(':').unwrap(); wr(pbase, usize::from_str_radix(o.trim_start_matches("0x"), 16).unwrap(), v.parse::<i64>().unwrap()); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let goal = match a.i("goal", 4) {
        0 => BattleSubPlanGoal::Trace { focus: target.id }, 4 => BattleSubPlanGoal::RunAway, _ => BattleSubPlanGoal::End };
    let data = OperationData::new(&cache, &ctx, &bb);

    // ---- 독립 재구현(ready/escape 는 미지수) ----
    let team = player.info.team;
    let et = 1 - team;
    let visible = bb[et].is_recent_visible(&game as &dyn AbstractGame, player, target);
    let gate_ok = matches!(goal, BattleSubPlanGoal::RunAway) && !(target.team == champ.team) && rd::<i64>(tp, 0x68) == 13 && !rd::<bool>(tp, 0x488) && visible;
    let current = effect.expected_damage_target(&ctx, champ as &dyn AbstractEntity, target) as i64;
    let thp = std::cmp::max(target.hp as i64, 1);
    let lethal = current >= thp;
    let dist = target.distance(champ);
    let range = game_ai::plan_legacy::old::max_range_cached(&data, champ, target);
    let spd = rd::<u64>(cp, 0x640);
    let range_term: i64 = if dist <= range + 15000 { 10 } else if dist <= range + spd * 12 { 4 } else { 0 };
    let ppl = rd::<i64>(pbase, 0x988) + rd::<i64>(pbase, 0x998);
    let tpr: &game_ai::ChampionScoreParameter = unsafe { &*(pbase.add(0x918) as *const game_ai::ChampionScoreParameter) };
    let pr = tpr.possible_risk(&data, 90);
    let incoming = ppl + pr + rd::<i64>(pbase, 0x9b0) / 2;
    let my_hp = std::cmp::max(champ.hp as i64, 1);
    let inc_term: i64 = if incoming * 100 >= my_hp * 60 { 10 } else if incoming * 100 >= my_hp * 30 { 5 } else { 0 };
    let tmax = std::cmp::max(rd::<u64>(tp, 0x628), 1);
    let low_term: i64 = if (target.hp as u64) * 100 <= tmax * 35 { 8 } else { 0 };
    let mut na = 0i64; let mut ne = 0i64;
    for p in 0..5usize { if let Some(e) = cache.player_champion[team][p] { if e.id != champ.id && d2(e, champ) < 14400000001 { na += 1; } } }
    for p in 0..5usize { if let Some(e) = cache.player_champion[et][p] { if bb[et].is_recent_visible(&game as &dyn AbstractGame, player, e) && d2(e, champ) < 14400000001 { ne += 1; } } }
    let mut why = format!("gate={} current={} thp={} lethal={} dist={} range={} spd={} range_term={} incoming={} my_hp={} inc_term={} tmax={} low_term={} na={} ne={} ",
        gate_ok, current, thp, lethal, dist, range, spd, range_term, incoming, my_hp, inc_term, tmax, low_term, na, ne);
    // pred(base, escape, combo)
    let pred = |base: i64, esc: i64, combo: bool| -> i64 {
        let mut b = base + esc + range_term + inc_term + low_term;
        if ne > na { if !(lethal || combo || ne < na + 2) { b -= 15; } } else { b += 5; }
        std::cmp::max(b, 0).min(95)
    };
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        v17(version, player, &data, param, &goal, champ, target, &effect)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(v) => {
            let mut verdict = "MISMATCH".to_string();
            if !gate_ok { if v == 0 { verdict = "MATCH(gate0)".into(); } }
            else if lethal {
                let p0 = pred(75, 0, false); let p1 = pred(75, 10, false);
                if v == p0 { verdict = "MATCH(lethal,esc=0)".into(); } else if v == p1 { verdict = "MATCH(lethal,esc=10)".into(); }
                why += &format!("p75e0={} p75e10={} ", p0, p1);
            } else {
                // combo(48)/heavy(14)/0 중 어느 것인지 식별
                let cands = [(48, 0, true), (48, 10, true), (14, 10, false), (0, 0, false)];
                for (b, e, c) in cands { let p = pred(b, e, c); why += &format!("p{}e{}={} ", b, e, p); if v == p && verdict == "MISMATCH" { verdict = format!("MATCH(base={},esc={})", b, e); } }
                if v == 0 { verdict = "MATCH(nonlethal→0)".into(); }
            }
            println!("RESULT\tgot={}\t{}\t{}", v, verdict, why);
        }
    }
    std::process::exit(0);
}
