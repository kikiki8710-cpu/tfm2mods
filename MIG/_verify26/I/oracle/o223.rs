#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치I · 223 DefenseNexusSubPlan::score 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10·미니언 없음).
//!  관측: score() 반환 ↔ [interaction_score + 액션별 가산] 을 같은 rnd 순서로 따로 계산한 기대값 대조,
//!        rnd 320B 전후 비트 비교(콜리 호출 순서 = interaction_score → calculate_action_score), self 24B 불변, 패닉.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/I/oracle/o223.rs
//!  실행: o223.exe act=attack|skill|skill2|around tgt=e0|e1|none base=0|1 minion=0|1 front=none|e0|e0,e1 near=0|1 ver=N lvl=N aeff=0|1 seff=0|1 s2eff=0|1
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan;
use game_ai::plan_legacy::old::is_base_attacking_minion;
use game_ai::{SmallActionPlay, SmallActionAttack, SmallActionAround, MinionActionType};

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

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
}

fn rndbytes(r: &rand::rngs::StdRng) -> [u8; 320] { unsafe { std::ptr::read(r as *const rand::rngs::StdRng as *const [u8; 320]) } }

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
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    let nexus0 = cache.nexus[0].expect("nexus0");
    let nx: u64 = rd(ep(nexus0), 0x660); let ny: u64 = rd(ep(nexus0), 0x668);
    // 내 챔피언 이펙트/레벨
    if a.i("aeff", 1) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.i("adm", 50) as usize, 100000, CastingType::Targeting))); } }
    else { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); } }
    if a.i("seff", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, Some(mkeff(80, 120000, CastingType::Targeting))); } }
    if a.i("seff", 0) == 2 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, None); } }
    if a.i("s2eff", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill2_effect, Some(mkeff(90, 130000, CastingType::Targeting))); } }
    if a.i("s2eff", 0) == 2 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill2_effect, None); } }
    if a.i("ueff", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).ult_effect, Some(mkeff(200, 150000, CastingType::Targeting))); } }
    println!("champ_effects	attack={}	skill={}	skill2={}	ult={}", champ.attack_effect.is_some(), champ.skill_effect.is_some(), champ.skill2_effect.is_some(), champ.ult_effect.is_some());
    if a.i("lvl", -1) >= 0 { wr(cp, 0x5c8, a.i("lvl", 1) as usize); }
    // 적 e0/e1 세팅
    let e0 = cache.player_champion[1][0].expect("e0"); let e1 = cache.player_champion[1][1].expect("e1");
    if a.i("near", 0) == 1 {
        wr(ep(e0), 0x660, nx + 1000); wr(ep(e0), 0x668, ny);
        wr(ep(e1), 0x660, nx + 50000); wr(ep(e1), 0x668, ny);
    }
    if a.i("minion", 0) == 1 { wr(ep(e0), 0x68, 1i64); wr(ep(e1), 0x68, 1i64); }
    if a.i("base", 0) == 1 {
        // e0 를 본진(내 넥서스) 타격 미니언으로: ty=Minion · nearest_enemy=Some(nexus[team].id)
        wr(ep(e0), 0x68, 1i64); wr(ep(e0), 0x88, 1i64); wr(ep(e0), 0x90, nexus0.id);
    }
    if a.i("base", 0) == 2 {
        // 쌍둥이 타워 타격
        let tw = cache.twin_towers[0][0];
        wr(ep(e0), 0x68, 1i64); wr(ep(e0), 0x88, 1i64); wr(ep(e0), 0x90, tw.id);
    }
    let front = a.s("front", "none");
    if front != "none" {
        let ids: Vec<usize> = front.split(',').map(|s| match s { "e0" => e0.id, "e1" => e1.id, _ => 999999 }).collect();
        if ids.len() > 0 { bb[1].top_minion_state.front_minion = Some(ids[0]); }
        if ids.len() > 1 { bb[1].mid_minion_state.front_minion = Some(ids[1]); }
        if ids.len() > 2 { bb[1].bottom_minion_state.front_minion = Some(ids[2]); }
    }
    if a.i("frontown", 0) == 1 { bb[0].top_minion_state.front_minion = Some(e0.id); }
    let data = OperationData::new(&cache, &ctx, &bb);
    let tgts = a.s("tgt", "e0");
    let tgt: usize = match tgts.as_str() { "e0" => e0.id, "e1" => e1.id, "a1" => cache.player_champion[0][1].unwrap().id, _ => 999999 };
    let act = a.s("act", "attack");
    let seed = a.i("seed", 99) as u64;
    let mut rndA = rand::rngs::StdRng::seed_from_u64(seed);
    let action: SmallActionPlay = match act.as_str() {
        "attack" => SmallActionPlay::Attack(SmallActionAttack::new(&data, tgt)),
        "skill" => SmallActionPlay::Skill(game_ai::SmallActionSkill::new(&data, tgt)),
        "skill2" => SmallActionPlay::Skill2(game_ai::SmallActionSkill2::new(&data, tgt)),
        "ult" => SmallActionPlay::Ult(game_ai::SmallActionUlt::new(&data, tgt)),
        "around" => { let mut r0 = rand::rngs::StdRng::seed_from_u64(1); SmallActionPlay::Around(SmallActionAround::new(version, &mut r0, &data, player, tgt, a.i("radius", 30000) as usize)) },
        "aroundhide" => { let mut r0 = rand::rngs::StdRng::seed_from_u64(1); SmallActionPlay::AroundHide(game_ai::SmallActionAroundHide::new(version, &mut r0, &data, player, tgt, a.i("radius", 30000) as usize)) },
        _ => panic!("act"),
    };
    let ap = &action as *const SmallActionPlay as *const u8;
    println!("action\ttag={}\tpayload8={}\ttgt={}\te0={}\te1={}\tnexus0={}\tchamp={}\tlvl={}", rd::<u8>(ap, 0xb1), rd::<usize>(ap, 8), tgt, e0.id, e1.id, nexus0.id, champ.id, rd::<usize>(cp, 0x5c8));
    let sp: DefenseNexusSubPlan = Default::default();
    let spp = &sp as *const DefenseNexusSubPlan as *const u8;
    let self_before: [u8; 24] = unsafe { std::ptr::read(spp as *const [u8; 24]) };
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let mut dbgA: DebugFrameData = Default::default();
    let mut dbgB: DebugFrameData = Default::default();
    // 기대값: 같은 순서로 콜리 직접 호출
    let mut rndB = rand::rngs::StdRng::seed_from_u64(seed);
    let mode = a.s("mode", "score");
    let rndA0 = rndbytes(&rndA); let rndB0 = rndbytes(&rndB);
    let expect = if mode != "expect" { Err(Box::new(0u8) as Box<dyn std::any::Any + Send>) } else { std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let base = game_ai::interaction_score(version, &mut rndB, player, &data, param, &action, &mut dbgB);
        let tent = data.cache.game.get_entity_by_id(tgt);
        let mut extra: i64 = 0;
        let mut bonus100 = false;
        if version > 1 && matches!(act.as_str(), "attack" | "skill" | "skill2" | "ult") {
            if let Some(t) = tent { if is_base_attacking_minion(player, &data, t) { bonus100 = true; } }
        }
        match act.as_str() {
            "attack" => { extra = match tent { None => -99999, Some(t) => game_ai::calculate_action_score(version, &mut rndB, player, &data, param, champ.attack(), champ.attack_effect.as_ref().unwrap(), champ.attack_speed_mult(), t, MinionActionType::Push, &mut dbgB) }; }
            "skill" => { extra = match tent { None => -99999, Some(t) => game_ai::calculate_action_score(version, &mut rndB, player, &data, param, champ.skill(), champ.skill_effect.as_ref().unwrap(), champ.cooldown_reduce(false), t, MinionActionType::Push, &mut dbgB) }; }
            "skill2" => { extra = match tent { None => -99999, Some(t) => { let eff = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }; game_ai::calculate_action_score(version, &mut rndB, player, &data, param, champ.skill2(), eff.unwrap(), champ.cooldown_reduce(false), t, MinionActionType::Push, &mut dbgB) } }; }
            "ult" => { extra = 0; }
            "around" | "aroundhide" => {
                extra = 0;
                if let Some(t) = tent {
                    if t.team != champ.team && t.ty.is_any_type_minion() {
                        let ebb = &bb[1 - player.info.team];
                        let fm: Vec<&Entity> = [ebb.top_minion_state.front_minion, ebb.mid_minion_state.front_minion, ebb.bottom_minion_state.front_minion]
                            .iter().filter_map(|o| o.and_then(|id| data.cache.game.get_entity_by_id(id))).collect();
                        let d2 = |e: &Entity| { let dx = e.x.abs_diff(nexus0.x); let dy = e.y.abs_diff(nexus0.y); dx * dx + dy * dy };
                        let nearest = fm.iter().min_by_key(|e| d2(e));
                        if let Some(n) = nearest { if n.id == t.id { extra = 5; } }
                    }
                }
            }
            _ => {}
        }
        (base, extra, bonus100)
    })) };
    let got = if mode == "expect" { Err(Box::new(0u8) as Box<dyn std::any::Any + Send>) } else { std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sp.score(version, param, &mut rndA, player, &data, &action, &mut dbgA)
    })) };
    let self_after: [u8; 24] = unsafe { std::ptr::read(spp as *const [u8; 24]) };
    let h = |b: [u8; 320]| -> u64 { let mut h: u64 = 0xcbf29ce484222325; for x in b.iter() { h ^= *x as u64; h = h.wrapping_mul(0x100000001b3); } h };
    if mode == "expect" {
        match expect {
            Ok((base, extra, b100)) => println!("RESULT	mode=expect	val={}	base={}	extra={}	bonus100={}	rnd={:016x}	rnd_changed={}", base + extra + if b100 { 100 } else { 0 }, base, extra, b100, h(rndbytes(&rndB)), rndbytes(&rndB) != rndB0),
            Err(_) => println!("RESULT	mode=expect	PANIC"),
        }
    } else {
        match got {
            Ok(g) => println!("RESULT	mode=score	val={}	rnd={:016x}	rnd_changed={}	self_eq={}", g, h(rndbytes(&rndA)), rndbytes(&rndA) != rndA0, self_before == self_after),
            Err(_) => println!("RESULT	mode=score	PANIC"),
        }
    }
}
