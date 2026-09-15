#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치G · 215 v16_gambler_ult_cc_bonus 오라클 (define hidden → link_name 직접 진입).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10, 미니언 없음).
//!  독립 재구현(pub 콜리 champion_hp_value/expected_damage_target/possible_risk/is_recent_visible 로 항 계산) ↔ 실제 반환 대조.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/G/oracle/o215.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::old::BattleSubPlanGoal;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common24v16_gambler_ult_cc_bonus"]
    fn v16(player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter, goal: &BattleSubPlanGoal,
           support: Option<usize>, action: &Box<dyn Action>, effect: &Effect, champ: &Entity, target: &Entity) -> i64;
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
fn mkeff_stun(dur: u64, range: u64) -> Effect {
    Effect { ty: Arc::new(StunEffect { duration: dur }), range, growth_range: 0, start_timing: 10,
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    // target 선택
    let tgt = a.s("tgt", "e0");
    let target: &Entity = match tgt.chars().next() {
        Some('e') => cache.player_champion[1][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('a') => cache.player_champion[0][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('t') => cache.top_tower[1].unwrap(),
        _ => panic!("tgt"),
    };
    let tp = ep(target);
    // 내 위치를 target 근처로(거리 mdist) — 기본은 원래 위치
    if a.i("mdist", -1) >= 0 { wr(cp, 0x660, target.x + a.i("mdist", 0) as u64); wr(cp, 0x668, target.y); }
    if a.i("thp", -1) >= 0 { wr(tp, 0x670, a.i("thp", 0) as usize); }
    if a.i("immune", 0) == 1 { wr(tp, 0x468, 1u8); }
    match a.s("cc", "none").as_str() {
        "stun" => unsafe { std::ptr::write(&mut (*(tp as *mut Entity)).cc, vec![CCState::Stun { tick: 100 }]); },
        "block" => unsafe { std::ptr::write(&mut (*(tp as *mut Entity)).cc, vec![CCState::BlockAttack { tick: 100 }]); },
        "charm" => unsafe { std::ptr::write(&mut (*(tp as *mut Entity)).cc, vec![CCState::BlockSkill { tick: 100 }, CCState::Charm { tick: 100, dx: 0, dy: 0 }]); },
        _ => {}
    }
    // 아군(나 제외) 배치: ally=D → target 에서 D 만큼(각자 y 로 1000 씩 어긋나게)
    let ally = a.i("ally", -1);
    let nally = a.i("nally", 4) as usize;
    if ally >= 0 {
        let mut k = 0usize;
        for p in 0..5usize { if p == me { continue; } if k >= nally { break; }
            let e = cache.player_champion[0][p].unwrap(); let eb = ep(e);
            wr(eb, 0x660, target.x + ally as u64); wr(eb, 0x668, target.y + (k as u64) * 1000); k += 1; }
    }
    // 적(target 제외) 배치: enemy=D · vis=1 → bb[1].last_visible[p]=tick
    let enemy = a.i("enemy", -1);
    let nenemy = a.i("nenemy", 4) as usize;
    let vis = a.i("vis", 0);
    let mut k = 0usize;
    for p in 0..5usize {
        let e = cache.player_champion[1][p].unwrap(); let eb = ep(e);
        if vis == 1 { bb[1].last_visible[p] = tick; }
        if e.id == target.id { continue; }
        if enemy >= 0 && k < nenemy { wr(eb, 0x660, target.x + enemy as u64); wr(eb, 0x668, target.y + (k as u64) * 1000); k += 1; }
    }
    // 액션 = GamblerUltAction(charm_duration) · 이펙트
    let charm = a.i("charm", 60) as usize;
    let action: Box<dyn Action> = Box::new(GamblerUltAction { attack: 100, attack_ratio: 0, charm_duration: charm, range: 100000,
        attack_range: 100000, speed: 1000, cooltime: 600, duration: 30, start_timing: 10, cancelable: false });
    let other_action: Box<dyn Action> = SwordmanChampionInfo::default().ult();
    let use_other = a.i("otheract", 0) == 1;
    let act_ref: &Box<dyn Action> = if use_other { &other_action } else { &action };
    let eff = a.s("eff", "atk");
    let (effect, cc_exp): (Effect, Option<usize>) = if let Some(d) = eff.strip_prefix("stun:") {
        let d: u64 = d.parse().unwrap(); (mkeff_stun(d, 100000), Some(d as usize))
    } else { (mkeff_atk(a.i("adm", 50) as usize, 100000), None) };
    // ScoreParameter: zeroed + positioning_score default (+ np=1 이면 near_enemies 에 target 파라미터 1개)
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let pbase = sp_ptr as *const u8;
    let mut tpbuf = [0u8; 216];
    let np = a.i("np", 0) == 1;
    if np {
        let tb = tpbuf.as_ptr();
        wr(tb, 0x58, target.id); wr(tb, 0x60, 1usize); wr(tb, 0x68, tgt[1..].parse::<usize>().unwrap_or(0));
        wr(tb, 0x70, a.i("ad", 0) as usize); wr(tb, 0x80, a.i("rd", 0) as usize);
        for kv in a.s("tpf", "").split(',').filter(|x| !x.is_empty()) { let (o, v) = kv.split_once(':').unwrap(); wr(tb, usize::from_str_radix(o.trim_start_matches("0x"), 16).unwrap(), v.parse::<i64>().unwrap()); }
        for kv in a.s("spf", "").split(',').filter(|x| !x.is_empty()) { let (o, v) = kv.split_once(':').unwrap(); wr(pbase, usize::from_str_radix(o.trim_start_matches("0x"), 16).unwrap(), v.parse::<i64>().unwrap()); }
        // risk_possible / gain_possible: 빈 bumpalo Vec (ptr=8 dangling, bump=&pool, cap=0, len=0)
        wr(tb, 0x18, 8usize); wr(tb, 0x20, &pool as *const _ as usize); wr(tb, 0x38, 8usize); wr(tb, 0x40, &pool as *const _ as usize);
        wr(pbase, 0x14d8, tb as usize); wr(pbase, 0x14e0, &pool as *const _ as usize); wr(pbase, 0x14e8, 1usize); wr(pbase, 0x14f0, 1usize);
    }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let id_of = |s: &str| -> usize {
        match s.chars().next() {
            Some('e') => cache.player_champion[1][s[1..].parse::<usize>().unwrap()].unwrap().id,
            Some('a') => cache.player_champion[0][s[1..].parse::<usize>().unwrap()].unwrap().id,
            Some('t') => cache.top_tower[1].unwrap().id,
            _ => 999999usize,
        }
    };
    let focus = id_of(&a.s("focus", "e0"));
    let goal = match a.i("goal", 0) {
        0 => BattleSubPlanGoal::Trace { focus }, 1 => BattleSubPlanGoal::Protect { focus }, 2 => BattleSubPlanGoal::Kiting { focus },
        3 => BattleSubPlanGoal::KitingBack { focus }, 4 => BattleSubPlanGoal::RunAway, 5 => BattleSubPlanGoal::Assassin { focus },
        6 => BattleSubPlanGoal::AssassinReady { focus }, _ => BattleSubPlanGoal::End };
    let sup = a.s("sup", "none");
    let support: Option<usize> = if sup == "none" { None } else { Some(id_of(&sup)) };
    let data = OperationData::new(&cache, &ctx, &bb);

    // ---- 독립 재구현(명세 logic 그대로) ----
    let mut pred: i64 = 0;
    let mut why = String::new();
    let cc_time: usize = match cc_exp { Some(t) => std::cmp::max(t, charm), None => charm };
    let gate_ok = !use_other && !(target.team == champ.team) && (rd::<i64>(tp, 0x68) == 13) && !rd::<bool>(tp, 0x468)
        && !target.cc.iter().any(|c| matches!(c, CCState::Airborne{..}|CCState::Stun{..}|CCState::Bind{..}|CCState::ForceMove{..}|CCState::Fear{..}|CCState::Charm{..}))
        && cc_time != 0;
    if gate_ok {
        let mut bonus: i64 = if (cc_time as i64) < 36 { 12 } else { std::cmp::min(cc_time as i64 / 3, 30) };
        why += &format!("base={} ", bonus);
        if np {
            let tpr: &game_ai::ChampionScoreParameter = unsafe { &*(tpbuf.as_ptr() as *const game_ai::ChampionScoreParameter) };
            let hv = std::cmp::min(game_ai::champion_hp_value(&data, param, tpr), 100);
            let ult = effect.expected_damage_target(&ctx, champ as &dyn AbstractEntity, target) as i64;
            let pr = tpr.possible_risk(&data, cc_time + 30);
            let incoming = a.i("ad", 0) + a.i("rd", 0) + pr;
            let thp = std::cmp::max(target.hp as i64, 1);
            bonus += std::cmp::min(ult * hv / thp, 35);
            bonus += hv / 6;
            if incoming > 0 { bonus += std::cmp::min(incoming * hv / thp, 22); }
            why += &format!("hv={} ult={} pr={} incoming={} thp={} bonus_after_np={} ", hv, ult, pr, incoming, thp, bonus);
        } else { bonus += 6; }
        let team = player.info.team;
        let mut ally_near = 0i64;
        for p in 0..5usize { if let Some(e) = cache.player_champion[team][p] { if e.id != champ.id && d2(e, target) < 14400000001 { ally_near += 1; } } }
        let et = 1 - team;
        let mut enemy_near = 0i64;
        for p in 0..5usize { if let Some(e) = cache.player_champion[et][p] {
            if e.id != target.id && bb[et].is_recent_visible(&game as &dyn AbstractGame, player, e) && d2(e, target) < 8100000001 { enemy_near += 1; } } }
        bonus += std::cmp::min(ally_near, 3) * 5;
        bonus += std::cmp::min(enemy_near, 2) * 4;
        let fr = match goal.focus() {
            None => false,
            Some(f) => f == target.id || game.get_entity_by_id(f).map_or(false, |fe| fe.team == champ.team && d2(fe, target) < 8100000001),
        };
        if fr || support == Some(target.id) { bonus += 12; }
        why += &format!("ally_near={} enemy_near={} focus_rel={} sup={} pre_clamp={} ", ally_near, enemy_near, fr, support == Some(target.id), bonus);
        pred = std::cmp::min(bonus, 70);
    } else { why += "gate->0 "; }

    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        v16(player, &data, param, &goal, support, act_ref, &effect, champ, target)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(v) => {
            println!("RESULT\tgot={}\tpred={}\t{}\tcc_time={}\t{}", v, pred, if v == pred { "MATCH" } else { "MISMATCH" }, cc_time, why);
        }
    }
    std::process::exit(0);
}
