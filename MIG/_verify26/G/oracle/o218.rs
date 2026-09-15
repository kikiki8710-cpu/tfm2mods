#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치G · 218 AgentVerHamster::update_state 오라클 (pub 메서드 직접 호출).
//!  관측: sret Vec<TurnEvent> 32B 원시값(항상 빈 Vec 인가 · bump 포인터) · &mut self 10704B 전후 diff(오프셋 목록) ·
//!  last_lapse(+0x29ca) · small_action 태그(+0x2909)/start_tick(+0x2858) · last_eval_*(+0x2948/+0x2950/+0x29c8) · prof 전역.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/G/oracle/o218.rs
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
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let dbg = a.i("dbg", 0) == 1;
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: dbg,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
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
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(a.i("seed", 99) as u64);
    let mut agent = AgentVerHamster::new(&mut rnd, version, 0, poss[me]);
    if a.i("init", 1) == 1 { agent.init(); }
    let ab = &agent as *const AgentVerHamster as *const u8;
    // 사전 세팅(can_skip_eval 경로 유도용)
    if a.i("let", -1) >= 0 { wr(ab, 0x2948, a.i("let", 0) as usize); }
    if a.i("lehp", -1) >= 0 { wr(ab, 0x2950, a.i("lehp", 0) as usize); }
    if a.i("lene", -1) >= 0 { wr(ab, 0x29c8, a.i("lene", 0) as u16); }
    if a.i("satag", -1) >= 0 { wr(ab, 0x2909, a.i("satag", 0) as u8); }
    if a.i("sast", -1) >= 0 { wr(ab, 0x2858, a.i("sast", 0) as usize); }
    let before: Vec<u8> = unsafe { std::slice::from_raw_parts(ab, 10704).to_vec() };
    println!("self_before\tversion={}\tlast_lapse={}\tsa_tag={}\tsa_start={}\tlast_eval_tick={}\tlast_eval_hp={}\tlast_ne={}\ttick={}",
        rd::<usize>(ab, 0x2910), rd::<u8>(ab, 0x29ca), rd::<u8>(ab, 0x2909), rd::<usize>(ab, 0x2858), rd::<usize>(ab, 0x2948), rd::<usize>(ab, 0x2950), rd::<u16>(ab, 0x29c8), tick);
    let rnd0 = rnd.clone();
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        agent.update_state(&mut rnd, player, &data)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(v) => {
            let vb = &v as *const _ as *const u8;
            let p0: usize = rd(vb, 0); let p1: usize = rd(vb, 8); let cap: usize = rd(vb, 16); let len: usize = rd(vb, 24);
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen();
            println!("RESULT\tvec_ptr={:#x}\tvec_bump={:#x}\tbump_is_pool={}\tcap={}\tlen={}\tlen_api={}\trnd_used={}",
                p0, p1, p1 == (&pool as *const _ as usize), cap, len, v.len(), r1 != r0);
            std::mem::forget(v);
        }
    }
    let after: Vec<u8> = unsafe { std::slice::from_raw_parts(ab, 10704).to_vec() };
    let mut diffs: Vec<usize> = (0..10704).filter(|&i| before[i] != after[i]).collect();
    // 8B 단위로 묶어 출력
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for &i in &diffs { if let Some(last) = ranges.last_mut() { if i <= last.1 + 1 { last.1 = i; continue; } } ranges.push((i, i)); }
    let rs: Vec<String> = ranges.iter().map(|(s, e)| format!("{:#x}..{:#x}", s, e + 1)).collect();
    println!("self_after\tversion={}\tlast_lapse={}\tsa_tag={}\tsa_start={}\tlast_eval_tick={}\tlast_eval_hp={}\tlast_ne={}",
        rd::<usize>(ab, 0x2910), rd::<u8>(ab, 0x29ca), rd::<u8>(ab, 0x2909), rd::<usize>(ab, 0x2858), rd::<usize>(ab, 0x2948), rd::<usize>(ab, 0x2950), rd::<u16>(ab, 0x29c8));
    println!("self_diff\tn={}\tranges={}", diffs.len(), rs.join(" "));
    println!("lapse_direct\t{}", game_ai::player_awareness_lapse(player, &data));
    std::mem::forget(agent);
    std::process::exit(0);
}
