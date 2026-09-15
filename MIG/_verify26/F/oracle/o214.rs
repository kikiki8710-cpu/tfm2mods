#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치F · 214 BattleSubPlan::score 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v · TLS 메모 함정 ③: 콜리 max_range_cached 가 MAX_RANGE_CACHE 소비자).
//!  세계 = TEMPLATE mkgame(타워 16·챔프 10·미니언 없음). 관측 = score · base(interaction_score 재계산) ·
//!  cas(calculate_action_score 재계산, rnd 순서 동일: interaction_score 뒤) · delta = score - base - cas · 패닉.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/F/oracle/o214.rs
//!  실행: o214.exe act=<stop|runaway|trace|trace_avoid|attack|skill|skill2|ult> tgt=<e0..e4|a0..a4|nexus|tower|jungle|none> ...
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::BattleSubPlan;
use game_ai::plan_legacy::old::{BattleSubPlanGoal, BattleTactic};
use game_ai::{SmallActionPlay, MinionActionType};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, range: u64, casting: CastingType) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting }
}
fn mkstun(duration: u64, range: u64) -> Effect {
    let se = StunEffect { duration };
    Effect { ty: Arc::new(se), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::Skill, casting: CastingType::Targeting }
}
fn mkrush(range: u64) -> Effect {
    let inner: Arc<dyn EffectType> = Arc::new(AttackEffect { ty: AttackEffectType::EnemyTarget, damage: 10, attack_ratio: 0, hp_ratio: 0, target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false });
    // ★RushEffect::expected_rush_effect = speed!=0 && applyed_effect.len()!=0 (g08.ll:108954) — 빈 Vec 이면 false
    let re = RushEffect { applyed_effect: vec![(inner, CastingType::Targeting)], speed: 1000, move_speed_ratio: 0, range, casting_target: CastingTarget::Enemy, penetrate: false };
    Effect { ty: Arc::new(re), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::Skill, casting: CastingType::Position }
}

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
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
    let run = a.i("run", 0) as usize;
    if run > 0 {
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        for _ in 0..run { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut r, &mut fd); }
    }
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
    // 내 챔피언: 위치·레벨·이펙트
    if a.i("mx", -1) >= 0 { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    if a.i("lv", -1) >= 0 { wr(cp, 0x5c8, a.i("lv", 0) as usize); }
    let adm = a.i("adm", 50) as usize; let arng = a.i("arng", 100000) as u64;
    match a.i("atk", 1) {
        1 => unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(adm, arng, CastingType::Targeting))); },
        0 => unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); },
        _ => {}
    }
    // skill: sk=atk(공격) | stun | rush | none
    let sdm = a.i("sdm", 60) as usize; let srng = a.i("srng", 120000) as u64; let stun = a.i("stun", 120) as u64;
    let skf = |kind: &str| -> Option<Effect> { match kind { "atk" => Some(mkeff(sdm, srng, CastingType::Targeting)), "stun" => Some(mkstun(stun, srng)), "rush" => Some(mkrush(srng)), _ => None } };
    let sk = a.s("sk", "atk"); let sk2 = a.s("sk2", "atk"); let ul = a.s("ul", "atk");
    unsafe {
        std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, skf(&sk));
        std::ptr::write(&mut (*(cp as *mut Entity)).skill2_effect, skf(&sk2));
        let udm = a.i("udm", 100) as usize; let urng = a.i("urng", 150000) as u64;
        let ue = match ul.as_str() { "atk" => Some(mkeff(udm, urng, CastingType::Targeting)), "stun" => Some(mkstun(stun, urng)), "rush" => Some(mkrush(urng)), _ => None };
        std::ptr::write(&mut (*(cp as *mut Entity)).ult_effect, ue);
    }
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    // 적 챔피언: near=D 이면 내 위치 + D + p*1000 (x), vis=1 이면 블랙보드 가시, evis=1 이면 visible_state
    let near = a.i("near", -1);
    for p in 0..5usize {
        let e = cache.player_champion[1][p].expect("enemy");
        let eb = ep(e);
        if near >= 0 {
            let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
            wr(eb, 0x660, mx + near as u64 + (p as u64) * 1000); wr(eb, 0x668, my);
        }
        if a.i("vis", 0) == 1 { bb[1].last_visible[p] = tick; }
        if a.i("evis", 0) == 1 { wr(eb, 0x38, 0i64); }
        if a.i("ehp", -1) >= 0 { wr(eb, 0x670, a.i("ehp", 0) as usize); }
        if a.i("emax", -1) >= 0 { wr(eb, 0x628, a.i("emax", 0) as usize); }
        if a.i("elv", -1) >= 0 { wr(eb, 0x5c8, a.i("elv", 0) as usize); }
    }
    // e0 만 좌표 지정
    if a.i("e0x", -1) >= 0 { let e = cache.player_champion[1][0].unwrap(); wr(ep(e), 0x660, a.i("e0x", 0) as u64); wr(ep(e), 0x668, a.i("e0y", 0) as u64); }
    let id_of = |s: &str| -> usize {
        match s {
            "nexus" => cache.nexus[1].unwrap().id,
            "mynexus" => cache.nexus[0].unwrap().id,
            "tower" => cache.top_tower[1].unwrap().id,
            "tower2" => cache.top_tower2[1].unwrap().id,
            "mytower" => cache.top_tower[0].unwrap().id,
            "jungle" => cache.jungles.get(a.i("jidx", 0) as usize).map(|e| e.id).unwrap_or(999999),
            "epic" | "serpen" => { let want: i64 = if s == "epic" { 5 } else { 6 }; let mut f = 999999usize; for id in 0..4000usize { if let Some(e) = game.get_entity_by_id(id) { if rd::<i64>(ep(e), 0x68) == want { f = id; break; } } } f },
            "none" => 999999usize,
            _ => match s.chars().next() {
                Some('e') => cache.player_champion[1][s[1..].parse::<usize>().unwrap()].unwrap().id,
                Some('a') => cache.player_champion[0][s[1..].parse::<usize>().unwrap()].unwrap().id,
                _ => 999999usize,
            }
        }
    };
    let goal = match a.i("goal", 0) {
        0 => BattleSubPlanGoal::Trace { focus: id_of("e0") }, 1 => BattleSubPlanGoal::Protect { focus: id_of("a1") }, 2 => BattleSubPlanGoal::Kiting { focus: id_of("e0") },
        3 => BattleSubPlanGoal::KitingBack { focus: id_of("e0") }, 4 => BattleSubPlanGoal::RunAway, 5 => BattleSubPlanGoal::Assassin { focus: id_of("e0") },
        6 => BattleSubPlanGoal::AssassinReady { focus: id_of("e0") }, _ => BattleSubPlanGoal::End };
    let sup = a.s("sup", "none");
    let support: Option<usize> = if sup == "none" { None } else { Some(id_of(&sup)) };
    let tactic = match a.i("tactic", 0) { 1 => BattleTactic::Frontline, 2 => BattleTactic::BacklineDPS, 3 => BattleTactic::SkillBurst, 4 => BattleTactic::Peel, 5 => BattleTactic::AllIn, 6 => BattleTactic::Disengage, _ => BattleTactic::Standard };
    let sp = BattleSubPlan::new(goal, support, tactic, a.i("avoid", 0) == 1, a.i("hold", 0) as usize, a.i("dive", 0) == 1);
    let spp = &sp as *const BattleSubPlan as *const u8;
    let before: [u8; 48] = unsafe { std::ptr::read(spp as *const [u8; 48]) };
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let seed = a.i("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let rnd0 = rnd.clone();
    // 액션 조립
    let tgt = a.s("tgt", "e0");
    let tid = id_of(&tgt);
    let act = a.s("act", "stop");
    let action: SmallActionPlay = match act.as_str() {
        "stop" => SmallActionPlay::Stop,
        "runaway" => SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)),
        "recall" => SmallActionPlay::Recall(game_ai::SmallActionRecall::new(&data, player, 0)),
        "trace" => SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, tid, 0)),
        "trace_avoid" => SmallActionPlay::Trace(game_ai::SmallActionTrace::new_avoid_tower(&data, tid, 0)),
        "attack" => SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, tid)),
        "skill" => SmallActionPlay::Skill(game_ai::SmallActionSkill::new(&data, tid)),
        "skill2" => SmallActionPlay::Skill2(game_ai::SmallActionSkill2::new(&data, tid)),
        "ult" => SmallActionPlay::Ult(game_ai::SmallActionUlt::new(&data, tid)),
        _ => SmallActionPlay::Stop,
    };
    let ab = &action as *const SmallActionPlay as *const u8;
    let atag: u8 = rd(ab, 0xb1);
    // 세계 요약
    let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
    let tent = game.get_entity_by_id(tid);
    if let Some(t) = tent { let tb = ep(t); if a.i("thp", -1) >= 0 { wr(tb, 0x670, a.i("thp", 0) as usize); } if a.i("tmax", -1) >= 0 { wr(tb, 0x628, a.i("tmax", 0) as usize); }
        if a.i("tx", -1) >= 0 { wr(tb, 0x660, a.i("tx", 0) as u64); wr(tb, 0x668, a.i("ty", 0) as u64); } }
    let (tx, ty, thp, tmax, ttag) = match tent { Some(t) => (t.x, t.y, t.hp, t.stat_cached.hp, rd::<i64>(ep(t), 0x68)), None => (0, 0, 0, 0, -1) };
    let dist = game_core::utils::distance(mx, my, tx, ty);
    println!("world\ttick={}\tme=({},{})\tlv={}\thp={}\ttgt={}(id={})\tt=({},{})\tt_hp={}/{}\tt_tytag={}\tdist={}\tatag={}\tsup={:?}\tgoal_tag={}\tavoid={}",
        tick, mx, my, champ.level, champ.hp, tgt, tid, tx, ty, thp, tmax, ttag, dist, atag, support, rd::<i64>(spp, 0x10), rd::<u8>(spp, 0x28));
    println!("effects\tatk={:?}\tsk={}\tsk2={}\tul={}\tjungles={}", champ.attack_effect.as_ref().map(|e| e.range), sk, sk2, ul, cache.jungles.len());
    if a.i("scan", 0) == 1 {
        for e in cache.jungles.iter() { println!("jungle\tid={}\tty={}\tcamp={}\tpos=({},{})\thp={}/{}", e.id, rd::<i64>(ep(*e), 0x68), rd::<usize>(ep(*e), 0x98), e.x, e.y, e.hp, e.stat_cached.hp); }
        for id in 0..4000usize { if let Some(e) = game.get_entity_by_id(id) { let t: i64 = rd(ep(e), 0x68); if t != 13 && t != 2 && t != 4 { println!("ent\tid={}\tty={}\tpos=({},{})\thp={}/{}", id, t, e.x, e.y, e.hp, e.stat_cached.hp); } } }
    }
    // 보조 관측: max_range_cached(+25000) · can_tower_focused(me/target)
    if let Some(t) = tent {
        let mr = game_ai::plan_legacy::old::max_range_cached(&data, champ, t) + 25000;
        let dsq = t.distance_sq(champ);
        println!("aux\tmr={}\tdist_sq={}\tmr_sq={}\tfar={}\tctf_me={}\tctf_t={}", mr, dsq, mr * mr, dsq > mr * mr,
            game_ai::can_tower_focused(&ctx, &cache, player, champ.x, champ.y), game_ai::can_tower_focused(&ctx, &cache, player, t.x, t.y));
        if a.i("ctwt", 0) == 1 { println!("aux2\tctwt={}", game_ai::can_trace_without_tower(&ctx, &cache, player.info.id, t.x, t.y, mr)); }
        if let Some(ue) = champ.ult_effect.as_ref() { println!("aux3\tult_dmg={}\tt_hp={}\tcc={:?}", ue.expected_damage_target(&ctx, champ as &dyn AbstractEntity, t), t.hp, game_ai::effect_cc_time(version, ue)); }
        if let Some(se) = champ.skill_effect.as_ref() { println!("aux4\tskill_cc={:?}\tskill_dmg={}", game_ai::effect_cc_time(version, se), se.expected_damage_target(&ctx, champ as &dyn AbstractEntity, t)); }
        if let Some(ae) = champ.attack_effect.as_ref() { println!("aux5\tatk_dmg={}", ae.expected_damage_target(&ctx, champ as &dyn AbstractEntity, t)); }
    }
    // 기준값 재계산(같은 rnd 순서): base = interaction_score · cas = calculate_action_score(액션별)
    let mut rnd_b = rnd0.clone();
    let mut dbg_b: DebugFrameData = Default::default();
    let pre = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| game_ai::interaction_score(version, &mut rnd_b, player, &data, param, &action, &mut dbg_b)));
    let base = match pre { Ok(v) => v, Err(e) => { let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into()); println!("PRE_PANIC\tinteraction_score\t{}", msg.replace('\n', " ")); i64::MIN } };
    let mut cas: i64 = 0; let mut cas_ok = false;
    if let (Some(t), true) = (tent, base != i64::MIN) {
        match act.as_str() {
            "attack" => if let Some(ae) = champ.attack_effect.as_ref() { cas = game_ai::calculate_action_score(version, &mut rnd_b, player, &data, param, &champ.attack, ae, champ.attack_speed_mult(), t, MinionActionType::Normal, &mut dbg_b); cas_ok = true; },
            "skill" => if let Some(se) = champ.skill_effect.as_ref() { cas = game_ai::calculate_action_score(version, &mut rnd_b, player, &data, param, &champ.skill, se, champ.cooldown_reduce(false), t, MinionActionType::Normal, &mut dbg_b); cas_ok = true; },
            "skill2" => if let Some(se) = champ.skill2_effect.as_ref() { cas = game_ai::calculate_action_score(version, &mut rnd_b, player, &data, param, &champ.skill2, se, champ.cooldown_reduce(false), t, MinionActionType::Normal, &mut dbg_b); cas_ok = true; },
            "ult" => if let Some(ue) = champ.ult_effect.as_ref() { cas = game_ai::calculate_action_score(version, &mut rnd_b, player, &data, param, champ.ult(), ue, champ.cooldown_reduce(true), t, MinionActionType::Normal, &mut dbg_b); cas_ok = true; },
            _ => {}
        }
    }
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sp.score(version, param, &mut rnd, player, &data, &action, &mut dbgf)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(score) => {
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen(); let rb: u64 = rnd_b.clone().gen();
            println!("RESULT\tscore={}\tbase={}\tcas={}\tcas_ok={}\tdelta={}\trnd_used={}\trnd_same_as_recalc={}", score, base, cas, cas_ok, score - base - cas, r1 != r0, r1 == rb);
        }
    }
    let after: [u8; 48] = unsafe { std::ptr::read(spp as *const [u8; 48]) };
    let diff: Vec<String> = (0..48).filter(|&i| before[i] != after[i]).map(|i| format!("+{:#x}:{}->{}", i, before[i], after[i])).collect();
    println!("self_diff\t{}", diff.join(" "));
}
