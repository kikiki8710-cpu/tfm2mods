#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치K 오라클 — 234 Jungle::score · 235 AttackNexus::score · 236 v15 · 237 Recall::score · 239 AroundBush::update_state · 240 LineDefense::calc_spv
//!  케이스당 프로세스 1개(argv[1] = 케이스 이름). 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh <이 파일>
//!  케이스
//!   ld_<style0|1>_<ratio>_<n>   240: style(0=Aggressive,1=Defensive) · hp_ratio(hp=ratio, max=100) · near_allies/enemies 각 n개(216B 제로) → attack/util_value 관측
//!   v15_<sub>                    236: none | hp44 | hp45 | cc | dmg19 | dmg20 | far | near | ally | notvis   (target = 적 Top 챔프 / ally = 아군 Top)
//!   sc_<action>                  237/234/235: interaction_score 기저를 별도 rng 사본으로 재고, 세 score 를 각각 rng 사본으로 호출해 변환식 대조
//!                                action = runaway | recall | attack_missing | attack_enemy | skill_enemy | stop | trace | attack_jungle
//!   ab_<sub>                     239: reselect | hold | debug
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure"]
    fn v15_can_keep_support_pressure(version: usize, player: &PlayerState, data: &OperationData, parameter: &game_ai::ScoreParameter, support_target: Option<usize>) -> bool;
    #[link_name = "_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush12update_state"]
    fn ab_update_state(s: &mut game_ai::SmallActionAroundBush, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, debug: &mut DebugFrameData);
}

unsafe fn w8(p: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(p.add(off), v); }
unsafe fn w64(p: *mut u8, off: usize, v: u64) { std::ptr::write_volatile(p.add(off) as *mut u64, v); }
unsafe fn r8(p: *const u8, off: usize) -> u8 { std::ptr::read_volatile(p.add(off)) }
unsafe fn r64(p: *const u8, off: usize) -> u64 { std::ptr::read_volatile(p.add(off) as *const u64) }
unsafe fn ri64(p: *const u8, off: usize) -> i64 { std::ptr::read_volatile(p.add(off) as *const i64) }
fn snap(p: *const u8, n: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(p, n).to_vec() } }
fn ndiff(a: &[u8], b: &[u8]) -> usize { (0..a.len()).filter(|&i| a[i] != b[i]).count() }
fn mk_effect(range: u64) -> Effect {
    Effect { range, growth_range: 0, start_timing: 0, casting: CastingType::Targeting, target: CastingTarget::Enemy,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, attack_type: AttackType::BaseAttack }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: String = a.get(1).cloned().unwrap_or("ld_1_49_0".to_string());
    let ver: usize = 58;
    if a.len() > 99 { let _ = game_ai::interaction_score as *const (); }
    let mut setting = real_setting();
    let parts0: Vec<&str> = case.split('_').collect();
    if parts0[0] == "v22" && parts0[1] != "line0" { setting.epic_jungle.first_spawn_tick = 100000; }
    if parts0[0] == "v22" && parts0[1] == "disabled" { setting.tower_attack_disable_tick = 500; }
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let dbg_on = case == "ab_debug";
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: dbg_on,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let team = 0usize;
    let tick = 1000usize;
    game.set_tick(tick);
    let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let champ: &Entity = cache0.player_champion[team][1].expect("champ");   // 팀0 정글
    let cp = champ as *const Entity as *mut u8;
    let enemy: &Entity = cache0.player_champion[1][0].expect("enemy top");
    let ep = enemy as *const Entity as *mut u8;
    let ally: &Entity = cache0.player_champion[0][0].expect("ally top");
    let ap = ally as *const Entity as *mut u8;
    let champ_id = unsafe { r64(cp, 0x5c0) } as usize;
    let enemy_id = unsafe { r64(ep, 0x5c0) } as usize;
    let ally_id = unsafe { r64(ap, 0x5c0) } as usize;
    println!("ids\tchamp={}\tenemy={}\tally={}\tjungles={}\tchamp_pos=({},{})\tlevel={}", champ_id, enemy_id, ally_id, cache0.jungles.len(),
             unsafe { r64(cp, 0x660) }, unsafe { r64(cp, 0x668) }, unsafe { r64(cp, 0x5c8) });
    let parts: Vec<&str> = case.split('_').collect();
    unsafe {
        // 공통: 챔프 attack/skill effect 주입(None 이면 score 가 unwrap 패닉)
        std::ptr::write(cp.add(0x490) as *mut Option<Effect>, Some(mk_effect(80000)));
        std::ptr::write(cp.add(0x4c8) as *mut Option<Effect>, Some(mk_effect(80000)));
        w64(cp, 0x628, 100); w64(cp, 0x670, 100);
        if parts[0] == "v15" {
            let (cx, cy) = (r64(cp, 0x660), r64(cp, 0x668));
            let sub = parts[1];
            let d: u64 = if sub == "far" { 200000 } else { 50000 };
            w64(ep, 0x660, cx + d); w64(ep, 0x668, cy);
            w64(ap, 0x660, cx + 30000); w64(ap, 0x668, cy);
            match sub { "hp44" => w64(cp, 0x670, 44), "hp45" => w64(cp, 0x670, 45),
                        "ally2" => { w64(ap, 0x660, cx - 30000); w64(ep, 0x660, cx + 100000); }   // enemy-ally 130000 > 120000 · enemy-champ 100000
                        "ally3" => { w64(ap, 0x660, cx - 10000); w64(ep, 0x660, cx + 100000); }   // enemy-ally 110000 < 120000
                        "ally4" => { w64(ap, 0x660, cx + 170000); w64(ep, 0x660, cx + 190000); }  // enemy-ally 20000 · enemy-champ 190000 > 180000
                        _ => {} }
        }
        if parts[0] == "v22" {
            let tw: &Entity = cache0.top_tower[1].expect("enemy top tower");
            let tp = tw as *const Entity as *mut u8;
            let (tx, ty) = (r64(tp, 0x660), r64(tp, 0x668));
            w64(cp, 0x660, tx + 20000); w64(cp, 0x668, ty);
            std::ptr::write(tp.add(0x490) as *mut Option<Effect>, Some(mk_effect(80000)));
            w8(tp, 0x6b9, 1); w64(tp, 0x6a0, 0);
            let tid = r64(tp, 0x5c0);
            let atw: &Entity = cache0.top_tower[0].expect("ally top tower");
            let atid = r64(atw as *const Entity as *const u8, 0x5c0);
            match parts[1] {
                "nearest" => { w64(tp, 0x88, 1); w64(tp, 0x90, 900); w64(tp, 0x98, champ_id as u64); }
                "nearest0" => { w64(tp, 0x88, 1); w64(tp, 0x90, champ_id as u64); w64(tp, 0x98, 900); }   // .0 에 id 를 넣으면 안 맞아야 한다
                "nearestother" => { w64(tp, 0x88, 1); w64(tp, 0x90, 900); w64(tp, 0x98, 999); }
                "hittower" | "hittargettower" => { w64(cp, 0x28, 1); w64(cp, 0x30, tid); }
                "hitchamp" => { w64(cp, 0x28, 1); w64(cp, 0x30, enemy_id as u64); }
                "hitallytower" => { w64(cp, 0x28, 1); w64(cp, 0x30, atid); }
                "outrange" => { w64(cp, 0x660, tx + 200000); w64(tp, 0x88, 1); w64(tp, 0x90, 900); w64(tp, 0x98, champ_id as u64); }
                _ => {}
            }
            println!("v22setup	tower_id={}	tower_pos=({},{})	champ_pos=({},{})	tower_ty@0x68={}	nearest@0x88={} .0@0x90={} .1@0x98={}	champ_last_attacked_from tag@0x28={} id@0x30={}	setting first_spawn={} tps={} disable={}",
                     tid, tx, ty, r64(cp, 0x660), r64(cp, 0x668), r64(tp, 0x68), r64(tp, 0x88), r64(tp, 0x90), r64(tp, 0x98), r64(cp, 0x28), r64(cp, 0x30),
                     setting.epic_jungle.first_spawn_tick, setting.tick_per_second, setting.tower_attack_disable_tick);
        }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    if parts[0] == "v15" && parts[1] != "notvis" { bb[1].last_visible[0] = tick; }   // 적팀 보드 · Top(0) 최근 가시
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, Position::Jungle).expect("player");
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let pp = spbuf.as_mut_ptr() as *mut u8;
    let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
    let mut dbgf: DebugFrameData = Default::default();

    match parts[0] {
        "ld" => {
            let style: u8 = parts[1].parse().unwrap();
            let ratio: u64 = parts[2].parse().unwrap();
            let n: usize = parts[3].parse().unwrap();
            unsafe { w64(cp, 0x628, 100); w64(cp, 0x670, ratio); }
            let mut rnd = rand::rngs::StdRng::seed_from_u64(77);
            let ls = if style == 1 { LineStyle::Defensive } else { LineStyle::Aggressive };
            let sub = game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new(ver, &mut rnd, player, &data, LineType::Mid, game_ai::MinionActionType::Normal, ls);
            let sp = &sub as *const _ as *const u8;
            println!("self\tstyle@0={}\tline@1={}\tmat@2={}", unsafe { r8(sp, 0) }, unsafe { r8(sp, 1) }, unsafe { r8(sp, 2) });
            // 힙 원소: bumpalo Vec<ChampionScoreParameter> n개(제로) 를 0x14b8/0x14d8 에 꽂는다
            let mut va: bumpalo::collections::Vec<game_ai::ChampionScoreParameter> = bumpalo::collections::Vec::with_capacity_in(n.max(1), &pool);
            let mut ve: bumpalo::collections::Vec<game_ai::ChampionScoreParameter> = bumpalo::collections::Vec::with_capacity_in(n.max(1), &pool);
            for _ in 0..n {
                va.push(unsafe { std::mem::MaybeUninit::<game_ai::ChampionScoreParameter>::zeroed().assume_init() });
                ve.push(unsafe { std::mem::MaybeUninit::<game_ai::ChampionScoreParameter>::zeroed().assume_init() });
            }
            let (pa, pe) = (va.as_ptr() as *const u8, ve.as_ptr() as *const u8);
            unsafe {
                std::ptr::write(pp.add(0x14b8) as *mut bumpalo::collections::Vec<game_ai::ChampionScoreParameter>, va);
                std::ptr::write(pp.add(0x14d8) as *mut bumpalo::collections::Vec<game_ai::ChampionScoreParameter>, ve);
            }
            let before = snap(pp, 5384);
            let rb = snap(&rnd as *const _ as *const u8, 320);
            let pm: &mut game_ai::ScoreParameter = unsafe { &mut *spbuf.as_mut_ptr() };
            sub.calculate_score_parameter_value(&mut rnd, player, &data, pm);
            let after = snap(pp, 5384);
            let ra = snap(&rnd as *const _ as *const u8, 320);
            let changed: Vec<usize> = (0..5384).filter(|&i| before[i] != after[i]).collect();
            let mut offs: Vec<usize> = changed.iter().map(|&i| i & !7).collect(); offs.dedup();
            let hp_ratio = ratio * 100 / 100;
            let expect: i64 = if style == 1 { if hp_ratio > 50 { 50 } else { 70 } } else { if hp_ratio < 50 { 50 } else { 30 } };
            let ev_enemy: i64 = if style == 1 { 50 } else { 100 };
            unsafe {
                println!("out\tstyle={}\tratio={}\tattack_value@0x9c0={}\tutil_value@0x9c8={}\texpect={}\tmatch={}\tself_changed_qwords={:?}\trnd_bytes_changed={}",
                         style, ratio, ri64(pp, 0x9c0), ri64(pp, 0x9c8), expect, ri64(pp, 0x9c0) == expect && ri64(pp, 0x9c8) == expect, offs, ndiff(&rb, &ra));
                for i in 0..n {
                    let ea = pa.add(i * 216); let ee = pe.add(i * 216);
                    println!("ally[{}]\tattack@0xa8={}\tutil@0xb0={}\texpect=50\tmatch={}\tother_nonzero_bytes={}", i, ri64(ea, 0xa8), ri64(ea, 0xb0),
                             ri64(ea, 0xa8) == 50 && ri64(ea, 0xb0) == 50, (0..216usize).filter(|&k| !(0xa8..0xb8).contains(&k) && r8(ea, k) != 0).count());
                    println!("enemy[{}]\tattack@0xa8={}\tutil@0xb0={}\texpect={}\tmatch={}\tother_nonzero_bytes={}", i, ri64(ee, 0xa8), ri64(ee, 0xb0), ev_enemy,
                             ri64(ee, 0xa8) == ev_enemy && ri64(ee, 0xb0) == ev_enemy, (0..216usize).filter(|&k| !(0xa8..0xb8).contains(&k) && r8(ee, k) != 0).count());
                }
            }
            std::mem::forget(spbuf);
        }
        "v15" => {
            let sub = parts[1];
            unsafe {
                match sub {
                    "cc" => w64(pp, 0x990, 1),
                    "dmg19" => w64(pp, 0x988, 19),
                    "dmg20" => w64(pp, 0x988, 20),
                    _ => {}
                }
            }
            let target: Option<usize> = if sub == "none" { None } else if sub.starts_with("ally") { Some(ally_id) } else { Some(enemy_id) };
            let hp = unsafe { r64(cp, 0x670) };
            let r = unsafe { v15_can_keep_support_pressure(ver, player, &data, param, target) };
            println!("v15\tsub={}\ttarget={:?}\tchamp_hp={}/100\tcc={}\tdmg={}\tenemy_pos=({},{})\tchamp_pos=({},{})\tbb1_last_visible[0]={}\tresult={}",
                     sub, target, hp, unsafe { r64(pp, 0x990) }, unsafe { r64(pp, 0x988) }, unsafe { r64(ep, 0x660) }, unsafe { r64(ep, 0x668) },
                     unsafe { r64(cp, 0x660) }, unsafe { r64(cp, 0x668) }, bb[1].last_visible[0], r);
            std::mem::forget(spbuf);
        }
        "sc" => {
            let act_name = parts[1..].join("_");
            let jungle_t: Option<&Entity> = cache.jungles.get(0).copied();
            let act: game_ai::SmallActionPlay = match act_name.as_str() {
                "runaway" => game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, tick)),
                "recall" => game_ai::SmallActionPlay::Recall(game_ai::SmallActionRecall::new(&data, player, tick)),
                "attack_missing" => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, 999999)),
                "attack_enemy" => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy_id)),
                "attack_jungle" => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, jungle_t.map(|e| e.id).unwrap_or(999998))),
                "skill_enemy" => game_ai::SmallActionPlay::Skill(game_ai::SmallActionSkill::new(&data, enemy_id)),
                "skill_missing" => game_ai::SmallActionPlay::Skill(game_ai::SmallActionSkill::new(&data, 999999)),
                "trace" => game_ai::SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, enemy_id, tick)),
                "stop" => game_ai::SmallActionPlay::Stop,
                _ => panic!("unknown action"),
            };
            let acp = &act as *const _ as *const u8;
            let tag = unsafe { r8(acp, 0xb1) };
            if let Some(j) = jungle_t {
                let jp = j as *const Entity as *const u8;
                println!("jungle0\tid={}\tty@0x68={}\tcamp_type0@0x98={}\tpos=({},{})", j.id, unsafe { r64(jp, 0x68) }, unsafe { r64(jp, 0x98) }, unsafe { r64(jp, 0x660) }, unsafe { r64(jp, 0x668) });
            }
            let mut dbg0: DebugFrameData = Default::default();
            let mut r0 = rand::rngs::StdRng::seed_from_u64(77);
            let base = game_ai::interaction_score(ver, &mut r0, player, &data, param, &act, &mut dbg0);
            let r0b = snap(&r0 as *const _ as *const u8, 320);
            println!("base\taction={}\ttag@0xb1={}\tinteraction_score={}", act_name, tag, base);
            // Recall
            let mut r1 = rand::rngs::StdRng::seed_from_u64(77);
            let mut d1: DebugFrameData = Default::default();
            let rc = game_ai::plan_legacy::sub_plan::RecallSubPlan::default();
            let s_recall = rc.score(ver, param, &mut r1, player, &data, &act, &mut d1);
            let exp_recall: i64 = match tag { 3 | 4 | 8 => base + 50, _ => if base > 0 { if matches!(tag, 16 | 17) { -777 } else { base / 3 } } else { base * 3 } };
            println!("recall_score\t{}\texpect={}\tmatch={}\trnd_same_as_interaction={}", s_recall, exp_recall, s_recall == exp_recall, snap(&r1 as *const _ as *const u8, 320) == r0b);
            // Jungle / AttackNexus (Attack/Skill 은 콜리 가산이 있어 대상 소실 케이스만 정확 예측)
            let mut r2 = rand::rngs::StdRng::seed_from_u64(77);
            let mut d2: DebugFrameData = Default::default();
            let js = game_ai::plan_legacy::sub_plan::JungleSubPlan::new(team, JungleType::Rhino);
            let s_j = js.score(ver, param, &mut r2, player, &data, &act, &mut d2);
            let mut r3 = rand::rngs::StdRng::seed_from_u64(77);
            let mut d3: DebugFrameData = Default::default();
            let an = game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::default();
            let s_a = an.score(ver, param, &mut r3, player, &data, &act, &mut d3);
            let mut note = String::new();
            if act_name == "attack_missing" || act_name == "skill_missing" {
                note = format!("expect jungle={} (단독) attacknexus={} (base-99999) · jungle_match={} · attacknexus_match={}", -99999, base - 99999, s_j == -99999, s_a == base - 99999);
            } else if matches!(tag, 15 | 16 | 17) {
                // 콜리 직접 호출로 가산 재현
                let t: &Entity = game.get_entity_by_id(unsafe { r64(acp, 8) } as usize).expect("target");
                let (actp, effp): (&Box<dyn Action>, &Effect) = if tag == 15 { (&champ.attack, champ.attack_effect.as_ref().unwrap()) } else { (&champ.skill, champ.skill_effect.as_ref().unwrap()) };
                let mut r4 = rand::rngs::StdRng::seed_from_u64(77);
                let mut d4: DebugFrameData = Default::default();
                let _ = game_ai::interaction_score(ver, &mut r4, player, &data, param, &act, &mut d4);
                let jadd = game_ai::calculate_jungle_action_score(&mut r4, player, &data, param, actp, effp, t);
                let tp = t as *const Entity as *const u8;
                let is_j = unsafe { r64(tp, 0x68) } == 4 && unsafe { r64(tp, 0x98) } < 2;
                let exp_j = if is_j { (base + jadd).max(1) } else { base + jadd };
                let mut r5 = rand::rngs::StdRng::seed_from_u64(77);
                let mut d5: DebugFrameData = Default::default();
                let _ = game_ai::interaction_score(ver, &mut r5, player, &data, param, &act, &mut d5);
                let mult = if tag == 15 { champ.attack_speed_mult() } else { champ.cooldown_reduce(false) };
                let aadd = game_ai::calculate_action_score(ver, &mut r5, player, &data, param, actp, effp, mult, t, game_ai::MinionActionType::Push, &mut d5);
                let tb = if tag != 15 && unsafe { r64(tp, 0x68) } == 2 { 3 } else { 0 };
                let exp_a = base + aadd + tb;
                note = format!("target ty@0x68={} camp0@0x98={} is_jungle={} · jungle_add={} expect_jungle={} match={} · action_add={} tower_bonus={} expect_attacknexus={} match={}",
                               unsafe { r64(tp, 0x68) }, unsafe { r64(tp, 0x98) }, is_j, jadd, exp_j, s_j == exp_j, aadd, tb, exp_a, s_a == exp_a);
            } else {
                note = format!("expect jungle=base({}) attacknexus=base({}) · jungle_match={} · attacknexus_match={}", base, base, s_j == base, s_a == base);
            }
            println!("jungle_score\t{}\tattacknexus_score\t{}\t{}", s_j, s_a, note);
            std::mem::forget(spbuf);
        }
        "v22" => {
            let tw: &Entity = cache.top_tower[1].expect("enemy top tower");
            let target: &Entity = if parts[1] == "hittargettower" { tw } else { cache.player_champion[1][0].expect("enemy") };
            let r = game_ai::v22_current_line_non_champion_action_tower_risk(ver, &ctx, &cache, player, target);
            println!("v22	sub={}	target_ty@0x68={}	result={}", parts[1], unsafe { r64(target as *const Entity as *const u8, 0x68) }, r);
            std::mem::forget(spbuf);
        }
        "ab" if parts[1] == "map" => {
            const C: [(usize, usize); 71] = [(0,0),(1,0),(2,0),(19,0),(20,0),(21,0),(0,1),(0,2),(7,4),(8,4),(9,4),(20,4),(20,5),(12,6),(4,7),(4,8),(29,8),(4,9),(14,9),(29,9),(29,10),(26,11),(6,12),(12,12),(13,12),(26,12),(12,13),(26,13),(9,14),(26,14),(20,15),(21,15),(17,16),(21,16),(16,17),(17,17),(25,18),(0,19),(25,19),(0,20),(4,20),(5,20),(15,20),(25,20),(0,21),(15,21),(16,21),(25,21),(25,22),(29,24),(18,25),(19,25),(20,25),(21,25),(22,25),(29,25),(11,26),(12,26),(13,26),(14,26),(29,26),(28,28),(29,28),(8,29),(9,29),(10,29),(24,29),(25,29),(26,29),(28,29),(29,29)];
            let mut cells: Vec<(usize, usize, usize)> = Vec::new();
            for y in 0..30 { for x in 0..30 { if map.bushes[y][x] != 0 { cells.push((x, y, map.bushes[y][x])); } } }
            let set: std::collections::BTreeSet<(usize, usize)> = cells.iter().map(|c| (c.0, c.1)).collect();
            let cset: std::collections::BTreeSet<(usize, usize)> = C.iter().cloned().collect();
            let missing: Vec<_> = cset.difference(&set).cloned().collect();
            let extra: Vec<_> = set.difference(&cset).cloned().collect();
            let mut ids: Vec<usize> = cells.iter().map(|c| c.2).collect(); ids.sort(); ids.dedup();
            println!("map_bushes	nonzero_cells={}	candidates=71	equal={}	candidates_not_in_map={:?}	map_not_in_candidates={:?}	bush_ids={:?}", cells.len(), set == cset, missing, extra, ids);
            for id in &ids { let n = cells.iter().filter(|c| c.2 == *id).count(); print!("bush{}:{} ", id, n); } println!();
            std::mem::forget(spbuf);
        }
        "ab" => {
            let sub = parts[1];
            let mut rnd = rand::rngs::StdRng::seed_from_u64(77);
            let bush = map.bushes[0][0];
            let mut ab = game_ai::SmallActionAroundBush::new(&mut rnd, &data, player, bush);
            let abp = &mut ab as *mut _ as *mut u8;
            let (cx, cy) = unsafe { (r64(cp, 0x660), r64(cp, 0x668)) };
            unsafe {
                w64(abp, 0x18, cx + 15000); w64(abp, 0x20, cy);          // 거리 15000 < 16000
                w64(abp, 0x8, if sub == "hold" { 5000 } else { 0 });    // change_tick
            }
            let before = snap(abp, 120);
            let mut rnd2 = rand::rngs::StdRng::seed_from_u64(99);
            let rb = snap(&rnd2 as *const _ as *const u8, 320);
            let n_cand = {
                let mut k = 0; for y in 0..30 { for x in 0..30 { if map.bushes[y][x] == bush { k += 1; } } } k
            };
            println!("ab\tsub={}\tbush={}\tmap_cells_with_bush={}\tbefore: change_tick={} bush={} target=({},{})",
                     sub, bush, n_cand, unsafe { r64(abp, 8) }, unsafe { r64(abp, 16) }, unsafe { r64(abp, 24) }, unsafe { r64(abp, 32) });
            unsafe { ab_update_state(&mut ab, &mut rnd2, player, &data, &mut dbgf); }
            let after = snap(abp, 120);
            let ra = snap(&rnd2 as *const _ as *const u8, 320);
            let changed: Vec<usize> = (0..120).filter(|&i| before[i] != after[i]).map(|i| i & !7).collect();
            let mut offs = changed.clone(); offs.dedup();
            let (tx, ty, ct) = unsafe { (r64(abp, 24), r64(abp, 32), r64(abp, 8)) };
            let (gx, gy) = ((tx.wrapping_sub(16000)) / 32000, (ty.wrapping_sub(16000)) / 32000);
            let cell_ok = tx % 32000 == 16000 && ty % 32000 == 16000 && gx < 30 && gy < 30 && map.bushes[gy as usize][gx as usize] == bush;
            println!("after\tchanged_qwords={:?}\ttarget=({},{}) cell=({},{}) cell_in_same_bush={}\tchange_tick={} (tick={} · delta={})\trnd_bytes_changed={}\tdebug_lines={}",
                     offs, tx, ty, gx, gy, cell_ok, ct, tick, ct as i64 - tick as i64, ndiff(&rb, &ra), dbgf.lines.len());
            if !dbgf.lines.is_empty() { println!("line0\t{:?}", dbgf.lines[0]); }
            std::mem::forget(spbuf);
        }
        _ => println!("unknown case"),
    }
}
