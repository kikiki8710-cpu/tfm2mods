#![allow(unused, dead_code, non_snake_case)]
// 3차 배치A 오라클 3 — 04 handle_line_defense:
//   ① has_line_defense_threat 를 실제로 true 로 만든다 (2차가 "가능한데 안 했다"로 남긴 것)
//   ② 그 위에서 Battle/Gather 인원 구간(>1 / (n-1)<2)을 진리표로 확인
//   ⚠BRIEF §3① : start_game 뒤 init_tower/init_nexus 재호출 금지
use game_core::*;
use game_ai::plan_legacy::old as old;
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
    game
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    // 미니언 웨이브를 강제로 켠다(GameSetting::default() 는 전부 0 이라 아무것도 안 나온다)
    setting.minion_wave_setting.start_tick = 1;
    setting.minion_wave_setting.tick_per_wave = 600;
    setting.minion_wave_setting.melee_count = 3;
    setting.minion_wave_setting.range_count = 3;
    setting.minion_wave_setting.tick_per_spawn = 5;
    setting.minion_wave_setting.growth_start_tick = 100000;
    setting.minion_wave_setting.growth_tick = 100000;
    setting.minion_wave_setting.growth_end_tick = 100000;
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
    // ── STEP 0: 미니언이 생기는 틱을 찾는다 (run_tick 은 pub)
    {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let mut found = 0usize;
        for k in 1..=1200usize {
            let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ()));
            { let mut fd: Option<&mut GameFrameData> = None; g.run_tick(&ctx, &mut rnd, &mut fd); }
            if !g.world.minion_ids.is_empty() { found = k; break; }
        }
        println!("step0\tminion_spawn_tick={}\tminions={}\ttowers={}", found,
                 g.world.minion_ids.len(), g.world.tower_ids.len());
    }
    // ── STEP 1~: threat 를 켜고 인원 구간을 훑는다
    for &(pushed, cnt, use_target) in [(false, 0i32, false), (true, 0, false), (false, -3, false),
                                       (true, 0, true), (false, -3, true)].iter() {
      for def in [MorgardDefenseStrategy::Gather, MorgardDefenseStrategy::Battle].iter() {
        for n in 0usize..=4 {
            let mut g = mkgame(&setting, &ms, &map, &ctx);
            let mut rnd0 = rand::rngs::StdRng::seed_from_u64(5);
            let mut spawned = 0usize;
            for _ in 1..=700usize {
                { let mut fd: Option<&mut GameFrameData> = None; g.run_tick(&ctx, &mut rnd0, &mut fd); }
                if !g.world.minion_ids.is_empty() { spawned = g.world.minion_ids.len(); break; }
            }
            g.mode.epic_minion_buff_time = [600, 600];
            { let mut st: Strategy = g.world.strategy[0]; st.morgard_defense = *def;
              g.set_strategy(0, st); g.set_strategy(1, st); }
            // 아군(팀0) Top 타워
            let l = LineType::Top;
            let mut txy = None;
            for id in g.world.tower_ids.iter().cloned().collect::<Vec<_>>() {
                if let Some(e) = g.world.entity.get(id) {
                    if matches!(e.team, TeamType::Player(0)) && txy.is_none() { txy = Some((id, e.x, e.y)); }
                }
            }
            let (tid, tx, ty) = txy.unwrap();
            // 적팀 미니언을 그 라인 시작지점 근처로 옮기고 nearest_enemy = Some(tower_id)
            let mids: Vec<usize> = g.world.minion_ids.iter().cloned()
                .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
            let mut retarget = 0usize;
            for id in mids.iter() {
                if let Some(e) = g.world.entity.get_mut(*id) {
                    if use_target {
                        e.x = tx; e.y = ty;
                        if let EntityType::Minion { info } = &mut e.ty { info.nearest_enemy = Some(tid); retarget += 1; }
                    }
                }
            }
            // 적 챔피언 n 명을 타워 옆에
            let en: Vec<usize> = g.world.champion_ids.iter().cloned()
                .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
            for (k, id) in en.iter().enumerate() {
                if let Some(e) = g.world.entity.get_mut(*id) {
                    if k < n { e.x = tx + 1000 * (k as u64); e.y = ty; } else { e.x = 900000; e.y = 10000; }
                }
            }
            let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
            for b in bb.iter_mut() {
                b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5];
                b.last_reveal_hidden_span = [0; 5];
            }
            if pushed { bb[0].top_minion_state.from_mid = -5000; }
            bb[0].top_minion_state.minion_count = cnt;
            let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbg: DebugFrameData = Default::default();
            let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
            let ps = g.get_player_by_position(0, Position::Top).unwrap();
            let gd: &dyn AbstractGame = &g;
            let thr = old::has_line_defense_threat(ps, &data, l, tid);
            let mut near = 0;
            for e in cache.champions(1, &pool).iter() {
                let v = bb[1].is_recent_visible(gd, ps, e);
                let dx = if e.x > tx { e.x - tx } else { tx - e.x };
                let dy = if e.y > ty { e.y - ty } else { ty - e.y };
                if v && dx * dx + dy * dy < 40000000001u64 { near += 1; }
            }
            let s = ps.strategy(&mut rnd, gd);
            let r = old::handle_line_defense(3, &mut rnd, ps, &data, l, &mut dbg);
            println!("row\tpushed={}\tmcount={}\ttarget={}\tminions={}\tretarget={}\tdef={:?}\tn={}\tnear={}\tthreat={}\thld={}",
                     pushed, cnt, use_target, spawned, retarget, s.morgard_defense, n, near, thr, r);
        }
      }
    }

    // ── STEP 2: 반경 임계(40000000001 = 200000^2 + 1) 경계 실측
    for &d in [199999u64, 200000, 200001, 200002].iter() {
        let mut g = mkgame(&setting, &ms, &map, &ctx);
        let mut rnd0 = rand::rngs::StdRng::seed_from_u64(5);
        for _ in 1..=700usize {
            { let mut fd: Option<&mut GameFrameData> = None; g.run_tick(&ctx, &mut rnd0, &mut fd); }
            if !g.world.minion_ids.is_empty() { break; }
        }
        g.mode.epic_minion_buff_time = [600, 600];
        { let mut st: Strategy = g.world.strategy[0];
          st.morgard_defense = MorgardDefenseStrategy::Gather;
          g.set_strategy(0, st); g.set_strategy(1, st); }
        let l = LineType::Top;
        let mut txy = None;
        for id in g.world.tower_ids.iter().cloned().collect::<Vec<_>>() {
            if let Some(e) = g.world.entity.get(id) {
                if matches!(e.team, TeamType::Player(0)) && txy.is_none() { txy = Some((id, e.x, e.y)); }
            }
        }
        let (tid, tx, ty) = txy.unwrap();
        let mids: Vec<usize> = g.world.minion_ids.iter().cloned()
            .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
        for id in mids.iter() {
            if let Some(e) = g.world.entity.get_mut(*id) {
                e.x = tx; e.y = ty;
                if let EntityType::Minion { info } = &mut e.ty { info.nearest_enemy = Some(tid); }
            }
        }
        let en: Vec<usize> = g.world.champion_ids.iter().cloned()
            .filter(|id| matches!(g.world.entity.get(*id).map(|e| e.team), Some(TeamType::Player(1)))).collect();
        for (k, id) in en.iter().enumerate() {
            if let Some(e) = g.world.entity.get_mut(*id) {
                if k < 2 { e.x = tx + d; e.y = ty; } else { e.x = 900000; e.y = 10000; }
            }
        }
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        for b in bb.iter_mut() {
            b.last_visible = [usize::MAX; 5]; b.last_reveal_tick = [usize::MAX; 5];
            b.last_reveal_hidden_span = [0; 5];
        }
        bb[0].top_minion_state.from_mid = -5000;
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbg: DebugFrameData = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let r = old::handle_line_defense(3, &mut rnd, ps, &data, l, &mut dbg);
        println!("bound	dx={}	d2={}	hld={}", d, d as u128 * d as u128, r);
    }
}
