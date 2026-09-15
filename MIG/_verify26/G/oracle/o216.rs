#![allow(unused, dead_code, non_snake_case)]
//! 26차 배치G · 216 LineDefenseSubPlan::unsafe_v19_non_champion_walkup 오라클 (define hidden → link_name 직접 진입).
//!  한 프로세스 = 한 케이스(argv k=v). self 는 본문이 안 읽으므로(readonly captures(none) deref(3)) 3B 제로 버퍼.
//!  action = 184B 제로 버퍼 + 태그(+0xb1) + target(+0x8). debug 를 ctx.debug=1 로 켜면 infos 문자열로 내부 값(die_tick·score_risk·high_risk·minion_wave)을 관측한다.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify26/G/oracle/o216.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan30unsafe_v19_non_champion_walkup"]
    fn walkup(this: *const u8, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData,
              param: &game_ai::ScoreParameter, action: *const u8, has_runaway: bool, debug: &mut DebugFrameData) -> bool;
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }

fn mkeff_atk(damage: usize, range: u64, casting: CastingType) -> Effect {
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
    let dbg = a.i("dbg", 1) == 1;
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
    let tgt = a.s("tgt", "t");
    let target: &Entity = match tgt.chars().next() {
        Some('e') => cache.player_champion[1][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('a') => cache.player_champion[0][tgt[1..].parse::<usize>().unwrap()].unwrap(),
        Some('t') => cache.top_tower[1].unwrap(),
        Some('n') => cache.nexus[1].unwrap(),
        Some('m') => cache.top_tower[0].unwrap(),   // 아군 타워
        _ => panic!("tgt"),
    };
    let tp = ep(target);
    // 내 위치: target 에서 mdist(x 방향) — 기본 200000
    let mdist = a.i("mdist", 200000);
    if mdist >= 0 { wr(cp, 0x660, target.x + mdist as u64); wr(cp, 0x668, target.y); }
    if a.i("mhp", -1) >= 0 { wr(cp, 0x670, a.i("mhp", 0) as usize); }
    // 내 attack_effect(atk=damage, arng=range)
    let atk = a.i("atk", 50);
    let casting = if a.i("cpos", 0) == 1 { CastingType::Position } else { CastingType::Targeting };
    if atk >= 0 { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, Some(mkeff_atk(atk as usize, a.i("arng", 100000) as u64, casting))); } }
    else { unsafe { std::ptr::write(&mut (*(cp as *mut Entity)).attack_effect, None); } }
    if a.i("tvis", -1) >= 0 { wr(tp, 0x38, a.i("tvis", 0) as i64); }   // target.visible_state[0] 태그
    // 적 챔피언: enemy=D → 내 위치에서 D · vis=1 → last_visible · eatk
    let enemy = a.i("enemy", -1);
    let nenemy = a.i("nenemy", 5) as usize;
    let vis = a.i("vis", 1);
    let mut k = 0usize;
    for p in 0..5usize {
        let e = cache.player_champion[1][p].unwrap(); let eb = ep(e);
        if vis == 1 { bb[1].last_visible[p] = tick; }
        if a.i("evis", 0) == 1 { wr(eb, 0x38, 0i64); }
        if a.i("eatk", -1) >= 0 { unsafe { std::ptr::write(&mut (*(eb as *mut Entity)).attack_effect, Some(mkeff_atk(a.i("eatk", 0) as usize, a.i("erng", 100000) as u64, CastingType::Targeting))); } }
        if enemy >= 0 && k < nenemy { wr(eb, 0x660, champ.x + enemy as u64); wr(eb, 0x668, champ.y + (k as u64) * 1000); k += 1; }
        else if enemy >= 0 { wr(eb, 0x660, 1u64); wr(eb, 0x668, 1u64); }
        let enemyt = a.i("enemyt", -1);
        if enemyt >= 0 && k < nenemy { wr(eb, 0x660, target.x + enemyt as u64); wr(eb, 0x668, target.y + (k as u64) * 1000); k += 1; }
        else if enemyt >= 0 { wr(eb, 0x660, 1u64); wr(eb, 0x668, 1u64); }
    }
    // ScoreParameter
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default()); }
    let pbase = sp_ptr as *const u8;
    wr(pbase, 0x918 + 0x18, 8usize); wr(pbase, 0x918 + 0x20, &pool as *const _ as usize); wr(pbase, 0x918 + 0x38, 8usize); wr(pbase, 0x918 + 0x40, &pool as *const _ as usize);
    for kv in a.s("spf", "").split(',').filter(|x| !x.is_empty()) { let (o, v) = kv.split_once(':').unwrap(); wr(pbase, usize::from_str_radix(o.trim_start_matches("0x"), 16).unwrap(), v.parse::<i64>().unwrap()); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    // action 184B: 태그 +0xb1, target +0x8
    let mut act = [0u8; 184];
    let ab = act.as_ptr();
    wr(ab, 0xb1, a.i("atag", 15) as u8);
    wr(ab, 0x8, target.id);
    let this = [0u8; 3];
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(a.i("seed", 99) as u64);
    let rnd0 = rnd.clone();
    let has_runaway = a.i("runaway", 0) == 1;
    // 보조 관측
    let in_range = champ.attack_effect.as_ref().map(|e| e.is_in_range(champ, target));
    let dmg = champ.attack_effect.as_ref().map(|e| e.expected_damage_target(&ctx, champ as &dyn AbstractEntity, target));
    let tvis: i64 = rd(tp, 0x38);
    // 예상 walkup 위치(가시 대상 · 이동계 아님 가정): range = eff.range(champ) + caster_radius(Targeting) + range_adjust + target.radius
    let (wx, wy) = if let Some(e) = champ.attack_effect.as_ref() {
        let dx = champ.x as i64 - target.x as i64; let dy = champ.y as i64 - target.y as i64;
        let sz = game_core::utils::isqrt(dx*dx + dy*dy);
        let cr = if matches!(e.casting, CastingType::Targeting) { champ.radius() as u64 } else { 0 };
        let range = e.range(champ) + cr + e.range_adjust(champ, target) + target.radius() as u64;
        let backoff: u64 = if a.i("atag", 15) == 18 { 150000 } else { 15000 };
        let fd = range.saturating_sub(backoff) as i64;
        let (x, y) = Game::adjust_position(&map, &setting, target.x as i64 + fd * dx / sz.max(1), target.y as i64 + fd * dy / sz.max(1));
        (x, y)
    } else { (0, 0) };
    let tps = setting.tick_per_second;
    let mut att = Vec::new();
    for p in 0..5usize { let e = cache.player_champion[1][p].unwrap();
        let visible = rd::<i64>(ep(e), 0x38) == 0 || bb[1].is_recent_visible(&game as &dyn AbstractGame, player, e);
        let r = game_ai::plan_legacy::old::max_range_nearly_can_use(e, champ, tps) + 20000;
        let d2 = game_core::utils::distance_sq(e.x, e.y, wx, wy);
        if visible && r != 0 && d2 <= r * r { att.push(format!("e{}(r={},d2={})", p, r, d2)); } }
    println!("est\twalkup=({},{})\tattackers={:?}\trange_eff={:?}", wx, wy, att, champ.attack_effect.as_ref().map(|e| e.range(champ)));
    println!("world\tme=({},{})\ttarget={}\tid={}\tty={}\thp={}\tpos=({},{})\ttvis0={}\tin_range={:?}\tdmg={:?}\tchamp_hp={}",
        champ.x, champ.y, tgt, target.id, rd::<i64>(tp, 0x68), target.hp, target.x, target.y, tvis, in_range, dmg, champ.hp);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        walkup(this.as_ptr(), version, &mut rnd, player, &data, param, ab, has_runaway, &mut dbgf)
    }));
    match res {
        Err(e) => {
            let msg = e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or("?".into());
            println!("RESULT\tPANIC\t{}", msg.replace('\n', " "));
        }
        Ok(v) => {
            let r1: u64 = rnd.clone().gen(); let r0: u64 = rnd0.clone().gen();
            let infos: Vec<String> = dbgf.infos.get(&champ.id).cloned().unwrap_or_default();
            println!("RESULT\tgot={}\trnd_used={}\tinfos={:?}", v, r1 != r0, infos);
        }
    }
    std::process::exit(0);
}
