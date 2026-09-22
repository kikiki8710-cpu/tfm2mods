#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치B · 196 LineDefenseSubPlan::action_candidates 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 시나리오(argv[1]) — 콜리에 TLS 메모(POS_EVAL_CACHE·DIE_TICK_CACHE·SIEGE_STANCE_CACHE …)가 있어 프로세스를 가른다.
//!  관측: sret Vec 원소 수 · 원소 태그(+0xb1) · variant 별 live 바이트(initializes 범위) · debug.infos · 결정성.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/B/oracle/o196.rs → %TEMP%\tfm2_spanprobe\o196.exe
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }
fn ep(e: &Entity) -> *const u8 { e as *const Entity as *const u8 }
fn hex(p: *const u8, a: usize, b: usize) -> String { (a..b).map(|k| format!("{:02x}", rd::<u8>(p, k))).collect::<Vec<_>>().join("") }

fn mkeff(damage: usize, range: u64) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}
fn tagname(t: u8) -> &'static str {
    match t { 3 => "RunAway", 4 => "Recall", 5 => "Around", 6 => "AroundHide", 7 => "AroundRegion", 8 => "AroundRunAway", 9 => "Positioning",
              11 => "AroundPositionBush", 12 => "AroundBush", 13 => "LaneMinionPosition", 14 => "Trace", 15 => "Attack", 16 => "Skill", 17 => "Skill2", 18 => "Ult", 19 => "Stop", 255 => "None", _ => "AroundPosition?(untagged)" }
}
/// variant 별 live 바이트(콜리 define 의 initializes 범위 + 이 함수의 +177 태그) 만 찍는다 — 나머지는 alloca 잔재라 프로세스마다 다를 수 있다
fn live_dump(p: *const u8) -> String {
    let t: u8 = rd(p, 177);
    let body = match t {
        3 => format!("[0,56)={} +125={:02x} [128,132)={}", hex(p, 0, 56), rd::<u8>(p, 125), hex(p, 128, 132)),
        5 | 8 => format!("[0,56)={} +125={:02x} [128,130)={}", hex(p, 0, 56), rd::<u8>(p, 125), hex(p, 128, 130)),
        14 => format!("[0,16)={} +85={:02x} [88,150)={}", hex(p, 0, 16), rd::<u8>(p, 85), hex(p, 88, 150)),
        15 | 16 | 17 | 18 => format!("[0,17)={}", hex(p, 0, 17)),
        13 => format!("[0,48)={} +117={:02x} +120={:02x}", hex(p, 0, 48), rd::<u8>(p, 117), rd::<u8>(p, 120)),
        _ => format!("[0,184)={}", hex(p, 0, 184)),
    };
    format!("tag={}({}) {}", t, tagname(t), body)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let scen = arg(&a, 1, 0);
    if a.len() > 99 { let _ = game_ai::position_eval_at as *const (); }
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let dbg_ctx = scen == 3 || scen == 9;
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: dbg_ctx,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick0 = arg(&a, 2, 1000) as usize;
    game.set_tick(tick0);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).expect("player");
    let champ = cache.player_champion[0][0].expect("champ");
    let seed = arg(&a, 3, 7) as u64;
    let ver = arg(&a, 4, 55) as usize;
    println!("game\ttick={}\tpid={}\tchamp_id={}\tchamp_xy=({},{})\tseed={}\tver={}\tctx.debug={}", tick0, player.info.id, champ.id, champ.x, champ.y, seed, ver, dbg_ctx);

    // 최근접 적 타워(L452~454: iter_towers_without_nexus(1-team) · can_target · min dist²)
    let mut best: Option<&Entity> = None; let mut bd = u64::MAX;
    for t in cache.iter_towers_without_nexus(1) {
        let tp = ep(t);
        let can: bool = rd(tp, 0x6b9); let blk: usize = rd(tp, 0x6a0);
        if !(can && blk == 0) { continue; }
        let dx = t.x.abs_diff(champ.x); let dy = t.y.abs_diff(champ.y);
        let d = dx * dx + dy * dy;
        if d < bd { bd = d; best = Some(t); }
    }
    let et = best.expect("enemy tower");
    println!("nearest_enemy_tower\tid={}\tdist2={}\tty_tag={}\tnearest_enemy_tag={}", et.id, bd, rd::<i64>(ep(et), 0x68), rd::<i64>(ep(et), 0x88));
    // 아군 최근접 타워(L160/L230: cache.tower(line=Top, team) 1순위 = top_tower[0])
    let my_tower = cache.top_tower[0].expect("my top tower");
    println!("my_top_tower\tid={}", my_tower.id);

    match scen {
        // 2: 적 타워가 나를 물고 있음(L771 nearest_enemy.1 == champ.id) → v47 소커가 내가 아니면 L776~778 (RunAway with_skill=true 단독)
        2 => { wr(ep(et), 0x88, 1i64); wr(ep(et), 0x98, champ.id); println!("setup\tenemy_tower.nearest_enemy=(?,champ.id)"); }
        // 5: 적 챔피언(팀1 Top)을 내 앞 d=150000 에 가시 상태로 · 내 사거리 R=100000 · 적 사거리 0 → L300~302 (dist>mr² && emr<mr) Trace 기대(+ L353 Around) → L837 max score
        // 6: 적 챔피언 d=50000 · 내 사거리 0 · 적 사거리 100000 → L308 (dist<=emr_near²) runaway → L336 RunAway(with_skill=false)(+ Around)
        // 7: 나를 적 타워 앞 d=50000 으로 · 내 사거리 100000 → attack_tower_action L385 통과 → v22 → Attack(tower) 후보 → closure#3 L472 유지 → ret 비지 않으면 L876
        5 | 6 => {
            let e = cache.player_champion[1][0].expect("enemy");
            let d: u64 = if scen == 5 { 150000 } else { 50000 };
            wr(ep(e), 0x660, champ.x + d); wr(ep(e), 0x668, champ.y);
            wr(ep(e), 0x38, 0i64);   // visible_state[team0] = Visible
            let (r_me, r_en) = if scen == 5 { (100000u64, 0u64) } else { (0u64, 100000u64) };
            unsafe { std::ptr::write(&mut (*(ep(champ) as *mut Entity)).attack_effect, Some(mkeff(100, r_me)));
                     std::ptr::write(&mut (*(ep(e) as *mut Entity)).attack_effect, Some(mkeff(100, r_en))); }
            println!("setup	enemy id={} at ({},{}) visible · my_range={} enemy_range={}", e.id, e.x, e.y, r_me, r_en);
        }
        7 => {
            wr(ep(champ), 0x660, et.x.saturating_sub(50000)); wr(ep(champ), 0x668, et.y);
            unsafe { std::ptr::write(&mut (*(ep(champ) as *mut Entity)).attack_effect, Some(mkeff(100, 100000))); }
            println!("setup	me at ({},{}) near enemy tower {} at ({},{}) · my_range=100000", champ.x, champ.y, et.id, et.x, et.y);
        }
        // 8/9: 나를 적 타워 앞 d=50000 · 적 챔피언을 내 옆 d=50000(가시) · 내/적 사거리 100000 → L308 runaway · L341~344 has_enemy_minion_in_range(타워) → L351 return [RunAway(with_skill=false)] 기대(단 attack_tower_action→Attack 이 act_actions 에 남으면 L876 [Attack]) · 9 = ctx.debug
        8 | 9 => {
            wr(ep(champ), 0x660, et.x.saturating_sub(50000)); wr(ep(champ), 0x668, et.y);
            let e = cache.player_champion[1][0].expect("enemy");
            wr(ep(e), 0x660, champ.x.saturating_sub(50000)); wr(ep(e), 0x668, champ.y);
            wr(ep(e), 0x38, 0i64);
            unsafe { std::ptr::write(&mut (*(ep(champ) as *mut Entity)).attack_effect, Some(mkeff(100, 100000)));
                     std::ptr::write(&mut (*(ep(e) as *mut Entity)).attack_effect, Some(mkeff(100, 100000))); }
            println!("setup	me at ({},{}) tower {} at ({},{}) enemy {} at ({},{}) · ranges 100000/100000", champ.x, champ.y, et.id, et.x, et.y, e.id, e.x, e.y);
        }
        _ => {}
    }

    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut sub = game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new(ver, &mut rnd, player, &data, LineType::Top, game_ai::MinionActionType::Normal, LineStyle::Defensive);
    let sub_before: [u8; 3] = unsafe { std::ptr::read(&sub as *const _ as *const [u8; 3]) };
    // ScoreParameter: 0 버퍼 + positioning_score=Default + version (24차 F 선례 — wave_snapshot/positioning_score/version 만 이 함수 범위에서 읽힌다)
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe {
        std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), PositioningScoreData::default());
        std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).version), ver);
    }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    // team_plan: 이 함수가 안 읽는다(%7 readnone) — scen 4 는 쓰레기 바이트로 채워 확인
    let tp_default = game_ai::plan_legacy::team_plan::TeamPlan::default();
    let tp_size = std::mem::size_of::<game_ai::plan_legacy::team_plan::TeamPlan>();
    let garbage: Vec<u8> = vec![0xAB; tp_size.max(8)];
    let team_plan: &game_ai::plan_legacy::team_plan::TeamPlan = if scen == 4 { unsafe { &*(garbage.as_ptr() as *const game_ai::plan_legacy::team_plan::TeamPlan) } } else { &tp_default };
    println!("team_plan\tsize={}\tgarbage={}", tp_size, scen == 4);
    let mut dbg: DebugFrameData = Default::default();

    let rnd_snap = rnd.clone();
    let res = game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates(&mut sub, ver, &mut rnd, player, &data, param, team_plan, &mut dbg);
    let sub_after: [u8; 3] = unsafe { std::ptr::read(&sub as *const _ as *const [u8; 3]) };
    println!("self_bytes\tbefore={:02x?}\tafter={:02x?}\tSELF_UNCHANGED={}", sub_before, sub_after, sub_before == sub_after);
    println!("rnd_advanced={}", rnd != rnd_snap);
    println!("sret\tlen={}\tcap={}", res.len(), res.capacity());
    for (i, e) in res.iter().enumerate() {
        let p = e as *const game_ai::SmallActionPlay as *const u8;
        println!("elem[{}]\t{}", i, live_dump(p));
        let t: u8 = rd(p, 177);
        if t == 5 || t == 8 { println!("   around.target={}\tend_delay={}\tpurpose={}\tescape={}\tSTART_TICK={}", rd::<usize>(p, 8), rd::<usize>(p, 0x30), rd::<u8>(p, 0x80), rd::<u8>(p, 0x81), rd::<usize>(p, 0)); }
        if t == 3 { println!("   runaway.end_delay={}\twith_skill={}\twith_ult={}\tdodge={}\tcommitted={}\tgoal=({},{})", rd::<usize>(p, 0x18), rd::<u8>(p, 0x80), rd::<u8>(p, 0x81), rd::<u8>(p, 0x82), rd::<u8>(p, 0x83), rd::<u64>(p, 8), rd::<u64>(p, 0x10)); }
        if t == 15 || t == 16 || t == 17 { println!("   cast.target_id={}", rd::<usize>(p, 8)); }
        if t == 14 { println!("   trace.target={}", rd::<usize>(p, 0)); }
    }
    println!("debug.infos\t{:?}", dbg.infos);
    println!("purpose_pred\t{}", { let p = game_ai::line_phase_position_eval_purpose(player, &data); unsafe { *(&p as *const _ as *const u8) } });
}
