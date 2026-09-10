#![allow(unused, dead_code, non_snake_case)]
// 2차 반증검증 배치 A — 오라클 2단계: 내 5개 판단함수를 **직접 실행**한다.
//  ★검증 포인트: BRIEF/METHOD_MAP 이 "buff_value 는 pub(crate) 라 오라클 불가" 라고 적었는데
//    game_ai 크레이트 루트에 `game_ai::defensive_crisis` 로 재수출돼 있다 → 정말 불릴까?
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;                        // ★
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];

    let tuts = [TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
                TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
                TutorialType::Line, TutorialType::Total];

    for (ti, tut) in tuts.iter().enumerate() {
        let ctx = GameContext {
            pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: *tut, trace_level: TraceLevel::Off,
        };
        println!("morgard\ttut={:?}\tmorgard_exists={}\tline_exists(Top,Mid,Bot)={},{},{}",
                 tut, game_ai::plan_legacy::rule_scope::morgard_exists(&ctx),
                 game_ai::plan_legacy::rule_scope::line_exists(&ctx, LineType::Top),
                 game_ai::plan_legacy::rule_scope::line_exists(&ctx, LineType::Mid),
                 game_ai::plan_legacy::rule_scope::line_exists(&ctx, LineType::Bottom));
    }

    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tp: TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();

    // ---- 캐시 상태 ----
    for t in 0..2usize {
        println!("cache\tteam={}\ttwin_towers.len={}", t, cache.twin_towers[t].len());
    }
    let g: &dyn AbstractGame = &game;
    match g.get_game_mode().as_moba() {
        Some(m) => println!("moba\tremain_epic_time(0)={}\t(1)={}", m.remain_epic_time(0), m.remain_epic_time(1)),
        None => println!("moba\tNone"),
    }

    // ---- 03 defensive_crisis (★pub(crate) 라던 것) ----
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let me = cache.player_champion[t][p].unwrap();
        for v in [1usize, 3] {
            let r = game_ai::defensive_crisis(v, &mut rnd, ps, &data, me, &mut dbg);
            println!("dcrisis\tt{}p{}\tver={}\tdie_imminent={}\tcc_threat={}", t, p, v, r.die_imminent, r.cc_threat);
        }
    }}

    // ---- 04 handle_line_defense ----
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let mut o = Vec::new();
        for l in [LineType::Top, LineType::Mid, LineType::Bottom] {
            o.push(format!("{:?}={}", l, old::handle_line_defense(3, &mut rnd, ps, &data, l, &mut dbg)));
        }
        println!("hld\tt{}p{}\t{}", t, p, o.join(" "));
    }}

    // ---- 02 AttackNexusPlan::sub_plan ----
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let me = cache.player_champion[t][p].unwrap();
        let f = map.fountain(t);
        let inarea = me.x >= f.0 && me.x <= f.2 && !(me.y < f.1 || me.y > f.3);
        let mut o = Vec::new();
        for l in [LineType::Top, LineType::Mid, LineType::Bottom] {
            let plan = old::AttackNexusPlan::new(1 - t, l);
            let sp = plan.sub_plan(3, &mut rnd, ps, &data, &tp, &mut dbg);
            o.push(format!("{:?}", sp));
        }
        println!("anp\tt{}p{}\tx={} y={}\tin_fountain={}\thp={} max={}\t{}",
                 t, p, me.x, me.y, inarea, me.hp, me.stat_cached.hp, o.join(" | "));
    }}

    // ---- 00 ult ----
    let psd: PositioningScoreData = Default::default();
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let me = cache.player_champion[t][p].unwrap();
        let foe = cache.player_champion[1 - t][p].unwrap();
        let r = game_ai::ult(3, &mut rnd, ps, &data, &psd, foe);
        println!("ult\tt{}p{}\tlevel={}\tcan_ult={}\t{:?}", t, p, me.level, me.can_ult(), r);
    }}
}
