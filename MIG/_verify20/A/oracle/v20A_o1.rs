#![allow(unused, dead_code, non_snake_case)]
//! 20차 배치A 오라클 1 — `#103 position_risk_all_zero_near`(pub) 의 **격자·적 우물 게이트**를 실행으로 확인한다.
//!  세계 = TEMPLATE mkgame(start_game 직후, tick 0). 이 상태에서는 적 챔프(≤200000 없음)·타워(≤150000 없음)·
//!  미니언(미스폰)·투사체(없음)·정글(focused 없음) 이 전부 스킵되므로 반환값 = !(7x7 셀 영역 ∩ 적 우물 피해 rect).
//!  ⟹ consts[1..9](3·29·32000·16000·64001·960001·799999·160001·895999) 와 logic 1275~1284 의 실행 검증.
//!  ⚠TLS: EPC_CACHE(EntityPositioningCache) 는 적 챔프 루프에서만 닿는다 — 이 스윕은 그 루프에 진입하지 않는다.
//!  케이스 = (team, position, cx, cy) 30x30 스윕 × 2팀 = 1800 (같은 프로세스 · TLS 미접촉).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify20/A/oracle/v20A_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn wr<T: Copy>(base: *mut u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut T, v) }
}

// 명세 logic 1275~1284 + consts 의 독립 재구현(예측)
fn predict(team: usize, cx: i64, cy: i64) -> bool {
    let min_xi = (cx - 3).max(0); let max_xi = (cx + 3).min(29);
    let min_yi = (cy - 3).max(0); let max_yi = (cy + 3).min(29);
    let lx = min_xi * 32000 + 16000; let rx = max_xi * 32000 + 16000;
    let ly = min_yi * 32000 + 16000; let ry = max_yi * 32000 + 16000;
    let ri = |lx1: i64, rx1: i64, ly1: i64, ry1: i64| lx <= rx1 && lx1 <= rx && ly <= ry1 && ly1 <= ry;
    let hit = if team == 1 {
        ri(0, 64000, 800000, 960000) || ri(0, 160000, 896000, 960000)
    } else {
        ri(800000, 960000, 0, 64000) || ri(896000, 960000, 0, 160000)
    };
    !hit
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    println!("towers\t{}\ttick\t{}", game.world.tower_ids.len(), game.tick());

    let mut ps: PositioningScoreData = unsafe { std::mem::zeroed() };
    let psp = &mut ps as *mut PositioningScoreData as *mut u8;
    let mut total = 0usize; let mut mism = 0usize; let mut falses = 0usize;
    println!("team\tpos\tcx\tcy\tgame\tmine\tok");
    for t in 0..2usize {
        for pos in [Position::Top, Position::Support] {
            let player = game.get_player_by_position(t, pos).expect("player");
            let champ = cache.player_champion[t][pos.as_index()].expect("champ");
            println!("# team {} pos {:?} champ ({}, {}) id {}", t, pos, champ.x, champ.y, champ.id);
            for cx in 0..30i64 { for cy in 0..30i64 {
                wr::<usize>(psp, 0xab8, cx as usize);
                wr::<usize>(psp, 0xac0, cy as usize);
                let g = game_ai::position_risk_all_zero_near(2, player, &data, &ps, game_ai::PositionEvalPurpose::RunAway);
                let m = predict(t, cx, cy);
                total += 1; if g != m { mism += 1; } if !g { falses += 1; }
                if g != m || !g {
                    println!("{}\t{:?}\t{}\t{}\t{}\t{}\t{}", t, pos, cx, cy, g, m, if g == m { "MATCH" } else { "MISMATCH" });
                }
            }}
        }
    }
    println!("summary\ttotal={}\tmismatch={}\tfalse_cases={}\tsetting_ok={}", total, mism, falses, ok);
}
