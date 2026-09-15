#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치J 오라클 — 228 handle_nexus_attack(pub) · 230/231 LineSafe/LineWait::score(pub) ·
//!  232 v30_line_champion_action_tower_aggro_risk(pub) · 233 nexus_final_stand_uncached(hidden → link_name).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10·미니언 없음).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/J/oracle/oJ.rs
//!  실행: oJ.exe fn=<final|v30|score|hna> ... (드라이버 = runJ.py)
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::{LineSafeSubPlan, LineWaitSubPlan};
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::{SmallActionPlay, SmallActionAttack, SmallActionSkill, SmallActionSkill2, SmallActionAround, MinionActionType};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus26nexus_final_stand_uncached"]
    fn nexus_final_stand_uncached(player: &PlayerState, data: &OperationData) -> bool;
}

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

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::check_kill_die_tick as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { m.insert(k.to_string(), v.to_string()); } }
    let a = Args { m };
    let mut setting = real_setting();
    // v30: 라인전 구간 = tick < first_spawn_tick - 30*tps · tower_attack_disable_tick 극성 실험
    if a.i("fst", -1) >= 0 { setting.epic_jungle.first_spawn_tick = a.i("fst", 0) as usize; }
    if a.i("tad", -1) >= 0 { setting.tower_attack_disable_tick = a.i("tad", 0) as usize; }
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
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    println!("world\ttick={}\tversion={}\ttwin0={}\ttwin1={}\tnexus0={}\tnexus1={}\tchamp_id={}\tchamp_lv={}\tchamp_hp={}\tchamp_xy=({},{})",
        tick, version, cache.twin_towers[0].len(), cache.twin_towers[1].len(), cache.nexus[0].is_some(), cache.nexus[1].is_some(),
        champ.id, champ.level, champ.hp, champ.x, champ.y);
    // 내 챔피언 공격 이펙트 · 레벨
    let atk = a.i("atk", 1);
    if atk == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.i("adm", 50) as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
    else if atk == 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); } }
    if a.i("skl", -1) == 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, None); } }
    if a.i("skl", -1) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).skill_effect, Some(mkeff(a.i("adm", 50) as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
    if a.i("lv", -1) >= 0 { wr(cp, 0x5c8, a.i("lv", 1) as usize); }
    if a.i("mx", -1) >= 0 { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    // 적 챔피언 전원 공격 이펙트 / hp
    for p in 0..5usize {
        let e = cache.player_champion[1][p].expect("enemy");
        let eb = ep(e);
        if a.i("eatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff(a.i("eatk", 0) as usize, a.i("erng", 100000) as u64, CastingType::Targeting))); } }
        else if a.i("eatk", -1) == -2 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, None); } }
        if a.i("ehp", -1) >= 0 { wr(eb, 0x670, a.i("ehp", 0) as usize); }
    }
    let id_of = |s: &str| -> usize {
        match s.chars().next() {
            Some('e') => cache.player_champion[1][s[1..].parse::<usize>().unwrap()].unwrap().id,
            Some('a') => cache.player_champion[0][s[1..].parse::<usize>().unwrap()].unwrap().id,
            Some('t') => cache.top_tower[s[1..].parse::<usize>().unwrap()].unwrap().id,
            Some('n') => cache.nexus[s[1..].parse::<usize>().unwrap()].unwrap().id,
            _ => 999999usize,
        }
    };
    let fnname = a.s("fn", "final");
    match fnname.as_str() {
        "final" => {
            // 233: twin=1 → 우리(팀0) 쌍둥이 타워 벡터 비움 · nexus=0 → 우리 넥서스 None · ex=1 → 적 챔프 p 를 우리 넥서스 좌표(+dx) 로
            let team = a.i("team", 0) as usize;
            if a.i("twin", 0) == 1 { cache.twin_towers[team].clear(); }
            let nexus_xy = cache.nexus[team].map(|n| (n.x, n.y, n.id));
            if a.i("nexus", 1) == 0 { cache.nexus[team] = None; }
            if a.i("ex", -1) >= 0 {
                let p = a.i("ex", 0) as usize;
                let e = cache.player_champion[1 - team][p].expect("enemy");
                let (nx, ny, _) = nexus_xy.expect("nexus xy");
                wr(ep(e), 0x660, nx + a.i("dx", 0) as u64); wr(ep(e), 0x668, ny);
            }
            let data = OperationData::new(&cache, &ctx, &bb);
            let pl = game.get_player_by_position(team, poss[me]).expect("player");
            println!("pre\ttwin={}\tnexus={}\tnexus_xy={:?}", cache.twin_towers[team].len(), cache.nexus[team].is_some(), nexus_xy);
            let r = if a.i("cached", 0) == 1 { game_ai::plan_legacy::old::nexus_final_stand(pl, &data) } else { unsafe { nexus_final_stand_uncached(pl, &data) } };
            println!("RESULT\tfinal_stand={}", r);
        }
        "v30" => {
            // 232: act=attack|skill|skill2|around · tgt=e0|a1|n1|t1|bad · near=1 → 나+타깃을 적 탑타워 옆(+dx) 으로
            let tgt = id_of(&a.s("tgt", "e0"));
            let mut rnd = rand::rngs::StdRng::seed_from_u64(a.i("seed", 11) as u64);
            let data = OperationData::new(&cache, &ctx, &bb);
            if a.i("near", 0) == 1 {
                let t = cache.top_tower[1].expect("enemy top tower");
                let dx = a.i("dx", 20000) as u64;
                wr(cp, 0x660, t.x + dx); wr(cp, 0x668, t.y);
                if let Some(te) = cache.game.get_entity_by_id(tgt) { if te.id != t.id { wr(ep(te), 0x660, t.x + dx + 5000); wr(ep(te), 0x668, t.y); } }
                let tb = ep(t);
                println!("tower\tid={}\txy=({},{})\tcan_target={}\tblock_tick={}\tatk_tag={}\trange={}",
                    t.id, t.x, t.y, rd::<u8>(tb, 0x6b9), rd::<u64>(tb, 0x6a0), rd::<i32>(tb, 0x4c0),
                    t.attack_effect.as_ref().map(|e| e.range).unwrap_or(0));
            }
            let action = match a.s("act", "attack").as_str() {
                "attack" => SmallActionPlay::Attack(SmallActionAttack::new(&data, tgt)),
                "skill" => SmallActionPlay::Skill(SmallActionSkill::new(&data, tgt)),
                "skill2" => SmallActionPlay::Skill2(SmallActionSkill2::new(&data, tgt)),
                _ => SmallActionPlay::Around(SmallActionAround::new(version, &mut rnd, &data, player, tgt, 10000)),
            };
            if a.i("thp", -1) >= 0 { if let Some(te) = cache.game.get_entity_by_id(tgt) { wr(ep(te), 0x670, a.i("thp", 0) as usize); } }
            let ap = &action as *const SmallActionPlay as *const u8;
            let te = cache.game.get_entity_by_id(tgt);
            println!("champ_eff\tatk_tag={}\tskill_tag={}\tskill2_tag={}\tlevel={}", rd::<i32>(cp, 0x4c0), rd::<i32>(cp, 0x4f8), rd::<i32>(cp, 0x530), rd::<usize>(cp, 0x5c8));
            println!("pre\taction_tag={}\ttarget={:?}\tchamp_xy=({},{})\ttgt_xy={:?}\ttgt_hp={:?}\tis_line_phase={}\ttad={}",
                rd::<u8>(ap, 0xb1), tgt, rd::<u64>(cp, 0x660), rd::<u64>(cp, 0x668),
                te.map(|e| (e.x, e.y)), te.map(|e| e.hp), ctx.is_line_phase(tick), setting.tower_attack_disable_tick);
            let r = game_ai::v30_line_champion_action_tower_aggro_risk(version, &data, player, &action);
            println!("RESULT\tv30={}", r);
        }
        "score" => {
            // 230/231: plan=safe|wait · act=attack|skill|skill2|around · tgt=... · what=game|mine
            let tgt = id_of(&a.s("tgt", "e0"));
            let mut rnd = rand::rngs::StdRng::seed_from_u64(a.i("seed", 11) as u64);
            let mut rnd2 = rand::rngs::StdRng::seed_from_u64(a.i("seed", 11) as u64);
            let mut dbg: DebugFrameData = Default::default();
            let data = OperationData::new(&cache, &ctx, &bb);
            let param = game_ai::calculate_score_parameter(version, &mut rnd, player, &data, &mut dbg);
            let mut rnd_a = rnd.clone();
            let action = match a.s("act", "attack").as_str() {
                "attack" => SmallActionPlay::Attack(SmallActionAttack::new(&data, tgt)),
                "skill" => SmallActionPlay::Skill(SmallActionSkill::new(&data, tgt)),
                "skill2" => SmallActionPlay::Skill2(SmallActionSkill2::new(&data, tgt)),
                _ => SmallActionPlay::Around(SmallActionAround::new(version, &mut rnd_a, &data, player, tgt, 10000)),
            };
            let ap = &action as *const SmallActionPlay as *const u8;
            let line = match a.i("line", 0) { 1 => LineType::Mid, 2 => LineType::Bottom, _ => LineType::Top };
            println!("pre\taction_tag={}\ttarget={}\ttgt_exists={}\tline={:?}", rd::<u8>(ap, 0xb1), tgt, cache.game.get_entity_by_id(tgt).is_some(), line);
            let what = a.s("what", "game");
            if what == "game" {
                let r = if a.s("plan", "safe") == "wait" {
                    LineWaitSubPlan::new(line).score(version, &param, &mut rnd_a, player, &data, &action, &mut dbg)
                } else {
                    LineSafeSubPlan::new(line).score(version, &param, &mut rnd_a, player, &data, &action, &mut dbg)
                };
                println!("RESULT\tscore={}", r);
            } else if what == "mine" {
                // 구경로 재현: interaction_score + economy(Pull) + arm 가산 (evaluate_action 이 None 인 경우만 유효)
                let base = game_ai::interaction_score(version, &mut rnd_a, player, &data, &param, &action, &mut dbg);
                let econ = game_ai::line_action_economy_adjustment(version, player, &data, &param, &action, MinionActionType::Pull);
                let add: i64 = match a.s("act", "attack").as_str() {
                    "attack" => match cache.game.get_entity_by_id(tgt) {
                        Some(t) => { let eff = champ.attack_effect.as_ref().unwrap();
                            game_ai::calculate_action_score(version, &mut rnd_a, player, &data, &param, &champ.attack, eff, champ.attack_speed_mult(), t, MinionActionType::Pull, &mut dbg) }
                        None => -99999 },
                    "skill" => match cache.game.get_entity_by_id(tgt) {
                        Some(t) => { let eff = champ.skill_effect.as_ref().unwrap();
                            game_ai::calculate_action_score(version, &mut rnd_a, player, &data, &param, &champ.skill, eff, champ.cooldown_reduce(false), t, MinionActionType::Pull, &mut dbg) }
                        None => -99999 },
                    _ => match cache.game.get_entity_by_id(tgt) {
                        Some(t) => if t.team == champ.team {
                            let d = t.x.abs_diff(champ.x).pow(2) + t.y.abs_diff(champ.y).pow(2);
                            if d > 39999999999 { match &t.ty { EntityType::Minion { .. } | EntityType::Tower { .. } => 50, EntityType::Nexus => 100, _ => 0 } } else { 0 }
                        } else { 0 },
                        None => 0 },
                };
                println!("mine\tbase={}\tecon={}\tadd={}", base, econ, add);
                println!("RESULT\tscore={}", base + econ + add);
            } else if what == "eval" {
                // evaluate_action 만 단독 호출 — Some/None 확인 (Lane 앵커)
                let ctxa = game_ai::plan_legacy::action_eval::ActionContext { priority: game_ai::plan_legacy::action_eval::PriorityProfile::Lane, anchor: game_ai::plan_legacy::action_eval::Anchor::Lane { line } };
                let r = game_ai::plan_legacy::action_eval::evaluate_action(version, &ctxa, &param, &mut rnd_a, player, &data, &action, &mut dbg);
                println!("RESULT\teval={:?}", r);
            }
        }
        "hna" => {
            // 228: handle_nexus_attack — 기본 세계에서 어느 라인이 나오는지 · twin=1 이면 적 쌍둥이 비움
            if a.i("twin", 0) == 1 { cache.twin_towers[1].clear(); }
            let mut rnd = rand::rngs::StdRng::seed_from_u64(a.i("seed", 11) as u64);
            let mut dbg: DebugFrameData = Default::default();
            // 전방 미니언 지정: fm=<top,mid,bot entity id 지정용 'e0'/'a0'/'-'>
            let fm = a.s("fm", "-,-,-");
            let ids: Vec<Option<usize>> = fm.split(',').map(|s| if s == "-" { None } else { Some(id_of(s)) }).collect();
            bb[0].top_minion_state.front_minion = ids.get(0).cloned().flatten();
            bb[0].mid_minion_state.front_minion = ids.get(1).cloned().flatten();
            bb[0].bottom_minion_state.front_minion = ids.get(2).cloned().flatten();
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut tp: TeamPlan = Default::default();
            let tpp = &tp as *const TeamPlan as *const u8;
            let n = std::mem::size_of::<TeamPlan>();
            let before: Vec<u8> = (0..n).map(|i| rd::<u8>(tpp, i)).collect();
            for l in [LineType::Top, LineType::Mid, LineType::Bottom] {
                println!("line_exists\t{:?}\t{}", l, game_ai::plan_legacy::rule_scope::line_exists(&ctx, l));
            }
            let r = tp.handle_nexus_attack(version, &mut rnd, player, &data, &mut dbg);
            let after: Vec<u8> = (0..n).map(|i| rd::<u8>(tpp, i)).collect();
            let diff = before.iter().zip(after.iter()).filter(|(x, y)| x != y).count();
            println!("RESULT\thna={:?}\tself_diff_bytes={}\tself_size={}", r, diff, n);
        }
        _ => println!("unknown fn"),
    }
}
