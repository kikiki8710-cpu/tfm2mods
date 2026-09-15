#![allow(unused, dead_code, non_snake_case, invalid_reference_casting)]
#![feature(thread_local)]
//! 26차 배치H 오라클 — 219 BattleSubPlan::calculate_score_parameter_value(pub) · 221 precompute_champion_powers(pub) ·
//!  222 nexus_last_stand_uncached(hidden · link_name 직접 진입) 진리표 + TLS(CHAMP_POWERS_MEMO · LAST_STAND_MEMO) 직독.
//!  한 프로세스 = 한 케이스(argv k=v · fn=219|221|222). 세계 = TEMPLATE mkgame(10 swordman) + raw write_volatile(함정 ⑦).
//!  predict = 명세 `logic` 독립 재구현(콜리는 pub/hidden 함수를 그대로 씀 · 합성만 검증) ↔ game 실행 결과 대조.
//!  219 대조 축: self 2필드 · near_enemies/near_allies 원소별 attack/util_value · ScoreParameter 5384B 중 바뀐 바이트(=write 표면) ·
//!               BattleSubPlan 48B 불변 · rnd 미소비.
//!  221 대조 축: p 4필드 · p 216B 중 바뀐 바이트 · TLS 슬롯/seed/tick 직독 · 같은 프로세스 2~4차 호출로 hit/miss/무효화 실측.
//!  222 대조 축: bool · LAST_STAND_MEMO 직독(nexus_last_stand pub 경유).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/H/oracle/o26H.rs
//! 실행: o26H.exe fn=219 k=v ...   (드라이버 = run26H.py)
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::BattleSubPlan;
use game_ai::plan_legacy::old::{BattleSubPlanGoal, BattleTactic};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }
fn bytes<T>(p: &T) -> Vec<u8> { unsafe { std::slice::from_raw_parts(p as *const T as *const u8, std::mem::size_of::<T>()).to_vec() } }
fn diff_ranges(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = vec![]; let mut i = 0;
    while i < a.len() { if a[i] != b[i] { let s = i; while i < a.len() && a[i] != b[i] { i += 1; } out.push((s, i)); } else { i += 1; } }
    out
}

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common29v15_can_keep_support_pressure"]
    fn v15_can_keep_support_pressure(version: usize, player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter, st: Option<usize>) -> bool;
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus25nexus_last_stand_uncached"]
    fn nexus_last_stand_uncached(player: &PlayerState, data: &OperationData) -> bool;
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus30base_attacking_minion_uncached"]
    fn base_attacking_minion_uncached(player: &PlayerState, data: &OperationData) -> Option<usize>;
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai5utils17CHAMP_POWERS_MEMO0s_023___RUST_STD_INTERNAL_VAL"]
    static CPM: [u8; 512];
    #[thread_local] #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai5utils17CHAMP_POWERS_MEMO0023___RUST_STD_INTERNAL_VAL"]
    static CPM_B: [u8; 512];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus15LAST_STAND_MEMO0s_023___RUST_STD_INTERNAL_VAL"]
    static LAST_STAND: [u8; 96];
    #[thread_local] #[link_name = "_RNvNCNKNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus15LAST_STAND_MEMO0023___RUST_STD_INTERNAL_VAL"]
    static LAST_STAND_B: [u8; 96];
}

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

fn mkeff(damage: usize, range: u64, cc: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: cc, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}
fn inj_eff(e: *const Entity, off: usize, eff: Effect) { unsafe { std::ptr::write_volatile((ep(e) as usize + off) as *mut Option<Effect>, Some(eff)); } }
fn dsq(a: &Entity, b: &Entity) -> u64 {
    let dx = if a.x < b.x { b.x - a.x } else { a.x - b.x };
    let dy = if a.y < b.y { b.y - a.y } else { a.y - b.y };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}

fn goal_of(tag: i64, focus: usize) -> BattleSubPlanGoal {
    match tag { 0 => BattleSubPlanGoal::Trace { focus }, 1 => BattleSubPlanGoal::Protect { focus }, 2 => BattleSubPlanGoal::Kiting { focus },
        3 => BattleSubPlanGoal::KitingBack { focus }, 4 => BattleSubPlanGoal::RunAway, 5 => BattleSubPlanGoal::Assassin { focus },
        6 => BattleSubPlanGoal::AssassinReady { focus }, _ => BattleSubPlanGoal::End }
}
fn tactic_of(tag: i64) -> BattleTactic {
    match tag { 0 => BattleTactic::Standard, 1 => BattleTactic::Frontline, 2 => BattleTactic::BacklineDPS, 3 => BattleTactic::SkillBurst,
        4 => BattleTactic::Peel, 5 => BattleTactic::AllIn, _ => BattleTactic::Disengage }
}
/// 명세 tactic_modifier 표 (attack, util, self, ally, skills_down)
fn modifier(tag: i64) -> (i64, i64, i64, i64, Option<i64>) {
    match tag { 1 => (100, 100, 70, 140, None), 2 => (90, 90, 130, 100, None), 3 => (120, 100, 130, 100, Some(60)),
        4 => (70, 150, 110, 140, None), 5 => (130, 100, 90, 100, None), 6 => (100, 100, 150, 100, None), _ => (100, 100, 100, 100, None) }
}

fn mk_csp<'a>(pool: &'a bumpalo::Bump, id: usize, team: usize, pos: usize) -> game_ai::ChampionScoreParameter<'a> {
    game_ai::ChampionScoreParameter { action: SmallAction::Stop, risk_possible: bumpalo::collections::Vec::new_in(pool), gain_possible: bumpalo::collections::Vec::new_in(pool),
        id, team, pos, applyed_damage: 0, applyed_cc: 0, risk_damage: 0, risk_epic_damage: 0, risk_cc: 0, risk_possible_tower: 0, action_time: 0,
        attack_value: 7, util_value: 8, attack_power: 0, util_power_base: 0, cc_time_x_inv_cd: 0, buff_inv_cd_count: 0 }
}
fn mk_param<'a>(pool: &'a bumpalo::Bump, champ_id: usize, team: usize, pos: usize, ne: usize, na: usize, version: usize) -> game_ai::ScoreParameter<'a> {
    let mut nes = bumpalo::collections::Vec::new_in(pool); for i in 0..ne { nes.push(mk_csp(pool, 100 + i, 1 - team, i)); }
    let mut nas = bumpalo::collections::Vec::new_in(pool); for i in 0..na { nas.push(mk_csp(pool, 200 + i, team, i)); }
    game_ai::ScoreParameter { wave_snapshot: None, player: mk_csp(pool, champ_id, team, pos), positioning_score: Default::default(),
        near_allies: nas, near_enemies: nes, version, v3_turnback_hold: false }
}

/// 명세 logic 재구현 — (self_a, self_u, enemies[(a,u)], allies[(a,u)], 진단문자열)
fn predict219(goal: i64, focus: usize, tactic: i64, version: usize, player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter, st: Option<usize>) -> (i64, i64, Vec<(i64, i64)>, Vec<(i64, i64)>, String) {
    let cache = data.cache; let team = player.info.team; let pos = player.info.position.as_index();
    let mut sa; let mut su; let mut es: Vec<(i64, i64)> = vec![]; let mut al: Vec<(i64, i64)> = vec![];
    let mut diag = String::new();
    match goal {
        0 | 5 => { sa = 30; su = 30;
            for e in param.near_enemies.iter() { let v = if goal == 5 { 60 } else if e.id == focus { 60 } else { 30 }; es.push((v, v)); }
            for _ in param.near_allies.iter() { al.push((30, 30)); } }
        1 | 2 | 3 => { sa = 50; su = 50;
            for e in param.near_enemies.iter() { let v = if e.id == focus { 80 } else { 50 }; es.push((v, v)); }
            for _ in param.near_allies.iter() { al.push((50, 50)); } }
        6 => { sa = 100; su = 100; for _ in param.near_enemies.iter() { es.push((10, 10)); } for _ in param.near_allies.iter() { al.push((10, 10)); } }
        _ => { sa = 100; su = 100;
            let champ = cache.player_champion[team][pos].unwrap();
            let csf = unsafe { v15_can_keep_support_pressure(version, player, data, param, st) };
            let enemy_team = 1 - team;
            let mut near_tower = false;
            for t in cache.iter_towers_without_nexus(team) {
                let ct: u8 = rd(ep(t), 0x6b9); let btt: usize = rd(ep(t), 0x6a0);
                let d2 = dsq(t, champ);
                if !(ct != 0 && btt == 0 && d2 < 2500000001) { continue; }
                // ★명세 L914~916 은 `e.range(t)` 만 적었으나 IR(m06.ll:33187 · battle.rs:915 closure$0) 은 `e.range(t) + t.radius()` — Effect::range(effect.rs:26) 에는 radius 가 없다(v46_stage2 L797 도 동일)
                let tar = t.attack_effect.as_ref().map(|e| e.range(t) + t.radius() as u64).unwrap_or(0);
                for c in cache.player_champion[enemy_team].iter().flatten() {
                    let rwe = (c.radius() as u64) + tar;
                    let vis = data.blackboard[enemy_team].is_recent_visible(cache.game, player, c);
                    let d2c = dsq(t, c);
                    diag += &format!("tower id={} ({},{}) d2me={} tar={} [tag={} rng={} grw={} lvl={} sbr={} rmul={} rad={}] | enemy id={} ({},{}) rad={} vis={} d2c={} rwe2={} ; ", t.id, t.x, t.y, d2, tar,
                        rd::<i32>(ep(t), 0x4c0), rd::<u64>(ep(t), 0x4a0), rd::<u64>(ep(t), 0x4a8), rd::<u64>(ep(t), 0x5c8), rd::<u64>(ep(t), 0x438), rd::<i32>(ep(t), 0x470), rd::<u64>(ep(t), 0x680), c.id, c.x, c.y, c.radius(), vis, d2c, rwe * rwe);
                    if vis && d2c <= rwe * rwe { near_tower = true; }
                }
            }
            let ev = if near_tower { 50 } else if csf { 35 } else { 10 };
            diag += &format!("near_tower={} csf={} ev={}", near_tower, csf, ev);
            for _ in param.near_enemies.iter() { es.push((ev, ev)); } for _ in param.near_allies.iter() { al.push((100, 100)); } }
    }
    if tactic != 0 {
        let (mut atk, util, slf, ally, sd) = modifier(tactic);
        if let Some(reduced) = sd {
            let champ = cache.player_champion[team][pos].unwrap();
            let usable = champ.can_skill() as usize + champ.can_skill2() as usize + champ.can_ult() as usize;
            diag += &format!(" usable={}", usable);
            if usable == 0 { atk = reduced; }
        }
        for e in es.iter_mut() { e.0 = e.0 * atk / 100; e.1 = e.1 * util / 100; }
        sa = sa * slf / 100; su = su * slf / 100;
        for a in al.iter_mut() { a.0 = a.0 * ally / 100; a.1 = a.1 * ally / 100; }
    }
    (sa, su, es, al, diag)
}

/// 221 명세 logic 재구현(메모 없이 순수 계산) — (attack_power, util_power_base, cc_acc, buff_acc)
fn predict221(version: usize, data: &OperationData, team: usize, pos: usize) -> (i64, i64, i64, i64, String) {
    let cache = data.cache; let c = &cache.player_champion_cache[team][pos];
    let mut atk: usize = 0; for i in 0..5 { atk = atk.wrapping_add(c.attack_per_sec[i]).wrapping_add(c.skill_per_sec[i]).wrapping_add(c.skill2_per_sec[i]).wrapping_add(c.ult_per_sec[i]); }
    let mut heal: usize = 0; let mut shield: usize = 0;
    for i in 0..5 { heal = heal.wrapping_add(c.skill_heal_sec[i]).wrapping_add(c.skill2_heal_sec[i]).wrapping_add(c.ult_heal_sec[i]);
                    shield = shield.wrapping_add(c.skill_shield_sec[i]).wrapping_add(c.skill2_shield_sec[i]).wrapping_add(c.ult_shield_sec[i]); }
    let ap = (atk / 5) as i64; let up = (heal / 5 + shield / 5) as i64;
    let mut cc_acc: i64 = 0; let mut buff_acc: i64 = 0; let mut diag = format!("atk_total={} heal={} shield={}", atk, heal, shield);
    if let Some(champ) = cache.player_champion[team][pos] {
        let none: Option<Effect> = None;
        let s1 = (champ.skill_effect(), champ.skill_cooltime());
        let s2 = (if champ.level > 2 { champ.skill2_effect() } else { &none }, champ.skill2_cooltime());
        let s3 = (if champ.level > 4 { champ.ult_effect() } else { &none }, champ.ult_cooltime());
        for (k, (eo, cd)) in [s1, s2, s3].into_iter().enumerate() {
            let Some(eff) = eo.as_ref() else { diag += &format!(" slot{}=None", k); continue };
            let cd_i = cd.max(1) as i64;
            let cct = game_ai::effect_cc_time(version, eff);
            if let Some(cc) = cct { cc_acc += (cc as i64 * 1000) / cd_i; }
            let bt = game_ai::effect_buff_target(version, eff, data.context, champ as &dyn AbstractEntity, champ as &dyn AbstractEntity);
            if bt.is_some() { buff_acc += 1000 / cd_i; }
            diag += &format!(" slot{}=Some(cd={} cc={:?} buff={})", k, cd, cct, bt.is_some());
        }
    } else { diag += " champ=None"; }
    (ap, up, cc_acc, buff_acc, diag)
}

fn cpm_dump(tag: &str) {
    unsafe {
        let b = &CPM; let flag: i64 = rd(b.as_ptr(), 0); let seed: u64 = rd(b.as_ptr(), 8 + 480); let tick: u64 = rd(b.as_ptr(), 8 + 488);
        let mut slots = String::new();
        for t in 0..2 { for p in 0..5 { let o = 8 + (t * 5 + p) * 48; let tg: i64 = rd(b.as_ptr(), o);
            if tg != 0 { slots += &format!("[{}][{}]=(tag={} ver={} a={} u={} cc={} b={}) ", t, p, tg, rd::<i64>(b.as_ptr(), o + 8), rd::<i64>(b.as_ptr(), o + 16), rd::<i64>(b.as_ptr(), o + 24), rd::<i64>(b.as_ptr(), o + 32), rd::<i64>(b.as_ptr(), o + 40)); } } }
        println!("tls_cpm\t{}\tstate={} stateB={}\tflag={} seed={} tick={}\t{}", tag, b[504], CPM_B[504], flag, seed, tick, slots);
    }
}
fn ls_dump(tag: &str) {
    unsafe { for (nm, b) in [("s_0", &LAST_STAND), ("00", &LAST_STAND_B)] {
        let items: u64 = rd(b.as_ptr(), 48); let mask: u64 = rd(b.as_ptr(), 32); let ctrl: *const u8 = rd(b.as_ptr(), 24);
        let mut ents = String::new();
        if items > 0 && !ctrl.is_null() { // hashbrown: ctrl 바이트 앞쪽에 버킷이 거꾸로 (bucket i = ctrl - (i+1)*16) · 원소 {usize key, (u8,u8,u8)} 16B
            for i in 0..=(mask as usize) { let c: u8 = *ctrl.add(i); if c & 0x80 == 0 { let bp = ctrl.sub((i + 1) * 16);
                ents += &format!("[{}]key={} v=({},{},{}) ", i, rd::<u64>(bp, 0), rd::<u8>(bp, 8), rd::<u8>(bp, 9), rd::<u8>(bp, 10)); } } }
        println!("tls_last_stand[{}]	{}	state={}	flag={} seed={} tick={} ctrl={:?} bucket_mask={} growth_left={} items={}	{}", nm, tag, b[88],
            rd::<i64>(b.as_ptr(), 0), rd::<u64>(b.as_ptr(), 8), rd::<u64>(b.as_ptr(), 16), ctrl, mask, rd::<u64>(b.as_ptr(), 40), items, ents); } }
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut m = HashMap::new();
    for a in &argv { if let Some((k, v)) = a.split_once('=') { m.insert(k.to_string(), v.parse::<i64>().unwrap_or(0)); } }
    let args = Args { m };
    if args.get("dummy", 0) == 99 { let _ = game_ai::precompute_champion_powers as *const (); }
    let which = args.get("fn", 219);
    let setting = real_setting(); let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default(); let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new(); let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new(); let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false, tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(11);
    for _ in 0..(args.get("ticks", 0) as usize) { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    if args.has("tick") { game.set_tick(args.get("tick", 100) as usize); }
    let pteam = args.get("pteam", 0) as usize; let ppos = args.get("ppos", 1) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let version = args.get("ver", 55) as usize;
    // ── 좌표·이펙트 세팅(임시 캐시로 포인터만 얻는다) ──
    {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let ch = c0.player_champion[pteam][ppos].expect("champ");
        let et = 1 - pteam; let en = c0.player_champion[et][args.get("epos", 0) as usize].expect("enemy");
        if args.has("x") { wr(ep(ch), 0x660, args.get("x", 0) as u64); wr(ep(ch), 0x668, args.get("y", 0) as u64); }
        if args.has("lvl") { wr(ep(ch), 0x5c8, args.get("lvl", 1) as u64); }
        if args.has("eff") { inj_eff(ch, 0x490, mkeff(50, args.get("eff", 30000) as u64, args.get("cc", 0) as usize)); }
        if args.has("seff") { inj_eff(ch, 0x4c8, mkeff(50, args.get("seff", 30000) as u64, args.get("cc", 0) as usize)); }
        if args.has("seff2") { inj_eff(ch, 0x500, mkeff(50, args.get("seff2", 30000) as u64, args.get("cc", 0) as usize)); }
        if args.has("ueff") { inj_eff(ch, 0x538, mkeff(50, args.get("ueff", 30000) as u64, args.get("cc", 0) as usize)); }
        if args.has("scd") { wr(ep(ch), 0xb8, args.get("scd", 0) as i64); }   // Champion.skill_cooldown
        if args.has("ex") { wr(ep(en), 0x660, args.get("ex", 0) as u64); wr(ep(en), 0x668, args.get("ey", 0) as u64); }
        if args.has("eeff") { inj_eff(en, 0x490, mkeff(50, args.get("eeff", 30000) as u64, 0)); }
        if args.has("enoeff") { unsafe { std::ptr::write_volatile((ep(en) as usize + 0x490) as *mut Option<Effect>, None); } }
        // 타워 위치 참조용 출력 · nt=1 이면 내 챔프와 적 챔프를 첫 타워 곁으로
        let mut towers: Vec<(usize, u64, u64, u64)> = vec![];
        for t in c0.iter_towers_without_nexus(pteam) { towers.push((t.id, t.x, t.y, t.attack_effect.as_ref().map(|e| e.range(t)).unwrap_or(0))); }
        println!("towers\t{:?}", towers);
        if let Some(nx) = c0.nexus[pteam] { println!("nexus\tid={} ({},{}) twins={:?}", nx.id, nx.x, nx.y, c0.twin_towers[pteam].iter().map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>()); }
        if args.has("nt") { let ti = args.get("nt", 0) as usize; let (_, tx, ty, _) = towers[ti];
            wr(ep(ch), 0x660, tx + args.get("mdx", 30000) as u64); wr(ep(ch), 0x668, ty);
            wr(ep(en), 0x660, tx + args.get("edx", 20000) as u64); wr(ep(en), 0x668, ty + args.get("edy", 0) as u64); }
        if args.has("atnexus") { let nx = c0.nexus[pteam].unwrap(); wr(ep(en), 0x660, nx.x + args.get("atnexus", 20000) as u64); wr(ep(en), 0x668, nx.y); }
        if args.has("attwin") { let tw = c0.twin_towers[pteam][args.get("attwin", 0) as usize]; wr(ep(en), 0x660, tw.x + args.get("twdx", 20000) as u64); wr(ep(en), 0x668, tw.y); }
        if args.has("atme") { wr(ep(en), 0x660, ch.x + args.get("atme", 20000) as u64); wr(ep(en), 0x668, ch.y); }
    }
    for _ in 0..(args.get("post", 0) as usize) { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(pteam, poss[ppos]).expect("player");
    let champ = cache.player_champion[pteam][ppos].expect("champ");
    let gtick = (&game as &dyn AbstractGame).tick(); let gseed = (&game as &dyn AbstractGame).seed();
    println!("world\ttick={} seed={} pteam={} ppos={} champ id={} ({},{}) lvl={} can_skill={} can_skill2={} can_ult={} atk_some={} skill_some={}",
        gtick, gseed, pteam, ppos, champ.id, champ.x, champ.y, champ.level, champ.can_skill(), champ.can_skill2(), champ.can_ult(), champ.attack_effect.is_some(), champ.skill_effect.is_some());

    if which == 219 {
        let goal = args.get("goal", 0); let tactic = args.get("tactic", 0);
        let ne = args.get("ne", 2) as usize; let na = args.get("na", 1) as usize;
        let fi = args.get("fi", -1); let focus = if fi < 0 { 999usize } else { 100 + fi as usize };
        let st: Option<usize> = if args.has("st") { Some(args.get("st", 0) as usize) } else { None };
        let sp = BattleSubPlan::new(goal_of(goal, focus), st, tactic_of(tactic), false, 0, false);
        let spb = bytes(&sp);
        println!("subplan\tsize={} goal_tag={} focus={} tactic_tag={} st_tag={} st_val={}", std::mem::size_of::<BattleSubPlan>(), rd::<i64>(spb.as_ptr(), 0x10), rd::<i64>(spb.as_ptr(), 0x18), rd::<u8>(spb.as_ptr(), 0x2c), rd::<i64>(spb.as_ptr(), 0), rd::<i64>(spb.as_ptr(), 8));
        let mut param = mk_param(&pool, champ.id, pteam, ppos, ne, na, version);
        let before = bytes(&param);
        let eb: Vec<Vec<u8>> = param.near_enemies.iter().map(|e| bytes(e)).collect();
        let ab: Vec<Vec<u8>> = param.near_allies.iter().map(|e| bytes(e)).collect();
        let mut r_game = rand::rngs::StdRng::seed_from_u64(5); let mut r_chk = r_game.clone();
        sp.calculate_score_parameter_value(version, &mut r_game, player, &data, &mut param);
        let after = bytes(&param);
        let g_self = (param.player.attack_value, param.player.util_value);
        let g_es: Vec<(i64, i64)> = param.near_enemies.iter().map(|e| (e.attack_value, e.util_value)).collect();
        let g_al: Vec<(i64, i64)> = param.near_allies.iter().map(|e| (e.attack_value, e.util_value)).collect();
        let sp_same = bytes(&sp) == spb;
        let rnd_same = r_game.next_u64() == r_chk.next_u64();
        // predict: 같은 내용의 param2 (v15 가 param 을 읽으므로 자기값 100 세팅 후 호출)
        let mut param2 = mk_param(&pool, champ.id, pteam, ppos, ne, na, version);
        if goal == 4 || goal == 7 { param2.player.attack_value = 100; param2.player.util_value = 100; }
        let (pa, pu, pes, pal, diag) = predict219(goal, focus, tactic, version, player, &data, &param2, st);
        println!("diag\t{}", diag);
        println!("game\tself=({},{})\tenemies={:?}\tallies={:?}", g_self.0, g_self.1, g_es, g_al);
        println!("mine\tself=({},{})\tenemies={:?}\tallies={:?}", pa, pu, pes, pal);
        let sp_diff = diff_ranges(&before, &after);
        let e_diff: Vec<Vec<(usize, usize)>> = param.near_enemies.iter().zip(eb.iter()).map(|(e, b)| diff_ranges(b, &bytes(e))).collect();
        let a_diff: Vec<Vec<(usize, usize)>> = param.near_allies.iter().zip(ab.iter()).map(|(e, b)| diff_ranges(b, &bytes(e))).collect();
        println!("writes\tScoreParameter_diff={:?}\tenemy_elem_diff={:?}\tally_elem_diff={:?}\tsubplan_same={}\trnd_same={}", sp_diff, e_diff, a_diff, sp_same, rnd_same);
        let ok = g_self == (pa, pu) && g_es == pes && g_al == pal && sp_same && rnd_same;
        println!("RESULT\t{}", if ok { "MATCH" } else { "MISMATCH" });
    } else if which == 221 {
        let t = args.get("team", pteam as i64) as usize; let p = args.get("pos", ppos as i64) as usize;
        let mut cp = mk_csp(&pool, champ.id, t, p);
        let before = bytes(&cp);
        cpm_dump("before");
        game_ai::precompute_champion_powers(version, &data, &mut cp);
        let after = bytes(&cp);
        let g = (cp.attack_power, cp.util_power_base, cp.cc_time_x_inv_cd, cp.buff_inv_cd_count);
        cpm_dump("after1");
        let (pa, pu, pc, pb, diag) = predict221(version, &data, t, p);
        println!("diag\t{}", diag);
        println!("game\t{:?}", g); println!("mine\t{:?}", (pa, pu, pc, pb));
        println!("writes\tp216_diff={:?}", diff_ranges(&before, &after));
        let mut ok = g == (pa, pu, pc, pb);
        // ── 2차: 같은 프로세스 · 캐시 값을 바꿔도 hit 이면 재생되는가 (TLS 실측) ──
        if args.get("tls", 1) != 0 {
            let cc = &cache.player_champion_cache[t][p] as *const ChampionCache as *const u8;
            let old0: usize = rd(cc, 0x190); wr(cc, 0x190, old0 + 5000);          // attack_per_sec[0] += 5000 → 미스라면 attack_power 가 +1000
            let mut cp2 = mk_csp(&pool, champ.id, t, p);
            game_ai::precompute_champion_powers(version, &data, &mut cp2);
            let (pa2, _, _, _, _) = predict221(version, &data, t, p);
            println!("hit_replay\tsame_ver: game attack_power={} (pure recompute would be {}) replayed={}", cp2.attack_power, pa2, cp2.attack_power == g.0);
            cpm_dump("after2_hit");
            // 3차: version 바꾸면 슬롯 miss → 새 값(+1000) 으로 덮어씀
            let mut cp3 = mk_csp(&pool, champ.id, t, p);
            game_ai::precompute_champion_powers(version + 1, &data, &mut cp3);
            println!("ver_miss\tver+1: attack_power={} expect={} ok={}", cp3.attack_power, pa2, cp3.attack_power == pa2);
            cpm_dump("after3_vermiss");
            // 4차: 다른 슬롯도 채운 뒤 tick 을 바꾸면 전 슬롯 무효화(다른 슬롯 tag 0) 되는가
            let p2 = (p + 1) % 5; let mut cp4 = mk_csp(&pool, champ.id, t, p2);
            game_ai::precompute_champion_powers(version, &data, &mut cp4);
            cpm_dump("after4_slot2");
            unsafe { (*(&game as *const Game as *mut Game)).set_tick(gtick + 7); }
            let cache2 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let data2 = OperationData::new(&cache2, &ctx, &bb);
            let mut cp5 = mk_csp(&pool, champ.id, t, p);
            game_ai::precompute_champion_powers(version, &data2, &mut cp5);
            let (pa5, _, _, _, _) = predict221(version, &data2, t, p);
            println!("tick_inval\ttick+7: attack_power={} pure={} ok={}", cp5.attack_power, pa5, cp5.attack_power == pa5);
            cpm_dump("after5_tickinval");
            ok = ok && cp2.attack_power == g.0 && cp3.attack_power == pa2;
        }
        println!("RESULT\t{}", if ok { "MATCH" } else { "MISMATCH" });
    } else if which == 222 {
        let team = player.info.team; let enemy = 1 - team;
        // predict
        let mut p1 = false; let mut p3 = false; let mut diag = String::new();
        let mut pred = false;
        if let Some(nx) = cache.nexus[team] {
            let twins = &cache.twin_towers[team];
            for c in cache.player_champion[enemy].iter().flatten() {
                let Some(atk) = c.attack_effect.as_ref() else { diag += &format!("c{}:noeff ", c.id); continue };
                let r = atk.is_in_range(c, nx) || twins.iter().any(|t| atk.is_in_range(c, t));
                diag += &format!("c{}:nexus_or_twin={} ", c.id, r); if r { p1 = true; }
            }
            if p1 { pred = true; } else if let Some(me) = cache.player_champion[team][player.info.position.as_index()] {
                for c in cache.player_champion[enemy].iter().flatten() { if let Some(atk) = c.attack_effect.as_ref() { let r = atk.is_in_range(c, me); diag += &format!("c{}:me={} ", c.id, r); if r { p3 = true; } } }
                if p3 { let bam = unsafe { base_attacking_minion_uncached(player, &data) }; diag += &format!("bam={:?}", bam); pred = bam.is_some(); }
            } else { diag += "me=None"; }
        } else { diag += "nexus=None"; }
        let g = unsafe { nexus_last_stand_uncached(player, &data) };
        println!("diag\t{}", diag);
        println!("game\t{}\nmine\t{}", g, pred);
        ls_dump("before_pub");
        let g2 = game_ai::plan_legacy::old::nexus_last_stand(player, &data);
        ls_dump("after_pub1");
        let g3 = game_ai::plan_legacy::old::nexus_last_stand(player, &data);
        ls_dump("after_pub2");
        println!("pub_nexus_last_stand\t{} {} (uncached={})", g2, g3, g);
        println!("RESULT\t{}", if g == pred && g2 == g { "MATCH" } else { "MISMATCH" });
    }
}
