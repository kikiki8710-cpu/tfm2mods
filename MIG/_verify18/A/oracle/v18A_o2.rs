#![allow(unused, dead_code, non_snake_case)]
//! 18차 배치A 오라클 2 — `#60 can_trace_without_tower`(pub) 를 **명세 독립 재구현**(수법 ⓓ)으로 대조한다.
//!  spec: 12방향 원주점 p = (x + dx*range/1000, y + dy*range/1000) (sdiv 내림) → Game::adjust_position →
//!        can_tower_focused(ctx, cache, player, px, py) 를 순서대로 평가, 하나라도 false 면 true 반환.
//!  콜리 둘(can_tower_focused / adjust_position)은 pub 이라 **그대로 부른다**(이 오라클은 #60 자신의 조립만 검증).
//!  입력: 중심 (x,y) 를 맵 격자 12×12 + 적 타워 좌표 8곳, range ∈ {0, 999, 1000, 25000, 60000, 130000, 400000}.
//!  덤: 원주 표(1000·866·500)와 순서·break 위치는 `any` 의미론이라 외연으로만 확인된다(표기 불가 — open[2]).
//! TLS: 본문에 LocalKey 없음. can_tower_focused 내부 TLS 는 미확인 → 단일 프로세스지만 케이스마다 world 를 안 바꾼다(순수 읽기).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify18/A/oracle/v18A_o2.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

const CIRCLE: [(i64, i64); 12] = [(0,1000),(500,866),(866,500),(1000,0),(866,-500),(500,-866),(0,-1000),(-500,-866),(-866,-500),(-1000,0),(-866,500),(-500,866)];

fn spec_can_trace(ctx: &GameContext, cache: &AbstractGameWithCache, map: &MapDef, setting: &GameSetting,
                  player: &PlayerState, x: i64, y: i64, range: i64) -> (bool, usize) {
    let mut evals = 0usize;
    for (dx, dy) in CIRCLE.iter() {
        let px = x + (dx * range) / 1000;
        let py = y + (dy * range) / 1000;
        let (ax, ay) = Game::adjust_position(map, setting, px, py);
        evals += 1;
        if !game_ai::can_tower_focused(ctx, cache, player, ax, ay) { return (true, evals); }
    }
    (false, evals)
}

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(1000);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    println!("towers\t{}\ttick={}", game.world.tower_ids.len(), game.tick());
    // 적 타워 좌표(팀0 관점 = 팀1 타워)
    let mut centers: Vec<(i64, i64, &str)> = Vec::new();
    for (nm, arr) in [("top_tower", &cache.top_tower), ("top_tower2", &cache.top_tower2), ("mid_tower", &cache.mid_tower),
                      ("mid_tower2", &cache.mid_tower2), ("bottom_tower", &cache.bottom_tower), ("bottom_tower2", &cache.bottom_tower2)] {
        for t in 0..2usize {
            if let Some(e) = arr[t] { centers.push((e.x as i64, e.y as i64, nm)); }
        }
    }
    for gx in 0..12i64 { for gy in 0..12i64 {
        centers.push((gx * 80000 + 40000, gy * 80000 + 40000, "grid"));
    } }
    let ranges = [0i64, 999, 1000, 25000, 60000, 130000, 400000];
    let mut n = 0usize; let mut mism = 0usize; let mut trues = 0usize;
    let mut by_range = vec![(0usize, 0usize); ranges.len()];
    for t in 0..2usize {
        let player = game.get_player_by_position(t, Position::Mid).expect("player");
        let id = player.info.id;
        for &(x, y, nm) in centers.iter() {
            for (ri, &r) in ranges.iter().enumerate() {
                let actual = game_ai::can_trace_without_tower(&ctx, &cache, id, x as u64, y as u64, r as u64);
                let (pred, evals) = spec_can_trace(&ctx, &cache, &map, &setting, player, x, y, r);
                n += 1; by_range[ri].0 += 1;
                if actual { trues += 1; by_range[ri].1 += 1; }
                if actual != pred {
                    mism += 1;
                    if mism <= 10 { println!("MISM\tteam={}\t{}\t({},{})\trange={}\tactual={}\tpred={}", t, nm, x, y, r, actual, pred); }
                }
            }
        }
    }
    println!("cases\t{}\tmism\t{}\ttrue\t{}\tfalse\t{}", n, mism, trues, n - trues);
    for (ri, &r) in ranges.iter().enumerate() {
        println!("range\t{}\tcases={}\ttrue={}", r, by_range[ri].0, by_range[ri].1);
    }
    // 판별 단서: 적 타워 정중앙, range 0 → 12점 전부 타워 밑 → false 여야 한다
    let p0 = game.get_player_by_position(0, Position::Mid).expect("p");
    if let Some(e) = cache.mid_tower[1] {
        println!("anchor\tenemy mid_tower ({},{}) range=0 → {}\trange=400000 → {}", e.x, e.y,
                 game_ai::can_trace_without_tower(&ctx, &cache, p0.info.id, e.x, e.y, 0),
                 game_ai::can_trace_without_tower(&ctx, &cache, p0.info.id, e.x, e.y, 400000));
    }
}
