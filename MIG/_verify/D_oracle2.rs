#![allow(unused, dead_code, non_snake_case)]
// D 배치 2단계 — &PlayerState 까지 만들어 담당 pub 함수(best_jungle_goal / is_cleared /
// engage_requires_dive / max_range_nearly_can_use)를 실제로 실행할 수 있는가?
use game_core::*;
use game_ai::plan_legacy::old as old;
use rand::SeedableRng;
use std::sync::Arc;

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

    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);

    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let st: AthleteStat = Default::default();
            let gp = GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new());
            game.add_player(gp);
            pid += 1;
        }
    }
    game.start_game(&mut rnd, &ctx);

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    for t in 0..2usize {
        for p in 0..5usize {
            let ps = game.get_player_by_position(t, poss[p]);
            println!("player\t{}\t{}\t{}", t, p, ps.is_some());
        }
    }
    println!("champ0\t{}", cache.player_champion[0][0].is_some());
}
