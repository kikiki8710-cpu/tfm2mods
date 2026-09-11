#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 C — probe E
//  10 open[5]: dyn AbstractGame vtable 슬롯 0x28(tick) / 0x40(get_game_mode) / 0x1f0(get_entity_by_id) 를
//              실제 vtable 을 읽어 함수 주소와 대조 (Game 구현체 · ExpectedGame 은 별도)
//  14 open[1]: MapDef.bushes 값 사전 (부시 인덱스 → 셀/중심좌표)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60; setting.width = 960000; setting.height = 960000; setting.champion_radius = 10000;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);

    // ---- vtable 슬롯 대조 ----
    {
        let g: &dyn AbstractGame = &game;
        let raw: [usize; 2] = unsafe { std::mem::transmute(g) };
        let vt = raw[1] as *const usize;
        let cands: Vec<(&str, usize)> = vec![
            ("tick", <Game as AbstractGame>::tick as usize),
            ("get_game_mode", <Game as AbstractGame>::get_game_mode as usize),
            ("get_entity_by_id", <Game as AbstractGame>::get_entity_by_id as usize),
            ("strategy", <Game as AbstractGame>::strategy as usize),
        ];
        for i in 0..102usize {
            let slot = unsafe { *vt.add(i) };
            for (n, a) in cands.iter() {
                if slot == *a {
                    println!("vtslot\tidx={}\toff=0x{:x}\tname={}\taddr=0x{:x}", i, i * 8, n, a);
                }
            }
        }
        // 슬롯 0x28/0x40/0x1f0 이 무엇인지 역방향으로도 찍는다
        for &off in [0x28usize, 0x40, 0x1f0].iter() {
            let slot = unsafe { *vt.add(off / 8) };
            let nm = cands.iter().find(|(_, a)| *a == slot).map(|(n, _)| *n).unwrap_or("(unknown)");
            println!("vtoff\toff=0x{:x}\tidx={}\tslot=0x{:x}\tmatch={}", off, off / 8, slot, nm);
        }
    }

    // ---- bushes 값 사전 ----
    {
        let mut cells: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 40];
        for cy in 0..30usize { for cx in 0..30usize {
            let b = map.bushes[cy][cx];
            if b < 40 { cells[b].push((cy, cx)); }
        }}
        for b in 0..40usize {
            if cells[b].is_empty() { continue; }
            let n = cells[b].len();
            let sx: usize = cells[b].iter().map(|(_, x)| *x).sum();
            let sy: usize = cells[b].iter().map(|(y, _)| *y).sum();
            let cx = (sx as f64 / n as f64) * 32000.0 + 16000.0;
            let cy = (sy as f64 / n as f64) * 32000.0 + 16000.0;
            println!("bush\tid={}\tncell={}\tcenter=({:.0},{:.0})\tis_top_side={}\tcells={:?}",
                b, n, cx, cy, map_regions::is_top_side(&ctx, cx as u64, cy as u64), cells[b]);
        }
    }
}
