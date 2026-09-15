#![allow(unused, dead_code, non_snake_case)]
//! 24차 C · 182 SmallActionRunAway::get_input 오라클 — hidden 심볼 `#[link_name]` 직접 진입.
//! 수법 ⓓ(독립 재구현 대조): 명세(IR 독해)대로 L96~L470 목표 선정을 다시 구현해(콜리는 게임 hidden 심볼 그대로 호출)
//! 실제 get_input 이 &mut self 에 남긴 goal_x/goal_y/goal_risk/goal_committed/prog_best_* 와 대조한다.
//! 갈리는 독해 2종을 변형으로 함께 예측한다: ①blackboard[1-team](IR) vs [team](명세 구판) ②walls[yi][xi](IR) vs [xi][yi](명세 구판).
//! 한 프로세스 = 한 케이스(argv[1]).
use game_core::*;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB4_18SmallActionRunAway9get_input"]
    fn ra_get_input(s: &mut game_ai::SmallActionRunAway, version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData, dbg: &mut DebugFrameData) -> Option<Input>;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12small_action25positioning_score_at_cell"]
    fn ps_at_cell(version: usize, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData, xi: i64, yi: i64, purpose: game_ai::PositionEvalPurpose) -> PositioningScore;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12small_action29positioning_score_at_position"]
    fn ps_at_pos(version: usize, player: &PlayerState, data: &OperationData, ps: &PositioningScoreData, x: u64, y: u64, purpose: game_ai::PositionEvalPurpose) -> PositioningScore;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12small_action22positioning_risk_value"]
    fn risk_value(version: usize, player: &PlayerState, risk: i64, on_traj: bool) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12small_action25positioning_choice_window"]
    fn choice_window(version: usize, player: &PlayerState, min: usize, max: usize, b: bool) -> (usize, usize);
    #[link_name = "_RINvNtCshdEBA0ozCnw_7game_ai12small_action23positioning_window_pickTyyxEEB4_"]
    fn window_pick<'a>(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData, cands: &'a [(u64, u64, i64)]) -> &'a (u64, u64, i64);
}

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30;
    s.respawn_growth_term = 1800; s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20; s.support_gold_reduction = 15;
    s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
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
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

// ---- raw 접근 헬퍼 (명세 오프셋 그대로) ----
unsafe fn rd_u64(p: *const u8, off: usize) -> u64 { std::ptr::read_volatile(p.add(off) as *const u64) }
unsafe fn rd_i64(p: *const u8, off: usize) -> i64 { std::ptr::read_volatile(p.add(off) as *const i64) }
unsafe fn rd_u8(p: *const u8, off: usize) -> u8 { std::ptr::read_volatile(p.add(off)) }
unsafe fn wr_u64(p: *mut u8, off: usize, v: u64) { std::ptr::write_volatile(p.add(off) as *mut u64, v) }
unsafe fn wr_u8(p: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(p.add(off), v) }
fn ex(e: &Entity) -> u64 { unsafe { rd_u64(e as *const Entity as *const u8, 0x660) } }
fn ey(e: &Entity) -> u64 { unsafe { rd_u64(e as *const Entity as *const u8, 0x668) } }
fn absd(a: u64, b: u64) -> u64 { if a < b { b - a } else { a - b } }
fn dsq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 { let dx = absd(x1, x2); let dy = absd(y1, y2); dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) }
fn dsq_e(a: &Entity, b: &Entity) -> u64 { dsq(ex(a), ey(a), ex(b), ey(b)) }

#[derive(Debug, Clone, PartialEq, Default)]
struct SaSnap { goal_x: u64, goal_y: u64, goal_risk: i64, prog_best: u64, prog_tick: u64, committed: u8, pf_tag: u8, with_skill: u8, with_ult: u8, dodge: u8 }
fn snap(sa: &game_ai::SmallActionRunAway) -> SaSnap {
    let p = sa as *const _ as *const u8;
    unsafe { SaSnap { goal_x: rd_u64(p, 8), goal_y: rd_u64(p, 16), goal_risk: rd_i64(p, 32), prog_best: rd_u64(p, 40), prog_tick: rd_u64(p, 48),
        committed: rd_u8(p, 0x83), pf_tag: rd_u8(p, 0x7d), with_skill: rd_u8(p, 0x80), with_ult: rd_u8(p, 0x81), dodge: rd_u8(p, 0x82) } }
}

#[derive(Debug, Clone, Default)]
struct Pred { ret_none: bool, kept: bool, goal_x: u64, goal_y: u64, goal_risk: i64, committed: u8, prog_best: u64, prog_tick: u64,
    home: (u64, u64), nearby: u64, risk_weight: i64, n_cand: usize, n_cell_scores: usize, enemy_dist: u64, vis_cnt: usize, note: String }

/// L96~L470 독립 재구현. bb_enemy=true → blackboard[1-team](IR) · walls_yx=true → walls[yi][xi](IR).
unsafe fn predict(bb_enemy: bool, walls_yx: bool, sa: &game_ai::SmallActionRunAway, version: usize, rnd: &mut rand::rngs::StdRng,
                  player: &PlayerState, data: &OperationData, ps: &PositioningScoreData, game: &dyn AbstractGame, map: &MapDef, ms: &MapSetting) -> Pred {
    let mut P = Pred::default();
    let cache = data.cache;
    let team = player.info.team; assert!(team < 2);
    let pos = player.info.position as usize;
    let champ = match cache.player_champion[team][pos] { Some(c) => c, None => { P.ret_none = true; return P; } };
    // L100
    let nearest_ally_tower: Option<&Entity> = cache.iter_towers(team).min_by_key(|t| t.distance(champ).wrapping_add(if t.can_target && t.block_target_tick == 0 { 0 } else { 100000 }));
    let nexus = cache.nexus[team];
    let enemy_team = 1 - team;
    let bbi = if bb_enemy { enemy_team } else { team };
    let bb = &data.blackboard[bbi];
    let nearest_enemy = cache.player_champion[enemy_team].iter().flatten().filter(|e| bb.is_recent_visible(cache.game, player, e)).min_by_key(|e| dsq_e(e, champ));
    let enemy_champion_to_champ = nearest_enemy.map(|e| e.distance(champ)).unwrap_or(u64::MAX);
    P.enemy_dist = enemy_champion_to_champ;
    let (enemy_positions, vis_cnt) = game_ai::collect_visible_enemy_positions(player, data);
    P.vis_cnt = vis_cnt;
    let enemy_knows = version > 1 && game_ai::enemy_knows_my_position(player, data);
    let eff_risk = |s: &PositioningScore| -> i64 { s.risk.wrapping_add(if enemy_knows { s.unseen_champ_threat } else { 0 }) };
    let now_tick = game.tick() as u64;
    let sp = sa as *const _ as *const u8;
    let goal_x0 = rd_u64(sp, 8); let goal_y0 = rd_u64(sp, 16);
    let (cx, cy) = (ex(champ), ey(champ));
    // L125~135 진행도
    let pf_tag = rd_u8(sp, 0x7d);
    let prog_now = if pf_tag == 2 || rd_u64(sp, 0x48) == 0 {
        game_core::utils::usqrt(dsq(cx, cy, goal_x0, goal_y0))
    } else {
        let plen = rd_u64(sp, 0x48); let idx = std::cmp::min(plen - 1, rd_u64(sp, 0x50)); assert!(idx < 70);
        let pathp = rd_u64(sp, 0x60) as *const u8;
        let wx = rd_u64(pathp, (idx * 16) as usize); let wy = rd_u64(pathp, (idx * 16 + 8) as usize);
        game_core::utils::usqrt(dsq(cx, cy, wx, wy)).wrapping_add((plen - 1 - idx).wrapping_mul(32000))
    };
    let mut prog_best = rd_u64(sp, 0x28); let mut prog_tick = rd_u64(sp, 0x30);
    if prog_now < prog_best { prog_best = prog_now; prog_tick = now_tick; }
    let mut committed = rd_u8(sp, 0x83);
    let goal_risk0 = rd_i64(sp, 0x20);
    let dodge = rd_u8(sp, 0x82) != 0;
    let mut stale: Option<(u64, u64)> = None;
    // L138~161
    if committed != 0 {
        if now_tick.saturating_sub(prog_tick) > 119 {
            stale = Some((std::cmp::min(goal_x0 / 32000, 29), std::cmp::min(goal_y0 / 32000, 29)));
            committed = 0;
        } else if dsq(cx, cy, goal_x0, goal_y0) > 143999999 && !game_ai::is_enemy_well_danger(version, player, goal_x0, goal_y0) {
            let s = ps_at_pos(version, player, data, ps, goal_x0, goal_y0, game_ai::PositionEvalPurpose::RunAway);
            let on_traj = s.on_trajectory || s.on_periodic_trajectory;
            let risk_now = risk_value(version, player, eff_risk(&s), on_traj);
            let keep = risk_now <= goal_risk0 && (!dodge || !on_traj);
            if keep {
                P.kept = true; P.goal_x = goal_x0; P.goal_y = goal_y0; P.goal_risk = goal_risk0; P.committed = committed;
                P.prog_best = prog_best; P.prog_tick = prog_tick; P.note = format!("KEEP risk_now={} goal_risk={} on_traj={}", risk_now, goal_risk0, on_traj);
                return P;
            }
        }
    }
    // L162~201 홈
    let mp = map as *const MapDef as *const u8;
    let fo = 0x6d70 + team * 32;
    let (flx, fly, frx, fry) = (rd_u64(mp, fo), rd_u64(mp, fo + 8), rd_u64(mp, fo + 16), rd_u64(mp, fo + 24));
    let (mut hx, mut hy) = ((flx.wrapping_add(frx)) >> 1, (fly.wrapping_add(fry)) >> 1);
    if let (Some(tower), Some(nx)) = (nearest_ally_tower, nexus) {
        let tower_to_nexus = tower.distance(nx); let nexus_to_champ = champ.distance(nx); let tower_to_champ = tower.distance(champ);
        let hp = rd_u64(champ as *const Entity as *const u8, 0x670); let maxhp = rd_u64(champ as *const Entity as *const u8, 0x628);
        let hp_ratio = hp.wrapping_mul(100) / std::cmp::max(maxhp, 1);
        let near_base = dsq_e(champ, nx) < 67600000001 || cache.twin_towers[team].iter().any(|t| dsq_e(champ, t) < 32400000001);
        let mut go_tower = true;
        if near_base && (hp_ratio < 36 || (hp_ratio < 46 && enemy_champion_to_champ < 160001)) {
            go_tower = game_ai::plan_legacy::old::nexus_is_critical(player, data) || (version > 1 && game_ai::plan_legacy::old::nexus_final_stand(player, data));
        }
        if go_tower {
            if tower_to_nexus > nexus_to_champ.saturating_sub(30000) && tower_to_champ > enemy_champion_to_champ.saturating_sub(30000) {
            } else if vis_cnt <= 1 { hx = ex(tower); hy = ey(tower); }
            else {
                let near = enemy_positions[..vis_cnt].iter().filter(|p| dsq(ex(tower), ey(tower), p.0, p.1) < 14400000000).count();
                if near > 1 {} else { hx = ex(tower); hy = ey(tower); }
            }
        }
    }
    P.home = (hx, hy);
    // L213~305
    let champ_cx = cx / 32000; let champ_cy = cy / 32000; let hcx = hx / 32000; let hcy = hy / 32000;
    let (pcx, pcy) = (ps.cx as u64, ps.cy as u64);
    let pve = game_ai::PveHazardContext::new(version, player, data);
    let nearby = enemy_positions[..vis_cnt].iter().filter(|p| dsq(cx, cy, p.0, p.1) < 40000000000).count() as u64;
    let risk_weight: i64 = if nearby > 2 { 5 } else if nearby == 2 { 3 } else { 1 };
    P.nearby = nearby; P.risk_weight = risk_weight;
    let f2_anchor = if version > 1 { enemy_positions[..vis_cnt].iter().filter(|p| dsq(cx, cy, p.0, p.1) < 62500000000).min_by_key(|p| dsq(cx, cy, p.0, p.1)).copied() } else { None };
    let f2_now = f2_anchor.map(|a| game_core::utils::distance(cx, cy, a.0, a.1));
    let no_structures = nearest_ally_tower.is_none() && nexus.is_none();
    let for_nexus_weight: i64 = if no_structures { 0 } else { 10 };
    let b1_free: Option<Arc<game_ai::FreeDistMap>> = if version > 1 && !no_structures { Some(game_ai::shared_free_dist(map)) } else { None };
    let free_d = |f: &Arc<game_ai::FreeDistMap>, i: usize| -> u16 { let v: &Vec<u16> = std::mem::transmute(&**f); v[i] };
    let mut b1_w: i64 = 0;
    if version > 1 && !no_structures {
        let speed = rd_u64(champ as *const Entity as *const u8, 0x640);
        let edge_ticks = std::cmp::max(32000 / std::cmp::max(speed, 1), 1);
        let mut edge_dmg: u64 = 0;
        for e in cache.player_champion[enemy_team].iter().flatten() {
            if !bb.is_recent_visible(cache.game, player, e) { continue; }
            if dsq_e(e, champ) > 40000000000 { continue; }
            let dps = game_ai::fight_dps(version, data.context, e, champ) as u64;
            edge_dmg = edge_dmg.saturating_add(dps.saturating_mul(edge_ticks));
        }
        let hp = rd_u64(champ as *const Entity as *const u8, 0x670);
        let chase = edge_dmg / (std::cmp::max(hp, 1) * 10);
        b1_w = ((chase as i64).wrapping_mul(risk_weight)) / 5 + 2;
    }
    let field_right = team == 0;
    let mut structure_cells: Vec<(u64, u64)> = cache.iter_towers(team).map(|t| (ex(t) / 32000, ey(t) / 32000)).collect();
    if let Some(nx) = nexus { structure_cells.push((ex(nx) / 32000, ey(nx) / 32000)); }
    let c1_chaser = if version > 1 { enemy_positions[..vis_cnt].iter().filter(|p| dsq(cx, cy, p.0, p.1) < 40000000000).min_by_key(|p| dsq(cx, cy, p.0, p.1)).copied() } else { None };
    let home_idx = std::cmp::min(hcy, 29) * 30 + std::cmp::min(hcx, 29);
    let c1_home_me: Option<u16> = b1_free.as_ref().map(|f| free_d(f, ((std::cmp::min(champ_cy, 29) * 30 + std::cmp::min(champ_cx, 29)) * 900 + home_idx) as usize));
    let mut candidates: Vec<(u64, u64, i64)> = Vec::new();
    let mut cell_scores: Vec<(u64, u64, i64)> = Vec::new();
    let path = &ms.path;
    for dx in 0..7u64 { let xi = pcx.wrapping_sub(3).wrapping_add(dx); for dy in 0..7u64 { let yi = pcy.wrapping_sub(3).wrapping_add(dy);
        if xi > 29 || yi > 29 { continue; }
        let wall = if walls_yx { rd_u64(mp, 0x78 + (yi * 240 + xi * 8) as usize) } else { rd_u64(mp, 0x78 + (xi * 240 + yi * 8) as usize) };
        if wall != 0 { continue; }
        let wx = xi * 32000 + 16000; let wy = yi * 32000 + 16000;
        if no_structures && ((xi == pcx && yi == pcy) || (if field_right { wx <= frx } else { wx >= flx })) { continue; }
        if game_ai::is_enemy_well_danger(version, player, wx, wy) { continue; }
        if game_ai::v3_lethal_tower_position(version, player, data, wx, wy) { continue; }
        if game_ai::v3_lethal_wave_position(version, player, data, wx, wy) { continue; }
        if pve.lethal_zone_goal(wx, wy) || stale == Some((xi, yi)) { continue; }
        if let Some(a) = f2_anchor { if game_core::utils::distance(wx, wy, a.0, a.1).wrapping_add(8000) < f2_now.unwrap() { continue; } }
        if structure_cells.contains(&(xi, yi)) { continue; }
        let cs = ps_at_cell(version, player, data, ps, xi as i64, yi as i64, game_ai::PositionEvalPurpose::RunAway);
        if dodge && (cs.on_trajectory || cs.on_periodic_trajectory) { continue; }
        let risk = match path.find_path(std::cmp::min(champ_cx, 29) as usize, std::cmp::min(champ_cy, 29) as usize, xi as usize, yi as usize) {
            Some((nx, ny)) => { let n = ps_at_cell(version, player, data, ps, nx as i64, ny as i64, game_ai::PositionEvalPurpose::RunAway); (eff_risk(&n).wrapping_add(eff_risk(&cs))) / 2 }
            None => eff_risk(&cs) };
        let risk = risk_value(version, player, risk, cs.on_trajectory || cs.on_periodic_trajectory);
        let score = match &b1_free {
            None => { let fn_ = ((hcx as i64 - xi as i64).abs() + (hcy as i64 - yi as i64).abs()); 0i64.wrapping_sub(risk.wrapping_mul(risk_weight).wrapping_add(fn_.wrapping_mul(for_nexus_weight))) }
            Some(f) => { let mut d = free_d(f, ((yi * 30 + xi) * 900 + home_idx) as usize); if d == 0xFFFF { d = 150; } 0i64.wrapping_sub(b1_w.wrapping_mul(d as i64).wrapping_add(risk.wrapping_mul(risk_weight))) } };
        candidates.push((wx, wy, score));
        if let (Some(ch), Some(f)) = (c1_chaser, &b1_free) {
            let home_cand = free_d(f, ((yi * 30 + xi) * 900 + home_idx) as usize);
            let worse = dsq(wx, wy, ch.0, ch.1) < dsq(cx, cy, ch.0, ch.1) && home_cand != 0xFFFF && home_cand > c1_home_me.unwrap();
            if worse { continue; }
        }
        cell_scores.push((wx, wy, score));
    } }
    P.n_cand = candidates.len(); P.n_cell_scores = cell_scores.len();
    if version > 1 && !cell_scores.is_empty() { candidates = cell_scores; }
    let (gx, gy);
    if candidates.is_empty() {
        if no_structures { gx = if field_right { frx.wrapping_add(32000) } else { flx.saturating_sub(32000) }; gy = cy; }
        else { gx = hcx * 32000 + 16000; gy = hcy * 32000 + 16000; }
        P.goal_risk = goal_risk0; committed = 0;
        P.note = format!("EMPTY no_structures={}", no_structures);
    } else {
        candidates.sort_by_key(|c| 0i64.wrapping_sub(c.2));
        let (mn, mx) = choice_window(version, player, player.info.parameter.positioning_runaway_min_range(), player.info.parameter.positioning_runaway_max_range(), true);
        let len = candidates.len();
        let min_idx = std::cmp::min(len * mn / 1000, len - 1);
        let max_idx = { let r = len * mx / 1000; if r < min_idx { min_idx } else if r > len - 1 { len - 1 } else { r } };
        let cands: Vec<(u64, u64, i64)> = candidates.iter().skip(min_idx).take(max_idx - min_idx + 1).copied().collect();
        let picked: (u64, u64, i64) = if version > 1 { *window_pick(version, rnd, player, data, &cands) } else { *cands.choose(rnd).unwrap() };
        let pre = ps_at_pos(version, player, data, ps, goal_x0, goal_y0, game_ai::PositionEvalPurpose::RunAway);
        let pre_risk = eff_risk(&pre);
        let pre_xi = std::cmp::min(goal_x0 / 32000, 29); let pre_yi = std::cmp::min(goal_y0 / 32000, 29);
        let pre_score = match &b1_free {
            None => { let fn_ = (hcx as i64 - pre_xi as i64).abs() + (hcy as i64 - pre_yi as i64).abs(); 0i64.wrapping_sub(pre_risk.wrapping_mul(risk_weight).wrapping_add(fn_.wrapping_mul(for_nexus_weight))) }
            Some(f) => { let mut d = free_d(f, ((pre_yi * 30 + pre_xi) * 900 + home_idx) as usize); if d == 0xFFFF { d = 150; } 0i64.wrapping_sub(b1_w.wrapping_mul(d as i64).wrapping_add(pre_risk.wrapping_mul(risk_weight))) } };
        if picked.2 > pre_score { gx = picked.0; gy = picked.1; }
        else if dsq(cx, cy, goal_x0, goal_y0) > 143999999 { gx = goal_x0; gy = goal_y0; }
        else { gx = picked.0; gy = picked.1; }
        let s = ps_at_pos(version, player, data, ps, gx, gy, game_ai::PositionEvalPurpose::RunAway);
        P.goal_risk = risk_value(version, player, eff_risk(&s), s.on_trajectory || s.on_periodic_trajectory);
        committed = 1;
        P.note = format!("PICK len={} win=({},{}) idx=[{}..={}] picked={:?} pre_score={} pre_risk={} b1_w={} fnw={}", len, mn, mx, min_idx, max_idx, picked, pre_score, pre_risk, b1_w, for_nexus_weight);
    }
    P.goal_x = gx; P.goal_y = gy; P.committed = committed; P.prog_best = u64::MAX; P.prog_tick = now_tick;
    P
}

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    println!("setting_ok\ttps={}\twidth={}\theight={}", setting.tick_per_second, setting.width, setting.height);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    // walls 대칭성(전치) 검사 — walls[a][b] != walls[b][a] 셀 수
    let mp = &map as *const MapDef as *const u8;
    let mut asym = 0; let mut nwall = 0; let mut asym_list: Vec<(u64,u64)> = Vec::new();
    for a in 0..30u64 { for b in 0..30u64 { unsafe {
        let w1 = rd_u64(mp, 0x78 + (a * 240 + b * 8) as usize); let w2 = rd_u64(mp, 0x78 + (b * 240 + a * 8) as usize);
        if w1 != 0 { nwall += 1; }
        if w1 != w2 { asym += 1; if asym_list.len() < 12 { asym_list.push((a, b)); } }
    } } }
    println!("walls\tnonzero={}\ttranspose_asym={}\tsample(a=row,b=col: walls[a][b]!=walls[b][a])={:?}", nwall, asym, asym_list);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let version: usize = if case == 6 { 1 } else { 2 };
    let tick: usize = match case { 0 | 1 | 2 => 0, _ => 1000 };
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    // blackboard.last_visible: 기본 0 → tick 0 이면 전부 '최근 가시'. tick 1000 에선 last_visible 값으로 제어.
    let team = 0usize; let enemy = 1usize;
    match case {
        4 | 9 | 10 | 11 | 12 => { for p in 0..5 { bb[enemy].last_visible[p] = tick; bb[team].last_visible[p] = 0; } }   // 적 팀 blackboard 만 최근
        5 => { for p in 0..5 { bb[enemy].last_visible[p] = 0; bb[team].last_visible[p] = tick; } }              // 내 팀 blackboard 만 최근 (IR 독해면 적 비가시)
        _ => { for p in 0..5 { bb[enemy].last_visible[p] = tick; bb[team].last_visible[p] = tick; } }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, Position::Top).unwrap();
    let champ = cache.player_champion[team][0].unwrap();
    let cp = champ as *const Entity as *mut Entity as *mut u8;
    let nexus = cache.nexus[team].unwrap();
    let (nxx, nxy) = (ex(nexus), ey(nexus));
    println!("champ0\tid={} x={} y={} hp={} maxhp={} speed={}\tnexus=({}, {})\tfountain={:?}", champ.id, ex(champ), ey(champ), unsafe{rd_u64(cp,0x670)}, unsafe{rd_u64(cp,0x628)}, unsafe{rd_u64(cp,0x640)}, nxx, nxy,
        unsafe { (rd_u64(mp, 0x6d70), rd_u64(mp, 0x6d78), rd_u64(mp, 0x6d80), rd_u64(mp, 0x6d88)) });
    // default 챔프는 hp=maxhp=1·speed=1 (TEMPLATE 함정 ⑥) → 실전형 값으로 세팅 (max 1000 · speed 300)
    unsafe { wr_u64(cp, 0x628, 1000); wr_u64(cp, 0x670, 1000); wr_u64(cp, 0x640, 300); }
    let maxhp = unsafe { rd_u64(cp, 0x628) };
    let e0 = cache.player_champion[enemy][0].unwrap() as *const Entity as *mut Entity as *mut u8;
    let e1 = cache.player_champion[enemy][1].unwrap() as *const Entity as *mut Entity as *mut u8;
    let e2 = cache.player_champion[enemy][2].unwrap() as *const Entity as *mut Entity as *mut u8;
    unsafe {
        match case {
            0 => {}
            1 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000); }                                   // 맵 중앙, 풀피
            2 => { wr_u64(cp, 0x660, nxx + 150000); wr_u64(cp, 0x668, nxy); wr_u64(cp, 0x670, maxhp * 30 / 100); }   // 본진 근처(150000) hp30 → 분수
            3 => { wr_u64(cp, 0x660, nxx + 150000); wr_u64(cp, 0x668, nxy); wr_u64(cp, 0x670, maxhp * 36 / 100); }   // hp36 → 타워(경계)
            4 | 5 => { wr_u64(cp, 0x660, nxx + 150000); wr_u64(cp, 0x668, nxy); wr_u64(cp, 0x670, maxhp * 40 / 100);   // hp40 + 적 1명 150000 안
                       wr_u64(e0, 0x660, nxx + 150000 + 120000); wr_u64(e0, 0x668, nxy); }
            6 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000); }                                   // version 1
            7 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000); }                                   // committed + 정체(stale)
            8 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000); }                                   // committed + 유지(keep)
            9 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000);                                     // 적 3명 근접(200000 안) → risk_weight 5 · f2 · c1 · fight_dps
                   wr_u64(e0, 0x660, 480000 + 90000); wr_u64(e0, 0x668, 480000);
                   wr_u64(e1, 0x660, 480000); wr_u64(e1, 0x668, 480000 + 110000);
                   wr_u64(e2, 0x660, 480000 - 130000); wr_u64(e2, 0x668, 480000 - 50000); }
            11 => { wr_u64(cp, 0x660, nxx + 150000); wr_u64(cp, 0x668, nxy); wr_u64(cp, 0x670, maxhp * 40 / 100);      // hp40 + 적 거리 정확히 160000 → 분수(경계 포함)
                    wr_u64(e0, 0x660, nxx + 150000 + 160000); wr_u64(e0, 0x668, nxy); }
            12 => { wr_u64(cp, 0x660, nxx + 150000); wr_u64(cp, 0x668, nxy); wr_u64(cp, 0x670, maxhp * 40 / 100);      // hp40 + 적 거리 160001 → 타워
                    wr_u64(e0, 0x660, nxx + 150000 + 160001); wr_u64(e0, 0x668, nxy); }
            13 | 14 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000); }                              // 정체 경계 120 / 119
            10 => { wr_u64(cp, 0x660, 480000); wr_u64(cp, 0x668, 480000);                                    // 적 2명(=2 → risk_weight 3)
                   wr_u64(e0, 0x660, 480000 + 90000); wr_u64(e0, 0x668, 480000);
                   wr_u64(e1, 0x660, 480000); wr_u64(e1, 0x668, 480000 + 110000); }
            _ => {}
        }
    }
    let mut ps: PositioningScoreData = Default::default();
    ps.cx = (ex(champ) / 32000) as usize; ps.cy = (ey(champ) / 32000) as usize;
    let mut sa = game_ai::SmallActionRunAway::new(&data, player, version);
    let sp = &mut sa as *mut _ as *mut u8;
    unsafe {
        match case {
            7 => { wr_u8(sp, 0x83, 1); wr_u64(sp, 8, 480000 + 96000); wr_u64(sp, 16, 480000); wr_u64(sp, 0x30, tick as u64 - 200); wr_u64(sp, 0x28, 999); }
            8 => { wr_u8(sp, 0x83, 1); wr_u64(sp, 8, 480000 + 96000); wr_u64(sp, 16, 480000); wr_u64(sp, 0x30, tick as u64 - 10); wr_u64(sp, 0x28, 999); std::ptr::write_volatile(sp.add(0x20) as *mut i64, i64::MAX); }
            13 => { wr_u8(sp, 0x83, 1); wr_u64(sp, 8, 480000 + 96000); wr_u64(sp, 16, 480000); wr_u64(sp, 0x30, tick as u64 - 120); wr_u64(sp, 0x28, 999); std::ptr::write_volatile(sp.add(0x20) as *mut i64, i64::MAX); }  // 120 > 119 → stale
            14 => { wr_u8(sp, 0x83, 1); wr_u64(sp, 8, 480000 + 96000); wr_u64(sp, 16, 480000); wr_u64(sp, 0x30, tick as u64 - 119); wr_u64(sp, 0x28, 999); std::ptr::write_volatile(sp.add(0x20) as *mut i64, i64::MAX); }  // 119 → keep
            _ => {}
        }
    }
    let before = snap(&sa);
    println!("case={}\tversion={}\ttick={}\tsa_before={:?}", case, version, tick, before);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let mut rnd_clone = rnd.clone();
    // 예측 3종 (IR 독해 / bb[team] 변형 / walls[xi][yi] 변형) — 각각 rnd 사본
    let p_ir = unsafe { predict(true, true, &sa, version, &mut rnd.clone(), player, &data, &ps, &game as &dyn AbstractGame, &map, &ms) };
    let p_bb = unsafe { predict(false, true, &sa, version, &mut rnd.clone(), player, &data, &ps, &game as &dyn AbstractGame, &map, &ms) };
    let p_wl = unsafe { predict(true, false, &sa, version, &mut rnd.clone(), player, &data, &ps, &game as &dyn AbstractGame, &map, &ms) };
    let mut dbg: DebugFrameData = Default::default();
    let ret = unsafe { ra_get_input(&mut sa, version, &mut rnd, player, &data, &ps, &mut dbg) };
    let after = snap(&sa);
    println!("real\tret={:?}\tsa_after={:?}", ret.as_ref().map(|i| format!("{:?}", i)), after);
    for (nm, p) in [("IR", &p_ir), ("bb[team]", &p_bb), ("walls[xi][yi]", &p_wl)] {
        let m = (p.goal_x == after.goal_x) && (p.goal_y == after.goal_y) && (p.goal_risk == after.goal_risk) && (p.committed == after.committed) && (p.prog_best == after.prog_best) && (p.prog_tick == after.prog_tick);
        println!("pred[{}]\t{}\tgoal=({}, {}) risk={} committed={} prog=({}, {}) kept={} home={:?} nearby={} rw={} ncand={}/{} enemy_dist={} vis={}\t{}",
            nm, if m { "MATCH" } else { "MISMATCH" }, p.goal_x, p.goal_y, p.goal_risk, p.committed, p.prog_best, p.prog_tick, p.kept, p.home, p.nearby, p.risk_weight, p.n_cand, p.n_cell_scores, p.enemy_dist, p.vis_cnt, p.note);
    }
}
