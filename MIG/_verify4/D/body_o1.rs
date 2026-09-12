
/// 3차 앵커(`_verify3\D\D3_o10.rs`·`D3_o11.rs`)와 같은 세계. **다른 점은 real_setting() 을 쓴다는 것뿐**이다
/// (3차 앵커는 `Default::default()` + tps=60 만 세팅해 width/height/champion_radius/visible_distance 가 0 이었다).
fn world(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext,
         boost: bool, thp: usize, tdef: usize, myhp: usize, twr_hp: usize) -> Game {
    let mut g = mkgame(setting, ms, map, ctx);
    let ids: Vec<usize> = g.world.champion_ids.clone();
    let tw: Vec<usize> = g.world.tower_ids.clone();
    let t1t0 = (272000u64, 48000u64);
    if boost {
        for id in ids.iter() {
            let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
            e.stat.attack = 5000; e.stat_cached.attack = 5000;
            e.stat.defence = 0; e.stat_cached.defence = 0;
            e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
            e.radius = 5000;
            if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        }
    }
    {
        let e = g.world.entity.get_mut(23).unwrap();
        e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp;
        e.stat.defence = tdef; e.stat_cached.defence = tdef;
        e.x = t1t0.0; e.y = t1t0.1;
    }
    for id in tw.iter() {
        let e = g.world.entity.get_mut(*id).unwrap();
        e.stat.hp = twr_hp; e.stat_cached.hp = twr_hp; e.hp = twr_hp;
    }
    for (k, id) in ids.iter().enumerate().take(5) {
        let e = g.world.entity.get_mut(*id).unwrap();
        if k == 0 {
            e.x = t1t0.0 + 20000; e.y = t1t0.1;
            e.stat.hp = myhp; e.stat_cached.hp = myhp; e.hp = myhp;
        } else { e.x = 15000; e.y = 913000; }
    }
    g
}

fn main() {
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
    let tp: TeamPlan = Default::default();

    {
        let g = mkgame(&setting, &ms, &map, &ctx);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        println!("towers\t{}\ttwin0={}\ttwin1={}", g.world.tower_ids.len(),
                 cache.twin_towers[0].len(), cache.twin_towers[1].len());
        let mut top = 0usize;
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                if game_core::is_top_side(&ctx, e.x, e.y) { top += 1; } } } }
        println!("is_top_side_true\t{}/10\texpect 8/10", top);
    }

    println!("--- A: 3차 앵커 재실행 (real_setting) ---");
    for &(tag, boost) in [("o11-true", true), ("o10-false", false)].iter() {
        let (thp, myhp, twr) = if boost { (50usize, 2000usize, 100usize) } else { (1, 1, 1) };
        let g = world(&setting, &ms, &map, &ctx, boost, thp, 0, myhp, twr);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf);
        println!("A\t{}\tviable={}", tag, v);
    }

    println!("--- B: target hp 스윕 (o11 세계) ---");
    for &thp in [1usize, 50, 2000, 20000, 200000, 2000000, 20000000].iter() {
        let g = world(&setting, &ms, &map, &ctx, true, thp, 0, 2000, 100);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf);
        println!("B\tthp={}\tviable={}", thp, v);
    }

    println!("--- B2: 내 hp 스윕 (o11 세계, thp=2000) ---");
    for &myhp in [1usize, 100, 2000, 200000, 20000000].iter() {
        let g = world(&setting, &ms, &map, &ctx, true, 2000, 0, myhp, 100);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let e = cache.game.get_entity_by_id(23).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf);
        println!("B2\tmyhp={}\tviable={}", myhp, v);
    }

    println!("--- C: check_kill_die_tick 직접 호출(pub 확인) + B<A 대조 ---");
    for &(thp, myhp) in [(2000usize, 2000usize), (20000000, 2000), (2000, 1), (50, 2000)].iter() {
        let g = world(&setting, &ms, &map, &ctx, true, thp, 0, myhp, 100);
        let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let ps = g.get_player_by_position(0, Position::Top).unwrap();
        let tgt = cache.game.get_entity_by_id(23).unwrap();
        let me = cache.player_champion[0][0].unwrap();
        let mut a6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        a6.push(tgt);
        let mut a7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        for t in cache.iter_towers_without_nexus(1) { a7.push(t); break; }
        let mut b6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        b6.push(me);
        let b7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        let mut r1 = rand::rngs::StdRng::seed_from_u64(7);
        let ka = game_ai::check_kill_die_tick(3, &mut r1, &data, ps, me, a6, a7, &mut dbgf);
        let mut r2 = rand::rngs::StdRng::seed_from_u64(7);
        let kb = game_ai::check_kill_die_tick(3, &mut r2, &data, ps, tgt, b6, b7, &mut dbgf);
        let mut r3 = rand::rngs::StdRng::seed_from_u64(7);
        let v = old::single_tower_dive_is_viable(3, &mut r3, ps, &data, &tp, tgt, &mut dbgf);
        println!("C\tthp={}\tmyhp={}\tkdt_me={}\tkdt_tgt={}\tB<A={}\tviable={}",
                 thp, myhp, ka, kb, kb < ka, v);
    }
    println!("done\tsetting_ok={}", ok);
}
