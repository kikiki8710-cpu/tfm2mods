#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #142 path_needs_tower_escape (define hidden m11.ll:52695) — `#[link_name]` 직접 진입.
//! 표적: L139 극성(명세 「이미 그 위치면 → false」 vs IR phi `[ true, %35 ]`) · 45/66 경계 · 마지막 피격자 게이트 ·
//!       타워 nearest_enemy 게이트 · 웨이포인트 도달 반경(move_speed*10)² · index 클램프.
//! 한 프로세스 = 한 케이스(argv[1]). `game` 값과 두 예측(spec / IR)을 같이 찍는다.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12small_action23path_needs_tower_escape"]
    fn pnte(version: usize, player: &PlayerState, data: &OperationData, pf: &game_ai::PathFinder) -> bool;
}

fn w64(p: *mut Entity, off: usize, v: u64) { unsafe { std::ptr::write_volatile((p as *mut u8).add(off) as *mut u64, v) } }
fn r64(p: *const Entity, off: usize) -> u64 { unsafe { std::ptr::read_volatile((p as *const u8).add(off) as *const u64) } }

fn dist_sq(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 { let dx = x1.abs_diff(x2); let dy = y1.abs_diff(y2); dx * dx + dy * dy }

/// 명세 logic 재구현. `l139_true` = IR 판(이미 그 위치면 true) / false = 명세 판(→ false)
fn mine(l139_true: bool, path_len: usize, index: usize, path: &[(u64, u64)], hp: u64, max_hp: u64, ms: u64,
        cx: u64, cy: u64, tower_targets: bool, recent_from_enemy_tower: Option<bool>,
        unnec: &dyn Fn(u64, u64) -> bool) -> bool {
    if path_len == 0 { return false; }
    let hp_ratio = hp * 100 / max_hp.max(1);
    if hp_ratio > 45 {
        if !tower_targets {
            let recent = hp_ratio < 66 && recent_from_enemy_tower == Some(true);
            if !recent { return false; }
        }
    }
    if unnec(cx, cy) { return l139_true; }
    let idx = index.min(path_len - 1);
    let (mut x, mut y) = path[idx];
    if dist_sq(x, y, cx, cy) < (ms * 10) * (ms * 10) {
        let ni = (idx + 1).min(path_len - 1);
        let (nx, ny) = path[ni]; x = nx; y = ny;
    }
    unnec(x, y)
}

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    let V = 55usize;

    let champ = cache.player_champion[0][0].unwrap();      // team0 Top
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let cp = champ as *const Entity as *mut Entity;
    let unnec = |x: u64, y: u64| game_ai::is_unnecessary_enemy_tower_position(V, player, &data, x, y);

    // 「불필요한 적 타워 위치」 후보 스캔 (30x30 셀 중심)
    let mut p_true: Option<(u64, u64)> = None; let mut n_true = 0;
    for cy in 0..30u64 { for cx in 0..30u64 {
        let (x, y) = (cx * 32000 + 16000, cy * 32000 + 16000);
        if unnec(x, y) { n_true += 1; if p_true.is_none() { p_true = Some((x, y)); } }
    } }
    let (cx0, cy0) = (champ.x, champ.y);
    println!("scan\tunnec_true_cells={}\tfirst={:?}\tchamp0=({},{})\tunnec(champ0)={}", n_true, p_true, cx0, cy0, unnec(cx0, cy0));
    let p_true = match p_true { Some(p) => p, None => { println!("재료 부재: unnec=true 셀 없음"); return; } };
    let p_false = (cx0, cy0);
    assert!(!unnec(p_false.0, p_false.1), "champ0 위치가 unnec=true 라 기준점으로 못 씀");

    // 적/아군 타워·적 챔피언 id
    let mut enemy_tower: Option<usize> = None; let mut ally_tower: Option<usize> = None;
    for id in game.world.tower_ids.iter() {
        if let Some(e) = game.get_entity_by_id(*id) {
            match e.team { TeamType::Player(1) => { if enemy_tower.is_none() { enemy_tower = Some(*id) } }
                           TeamType::Player(0) => { if ally_tower.is_none() { ally_tower = Some(*id) } } _ => {} }
        }
    }
    let enemy_champ_id = cache.player_champion[1][0].unwrap().id;
    println!("ids\tchamp={}\tenemy_tower={:?}\tally_tower={:?}\tenemy_champ={}", champ.id, enemy_tower, ally_tower, enemy_champ_id);

    // 기본 세팅: max_hp 100 · move_speed 1000 (반경 10000)
    w64(cp, 0x628, 100); w64(cp, 0x640, 1000);
    let near = (cx0 + 3000, cy0 + 4000);   // dist² = 25e6 < 1e8
    let far  = (cx0 + 300000, cy0);        // 멀리(unnec 여부는 별개)
    // (hp, champ_pos, path, index, last_attacked_from, tower_targets)
    let (hp, cpos, path, index, laf, tt): (u64, (u64,u64), Vec<(u64,u64)>, usize, Option<usize>, bool) = match case {
        0  => (40, p_false, vec![], 0, None, false),                       // path_len 0 → false
        1  => (40, p_false, vec![p_true], 0, None, false),                 // hp≤45 · 다음점=unnec → true
        2  => (40, p_false, vec![far], 0, None, false),                    // 다음점 unnec 아님 → false (far 가 unnec 이면 스캔 로그로 판단)
        3  => (40, p_true,  vec![far], 0, None, false),                    // ★이미 unnec 위치: 명세 false / IR true
        4  => (80, p_false, vec![p_true], 0, None, false),                 // hp>45 · 타워표적 X · hp≥66 → false
        5  => (60, p_false, vec![p_true], 0, None, false),                 // 45<hp<66 · 피격자 None → false
        6  => (60, p_false, vec![p_true], 0, enemy_tower, false),          // 피격자 = 적 타워 → 통과 → true
        7  => (60, p_false, vec![p_true], 0, ally_tower, false),           // 아군 타워 → false
        8  => (60, p_false, vec![p_true], 0, Some(enemy_champ_id), false), // 적 챔피언 → false
        9  => (45, p_false, vec![p_true], 0, None, false),                 // 경계 45: 게이트 없음 → true
        10 => (46, p_false, vec![p_true], 0, None, false),                 // 경계 46: 게이트 → false
        11 => (65, p_false, vec![p_true], 0, enemy_tower, false),          // 경계 65 <66 → true
        12 => (66, p_false, vec![p_true], 0, enemy_tower, false),          // 경계 66 → false
        13 => (40, p_false, vec![near, p_true], 0, None, false),           // 도달반경 안 → 다음점 p_true → true
        14 => (40, p_false, vec![near, far], 0, None, false),              // 도달반경 안 → 다음점 far → false
        15 => (40, p_false, vec![near], 0, None, false),                   // path_len 1 · 도달 → next=0 → near(unnec?)
        16 => (40, p_false, vec![far, p_true], 7, None, false),            // index 클램프 → path[1]=p_true → true
        17 => (80, p_false, vec![p_true], 0, None, true),                  // 타워가 champ 표적 → 게이트 통과 → true
        18 => (80, p_true,  vec![far], 0, None, true),                     // 타워 표적 + 이미 unnec 위치 → IR true
        19 => (40, p_false, vec![p_true; 3], 2, None, false),              // index=len-1 → next=len-1
        _  => (40, p_false, vec![p_true], 0, None, false),
    };
    w64(cp, 0x670, hp); w64(cp, 0x660, cpos.0); w64(cp, 0x668, cpos.1);
    match laf { Some(id) => { w64(cp, 0x28, 1); w64(cp, 0x30, id as u64); } None => { w64(cp, 0x28, 0); } }
    if tt {
        let tid = enemy_tower.unwrap();
        let te = game.get_entity_by_id(tid).unwrap() as *const Entity as *mut Entity;
        w64(te, 0x88, 1); w64(te, 0x90, 0); w64(te, 0x98, champ.id as u64);   // Tower.info.nearest_enemy = Some((0, champ.id))
    }
    let mut pf = game_ai::PathFinder::new_target_no_check("o142", 0, 0);
    pf.path_len = path.len(); pf.index = index;
    for (i, p) in path.iter().enumerate() { pf.path[i] = *p; }
    println!("pf\tpath_len={}\tindex={}\tpath0={:?}\tunnec(path)={:?}", pf.path_len, pf.index, path.get(0),
             path.iter().map(|p| unnec(p.0, p.1)).collect::<Vec<_>>());
    let recent = laf.map(|id| game.get_entity_by_id(id).map(|e| e.team == TeamType::Player(1) && e.ty.is_tower()).unwrap_or(false));
    let m_spec = mine(false, path.len(), index, &path, hp, 100, 1000, cpos.0, cpos.1, tt, recent, &unnec);
    let m_ir   = mine(true,  path.len(), index, &path, hp, 100, 1000, cpos.0, cpos.1, tt, recent, &unnec);
    let g = unsafe { pnte(V, player, &data, &pf) };
    println!("case{}\thp={}\tchamp_at_unnec={}\tgame={}\tmine_spec={}\tmine_ir={}\t{}", case, hp, unnec(cpos.0, cpos.1), g, m_spec, m_ir,
             if g == m_ir && g == m_spec { "MATCH" } else if g == m_ir { "MATCH_IR(spec 반전)" } else if g == m_spec { "MATCH_SPEC(IR 반전)" } else { "**MISMATCH**" });
}
