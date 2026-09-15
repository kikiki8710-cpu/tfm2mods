#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치D · 208 AgentVerHamster::get_input 오라클(pub 직접 호출).
//!  세계 = TEMPLATE mkgame(챔프 10 · 타워 16 · 미니언 0). agent = AgentVerHamster::new(&mut rnd, version, team, position)(pub).
//!  관측: sret 64B 원문 · &mut self 10,704B 전후 diff(오프셋 목록) · rnd 소비(gen_range(min..=max) 1회 예측 대조) ·
//!        next_input_tick = tick + delay/100 예측 · stay_events 원소(208B) · freeze_* 시퀀스(같은 프로세스에서 3회 호출 — 이 함수 자체 TLS 0).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/D/oracle/o208.rs
//!  실행: o208.exe version=2 tick=3000 debug=1 early=0 seq=3 me=0
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;
use game_ai::AgentVerHamster;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

struct Args { m: HashMap<String, String> }
impl Args { fn i(&self, k: &str, d: i64) -> i64 { self.m.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) } }

const SELF_SZ: usize = 10704;
fn snap(p: *const u8) -> Vec<u8> { unsafe { std::slice::from_raw_parts(p, SELF_SZ).to_vec() } }
fn diff_ranges(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new(); let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] { let s = i; while i < a.len() && a[i] != b[i] { i += 1; } out.push((s, i)); } else { i += 1; }
    }
    out
}
fn name_of(off: usize) -> &'static str {
    match off {
        0x0..=0xdf => "debug", 0xe0..=0x1bf => "big_debug", 0x1c0..=0x29f => "small_debug", 0x2a0..=0x2cf => "stay_home_full_nobuy_stack",
        0x2d0..=0x40f => "static_rnd", 0x410..=0x42f => "noinput_bucket", 0x430..=0x43f => "freeze_anchor", 0x440..=0x46f => "freeze_eps",
        0x470..=0x49f => "freeze_plan", 0x4a0..=0x4cf => "freeze_field_action", 0x4d0..=0x4ff => "stay_home_full_nobuy_plan", 0x500..=0x52f => "noinput_lapse_action",
        0x530..=0x1d47 => "plan_system", 0x1d48..=0x1d5f => "failed_action", 0x1d60..=0x1d77 => "events", 0x1d78..=0x1d8f => "stay_events",
        0x1d90..=0x2857 => "positioning_score", 0x2858..=0x290f => "small_action", 0x2910..=0x2917 => "version", 0x2918..=0x291f => "small_action_score",
        0x2920..=0x2927 => "last_battle_tick", 0x2928..=0x292f => "trace_escape_target", 0x2930..=0x2937 => "trace_escape_ticks", 0x2938..=0x293f => "trace_escape_last_tick",
        0x2940..=0x2947 => "next_input_tick", 0x2948..=0x294f => "last_eval_tick", 0x2950..=0x2957 => "last_eval_hp", 0x2958..=0x295f => "input_chances",
        0x2960..=0x2967 => "freeze_since", 0x2968..=0x296f => "freeze_eps_lapse", 0x2970..=0x2977 => "freeze_eps_dead_target", 0x2978..=0x297f => "freeze_ticks",
        0x2980..=0x2997 => "freeze_pos", 0x2998..=0x299f => "lapse_chances", 0x29a0..=0x29a7 => "last_combat_tick", 0x29a8..=0x29af => "stay_home_full_nobuy",
        0x29b0..=0x29c7 => "noinput_lapse_pos", 0x29c8..=0x29c9 => "last_eval_nearby_enemies", 0x29ca => "last_lapse", 0x29cb => "freeze_fired", _ => "?",
    }
}
fn show_diff(tag: &str, a: &[u8], b: &[u8]) {
    let rs = diff_ranges(a, b);
    let mut s = String::new();
    for (x, y) in &rs {
        let mut sub = name_of(*x).to_string();
        if *x >= 0x530 && *x < 0x1d48 { sub = format!("plan_system+{:#x}", x - 0x530); }
        if *x >= 0x2858 && *x < 0x2910 { sub = format!("small_action+{:#x}", x - 0x2858); }
        let w = y - x;
        let val = if w == 8 { format!("{}->{}", u64::from_le_bytes(a[*x..*x+8].try_into().unwrap()), u64::from_le_bytes(b[*x..*x+8].try_into().unwrap())) }
                  else if w == 1 { format!("{}->{}", a[*x], b[*x]) } else { format!("{}B", w) };
        s += &format!("{:#x}({}:{}) ", x, sub, val);
    }
    println!("{}\tranges={}\t{}", tag, rs.len(), s);
}
fn u64at(b: &[u8], o: usize) -> u64 { u64::from_le_bytes(b[o..o+8].try_into().unwrap()) }

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
    let debug = a.i("debug", 0) == 1;
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick0 = a.i("tick", 3000) as usize;
    game.set_tick(tick0);
    let version = a.i("version", 2) as usize;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let me = a.i("me", 0) as usize;
    let team = a.i("team", 0) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let player = game.get_player_by_position(team, poss[me]).expect("player");
    let pb = player as *const PlayerState as *const u8;
    let champ = cache.player_champion[team][me].expect("champ");
    let cp = ep(champ);
    if a.i("mx", -1) >= 0 { wr(cp, 0x660, a.i("mx", 0) as u64); wr(cp, 0x668, a.i("my", 0) as u64); }
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    if a.i("atk", 0) == 1 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff(50, 100000))); } }
    let ap: &AthleteParameter = unsafe { &*(pb.add(0x180) as *const AthleteParameter) };
    let (dmin, dmax) = (ap.input_delay_min(), ap.input_delay_max());
    println!("player\tteam={}\tpos_tag={}\tinput_delay=({},{})\tchamp=({},{}) hp={}/{} id={}", rd::<usize>(pb, 0x930), rd::<i32>(pb, 0x9c0), dmin, dmax, champ.x, champ.y, champ.hp, champ.stat_cached.hp, champ.id);
    let (flx, fly, frx, fry) = map.fountains[team];
    println!("fountain\t({},{})-({},{})\tin_rect={}", flx, fly, frx, fry, flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry);
    let seed = a.i("seed", 99) as u64;
    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd_new = rand::rngs::StdRng::seed_from_u64(seed + 1000);
    let mut agent = AgentVerHamster::new(&mut rnd_new, version, team, poss[me]);
    let sp = &agent as *const AgentVerHamster as *const u8;
    println!("agent_init\tversion={}\tnext_input_tick={}\tinput_chances={}\tsa_tag={}\tplan_tag={}\tlast_lapse={}\tfreeze_since={}\tlast_combat_tick={}",
             rd::<usize>(sp, 0x2910), rd::<usize>(sp, 0x2940), rd::<usize>(sp, 0x2958), rd::<u8>(sp, 0x2909), rd::<i64>(sp, 0xb18), rd::<u8>(sp, 0x29ca), rd::<usize>(sp, 0x2960), rd::<usize>(sp, 0x29a0));
    if a.i("early", 0) == 1 { wr(sp, 0x2940, tick0 + 100); }
    if a.i("lapse", -1) >= 0 { wr(sp, 0x29ca, a.i("lapse", 0) as u8); }
    if a.i("satag", -1) >= 0 { wr(sp, 0x2909, a.i("satag", 0) as u8); }
    if a.i("lct", -1) >= 0 { wr(sp, 0x29a0, a.i("lct", 0) as usize); }
    let seq = a.i("seq", 1) as usize;
    let mut tick = tick0;
    for k in 0..seq {
        if k > 0 {
            // 다음 호출: next_input_tick 까지 진행(+extra)
            let nit: usize = rd(sp, 0x2940);
            tick = std::cmp::max(nit, tick) + a.i("extra", 0) as usize;
            unsafe { (*(&game as *const Game as *mut Game)).set_tick(tick); }
        }
        if a.i("each", 0) == 1 { if a.i("lapse", -1) >= 0 { wr(sp, 0x29ca, a.i("lapse", 0) as u8); } if a.i("satag", -1) >= 0 { wr(sp, 0x2909, a.i("satag", 0) as u8); } }
        let before = snap(sp);
        let rnd0 = rnd.clone();
        // 예측: delay 롤 1회 = gen_range(min..=max)/100
        let mut rp = rnd.clone();
        let roll: usize = rp.gen_range(dmin..=dmax);
        let pred_next = tick + roll / 100;
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            agent.get_input(&mut rnd, player, &data)
        }));
        match res {
            Err(e) => {
                let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
                println!("RESULT\tk={}\tPANIC\t{}", k, msg.replace('\n', " ")); return;
            }
            Ok(r) => {
                let rb = &r as *const _ as *const u8;
                let raw: [u8; 64] = unsafe { std::ptr::read(rb as *const [u8; 64]) };
                let itag: i64 = rd(rb, 0);
                println!("RESULT\tk={}\ttick={}\tinput_tag={}\tvec=(ptr={:#x} bump={:#x} cap={} len={})\tpool={:#x}\traw={}", k, tick, itag,
                         rd::<usize>(rb, 32), rd::<usize>(rb, 40), rd::<usize>(rb, 48), rd::<usize>(rb, 56), &pool as *const bumpalo::Bump as usize,
                         raw.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(""));
                std::mem::forget(r);
            }
        }
        let after = snap(sp);
        let nit: usize = rd(sp, 0x2940);
        // rnd 소비 대조: 정확히 1회 gen_range 만 소비됐는가(rp 와 다음 u64 비교) · 전혀 안 소비됐는가(rnd0)
        let n_after: u64 = rnd.clone().gen(); let n_rp: u64 = rp.clone().gen(); let n_0: u64 = rnd0.clone().gen();
        println!("rnd\tk={}\tuntouched={}\texactly_one_gen_range={}\tpred_next_input_tick={}\tactual={}\tmatch={}", k, n_after == n_0, n_after == n_rp, pred_next, nit, pred_next == nit);
        show_diff(&format!("self_diff\tk={}", k), &before, &after);
        println!("counters\tk={}\tinput_chances={}\tlapse_chances={}\tnoinput_bucket={:?}\tstay_home_full_nobuy={}\tfreeze_since={}\tfreeze_fired={}\tfreeze_ticks={}\tfreeze_eps={:?}\tfreeze_plan={:?}\tfreeze_pos={:?}\tfreeze_field_action={:?}\tanchor=({},{})\tlast_combat_tick={}\tlast_lapse={}\tsa_tag={}\tstay_events_len={}",
            k, rd::<usize>(sp, 0x2958), rd::<usize>(sp, 0x2998), [rd::<usize>(sp,0x410),rd::<usize>(sp,0x418),rd::<usize>(sp,0x420),rd::<usize>(sp,0x428)], rd::<usize>(sp, 0x29a8),
            rd::<usize>(sp, 0x2960), rd::<u8>(sp, 0x29cb), rd::<usize>(sp, 0x2978),
            (0..6).map(|i| rd::<usize>(sp, 0x440 + i*8)).collect::<Vec<_>>(), (0..6).map(|i| rd::<usize>(sp, 0x470 + i*8)).collect::<Vec<_>>(),
            (0..3).map(|i| rd::<usize>(sp, 0x2980 + i*8)).collect::<Vec<_>>(), (0..6).map(|i| rd::<usize>(sp, 0x4a0 + i*8)).collect::<Vec<_>>(),
            rd::<u64>(sp, 0x430), rd::<u64>(sp, 0x438), rd::<usize>(sp, 0x29a0), rd::<u8>(sp, 0x29ca), rd::<u8>(sp, 0x2909), rd::<usize>(sp, 0x1d88));
        // stay_events 마지막 원소
        let len: usize = rd(sp, 0x1d88);
        if len > 0 {
            let ptr: *const u8 = rd(sp, 0x1d80);
            let e = unsafe { ptr.add((len - 1) * 208) };
            let sread = |o: usize| -> String { let p: *const u8 = rd(e, o + 8); let l: usize = rd(e, o + 16); unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(p, l)).to_string() } };
            let tk: *const u8 = rd(e, 0x78); let tl: usize = rd(e, 0x80);
            let tks = unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(tk, tl)).to_string() };
            println!("stay_event\tk={}\tgoal=({},{},{})\tplan_label={}\tplan_detail={}\tsub_plan={}\taction={}\ttarget_kind={:?}\ttick={}\tteam={}\tpos=({},{})\thp_pct={}\tnear_enemy={}\tnear_ally={}\tsince_combat={}\tposition={}\tcan_buy={}\tin_lapse={}\tkind={}\tpos_band={}",
                k, rd::<i64>(e, 0), rd::<u64>(e, 8), rd::<u64>(e, 16), sread(0x18), sread(0x30), sread(0x48), sread(0x60), tks,
                rd::<usize>(e, 0x88), rd::<usize>(e, 0x90), rd::<u64>(e, 0x98), rd::<u64>(e, 0xa0), rd::<usize>(e, 0xa8), rd::<usize>(e, 0xb0), rd::<usize>(e, 0xb8), rd::<usize>(e, 0xc0),
                rd::<i32>(e, 0xc8), rd::<u8>(e, 0xcc), rd::<u8>(e, 0xcd), rd::<u8>(e, 0xce), rd::<u8>(e, 0xcf));
        }
    }
    std::mem::forget(agent);
}
