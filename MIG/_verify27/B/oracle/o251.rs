#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치B 오라클 — #251 fight_check::battle_action (pub · m15.ll:23700) — `game_ai::battle_action` 직접 호출
//! 명세 logic 을 독립 재구현(모델)해 반환 Vec<SmallActionPlay> 의 (태그@+0xb1, target@+8) 열과 대조한다(수법 ⓓ).
//! 셀프캐스트(L721/L780 should_add_self_* · internal fastcc) 만 모델이 못 부르므로 self 원소는 "관측"으로만 찍는다.
//! 사용: o251.exe <case>  (케이스당 프로세스 1개 — is_dash_worth 경로의 CHAMP_POWERS_MEMO 대비)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn p32(base: *mut u8, off: usize, v: i32) { std::ptr::write_volatile(base.add(off) as *mut i32, v); }
unsafe fn set_xy(e: *mut Entity, x: u64, y: u64) { p64(e as *mut u8, 0x660, x as i64); p64(e as *mut u8, 0x668, y as i64); }
unsafe fn set_visible(e: *mut Entity, team: usize, tag: i64) { p64(e as *mut u8, 0x38 + team * 24, tag); }

// ── 모델(명세 logic 독립 재구현) — self 캐스트 항은 제외(internal fastcc 콜리) ─────────────
fn radius(e: &Entity) -> u64 { e.radius() as u64 }
fn model(player: &PlayerState, data: &OperationData, cache: &AbstractGameWithCache, bb: &[Blackboard; 2],
         game: &dyn AbstractGame, ctx: &GameContext) -> Vec<(u8, usize)> {
    let team = player.info.team; let pos = player.info.position as usize;
    let champ: &Entity = cache.player_champion[team][pos].unwrap();
    let near_allies: Vec<&Entity> = cache.iter_champions(team)
        .filter(|a| a.id != champ.id && a.distance_sq(champ) < 22500000000).collect();
    let enemy_row = &cache.player_champion[1 - team];
    let nea: Vec<(SmallAction, &Entity)> = enemy_row.iter().enumerate()
        .map(|(i, c)| (bb[1 - team].small_actions[i].clone(), *c))
        .filter(|(a, c)| a.is_some() && c.is_some() && data.can_target(game, player, c.unwrap()))
        .map(|(a, c)| (a.unwrap(), c.unwrap())).collect();
    let mut out: Vec<(u8, usize)> = Vec::new();
    let attack = champ.attack_effect.as_ref();
    let skill = champ.skill_effect.as_ref();
    let skill2 = champ.skill2_effect().as_ref();
    let ms = champ.stat_cached.move_speed as u64;
    let runaway = |a: &SmallAction| matches!(a, SmallAction::RunAway);
    let ms_for = |a: &SmallAction, e: &Entity| -> u64 {
        if runaway(a) && !e.block_move() { ms.saturating_sub(e.stat_cached.move_speed as u64) } else { ms } };
    if let Some(at) = attack { if champ.can_attack() {
        for (a, e) in nea.iter() {
            if !e.is_visible_from(champ) { continue; }
            let range = at.range(champ) + at.range_adjust(champ, e) + radius(champ) + radius(e);
            let d2 = e.distance_sq(champ);
            let m = range + ms_for(a, e) * 30;
            if d2 > m * m { continue; }
            out.push((15, e.id));
        } } }
    if let Some(sk) = skill { if champ.can_skill() {
        for (a, e) in nea.iter() {
            if !e.is_visible_from(champ) { continue; }
            if !sk.target.check(champ, e) { continue; }
            let m = ms_for(a, e) * 30 + sk.range(champ) + sk.range_adjust(champ, e) + radius(champ) + radius(e);
            let d2 = e.distance_sq(champ);
            if d2 > m * m { continue; }
            if sk.ty.can_move() && e.ty.is_champion() {
                let dmg = sk.expected_damage_target(ctx, champ as &dyn AbstractEntity, e);
                if !game_ai::is_dash_worth(data, player, champ, e, dmg) { continue; }
            }
            out.push((16, e.id));
        }
        let walk: u64 = if sk.ty.expected_buff_deep(ctx, champ as &dyn AbstractEntity).is_some() { 90 } else { 30 };
        for a in near_allies.iter() {
            if !sk.target.check(champ, a) { continue; }
            let m = sk.range(champ) + walk * ms + sk.range_adjust(champ, a) + radius(champ) + radius(a);
            if a.distance_sq(champ) > m * m { continue; }
            out.push((16, a.id));
        }
        // L721 self: should_add_self_etc_buff_action — 모델 불가(internal fastcc) → 관측만
    } }
    if let Some(sk) = skill2 { if champ.can_skill2() {
        for (a, e) in nea.iter() {
            if !e.is_visible_from(champ) { continue; }
            if !sk.target.check(champ, e) { continue; }
            let mut m = sk.range(champ) + sk.range_adjust(champ, e) + radius(champ) + radius(e);
            let d2 = e.distance_sq(champ);
            m += ms_for(a, e) * 30;
            if d2 > m * m { continue; }
            if sk.ty.can_move() && e.ty.is_champion() {
                let dmg = sk.expected_damage_target(ctx, champ as &dyn AbstractEntity, e);
                if !game_ai::is_dash_worth(data, player, champ, e, dmg) { continue; }
            }
            out.push((17, e.id));
        }
        let walk: u64 = if sk.ty.expected_buff_deep(ctx, champ as &dyn AbstractEntity).is_none() { 30 } else { 90 };
        for a in near_allies.iter() {
            if !sk.target.check(champ, a) { continue; }
            let mut m = sk.range(champ) + walk * ms + sk.range_adjust(champ, a) + radius(champ);
            let d2 = a.distance_sq(champ);
            m += radius(a);
            if d2 > m * m { continue; }
            out.push((17, a.id));
        }
        // L780 self: should_add_self_skill2_action — 관측만
    } }
    out
}

fn attack_max(champ: &Entity, e: &Entity, ms: u64) -> u64 {
    let at = champ.attack_effect.as_ref().unwrap();
    at.range(champ) + at.range_adjust(champ, e) + radius(champ) + radius(e) + ms * 30
}
fn skill_ally_max(champ: &Entity, a: &Entity, walk: u64) -> u64 {
    let sk = champ.skill_effect.as_ref().unwrap();
    sk.range(champ) + walk * (champ.stat_cached.move_speed as u64) + sk.range_adjust(champ, a) + radius(champ) + radius(a)
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms_: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms_, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms_, &map, &ctx);
    let tick = 1000usize;
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];

    let player: &PlayerState = game.get_player_by_position(0, Position::Top).expect("player");
    let champ: &Entity = cache.player_champion[0][player.info.position as usize].expect("champ");
    let ally0: &Entity = cache.player_champion[0][2].expect("ally(mid)");
    let ep_pos = game.get_player_by_position(1, Position::Top).expect("ep").info.position as usize;
    let enemy: &Entity = cache.player_champion[1][ep_pos].expect("enemy");
    // ★default 챔프 ms==1(TEMPLATE ⑥) → *30 항·saturating_sub 판별력 확보: 첫 읽기 전에 raw store
    unsafe { p64(champ as *const Entity as *mut u8, 0x640, 500); p64(enemy as *const Entity as *mut u8, 0x640, 200); }
    let ms = unsafe { std::ptr::read_volatile(&champ.stat_cached.move_speed) } as u64;
    println!("setup\ttick={}\tchamp.id={}\tlevel={}\tms={}\tradius={}\tatk={:?}\tskill={:?}\tskill2={:?}\tcan_attack={}\tcan_skill={}\tcan_skill2={}",
             game.tick(), champ.id, champ.level, ms, champ.radius(),
             champ.attack_effect.as_ref().map(|e| (e.range, e.growth_range, e.target as i32)),
             champ.skill_effect.as_ref().map(|e| (e.range, e.growth_range, e.target as i32, e.ty.can_move())),
             champ.skill2_effect().as_ref().map(|e| (e.range, e.growth_range, e.target as i32)),
             champ.can_attack(), champ.can_skill(), champ.can_skill2());
    println!("setup2\tenemy.id={}\tenemy.ms={}\tally0.id={}\tvisible_before={}\tblock_move={}", enemy.id,
             enemy.stat_cached.move_speed, ally0.id, enemy.is_visible_from(champ), enemy.block_move());
    let cp = champ as *const Entity as *mut Entity;
    let ep = enemy as *const Entity as *mut Entity;
    let ap = ally0 as *const Entity as *mut Entity;
    let mut note = String::new();
    unsafe {
        // 공통: 적 전원 소행동 Dodge(비-RunAway) · 아군팀(0) 시야에 Visible
        for i in 0..5 { bb[1].small_actions[i] = Some(SmallAction::Dodge); }
        for i in 0..5 { if let Some(e) = cache.player_champion[1][i] { set_visible(e as *const Entity as *mut Entity, 0, 0); } }
        let amax = attack_max(champ, enemy, ms);
        match case {
            0 => { note = format!("baseline amax={}", amax); }
            1 => { set_xy(ep, champ.x + amax, champ.y); note = format!("enemy at amax={} (경계 포함 기대)", amax); }
            2 => { set_xy(ep, champ.x + amax + 1, champ.y); note = format!("enemy at amax+1={} (제외 기대)", amax + 1); }
            3 => { bb[1].small_actions[ep_pos] = Some(SmallAction::RunAway);
                   let m2 = attack_max(champ, enemy, ms.saturating_sub(enemy.stat_cached.move_speed as u64));
                   set_xy(ep, champ.x + m2, champ.y); note = format!("RunAway 상대속도 경계 m2={} (포함 기대)", m2); }
            4 => { bb[1].small_actions[ep_pos] = Some(SmallAction::RunAway);
                   let m2 = attack_max(champ, enemy, ms.saturating_sub(enemy.stat_cached.move_speed as u64));
                   set_xy(ep, champ.x + m2 + 1, champ.y); note = format!("RunAway 경계+1={} (제외 기대 · 절대속도면 포함)", m2 + 1); }
            5 => { set_xy(ep, champ.x + amax, champ.y); set_visible(ep, 0, 2); note = "enemy at amax, Unknown 시야 (제외 기대)".into(); }
            6 => { set_xy(ep, champ.x + amax, champ.y); bb[1].small_actions[ep_pos] = None; note = "enemy at amax, 소행동 None (제외 기대)".into(); }
            7 => { set_xy(ep, champ.x + amax, champ.y); p32(cp as *mut u8, 0x4c0, -1); note = "attack None(태그 -1) → Attack 0 기대".into(); }
            8 => { // 스킬 아군 대상: target=Ally(0)
                   p32(cp as *mut u8, 0x4f0, 0);
                   let walk = if champ.skill_effect.as_ref().unwrap().ty.expected_buff_deep(&ctx, champ as &dyn AbstractEntity).is_some() { 90 } else { 30 };
                   let m = skill_ally_max(champ, ally0, walk); set_xy(ap, champ.x + m, champ.y);
                   note = format!("skill.target=Ally · ally0 at m={} walk={} (Skill 포함 기대)", m, walk); }
            9 => { p32(cp as *mut u8, 0x4f0, 0);
                   let walk = if champ.skill_effect.as_ref().unwrap().ty.expected_buff_deep(&ctx, champ as &dyn AbstractEntity).is_some() { 90 } else { 30 };
                   let m = skill_ally_max(champ, ally0, walk); set_xy(ap, champ.x + m + 1, champ.y);
                   note = format!("skill.target=Ally · ally0 at m+1={} (제외 기대)", m + 1); }
            10 => { p32(cp as *mut u8, 0x4f0, 0); set_xy(ap, champ.x + 150000, champ.y); note = "ally0 at 150000 (near_allies 경계 < 이므로 제외 기대)".into(); }
            11 => { p32(cp as *mut u8, 0x4f0, 0); set_xy(ap, champ.x + 149999, champ.y);
                    let walk = if champ.skill_effect.as_ref().unwrap().ty.expected_buff_deep(&ctx, champ as &dyn AbstractEntity).is_some() { 90 } else { 30 };
                    note = format!("ally0 at 149999 (near_allies 포함 · 사거리 m={} 비교는 모델대로)", skill_ally_max(champ, ally0, walk)); }
            12 => { p64(cp as *mut u8, 0x5c8, 3); set_xy(ep, champ.x + amax, champ.y); note = "level=3 → skill2 게이트 열림 · enemy at amax".into(); }
            13 => { p64(cp as *mut u8, 0x5c8, 3); p32(cp as *mut u8, 0x500 + 0x28, 0); set_xy(ap, champ.x + 30000, champ.y); note = "level=3 · skill2.target=Ally · ally0 at 30000".into(); }
            14 => { set_xy(ep, champ.x + amax / 2, champ.y); p32(cp as *mut u8, 0x4f0, 5); note = "enemy at amax/2 · skill.target=Enemy(5) → Attack+Skill 기대".into(); }
            15 => { p64(cp as *mut u8, 0x640, 6000); p32(cp as *mut u8, 0x4f0, 0); set_xy(ap, champ.x + 150000, champ.y);
                    note = "ms=6000(사거리 m=200000) · skill.target=Ally · ally0 at 150000 → near_allies  게이트로 제외 기대".into(); }
            16 => { p64(cp as *mut u8, 0x640, 6000); p32(cp as *mut u8, 0x4f0, 0); set_xy(ap, champ.x + 149999, champ.y);
                    note = "ms=6000 · ally0 at 149999 → near_allies 통과·Skill 포함 기대".into(); }
            17 => { p64(cp as *mut u8, 0x640, 6000); p32(cp as *mut u8, 0x4f0, 0); set_xy(ap, champ.x + 106066, champ.y + 106066);
                    note = "ms=6000 · ally0 대각 (106066,106066) d²=22499993... <150000² 포함 기대".into(); }
            _ => {}
        }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let exp = model(player, &data, &cache, &bb, &game as &dyn AbstractGame, &ctx);
    let mut rng = rand::rngs::StdRng::seed_from_u64(99);
    let got = game_ai::battle_action(30, &mut rng, player, &data, 0);
    let mut gv: Vec<(u8, usize)> = Vec::new();
    let mut self_obs: Vec<(u8, usize)> = Vec::new();
    for p in got.iter() {
        let b = p as *const game_ai::SmallActionPlay as *const u8;
        let tag = unsafe { *b.add(0xb1) };
        let target = unsafe { *(b.add(8) as *const u64) } as usize;
        if target == champ.id { self_obs.push((tag, target)); } else { gv.push((tag, target)); }
    }
    let vb = &got as *const _ as *const u64;
    let (vptr, vbump, vcap, vlen) = unsafe { (*vb, *vb.add(1), *vb.add(2), *vb.add(3)) };
    println!("case={}\t{}\n  got={:?}\tself_obs={:?}\tmodel={:?}\t{}\tvec=[ptr={:#x} bump={:#x}({}) cap={} len={}]",
             case, note, gv, self_obs, exp, if gv == exp { "MATCH" } else { "DIFF" },
             vptr, vbump, if vbump == (&pool as *const bumpalo::Bump as u64) { "==ctx.pool" } else { "!=pool" }, vcap, vlen);
}
