#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치C 오라클 — 150(pub) · 156(pub) · 151/153/154/157(define hidden → link_name 직접 링크).
//!  한 프로세스 = 한 케이스(argv). 세계 = TEMPLATE mkgame + (150 cover 케이스만) 미니언 600틱.
//!  엔티티 변조 = raw 포인터 write_volatile(TEMPLATE 함정 ⑦).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify23/C/oracle/o23c.rs
//! 실행: o23c.exe <fn> <args...>   (드라이버 = run23c.py)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }

extern "Rust" {
    #[link_name = "_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_16SmallActionSkill9get_input"]
    fn skill_get_input(s: &mut game_ai::SmallActionSkill, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData) -> Option<Input>;
    #[link_name = "_RNvMs3_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_17SmallActionSkill29get_input"]
    fn skill2_get_input(s: &mut game_ai::SmallActionSkill2, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData) -> Option<Input>;
    #[link_name = "_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_14SmallActionUlt9get_input"]
    fn ult_get_input(s: &mut game_ai::SmallActionUlt, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData) -> Option<Input>;
    #[link_name = "_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_29SmallActionAroundPositionBush9get_input"]
    fn bush_get_input(s: &mut game_ai::SmallActionAroundPositionBush, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData) -> Option<Input>;
}

/// 실전 세팅 + 미니언 + 에픽 첫 스폰(라인전 페이즈 게이트용)
pub fn real_setting2() -> GameSetting {
    let mut s = real_setting();
    s.epic_jungle.first_spawn_tick = 18000;     // is_line_phase: tick < 18000 - 60*30 = 16200
    s.minion_wave_setting.start_tick = 10;
    s.minion_wave_setting.tick_per_wave = 660;
    s.minion_wave_setting.melee_count = 2;
    s.minion_wave_setting.range_count = 1;
    s.minion_wave_setting.tick_per_spawn = 30;
    s.minion_wave_setting.growth_start_tick = 1800;
    s.minion_wave_setting.growth_tick = 1800;
    s.minion_wave_setting.growth_tick_2v2 = 400;
    s.minion_wave_setting.growth_tick_3v3 = 800;
    s.minion_wave_setting.growth_end_tick = 24000;
    s.minion_wave_setting.growth_end_tick_2v2 = 30000;
    s.minion_wave_setting.growth_end_tick_3v3 = 24000;
    s.minion_wave_setting.exp_range = 150000;
    s.minion_wave_setting.exp_decay2 = 100;
    s.minion_wave_setting.exp_decay3 = 80;
    s.minion_wave_setting.exp_decay4 = 60;
    s.melee_minion.stat.attack = 10; s.melee_minion.stat.hp = 400; s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1; s.melee_minion.growth.hp = 30; s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100; s.melee_minion.attack.range = 3000; s.melee_minion.attack.cooltime = 30;
    s.melee_minion.attack.duration = 24; s.melee_minion.attack.start_timing = 16; s.melee_minion.exp = 40; s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15; s.range_minion.stat.hp = 250; s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1; s.range_minion.growth.hp = 20; s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000; s.range_minion.attack.speed = 3000; s.range_minion.attack.cooltime = 40;
    s.range_minion.attack.duration = 24; s.range_minion.attack.start_timing = 16; s.range_minion.exp = 30; s.range_minion.gold = 20;
    s
}

fn set_attack_damage(e: *const Entity, dmg: usize) {
    // Effect.ty(Arc<dyn EffectType>, Entity+0x490 팻포인터 16B) 를 TowerAttackEffect(dmg, 0) 로 교체(22차 A 수법 · 옛 Arc 누수)
    let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
    let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
    wr(ep(e), 0x490, raw[0]); wr(ep(e), 0x498, raw[1]);
}

fn dist2(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { ax.abs_diff(bx).pow(2) + ay.abs_diff(by).pow(2) }

// ───────── 150 v22_lane_tower_pressure_attack_allowed 명세 독립 재구현 ─────────
fn predict150(version: usize, data: &OperationData, player: &PlayerState, tower: &Entity, ctx: &GameContext, game: &dyn AbstractGame) -> (bool, String) {
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = match data.cache.player_champion[team][pos] { Some(e) => e, None => return (false, "nochamp".into()) };
    let champ_attack = match champ.attack_effect.as_ref() { Some(a) => a, None => return (false, "champ_noatk".into()) };
    if !champ.can_attack() { return (false, "cannot_attack".into()); }
    let mut log = String::new();
    let ss = game_ai::v47_siege_stance(version, data, player, tower);
    log += &format!("siege={:?} ", ss);
    if let Some(soaker) = ss {
        if soaker == champ.id { return (true, log + "soaker"); }
        // v47_tower_covered_for_me 인라인
        let tp = ep(tower);
        if rd::<i64>(tp, 0x68) == 2 && rd::<i64>(tp, 0x88) == 1 {
            let tid: usize = rd(tp, 0x98);
            if tid != champ.id {
                if let Some(t) = game.get_entity_by_id(tid) {
                    if let Some(ta) = tower.attack_effect.as_ref() {
                        if t.hp > ta.expected_damage_target(ctx, tower as &dyn AbstractEntity, t) { return (true, log + "covered"); }
                    }
                }
            }
        }
    }
    let tower_attack = match tower.attack_effect.as_ref() { Some(a) => a, None => return (false, log + "tower_noatk") };
    let tp = ep(tower);
    let targeting = rd::<i64>(tp, 0x68) == 2 && rd::<i64>(tp, 0x88) == 1;
    let tower_target_id: usize = rd(tp, 0x98);
    let mut count = 0usize; let mut will_die = false;
    for m in data.cache.iter_minions(team) {
        if tower_attack.is_in_range(tower, m) {
            count += 1;
            if targeting && m.id == tower_target_id {
                will_die = m.hp <= tower_attack.expected_damage_target(ctx, tower as &dyn AbstractEntity, m);
            }
        }
    }
    log += &format!("count={} will_die={} ", count, will_die);
    if !(count == 0 || (count == 1 && will_die)) { return (true, log + "cover"); }
    let tick = game.tick();
    if !ctx.is_line_phase(tick) { return (false, log + "not_line_phase"); }
    let dmg = champ_attack.expected_damage_target(ctx, champ as &dyn AbstractEntity, tower);
    log += &format!("dmg={} ", dmg);
    if dmg == 0 { return (false, log + "dmg0"); }
    let finish_now = dmg >= tower.hp;
    let finish_soon = dmg.saturating_mul(2) >= tower.hp;
    if !finish_soon { return (false, log + "not_soon"); }
    let enemy = 1 - team;
    let mut threat = false;
    for e in data.cache.player_champion[enemy].iter().flatten() {
        if !data.blackboard[enemy].is_recent_visible(game, player, e) { continue; }
        let r = std::cmp::max(game_ai::max_range(e, champ) + 50000, 120000);
        let d2c = dist2(e.x, e.y, champ.x, champ.y); let d2t = dist2(e.x, e.y, tower.x, tower.y);
        log += &format!("[e{} r={} d2c={} d2t={}]", e.id, r, d2c, d2t);
        if d2c <= r * r || d2t < 32400000001 { threat = true; break; }
    }
    let tower_dmg = tower_attack.expected_damage_target(ctx, tower as &dyn AbstractEntity, champ);
    let floor = champ.stat_cached.hp >> 2;
    let absorb = tower_dmg == 0 || champ.hp > floor + tower_dmg;
    log += &format!("threat={} tower_dmg={} floor={} absorb={} finish_now={} ", threat, tower_dmg, floor, absorb, finish_now);
    if finish_now { (absorb || !threat, log + "finish_now") } else { (count == 1 && absorb && !threat, log + "finish_soon") }
}

fn play_tag(o: &Option<game_ai::SmallActionPlay>) -> (i32, &'static str) {
    let raw: i8 = rd(o as *const _ as *const u8, 0xb1);
    match o {
        None => (raw as i32, "None"),
        Some(game_ai::SmallActionPlay::RunAway(_)) => (raw as i32, "RunAway"),
        Some(game_ai::SmallActionPlay::AroundPosition(_)) => (raw as i32, "AroundPosition"),
        Some(_) => (raw as i32, "Other"),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let which = a.get(1).cloned().unwrap_or_default();
    if a.len() > 99 { let _ = game_ai::v22_lane_tower_pressure_attack_allowed as *const (); }
    let setting = real_setting2();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let tut = match arg(&a, 2, 0) { 1 => TutorialType::First, 2 => TutorialType::TopSolo, 3 => TutorialType::Bottom, 4 => TutorialType::MidSolo,
                                     5 => TutorialType::MidBottom, 6 => TutorialType::JungleOnly, 7 => TutorialType::Line, 8 => TutorialType::Total, _ => TutorialType::None };
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);

    if which == "150" {
        // argv: 150 tutorial tick cover(0/1/2/3) champ_dmg tower_dmg tower_hp champ_hp champ_maxhp threat(0/1/2/3/4) seen(0/1) version
        let tick = arg(&a, 3, 1000) as usize; let cover = arg(&a, 4, 0);
        let cdmg = arg(&a, 5, -1); let tdmg = arg(&a, 6, -1); let thp = arg(&a, 7, -1); let chp = arg(&a, 8, -1); let cmax = arg(&a, 9, -1);
        let threat = arg(&a, 10, 0); let seen = arg(&a, 11, 1); let version = arg(&a, 12, 55) as usize;
        if cover > 0 {
            for _ in 0..600usize { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd, &mut fd); }
        }
        game.set_tick(tick);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let tower = cache.top_tower[1].or(cache.mid_tower[1]).or(cache.bottom_tower[1]).expect("enemy tower");
        let ec = cache.player_champion[1][0].expect("enemy champ");
        let cp = ep(champ); let tp = ep(tower); let ecp = ep(ec);
        // 챔피언을 타워에서 400k 떨어진 곳에(위협 반경 분리)
        let (tx, ty) = (tower.x, tower.y);
        let cx = if tx >= 400000 { tx - 400000 } else { tx + 400000 }; let cy = ty;
        println!("pos	tower=({},{})	champ=({},{})", tx, ty, cx, cy);
        wr(cp, 0x660, cx); wr(cp, 0x668, cy);
        if cdmg >= 0 { set_attack_damage(champ, cdmg as usize); }
        if tdmg >= 0 { set_attack_damage(tower, tdmg as usize); }
        if thp >= 0 { wr(tp, 0x670, thp as usize); }
        if chp >= 0 { wr(cp, 0x670, chp as usize); }
        if cmax >= 0 { wr(cp, 0x628, cmax as usize); }
        // 미니언 커버: 아군(팀0) 미니언 전부 멀리 → cover 개수만큼 타워 위치로
        if cover > 0 {
            let mut placed = 0usize; let mut first_id = 0usize;
            let ids: Vec<usize> = cache.iter_minions(0).map(|m| m.id).collect();
            for m in cache.iter_minions(0) {
                let mp = ep(m);
                let want = match cover { 1 | 3 => 1, 2 => 2, _ => 0 };
                if placed < want { wr(mp, 0x660, tx); wr(mp, 0x668, ty); if placed == 0 { first_id = m.id; } placed += 1; }
                else { wr(mp, 0x660, 1u64); wr(mp, 0x668, 1u64); }
            }
            println!("minions\tally={}\tplaced={}\tfirst_id={}", ids.len(), placed, first_id);
            if cover == 1 || cover == 2 { wr(tp, 0x88, 1i64); wr(tp, 0x98, first_id); }   // targeting = first minion
            if cover == 3 { wr(tp, 0x88, 0i64); }                                            // 타겟 없음
        }
        // 적 챔피언 위치·시야
        let r = std::cmp::max(game_ai::max_range(ec, champ) + 50000, 120000);
        match threat {
            1 => { wr(ecp, 0x660, cx); wr(ecp, 0x668, cy + r); }             // dist == r → 위협(<=) (타워 반대쪽 = y축)
            2 => { wr(ecp, 0x660, cx); wr(ecp, 0x668, cy + r + 1); }         // dist == r+1 → 비위협(챔피언 기준)
            3 => { wr(ecp, 0x660, tx - 180000); wr(ecp, 0x668, ty); }        // 타워 기준 dist == 180000 → 위협(<180000²+1)
            4 => { wr(ecp, 0x660, tx - 180001); wr(ecp, 0x668, ty); }        // 180001 → 비위협
            _ => { wr(ecp, 0x660, 1u64); wr(ecp, 0x668, 1u64); }
        }
        if seen == 1 { bb[1].last_visible[0] = tick; }
        let data = OperationData::new(&cache, &ctx, &bb);
        let (mine, info) = predict150(version, &data, player, tower, &ctx, &game as &dyn AbstractGame);
        let g = game_ai::v22_lane_tower_pressure_attack_allowed(version, &data, player, tower);
        println!("RESULT150\ttick={}\ttut={:?}\tcover={}\tcdmg={}\ttdmg={}\tthp={}\tchp={}\tcmax={}\tthreat={}\tseen={}\tr={}\tgame={}\tmine={}\tok={}\t{}",
                 tick, tut, cover, cdmg, tdmg, thp, chp, cmax, threat, seen, r, g, mine, g == mine, info);
        return;
    }

    if which == "156" {
        // argv: 156 tutorial kind(0 SafeWait/1 Hard) threat(0 none/1 dist=160000/2 dist=160001/3 far) seen(0/1) vis(0/1 enemy.visible_state[0]=Visible) tick
        let kind = arg(&a, 3, 0); let threat = arg(&a, 4, 0); let seen = arg(&a, 5, 1); let vis = arg(&a, 6, 1); let tick = arg(&a, 7, 1000) as usize;
        game.set_tick(tick);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let ec = cache.player_champion[1][0].expect("enemy champ");
        let cp = ep(champ); let ecp = ep(ec);
        // 내 챔피언을 맵 중앙으로(우물 무시 술어 회피), 적은 거리별
        let (cx, cy) = (480000u64, 480000u64);
        wr(cp, 0x660, cx); wr(cp, 0x668, cy);
        match threat { 1 => { wr(ecp, 0x660, cx + 160000); wr(ecp, 0x668, cy); }
                       2 => { wr(ecp, 0x660, cx + 160001); wr(ecp, 0x668, cy); }
                       3 => { wr(ecp, 0x660, cx + 300000); wr(ecp, 0x668, cy); }
                       _ => { wr(ecp, 0x660, 1u64); wr(ecp, 0x668, 1u64); } }
        if vis == 1 { unsafe { std::ptr::write_volatile(&mut (*(ecp as *mut Entity)).visible_state[0], VisibleState::Visible); } }
        if seen == 1 { bb[1].last_visible[0] = tick; }
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
        tp.objective_discipline = Some(game_ai::plan_legacy::team_plan::ObjectiveDisciplineState {
            wait_pos: (123456, 654321), until_tick: tick + 1000, target: JungleType::Rhino,
            kind: if kind == 1 { game_ai::plan_legacy::team_plan::ObjectiveDisciplineKind::HardDisengage } else { game_ai::plan_legacy::team_plan::ObjectiveDisciplineKind::SafeWait } });
        let st = tp.v27_active_objective_discipline(55, &data, JungleType::Rhino);
        let ign = game_ai::plan_legacy::old::is_ignored_well_enemy(55, player, ec);
        let rv = data.blackboard[1].is_recent_visible(&game as &dyn AbstractGame, player, ec);
        let visf = ec.is_visible_from(champ);
        let d2 = dist2(ec.x, ec.y, champ.x, champ.y);
        let has_threat = visf && !ign && rv && d2 < 25600000001;
        let mine = if kind == 1 { if has_threat { "RunAway" } else { "AroundPosition" } } else { if has_threat { "AroundPosition" } else { "None" } };
        let rnd_before: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let r = tp.v27_objective_discipline_action(55, &mut rnd, player, &data, JungleType::Rhino);
        let rnd_after: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let (tag, name) = play_tag(&r);
        let mut extra = String::new();
        if let Some(game_ai::SmallActionPlay::AroundPosition(ap)) = &r { let p = ap as *const _ as *const u8; extra = format!("ap.target=({},{})", rd::<u64>(p, 0x20), rd::<u64>(p, 0x28)); }
        println!("RESULT156\tkind={}\tthreat={}\tseen={}\tvis={}\tstate={:?}\tign={}\trv={}\tvisf={}\td2={}\thas_threat={}\tgame={}\ttag={}\tmine={}\tok={}\trnd_changed={}\t{}",
                 kind, threat, seen, vis, st.is_some(), ign, rv, visf, d2, has_threat, name, tag, mine, name == mine, rnd_before != rnd_after, extra);
        return;
    }

    if which == "151" || which == "153" || which == "154" {
        // argv: <fn> tutorial case level  — case 0 없는 대상 / 1 적 챔피언(우물 안) / 2 적 챔피언(우물 밖·멀리) / 3 아군 챔피언 / 4 적 챔피언 근접
        let case = arg(&a, 3, 0); let level = arg(&a, 4, -1);
        game.set_tick(1000);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let ec = cache.player_champion[1][0].expect("enemy champ");
        let ally = cache.player_champion[0][2].expect("ally mid");
        let cp = ep(champ); let ecp = ep(ec);
        if level >= 0 { wr(cp, 0x5c8, level as usize); }
        wr(cp, 0x660, 480000u64); wr(cp, 0x668, 480000u64);
        let target_id = match case {
            0 => 987654usize,
            1 => { wr(ecp, 0x660, 900000u64); wr(ecp, 0x668, 30000u64); ec.id }       // 적 우물(팀0 기준 x∈[800k,960k]·y≤64k)
            2 => { wr(ecp, 0x660, 200000u64); wr(ecp, 0x668, 700000u64); ec.id }
            3 => ally.id,
            5 => { wr(ecp, 0x660, 510000u64); wr(ecp, 0x668, 480000u64); ec.id }
            6 => { wr(ecp, 0x660, 540000u64); wr(ecp, 0x668, 480000u64); ec.id }
            7 => { wr(ecp, 0x660, 580000u64); wr(ecp, 0x668, 480000u64); ec.id }
            _ => { wr(ecp, 0x660, 490000u64); wr(ecp, 0x668, 480000u64); ec.id }
        };
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps: PositioningScoreData = Default::default();
        let well = game_ai::is_enemy_well_danger(55, player, rd::<u64>(ecp, 0x660), rd::<u64>(ecp, 0x668));
        println!("pre\tchamp.level={}\tskill={}\tskill2={}\tult={}\tenemy_well_danger(ec)={}\tec.team={:?}\tally.team={:?}",
                 champ.level, champ.skill_effect.is_some(), champ.skill2_effect().is_some(), champ.ult_effect().is_some(), well, ec.team, ally.team);
        let rnd_before: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let (r, is_act_before, is_act_after, self_bytes) = unsafe {
            if which == "151" {
                let mut s = game_ai::SmallActionSkill::new(&data, target_id);
                let sp = &s as *const _ as *const u8; let b: u8 = rd(sp, 0x10);
                let r = skill_get_input(&mut s, 55, &mut rnd, player, &data, &ps);
                let a2: u8 = rd(sp, 0x10); (r, b, a2, format!("{:?}", std::slice::from_raw_parts(sp, 24)))
            } else if which == "153" {
                let mut s = game_ai::SmallActionSkill2::new(&data, target_id);
                let sp = &s as *const _ as *const u8; let b: u8 = rd(sp, 0x10);
                let r = skill2_get_input(&mut s, 55, &mut rnd, player, &data, &ps);
                let a2: u8 = rd(sp, 0x10); (r, b, a2, format!("{:?}", std::slice::from_raw_parts(sp, 24)))
            } else {
                let mut s = game_ai::SmallActionUlt::new(&data, target_id);
                let sp = &s as *const _ as *const u8; let b: u8 = rd(sp, 0x10);
                let r = ult_get_input(&mut s, 55, &mut rnd, player, &data, &ps);
                let a2: u8 = rd(sp, 0x10); (r, b, a2, format!("{:?}", std::slice::from_raw_parts(sp, 24)))
            }
        };
        let rnd_after: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let tag: i64 = rd(&r as *const _ as *const u8, 0);
        println!("RESULT{}\tcase={}\tlevel={}\ttarget={}\tgame={:?}\ttag={}\tis_act={}->{}\trnd_changed={}\tself={}",
                 which, case, level, target_id, r, tag, is_act_before, is_act_after, rnd_before != rnd_after, self_bytes);
        return;
    }

    if which == "157" {
        // argv: 157 tutorial case  — 0 목표=내 위치(dist 0) / 1 dist=16000 / 2 dist=16001 / 3 멀리(200k) / 4 멀리 2회 호출
        let case = arg(&a, 3, 0);
        game.set_tick(1000);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let cp = ep(champ);
        wr(cp, 0x660, 480000u64); wr(cp, 0x668, 480000u64);
        let (tx, ty) = match case { 0 => (480000u64, 480000u64), 1 => (496000, 480000), 2 => (496001, 480000), _ => (680000, 480000) };
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps: PositioningScoreData = Default::default();
        let mut s = game_ai::SmallActionAroundPositionBush::new(&data, tx, ty);
        let sp = &s as *const _ as *const u8;
        let tag_before: u8 = rd(sp, 0x5d);
        let rnd_before: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let r = unsafe { bush_get_input(&mut s, 55, &mut rnd, player, &data, &ps) };
        let rnd_after: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let tag_after: u8 = rd(sp, 0x5d);
        let pf_bytes: Vec<u64> = (0..9).map(|i| rd::<u64>(sp, 0x18 + i * 8)).collect();
        let d2 = dist2(champ.x, champ.y, tx, ty);
        println!("RESULT157\tcase={}\ttarget=({},{})\td2={}\tgame={:?}\tpf_tag={}->{}\trnd_changed={}\tpf={:x?}", case, tx, ty, d2, r, tag_before, tag_after, rnd_before != rnd_after, pf_bytes);
        if case == 4 {
            let r2 = unsafe { bush_get_input(&mut s, 55, &mut rnd, player, &data, &ps) };
            let rnd_after2: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
            let tag2: u8 = rd(sp, 0x5d);
            let pf2: Vec<u64> = (0..9).map(|i| rd::<u64>(sp, 0x18 + i * 8)).collect();
            println!("RESULT157b\tcase={}\tgame2={:?}\tpf_tag={}\trnd_changed={}\tpf={:x?}", case, r2, tag2, rnd_after != rnd_after2, pf2);
        }
        return;
    }
    println!("unknown fn");
}
