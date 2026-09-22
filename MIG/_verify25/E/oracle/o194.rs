#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치E · 194 HideSubPlan::action_candidates 오라클 (pub 직접 호출) — 명세 `logic` 독립 재구현(predict) ↔ 실행 대조.
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(real_setting). rnd 소비는 같은 시드 복제본으로
//!  wait_around(12칸 choose)/AroundBush(cell 목록 choose) 를 같은 순서로 불러 next_u64 로 대조.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/E/oracle/o194.rs
//!  실행: o194.exe k=v ...   (드라이버 = run194.py)
//!   bushi=<distinct bush 순번> cm=<check_move 0/1> esm=<enemy_spotted_me 초기 0/1> ol=<out_line 0..2>
//!   at_camp=1 (챔프를 목표 캠프 좌표로 이동) vis=1 (champ.visible_state[enemy]=Visible)
//!   near=1 (적 챔프(1-team, npos)를 부시 좌표 근처로 이동) lv=<blackboard last_visible 세팅 tick, -1=안함>
//!   evis=1 (그 적 챔프의 visible_state[team]=Visible → is_visible_from(champ) 참) nearc=1 (적을 champ 150000 안으로)
//!   jhp0=1 (부시 150000 안 적진영 정글 hp=0) tick=<set_tick> team= pos=
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use rand::seq::SliceRandom;
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } }

fn dist_sq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 { let dx = x1.abs_diff(x2); let dy = y1.abs_diff(y2); dx * dx + dy * dy }

fn tagname(t: u8) -> &'static str {
    match t { 3 => "RunAway", 4 => "Recall", 5 => "Around", 6 => "AroundHide", 7 => "AroundRegion", 8 => "AroundRunAway", 9 => "Positioning",
        11 => "AroundPositionBush", 12 => "AroundBush", 13 => "LaneMinionPosition", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2",
        18 => "Ult", 19 => "Stop", 0 | 1 | 2 => "AroundPosition(untagged)", _ => "?" }
}

fn mkeff(damage: usize, start_timing: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 100000, growth_range: 0, start_timing,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

/// 예측 원소: (태그, 확정 바이트들 (off,val))
struct Pred { tag: u8, bytes: Vec<(usize, u64, u8)> } // (off, value, size)

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::can1v1win as *const (); }
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m };
    let mut setting = real_setting();
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
    let nticks = a.get("ticks", 0) as usize;
    if nticks > 0 { let mut rnd0 = rand::rngs::StdRng::seed_from_u64(9); for _ in 0..nticks { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); } }
    let tick = a.get("tick", 1000) as usize;
    game.set_tick(tick);
    let version = a.get("version", 55) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let team = a.get("team", 0) as usize; let et = 1 - team;
    let posi = a.get("pos", 1) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let champ = cache.player_champion[team][posi].expect("champ");
    let cp = ep(champ);
    println!("champ id={} team={} pos={} xy=({},{}) tick={} jungles={}", champ.id, team, posi, champ.x, champ.y, game.tick(), cache.jungles.len());

    // ── bush 목록(BUSH_POSITIONS 순서 · distinct id · 첫 셀) ──
    let mut ids: Vec<(usize, usize, usize)> = Vec::new(); // (id, x, y)
    for &(x, y) in BUSH_POSITIONS.iter() { let id = map.bushes[y][x]; if !ids.iter().any(|t| t.0 == id) { ids.push((id, x, y)); } }
    println!("distinct bushes={} first={:?}", ids.len(), &ids[..ids.len().min(12)]);
    let bushi = a.get("bushi", 0) as usize;
    let (bush, bx_c, by_c) = ids[bushi % ids.len()];
    let bx = bx_c as u64 * 32000 + 16000; let by = by_c as u64 * 32000 + 16000;
    let top = is_top_side(&ctx, bx, by);
    let cells: Vec<(usize, usize)> = BUSH_POSITIONS.iter().copied().filter(|&(x, y)| map.bushes[y][x] == bush).collect();
    println!("bush id={} cell=({},{}) world=({},{}) top={} cells={}", bush, bx_c, by_c, bx, by, top, cells.len());

    // ── 세계 조작 ──
    let (ex, ey) = map.camp_pos(if top { JungleType::Morgard } else { JungleType::Serpen }, team == 0);
    println!("camp=({},{}) champ_dist_sq={} > 1e10? {}", ex, ey, dist_sq(champ.x, champ.y, ex, ey), dist_sq(champ.x, champ.y, ex, ey) > 10000000000);
    if a.get("at_camp", 0) == 1 { wr(cp, 0x660, ex); wr(cp, 0x668, ey); }
    if a.get("at_camp_d", -1) >= 0 { wr(cp, 0x660, ex + a.get("at_camp_d", 0) as u64); wr(cp, 0x668, ey); }
    if a.get("at_bush", 0) == 1 { wr(cp, 0x660, bx); wr(cp, 0x668, by + 40000); }
    if a.get("vis", 0) == 1 { wr::<i64>(cp, 0x38 + 24 * et, 0); }
    let npos = a.get("npos", 0) as usize;
    let enemy = cache.player_champion[et][npos].expect("enemy");
    let np = ep(enemy);
    if a.get("near", 0) == 1 { wr(np, 0x660, bx + 60000); wr(np, 0x668, by); }
    if a.get("near_d", -1) >= 0 { wr(np, 0x660, bx + a.get("near_d", 0) as u64); wr(np, 0x668, by); }
    if a.get("nearc", 0) == 1 { let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668); wr(np, 0x660, cx + 50000); wr(np, 0x668, cy); }
    if a.get("nearc_d", -1) >= 0 { let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668); wr(np, 0x660, cx + a.get("nearc_d", 0) as u64); wr(np, 0x668, cy); }
    if a.get("evis", 0) == 1 { wr::<i64>(np, 0x38 + 24 * team, 0); }
    let lv = a.get("lv", -1);
    if lv >= 0 { bb[et].last_visible[npos] = lv as usize; }
    if a.get("pj", 0) == 1 { for j in cache.jungles.iter() { let jp = ep(*j); let ty: i64 = rd(jp, 0x68); let ct: usize = rd(jp, 0x98); println!("  jungle id={} ty={} camp_team={} xy=({},{}) hp={} d2bush={}", j.id, ty, ct, j.x, j.y, j.hp, dist_sq(bx, by, j.x, j.y)); } }
    let mut jhp_id: Option<usize> = None;
    if a.get("jhp0", 0) == 1 {
        for j in cache.jungles.iter() {
            let jp = ep(*j);
            let ty: i64 = rd(jp, 0x68); let camp_team: usize = rd(jp, 0x98);
            if ty == 4 && camp_team == et && dist_sq(bx, by, j.x, j.y) <= 22500000000 { wr(jp, 0x670, 0usize); jhp_id = Some(j.id); if a.get("at_j", 0) == 1 { wr(cp, 0x660, j.x + 15000); wr(cp, 0x668, j.y); } break; }
        }
        println!("jhp0 target={:?}", jhp_id);
    }
    if a.get("cdmg", -1) >= 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(a.get("cdmg", 0) as usize, 0))); } }
    if a.get("edmg", -1) >= 0 { for s_ in 0..5 { if let Some(e) = cache.player_champion[et][s_] { unsafe { std::ptr::write(&mut (*(ep(e) as *mut Entity)).attack_effect, Some(mkeff(a.get("edmg", 0) as usize, 0))); } } } }
    if a.get("ehp", -1) >= 0 { wr(np, 0x670, a.get("ehp", 0) as usize); }
    if a.get("chp", -1) >= 0 { wr(cp, 0x670, a.get("chp", 0) as usize); }
    let vs_c: [i64; 2] = [rd(cp, 0x38), rd(cp, 0x38 + 24)];
    let vs_e: [i64; 2] = [rd(np, 0x38), rd(np, 0x38 + 24)];
    println!("champ.visible_state tags={:?} enemy.visible_state tags={:?} enemy xy=({},{})", vs_c, vs_e, enemy.x, enemy.y);
    let data = OperationData::new(&cache, &ctx, &bb);
    let is_vis = (&game as &dyn AbstractGame).is_visible(et, champ.id);
    println!("is_visible_to_enemy={} can_attack={} atk={} can_skill={} sk={} can_skill2={} sk2={} ms={}", is_vis, champ.can_attack(), champ.attack_effect.is_some(), champ.can_skill(), champ.skill_effect.is_some(), champ.can_skill2(), champ.skill2_effect().is_some(), champ.stat_cached.move_speed);

    // ── self 조립 ──
    let ol = a.get("ol", 1) as u8;
    let olt = match ol { 0 => game_ai::AroundBushOutlineType::None, 2 => game_ai::AroundBushOutlineType::Inline, _ => game_ai::AroundBushOutlineType::Outline };
    let mut hs = game_ai::plan_legacy::sub_plan::HideSubPlan::new(bush, olt);
    let hp = &mut hs as *mut _ as *const u8;
    let cm0 = a.get("cm", 0) as u8; let esm0 = a.get("esm", 0) as u8;
    wr::<u8>(hp, 9, cm0); wr::<u8>(hp, 10, esm0);
    for k in 11..16 { wr::<u8>(hp, k, 0xA0 + k as u8); } // 패딩 카나리
    let self_before: [u8; 16] = rd(hp, 0);
    println!("self_before={:?}", self_before);

    // ScoreParameter 미사용(IR %6 define 외 출현 0) — 0 버퍼. DebugFrameData readnone — Default.
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
    let mut dbgf: DebugFrameData = Default::default();

    let seed = a.get("seed", 11) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd_p = rand::rngs::StdRng::seed_from_u64(seed); // predict 용 복제본

    // ══ predict (명세 logic 독립 재구현) ══
    let mut preds: Vec<Pred> = Vec::new();
    let mut p_cm = cm0; let mut p_esm = esm0;
    let mut plog = String::new();
    let team_c = champ; // champ
    // L26~30
    if is_vis { p_esm = 1; } else if p_cm == 1 { p_esm = 0; }
    // L34~54
    if p_cm == 0 {
        let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668);
        if dist_sq(cx, cy, ex, ey) > 10000000000 {
            // AroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline)
            let wa = game_ai::wait_around(&mut rnd_p, ex, ey, 60000);
            let wap = &wa as *const _ as *const u8;
            let mut b = vec![(0usize, game.tick() as u64, 8u8), (8, ex, 8), (16, ey, 8), (24, 0, 8), (32, ex, 8), (40, ey, 8), (88, 80000, 8), (96, 5, 8), (173, 2, 1), (176, 6, 1), (177, 1, 1)];
            for k in 0..5 { b.push((48 + k * 8, rd::<u64>(wap, k * 8), 8)); }
            preds.push(Pred { tag: 1, bytes: b });
            plog += "L43/50 AroundPosition ";
        } else { p_cm = 1; plog += "L45/52 check_move=1 "; }
    }
    // L57~101
    if p_cm == 1 {
        // nearest_enemy (L61~63)
        let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668);
        let mut best: Option<(u64, &Entity)> = None;
        for slot in 0..5 { if let Some(e) = cache.player_champion[et][slot] {
            if bb[et].is_recent_visible(&game as &dyn AbstractGame, player, e) && dist_sq(e.x, e.y, bx, by) < 62500000001 {
                let k = dist_sq(e.x, e.y, cx, cy);
                if best.map_or(true, |(bk, _)| k < bk) { best = Some((k, e)); }
            } } }
        plog += &format!("nearest={:?} ", best.map(|(k, e)| (e.id, k)));
        if is_vis {
            if let Some((_, en)) = best {
                let allies = (0..5).filter(|&s| cache.player_champion[team][s].map_or(false, |e| e.id != champ.id && dist_sq(e.x, e.y, cx, cy) < 22500000001)).count();
                let enemies = (0..5).filter(|&s| cache.player_champion[et][s].map_or(false, |e| e.is_visible_from(champ) && dist_sq(e.x, e.y, cx, cy) < 22500000001)).count();
                let dominated = allies < enemies;
                let mut rtmp = rnd_p.clone();
                let can_fight = !dominated && game_ai::can1v1win(&mut rtmp, player, &data, champ, en);
                plog += &format!("L70 allies={} enemies={} dominated={} can_fight={} ", allies, enemies, dominated, can_fight);
                if can_fight {
                    let v = game_ai::battle_action(version, &mut rtmp, player, &data, 5);
                    for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); }
                    plog += &format!("L80 battle_action n={} ", v.len());
                    let t = (&game as &dyn AbstractGame).get_entity_by_id(en.id);
                    let (gx, gy) = t.map_or((0, 0), |t| (t.x, t.y));
                    preds.push(Pred { tag: 14, bytes: vec![(0, 0, 8), (85, 2, 1), (88, game.tick() as u64, 8), (96, en.id as u64, 8), (104, gx, 8), (112, gy, 8), (120, 15000, 8), (128, 5, 8), (136, 0, 8), (144, 0, 1), (145, 0, 1), (146, 0, 1), (147, 0, 1), (148, 0, 1), (149, 2, 1), (177, 14, 1)] });
                } else {
                    preds.push(Pred { tag: 3, bytes: vec![(0x18, 5, 8), (125, 2, 1), (128, 0, 1), (177, 3, 1)] });
                    plog += "L84 RunAway ";
                    if !dominated { let v = game_ai::battle_action(version, &mut rtmp, player, &data, 5); for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); } plog += &format!("L87 battle_action n={} ", v.len()); }
                }
            } else {
                preds.push(Pred { tag: 3, bytes: vec![(0x18, 5, 8), (125, 2, 1), (128, 0, 1), (177, 3, 1)] });
                plog += "L91 RunAway ";
            }
        } else {
            if let Some((_, en)) = best {
                preds.push(Pred { tag: 12, bytes: vec![(0, game.tick() as u64, 8), (0x10, bush as u64, 8), (109, 2, 1), (112, ol as u64, 1), (177, 12, 1)] });
                plog += "L94 AroundBush(target) ";
            } else {
                let chosen = cells.choose(&mut rnd_p).copied().unwrap();
                preds.push(Pred { tag: 12, bytes: vec![(0, game.tick() as u64, 8), (8, game.tick() as u64, 8), (0x10, bush as u64, 8), (0x18, chosen.0 as u64 * 32000 + 16000, 8), (0x20, chosen.1 as u64 * 32000 + 16000, 8), (109, 2, 1), (112, ol as u64, 1), (177, 12, 1)] });
                plog += &format!("L96 AroundBush(out_line cell={:?}) ", chosen);
            }
            // steal_jungle_action (L142~)
            let (sbx, sby) = (bx, by);
            let ms_ = champ.stat_cached.move_speed;
            for j in cache.jungles.iter() {
                let jp = ep(*j);
                let ty: i64 = rd(jp, 0x68); let camp_team: usize = rd(jp, 0x98);
                if !(ty == 4 && camp_team == et) { continue; }
                if dist_sq(j.x, j.y, sbx, sby) > 22500000000 { continue; }
                if champ.can_attack() { if let Some(atk) = champ.attack_effect.as_ref() {
                    let dmg = atk.expected_damage_target(&ctx, champ as &dyn AbstractEntity, j);
                    let range = atk.range(champ) + atk.range_adjust(champ, j) + champ.radius() as u64 + j.radius() as u64;
                    let d2 = dist_sq(j.x, j.y, cx, cy); let md = range + ms_ as u64 * 20;
                    plog += &format!("[jungle {} hp={} dmg={} d2={} md2={}] ", j.id, j.hp, dmg, d2, md * md);
                    if j.hp <= dmg && d2 <= md * md { preds.push(Pred { tag: 15, bytes: vec![(8, j.id as u64, 8), (177, 15, 1)] }); plog += "L180 Attack "; }
                } }
                if champ.can_skill() { if let Some(sk) = champ.skill_effect.as_ref() { if sk.target.check(champ, j) {
                    let dmg = sk.expected_damage_target(&ctx, champ as &dyn AbstractEntity, j);
                    let range = sk.range(champ) + sk.range_adjust(champ, j) + champ.radius() as u64 + j.radius() as u64;
                    let d2 = dist_sq(j.x, j.y, cx, cy); let md = range + ms_ as u64 * 20;
                    if j.hp <= dmg && d2 <= md * md { preds.push(Pred { tag: 16, bytes: vec![(8, j.id as u64, 8), (177, 16, 1)] }); plog += "L195 Skill "; }
                } } }
                if champ.can_skill2() { if let Some(sk) = champ.skill2_effect().as_ref() { if sk.target.check(champ, j) {
                    let dmg = sk.expected_damage_target(&ctx, champ as &dyn AbstractEntity, j);
                    let range = sk.range(champ) + sk.range_adjust(champ, j) + champ.radius() as u64 + j.radius() as u64;
                    let d2 = dist_sq(j.x, j.y, cx, cy); let md = range + ms_ as u64 * 20;
                    if j.hp <= dmg && d2 <= md * md { preds.push(Pred { tag: 17, bytes: vec![(8, j.id as u64, 8), (177, 17, 1)] }); plog += "L211 Skill2 "; }
                } } }
            }
        }
    } else {
        // L107: is_vis && !check_move
        if is_vis {
            let cx: u64 = rd(cp, 0x660); let cy: u64 = rd(cp, 0x668);
            let mut best: Option<(u64, &Entity)> = None;
            for slot in 0..5 { if let Some(e) = cache.player_champion[et][slot] { if e.is_visible_from(champ) {
                let k = dist_sq(e.x, e.y, cx, cy); if best.map_or(true, |(bk, _)| k < bk) { best = Some((k, e)); } } } }
            plog += &format!("L110 nearest={:?} ", best.map(|(k, e)| (e.id, k)));
            let mut rtmp = rnd_p.clone();
            if let Some((_, en)) = best {
                let allies = (0..5).filter(|&s| cache.player_champion[team][s].map_or(false, |e| e.id != champ.id && dist_sq(e.x, e.y, cx, cy) < 22500000001)).count();
                let enemies = (0..5).filter(|&s| cache.player_champion[et][s].map_or(false, |e| e.is_visible_from(champ) && dist_sq(e.x, e.y, cx, cy) < 22500000001)).count();
                let dominated = allies < enemies;
                let can_fight = !dominated && game_ai::can1v1win(&mut rtmp, player, &data, champ, en);
                plog += &format!("L114 allies={} enemies={} dominated={} can_fight={} ", allies, enemies, dominated, can_fight);
                if can_fight { let v = game_ai::battle_action(version, &mut rtmp, player, &data, 5); for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); } plog += &format!("L124 battle n={} ", v.len()); }
                else {
                    preds.push(Pred { tag: 3, bytes: vec![(0x18, 5, 8), (125, 2, 1), (128, 0, 1), (177, 3, 1)] }); plog += "L126 RunAway ";
                    if !dominated { let v = game_ai::battle_action(version, &mut rtmp, player, &data, 5); for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); } plog += &format!("L128 battle n={} ", v.len()); }
                }
            } else {
                preds.push(Pred { tag: 3, bytes: vec![(0x18, 5, 8), (125, 2, 1), (128, 0, 1), (177, 3, 1)] }); plog += "L132 RunAway ";
                let v = game_ai::battle_action(version, &mut rtmp, player, &data, 5); for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); } plog += &format!("L133 battle n={} ", v.len());
            }
        }
    }
    // L136 attack_summon_action
    { let mut rtmp = rnd_p.clone(); let v = game_ai::attack_summon_action(player, &data); for e in v.iter() { let p = e as *const _ as *const u8; preds.push(Pred { tag: rd(p, 177), bytes: vec![] }); } plog += &format!("L136 summon n={} ", v.len()); }
    println!("predict: {}", plog);

    // ══ 실행 ══
    let res = game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates(&mut hs, version, &mut rnd, player, &data, param, &mut dbgf);
    let self_after: [u8; 16] = rd(hp, 0);
    let vp = &res as *const _ as *const u8;
    let (rptr, rbump, rcap, rlen): (usize, usize, usize, usize) = (rd(vp, 0), rd(vp, 8), rd(vp, 16), rd(vp, 24));
    println!("res ptr={:#x} bump={:#x} cap={} len={} (bump==ctx.pool? {})", rptr, rbump, rcap, rlen, rbump == (&pool as *const _ as usize));
    println!("self_after={:?} predict cm={} esm={}", self_after, p_cm, p_esm);
    let mut fails = 0;
    if self_after[9] != p_cm || self_after[10] != p_esm || self_after[..9] != self_before[..9] { fails += 1; println!("SELF MISMATCH"); }
    if self_after[11..] != self_before[11..] { println!("SELF PADDING CHANGED {:?} -> {:?}", &self_before[11..], &self_after[11..]); }
    let mut got: Vec<u8> = Vec::new();
    for i in 0..rlen { let p = (rptr + i * 184) as *const u8; let t: u8 = rd(p, 177); got.push(t); }
    let want: Vec<u8> = preds.iter().map(|p| p.tag).collect();
    println!("tags got ={:?}", got.iter().map(|&t| tagname(t)).collect::<Vec<_>>());
    println!("tags want={:?}", want.iter().map(|&t| tagname(t)).collect::<Vec<_>>());
    if got.len() != want.len() { fails += 1; println!("LEN MISMATCH"); }
    for (i, pr) in preds.iter().enumerate() {
        if i >= rlen { break; }
        let p = (rptr + i * 184) as *const u8;
        let t: u8 = rd(p, 177);
        if pr.tag <= 2 { if t > 2 { fails += 1; println!("elem{} tag {} != untagged", i, t); } } else if t != pr.tag { fails += 1; println!("elem{} tag {} != {}", i, t, pr.tag); }
        for &(off, val, sz) in pr.bytes.iter() {
            let g: u64 = match sz { 1 => rd::<u8>(p, off) as u64, _ => rd::<u64>(p, off) };
            if g != val { fails += 1; println!("elem{} +{:#x} got {} want {}", i, off, g, val); }
        }
        // 원소 덤프(비어있지 않은 8B 단위)
        let mut dump = String::new();
        for off in (0..184).step_by(8) { let v: u64 = rd(p, off); if v != 0 { dump += &format!("+{:#x}={} ", off, v); } }
        println!("  elem{} {}: {}", i, tagname(t), dump);
    }
    // rnd 소비 대조
    let n1 = rnd.next_u64(); let n2 = rnd_p.next_u64();
    println!("rnd after: game={} predict={} {}", n1, n2, if n1 == n2 { "RND_MATCH" } else { "RND_MISMATCH" });
    if n1 != n2 { fails += 1; }
    println!("RESULT {}", if fails == 0 { "MATCH" } else { "MISMATCH" });
}
