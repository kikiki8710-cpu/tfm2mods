#![allow(unused, dead_code, non_snake_case)]
// #10 unknown[4] 닫기 — PlayerState::strategy 가 RNG 상태를 전진시키는가(= 이 함수에 부작용이 있는가)
use game_core::*;
use rand::{SeedableRng, Rng};
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
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
    let dynr: &dyn AbstractGame = &game;

    for p in 0..5usize {
        let ps = game.get_player_by_position(0, poss[p]).unwrap();
        // 기준: strategy 를 부르지 않고 뽑은 난수열
        let mut a = rand::rngs::StdRng::seed_from_u64(42);
        let base: Vec<u64> = (0..4).map(|_| a.gen::<u64>()).collect();
        // 비교: strategy 를 한 번 부른 뒤 뽑은 난수열
        let mut b = rand::rngs::StdRng::seed_from_u64(42);
        let s1 = ps.strategy(&mut b, dynr);
        let after: Vec<u64> = (0..4).map(|_| b.gen::<u64>()).collect();
        // strategy 를 두 번 부르면 결과가 달라지나
        let mut c = rand::rngs::StdRng::seed_from_u64(42);
        let s2 = c.gen::<u64>();
        let s3 = ps.strategy(&mut c, dynr);
        println!("strategy\tpos={:?}\trng_advanced={}\tsame_result_diff_rng={}\ts={:?}",
            poss[p], base != after, format!("{:?}", s1) == format!("{:?}", s3), s1);
        // 24B 원본 바이트 (object_finish 는 +0xf)
        let raw = unsafe { std::slice::from_raw_parts(&s1 as *const Strategy as *const u8, std::mem::size_of::<Strategy>()) };
        println!("strategy_raw\tpos={:?}\tsize={}\tbytes={}\tobject_finish@0xf={}",
            poss[p], std::mem::size_of::<Strategy>(),
            raw.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(""), raw[0xf]);
    }
}
