#![allow(unused, dead_code, non_snake_case)]
//! 23차 B · #146 SmallActionAroundBush::new_with_out_line (pub) — 직접 호출.
//! 표적: 71셀 표(@anon…94, IR 에서 디코드) 필터 → SliceRandom::choose(rng) 1회(gen_range::<u32>(0..len)) → 셀 중심 변환 ·
//!       start_tick/change_tick = game.tick() · path_finder None(태그 2 @+0x6d) · rng 소비량(clone 대조) · 후보 0 → print+패닉(exit≠0).
//! 한 프로세스 = 한 케이스(argv[1] = 후보 bush id 순번 · 100 = 존재하지 않는 bush id · 200+k = 시드 k).
use game_core::*;
use rand::{SeedableRng, Rng, RngCore};
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

const TABLE: [(usize, usize); 71] = [(0,0), (1,0), (2,0), (19,0), (20,0), (21,0), (0,1), (0,2), (7,4), (8,4), (9,4), (20,4), (20,5), (12,6), (4,7), (4,8), (29,8), (4,9), (14,9), (29,9), (29,10), (26,11), (6,12), (12,12), (13,12), (26,12), (12,13), (26,13), (9,14), (26,14), (20,15), (21,15), (17,16), (21,16), (16,17), (17,17), (25,18), (0,19), (25,19), (0,20), (4,20), (5,20), (15,20), (25,20), (0,21), (15,21), (16,21), (25,21), (25,22), (29,24), (18,25), (19,25), (20,25), (21,25), (22,25), (29,25), (11,26), (12,26), (13,26), (14,26), (29,26), (28,28), (29,28), (8,29), (9,29), (10,29), (24,29), (25,29), (26,29), (28,29), (29,29)];

fn rd<T: Copy>(p: *const u8, off: usize) -> T { unsafe { std::ptr::read_unaligned(p.add(off) as *const T) } }

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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(777);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Top).unwrap();

    // 표 셀에서 만나는 bush id 들
    let mut ids: Vec<usize> = TABLE.iter().map(|(x, y)| map.bushes[*y][*x]).collect();
    ids.sort(); ids.dedup();
    let all_ids: std::collections::BTreeSet<usize> = map.bushes.iter().flatten().copied().collect();
    println!("bush_ids_in_table\t{:?}\tall_ids_in_map={:?}\ttick={}", ids, all_ids.iter().take(40).collect::<Vec<_>>(), game.tick());
    let (bush, seed, out_line) = if case >= 200 { (ids[1], (case - 200) as u64, game_ai::AroundBushOutlineType::Outline) }
        else if case == 100 { (999_999usize, 1u64, game_ai::AroundBushOutlineType::Inline) }
        else { (ids[(case as usize) % ids.len()], 11u64, if case % 2 == 0 { game_ai::AroundBushOutlineType::None } else { game_ai::AroundBushOutlineType::Inline }) };
    let cand: Vec<(usize, usize)> = TABLE.iter().copied().filter(|(x, y)| map.bushes[*y][*x] == bush).collect();
    println!("bush={}\tcand_len={}\tcand={:?}", bush, cand.len(), cand);

    let mut rnd = rand::rngs::StdRng::seed_from_u64(seed);
    let mut rnd2 = rnd.clone();
    // 예측: choose = gen_range::<u32>(0..len)(len<2^32) → cand[idx]
    let pred = if cand.is_empty() { None } else { let i = rnd2.gen_range(0..cand.len() as u32) as usize; Some((i, cand[i])) };
    let r = game_ai::SmallActionAroundBush::new_with_out_line(&mut rnd, &data, player, bush, out_line);
    let p = &r as *const _ as *const u8;
    let (st, ct, b, tx, ty): (usize, usize, usize, u64, u64) = (rd(p, 0), rd(p, 8), rd(p, 0x10), rd(p, 0x18), rd(p, 0x20));
    let pf_tag: u8 = rd(p, 0x6d); let ol: u8 = rd(p, 0x70);
    let (px, py) = pred.map(|(_, c)| ((c.0 * 32000 + 16000) as u64, (c.1 * 32000 + 16000) as u64)).unwrap_or((0, 0));
    let rng_same = rnd.next_u64() == rnd2.next_u64();
    let m = st == 777 && ct == 777 && b == bush && tx == px && ty == py && pf_tag == 2 && ol == out_line as u8 && rng_same;
    println!("case{}\tgame: start={} change={} bush={} target=({},{}) pf_tag={} out_line={}\tpred: idx={:?} target=({},{})\trng_after_same={}\t{}",
             case, st, ct, b, tx, ty, pf_tag, ol, pred.map(|x| x.0), px, py, rng_same, if m { "MATCH" } else { "**MISMATCH**" });
    println!("dbg\t{:?}", r);
}
