#![allow(unused, dead_code, non_snake_case)]
// towerchk — ★`start_game` 뒤 `init_tower`/`init_nexus` 를 또 부르면 타워가 2배가 되는가?
//
// 왜 확인하나: 2026-09-11 2차 배치 A 가 "재호출하면 타워 16→32, twin_towers 2→4" 라고 경고했는데,
// **그 경고를 쓴 배치 A 자신의 템플릿(A2_oracle3.rs)도 재호출 형태**였다. 배치 D 도 같다.
// 그렇다면 배치 A 의 02(3분기)·배치 D 의 15(팀당 10개 = 6+4, "twin 2쌍 좌표 중복") 실측이
// 자기 설정 버그를 게임 사실로 읽은 것일 수 있다. 넘기기 전에 실제로 갈라 본다.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext,
          reinit: bool) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, ctx);
    if reinit {
        game.init_tower(ctx);
        game.init_nexus(setting, map);
    }
    game
}

fn main() {
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
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

    println!("mode\ttower_ids\tnexus_ids\ttwin0\ttwin1\ttowers0\ttowers1\tcoord_dupe0");
    for (label, reinit) in [("start_game만", false), ("+init_tower/nexus", true)] {
        let game = mkgame(&setting, &ms, &map, &ctx, reinit);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let n_tower = game.world.tower_ids.len();
        let n_nexus = 0usize;   // nexus_ids 필드명 미확인 — 타워만 본다
        let tw0 = cache.twin_towers[0].len();
        let tw1 = cache.twin_towers[1].len();
        // iter_towers_without_nexus 는 pub 이 아닐 수 있으니 cache 필드로 직접 센다
        let mut c0 = 0usize; let mut c1 = 0usize;
        let mut xy0: Vec<(u64, u64)> = Vec::new();
        for id in game.world.tower_ids.iter() {
            if let Some(e) = game.world.entity.get(*id) {
                match e.team {
                    TeamType::Player(0) => { c0 += 1; xy0.push((e.x, e.y)); }
                    TeamType::Player(1) => { c1 += 1; }
                    _ => {}
                }
            }
        }
        let uniq = { let mut v = xy0.clone(); v.sort(); v.dedup(); v.len() };
        let dupe = xy0.len() - uniq;
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                 label, n_tower, n_nexus, tw0, tw1, c0, c1, dupe);
    }
}
