#![allow(unused, dead_code, non_snake_case)]
//! 23차 A 오라클 — 134 check_cell(hidden, link_name) · 135 v3_lethal_tower_position · 136 v23_should_break_objective_hunt_anchor
//! · 140 SmallActionAroundBush::new_with_target · 141 SmallActionPlay::get_input(Stop/Attack 경로).
//! 한 프로세스 = 한 케이스(argv[1]). 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh <이 파일>
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action6around10check_cell"]
    fn check_cell(version: usize, player: &PlayerState, ps: &PositioningScoreData, data: &OperationData,
                  td: &game_ai::TowerDodgeContext, out_line: game_ai::AroundBushOutlineType,
                  sx: usize, sy: usize, nx: usize, ny: usize, tx: usize, ty: usize) -> game_ai::PathVerdict;
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

// 140 명세 consts[0] 의 71셀 표 (x,y)
const BUSH_CELLS: [(usize, usize); 71] = [
 (0,0),(1,0),(2,0),(19,0),(20,0),(21,0),(0,1),(0,2),(7,4),(8,4),(9,4),(20,4),(20,5),(12,6),(4,7),(4,8),(29,8),(4,9),(14,9),(29,9),
 (29,10),(26,11),(6,12),(12,12),(13,12),(26,12),(12,13),(26,13),(9,14),(26,14),(20,15),(21,15),(17,16),(21,16),(16,17),(17,17),(25,18),(0,19),(25,19),(0,20),
 (4,20),(5,20),(15,20),(25,20),(0,21),(15,21),(16,21),(25,21),(25,22),(29,24),(18,25),(19,25),(20,25),(21,25),(22,25),(29,25),(11,26),(12,26),(13,26),(14,26),
 (29,26),(28,28),(29,28),(8,29),(9,29),(10,29),(24,29),(25,29),(26,29),(28,29),(29,29)];

fn absdiff(a: u64, b: u64) -> u64 { if a < b { b - a } else { a - b } }
fn dsq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 { let dx = absdiff(x1, x2); let dy = absdiff(y1, y2); dx * dx + dy * dy }

// 독립 재구현(140): 후보 필터 + min_by_key(첫 동률 유지)
fn mine140(map: &MapDef, tx: u64, ty: u64, bush: usize) -> Option<(u64, u64)> {
    let mut best: Option<(u64, (u64, u64))> = None;
    for &(x, y) in BUSH_CELLS.iter() {
        if map.bushes[y][x] != bush { continue; }
        let cx = x as u64 * 32000 + 16000; let cy = y as u64 * 32000 + 16000;
        let k = dsq(tx, ty, cx, cy);
        match best { None => best = Some((k, (cx, cy))), Some((bk, _)) => if k < bk { best = Some((k, (cx, cy))); } }
    }
    best.map(|b| b.1)
}
// 독립 재구현(135 클로저): any(dist² <= (range_or_0 + 20000 + radius)²)
fn mine135(cache: &AbstractGameWithCache, team: usize, x: u64, y: u64) -> bool {
    for t in cache.iter_towers_without_nexus(1 - team) {
        let r = t.attack_effect.as_ref().map(|e| e.range(t)).unwrap_or(0) + 20000 + t.radius() as u64;
        if dsq(t.x, t.y, x, y) <= r * r { return true; }
    }
    false
}
fn setxy(e: &Entity, x: u64, y: u64) { let p = e as *const Entity as *mut Entity; unsafe { std::ptr::write_volatile(&mut (*p).x, x); std::ptr::write_volatile(&mut (*p).y, y); } }
fn sethp(e: &Entity, hp: usize, max: usize) { let p = e as *const Entity as *mut Entity; unsafe { std::ptr::write_volatile(&mut (*p).hp, hp); std::ptr::write_volatile(&mut (*p).stat_cached.hp, max); } }

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    if std::env::args().count() > 99 { let _ = game_ai::v3_lethal_tower_position as *const (); }
    let setting = real_setting();
    println!("setting_ok\ttps={}\twidth={}\theight={}", setting.tick_per_second, setting.width, setting.height);
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
    println!("towers\t{}\ttwin0={}\ttwin1={}\ttick={}", game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len(), game.tick());
    let champ = cache.player_champion[0][0].unwrap();
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    println!("champ\tid={} x={} y={} hp={} max={} team={:?}", champ.id, champ.x, champ.y, champ.hp, champ.stat_cached.hp, champ.team);

    match case {
        // ───────── 140 ─────────
        10..=13 => {
            // 표에 있는 셀의 bush id 집합
            let mut ids: Vec<usize> = BUSH_CELLS.iter().map(|&(x, y)| map.bushes[y][x]).collect();
            ids.sort(); ids.dedup();
            println!("bush_ids_in_table\t{:?}", ids);
            let targets: Vec<(u64, u64)> = match case {
                10 => vec![(champ.x, champ.y)],
                11 => vec![(100000, 900000), (900000, 100000), (480000, 480000), (16000, 16000), (943000, 943000)],
                12 => { let mut v = Vec::new(); let mut s: u64 = 0x9E3779B97F4A7C15; for _ in 0..200 { s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); let x = (s >> 20) % 960000; s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); let y = (s >> 20) % 960000; v.push((x, y)); } v }
                13 => vec![(16000 + 32000 * 12, 16000 + 32000 * 12)],   // 셀 (12,12)와 (13,12)·(12,13) 동률 검사용: 정확히 셀 중심
                _ => vec![(champ.x, champ.y)],
            };
            let mut n = 0; let mut bad = 0;
            for &bush in ids.iter() {
                for &(tx, ty) in targets.iter() {
                    setxy(champ, tx, ty);
                    let ol = if n % 3 == 0 { game_ai::AroundBushOutlineType::None } else if n % 3 == 1 { game_ai::AroundBushOutlineType::Outline } else { game_ai::AroundBushOutlineType::Inline };
                    let r = game_ai::SmallActionAroundBush::new_with_target(&data, champ, bush, ol);
                    let exp = mine140(&map, tx, ty, bush).unwrap();
                    let ok = (r.target_x, r.target_y) == exp && r.bush == bush;
                    if !ok { bad += 1; }
                    if case != 12 || !ok { println!("140\tbush={} tgt=({},{})\tgame=({},{}) bush={}\tmine=({},{})\t{}\t{:?}", bush, tx, ty, r.target_x, r.target_y, r.bush, exp.0, exp.1, if ok { "MATCH" } else { "MISMATCH" }, r); }
                    n += 1;
                }
            }
            println!("140_summary\tcases={} mismatch={}", n, bad);
        }
        14 => { // 표에 없는 bush id → unwrap 패닉 기대(exit 101)
            let r = game_ai::SmallActionAroundBush::new_with_target(&data, champ, 987654, game_ai::AroundBushOutlineType::None);
            println!("140\tNO_PANIC\t{:?}", r);
        }
        15 => { // 셀 표 밖(0,0 등)에 있는 bush id 가 실제 map.bushes 에 있는지 — 표와 맵의 정합 관측
            let mut in_map = std::collections::BTreeMap::new();
            for y in 0..30 { for x in 0..30 { let b = map.bushes[y][x]; if b != 0 { *in_map.entry(b).or_insert(0usize) += 1; } } }
            let mut in_tbl = std::collections::BTreeMap::new();
            for &(x, y) in BUSH_CELLS.iter() { *in_tbl.entry(map.bushes[y][x]).or_insert(0usize) += 1; }
            println!("map_bush_cells\t{:?}", in_map);
            println!("table_bush_cells\t{:?}", in_tbl);
        }
        // ───────── 135 ─────────
        20..=29 => {
            let team = 0usize;
            // 타워 표
            for t in cache.iter_towers_without_nexus(1 - team) {
                let rr = t.attack_effect.as_ref().map(|e| e.range(t)).unwrap_or(0);
                println!("tower\tid={} x={} y={} range={} radius={} r={} lvl={} buf_range={} mult={}", t.id, t.x, t.y, rr, t.radius(), rr + 20000 + t.radius() as u64, t.level(), t.stat_buff_cached.range, t.stat_buff_cached.radius_mult);
            }
            let (version, hp) = match case { 20 => (2, 0usize), 21 => (1, 0), 22 => (2, 999999), 23 => (3, 0), _ => (2, 0) };
            sethp(champ, hp, 100000);
            println!("lethal_hp\t{}", game_ai::v3_lethal_tower_hp(&data, player, champ));
            let t0 = cache.iter_towers_without_nexus(1 - team).next().unwrap();
            let r0 = t0.attack_effect.as_ref().map(|e| e.range(t0)).unwrap_or(0) + 20000 + t0.radius() as u64;
            let mut pts: Vec<(u64, u64)> = vec![(t0.x, t0.y), (t0.x + r0 - 1, t0.y), (t0.x + r0, t0.y), (t0.x + r0 + 1, t0.y), (t0.x, t0.y + r0), (t0.x, t0.y + r0 + 1)];
            let mut s: u64 = 0x1234567;
            for _ in 0..300 { s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); let x = (s >> 20) % 960000; s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); let y = (s >> 20) % 960000; pts.push((x, y)); }
            // 타워 근처 밀집 표본(경계 ±3)
            for t in cache.iter_towers_without_nexus(1 - team) { let r = t.attack_effect.as_ref().map(|e| e.range(t)).unwrap_or(0) + 20000 + t.radius() as u64; for d in 0..7u64 { pts.push((t.x + r - 3 + d, t.y)); if t.y + r >= 3 { pts.push((t.x, t.y + r - 3 + d)); } } }
            let mut bad = 0; let mut n = 0; let mut ntrue = 0;
            for &(x, y) in pts.iter() {
                let g = game_ai::v3_lethal_tower_position(version, player, &data, x, y);
                let m = if version < 2 || hp != 0 { false } else { mine135(&cache, team, x, y) };
                if g { ntrue += 1; }
                if g != m { bad += 1; }
                if n < 6 || g != m { println!("135\tv={} hp={} pt=({},{})\tgame={} mine={}\t{}", version, hp, x, y, g, m, if g == m { "MATCH" } else { "MISMATCH" }); }
                n += 1;
            }
            println!("135_summary\tv={} hp={} cases={} true={} mismatch={}", version, hp, n, ntrue, bad);
        }
        // ───────── 136 ─────────
        40..=49 => {
            use game_ai::plan_legacy::team_plan::{v23_healthy_allies_near_point as allies, v23_recent_visible_enemies_near_point as enemies, v23_should_break_objective_hunt_anchor as f};
            let obj = cache.player_champion[1][4].unwrap();
            setxy(obj, 900000, 900000);
            let camp = (500000u64, 500000u64);
            // 배치: 케이스별
            let (hp, max) = match case { 40 => (20, 100), 41 => (21, 100), 42 => (21, 100), 43 => (21, 100), 44 => (50, 0), 45 => (21, 100), 46 => (20, 100), 47 => (2099, 10000), 48 => (2100, 10000), 49 => (50, 0), _ => (21, 100) };
            sethp(obj, hp, max);
            match case {
                42 => { for p in 0..3 { setxy(cache.player_champion[1][p].unwrap(), camp.0, camp.1); } }               // 캠프 주변 적 3
                43 | 45 | 46 | 47 | 48 => { for p in 0..3 { setxy(cache.player_champion[1][p].unwrap(), champ.x + 1000, champ.y); } for p in 1..5 { setxy(cache.player_champion[0][p].unwrap(), 900000, 20000); } } // 내 주변 적 3, 아군 원거리
                _ => {}
            }
            if case == 49 { let g = f(player, &data, champ, obj, camp); println!("136	NO_PANIC {}", g); return; }
            let pa = allies(player, &data, champ.x, champ.y, 180000, 40);
            let pe = enemies(player, &data, champ.x, champ.y, 160000, 40);
            let ca = allies(player, &data, camp.0, camp.1, 180000, 40);
            let ce = enemies(player, &data, camp.0, camp.1, 180000, 40);
            println!("helpers\tpersonal_allies={} personal_enemies={} camp_allies={} camp_enemies={}", pa, pe, ca, ce);
            let mine = if max == 0 { None } else if hp * 100 / max < 21 { Some(false) } else if pe > 1 && pe > pa { Some(true) } else { Some(ce > 2 && ce >= ca + 2) };
            let g = f(player, &data, champ, obj, camp);
            println!("136\thp={} max={} pct={}\tgame={} mine={:?}\t{}", hp, max, if max > 0 { hp * 100 / max } else { 0 }, g, mine, if Some(g) == mine { "MATCH" } else { "MISMATCH" });
        }
        // ───────── 141 ─────────
        60..=69 => {
            let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
            let ps: PositioningScoreData = Default::default();
            let mut dbg: DebugFrameData = Default::default();
            let version = match case { 61 | 63 | 67 => 1usize, _ => 2 };
            // 위치 세팅
            if case == 66 || case == 67 {
                // 위험 경계 탐색: escape 점에서 x 를 늘리며 danger 가 처음 true 가 되는 x → 거기 champ 를 두고 escape 까지 dsq 관측
                let f = map.fountains[1];
                let (ex, ey) = game_ai::enemy_well_escape_position(&setting, &map, player, f.0, f.1);
                let mut first: Option<u64> = None;
                for x in (ex..960000).step_by(500) { if game_ai::is_enemy_well_danger(2, player, x, ey) { first = Some(x); break; } }
                println!("well\tdanger_first_x={:?}\tescape=({},{})", first, ex, ey);
                if let Some(x) = first { setxy(champ, x, ey); let (ex2, ey2) = game_ai::enemy_well_escape_position(&setting, &map, player, x, ey); println!("well\tescape_from_boundary=({},{}) dsq={}", ex2, ey2, dsq(x, ey, ex2, ey2)); }
            }
            match case {
                62 | 63 => { let f = map.fountains[1]; setxy(champ, f.0, f.1); println!("fountain1\t{:?}", f); }
                64 => { let f = map.fountains[1]; let (ex, ey) = game_ai::enemy_well_escape_position(&setting, &map, player, f.0, f.1); setxy(champ, ex, ey); println!("escape_pos\t({},{})", ex, ey); }
                _ => {}
            }
            let mut act = match case { 65 => game_ai::SmallActionPlay::Attack(game_ai::SmallActionAttack::new(&data, cache.player_champion[1][0].unwrap().id)), _ => game_ai::SmallActionPlay::Stop };
            let w1 = game_ai::is_enemy_well_danger(version, player, champ.x, champ.y);
            let w2 = game_ai::is_recent_enemy_well_damage_danger(version, player, champ);
            println!("well\tdanger={} recent={}", w1, w2);
            // 독립 재구현(Stop 경로)
            let mine: Option<Input> = (|| {
                if w1 || w2 {
                    let (x, y) = game_ai::enemy_well_escape_position(&setting, &map, player, champ.x, champ.y);
                    if version <= 1 || dsq(champ.x, champ.y, x, y) >= 4000001 { return Some(Input::Move { x, y }); }
                }
                if case == 65 { return None; } // Attack 콜리는 독립 재구현 대상 아님(관측만)
                let input = Some(Input::Move { x: champ.x, y: champ.y });
                let mut out = game_ai::safe_move_avoiding_enemy_well(version, player, &data, champ, champ.x, champ.y);
                if version > 1 { if let Some(Input::Move { x, y }) = out {
                    let (ax, ay) = Game::adjust_position(&map, &setting, x as i64, y as i64);
                    if dsq(champ.x, champ.y, ax, ay) < 4000001 {
                        if let Some((gx, gy)) = act.target_position() {
                            let (gx2, gy2) = Game::adjust_position(&map, &setting, gx as i64, gy as i64);
                            if dsq(champ.x, champ.y, gx2, gy2) > 4000000 { out = Some(Input::Move { x: gx, y: gy }); }
                        }
                    }
                } }
                out
            })();
            let g = act.get_input(version, &mut rnd, player, &data, &ps, &mut dbg);
            let gb: [u64; 4] = unsafe { std::mem::transmute_copy(&g) };
            println!("141\tcase={} v={} champ=({},{})\tgame={:?}\tbytes={:x?}\tmine={:?}\t{}", case, version, champ.x, champ.y, g, gb, mine, if case == 65 { "OBSERVE" } else if format!("{:?}", g) == format!("{:?}", mine) { "MATCH" } else { "MISMATCH" });
        }
        // ───────── 138 / 139 ─────────
        70..=79 => {
            use game_ai::plan_legacy::sub_plan::{EpicPokeSubPlan, SerpenPokeSubPlan};
            let csp = |id: usize, team: usize, pos: usize| game_ai::ChampionScoreParameter {
                action: SmallAction::Stop, risk_possible: bumpalo::collections::Vec::new_in(&pool), gain_possible: bumpalo::collections::Vec::new_in(&pool),
                id, team, pos, applyed_damage: 0, applyed_cc: 0, risk_damage: 0, risk_epic_damage: 0, risk_cc: 0, risk_possible_tower: 0, action_time: 0,
                attack_value: 0, util_value: 0, attack_power: 0, util_power_base: 0, cc_time_x_inv_cd: 0, buff_inv_cd_count: 0 };
            let param = game_ai::ScoreParameter { wave_snapshot: None, player: csp(champ.id, 0, 0), positioning_score: Default::default(),
                near_allies: bumpalo::collections::Vec::new_in(&pool), near_enemies: bumpalo::collections::Vec::new_in(&pool), version: 2, v3_turnback_hold: false };
            let version = 2usize;
            let action = match case { 71 | 74 => game_ai::SmallActionPlay::RunAway(game_ai::SmallActionRunAway::new(&data, player, version)), 72 | 75 => game_ai::SmallActionPlay::Recall(game_ai::SmallActionRecall::new(&data, player, version)), _ => game_ai::SmallActionPlay::Stop };
            // 에픽/세르펜 생존 여부 관측
            if let GameMode::Moba(m) = game.get_game_mode() { println!("objectives\tepic_live={} serpen_live={}", m.jungle_runner.epic.live_list.len(), m.jungle_runner.serpen.live_list.len()); }
            let mut r1 = rand::rngs::StdRng::seed_from_u64(5); let mut r2 = r1.clone();
            let mut d1: DebugFrameData = Default::default(); let mut d2: DebugFrameData = Default::default();
            let s = game_ai::interaction_score(version, &mut r1, player, &data, &param, &action, &mut d1);
            let (name, g) = if case < 73 { let sp: EpicPokeSubPlan = Default::default(); ("138", sp.score(version, &param, &mut r2, player, &data, &action, &mut d2)) } else { let sp: SerpenPokeSubPlan = Default::default(); ("139", sp.score(version, &param, &mut r2, player, &data, &action, &mut d2)) };
            let mine = if case < 73 { if case == 70 { s } else { s / 2 } } else { 0 };   // epic None → 미가시 → RunAway/Recall 은 s/2 · serpen None → 0
            println!("{}\tcase={} action_idx={}\tinteraction_score={}\tgame={} mine={}\t{}", name, case, match case { 71 | 74 => 0, 72 | 75 => 1, _ => 16 }, s, g, mine, if g == mine { "MATCH" } else { "MISMATCH" });
        }
        // ───────── 134 ─────────
        80..=89 => {
            let ps: PositioningScoreData = Default::default();
            let td = game_ai::TowerDodgeContext::new_tower_avoid(player, &data, champ.x, champ.y);
            // 지역 정보 덤프
            let lane = map.lane_seq(LineType::Mid, 1);
            println!("mid_seq_team1\t{:?}", lane);
            let mut line_regions: Vec<usize> = (0..27).filter(|&i| map.is_line_region[i]).collect();
            println!("line_regions\t{:?}", line_regions);
            let mut n = 0; let mut bad = 0;
            let mut s: u64 = 0xabcdef;
            let mut nsoft = 0; let mut nallow = 0; let mut ndanger = 0;
            for i in 0..400 {
                let mut r = || { s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (s >> 33) as usize };
                let (sx, sy, nx, ny, tx, ty) = (r() % 30, r() % 30, r() % 30, r() % 30, r() % 30, r() % 30);
                let ol_i = match case { 80 => 0, 81 => 1, 82 => 2, _ => i % 3 };
                let ol = match ol_i { 0 => game_ai::AroundBushOutlineType::None, 1 => game_ai::AroundBushOutlineType::Outline, _ => game_ai::AroundBushOutlineType::Inline };
                let g: [i32; 2] = unsafe { std::mem::transmute(check_cell(2, player, &ps, &data, &td, ol, sx, sy, nx, ny, tx, ty)) };
                // 독립 재구현
                let sr = map.regions[sy][sx]; let er = map.regions[ty][tx]; let nr = map.regions[ny][nx];
                let is_line = |r: usize| map.is_line_region[r];
                let top = |cx: usize, cy: usize| { let x = cx as u64 * 32000 + 16000; let ry = (setting.height - 16000).wrapping_sub(cy as u64 * 32000); ry >= x };
                let soft = match ol_i {
                    0 => false,
                    1 => {
                        if top(sx, sy) == top(tx, ty) { !(nr == er || nr == sr || !is_line(nr)) }
                        else { if !is_line(nr) { false } else if lane.iter().any(|&q| q == nr) { false } else if nr == sr || nr == er { false } else { true } }
                    }
                    _ => !(nr == sr || is_line(nr) || nr == er),
                };
                let mine = if soft { 1 } else if game_ai::dodge_tower_cell_with_context(2, player, &data, &ps, &td, sx, sy, nx, ny) { 0 } else { 2 };
                match g[0] { 0 => nallow += 1, 1 => nsoft += 1, _ => ndanger += 1 }
                if g[0] != mine { bad += 1; }
                if n < 5 || g[0] != mine { println!("134\tol={} s=({},{}) n=({},{}) t=({},{}) regions s={} n={} e={}\tgame={} mine={}\t{}", ol_i, sx, sy, nx, ny, tx, ty, sr, nr, er, g[0], mine, if g[0] == mine { "MATCH" } else { "MISMATCH" }); }
                n += 1;
            }
            println!("134_summary\tcase={} n={} allow={} soft={} danger={} mismatch={}", case, n, nallow, nsoft, ndanger, bad);
        }
        _ => { println!("no case"); }
    }
}
