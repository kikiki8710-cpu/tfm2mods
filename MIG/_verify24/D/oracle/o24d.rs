#![allow(unused, dead_code, non_snake_case, invalid_reference_casting)]
#![feature(thread_local)]
//! 24차 배치D 오라클 — 183 interaction_score(pub) 진리표 + INTER_CTX TLS 직독(미러 설계 근거).
//!  한 프로세스 = 한 케이스(argv). 세계 = TEMPLATE mkgame. 엔티티 변조 = raw write_volatile(함정 ⑦).
//!  `mine` = 명세(logic)를 IR 정정(hp 배수 = 현재 hp)대로 독립 재구현 + interaction_ctx 재구현 →
//!  게임 값·TLS 셀 내용과 대조.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify24/D/oracle/o24d.rs
//! 실행: o24d.exe <kind> ...   (드라이버 = run24d.py)
use game_core::*;
use rand::SeedableRng;
use rand::RngCore;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12lane_economy28line_effect_range_with_radii"]
    fn line_effect_range_with_radii(ef: &Effect, a: &Entity, b: &Entity) -> u64;
    // TLS 셀 직독: RefCell<(u64,usize,usize,InterActionCtx)> 104B + lazy 상태 i8 @104 + pad 7
    #[thread_local]
    #[link_name = "_RNvNCNKNvNtCshdEBA0ozCnw_7game_ai12action_score9INTER_CTX0s_023___RUST_STD_INTERNAL_VAL"]
    static INTER_RAW: [u8; 112];
}

fn tls_dump() -> (i64, u64, u64, u64, [u64; 8], u8, u8, u8) {
    let p = unsafe { INTER_RAW.as_ptr() };
    let borrow: i64 = rd(p, 0); let seed: u64 = rd(p, 8); let tick: u64 = rd(p, 16); let pid: u64 = rd(p, 24);
    let mut v = [0u64; 8]; for i in 0..8 { v[i] = rd(p, 32 + 8 * i); }
    (borrow, seed, tick, pid, v, rd(p, 96), rd(p, 97), rd(p, 104))
}

fn set_attack_damage(e: *const Entity, dmg: usize) {
    // Effect.ty(Arc<dyn EffectType>, Entity+0x490 팻포인터 16B) 를 TowerAttackEffect(dmg, 0) 로 교체(22차 A 수법 · 옛 Arc 누수)
    let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
    let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
    wr(ep(e), 0x490, raw[0]); wr(ep(e), 0x498, raw[1]);
}
fn dist2(a: &Entity, b: &Entity) -> u64 { a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) }

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ictx { nearest_vis_enemy: Option<usize>, near_ally_tower: Option<usize>, near_enemy_tower: Option<usize>, vis150_nearest: Option<usize>, vis150_mask: u8, ally100_count: u8 }

/// interaction_ctx(action_score.rs:581~608) 재구현 — m00.ll 91340~92550 독해대로
fn my_ictx(game: &dyn AbstractGame, player: &PlayerState, data: &OperationData, champ: &Entity) -> Ictx {
    let team = player.info.team; let enemy = 1 - team;
    // L583~584 nearest_vis_enemy: 적 챔프 중 is_recent_visible 만, 거리 제한 없음, min_by dist²(첫 최소)
    let mut best: Option<(u64, usize)> = None;
    for e in data.cache.player_champion[enemy].iter().flatten() {
        if !data.blackboard[enemy].is_recent_visible(game, player, e) { continue; }
        let d = dist2(champ, e);
        if best.map_or(true, |(bd, _)| d < bd) { best = Some((d, e.id)); }
    }
    let nearest_vis_enemy = best.map(|x| x.1);
    // L585~587 near_ally_tower: iter_towers_without_nexus(team) 중 dist² < 150000² (strict, +1 없음) → min dist²
    let near = |t_team: usize| -> Option<usize> {
        let mut b: Option<(u64, usize)> = None;
        for t in data.cache.iter_towers_without_nexus(t_team) {
            let d = dist2(t, champ);
            if d >= 22500000000 { continue; }
            if b.map_or(true, |(bd, _)| d < bd) { b = Some((d, t.id)); }
        }
        b.map(|x| x.1)
    };
    let near_ally_tower = near(team);
    let near_enemy_tower = near(enemy);
    // L593~599: 슬롯 0..5, dist² < 150000²+1 && is_recent_visible → mask |= 1<<slot, 최근접(strict <, 첫 승)
    let mut mask = 0u8; let mut vb: Option<(u64, usize)> = None;
    for slot in 0..5usize {
        if let Some(e) = data.cache.player_champion[enemy][slot] {
            let d = dist2(champ, e);
            if d < 22500000001 && data.blackboard[enemy].is_recent_visible(game, player, e) {
                mask |= 1 << slot;
                if vb.map_or(true, |(bd, _)| d < bd) { vb = Some((d, e.id)); }
            }
        }
    }
    // L604~605: 아군 슬롯 0..5, dist² < 100000²+1 (자기 자신 포함, 가시성 무관)
    let mut cnt = 0u8;
    for slot in 0..5usize {
        if let Some(a) = data.cache.player_champion[team][slot] {
            if dist2(a, champ) < 10000000001 { cnt += 1; }
        }
    }
    Ictx { nearest_vis_enemy, near_ally_tower, near_enemy_tower, vis150_nearest: vb.map(|x| x.1), vis150_mask: mask, ally100_count: cnt }
}

fn radius_adj(e: &Entity) -> u64 {
    let mult: i32 = rd(ep(e), 0x470);
    if mult == 0 { e.radius as u64 } else { (e.radius as u64) * (100 + mult as i64) as u64 / 100 }
}
fn tower_attack_range(ef: &Effect, t: &Entity, target: &Entity) -> u64 {
    let tr: usize = rd(ep(t), 0x438);
    let base = ef.range as u64 + tr as u64 + (t.level as u64 - 1) * ef.growth_range as u64;
    let r = base + ef.range_adjust(t, target) + radius_adj(t) + radius_adj(target);
    r.saturating_sub(30000)
}

/// 명세 logic 독립 재구현 (RunAway / Around / Trace 만 — Attack 계열은 internal fastcc 콜리라 게임값만)
fn predict(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, sp: &game_ai::ScoreParameter,
           action: &game_ai::SmallActionPlay, dbg: &mut DebugFrameData, game: &dyn AbstractGame, ctx: &GameContext, use_maxhp: bool) -> (i64, String) {
    let team = player.info.team; let enemy = 1 - team;
    let champ = data.cache.player_champion[team][player.info.position.as_index()].unwrap();
    let ictx = my_ictx(game, player, data, champ);
    let mut log = format!("ictx={:?} ", ictx);
    let undying: bool = rd(ep(champ), 0x488);
    let maxhp = champ.stat_cached.hp as i64; let hp = champ.hp as i64;
    let hp_mul = if use_maxhp { std::cmp::max(maxhp, 1) } else { std::cmp::max(hp, 1) };
    let no_self_risk = if undying { true } else if version > 1 {
        if game_ai::plan_legacy::old::nexus_final_stand(player, data) { true }
        else if (hp as u64) * 100 > (std::cmp::max(maxhp, 1) as u64) * 35 { game_ai::plan_legacy::old::base_defense_focus(player, data) } else { false }
    } else { false };
    let nearest_enemy_champion = ictx.nearest_vis_enemy.and_then(|id| game.get_entity_by_id(id));
    let near_ally_tower = ictx.near_ally_tower.and_then(|id| game.get_entity_by_id(id));
    let near_enemy_tower = ictx.near_enemy_tower.and_then(|id| game.get_entity_by_id(id));
    let p = &sp.player;
    let tad = ctx.setting.tower_attack_disable_tick;
    let mut act_score: i64 = -99999; let mut positioning_score: i64 = 0;
    let mut target_id: Option<usize> = None;
    match action {
        game_ai::SmallActionPlay::RunAway(_) | game_ai::SmallActionPlay::Recall(_) | game_ai::SmallActionPlay::AroundRunAway(_) => {
            if undying { return (-99999, log + "undying"); }
            if version > 1 {
                if game_ai::plan_legacy::old::nexus_final_stand(player, data) { return (-99999, log + "nfs"); }
                let hp_ratio = hp * 100 / std::cmp::max(maxhp, 1);
                if hp_ratio > 35 && game_ai::plan_legacy::old::base_defense_focus(player, data) { return (-99999, log + "bdf"); }
            }
            let applyed_damage = p.applyed_damage as i64;
            let dest = action.evaluation_position(version, player, data).map(|(x, y)| game_ai::position_eval_at(version, player, data, x, y, game_ai::PositionEvalPurpose::RunAway));
            let (base_damage, mut tower_damage) = match dest {
                Some(s) => (std::cmp::max(p.risk_damage as i64 + hp_mul * std::cmp::max(s.risk, 0) / -100, 0),
                            std::cmp::max(p.risk_possible_tower as i64 + hp_mul * std::cmp::max(s.tower_risk, 0) / -100, 0)),
                None => (p.risk_damage as i64, p.risk_possible_tower as i64) };
            log += &format!("dest={:?} bd={} td={} ", dest.map(|s| (s.risk, s.tower_risk)), base_damage, tower_damage);
            let vis = game.is_visible(enemy, champ.id);
            let in_rng = near_enemy_tower.map_or(false, |t| t.attack_effect.as_ref().unwrap().is_in_range(t, champ));
            if !(vis || in_rng) { tower_damage = 0; }
            let possible_damage = tower_damage + p.possible_risk(data, 9999);
            let hp_value = game_ai::champion_hp_value(data, sp, p);
            let f = ctx.map.fountains[team];
            log += &format!("vis={} inrng={} pd={} hpv={} fountain={:?} ", vis, in_rng, possible_damage, hp_value, f);
            if champ.x >= f.0 && champ.x <= f.2 && champ.y >= f.1 && champ.y <= f.3 { return (-99999, log + "fountain"); }
            if applyed_damage < hp {
                let base_score = hp_value * base_damage / hp;
                let possible_score = possible_damage * hp_value / hp;
                let ne = ictx.vis150_mask.count_ones() as i64; let na = ictx.ally100_count as i64; let diff = ne - na;
                let coef = if diff > 1 { 300 } else { match diff { 1 => 200, 0 => 100, -1 => 75, _ => 40 } };
                act_score = base_score / 4 - 2 + coef * possible_score / 800;
                log += &format!("bs={} ps={} ne={} na={} coef={} ", base_score, possible_score, ne, na, coef);
            } else { log += "applyed>=hp "; }
            // L947~954 positioning_score
            let near = ictx.vis150_nearest.and_then(|id| game.get_entity_by_id(id));
            let role = game_ai::get_battle_role(version, ctx, data.cache, player);
            let sc: usize = rd(ep(champ), 0xb8); let s2c: usize = rd(ep(champ), 0xc0);
            let ty_tag: i64 = rd(ep(champ), 0x68);
            let s2_some = if champ.level > 2 { champ.skill2_effect.is_some() } else { false };
            if matches!(role, game_ai::BattleRole::SkillCaster | game_ai::BattleRole::Utility) && champ.skill_effect.is_some() && ty_tag == 13 && sc > 30
                && s2_some && s2c >= 31 && near.map_or(false, |e| dist2(champ, e) < 10000000001) { positioning_score = 10; }
            log += &format!("role={:?} sc={} s2c={} ps10={} ", role, sc, s2c, positioning_score);
            return (act_score + positioning_score, log);
        }
        game_ai::SmallActionPlay::Around(_) | game_ai::SmallActionPlay::AroundHide(_) | game_ai::SmallActionPlay::LaneMinionPosition(_) => {
            let tid: usize = rd(action as *const _ as *const u8, 0x8); target_id = Some(tid);
            let Some(_target) = game.get_entity_by_id(tid) else { return (0, log + "around_notarget") };
            let base_damage = p.risk_damage as i64; let hp_value = game_ai::champion_hp_value(data, sp, p);
            let base_score = hp_value * base_damage / hp;
            let mut tower_score = 0i64;
            if let Some(t) = near_ally_tower { if let Some(target) = game.get_entity_by_id(tid) {
                let ef = t.attack_effect.as_ref().unwrap();
                if rd::<i64>(ep(target), 0x0) != rd::<i64>(ep(champ), 0x0) || rd::<i64>(ep(target), 0x8) != rd::<i64>(ep(champ), 0x8) {
                    if ef.is_in_range(t, target) && game.tick() < tad { if let Some(e) = nearest_enemy_champion {
                        tower_score = std::cmp::min(ef.expected_damage_target(ctx, t as &dyn AbstractEntity, e) as i64 * 100 / e.hp as i64, 100); } }
                }
            } }
            let bonus = match action { game_ai::SmallActionPlay::Around(_) => rd::<i64>(action as *const _ as *const u8, 0x20) * hp_value / 200,
                                       game_ai::SmallActionPlay::LaneMinionPosition(_) => rd::<i64>(action as *const _ as *const u8, 0x20), _ => 0 };
            act_score = tower_score - base_score + bonus;
            log += &format!("bs={} ts={} bonus={} ", base_score, tower_score, bonus);
        }
        game_ai::SmallActionPlay::Trace(tr) => {
            let tid: usize = rd(action as *const _ as *const u8, 0x60); target_id = Some(tid);
            let Some(t) = game.get_entity_by_id(tid) else { return (0, log + "trace_notarget") };
            let dive_ignore: bool = rd(action as *const _ as *const u8, 0x94);
            let t_ty: i64 = rd(ep(t), 0x68);
            let same_team = rd::<i64>(ep(t), 0x0) == rd::<i64>(ep(champ), 0x0) && rd::<i64>(ep(t), 0x8) == rd::<i64>(ep(champ), 0x8);
            if version > 1 && t_ty == 13 && !same_team && !matches!(game.get_game_mode(), GameMode::DeathMatch(_)) {
                if sp.v3_turnback_hold { return (-9999999, log + "turnback"); }
                if !(no_self_risk || dive_ignore) {
                    let reach = champ.attack_effect.as_ref().map(|ef| unsafe { line_effect_range_with_radii(ef, champ, t) }).unwrap_or(0);
                    let d = champ.distance(t);
                    log += &format!("reach={} d={} ", reach, d);
                    if d > reach {
                        let mut die: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(ctx.pool);
                        for e in data.cache.iter_champions(enemy) {
                            if dist2(champ, e) < 22500000001 && data.blackboard[enemy].is_recent_visible(game, player, e) { die.push(e); }
                        }
                        if !die.is_empty() {
                            let walk_tick = (d - reach) / std::cmp::max(champ.stat_cached.move_speed as u64, 1);
                            let n = die.len();
                            let my_die = game_ai::check_kill_die_tick(version, rnd, data, player, champ, die, bumpalo::collections::Vec::new_in(ctx.pool), dbg);
                            log += &format!("die_n={} walk={} my_die={} ", n, walk_tick, my_die);
                            if (my_die as u64) <= walk_tick { return (-9999999, log + "dive_die"); }
                        }
                    }
                }
            }
            let pts = action.evaluation_position(version, player, data).map(|(x, y)| game_ai::position_eval_at(version, player, data, x, y, game_ai::PositionEvalPurpose::Trace));
            let base_damage = pts.map(|s| hp_mul * std::cmp::max(s.risk, 0) / 100).unwrap_or(p.risk_damage as i64);
            let tower_possible = if dive_ignore { 0 } else { pts.map(|s| hp_mul * std::cmp::max(s.tower_risk, 0) / 100).unwrap_or(p.risk_possible_tower as i64) / 3 };
            let possible_damage = p.possible_risk(data, 9999) + tower_possible;
            let hp_value = game_ai::champion_hp_value(data, sp, p);
            let (base_score, possible_score) = if no_self_risk { (0, 0) } else { (hp_value * base_damage / hp, possible_damage * hp_value / hp) };
            log += &format!("pts={:?} bd={} tp={} pd={} hpv={} bs={} ps={} nsr={} ", pts.map(|s| (s.risk, s.tower_risk)), base_damage, tower_possible, possible_damage, hp_value, base_score, possible_score, no_self_risk);
            let mut tower_score = 0i64;
            if let Some(t2) = near_ally_tower { if let Some(target) = game.get_entity_by_id(tid) {
                let ef = t2.attack_effect.as_ref().unwrap();
                let ar = tower_attack_range(ef, t2, target);
                if dist2(target, t2) <= ar * ar && game.tick() < tad { if let Some(e) = nearest_enemy_champion {
                    tower_score = std::cmp::min(ef.expected_damage_target(ctx, t2 as &dyn AbstractEntity, e) as i64 * 100 / e.hp as i64, 100); } }
                log += &format!("ally_ar={} ", ar);
            } }
            if !dive_ignore { if let Some(t3) = near_enemy_tower { if let Some(target) = game.get_entity_by_id(tid) {
                let ef = t3.attack_effect.as_ref().unwrap();
                let ar = tower_attack_range(ef, t3, target);
                if dist2(target, t3) <= ar * ar && game.tick() < tad {
                    tower_score -= std::cmp::min(ef.expected_damage_target(ctx, t3 as &dyn AbstractEntity, champ) as i64 * 100 / hp, 100); }
                log += &format!("enemy_ar={} ", ar);
            } } }
            let mut enemy_possible_score = 0i64;
            if let Some(epar) = sp.near_enemies.iter().find(|q| q.id == t.id) {
                let ped = epar.possible_risk(data, 9999) + epar.risk_damage as i64;
                enemy_possible_score = ped * hp_value / hp;
                let ep_player = data.cache.player_by_champion_id(epar.id).unwrap();
                let in_recall = data.blackboard[enemy].in_recall(ep_player.info.position.as_index());
                let mr = game_ai::max_range(champ, t);
                log += &format!("eps={} in_recall={} mr={} ", enemy_possible_score, in_recall, mr);
                if in_recall && t.stat_cached.move_speed + 100 > champ.stat_cached.move_speed && dist2(champ, t) > mr * mr { return (-99999, log + "recall_escape"); }
            } else { log += "no_near_enemy_param "; }
            let mut ne = 0i64; for e in data.cache.player_champion[enemy].iter().flatten() { if dist2(champ, e) < 22500000001 && data.blackboard[enemy].is_recent_visible(game, player, e) { ne += 1; } }
            let mut na = 0i64; for a in data.cache.player_champion[team].iter().flatten() { if dist2(champ, a) < 10000000001 { na += 1; } }
            let diff = ne - na;
            let coef = if diff > 1 { 200 } else { match diff { 1 => 150, 0 => 80, -1 => 60, _ => 30 } };
            act_score = tower_score - (base_score + possible_score) + coef * enemy_possible_score / 100;
            log += &format!("ts={} ne={} na={} coef={} ", tower_score, ne, na, coef);
        }
        game_ai::SmallActionPlay::Attack(_) | game_ai::SmallActionPlay::Skill(_) | game_ai::SmallActionPlay::Skill2(_) | game_ai::SmallActionPlay::Ult(_) => {
            return (i64::MIN, log + "attack_family(internal fastcc 콜리 — 예측 불가)");
        }
        _ => { return (0, log + "default0"); }
    }
    // L965: Around/Trace(/Attack/Skill/Skill2) — 대상 재조회 실패면 0
    if let Some(tid) = target_id { if game.get_entity_by_id(tid).is_none() { return (0, log + "L965"); } }
    (act_score + positioning_score, log)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let kind = a.get(1).cloned().unwrap_or_default();
    if a.len() > 99 { let _ = game_ai::interaction_score as *const (); }
    // argv: kind cx cy hp maxhp e0dx e0dy seen0 e1dx e1dy seen1 vis tick tgt(slot) dive twocall
    let cx = arg(&a, 2, 480000) as u64; let cy = arg(&a, 3, 480000) as u64;
    let hp = arg(&a, 4, -1); let maxhp = arg(&a, 5, -1);
    let e0dx = arg(&a, 6, 900000); let e0dy = arg(&a, 7, 0); let seen0 = arg(&a, 8, 1);
    let e1dx = arg(&a, 9, 900000); let e1dy = arg(&a, 10, 0); let seen1 = arg(&a, 11, 1);
    let vis = arg(&a, 12, 0); let tick = arg(&a, 13, 1000) as usize; let tgt = arg(&a, 14, 0) as usize; let dive = arg(&a, 15, 0); let twocall = arg(&a, 16, 0);
    let version = 2usize;
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
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let player = game.get_player_by_position(0, Position::Top).expect("player");
    let champ = cache.player_champion[0][0].expect("champ");
    let cp = ep(champ);
    wr(cp, 0x660, cx); wr(cp, 0x668, cy);
    if maxhp >= 0 { wr(cp, 0x628, maxhp as usize); }
    if hp >= 0 { wr(cp, 0x670, hp as usize); }
    let e0 = cache.player_champion[1][0].expect("e0"); let e1 = cache.player_champion[1][1].expect("e1");
    wr(ep(e0), 0x660, (cx as i64 + e0dx) as u64); wr(ep(e0), 0x668, (cy as i64 + e0dy) as u64);
    wr(ep(e1), 0x660, (cx as i64 + e1dx) as u64); wr(ep(e1), 0x668, (cy as i64 + e1dy) as u64);
    for s in 2..5usize { if let Some(e) = cache.player_champion[1][s] { wr(ep(e), 0x660, 1u64); wr(ep(e), 0x668, 1u64); } }
    // 아군 나머지는 멀리(ally100_count 판별) — slot 1 만 근처에 둘 수 있게 argv 17
    let ally1 = arg(&a, 17, 900000);
    for s in 1..5usize { if let Some(e) = cache.player_champion[0][s] { wr(ep(e), 0x660, (cx as i64 + if s == 1 { ally1 } else { 900000 }) as u64); wr(ep(e), 0x668, cy); } }
    if seen0 == 1 { bb[1].last_visible[0] = tick; }
    if seen1 == 1 { bb[1].last_visible[1] = tick; }
    if vis == 1 { unsafe { std::ptr::write_volatile(&mut (*(cp as *mut Entity)).visible_state[1], VisibleState::Visible); } }
    let evis = arg(&a, 25, 1);
    if evis == 1 { unsafe { std::ptr::write_volatile(&mut (*(ep(e0) as *mut Entity)).visible_state[0], VisibleState::Visible); std::ptr::write_volatile(&mut (*(ep(e1) as *mut Entity)).visible_state[0], VisibleState::Visible); } }
    let data = OperationData::new(&cache, &ctx, &bb);
    let g: &dyn AbstractGame = &game;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    let mut dbg: DebugFrameData = Default::default();
    let e0dmg = arg(&a, 20, -1); let cdmg = arg(&a, 21, -1);
    if e0dmg >= 0 { set_attack_damage(e0, e0dmg as usize); set_attack_damage(e1, e0dmg as usize); }
    if cdmg >= 0 { set_attack_damage(champ, cdmg as usize); }
    let tdmg = arg(&a, 27, -1);
    if tdmg >= 0 { for t in 0..2usize { for tw in cache.iter_towers_without_nexus(t) { set_attack_damage(tw, tdmg as usize); } } }
    let msp = arg(&a, 28, -1);
    if msp >= 0 { wr(cp, 0x640, msp as usize); wr(ep(e0), 0x640, msp as usize); wr(ep(e1), 0x640, msp as usize); }
    println!("atk	champ.attack_effect={} e0.attack_effect={} champ.level={} ms={}", champ.attack_effect.is_some(), e0.attack_effect.is_some(), champ.level, champ.stat_cached.move_speed);
    let mut sp = game_ai::calculate_score_parameter(version, &mut rnd, player, &data, &mut dbg);
    let ov_risk = arg(&a, 18, -1); let ov_rpt = arg(&a, 19, -1);
    if ov_risk >= 0 { sp.player.risk_damage = ov_risk as usize; }
    if ov_rpt >= 0 { sp.player.risk_possible_tower = ov_rpt as usize; }
    let ov_ap = arg(&a, 22, -1); if ov_ap >= 0 { sp.player.attack_power = ov_ap; }
    let ov_ad = arg(&a, 23, -1); if ov_ad >= 0 { sp.player.applyed_damage = ov_ad as usize; }
    let ov_upb = arg(&a, 24, -1); if ov_upb >= 0 { sp.player.util_power_base = ov_upb; }
    let push_en = arg(&a, 26, 0);
    if push_en >= 1 {
        for (k, e) in [e0, e1].iter().enumerate() { if (k as i64) < push_en {
            let mut c = sp.player.clone(); c.id = e.id; c.team = 1; c.pos = k; c.attack_power = 400 + 100 * k as i64; c.util_power_base = 250; c.risk_damage = 150; c.applyed_damage = 0;
            sp.near_enemies.push(c); } }
        sp.player.attack_power = 450; sp.player.util_power_base = 300; sp.player.attack_value = 800; sp.player.util_value = 300;
        for c in sp.near_enemies.iter_mut() { c.attack_value = 700; c.util_value = 200; }
    }
    println!("hpv	{}	near_enemies={}", game_ai::champion_hp_value(&data, &sp, &sp.player), sp.near_enemies.len());
    // ★TLS 캐시(champion_hp_value 등)가 (seed,tick,pid) 키라 sp 계산 틱과 분리: 판정은 tick+1 에서
    unsafe { (*(&game as *const Game as *mut Game)).set_tick(tick + 1); }
    unsafe { if seen0 == 1 { std::ptr::write_volatile(&bb[1].last_visible[0] as *const usize as *mut usize, tick + 1); }
             if seen1 == 1 { std::ptr::write_volatile(&bb[1].last_visible[1] as *const usize as *mut usize, tick + 1); } }
    let tick = tick + 1;
    println!("world\tseed={}\ttick={}\tpid={}\tchamp=({},{}) hp={} maxhp={}\te0=({},{}) e1=({},{})\tsp.player: applyed={} risk={} rpt={}\tnear_enemies={}\ttowers: ally={:?} enemy={:?}",
             g.seed(), g.tick(), player.info.id, champ.x, champ.y, champ.hp, champ.stat_cached.hp, e0.x, e0.y, e1.x, e1.y,
             sp.player.applyed_damage, sp.player.risk_damage, sp.player.risk_possible_tower, sp.near_enemies.len(),
             cache.iter_towers_without_nexus(0).map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>(), cache.iter_towers_without_nexus(1).map(|t| (t.id, t.x, t.y)).collect::<Vec<_>>());
    let tgt_id = cache.player_champion[1][tgt].map(|e| e.id).unwrap_or(0);
    let action: game_ai::SmallActionPlay = match kind.as_str() {
        "runaway" => game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, tick)),
        "trace" => { let mut t = game_ai::SmallActionTrace::new(&data, tgt_id, tick); if dive == 1 { wr(&t as *const _ as *const u8, 0x94, true); } game_ai::SmallActionPlay::Trace(t) },
        "around" => game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(version, &mut rnd, &data, player, tgt_id, tick)),
        "attack" => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, tgt_id)),
        _ => game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, tick)),
    };
    let ap = &action as *const _ as *const u8;
    println!("action\tkind={}\ttag={}\ttgt_id={}\t+8={}\t+0x60={}\t+0x94={}\t+0x20={}", kind, rd::<i8>(ap, 0xb1), tgt_id, rd::<usize>(ap, 8), rd::<usize>(ap, 0x60), rd::<u8>(ap, 0x94), rd::<i64>(ap, 0x20));
    let before = tls_dump();
    println!("tls_before\tborrow={} seed={} tick={} pid={} v={:?} m={} c={} state={}", before.0, before.1, before.2, before.3, before.4, before.5, before.6, before.7);
    let mut rnd_g = rnd.clone(); let mut rnd_m = rnd.clone();
    let mut dbg_g: DebugFrameData = Default::default(); let mut dbg_m: DebugFrameData = Default::default();
    let gv = game_ai::interaction_score(version, &mut rnd_g, player, &data, &sp, &action, &mut dbg_g);
    let after = tls_dump();
    println!("tls_after\tborrow={} seed={} tick={} pid={} v={:?} m={} c={} state={}", after.0, after.1, after.2, after.3, after.4, after.5, after.6, after.7);
    let mi = my_ictx(g, player, &data, champ);
    let tls_ictx = Ictx { nearest_vis_enemy: if after.4[0] == 1 { Some(after.4[1] as usize) } else { None },
                          near_ally_tower: if after.4[2] == 1 { Some(after.4[3] as usize) } else { None },
                          near_enemy_tower: if after.4[4] == 1 { Some(after.4[5] as usize) } else { None },
                          vis150_nearest: if after.4[6] == 1 { Some(after.4[7] as usize) } else { None },
                          vis150_mask: after.5, ally100_count: after.6 };
    println!("ICTX\tkey_ok={}\tmine={:?}\ttls={:?}\tok={}", after.1 == g.seed() && after.2 as usize == g.tick() && after.3 as usize == player.info.id && after.0 == 0 && after.7 == 1, mi, tls_ictx, mi == tls_ictx);
    let (mv, log) = predict(version, &mut rnd_m, player, &data, &sp, &action, &mut dbg_m, g, &ctx, false);
    let (mv2, _) = predict(version, &mut rnd.clone(), player, &data, &sp, &action, &mut Default::default(), g, &ctx, true);
    let rg = rnd_g.clone().next_u64(); let rm = rnd_m.clone().next_u64(); let r0 = rnd.clone().next_u64();
    println!("RESULT183\tkind={}\tgame={}\tmine={}\tok={}\tmine_maxhp={}\tok_maxhp={}\trnd_changed_game={}\trnd_same_mine={}\tdbg_logs={}\t{}",
             kind, gv, mv, gv == mv, mv2, gv == mv2, rg != r0, rg == rm, dbg_g.logs.len(), log);
    if twocall >= 1 {
        // 같은 (seed,tick,pid) 로 세계를 바꿔 두 번째 호출 → hit 이면 ictx 불변(적 이동 무시), tick 바꾸면 miss
        wr(ep(e0), 0x660, cx + 5000); wr(ep(e0), 0x668, cy);   // e0 를 챔프 옆으로
        unsafe { std::ptr::write_volatile(&bb[1].last_visible[0] as *const usize as *mut usize, tick); }
        let data2 = OperationData::new(&cache, &ctx, &bb);
        let gv2 = game_ai::interaction_score(version, &mut rnd.clone(), player, &data2, &sp, &action, &mut Default::default());
        let t2 = tls_dump();
        println!("TWOCALL_same_key\tgame2={}\ttls_v={:?} m={} c={}\tunchanged={}", gv2, t2.4, t2.5, t2.6, t2.4 == after.4 && t2.5 == after.5);
        unsafe { (*(&game as *const Game as *mut Game)).set_tick(tick + 1); }
        unsafe { std::ptr::write_volatile(&bb[1].last_visible[0] as *const usize as *mut usize, tick + 1); }
        let cache3 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data3 = OperationData::new(&cache3, &ctx, &bb);
        let gv3 = game_ai::interaction_score(version, &mut rnd.clone(), player, &data3, &sp, &action, &mut Default::default());
        let t3 = tls_dump();
        let mi3 = my_ictx(&game as &dyn AbstractGame, player, &data3, champ);
        println!("TWOCALL_tick+1\tgame3={}\ttls tick={} v={:?} m={} c={}\tmine3={:?}\trecomputed={}", gv3, t3.2, t3.4, t3.5, t3.6, mi3, t3.2 as usize == tick + 1 && t3.5 == mi3.vis150_mask);
    }
}
