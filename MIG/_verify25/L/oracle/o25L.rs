#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치L · 192 SerpenCheckSubPlan::action_candidates / 193 JungleSubPlan::action_candidates 오라클 (pub 직접 호출).
//!  명세 `logic` 독립 재구현(predict) ↔ 실행 대조. 원소 대조는 variant 별 **live 바이트 범위**만(sret 잔재는 제외)으로 하고,
//!  live 밖 바이트의 차이도 같이 찍어 잔재 범위를 실측한다. 한 프로세스 = 한 케이스(argv · TLS 메모 콜리 때문).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/L/oracle/o25L.rs
//!  실행: o25L.exe <fn 192|193> k=v ...   (드라이버 = run25L.py)
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::plan_legacy::sub_plan::{SerpenCheckSubPlan, JungleSubPlan};
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::{SmallActionPlay, SmallActionRunAway, SmallActionAroundPosition, SmallActionTrace, SmallActionAttack, SmallActionSkill, SmallActionSkill2, AroundBushOutlineType, PositionEvalPurpose};

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: &Entity) -> *const u8 { e as *const Entity as *const u8 }

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

fn dsq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let dy = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
fn jt(i: i64) -> JungleType { match i { 0 => JungleType::Rhino, 1 => JungleType::Mushroom, 2 => JungleType::Stump, 3 => JungleType::Bee, 4 => JungleType::Morgard, _ => JungleType::Serpen } }
fn jtname(i: i64) -> &'static str { ["Rhino", "Mushroom", "Stump", "Bee", "Morgard", "Serpen"][(i.max(0).min(5)) as usize] }

/// 명세 is_blue_side / is_enemy_side 재구현 (map_regions.rs:7~8 · 58)
/// IR 원문: %78 = icmp ugt (x - y + height), width ; %79 = xor (team==0), %78 ; br %79 → Rhino/경유지 경로. 실측(격자 12점): game_core::is_blue_side == !%78 ⟹ %79 == !is_enemy_side
fn ir_ugt(ctx: &GameContext, x: u64, y: u64) -> bool { x.wrapping_sub(y).wrapping_add(ctx.setting.height) > ctx.setting.width }
fn spec_is_blue(ctx: &GameContext, x: u64, y: u64) -> bool { !ir_ugt(ctx, x, y) }
fn spec_is_enemy_side(ctx: &GameContext, team: usize, x: u64, y: u64) -> bool { if team == 0 { !spec_is_blue(ctx, x, y) } else { spec_is_blue(ctx, x, y) } }
/// 정정된 명세 L20/L26 분기값 = IR %79/%597 = !is_enemy_side
fn ir_branch_not_enemy(ctx: &GameContext, team: usize, x: u64, y: u64) -> bool { (team == 0) ^ ir_ugt(ctx, x, y) }

fn mkeff(range: u64, casting: CastingType) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage: 50, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting }
}

// ───────── 원소 판독 ─────────
fn tag_of(p: *const u8) -> u8 { rd(p, 177) }
fn vname(t: u8) -> &'static str {
    match t { 0 | 1 | 2 => "AroundPosition", 3 => "RunAway", 4 => "Recall", 5 => "Around", 6 => "AroundHide", 7 => "AroundRegion", 8 => "AroundRunAway", 9 => "Positioning",
              11 => "AroundPositionBush", 12 => "AroundBush", 13 => "LaneMinionPosition", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop", _ => "?" }
}
/// variant 별 live 바이트 범위 (생성자 define 의 initializes / 본문 store 실측 + 본 함수의 태그 store 177)
fn live_ranges(t: u8) -> Vec<(usize, usize)> {
    match t {
        3 => vec![(0, 56), (125, 126), (128, 132), (177, 178)],
        14 => vec![(0, 8), (85, 86), (88, 150), (177, 178)],
        15 | 16 | 17 => vec![(0, 17), (177, 178)],
        0 | 1 | 2 => vec![(0, 0x68), (173, 174), (0xb0, 178)],   // AroundPosition::new(m08.ll:103238~) store 집합: [0,104) · +173(Option<PathFinder> None 태그 i8 2 · alloca 69) · +176 i8 6 · +177 outline. [104,173)∪[174,176)∪[178,184) = alloca 잔재
        _ => vec![(0, 184)],
    }
}
fn elem_str(p: *const u8) -> String {
    let t = tag_of(p);
    match t {
        0 | 1 | 2 => format!("{}(goal={},{} tgt={},{} out={} pur={})", vname(t), rd::<u64>(p, 8), rd::<u64>(p, 16), rd::<u64>(p, 32), rd::<u64>(p, 40), t, rd::<u8>(p, 176)),
        3 => format!("RunAway(goal={},{} end={} with_skill={} with_ult={})", rd::<u64>(p, 8), rd::<u64>(p, 16), rd::<usize>(p, 24), rd::<u8>(p, 128), rd::<u8>(p, 129)),
        14 => format!("Trace(target={} end={})", rd::<usize>(p, 0x60), rd::<usize>(p, 0x80)),
        15 | 16 | 17 => format!("{}(target={} is_act={})", vname(t), rd::<usize>(p, 8), rd::<u8>(p, 16)),
        _ => format!("{}(tag={})", vname(t), t),
    }
}
fn vec_elems(v: &bumpalo::collections::Vec<SmallActionPlay>) -> Vec<*const u8> {
    let base = v.as_ptr() as *const u8;
    (0..v.len()).map(|i| unsafe { base.add(i * 184) }).collect()
}
/// game↔mine 원소 대조: live 범위 안 불일치 바이트 / live 밖 불일치 바이트를 따로 센다
fn cmp_elem(g: *const u8, m: *const u8) -> (bool, Vec<usize>, Vec<usize>) {
    let tg = tag_of(g); let tm = tag_of(m);
    if tg != tm { return (false, vec![177], vec![]); }
    let lr = live_ranges(tg);
    let mut bad_live = vec![]; let mut diff_dead = vec![];
    for off in 0..184usize {
        let live = lr.iter().any(|&(a, b)| off >= a && off < b);
        if rd::<u8>(g, off) != rd::<u8>(m, off) { if live { bad_live.push(off) } else { diff_dead.push(off) } }
    }
    (bad_live.is_empty(), bad_live, diff_dead)
}

// ───────── 공통 closure: has_non_target_action_range (serpen_check.rs:35~46 / jungle.rs:114~125 동일 사본) ─────────
fn has_non_target(version: usize, player: &PlayerState, data: &OperationData, champ: &Entity, enemy: usize) -> bool {
    data.cache.player_champion[enemy].iter().flatten().any(|c| {
        if !game_ai::nontarget_windup_perceived(version, player, data, c) { return false; }
        if rd::<i64>(ep(c), 0x68) != 13 { return false; }
        let st = rd::<i64>(ep(c), 0x70);
        let e: &Effect = match st {
            4 => c.skill_effect.as_ref().unwrap(),
            5 => if c.level > 2 { c.skill2_effect.as_ref().unwrap() } else { panic!("static None skill2 unwrap") },
            6 => if c.level > 4 { c.ult_effect.as_ref().unwrap() } else { panic!("static None ult unwrap") },
            _ => return false,
        };
        matches!(e.casting, CastingType::Position | CastingType::Direction) && e.is_in_range(c, champ)
    })
}

// ───────── 192 predict (spec logic 재구현) ─────────
fn predict192<'a>(mc_in: bool, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &'a OperationData, param: &game_ai::ScoreParameter,
                  team_plan: &TeamPlan, log: &mut Vec<String>) -> (bool, bumpalo::collections::Vec<'a, SmallActionPlay>) {
    let ctx = data.context; let bump = ctx.pool;
    let mut res = bumpalo::collections::Vec::new_in(bump);
    let team = player.info.team; let champ = data.cache.player_champion[team][player.info.position as usize].unwrap();
    let mut mc = mc_in;
    if !mc {
        let es = spec_is_enemy_side(ctx, team, champ.x, champ.y);
        let ne = ir_branch_not_enemy(ctx, team, champ.x, champ.y);
        log.push(format!("L20 is_enemy_side spec={} core={} ir_branch(!enemy)={}", es, game_core::is_enemy_side(ctx, team, champ.x, champ.y), ne));
        if ne {
            let camp = ctx.map.camp_pos(JungleType::Rhino, team == 0);
            let d = dsq(champ.x, champ.y, camp.0, camp.1);
            log.push(format!("L25 rhino=({},{}) dsq={} lt4900000001={}", camp.0, camp.1, d, d < 4900000001));
            if d < 4900000001 { mc = true; }
        } else { mc = true; }
    }
    let enemy = 1 - team;
    let hn = has_non_target(version, player, data, champ, enemy);
    let ps = game_ai::position_score_at_position(version, player, data, &param.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective);
    log.push(format!("L50 on_traj={} hn={} on_ptraj={}", ps.on_trajectory, hn, ps.on_periodic_trajectory));
    if ps.on_trajectory || hn || ps.on_periodic_trajectory {
        res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));
        return (mc, res);
    }
    let posture = team_plan.v25_objective_posture(version, player, data, JungleType::Serpen);
    match &posture {
        None => log.push("L58 posture=None".into()),
        Some(p) => {
            let pb = p as *const _ as *const u8;
            let kind = rd::<u8>(pb, 0x50); let nec = rd::<usize>(pb, 0x38); let fe_tag = rd::<i64>(pb, 0); let fe = rd::<usize>(pb, 8);
            let wp = (rd::<u64>(pb, 0x20), rd::<u64>(pb, 0x28));
            log.push(format!("L58 posture=Some kind={} near_enemy={} focus_tag={} focus={} wait={:?}", kind, nec, fe_tag, fe, wp));
            if kind > 2 {
                if kind == 4 && nec != 0 { res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))); }
                res.push(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new(rnd, data, wp.0, wp.1, 5)));
                return (mc, res);
            } else if kind == 2 && fe_tag == 1 {
                res.push(SmallActionPlay::Trace(SmallActionTrace::new_attack_range(data, fe, 5)));
            }
        }
    }
    let camp = ctx.map.camp_pos(JungleType::Serpen, team == 0);
    let d = dsq(champ.x, champ.y, camp.0, camp.1);
    log.push(format!("L78 serpen=({},{}) dsq={} gt22500000000={} mc={}", camp.0, camp.1, d, d > 22500000000, mc));
    if d > 22500000000 {
        if mc { res.push(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5))); }
        else { let r = ctx.map.camp_pos(JungleType::Rhino, team == 0); res.push(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new(rnd, data, r.0, r.1, 5))); }
    } else { res.push(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5))); }
    let game: &dyn AbstractGame = data.cache.game;
    let danger = data.cache.player_champion[enemy].iter().flatten().any(|c| data.blackboard[enemy].is_recent_visible(game, player, c) && dsq(c.x, c.y, champ.x, champ.y) < 22500000001)
        || data.cache.others[enemy].iter().any(|e| dsq(e.x, e.y, champ.x, champ.y) < 22500000001);
    log.push(format!("L94 danger={} others_len={}", danger, data.cache.others[enemy].len()));
    if danger { res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5))); }
    let vis = game.is_visible(enemy, champ.id);
    log.push(format!("L98 is_visible={}", vis));
    if vis { let b = game_ai::battle_action(version, rnd, player, data, 5); log.push(format!("L99 battle_action n={}", b.len())); res.extend(b); }
    let s = game_ai::attack_summon_action(player, data); log.push(format!("L101 summon n={}", s.len())); res.extend(s);
    (mc, res)
}

// ───────── 193 predict ─────────
fn range_sum(eff: &Effect, champ: &Entity, target: &Entity, ms30: u64) -> u64 {
    eff.range.wrapping_add(ms30).wrapping_add(champ.stat_buff_cached.range as u64).wrapping_add((champ.level as u64).wrapping_sub(1).wrapping_mul(eff.growth_range))
        .wrapping_add(eff.range_adjust(champ, target)).wrapping_add(champ.radius() as u64).wrapping_add(target.radius() as u64)
}
fn predict193<'a>(s_team: usize, s_camp: JungleType, cm_in: bool, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &'a OperationData,
                  param: &game_ai::ScoreParameter, log: &mut Vec<String>) -> (bool, bumpalo::collections::Vec<'a, SmallActionPlay>) {
    let ctx = data.context; let bump = ctx.pool;
    let mut res = bumpalo::collections::Vec::new_in(bump);
    let team = player.info.team; let champ = data.cache.player_champion[team][player.info.position as usize].unwrap();
    let enemy = 1 - team;
    let hn = has_non_target(version, player, data, champ, enemy);
    let ps = game_ai::position_score_at_position(version, player, data, &param.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective);
    log.push(format!("L129 on_traj={} hn={} on_ptraj={}", ps.on_trajectory, hn, ps.on_periodic_trajectory));
    if ps.on_trajectory || hn || ps.on_periodic_trajectory {
        res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));
        return (cm_in, res);
    }
    let b = game_ai::battle_action(version, rnd, player, data, 5); log.push(format!("L136 battle n={}", b.len())); res.extend(b);
    let s = game_ai::attack_summon_action(player, data); log.push(format!("L137 summon n={}", s.len())); res.extend(s);
    let game: &dyn AbstractGame = data.cache.game;
    let camp = ctx.map.camp_pos(s_camp, s_team == 0);
    let d = dsq(champ.x, champ.y, camp.0, camp.1);
    let c1 = d < 22500000001;
    let c2 = if c1 { false } else { game.is_visible(enemy, champ.id) };
    let c3 = if c1 || c2 { false } else { data.cache.jungles.iter().any(|e| dsq(e.x, e.y, champ.x, champ.y) < 22500000001) };
    log.push(format!("L142 camp=({},{}) dsq={} near={} vis={} jungles_near={} jungles_len={}", camp.0, camp.1, d, c1, c2, c3, data.cache.jungles.len()));
    if c1 || c2 || c3 { res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5))); }
    // L147 attack_jungle_action 인라인
    {
        let mode = game.get_game_mode();
        let m = match mode { GameMode::Moba(m) => m, _ => panic!("non-moba") };
        let cs = m.jungle_runner.get_camp_state(s_team, s_camp);
        let mut sub = bumpalo::collections::Vec::new_in(bump);
        let ms30 = (champ.stat_cached.move_speed as u64).wrapping_mul(30);
        log.push(format!("L62 live_list={:?} ms30={}", cs.live_list, ms30));
        for id in cs.live_list.iter() {
            let target = match game.get_entity_by_id(*id) { Some(t) => t, None => continue };
            if champ.can_attack() {
                let atk = champ.attack_effect.as_ref().unwrap();
                let mx = range_sum(atk, champ, target, ms30);
                let dd = dsq(target.x, target.y, champ.x, champ.y);
                log.push(format!("  L71 id={} atk max={} dsq={} push={}", id, mx, dd, dd <= mx.wrapping_mul(mx)));
                if dd <= mx.wrapping_mul(mx) { sub.push(SmallActionPlay::Attack(SmallActionAttack::new(data, target.id))); }
            }
            if let Some(sk) = champ.skill_effect.as_ref() {
                if champ.can_skill() && sk.target.check(champ, target) {
                    let mx = range_sum(sk, champ, target, ms30);
                    let dd = dsq(target.x, target.y, champ.x, champ.y);
                    log.push(format!("  L84 id={} skill max={} dsq={} push={}", id, mx, dd, dd <= mx.wrapping_mul(mx)));
                    if dd <= mx.wrapping_mul(mx) { sub.push(SmallActionPlay::Skill(SmallActionSkill::new(data, target.id))); }
                }
            }
            let sk2: Option<&Effect> = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None };
            if let Some(sk2) = sk2 {
                if champ.can_skill2() && sk2.target.check(champ, target) {
                    let mx = range_sum(sk2, champ, target, ms30);
                    let dd = dsq(target.x, target.y, champ.x, champ.y);
                    log.push(format!("  L97 id={} skill2 max={} dsq={} push={}", id, mx, dd, dd <= mx.wrapping_mul(mx)));
                    if dd <= mx.wrapping_mul(mx) { sub.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, target.id))); }
                }
            }
        }
        res.extend(sub);
    }
    // L148 move_action 인라인
    let mut cm = cm_in;
    let camp2 = ctx.map.camp_pos(s_camp, s_team == 0);
    let mv = if team == s_team || cm {
        log.push(format!("L22 direct (player.team==self.team={} check_move={})", team == s_team, cm));
        SmallActionAroundPosition::new(rnd, data, camp2.0, camp2.1, 5)
    } else {
        let champ2 = data.cache.player_champion[team][player.info.position as usize].unwrap();
        let es = spec_is_enemy_side(ctx, team, champ2.x, champ2.y);
        let ne = ir_branch_not_enemy(ctx, team, champ2.x, champ2.y);
        log.push(format!("L26 is_enemy_side spec={} core={} ir_branch(!enemy)={}", es, game_core::is_enemy_side(ctx, team, champ2.x, champ2.y), ne));
        let mut out: Option<SmallActionAroundPosition> = None;
        if ne {
            let ci = s_camp as u8;
            if ci.wrapping_sub(1) < 2 {
                let (ex, ey) = ctx.map.camp_pos(JungleType::Morgard, team == 0);
                let d = dsq(champ2.x, champ2.y, ex, ey);
                log.push(format!("L33 morgard=({},{}) dsq={} gt1e10={}", ex, ey, d, d > 10000000000));
                if d > 10000000000 { out = Some(SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, 5, AroundBushOutlineType::Outline)); }
            } else {
                let (ex, ey) = ctx.map.camp_pos(JungleType::Serpen, team == 0);
                let d = dsq(champ2.x, champ2.y, ex, ey);
                log.push(format!("L41 serpen=({},{}) dsq={} gt1e10={}", ex, ey, d, d > 10000000000));
                if d > 10000000000 { out = Some(SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, 5, AroundBushOutlineType::Outline)); }
            }
        }
        match out { Some(o) => o, None => { cm = true; SmallActionAroundPosition::new(rnd, data, camp2.0, camp2.1, 5) } }
    };
    res.push(SmallActionPlay::AroundPosition(mv));
    (cm, res)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() > 99 { let _ = game_ai::position_eval_at as *const (); }
    let which: i64 = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut m = HashMap::new();
    for s in a.iter().skip(2) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let args = Args { m };
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: args.get("dbg", 0) != 0,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let ticks = args.get("ticks", 0) as usize;
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(11);
    for _ in 0..ticks { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    if args.has("tick") { game.set_tick(args.get("tick", 100) as usize); }
    let pteam0 = args.get("pteam", 0) as usize; let ppos0 = args.get("ppos", 1) as usize;
    if args.has("post") { // 좌표 세팅 후 틱을 더 돌려 가시성(is_visible) 갱신
        { let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx); let ch = c0.player_champion[pteam0][ppos0].expect("champ");
          if args.has("at") { let (cx, cy) = map.camp_pos(jt(args.get("at", 0)), args.get("side", 1) != 0);
              wr(ep(ch), 0x660, (cx as i64 + args.get("dx", 0)) as u64); wr(ep(ch), 0x668, (cy as i64 + args.get("dy", 0)) as u64); }
          else if args.has("x") { wr(ep(ch), 0x660, args.get("x", 0) as u64); wr(ep(ch), 0x668, args.get("y", 0) as u64); } }
        for _ in 0..(args.get("post", 1) as usize) { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let pteam = args.get("pteam", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let ppos = args.get("ppos", 1) as usize;
    let player = game.get_player_by_position(pteam, poss[ppos]).expect("player");
    let champ = cache.player_champion[pteam][ppos].expect("champ");
    // 좌표 세팅: at=<jt> side=<0|1> dx dy 또는 x y
    if args.has("at") {
        let (cx, cy) = map.camp_pos(jt(args.get("at", 0)), args.get("side", 1) != 0);
        let x = (cx as i64 + args.get("dx", 0)) as u64; let y = (cy as i64 + args.get("dy", 0)) as u64;
        wr(ep(champ), 0x660, x); wr(ep(champ), 0x668, y);
    } else if args.has("x") { wr(ep(champ), 0x660, args.get("x", 0) as u64); wr(ep(champ), 0x668, args.get("y", 0) as u64); }
    if args.has("eff") { // 공격 이펙트 주입(사거리)
        let e = mkeff(args.get("eff", 30000) as u64, CastingType::Targeting);
        let ptr = (ep(champ) as usize + 0x490) as *mut Option<Effect>; // attack_effect: Option<Effect>(56B, casting 니치) 선두 = Effect.ty@+0x490
        unsafe { std::ptr::write_volatile(ptr, Some(e)); }
    }
    if args.has("lvl") { wr(ep(champ), 0x5c8, args.get("lvl", 1) as u64); }
    if args.has("eff2") { // skill2_effect 주입 (Option<Effect> @0x500, casting 니치 @0x530)
        let e = mkeff(args.get("eff2", 30000) as u64, CastingType::Targeting);
        let ptr = (ep(champ) as usize + 0x500) as *mut Option<Effect>;
        unsafe { std::ptr::write_volatile(ptr, Some(e)); }
    }
    if args.has("nt") { // 적 챔프(enemy team pos ntp, 기본 0) 를 논타겟 윈드업 상태로: action_state@0x70 · 시작틱@0x78 · skill_effect@0x4c8(casting@0x4f8) · level@0x5c8
        let et = 1 - pteam; let ntp = args.get("ntp", 0) as usize;
        let en = cache.player_champion[et][ntp].expect("enemy champ");
        let st = args.get("nt", 4) as i64;                       // 4=Skill 5=Skill2 6=Ult
        wr(ep(en), 0x70, st); wr(ep(en), 0x78, args.get("ntstart", 0) as i64);
        if args.has("ntlvl") { wr(ep(en), 0x5c8, args.get("ntlvl", 1) as u64); }
        let cast = match args.get("ntcast", 1) { 0 => CastingType::Targeting, 1 => CastingType::Position, 2 => CastingType::Direction, _ => CastingType::None };
        let e = mkeff(args.get("ntrange", 300000) as u64, cast);
        let off = match st { 5 => 0x500usize, 6 => 0x538, _ => 0x4c8 };
        if args.get("ntinj", 1) != 0 { unsafe { std::ptr::write_volatile((ep(en) as usize + off) as *mut Option<Effect>, Some(e)); } }
        if args.has("ex") { wr(ep(en), 0x660, args.get("ex", 0) as u64); wr(ep(en), 0x668, args.get("ey", 0) as u64); }
        println!("ntinj	enemy id={} ({},{}) state={} lvl={} cast_tag={}", en.id, en.x, en.y, rd::<i64>(ep(en), 0x70), en.level, rd::<i32>(ep(en), off + 0x30));
    }
    let version = args.get("ver", 55) as usize;
    let gtick = (&game as &dyn AbstractGame).tick();
    println!("world\ttick={}\tpteam={}\tppos={}\tchamp id={} x={} y={} lvl={} ms={} radius={} can_attack={} atk_some={} skill_some={}",
        gtick, pteam, ppos, champ.id, champ.x, champ.y, champ.level, champ.stat_cached.move_speed, champ.radius(), champ.can_attack(), champ.attack_effect.is_some(), champ.skill_effect.is_some());
    if which == 0 {
        println!("setting	width={} height={}", setting.width, setting.height);
        for (x, y) in [(0u64, 0u64), (100, 0), (0, 100), (500000, 400000), (400000, 500000), (672000, 672000), (672001, 672000), (672000, 672001), (32000, 928000), (928000, 32000), (960000, 0), (0, 960000), (960000, 960000)] {
            let ugt = ir_ugt(&ctx, x, y);
            println!("blue_grid	({},{})	ugt(x-y+h>w)={}	core_blue={}	spec_formula_blue(=ugt)={}	core_es0={}	core_es1={}	ir_br_t0(=t0^ugt)={}", x, y, ugt, game_core::is_blue_side(&ctx, x, y), ugt, game_core::is_enemy_side(&ctx, 0, x, y), game_core::is_enemy_side(&ctx, 1, x, y), ir_branch_not_enemy(&ctx, 0, x, y));
        }
        println!("is_visible	t1 sees champ={}	t0 sees champ={}", (&game as &dyn AbstractGame).is_visible(1, champ.id), (&game as &dyn AbstractGame).is_visible(0, champ.id));
        for side in [true, false] { for i in 0..6 { let c = map.camp_pos(jt(i), side); println!("camp\t{}\tblue={}\t({},{})\tenemy_side_t0={}\tenemy_side_t1={}", jtname(i), side, c.0, c.1, game_core::is_enemy_side(&ctx, 0, c.0, c.1), game_core::is_enemy_side(&ctx, 1, c.0, c.1)); } }
        for t in 0..2 { for p in 0..5 { if let Some(e) = cache.player_champion[t][p] { println!("champ\tt={} p={} id={} ({},{}) enemy_side={} blue={}", t, p, e.id, e.x, e.y, game_core::is_enemy_side(&ctx, t, e.x, e.y), game_core::is_blue_side(&ctx, e.x, e.y)); } } }
        println!("others\t{} {}\tjungles\t{}", cache.others[0].len(), cache.others[1].len(), cache.jungles.len());
        for e in cache.jungles.iter() { println!("jungle\tid={} ({},{}) ty={}", e.id, e.x, e.y, rd::<i64>(ep(e), 0x68)); }
        if let GameMode::Moba(mm) = (&game as &dyn AbstractGame).get_game_mode() {
            for t in 0..2 { for i in 0..4 { let cs = mm.jungle_runner.get_camp_state(t, jt(i)); println!("camp_state\tt={} {}\tlive={:?}\tblue={} ty={:?}", t, jtname(i), cs.live_list, cs.is_blue_side, cs.ty); } }
        }
        println!("is_visible\tt1 sees champ={}\tt0 sees champ={}", (&game as &dyn AbstractGame).is_visible(1, champ.id), (&game as &dyn AbstractGame).is_visible(0, champ.id));
        if args.has("cs") { // get_camp_state(cst, cs) 단건 프로브(Serpen/Morgard 패닉 여부)
            if let GameMode::Moba(mm) = (&game as &dyn AbstractGame).get_game_mode() {
                let t = args.get("cst", 0) as usize; let c = jt(args.get("cs", 5));
                let cs = mm.jungle_runner.get_camp_state(t, c);
                println!("camp_state_probe\tt={} {}\tlive={:?}\tblue={} ty={:?}", t, jtname(args.get("cs", 5)), cs.live_list, cs.is_blue_side, cs.ty);
            }
        }
        return;
    }
    // ScoreParameter 조립(positioning_score 만 읽힌다)
    let csp = game_ai::ChampionScoreParameter { action: SmallAction::Stop, risk_possible: bumpalo::collections::Vec::new_in(&pool), gain_possible: bumpalo::collections::Vec::new_in(&pool),
        id: champ.id, team: pteam, pos: ppos, applyed_damage: 0, applyed_cc: 0, risk_damage: 0, risk_epic_damage: 0, risk_cc: 0, risk_possible_tower: 0, action_time: 0,
        attack_value: 0, util_value: 0, attack_power: 0, util_power_base: 0, cc_time_x_inv_cd: 0, buff_inv_cd_count: 0 };
    let param = game_ai::ScoreParameter { wave_snapshot: None, player: csp, positioning_score: Default::default(),
        near_allies: bumpalo::collections::Vec::new_in(&pool), near_enemies: bumpalo::collections::Vec::new_in(&pool), version, v3_turnback_hold: false };
    let mut r_game = rand::rngs::StdRng::seed_from_u64(5); let mut r_mine = r_game.clone();
    let mut log: Vec<String> = vec![];
    let (gv, mv, self_g, self_m): (bumpalo::collections::Vec<SmallActionPlay>, bumpalo::collections::Vec<SmallActionPlay>, String, String);
    if which == 192 {
        let mc = args.get("mc", 0) != 0;
        let mut sp: SerpenCheckSubPlan = Default::default();
        wr(&sp as *const _ as *const u8, 0, mc as u8);
        let tp: TeamPlan = Default::default();
        if args.has("obj") { // TeamPlan.objective = Some(MainObjective::Serpen{phase, with_battle}) : 0x41f 태그(1=Serpen) · 0x420 phase · 0x421 with_battle
            let tb = &tp as *const _ as *const u8;
            wr(tb, 0x41f, args.get("obj", 1) as u8); wr(tb, 0x420, args.get("oph", 0) as u8); wr(tb, 0x421, args.get("owb", 0) as u8);
        }
        let mut dbg: DebugFrameData = Default::default();
        // ★predict 를 먼저 돌리면 TLS(POS_EVAL_CACHE·CAMP_POS_MEMO)가 predict 값으로 채워진다 — 순서: game 먼저.
        gv = sp.action_candidates(version, &mut r_game, player, &data, &param, &tp, &mut dbg);
        self_g = format!("move_check={}", rd::<u8>(&sp as *const _ as *const u8, 0));
        let (mcm, v) = predict192(mc, version, &mut r_mine, player, &data, &param, &tp, &mut log);
        mv = v; self_m = format!("move_check={}", mcm as u8);
        println!("dbg_infos\t{}", dbg.infos.len());
    } else {
        let st = args.get("steam", 1) as usize; let sc = jt(args.get("scamp", 0)); let cm = args.get("cm", 0) != 0;
        let mut sp = JungleSubPlan::new(st, sc);
        wr(&sp as *const _ as *const u8, 9, cm as u8);
        gv = sp.action_candidates(version, &mut r_game, player, &data, &param);
        self_g = format!("check_move={} team={} camp={}", rd::<u8>(&sp as *const _ as *const u8, 9), rd::<usize>(&sp as *const _ as *const u8, 0), rd::<u8>(&sp as *const _ as *const u8, 8));
        let (cmm, v) = predict193(st, sc, cm, version, &mut r_mine, player, &data, &param, &mut log);
        mv = v; self_m = format!("check_move={} team={} camp={}", cmm as u8, st, sc as u8);
    }
    for l in &log { println!("pred\t{}", l); }
    let ge = vec_elems(&gv); let me = vec_elems(&mv);
    println!("game\tn={}\t{}\t[{}]", gv.len(), self_g, ge.iter().map(|&p| elem_str(p)).collect::<Vec<_>>().join(" | "));
    println!("mine\tn={}\t{}\t[{}]", mv.len(), self_m, me.iter().map(|&p| elem_str(p)).collect::<Vec<_>>().join(" | "));
    let mut all_ok = gv.len() == mv.len() && self_g == self_m;
    for i in 0..gv.len().min(mv.len()) {
        let (ok, bad, dead) = cmp_elem(ge[i], me[i]);
        println!("elem[{}]\t{}\tlive_ok={}\tbad_live={:?}\tdead_diff={:?}", i, vname(tag_of(ge[i])), ok, bad, dead);
        if !ok { all_ok = false; }
    }
    println!("rng_same_after={}", r_game.gen::<u64>() == r_mine.gen::<u64>());
    println!("RESULT\t{}", if all_ok { "MATCH" } else { "MISMATCH" });
}
