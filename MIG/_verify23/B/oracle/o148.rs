#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #148 serpen_action_score / #149 epic_action_score (pub) — 직접 호출. argv[1] = 케이스, argv[2] = "s"(세르펜)|"e"(에픽).
//! base = interaction_score(같은 시드의 rng clone) 로 따로 재고, game − base 를 보정치로 본다.
//! 표적: RunAway(risk_epic_damage 0 → 보정 없음 / ≠0 → 200k·hp 게이트 -10 · champions() 에 자기 자신 포함 여부(open[0])) ·
//!       Trace(camp_pos(Serpen 5 / Morgard 4) 사거리 게이트 · attack_dist 식) · Attack 대상 None → -99999 · Around/Ult/Stop → 0 · Positioning → base.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn r64(p: *const Entity, off: usize) -> u64 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const u64) } }
fn w32(p: *mut Entity, off: usize, v: i32) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut i32, v) } }
fn r32(p: *const Entity, off: usize) -> i32 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const i32) } }

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let which = std::env::args().nth(2).unwrap_or("s".into());
    let is_serpen = which == "s";
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let V = 55usize;
    let champ = cache.player_champion[0][0].unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let allies: Vec<*mut Entity> = (1..5).map(|p| cache.player_champion[0][p].unwrap() as *const Entity as *mut Entity).collect();
    let enemy = cache.player_champion[1][0].unwrap();
    let camp = map.camp_pos(if is_serpen { JungleType::Serpen } else { JungleType::Morgard }, true);
    let camp_other = map.camp_pos(if is_serpen { JungleType::Morgard } else { JungleType::Serpen }, true);
    // attack_effect 주입(Trace 경로가 unwrap 한다) — range 100000 · growth 500
    let eff = Effect { ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, range: 100000, growth_range: 500, start_timing: 0,
                       target: CastingTarget::Enemy, attack_type: AttackType::Skill, casting: CastingType::Targeting };
    unsafe { std::ptr::write(&mut (*cp).attack_effect as *mut Option<Effect>, Some(eff)); }
    w64(cp, 0x5c8, 3);      // level 3 → (level-1)*growth = 1000
    w64(cp, 0x680, 10000);  // radius
    let stat_range: u64 = r64(cp, 0x438); let rmult: i32 = r32(cp, 0x470);
    let attack_dist = 100000 + 30000 + stat_range + 2 * 500 + (if rmult == 0 { 10000 } else { 10000 * (rmult as u64 + 100) / 100 });
    println!("champ\tid={} pos=({},{}) hp={} level={} stat_range={} radius_mult={} attack_dist={} camp={:?}", champ.id, champ.x, champ.y, champ.hp, r64(cp, 0x5c8), stat_range, rmult, attack_dist, camp);

    // 아군 전원: 멀리(> 200k) · hp 작게  (기본)
    w64(cp, 0x670, 500);
    for a in &allies { w64(*a, 0x660, 900000); w64(*a, 0x668, 900000); w64(*a, 0x670, 100); }
    w64(cp, 0x660, 100000); w64(cp, 0x668, 100000);

    let zp: std::mem::MaybeUninit<game_ai::ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let mut parameter: game_ai::ScoreParameter = unsafe { zp.assume_init() };
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    let mut rnd2 = rnd.clone();
    let mut dbg: DebugFrameData = Default::default();
    let mut dbg2: DebugFrameData = Default::default();

    // (action, risk_epic_damage, 예측 보정치 Some(delta)|None(관측), 설명)
    let (act, risk, pred, desc): (game_ai::SmallActionPlay, i64, Option<i64>, &str) = match case {
        0 => (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 0, Some(0), "RunAway risk=0 → base"),
        1 => (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 1, Some(0), "RunAway risk=1 · 아군 전원 멀리·저hp → champions() 가 자기 자신을 포함(dist 0·hp==hp)하므로 near=true → 0 (-10 사장)"),
        2 => { w64(allies[0], 0x660, 100000 + 200000); w64(allies[0], 0x668, 100000); w64(allies[0], 0x670, 500);
               (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 1, Some(0), "아군1 dist=200000(경계) hp==champ → 근접 → 0") }
        3 => { w64(allies[0], 0x660, 100000 + 200001); w64(allies[0], 0x668, 100000); w64(allies[0], 0x670, 500);
               (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 1, Some(0), "아군1 dist=200001 → (자기 자신 때문에) 0") }
        4 => { w64(allies[0], 0x660, 100000 + 1000); w64(allies[0], 0x668, 100000); w64(allies[0], 0x670, 499);
               (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 1, Some(0), "아군1 근접 hp=499<500 → (자기 자신 때문에) 0") }
        5 => { w64(allies[3], 0x660, 100000 + 1000); w64(allies[3], 0x668, 100000); w64(allies[3], 0x670, 500);
               (game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, 0)), 1, Some(0), "아군4(마지막) 근접 hp==500 → 0") }
        6 => { let mut a = game_ai::SmallActionRecall::new(&data, player, 0); (game_ai::SmallActionPlay::Recall(a), 1, Some(0), "Recall(Play1) 도 RunAway 팔 → 0(자기 자신)") }
        7 => { w64(cp, 0x660, camp.0); w64(cp, 0x668, camp.1);
               (game_ai::SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, enemy.id, 0)), 0, Some(0), "Trace · 캠프 위 → 0") }
        8 => { w64(cp, 0x660, camp.0 + attack_dist - 1); w64(cp, 0x668, camp.1);
               (game_ai::SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, enemy.id, 0)), 0, Some(0), "Trace · dist=attack_dist-1 → 0") }
        9 => { w64(cp, 0x660, camp.0 + attack_dist); w64(cp, 0x668, camp.1);
               (game_ai::SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, enemy.id, 0)), 0, Some(-99999), "Trace · dist=attack_dist(경계, < 아님) → -99999") }
        10 => { w64(cp, 0x660, camp_other.0); w64(cp, 0x668, camp_other.1);
               (game_ai::SmallActionPlay::Trace(game_ai::SmallActionTrace::new(&data, enemy.id, 0)), 0, None, "Trace · 다른 오브젝트 캠프 위(관측)") }
        11 => (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, 999_999)), 0, Some(-99999), "Attack 대상 없음 → -99999"),
        12 => (game_ai::SmallActionPlay::Stop, 1, Some(0), "Stop → 0"),
        13 => (game_ai::SmallActionPlay::Around(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, enemy.id, 0)), 1, Some(0), "Around → 0"),
        14 => (game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, enemy.id)), 0, None, "Attack 적 챔피언(관측 · calculate_*)"),
        15 => (game_ai::SmallActionPlay::AroundRunAway(game_ai::SmallActionAround::new(V, &mut rnd, &data, player, enemy.id, 0)), 1, Some(0), "AroundRunAway(Play5) 도 RunAway 팔 → 0(자기 자신)"),
        _ => (game_ai::SmallActionPlay::Stop, 0, Some(0), "-"),
    };
    parameter.player.risk_epic_damage = risk as usize;
    { let cs = cache.champions(0, &pool);
      println!("champions(0)	len={}	{:?}", cs.len(), cs.iter().map(|e| (e.id, r64(*e, 0x660), r64(*e, 0x668), r64(*e, 0x670))).collect::<Vec<_>>()); }
    let tag: u8 = unsafe { *((&act as *const _ as *const u8).add(0xb1)) };
    let base = game_ai::interaction_score(V, &mut rnd2, player, &data, &parameter, &act, &mut dbg2);
    let g = if is_serpen { game_ai::plan_legacy::sub_plan::serpen_action_score(V, &mut rnd, player, &data, &parameter, &act, &mut dbg) }
            else { game_ai::plan_legacy::sub_plan::epic_action_score(V, &mut rnd, player, &data, &parameter, &act, &mut dbg) };
    let delta = g - base;
    let m = match pred { Some(p) => if p == delta { "MATCH" } else { "**MISMATCH**" }, None => "OBS" };
    println!("case{}\t{}\ttag={}\trisk={}\tbase={}\tgame={}\tdelta={}\tpred={:?}\t{}\t// {}", case, which, tag, risk, base, g, delta, pred, m, desc);
}
