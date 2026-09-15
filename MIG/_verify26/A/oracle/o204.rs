#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치A · 204 calculate_score_parameter 오라클 (pub 직접 호출 · game_ai::calculate_score_parameter).
//!  한 프로세스 = 한 케이스(argv k=v) — 콜리 precompute_champion_powers 가 CHAMP_POWERS_MEMO(TLS · 키 (seed,tick))를 쓰므로(함정 ③).
//!  세계 = TEMPLATE mkgame(타워 16·챔프 10, 미니언 없음). 관측 = sret ScoreParameter 5384B 를 오프셋으로 직독.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/A/oracle/o204.rs
//!  실행: o204.exe me=<0..4> adist=<D|-1> aact=<tag|-1> edist=<D|-1> evis=<0|1> lastd=<n> eact=<tag|-1> eatk=<dmg|-1> erng=<r> ecd=<n> estate=<n> ... (드라이버 run204.py)
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

fn sa(tag: i64, target: usize) -> Option<SmallAction> {
    match tag {
        0 => Some(SmallAction::RunAway), 6 => Some(SmallAction::Attack { target_id: target }),
        7 => Some(SmallAction::Skill { target_id: target }), 8 => Some(SmallAction::Skill2 { target_id: target }),
        9 => Some(SmallAction::Ult { target_id: target }), 10 => Some(SmallAction::Stop), 4 => Some(SmallAction::Trace { target_id: target }),
        _ => None,
    }
}

// ChampionScoreParameter(216B) 를 base 포인터에서 읽어 한 줄로
fn csp(b: *const u8) -> String {
    let rp: *const u8 = rd(b, 0x18); let rlen: usize = rd(b, 0x30);
    let mut rs = String::new();
    for i in 0..rlen.min(16) { let g = unsafe { rp.add(i * 24) }; rs += &format!("{{from={} tick={} value={}}}", rd::<usize>(g, 0), rd::<usize>(g, 8), rd::<usize>(g, 0x10)); }
    let gp: *const u8 = rd(b, 0x38); let glen: usize = rd(b, 0x50);
    format!("act={} tgt={} id={} team={} pos={} ad={} ac={} rd={} red={} rc={} rpt={} at={} av={} uv={} ap={} up={} cc={} bc={} rp.len={} rp.cap={} [{}] gp.len={}",
        rd::<i64>(b, 0), rd::<usize>(b, 8), rd::<usize>(b, 0x58), rd::<usize>(b, 0x60), rd::<usize>(b, 0x68),
        rd::<usize>(b, 0x70), rd::<usize>(b, 0x78), rd::<usize>(b, 0x80), rd::<usize>(b, 0x88), rd::<usize>(b, 0x90), rd::<usize>(b, 0x98), rd::<usize>(b, 0xa0),
        rd::<i64>(b, 0xa8), rd::<i64>(b, 0xb0), rd::<i64>(b, 0xb8), rd::<i64>(b, 0xc0), rd::<i64>(b, 0xc8), rd::<i64>(b, 0xd0),
        rlen, rd::<usize>(b, 0x28), rs, glen)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::calculate_score_parameter as *const (); }
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
    let tut = match a.i("tut", 0) { 1 => TutorialType::First, 3 => TutorialType::Bottom, 5 => TutorialType::MidBottom, _ => TutorialType::None };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off };
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
    // 내 챔피언: 위치·공격 이펙트·쿨다운·행동상태
    if a.i("mx", -1) >= 0 { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    let atk = a.i("atk", -1);
    if atk >= 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(atk as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
    if a.i("mcd", -1) >= 0 { wr(cp, 0xb0, a.i("mcd", 0) as usize); }
    if a.i("mstate", -1) >= 0 { wr(cp, 0x70, a.i("mstate", 0) as i64); }
    if a.i("mspd", -1) >= 0 { wr(cp, 0x640, a.i("mspd", 0) as usize); }
    let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
    let my_id = champ.id;
    // 아군(같은 팀, me 제외): adist 로 x 축 거리 배치 · aact 로 블랙보드 행동(대상 = 적 e0 id) · aatk 로 공격 이펙트
    let e0 = cache.player_champion[1][0].expect("e0");
    let adist = a.i("adist", -1);
    for p in 0..5usize {
        let e = cache.player_champion[0][p].expect("ally");
        let eb = ep(e);
        if p == me { if a.i("mact", -1) >= 0 { bb[0].small_actions[p] = sa(a.i("mact", 0), e0.id); } continue; }
        if adist >= 0 { wr(eb, 0x660, mx + adist as u64); wr(eb, 0x668, my); }
        if a.i("aact", -1) >= 0 { bb[0].small_actions[p] = sa(a.i("aact", 0), e0.id); }
        if a.i("aatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff(a.i("aatk", 0) as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
        if a.i("acd", -1) >= 0 { wr(eb, 0xb0, a.i("acd", 0) as usize); }
        if a.i("askl", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).skill_effect, Some(mkeff(a.i("askl", 0) as usize, a.i("arng", 100000) as u64, CastingType::Targeting))); } }
        if a.i("astate", -1) >= 0 { wr(eb, 0x70, a.i("astate", 0) as i64); }
    }
    // 적: edist 거리 · evis(bb[1].last_visible[p] = tick - lastd) · eact(대상 = 내 id) · eatk/erng/ecd/estate · ecast(1=Position 논타겟)
    let edist = a.i("edist", -1);
    let evis = a.i("evis", 0);
    let lastd = a.i("lastd", 0) as usize;
    let eonly = a.i("eonly", -1);   // eonly=p 면 적 p 만 세팅, 나머지는 (1,1) 로 멀리
    for p in 0..5usize {
        let e = cache.player_champion[1][p].expect("enemy");
        let eb = ep(e);
        if eonly >= 0 && p as i64 != eonly { wr(eb, 0x660, 1u64); wr(eb, 0x668, 1u64); continue; }
        if edist >= 0 { wr(eb, 0x660, mx + edist as u64); wr(eb, 0x668, my); }
        if evis == 1 { bb[1].last_visible[p] = tick.saturating_sub(lastd); }
        if a.i("egvis", 0) == 1 { wr(eb, 0x38, 0i64); }   // visible_state[0]=Visible(game.is_visible 경로)
        if a.i("eact", -1) >= 0 { bb[1].small_actions[p] = sa(a.i("eact", 0), if a.s("etgt", "me") == "me" { my_id } else { cache.player_champion[0][a.s("etgt", "1").parse::<usize>().unwrap()].unwrap().id }); }
        if a.i("eatk", -1) >= 0 {
            let cst = if a.i("ecast", 0) == 1 { CastingType::Position } else { CastingType::Targeting };
            unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff(a.i("eatk", 0) as usize, a.i("erng", 100000) as u64, cst))); }
        }
        if a.i("eatk", -1) == -2 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, None); } }
        if a.i("eskl", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).skill_effect, Some(mkeff(a.i("eskl", 0) as usize, a.i("erng", 100000) as u64, CastingType::Targeting))); } }
        if a.i("eskl", -1) == -2 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).skill_effect, None); } }
        if a.i("escd", -1) >= 0 { wr(eb, 0xb8, a.i("escd", 0) as usize); }
        if a.i("ecd", -1) >= 0 { wr(eb, 0xb0, a.i("ecd", 0) as usize); }
        if a.i("estate", -1) >= 0 { wr(eb, 0x70, a.i("estate", 0) as i64); }
        if a.i("elvl", -1) >= 0 { wr(eb, 0x5c8, a.i("elvl", 0) as usize); }
        if a.i("espd", -1) >= 0 { wr(eb, 0x640, a.i("espd", 0) as usize); }
    }
    // 타워: twr=1 이면 적 top 타워 사거리 안(range - tgap)에 나를 놓는다 · tatk 로 타워 공격 이펙트 · tnear=me|none|e0|a1 로 nearest_enemy
    let twr = cache.top_tower[1].or(cache.mid_tower[1]).or(cache.bottom_tower[1]).or(cache.twin_towers[1].first().copied()).expect("enemy tower");
    let tb = ep(twr);
    if a.i("tatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(tb as *mut Entity)).attack_effect, Some(mkeff(a.i("tatk", 0) as usize, a.i("trng", 100000) as u64, CastingType::Targeting))); } }
    if a.i("twr", 0) == 1 {
        let r = twr.attack_effect.as_ref().map(|e| e.range).unwrap_or(0);
        let gap = a.i("tgap", 1000) as u64;
        wr(cp, 0x660, twr.x + r.saturating_sub(gap)); wr(cp, 0x668, twr.y);
    }
    match a.s("tnear", "keep").as_str() {
        "none" => { wr(tb, 0x88, 0i64); }
        "me" => { wr(tb, 0x88, 1i64); wr(tb, 0x90, 0usize); wr(tb, 0x98, my_id); }
        "e0" => { wr(tb, 0x88, 1i64); wr(tb, 0x90, 0usize); wr(tb, 0x98, e0.id); }
        "a1" => { wr(tb, 0x88, 1i64); wr(tb, 0x90, 0usize); wr(tb, 0x98, cache.player_champion[0][if me == 1 { 2 } else { 1 }].unwrap().id); }
        "gone" => { wr(tb, 0x88, 1i64); wr(tb, 0x90, 0usize); wr(tb, 0x98, 999999usize); }
        _ => {}
    }
    // 내 팀 타워: etwr=1 이면 적 e0 를 내 타워 사거리 안에 · mtatk 로 이펙트 · mtnear=e0|none|gone|a1|me
    let mtw = cache.top_tower[0].or(cache.mid_tower[0]).or(cache.bottom_tower[0]).expect("my tower");
    let mtb = ep(mtw);
    if a.i("mtatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(mtb as *mut Entity)).attack_effect, Some(mkeff(a.i("mtatk", 0) as usize, a.i("trng", 100000) as u64, CastingType::Targeting))); } }
    if a.i("etwr", 0) == 1 {
        let r = mtw.attack_effect.as_ref().map(|e| e.range).unwrap_or(0);
        let gap = a.i("tgap", 1000) as u64;
        wr(ep(e0), 0x660, mtw.x + r.saturating_sub(gap)); wr(ep(e0), 0x668, mtw.y);
        let mo = a.i("meoff", -60000);   // 나 = e0 위치 + meoff (기본 -60000 → 내 타워에서 35000: near_towers 포함 · e0 에서 60000: near_enemies 포함)
        wr(cp, 0x660, (mtw.x + r.saturating_sub(gap)) .wrapping_add(mo as u64)); wr(cp, 0x668, mtw.y);
    }
    match a.s("mtnear", "keep").as_str() {
        "none" => { wr(mtb, 0x88, 0i64); }
        "e0" => { wr(mtb, 0x88, 1i64); wr(mtb, 0x90, 0usize); wr(mtb, 0x98, e0.id); }
        "e1" => { wr(mtb, 0x88, 1i64); wr(mtb, 0x90, 0usize); wr(mtb, 0x98, cache.player_champion[1][1].unwrap().id); }
        "gone" => { wr(mtb, 0x88, 1i64); wr(mtb, 0x90, 0usize); wr(mtb, 0x98, 999999usize); }
        "twr" => { wr(mtb, 0x88, 1i64); wr(mtb, 0x90, 0usize); wr(mtb, 0x98, twr.id); }   // 비챔피언 엔티티(적 타워)
        _ => {}
    }
    // 아군 전원을 적 타워 사거리 안에(atwr=1)
    if a.i("atwr", 0) == 1 {
        let r = twr.attack_effect.as_ref().map(|e| e.range).unwrap_or(0);
        let gap = a.i("tgap", 1000) as u64;
        for p in 0..5usize { if p == me { continue; } let e = cache.player_champion[0][p].unwrap(); wr(ep(e), 0x660, twr.x + r.saturating_sub(gap)); wr(ep(e), 0x668, twr.y); }
        if a.i("twr", 0) == 0 { let mo = a.i("meoff", 45000); wr(cp, 0x660, (twr.x + r.saturating_sub(gap)).wrapping_add(mo as u64)); wr(cp, 0x668, twr.y); }   // 나: 아군 + meoff(기본 45000 → 타워에서 140000: near_towers 포함 · 사거리(≈120030) 밖)
    }
    let mx: u64 = rd(cp, 0x660); let my: u64 = rd(cp, 0x668);
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let seed = a.i("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let rnd0 = rnd.clone();
    let dbg_before: Vec<u8> = unsafe { std::slice::from_raw_parts(&dbgf as *const _ as *const u8, std::mem::size_of::<DebugFrameData>()).to_vec() };
    // 세계 요약
    let ex: u64 = rd(ep(e0), 0x660); let ey: u64 = rd(ep(e0), 0x668);
    println!("world\ttick={}\tme=({},{})\tid={}\tradius={}\tspd={}\tlvl={}\te0=({},{})\td_e0={}\ttower=({},{})\td_twr={}\ttrng={:?}\ttower_id={}\tjungles={}\tothers={}/{}\ttdisable={}",
        tick, mx, my, my_id, champ.radius, champ.stat_cached.move_speed, champ.level, ex, ey, game_core::utils::distance(mx, my, ex, ey),
        twr.x, twr.y, game_core::utils::distance(mx, my, twr.x, twr.y), twr.attack_effect.as_ref().map(|e| e.range), twr.id,
        cache.jungles.len(), cache.others[0].len(), cache.others[1].len(), setting.tower_attack_disable_tick);
    // 기대 피해값(콜리 계약 직접 호출 · 교차 대조용)
    let mut exp_dmg = String::new();
    for p in 0..5usize {
        let e = cache.player_champion[1][p].unwrap();
        if let Some(eff) = e.attack_effect.as_ref() {
            let d1 = eff.expected_damage_target(&ctx, e as &dyn AbstractEntity, champ);
            exp_dmg += &format!("e{}->me={} ", p, d1);
        }
    }
    if let Some(eff) = champ.attack_effect.as_ref() { exp_dmg += &format!("me->e0={} ", eff.expected_damage_target(&ctx, champ as &dyn AbstractEntity, e0)); } else { exp_dmg += "me_atk=None "; }
    for p in 0..5usize {
        if p == me { continue; }
        let e = cache.player_champion[0][p].unwrap();
        if let Some(eff) = e.attack_effect.as_ref() { exp_dmg += &format!("a{}->e0={} ", p, eff.expected_damage_target(&ctx, e as &dyn AbstractEntity, e0)); }
    }
    if let Some(eff) = twr.attack_effect.as_ref() { exp_dmg += &format!("twr->me={} twr->a1={} ", eff.expected_damage_target(&ctx, twr as &dyn AbstractEntity, champ), eff.expected_damage_target(&ctx, twr as &dyn AbstractEntity, cache.player_champion[0][if me == 1 { 2 } else { 1 }].unwrap())); }
    if let Some(eff) = mtw.attack_effect.as_ref() { exp_dmg += &format!("mtw->e0={} ", eff.expected_damage_target(&ctx, mtw as &dyn AbstractEntity, e0)); }
    for p in 0..5usize { let e = cache.player_champion[1][p].unwrap(); if let Some(eff) = e.skill_effect.as_ref() { exp_dmg += &format!("e{}skl->me={} ", p, eff.expected_damage_target(&ctx, e as &dyn AbstractEntity, champ)); } }
    println!("expdmg\t{}", exp_dmg);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        game_ai::calculate_score_parameter(version, &mut rnd, player, &data, &mut dbgf)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(sp) => {
            let b = &sp as *const _ as *const u8;
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen();
            let dbg_after: Vec<u8> = unsafe { std::slice::from_raw_parts(&dbgf as *const _ as *const u8, std::mem::size_of::<DebugFrameData>()).to_vec() };
            println!("RESULT\twave_tag={}\tversion={}\tv3tb={}\tcx={}\tcy={}\trnd_used={}\tdebug_changed={}\tna.len={}\tne.len={}",
                rd::<i64>(b, 0), rd::<usize>(b, 0x14f8), rd::<u8>(b, 0x1500), rd::<usize>(b, 0x14a8), rd::<usize>(b, 0x14b0), r1 != r0, dbg_before != dbg_after,
                rd::<usize>(b, 0x14d0), rd::<usize>(b, 0x14f0));
            println!("PLAYER\t{}", csp(unsafe { b.add(0x918) }));
            let ap: *const u8 = rd(b, 0x14b8); let alen: usize = rd(b, 0x14d0);
            for i in 0..alen.min(8) { println!("ALLY{}\t{}", i, csp(unsafe { ap.add(i * 216) })); }
            let np: *const u8 = rd(b, 0x14d8); let nlen: usize = rd(b, 0x14f0);
            for i in 0..nlen.min(8) { println!("ENEMY{}\t{}", i, csp(unsafe { np.add(i * 216) })); }
            // positioning_score value[0..2] 앞 50B 가 0 인지
            let mut nz = 0usize;
            for k in 0..49usize { for j in 0..50usize { if rd::<u8>(b, 0x9f0 + k * 56 + j) != 0 { nz += 1; } } }
            println!("POSSCORE\tnonzero_bytes={}", nz);
        }
    }
}
