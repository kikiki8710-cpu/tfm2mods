#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치A · 188 BattleSubPlan::action_candidates 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10, 미니언 없음).
//!  관측: sret Vec 원소의 태그(+0xb1)·주요 필드 / &mut self 5필드 전후 / rnd 소비 여부 / 패닉(End).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/A/oracle/o188.rs
//!  실행: o188.exe goal=<0..7> focus=<e0..e4|a0..a4|none> me=<0..4> ... (드라이버 = run188.py)
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::BattleSubPlan;
use game_ai::plan_legacy::old::{BattleSubPlanGoal, BattleTactic};
use game_ai::plan_legacy::team_plan::TeamPlan;

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

fn tagname(t: u8) -> &'static str {
    match t { 3 => "RunAway", 4 => "Recall", 5 => "Around", 6 => "AroundHide", 7 => "AroundRegion", 8 => "AroundRunAway", 9 => "Positioning",
              11 => "AroundPositionBush", 12 => "AroundBush", 13 => "LaneMinionPosition", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop",
              0 | 1 | 2 => "AroundPosition", _ => "?" }
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
    // 내 챔피언 위치·공격 이펙트
    if a.i("mx", -1) >= 0 { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    let atk = a.i("atk", 1);
    if atk == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.i("adm", 50) as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
    else if atk == 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); } }
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    // 적 챔피언 전원: 위치(near=D 이면 내 위치 + D, 그 외 원래) · 가시(vis=1 이면 bb[1].last_visible[pos]=tick) · 공격 이펙트
    let near = a.i("near", -1);
    let vis = a.i("vis", 0);
    for p in 0..5usize {
        let e = cache.player_champion[1][p].expect("enemy");
        let eb = ep(e);
        if near >= 0 {
            // 적 p 를 내 위치에서 (near + p*1000) 만큼 x 방향으로 떨어뜨린다(서로 겹치지 않게)
            let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
            wr(eb, 0x660, mx + near as u64 + (p as u64) * 1000); wr(eb, 0x668, my);
        }
        if vis == 1 { bb[1].last_visible[p] = tick; }
        if a.i("evis", 0) == 1 { wr(eb, 0x38, 0i64); }   // visible_state[0] = Visible (game.is_visible 경로)
        if a.i("eatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff(a.i("eatk", 0) as usize, a.i("erng", 100000) as u64, CastingType::Targeting))); } }
        if a.i("ehp", -1) >= 0 { wr(eb, 0x670, a.i("ehp", 0) as usize); }
    }
    // 아군: 그대로
    // 특정 적 하나만 멀리(far=p): 그 적을 (1,1) 로
    for f in a.s("far", "").split(',').filter(|x| !x.is_empty()) { let e = cache.player_champion[1][f.parse::<usize>().unwrap()].unwrap(); wr(ep(e), 0x660, 1u64); wr(ep(e), 0x668, 1u64); }
    // 캐스팅 중인 적(force_runaway 실험): cast=p → 그 적 action_state=Skill(4) elapsed=+0x78 · skill_effect casting=Position
    if a.i("cast", -1) >= 0 {
        let e = cache.player_champion[1][a.i("cast", 0) as usize].unwrap(); let eb = ep(e);
        wr(eb, 0x70, 4i64); wr(eb, 0x78, a.i("elapsed", 1000) as usize);
        unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).skill_effect, Some(mkeff(100, a.i("crng", 200000) as u64, CastingType::Position))); }
    }
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
    let tactic = match a.i("tactic", 0) { 1 => BattleTactic::Frontline, 2 => BattleTactic::BacklineDPS, 3 => BattleTactic::SkillBurst, 4 => BattleTactic::Peel, 5 => BattleTactic::AllIn, 6 => BattleTactic::Disengage, _ => BattleTactic::Standard };
    let mut sp = BattleSubPlan::new(goal, support, tactic, a.i("avoid", 0) == 1, a.i("hold", 0) as usize, a.i("dive", 0) == 1);
    let spp = &sp as *const BattleSubPlan as *const u8;
    // 생성자 인자 배치 확인(+0x28 avoid · +0x20 hold · +0x29 with_dive)
    let before: [u8; 48] = unsafe { std::ptr::read(spp as *const [u8; 48]) };
    println!("self_before\tsup_tag={}\tsup={}\tgoal_tag={}\tfocus={}\thold={}\tavoid={}\twith_dive={}\tdive_local={}\tdodge_claim={}\ttactic={}\tbail={}",
        rd::<i64>(spp, 0), rd::<usize>(spp, 8), rd::<i64>(spp, 0x10), rd::<usize>(spp, 0x18), rd::<usize>(spp, 0x20), rd::<u8>(spp, 0x28), rd::<u8>(spp, 0x29), rd::<u8>(spp, 0x2a), rd::<u8>(spp, 0x2b), rd::<u8>(spp, 0x2c), rd::<u8>(spp, 0x2d));
    if a.i("dodge_claim", -1) >= 0 { wr(spp, 0x2b, a.i("dodge_claim", 0) as u8); }
    if a.i("bail", -1) >= 0 { wr(spp, 0x2d, a.i("bail", 0) as u8); }
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let pbase = sp_ptr as *const u8;
    if a.i("cc", -1) >= 0 { wr(pbase, 0x990, a.i("cc", 0) as i64); }
    if a.i("adamage", -1) >= 0 { wr(pbase, 0x988, a.i("adamage", 0) as i64); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let mut tp: TeamPlan = Default::default();
    if a.i("stop", -1) >= 0 { tp.ally_battle_stop_tick[a.i("stop", 0) as usize] = Some(tick); }
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let seed = a.i("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let rnd0 = rnd.clone();
    // 세계 요약
    let ex: u64 = rd(ep(cache.player_champion[1][0].unwrap()), 0x660); let ey: u64 = rd(ep(cache.player_champion[1][0].unwrap()), 0x668);
    let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
    let d0 = game_core::utils::distance(mx, my, ex, ey);
    let twr = cache.top_tower[1].unwrap();
    let mt = cache.top_tower[0].unwrap();
    println!("mytower	top0=({},{})	d={}	can_target={}	atk_range={:?}	hp={}/{}", mt.x, mt.y, game_core::utils::distance(mx, my, mt.x, mt.y), mt.can_target(), mt.attack_effect.as_ref().map(|e| e.range), mt.hp, mt.stat_cached.hp);
    let dt = game_core::utils::distance(mx, my, twr.x, twr.y);
    println!("world\ttick={}\tme=({},{})\thp={}\te0=({},{})\td_e0={}\tenemy_tower=({},{})\td_tower={}\tatk_some={}\tarng={:?}\tradius={}",
        tick, mx, my, champ.hp, ex, ey, d0, twr.x, twr.y, dt, champ.attack_effect.is_some(), champ.attack_effect.as_ref().map(|e| e.range), champ.radius);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sp.action_candidates(version, &mut rnd, player, &data, param, &tp, &mut dbgf)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(v) => {
            let vb = &v as *const _ as *const u8;
            let ptr: *const u8 = rd(vb, 0); let cap: usize = rd(vb, 0x10); let len: usize = rd(vb, 0x18);
            let mut out = String::new();
            for i in 0..len {
                let e = unsafe { ptr.add(i * 184) };
                let t: u8 = rd(e, 0xb1);
                let mut f = format!("{}({})", tagname(t), t);
                match t {
                    14 => f += &format!("[target={} start={} margin={} end_delay={} avoid={} range_only={} minrange_tag={} pf_tag={} le_tag={}]",
                            rd::<usize>(e, 0x60), rd::<usize>(e, 0x58), rd::<u64>(e, 0x78), rd::<usize>(e, 0x80), rd::<u8>(e, 0x90), rd::<u8>(e, 0x91), rd::<i64>(e, 0), rd::<u8>(e, 0x55), rd::<u8>(e, 0x95)),
                    3 => f += &format!("[start={} goal=({},{}) end_delay={} risk={} pbd={} with_skill={} with_ult={} dodge={} committed={} pf_tag={}]",
                            rd::<usize>(e, 0), rd::<u64>(e, 8), rd::<u64>(e, 0x10), rd::<usize>(e, 0x18), rd::<i64>(e, 0x20), rd::<i64>(e, 0x28), rd::<u8>(e, 0x80), rd::<u8>(e, 0x81), rd::<u8>(e, 0x82), rd::<u8>(e, 0x83), rd::<u8>(e, 0x7d)),
                    6 => f += &format!("[start={} target={} goal=({},{}) gain={} end_delay={} pf_tag={}]",
                            rd::<usize>(e, 0), rd::<usize>(e, 8), rd::<u64>(e, 0x10), rd::<u64>(e, 0x18), rd::<i64>(e, 0x20), rd::<usize>(e, 0x28), rd::<u8>(e, 0x75)),
                    15 | 16 | 17 | 18 => f += &format!("[start={} target={} is_act={}]", rd::<usize>(e, 0), rd::<usize>(e, 8), rd::<u8>(e, 0x10)),
                    0 | 1 | 2 => f += &format!("[start={} goal=({},{}) diff={} target=({},{}) radius={} end_delay={} pf_tag={} purpose={} outline={}]",
                            rd::<usize>(e, 0), rd::<u64>(e, 8), rd::<u64>(e, 0x10), rd::<i64>(e, 0x18), rd::<u64>(e, 0x20), rd::<u64>(e, 0x28), rd::<u64>(e, 0x58), rd::<usize>(e, 0x60), rd::<u8>(e, 0xad), rd::<u8>(e, 0xb0), rd::<u8>(e, 0xb1)),
                    _ => {}
                }
                out += &f; out += " | ";
            }
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen();
            println!("RESULT\tlen={}\tcap={}\trnd_used={}\t{}", len, cap, r1 != r0, out);
        }
    }
    println!("self_after\tsup_tag={}\tsup={}\tgoal_tag={}\tfocus={}\thold={}\tavoid={}\twith_dive={}\tdive_local={}\tdodge_claim={}\ttactic={}\tbail={}",
        rd::<i64>(spp, 0), rd::<usize>(spp, 8), rd::<i64>(spp, 0x10), rd::<usize>(spp, 0x18), rd::<usize>(spp, 0x20), rd::<u8>(spp, 0x28), rd::<u8>(spp, 0x29), rd::<u8>(spp, 0x2a), rd::<u8>(spp, 0x2b), rd::<u8>(spp, 0x2c), rd::<u8>(spp, 0x2d));
    let after: [u8; 48] = unsafe { std::ptr::read(spp as *const [u8; 48]) };
    let diff: Vec<String> = (0..48).filter(|&i| before[i] != after[i]).map(|i| format!("+{:#x}:{}->{}", i, before[i], after[i])).collect();
    println!("self_diff\t{}", diff.join(" "));
    // 보조 관측: check_kill_die_tick(champ vs 가시 적 전원) — 치명 판정 재료(캐시는 같은 키라 같은 값)
    if a.i("kdt", 0) == 1 {
        let mut enemies = bumpalo::collections::Vec::new_in(&pool);
        for p in 0..5usize { let e = cache.player_champion[1][p].unwrap(); enemies.push(e); }
        let towers = bumpalo::collections::Vec::new_in(&pool);
        let mut rnd2 = rand::rngs::StdRng::seed_from_u64(seed);
        let die = game_ai::check_kill_die_tick(version, &mut rnd2, &data, player, champ, enemies, towers, &mut dbgf);
        println!("kdt\tdie_tick={}", die);
    }
}
