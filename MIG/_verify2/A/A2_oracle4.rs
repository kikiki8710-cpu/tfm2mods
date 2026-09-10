#![allow(unused, dead_code, non_snake_case)]
// 배치 A 오라클 4 — 04 handle_line_defense 가 왜 항상 false 인지 항별로 분해한다.
//  ⚠오라클3 에서 start_game 뒤에 init_tower/init_nexus 를 또 불러 타워가 2배로 생겼다 → 그것도 함께 확인.
use game_core::*;
use game_ai::plan_legacy::old as old;
use rand::SeedableRng;
use std::sync::Arc;

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext, extra_init: bool) -> Game {
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
    if extra_init { game.init_tower(ctx); game.init_nexus(setting, map); }
    game
}

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
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
    for extra in [false, true] {
        let g = mkgame(&setting, &ms, &map, &ctx, extra);
        println!("towers\textra_init={}\tcount={}", extra, g.world.tower_ids.len());
    }

    for n in [0usize, 1, 2, 3] {
        let mut g = mkgame(&setting, &ms, &map, &ctx, false);
        g.mode.epic_minion_buff_time = [600, 600];
        // 아군(팀0) Top 타워 좌표
        let mut txy = None;
        for id in g.world.tower_ids.iter().cloned().collect::<Vec<_>>() {
            if let Some(e) = g.world.entity.get(id) {
                if matches!(e.team, TeamType::Player(0)) && txy.is_none() { txy = Some((id, e.x, e.y)); }
            }
        }
        let (tid, tx, ty) = txy.unwrap();
        let eids: Vec<usize> = g.world.champion_ids.iter().cloned()
            .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
        for (k, id) in eids.iter().enumerate() {
            if k < n { if let Some(e) = g.world.entity.get_mut(*id) { e.x = tx + 1000 * (k as u64); e.y = ty; } }
        }
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        for b in bb.iter_mut() {
            b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5];
            b.last_reveal_hidden_span = [0; 5];
        }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let gd: &dyn AbstractGame = &g;
        // (1) 항별 분해
        let l = LineType::Top;
        let le = game_ai::plan_legacy::rule_scope::line_exists(&ctx, l);
        let me = game_ai::plan_legacy::rule_scope::morgard_exists(&ctx);
        let ep = g.mode.remain_epic_time(1);
        let thr = old::has_line_defense_threat(ps, &data, l, tid);
        // (2) is_recent_visible 을 직접
        let mut vis = 0;
        let chs = cache.champions(1, &pool);
        let mut near = 0;
        for e in chs.iter() {
            let v = bb[1].is_recent_visible(gd, ps, e);
            if v { vis += 1; }
            let dx = if e.x > tx { e.x - tx } else { tx - e.x };
            let dy = if e.y > ty { e.y - ty } else { ty - e.y };
            if v && dx * dx + dy * dy < 40000000001u64 { near += 1; }
        }
        let s = ps.strategy(&mut rnd, gd);
        let r = old::handle_line_defense(3, &mut rnd, ps, &data, l, &mut dbg);
        println!("split\tn={}\tline_exists={}\tmorgard={}\tepic1={}\tthreat={}\tchamps={}\tvisible={}\tnear={}\tmorgard_def={:?}\thld={}",
                 n, le, me, ep, thr, chs.len(), vis, near, s.morgard_defense, r);
    }
}
