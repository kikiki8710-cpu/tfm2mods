#![allow(unused, dead_code, non_snake_case)]
// D 배치 타당성 프로브 — `&OperationData` 를 오라클에서 **정상 생성**할 수 있는가?
// (METHOD_MAP §1 ⑥ 한계2 / IR_TOOLKIT §7 의 "OperationData 는 pub 생성자가 없어 구성 불가" 를 검증)
use game_core::*;
use game_ai::plan_legacy::old as old;
use rand::SeedableRng;

fn main() {
    let pool = bumpalo::Bump::new();
    let setting: GameSetting = Default::default();
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };

    let game = Game::new(1234u64, false, &setting, &ms, &map);
    let g: &dyn AbstractGame = &game;
    let cache = AbstractGameWithCache::new(g, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    println!("OK\tOperationData 생성 성공\ttick={}", g.tick());
    for t in 0..2usize {
        for p in 0..5usize {
            let ps = g.get_player_by_position(t, Position::from_index(p));
            println!("player\tteam={}\tpos={}\t{}", t, p, ps.is_some());
        }
    }
}
