#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치C 오라클 — #252 v16_knight_ult_zone_bonus (hidden define, m05.ll:55831) — `#[link_name]` 직접 호출
//! 명세 logic 을 독립 재구현(모델)해 i64 반환을 대조한다(수법 ⓓ). ScoreParameter/ChampionScoreParameter 는 제로버퍼(22차 D 실증)에
//! 필요한 칸만 raw store. champion_hp_value 가 TLS 메모라 **케이스당 프로세스 1개**.
//! 사용: o252.exe <case>
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common25v16_knight_ult_zone_bonus"]
    fn v16(player: *const PlayerState, data: *const OperationData, param: *const game_ai::ScoreParameter,
           action: *const Box<dyn Action>, champ: *const Entity, target: *const Entity) -> i64;
}

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

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn set_xy(e: *mut Entity, x: u64, y: u64) { p64(e as *mut u8, 0x660, x as i64); p64(e as *mut u8, 0x668, y as i64); }

// ── 모델(명세 logic 독립 재구현) ─────────────────────────────────────────
struct Ally { id: usize, applyed: i64, risk: i64, ptr: *const game_ai::ChampionScoreParameter<'static> }
fn model(player: &PlayerState, data: &OperationData, cache: &AbstractGameWithCache, bb: &[Blackboard; 2], game: &dyn AbstractGame,
         param: &game_ai::ScoreParameter, ku: Option<&KnightUltAction>, champ: &Entity, target: &Entity, allies: &[Ally],
         self_applyed: i64, self_risk: i64) -> i64 {
    let ku = match ku { Some(k) => k, None => return 0 };
    if target.team != champ.team || !target.ty.is_champion() { return 0; }
    let zr = ku.range as u64; let zone_sq = zr * zr; let inf = zr + 35000; let inf_sq = inf * inf;
    let team = player.info.team; let et = 1 - team;
    let (mut a_in, mut thr, mut pres) = (0i64, 0i64, 0i64);
    let vis = |e: &Entity| bb[et].is_recent_visible(game, player, e);
    for ally in cache.iter_champions(team) {
        if ally.distance_sq(target) > zone_sq { continue; }
        a_in += 1;
        let (incoming, hp_value) = if ally.id == champ.id {
            let p = &param.player;
            (self_applyed + self_risk + p.possible_risk(data, ku.duration + 30), game_ai::champion_hp_value(data, param, p).min(100))
        } else if let Some(a) = allies.iter().find(|a| a.id == ally.id) {
            let p: &game_ai::ChampionScoreParameter<'static> = unsafe { &*a.ptr };
            (a.applyed + a.risk + p.possible_risk(data, ku.duration + 30), game_ai::champion_hp_value(data, param, p).min(100))
        } else { (0, 50) };
        let close = cache.iter_champions(et).any(|e| vis(e) && e.distance_sq(ally) <= 100000u64 * 100000u64);
        if incoming > 0 || close { thr += 1; }
        if incoming > 0 { pres += ((incoming * hp_value) / (ally.hp.max(1) as i64)).min(45); }
    }
    let near = cache.iter_champions(et).filter(|e| vis(e)).filter(|e| e.distance_sq(target) <= inf_sq).count() as i64;
    let in_zone = cache.iter_champions(et).filter(|e| vis(e) && e.distance_sq(target) <= zone_sq).count() as i64;
    if in_zone == 0 && thr == 0 { return 0; }
    let cover = cache.iter_champions(team).filter(|e| e.distance_sq(champ) <= zone_sq).count() as i64;
    let mut s = (ku.damage_reduce as i64 / 3).min(18);
    s += 5 + a_in.min(4) * 6;
    s += thr.min(4) * 14;
    s += near.min(4) * 3;
    s += in_zone.min(3) * 6;
    s += pres.min(50) / 3;
    if cover > a_in { s -= (cover - a_in).min(3) * 12; }
    if target.id == champ.id && a_in < 2 && near < 2 { s -= 15; }
    s.max(0).min(75)
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick = 1000usize;
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let al: Vec<&Entity> = (0..5).map(|i| cache.player_champion[0][i].expect("ally")).collect();
    let en: Vec<&Entity> = (0..5).map(|i| cache.player_champion[1][i].expect("enemy")).collect();
    let epos: Vec<usize> = (0..5).map(|i| cache.player_by_champion_id(en[i].id).expect("ep").info.position as usize).collect();
    let champ: &Entity = al[2];
    let mut target: &Entity = al[0];
    let tower: &Entity = cache.iter_towers_without_nexus(0).next().expect("tower");

    // 기본 배치: target(al0) T=(300000,300000) · champ(al2) T+(40000,0) · 나머지 아군 (100000,100000) · 적 전원 (800000,800000)
    let (tx, ty) = (300000u64, 300000u64);
    unsafe {
        for (i, e) in al.iter().enumerate() { let p = *e as *const Entity as *mut Entity;
            match i { 0 => set_xy(p, tx, ty), 2 => set_xy(p, tx + 40000, ty), _ => set_xy(p, 100000, 100000) }
            p64(p as *mut u8, 0x670, 1000); }
        for e in en.iter() { set_xy(*e as *const Entity as *mut Entity, 800000, 800000); }
    }
    // KnightUltAction
    let (mut range, mut dr, mut dur) = (60000usize, 30usize, 180usize);
    // 제로버퍼 ScoreParameter + near_allies 배열
    let mut param = Box::new(Buf([0u8; 5384]));
    let mut na: Vec<Box<Buf<216>>> = (0..5).map(|_| Box::new(Buf([0u8; 216]))).collect();
    let mut allies: Vec<Ally> = Vec::new();
    let (mut self_applyed, mut self_risk) = (0i64, 0i64);
    let mut use_empty = false;
    let mut vis_enemy: Vec<(usize, u64, u64)> = Vec::new();   // (enemy idx, x, y) → 가시 + 이동
    let mut add_ally = |idx: usize, applyed: i64, risk: i64| {};
    let mut pending: Vec<(usize, i64, i64)> = Vec::new();      // near_allies 엔트리 (아군 슬롯, applyed, risk)
    unsafe {
        match case {
            0 => {}
            1 => { pending.push((0, 500, 0)); }
            2 => { pending.push((0, 500, 0)); vis_enemy.push((0, tx + 30000, ty)); }
            3 => { pending.push((0, 500, 0)); set_xy(en[0] as *const Entity as *mut Entity, tx + 30000, ty); }  // 비가시(last_visible=0)
            4 => { target = en[0]; pending.push((0, 500, 0)); }
            5 => { target = tower; pending.push((0, 500, 0)); }
            6 => { use_empty = true; pending.push((0, 500, 0)); }
            7 => { target = champ; self_applyed = 300; set_xy(al[0] as *const Entity as *mut Entity, 100000, 100000); }
            8 => { pending.push((0, 500, 0)); let (cx, cy) = (600000u64, 600000u64);
                   set_xy(al[2] as *const Entity as *mut Entity, cx, cy);
                   for i in [1usize, 3, 4] { set_xy(al[i] as *const Entity as *mut Entity, cx + 10000 * i as u64, cy); } }
            9 => { for i in 0..5 { set_xy(al[i] as *const Entity as *mut Entity, tx + 5000 * i as u64, ty); }
                   for i in 0..4 { vis_enemy.push((i, tx + 20000 + 5000 * i as u64, ty + 20000)); } }
            10 => { dr = 100; pending.push((0, 500, 0)); }
            11 => { pending.push((0, 1_000_000, 0)); }
            12 => { vis_enemy.push((0, tx + 90000, ty)); }
            13 => { vis_enemy.push((0, tx + 100000, ty)); set_xy(al[2] as *const Entity as *mut Entity, tx - 40000, ty); }
            14 => { vis_enemy.push((0, tx + 100001, ty)); set_xy(al[2] as *const Entity as *mut Entity, tx - 40000, ty); }
            15 => { vis_enemy.push((0, tx + 95000, ty)); set_xy(al[2] as *const Entity as *mut Entity, tx - 40000, ty); }
            16 => { vis_enemy.push((0, tx + 95001, ty)); set_xy(al[2] as *const Entity as *mut Entity, tx - 40000, ty); }
            17 => { pending.push((0, 500, 0)); set_xy(al[1] as *const Entity as *mut Entity, tx + 60000, ty); }
            18 => { pending.push((0, 500, 0)); set_xy(al[1] as *const Entity as *mut Entity, tx + 60001, ty); }
            19 => { pending.push((0, 500, 200)); pending.push((2, 0, 0)); self_applyed = 0; self_risk = 700; range = 30000; dur = 60; }
            20 => { pending.push((0, 0, 0)); vis_enemy.push((0, tx + 30000, ty)); }   // near_allies 에 있지만 incoming 0 → hp_value 는 champion_hp_value
            21 => { target = champ; self_applyed = 300; pending.push((0, 500, 0)); }   // 자기대상이지만 allies_in_zone=2 → -15 없음
            22 => { target = champ; self_applyed = 300; set_xy(al[0] as *const Entity as *mut Entity, 100000, 100000);
                    vis_enemy.push((0, tx + 90000, ty)); vis_enemy.push((1, tx + 90000, ty + 1000)); }   // enemies_near=2 → -15 없음
            23 => { pending.push((0, 200, 0)); }   // hp_value 캡 100 판별: 200*100/1000=20 → p/3=6 (캡 없으면 200*4000/1000→45→15)
            24 => { pending.push((0, 5, 0)); p64(al[0] as *const Entity as *mut u8, 0x670, 0); }   // ally.hp=0 → max(1) 분모 → 5*100/1=500 → 45
            25 => { set_xy(al[2] as *const Entity as *mut Entity, 100000, 100000); vis_enemy.push((0, tx + 30000, ty)); vis_enemy.push((1, tx + 30000, ty + 1000)); vis_enemy.push((2, tx + 30000, ty + 2000)); vis_enemy.push((3, tx + 30000, ty + 3000)); vis_enemy.push((4, tx + 70000, ty)); dr = 0; }   // champ 존 밖(아군 셋과 함께 → self_cover 4>1 → -36) · in_zone 4→캡3 · near 5→캡4 · 기대 5+6+14+12+18-36=19
            _ => {}
        }
        for (i, x, y) in vis_enemy.iter() { set_xy(en[*i] as *const Entity as *mut Entity, *x, *y); bb[1].last_visible[epos[*i]] = tick - 10; }
        let pp = param.0.as_mut_ptr();
        // param.player(+0x918): id=champ.id · team · risk_possible/gain_possible ptr 를 비-null 로
        p64(pp, 0x918 + 0x58, champ.id as i64); p64(pp, 0x918 + 0x60, 0); p64(pp, 0x918 + 0x18, 8); p64(pp, 0x918 + 0x38, 8);
        p64(pp, 0x918 + 0x70, self_applyed); p64(pp, 0x918 + 0x80, self_risk);
        p64(pp, 0x918 + 0xa8, 1000); p64(pp, 0x918 + 0xb0, 1000); p64(pp, 0x918 + 0xb8, 100); p64(pp, 0x918 + 0xc0, 50);
        for (k, (slot, ap, rk)) in pending.iter().enumerate() {
            let q = na[k].0.as_mut_ptr();
            p64(q, 0x58, al[*slot].id as i64); p64(q, 0x60, 0); p64(q, 0x18, 8); p64(q, 0x38, 8);
            p64(q, 0x70, *ap); p64(q, 0x80, *rk); p64(q, 0xa8, 1000); p64(q, 0xb0, 1000); p64(q, 0xb8, 100); p64(q, 0xc0, 50);
            allies.push(Ally { id: al[*slot].id, applyed: *ap, risk: *rk, ptr: q as *const game_ai::ChampionScoreParameter<'static> });
        }
        // near_allies: 연속 배열이 필요하므로 216B 박스들을 하나의 연속 버퍼로 복사
    }
    let mut nabuf = Box::new(Buf([0u8; 216 * 5]));
    for k in 0..pending.len() { nabuf.0[k * 216..(k + 1) * 216].copy_from_slice(&na[k].0); }
    for k in 0..pending.len() { allies[k].ptr = unsafe { nabuf.0.as_ptr().add(k * 216) } as *const game_ai::ChampionScoreParameter<'static>; }
    unsafe {
        let pp = param.0.as_mut_ptr();
        p64(pp, 0x14b8, nabuf.0.as_ptr() as i64); p64(pp, 0x14c8, 5); p64(pp, 0x14d0, pending.len() as i64);
        // near_enemies(+0x14d8) ptr 도 비-null
        p64(pp, 0x14d8, 8);
    }
    let ku = KnightUltAction { cooltime: 0, tick: 0, range, damage_reduce: dr, damage_reduce_hp_ratio: 0, move_speed_reduce: 0,
                               duration: dur, start_timing: 0, cast_range: 0, cancelable: false };
    let ku2 = KnightUltAction { cooltime: 0, tick: 0, range, damage_reduce: dr, damage_reduce_hp_ratio: 0, move_speed_reduce: 0,
                                duration: dur, start_timing: 0, cast_range: 0, cancelable: false };
    let action: Box<dyn Action> = Box::new(ku2);
    let action_ref: &Box<dyn Action> = if use_empty { &champ.empty } else { &action };
    let data = OperationData::new(&cache, &ctx, &bb);
    let sp: &game_ai::ScoreParameter = unsafe { &*(param.0.as_ptr() as *const game_ai::ScoreParameter) };
    let exp = model(player, &data, &cache, &bb, &game as &dyn AbstractGame, sp, if use_empty { None } else { Some(&ku) }, champ, target,
                    &allies, self_applyed, self_risk);
    let hv = game_ai::champion_hp_value(&data, sp, &sp.player);
    let got = unsafe { v16(player as *const PlayerState, &data as *const OperationData, sp as *const game_ai::ScoreParameter,
                           action_ref as *const Box<dyn Action>, champ as *const Entity, target as *const Entity) };
    println!("case={}\tgot={}\tmodel={}\t{}\t(hp_value(self)={} target.id={} champ.id={} range={} dr={})", case, got, exp,
             if got == exp { "MATCH" } else { "DIFF" }, hv, target.id, champ.id, range, dr);
}
