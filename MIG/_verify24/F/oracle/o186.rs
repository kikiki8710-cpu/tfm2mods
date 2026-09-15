#![allow(unused, dead_code, non_snake_case)]
//! 24차 배치F · 186 calculate_action_score 오라클 (pub 직접 호출) — 명세 `logic` 독립 재구현(predict) ↔ 실행 대조.
//!  한 프로세스 = 한 케이스(argv). 세계 = TEMPLATE mkgame(+last_hit 스탯) + 미니언 150틱(L/W 모드).
//!  rnd 소비 순서 검증: 같은 시드의 StdRng 복제본으로 predict 가 pub 콜리(range_misjudge_rng/roll_i64/gen_range)를 같은 순서로 부른다.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify24/F/oracle/o186.rs
//!  실행: o186.exe <mode S|L|W|C> k=v ...   (드라이버 = run186.py)
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, start_timing: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 100000, growth_range: 0, start_timing,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

pub fn mkgame2(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext, last_hit: usize, seed: u64) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(seed, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60; st.last_hit = last_hit;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

fn minion_setting(s: &mut GameSetting) {
    s.minion_wave_setting.start_tick = 10; s.minion_wave_setting.tick_per_wave = 660;
    s.minion_wave_setting.melee_count = 2; s.minion_wave_setting.range_count = 1;
    s.minion_wave_setting.tick_per_spawn = 30; s.minion_wave_setting.growth_start_tick = 1800;
    s.minion_wave_setting.growth_tick = 1800; s.minion_wave_setting.growth_tick_2v2 = 400;
    s.minion_wave_setting.growth_tick_3v3 = 800; s.minion_wave_setting.growth_end_tick = 24000;
    s.minion_wave_setting.growth_end_tick_2v2 = 30000; s.minion_wave_setting.growth_end_tick_3v3 = 24000;
    s.minion_wave_setting.exp_range = 150000; s.minion_wave_setting.exp_decay2 = 100;
    s.minion_wave_setting.exp_decay3 = 80; s.minion_wave_setting.exp_decay4 = 60;
    s.melee_minion.stat.attack = 10; s.melee_minion.stat.hp = 400; s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1; s.melee_minion.growth.hp = 30; s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100; s.melee_minion.attack.range = 3000; s.melee_minion.attack.cooltime = 30;
    s.melee_minion.attack.duration = 24; s.melee_minion.attack.start_timing = 16; s.melee_minion.exp = 40; s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15; s.range_minion.stat.hp = 250; s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1; s.range_minion.growth.hp = 20; s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000; s.range_minion.attack.speed = 3000; s.range_minion.attack.cooltime = 40;
    s.range_minion.attack.duration = 24; s.range_minion.attack.start_timing = 16; s.range_minion.exp = 30; s.range_minion.gold = 20;
}

struct Args { m: HashMap<String, i64>, mode: String }
impl Args {
    fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) }
}

/// 라인전 플래그(L76/L199 재구현)
fn line_flag(ctx: &GameContext, tick: usize, pos_is_jungle: bool) -> bool {
    let in_set = matches!(ctx.tutorial, TutorialType::None | TutorialType::MidBottom | TutorialType::Line | TutorialType::Total);
    let tps = ctx.setting.tick_per_second;
    let boundary = ctx.setting.epic_jungle.first_spawn_tick.saturating_sub(30 * tps);
    if in_set { if tick < boundary { !pos_is_jungle } else { false } } else { !pos_is_jungle }
}

fn moba_buff(game: &dyn AbstractGame, team: usize) -> Option<usize> {
    match game.get_game_mode() {
        GameMode::Moba(m) => Some(rd::<usize>(m as *const MobaMode as *const u8, 0x240 + team * 8)),
        _ => None,
    }
}

/// ★명세 logic 독립 재구현. 콜리는 pub 함수를 그대로 쓴다(합성만 검증).
fn predict(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, param: &game_ai::ScoreParameter,
           action_cool: usize, effect: &Effect, speed_mult: usize, t: &Entity, ty: u8, ctx: &GameContext, game: &dyn AbstractGame,
           log: &mut String) -> Result<i64, String> {
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = data.cache.player_champion[team][pos].ok_or("nochamp")?;
    let acc = player.info.parameter.last_hit_accuracy() as i64;
    let base = 1000 - acc; let max_v = 2000 - acc;
    let mut jrng = game_ai::range_misjudge_rng(version, data, player, t.id);
    *log += &format!("acc={} jrng={} ", acc, jrng.is_some());
    let mut roll = |rnd: &mut rand::rngs::StdRng, jrng: &mut Option<NoiseRng>| -> i64 { game_ai::range_misjudge_roll_i64(rnd, jrng, acc, max_v) };
    let dmg0 = effect.expected_damage_target(ctx, champ as &dyn AbstractEntity, t) as i64;
    let r1 = roll(rnd, &mut jrng); let value = r1 * dmg0 / 1000;
    let r2 = roll(rnd, &mut jrng); let hp = r2 * (t.hp as i64) / 1000;
    let r3 = roll(rnd, &mut jrng); let total = r3 * (t.stat_cached.hp as i64) / 1000;
    if speed_mult == 0 { return Err("speed_mult0".into()); }
    let start_raw = effect.start_timing * 100 / speed_mult;
    let r4 = roll(rnd, &mut jrng) as u64; let start = (r4.wrapping_mul(start_raw as u64)) / 1000;
    let cool_raw = action_cool * 100 / speed_mult;
    let r5 = roll(rnd, &mut jrng) as u64; let cool_pre = r5.wrapping_mul(cool_raw as u64); let cooltime = cool_pre / 1000;
    *log += &format!("dmg0={} value={} hp={} total={} start={} cool={} cool_pre={} ", dmg0, value, hp, total, start, cooltime, cool_pre);
    let tp = ep(t);
    let tty: i64 = rd(tp, 0x68);
    let buff_enemy = moba_buff(game, 1 - team);
    let flag = line_flag(ctx, game.tick(), pos == 1);
    let mut gen = |rnd: &mut rand::rngs::StdRng| -> i64 { rnd.gen_range(0..=1000usize) as i64 };
    let sty = |ty: u8, normal: &mut dyn FnMut() -> i64| -> i64 { match ty { 0 => -9999, 2 => 10, _ => normal() } };
    if tty == 1 {
        *log += "MINION ";
        let ne_tag: i64 = rd(tp, 0x88);
        if ne_tag & 1 == 1 {
            let nid: usize = rd(tp, 0x90);
            if let Some(target) = game.get_entity_by_id(nid) {
                let tt: i64 = rd(ep(target), 0x68);
                if tt == 2 {
                    if matches!(buff_enemy, Some(b) if b != 0) { *log += "L32 "; return Ok(30); }
                    let twty: i8 = rd(ep(target), 0x128);
                    if twty == 3 || twty == 4 { *log += "L37 "; return Ok(if matches!(buff_enemy, Some(b) if b != 0) { 70 } else { 50 }); }
                } else if tt == 3 {
                    *log += "L46 "; return Ok(if matches!(buff_enemy, Some(b) if b != 0) { 100 } else { 70 });
                }
            }
        }
        if matches!(buff_enemy, Some(b) if b != 0) { *log += "L55 "; return Ok(20); }
        if let Some(snap) = param.wave_snapshot.as_ref() {
            if let Some(traj) = snap.find(t.id) {
                *log += "SNAP ";
                let err = std::cmp::min(((base * base) as u64 / 1000) as i64, 1000);
                if gen(rnd) < err { *log += "L72 "; return Ok(-9999); }
                let p_raw = traj.hp_at_tick(start as usize);
                let r6 = roll(rnd, &mut jrng); let pre = r6 * p_raw; let predicted = pre / 1000;
                let death = traj.expected_death_tick as u64;
                *log += &format!("flag={} p_raw={} predicted={} death={} ", flag, p_raw, predicted, death);
                if pre > 999 {
                    let can_last_hit = predicted > value + 5;
                    let will_die_soon = death <= cooltime + start;
                    if !can_last_hit {
                        let urgency: i64 = if death > start + 5 { if will_die_soon { 25 } else { 15 } } else { 30 };
                        let n = snap.count.min(12);
                        let mut concurrent = 0i64;
                        for o in snap.minions[..n].iter() { if o.entity_id == t.id { continue; } let h = o.hp_at_tick((cooltime + start) as usize); if h > 0 && h <= value + 5 { concurrent += 1; } }
                        let mut multi = 0i64;
                        if concurrent > 0 {
                            let mut earlier = 0i64;
                            for o in snap.minions[..n].iter() { if o.entity_id == t.id { continue; } if (o.expected_death_tick as u64) < death { earlier += 1; } }
                            multi = if earlier == 0 { 5 } else { -5 };
                        }
                        *log += &format!("L97 urgency={} concurrent={} multi={} ", urgency, concurrent, multi);
                        return Ok(if flag { urgency + multi } else { urgency + 3 + multi });
                    } else if will_die_soon && predicted <= 2 * value {
                        *log += "L152 "; return Ok(if ty == 0 { -9999 } else { 5 });
                    }
                }
                if predicted > 3 * value {
                    *log += "L159 ";
                    return Ok(sty(ty, &mut || { let g = gen(rnd) < err; if cool_pre > 29999 { if g { 10 } else { -9999 } } else if flag { if g { 10 } else { -9999 } } else { if g { -9999 } else { 10 } } }));
                } else {
                    *log += "L178 ";
                    return Ok(sty(ty, &mut || { if gen(rnd) < err { 10 } else { -9999 } }));
                }
            }
        }
        // 레거시 — 공격자 없음 가정(케이스 생성이 보장)
        *log += "LEGACY ";
        let (applyed0, expected0) = (0i64, 0i64);
        let r7 = roll(rnd, &mut jrng) as u64; let applyed = (r7.wrapping_mul(applyed0 as u64) / 1000) as i64;
        let r8 = roll(rnd, &mut jrng) as u64; let expected = (r8.wrapping_mul(expected0 as u64) / 1000) as i64;
        let err = 1000 - rnd.gen_range((acc as usize)..=1000usize) as i64;
        *log += &format!("flag={} err={} ", flag, err);
        if value - 5 + applyed < hp {
            if value + total * 3 / 10 + expected < hp {
                *log += "L365 ";
                return Ok(sty(ty, &mut || { let g = gen(rnd) < err; if cool_pre > 29999 { if g { 10 } else { -9999 } } else if flag { if g { 10 } else { -9999 } } else { if g { -9999 } else { 10 } } }));
            } else {
                *log += "L393 ";
                return Ok(sty(ty, &mut || { if gen(rnd) < err { 10 } else { -9999 } }));
            }
        } else {
            if gen(rnd) < err { *log += "L329 "; return Ok(-9999); }
            if expected + total * 3 / 10 < hp {
                *log += "L333 ";
                return Ok(sty(ty, &mut || { let g = gen(rnd) < err; if cool_pre > 29999 { if g { 10 } else { -9999 } } else if flag { if g { 10 } else { -9999 } } else { if g { -9999 } else { 10 } } }));
            } else {
                *log += "L359 ";
                let g = gen(rnd) < err;
                return Ok(if flag { if g { -9999 } else { 15 } } else { if g { -9999 } else { 20 } });
            }
        }
    }
    let mut bonus = 0i64;
    if tty == 13 {
        *log += "CHAMP ";
        if t.team != champ.team {
            if let Some(epl) = data.cache.player_by_champion_id(t.id) {
                let bbp = &data.blackboard[1 - team];
                if let Some(sa) = bbp.small_actions[epl.info.position.as_index()].as_ref() {
                    let tid = match sa { SmallAction::Attack { target_id } => Some(*target_id), SmallAction::Skill { target_id } => Some(*target_id), SmallAction::Skill2 { target_id } => Some(*target_id), _ => None };
                    if let Some(tid) = tid {
                        if let Some(tt) = game.get_entity_by_id(tid) {
                            let ttt: i64 = rd(ep(tt), 0x68);
                            if ttt == 2 { let twty: i8 = rd(ep(tt), 0x128); bonus = if twty == 3 || twty == 4 { 80 } else { 10 }; }
                            else if ttt == 3 { bonus = 100; }
                        }
                    }
                }
            }
        }
        *log += &format!("bonus={} ", bonus);
    }
    let value2 = effect.expected_damage_target(ctx, champ as &dyn AbstractEntity, t) as i64;
    let has_epic_buff = match moba_buff(game, team) { Some(b) => b == 0, None => true };
    let coef: i64 = match tty {
        3 => 200,
        2 => {
            // ★IR 39642~39647: %958 = t+0x490 = **t(타워).attack_effect** (champ 것이 아니다) — 명세 정정 대상
            let cae = t.attack_effect.as_ref().ok_or("t_noatk")?;
            let mut in_range = 0i64;
            for e in game.iter_entity() {
                let ety: i64 = rd(ep(e), 0x68);
                if e.team == champ.team && ety == 1 && cae.is_in_range(t, e) { in_range += 1; }
            }
            let ne_tag: i64 = rd(tp, 0x88); let nid: usize = rd(tp, 0x98);
            let cond = if ne_tag & 1 == 1 && nid != champ.id {
                match game.get_entity_by_id(nid) {
                    Some(e) => { let d = cae.expected_damage_target(ctx, t as &dyn AbstractEntity, e); *log += &format!("e.hp={} d={} ", e.hp, d); e.hp >= d || in_range > 1 }
                    None => in_range > 1,
                }
            } else { false };
            *log += &format!("in_range={} cond={} heb={} ", in_range, cond, has_epic_buff);
            if cond { if ty == 2 { if has_epic_buff { 160 } else { 240 } } else { if has_epic_buff { 80 } else { 160 } } }
            else { if (t.hp as i64) > value2 { 0 } else { 30 } }
        }
        _ => 0,
    };
    if t.hp == 0 { return Err("hp0".into()); }
    *log += &format!("coef={} value2={} thp={} ", coef, value2, t.hp);
    Ok(std::cmp::min(coef, coef * value2 / (t.hp as i64)) + bonus)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::calculate_action_score as *const (); }
    let mode = av.get(1).cloned().unwrap_or("S".into());
    let mut m = HashMap::new();
    for s in av.iter().skip(2) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
    let a = Args { m, mode: mode.clone() };
    let mut setting = real_setting();
    minion_setting(&mut setting);
    setting.epic_jungle.first_spawn_tick = a.get("first_spawn", 18000) as usize;
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tut = match a.get("tut", 0) { 1 => TutorialType::First, 2 => TutorialType::TopSolo, 3 => TutorialType::Bottom, 4 => TutorialType::MidSolo,
                                     5 => TutorialType::MidBottom, 6 => TutorialType::JungleOnly, 7 => TutorialType::Line, 8 => TutorialType::Total, _ => TutorialType::None };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off };
    let last_hit = a.get("last_hit", 100) as usize;
    let mut game = mkgame2(&setting, &ms, &map, &ctx, last_hit, 1234);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(9);
    let need_minions = mode == "L" || mode == "W" || a.get("minions", 0) == 1;
    if need_minions { for _ in 0..(a.get("ticks", 150) as usize) { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); } }
    let tick = a.get("tick", 1000) as usize;
    game.set_tick(tick);
    let version = a.get("version", 55) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let posi = a.get("pos", 0) as usize; // 0 Top / 1 Jungle
    let pos = [Position::Top, Position::Jungle][posi];
    let player = game.get_player_by_position(0, pos).expect("player");
    let champ = cache.player_champion[0][posi].expect("champ");
    let cp = ep(champ);
    // 챔피언 attack_effect 세팅(타워 경로 unwrap 방지 + cond 판정 피해량)
    let cdmg = a.get("cdmg", 100) as usize;
    unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(cdmg, 0))); }
    // 에픽버프 시간
    if let GameMode::Moba(mm) = (&game as &dyn AbstractGame).get_game_mode() {
        let mp = mm as *const MobaMode as *const u8;
        wr(mp, 0x240, a.get("buff0", 0) as usize); wr(mp, 0x248, a.get("buff1", 0) as usize);
    }
    let dmg = a.get("dmg", 100) as usize;
    let start_timing = a.get("start", 10) as usize;
    let effect = mkeff(dmg, start_timing);
    let mut ad = DataActionDef::default(); ad.cooltime = a.get("cool", 60) as usize;
    let action: Box<dyn Action> = Box::new(ad);
    let speed_mult = a.get("speed", 100) as usize;
    let ty = a.get("ty", 1) as u8;
    let mty = match ty { 0 => game_ai::MinionActionType::Pull, 2 => game_ai::MinionActionType::Push, _ => game_ai::MinionActionType::Normal };
    // ScoreParameter: wave_snapshot 만 읽힌다(IR mem[13..15]) — 나머지는 0 버퍼
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    // 대상 t
    let t: &Entity = match mode.as_str() {
        "S" => {
            let kind = a.get("kind", 0); // 0 top_tower / 1 twin / 2 nexus
            match kind { 0 => cache.top_tower[1].expect("tower"), 1 => cache.twin_towers[1][0], _ => cache.nexus[1].expect("nexus") }
        }
        "C" => cache.player_champion[a.get("cteam", 1) as usize][a.get("cpos", 0) as usize].expect("champ t"),
        _ => { let tm = a.get("tteam", 1) as usize; cache.iter_minions(tm).nth(a.get("nth", 0) as usize).expect("minion") }
    };
    let tp = ep(t);
    if a.get("thp", -1) >= 0 { wr(tp, 0x670, a.get("thp", 0) as usize); }
    if a.get("tdmg", -1) >= 0 { unsafe { std::ptr::write(&mut (*(tp as *mut Entity)).attack_effect, Some(mkeff(a.get("tdmg", 0) as usize, 0))); } }
    if a.get("tnoatk", 0) == 1 { unsafe { std::ptr::write(&mut (*(tp as *mut Entity)).attack_effect, None); } }
    if a.get("tmax", -1) >= 0 { wr(tp, 0x628, a.get("tmax", 0) as usize); }
    if mode == "L" || mode == "W" {
        // 미니언 nearest_enemy: mne=0 없음 / 1 내(팀0) top 타워 / 2 내 쌍둥이 / 3 내 넥서스 / 4 적 챔피언
        let mne = a.get("mne", 0);
        let nid = match mne { 1 => Some(cache.top_tower[0].unwrap().id), 2 => Some(cache.twin_towers[0][0].id), 3 => Some(cache.nexus[0].unwrap().id), 4 => Some(cache.player_champion[1][0].unwrap().id), _ => None };
        match nid { Some(i) => { wr(tp, 0x88, 1i64); wr(tp, 0x90, i); } None => { wr(tp, 0x88, 0i64); } }
    }
    if mode == "S" {
        // 타워 nearest_enemy: tgt=0 없음 / 1 = champ 자신 / 2 = 아군 Jungle 챔피언(hp=ehp) / 3 = 존재하지 않는 id
        let tgt = a.get("tgt", 0);
        match tgt {
            1 => { wr(tp, 0x88, 1i64); wr(tp, 0x98, champ.id); }
            2 => { let other = cache.player_champion[0][1].expect("jungle"); wr(tp, 0x88, 1i64); wr(tp, 0x98, other.id);
                   if a.get("ehp", -1) >= 0 { wr(ep(other), 0x670, a.get("ehp", 0) as usize); } }
            3 => { wr(tp, 0x88, 1i64); wr(tp, 0x98, 999999usize); }
            _ => { wr(tp, 0x88, 0i64); }
        }
        // 아군 미니언 n 마리를 타워 위치로(in_range_minion)
        let n = a.get("inrange", 0) as usize;
        if need_minions {
            let mut placed = 0usize;
            for mn in cache.iter_minions(0) { let mp = ep(mn); if placed < n { wr(mp, 0x660, t.x); wr(mp, 0x668, t.y); placed += 1; } else { wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); } }
            for mn in cache.iter_minions(1) { let mp = ep(mn); wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); }
        }
    }
    if mode == "C" {
        // 적 Top 플레이어의 small_action → 내 타워/쌍둥이/넥서스/없음
        let sa = a.get("sa", 0);
        let tgt_id = match a.get("satgt", 0) { 0 => cache.top_tower[0].unwrap().id, 1 => cache.twin_towers[0][0].id, 2 => cache.nexus[0].unwrap().id, 3 => 999999usize, _ => champ.id };
        bb[1].small_actions[0] = match sa { 1 => Some(SmallAction::Attack { target_id: tgt_id }), 2 => Some(SmallAction::Skill { target_id: tgt_id }), 3 => Some(SmallAction::Skill2 { target_id: tgt_id }), 4 => Some(SmallAction::Ult { target_id: tgt_id }), _ => None };
    }
    if mode == "W" {
        let mut snap = game_ai::MinionWaveSnapshot::default();
        let mut tr = game_ai::MinionHpTrajectory::default();
        tr.entity_id = t.id; tr.current_hp = a.get("P", 500); tr.checkpoint_count = 0; tr.expected_death_tick = a.get("D", 100) as usize;
        snap.minions[0] = tr; snap.count = 1;
        // 다른 미니언 궤적(동시 막타·먼저 죽는 쪽)
        let n2 = a.get("n2", 0) as usize;
        for k in 0..n2 {
            let mut o = game_ai::MinionHpTrajectory::default();
            o.entity_id = 500000 + k; o.current_hp = a.get(&format!("P{}", k + 2), 100); o.checkpoint_count = 0; o.expected_death_tick = a.get(&format!("D{}", k + 2), 50) as usize;
            snap.minions[1 + k] = o; snap.count = 2 + k;
        }
        if a.get("miss", 0) == 1 { snap.minions[0].entity_id = 777777; }   // 스냅샷은 있으나 t 미등재 → 레거시
        unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).wave_snapshot), Some(snap)); }
    }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let data = OperationData::new(&cache, &ctx, &bb);
    // 공격자 없음 가정 검증(레거시 경로)
    let mut attackers = 0usize;
    for e in (&game as &dyn AbstractGame).iter_entity() {
        let b = ep(e); let ety: i64 = rd(b, 0x68);
        if matches!(ety, 1 | 2 | 7 | 8 | 9 | 10) {
            let ne: i64 = rd(b, 0x88); let nid: usize = rd(b, 0x90); let nid2: usize = rd(b, 0x98);
            let st: i64 = rd(b, 0x70); let stid: usize = rd(b, 0x78);
            if (ne & 1 == 1 && (nid == t.id || nid2 == t.id)) || (st & 1 == 1 && stid == t.id) { attackers += 1; }
        }
    }
    let nproj = (&game as &dyn AbstractGame).iter_projectile().count();
    let seed = a.get("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd2 = rnd.clone();
    let mut log = String::new();
    let pred = predict(version, &mut rnd2, player, &data, param, a.get("cool", 60) as usize, &effect, speed_mult, t, ty, &ctx, &game as &dyn AbstractGame, &mut log);
    let mut dbgf: DebugFrameData = Default::default();
    let real = game_ai::calculate_action_score(version, &mut rnd, player, &data, param, &action, &effect, speed_mult, t, mty, &mut dbgf);
    // rnd 소비량 비교: 두 rng 로 다음 값을 뽑아 같으면 소비 횟수 동일
    let a1: u64 = rnd.gen(); let a2: u64 = rnd2.gen();
    let tty: i64 = rd(tp, 0x68);
    println!("mode={} tty={} t.id={} t.hp={} t.max={} attackers={} proj={} | {}", mode, tty, t.id, t.hp, t.stat_cached.hp, attackers, nproj, log);
    match pred {
        Ok(p) => println!("RESULT\tpred={}\treal={}\t{}\trng_sync={}", p, real, if p == real { "MATCH" } else { "MISMATCH" }, a1 == a2),
        Err(e) => println!("RESULT\tpred=ERR({})\treal={}\tSKIP", e, real),
    }
}
