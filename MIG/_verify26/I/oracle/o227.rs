#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치I · 227 v47_tower_focus_position_dangerous 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame(타워 16·챔프 10·미니언 없음).
//!  관측: 반환 bool · 적 top 타워(t) 의 사거리 R = attack.range + 15000 + t.stat_buff_cached.range + growth*(level-1) + t.radius() + champ.radius()
//!        경계(x = t.x + R → true / R+1 → false) · nearest_enemy 조준 예외 · 타워 비활성 tick · attack_effect None.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/I/oracle/o227.rs
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

fn mkeff(damage: usize, range: u64, growth: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: growth, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

struct Args { m: HashMap<String, String> }
impl Args {
    fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) }
    fn s(&self, k: &str, d: &str) -> String { self.m.get(k).cloned().unwrap_or(d.to_string()) }
}

fn radius_of(e: &Entity) -> u64 {
    let mult: i32 = rd(ep(e), 0x470); let r: u64 = rd(ep(e), 0x680);
    if mult == 0 { r } else { r * ((mult as i64 + 100) as u64) / 100 }
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
    let version = a.i("ver", 55) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = a.i("me", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(0, poss[me]).expect("player");
    let champ = cache.player_champion[0][me].expect("champ");
    let cp = ep(champ);
    // 적 top 타워 하나만 남기고 나머지 적 타워는 attack_effect None 으로(간섭 제거)
    let tsel = a.s("tower", "top");
    let tw: &Entity = match tsel.as_str() { "top2" => cache.top_tower2[1].unwrap(), "mid" => cache.mid_tower[1].unwrap(), _ => cache.top_tower[1].unwrap() };
    let all: Vec<&Entity> = cache.iter_towers(1).collect();
    println!("towers_enemy\t{}\tty_tags={:?}", all.len(), all.iter().map(|t| rd::<i64>(ep(*t), 0x68)).collect::<Vec<_>>());
    for t in all.iter() {
        if t.id != tw.id { unsafe { std::ptr::write(&mut (*(ep(*t) as *mut Entity)).attack_effect, None); } }
    }
    let arng = a.i("arng", 100000) as u64; let growth = a.i("growth", 0) as u64;
    if a.i("teff", 1) == 1 { unsafe { std::ptr::write(&mut (*(ep(tw) as *mut Entity)).attack_effect, Some(mkeff(100, arng, growth))); } }
    else { unsafe { std::ptr::write(&mut (*(ep(tw) as *mut Entity)).attack_effect, None); } }
    if a.i("tlvl", -1) >= 0 { wr(ep(tw), 0x5c8, a.i("tlvl", 1) as usize); }
    if a.i("tbrange", -1) >= 0 { wr(ep(tw), 0x438, a.i("tbrange", 0) as u64); }
    if a.i("tmult", -1234) != -1234 { wr(ep(tw), 0x470, a.i("tmult", 0) as i32); }
    if a.i("cmult", -1234) != -1234 { wr(cp, 0x470, a.i("cmult", 0) as i32); }
    // 타워 조준 대상
    let ne = a.s("ne", "none");
    match ne.as_str() {
        "me" => { wr(ep(tw), 0x88, 1i64); wr(ep(tw), 0x90, 0usize); wr(ep(tw), 0x98, champ.id); }
        "a1" => { wr(ep(tw), 0x88, 1i64); wr(ep(tw), 0x90, 0usize); wr(ep(tw), 0x98, cache.player_champion[0][1].unwrap().id); }
        "gone" => { wr(ep(tw), 0x88, 1i64); wr(ep(tw), 0x90, 0usize); wr(ep(tw), 0x98, 999999usize); }
        _ => { wr(ep(tw), 0x88, 0i64); }
    }
    println!("ne_check	tag={}	id={}	ty={:?}", rd::<i64>(ep(tw), 0x88), rd::<usize>(ep(tw), 0x98), format!("{:?}", tw.ty).chars().take(1400).collect::<String>());
    if a.i("a1hp", -1) >= 0 { wr(ep(cache.player_champion[0][1].unwrap()), 0x670, a.i("a1hp", 0) as usize); }
    let tx: u64 = rd(ep(tw), 0x660); let ty: u64 = rd(ep(tw), 0x668);
    let tlvl: usize = rd(ep(tw), 0x5c8); let tbr: u64 = rd(ep(tw), 0x438);
    let R = arng + 15000 + tbr + growth * (tlvl as u64 - 1) + radius_of(tw) + radius_of(champ);
    println!("tower\tid={}\tty={}\tx={}\ty={}\tlvl={}\tbuff_range={}\tt_radius={}\tc_radius={}\tR={}\ttower_attack_disable={}\ttick={}",
        tw.id, rd::<i64>(ep(tw), 0x68), tx, ty, tlvl, tbr, radius_of(tw), radius_of(champ), R, setting.tower_attack_disable_tick, tick);
    let data = OperationData::new(&cache, &ctx, &bb);
    let (x, y) = match a.s("pos", "far").as_str() {
        "far" => (1u64, 1u64),
        "on" => (tx, ty),
        "R" => (tx + R, ty),
        "R1" => (tx + R + 1, ty),
        "Rm" => (tx + R - 1, ty),
        _ => (a.i("x", 1) as u64, a.i("y", 1) as u64),
    };
    if a.i("stance", 0) == 1 {
        let st = game_ai::v47_siege_stance(version, &data, player, tw);
        println!("STANCE	{:?}	champ={}	a1={}", st, champ.id, cache.player_champion[0][1].unwrap().id);
    }
    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        game_ai::v47_tower_focus_position_dangerous(version, &data, player, x, y)
    }));
    match got {
        Ok(v) => println!("RESULT\tval={}\tx={}\ty={}", v, x, y),
        Err(_) => println!("RESULT\tPANIC"),
    }
}
