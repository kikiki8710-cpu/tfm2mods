#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치D 오라클 — 256 LegacyPlanHandler::update_on_dead(pub) — 수법 ⓒ 구조체 통째 바이트 diff(6168B) 로 `mem` writes 표 대조.
//!  케이스: 기본(new 직후 plan = 수동이면 교체 없음) / `force=2` → plan 태그를 2(ForcePassive · 페이로드 없음 · is_passive 아님) 로 덮어 교체 경로 강제.
//!  사전에 v2_egowave(0x1815)=7 · v2_obj_part(0x1800)=1 을 심어 L639/L640 리셋을 본다. tick 은 set_tick 으로 0 아니게.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify27/D/oracle/o27D_uod.rs → %TEMP%\tfm2_spanprobe\o27D_uod.exe
use game_core::*;
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }

struct Args { m: HashMap<String, i64> }
impl Args { fn get(&self, k: &str, d: i64) -> i64 { *self.m.get(k).unwrap_or(&d) } fn has(&self, k: &str) -> bool { self.m.contains_key(k) } }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    let mut m = HashMap::new();
    for s in av.iter().skip(1) { if let Some((k, v)) = s.split_once('=') { if let Ok(x) = v.parse::<i64>() { m.insert(k.to_string(), x); } } }
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
    let tick = a.get("tick", 3000) as usize;
    game.set_tick(tick);
    let version = a.get("version", 2) as usize;
    let team = a.get("team", 0) as usize; let posi = a.get("pos", 2) as usize;
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, poss[posi]).expect("player");
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let mut h = game_ai::plan_legacy::handler::LegacyPlanHandler::new(version, &mut rnd, team, poss[posi]);
    let hp = &h as *const _ as *const u8;
    let tag0: i64 = rd(hp, 0x5e8);
    wr(hp, 0x1815, 7u8); wr(hp, 0x1800, 1u8);
    if a.has("force") { wr(hp, 0x5e8, a.get("force", 2) as i64); }
    let tag1: i64 = rd(hp, 0x5e8);
    let before: Vec<u8> = (0..6168usize).map(|i| rd::<u8>(hp, i)).collect();
    let rnd_before: [u8; 320] = unsafe { std::mem::transmute_copy(&rnd) };
    let mut dbg: DebugFrameData = Default::default();
    h.update_on_dead(version, &mut rnd, player, &data, &mut dbg);
    let after: Vec<u8> = (0..6168usize).map(|i| rd::<u8>(hp, i)).collect();
    let rnd_after: [u8; 320] = unsafe { std::mem::transmute_copy(&rnd) };
    let tag2: i64 = rd(hp, 0x5e8);
    println!("world tick={} plan_tag new={} pre={} post={} rnd_changed={}", tick, tag0, tag1, tag2, rnd_before != rnd_after);
    // 변경 오프셋 구간 출력
    let mut i = 0usize; let mut ranges = Vec::new();
    while i < 6168 { if before[i] != after[i] { let s = i; while i < 6168 && before[i] != after[i] { i += 1; } ranges.push((s, i)); } else { i += 1; } }
    for (s, e) in &ranges {
        let name = match *s { x if x < 0xf8 => "data(GoalData)", x if x < 0x5e8 => "team_plan", x if x < 0x7b0 => "plan", x if x < 0x7c8 => "misunderstood", x if x < 0x7e0 => "chats", x if x < 0x7f8 => "chats_wait", x if x < 0x810 => "received_chats",
            x if x >= 0x15f8 && x < 0x1608 => "ff_battle_exit", x if x >= 0x1608 && x < 0x1610 => "ff_battle_exit_latch", x if x >= 0x1610 && x < 0x1620 => "mf_swap", 0x1800 | 0x1801 => "v2_obj_part", 0x1815 => "v2_egowave", _ => "?" };
        let bytes: Vec<String> = (*s..(*e).min(s + 16)).map(|k| format!("{:02x}", after[k])).collect();
        println!("changed\t{:#x}..{:#x}\t{}\t{}\t{}", s, e, e - s, name, bytes.join(" "));
    }
    println!("v2_egowave={} v2_obj_part_tag={} mf_swap=({}, {}) ff_exit0={} ff_exit2={}", rd::<u8>(hp, 0x1815), rd::<u8>(hp, 0x1800), rd::<u8>(hp, 0x1610), rd::<u64>(hp, 0x1618), rd::<u8>(hp, 0x1600), rd::<u64>(hp, 0x15f8));
    println!("ranges={}", ranges.len());
}
