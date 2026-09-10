#![allow(unused, dead_code, non_snake_case)]
// 배치 A 오라클 3 — 게임 상태를 손으로 흔들어 판정 분기를 실제로 태운다.
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
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
    game.init_tower(ctx);
    game.init_nexus(setting, map);
    game
}

fn is_t1(g: &Game, id: usize) -> bool {
    matches!(g.world.entity.get(id).map(|e| e.team), Some(TeamType::Player(1)))
}
fn is_t0(g: &Game, id: usize) -> bool {
    matches!(g.world.entity.get(id).map(|e| e.team), Some(TeamType::Player(0)))
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
    let game0 = mkgame(&setting, &ms, &map, &ctx);
    let tids: Vec<usize> = game0.world.tower_ids.iter().cloned().collect();
    let mut tower_xy = (0u64, 0u64);
    for id in tids.iter() {
        if let Some(e) = game0.world.entity.get(*id) {
            println!("tower\tid={}\tteam={:?}\tx={}\ty={}", id, e.team, e.x, e.y);
            if tower_xy == (0, 0) {
                if let TeamType::Player(0) = e.team { tower_xy = (e.x, e.y); }
            }
        }
    }
    println!("tower0\t{:?}", tower_xy);

    // ---- 실험 A: 적 챔프 N 명을 아군 타워 반경 안에 두고 handle_line_defense ----
    for n in 0..=5usize {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        g.mode.epic_minion_buff_time = [600, 600];
        let eids: Vec<usize> = g.world.champion_ids.iter().cloned().filter(|id| is_t1(&g, *id)).collect();
        for (k, id) in eids.iter().enumerate() {
            if k < n {
                if let Some(e) = g.world.entity.get_mut(*id) { e.x = tower_xy.0 + 1000 * (k as u64); e.y = tower_xy.1; }
            }
        }
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        for b in bb.iter_mut() { b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5]; }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let s = ps.strategy(&mut rnd, &g as &dyn AbstractGame);
        let r = old::handle_line_defense(3, &mut rnd, ps, &data, LineType::Top, &mut dbg);
        println!("hldN\tnear={}\tepic1={}\tmorgard={:?}\thld={}", n, g.mode.remain_epic_time(1), s.morgard_defense, r);
    }

    // ---- 실험 B: defensive_crisis + check_kill_die_tick, tps 를 바꿔 임계 확인 ----
    for tps in [60usize, 6, 1] {
        let mut st2: GameSetting = Default::default();
        st2.tick_per_second = tps;
        let map2 = MapDef::moba(&st2);
        let ctx2 = GameContext {
            pool: &pool, setting: &st2, macro_weights: &mw, map_setting: &ms,
            map: &map2, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off,
        };
        let mut g = mkgame(&st2, &ms, &map2, &ctx2);
        let myid = g.world.champion_ids.iter().cloned().find(|id| is_t0(&g, *id)).unwrap();
        let me_xy = { let e = g.world.entity.get(myid).unwrap(); (e.x, e.y) };
        let eids: Vec<usize> = g.world.champion_ids.iter().cloned().filter(|id| is_t1(&g, *id)).collect();
        for id in eids.iter() {
            if let Some(e) = g.world.entity.get_mut(*id) { e.x = me_xy.0 + 1000; e.y = me_xy.1; e.level = 6; }
        }
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        for b in bb.iter_mut() { b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5]; }
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx2);
        let data = OperationData::new(&cache, &ctx2, &bb);
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let me = cache.player_champion[0][0].unwrap();
        let foe = cache.player_champion[1][0].unwrap();
        let mr = game_ai::max_range(foe, me);
        let dsq = { let dx = if foe.x > me.x { foe.x - me.x } else { me.x - foe.x };
                    let dy = if foe.y > me.y { foe.y - me.y } else { me.y - foe.y }; dx * dx + dy * dy };
        let ev = bumpalo::collections::Vec::new_in(&pool);
        let tv = bumpalo::collections::Vec::new_in(&pool);
        let die = game_ai::check_kill_die_tick(3, &mut rnd, &data, ps, me, ev, tv, &mut dbg);
        let r = game_ai::defensive_crisis(3, &mut rnd, ps, &data, me, &mut dbg);
        println!("dcrisisB\ttps={}\tmax_range={}\tdsq={}\tgate={}\tdie={}\tdie<2tps={}\tdie_imm={}\tcc={}",
                 tps, mr, dsq, dsq <= (mr + 30000) * (mr + 30000), die, die < tps * 2, r.die_imminent, r.cc_threat);
    }

    // ---- 실험 C: 02 sub_plan 의 Recall 분기 ----
    {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        let id = g.world.champion_ids.iter().cloned().find(|i| is_t0(&g, *i)).unwrap();
        if let Some(e) = g.world.entity.get_mut(id) { e.stat_cached.hp = 999; e.hp = 500; }
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let tp: TeamPlan = Default::default();
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let plan = old::AttackNexusPlan::new(1, LineType::Mid);
        let e0 = cache.player_champion[0][0].unwrap();
        println!("anpC\tin_fountain\tx={} y={} hp={}/{}\t{:?}", e0.x, e0.y, e0.hp, e0.stat_cached.hp,
                 plan.sub_plan(3, &mut rnd, ps, &data, &tp, &mut dbg));
    }
    {
        let mut g2 = mkgame(&setting, &ms, &map, &ctx);
        let id = g2.world.champion_ids.iter().cloned().find(|i| is_t0(&g2, *i)).unwrap();
        if let Some(e) = g2.world.entity.get_mut(id) { e.stat_cached.hp = 999; e.hp = 500; e.x = 500000; e.y = 500000; }
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let cache2 = AbstractGameWithCache::new(&g2 as &dyn AbstractGame, &ctx);
        let data2 = OperationData::new(&cache2, &ctx, &bb);
        let tp: TeamPlan = Default::default();
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps2 = g2.get_player_by_position(0, Position::Top).unwrap();
        let plan = old::AttackNexusPlan::new(1, LineType::Mid);
        let e0 = cache2.player_champion[0][0].unwrap();
        println!("anpC\toutside\tx={} y={} hp={}/{}\t{:?}", e0.x, e0.y, e0.hp, e0.stat_cached.hp,
                 plan.sub_plan(3, &mut rnd, ps2, &data2, &tp, &mut dbg));
    }
    // 적 트윈타워를 전부 지우면 AttackNexus 가 나오는가
    {
        let mut g3 = mkgame(&setting, &ms, &map, &ctx);
        let twin: Vec<usize> = g3.world.tower_ids.iter().cloned().collect();
        let mut removed = 0;
        for id in twin.iter() {
            let t1 = matches!(g3.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)));
            if t1 { g3.world.remove_entity(*id); removed += 1; }
        }
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let cache3 = AbstractGameWithCache::new(&g3 as &dyn AbstractGame, &ctx);
        let data3 = OperationData::new(&cache3, &ctx, &bb);
        let tp: TeamPlan = Default::default();
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps3 = g3.get_player_by_position(0, Position::Top).unwrap();
        let plan = old::AttackNexusPlan::new(1, LineType::Mid);
        println!("anpC\tno_enemy_tower\tremoved={}\ttwin1len={}\t{:?}", removed, cache3.twin_towers[1].len(),
                 plan.sub_plan(3, &mut rnd, ps3, &data3, &tp, &mut dbg));
    }
}
