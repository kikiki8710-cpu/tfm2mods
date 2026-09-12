
// D4_o2 — `single_tower_dive_is_viable` 판별축 검증.
//  ★한 번 실행에 **한 케이스만** 측정하고 끝낸다(인자로 케이스 선택).
//    이유 = `check_kill_die_tick`(fight_check.rs:917)은 **캐시된 진입점**(DieTickKey)이라
//    같은 프로세스에서 입력을 흔들면 캐시가 먼저 답할 수 있다. 3차의 「9축 전부 무관」이
//    그 아티팩트인지 가리려면 프로세스를 분리해야 한다.
//  케이스: o10_<towerhp>   = D3_o10 세계(아군 스폰 그대로, boost, target hp 2000)
//          o11_<targethp>  = D3_o11 세계(아군 4명 한 점 적층, tower hp 100)
//          seq             = 한 프로세스 안에서 o11 -> o10 순서로 두 번(캐시 오염 재현용)

fn one(setting: &GameSetting, ms: &MapSetting, map: &MapDef, pool: &bumpalo::Bump,
       ctx: &GameContext, stack_allies: bool, thp: usize, twr_hp: usize, tag: &str) {
    let tp: TeamPlan = Default::default();
    let t1t0 = (272000u64, 48000u64);
    let mut g = mkgame(setting, ms, map, ctx);
    let ids: Vec<usize> = g.world.champion_ids.clone();
    let tw: Vec<usize> = g.world.tower_ids.clone();
    for id in ids.iter() {
        let e = g.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 5000; e.stat_cached.attack = 5000;
        e.stat.defence = 0; e.stat_cached.defence = 0;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
    }
    { let e = g.world.entity.get_mut(23).unwrap();
      e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp; e.x = t1t0.0; e.y = t1t0.1; }
    { let e = g.world.entity.get_mut(18).unwrap(); e.x = t1t0.0 + 20000; e.y = t1t0.1; }
    if twr_hp != 0 {
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = twr_hp; e.stat_cached.hp = twr_hp; e.hp = twr_hp; }
    }
    if stack_allies {
        for id in ids.iter().skip(1).take(4) {
            let e = g.world.entity.get_mut(*id).unwrap(); e.x = 15000; e.y = 913000; }
    }
    let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let ps = g.get_player_by_position(0, Position::Top).unwrap();
    let tgt = cache.game.get_entity_by_id(23).unwrap();
    let me = cache.player_champion[0][0].unwrap();
    let mut r = rand::rngs::StdRng::seed_from_u64(7);
    let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, tgt, &mut dbgf);
    // ★ 내 재현: return check_kill_die_tick(target) < check_kill_die_tick(me)   (single_battle.rs:944)
    let mut a6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    a6.push(tgt);
    let mut a7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    for t in cache.iter_towers_without_nexus(1) { a7.push(t); break; }
    let mut b6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    b6.push(me);
    let b7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    let mut r1 = rand::rngs::StdRng::seed_from_u64(7);
    let ka = game_ai::check_kill_die_tick(3, &mut r1, &data, ps, me, a6, a7, &mut dbgf);
    let mut r2 = rand::rngs::StdRng::seed_from_u64(7);
    let kb = game_ai::check_kill_die_tick(3, &mut r2, &data, ps, tgt, b6, b7, &mut dbgf);
    println!("{}\tstack={}\tthp={}\ttwr_hp={}\tviable={}\tkdt_me={}\tkdt_tgt={}",
             tag, stack_allies, thp, twr_hp, v, ka, kb);
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("o10_0".to_string());
    let setting = real_setting();
    setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };

    if arg == "seq" {
        one(&setting, &ms, &map, &pool, &ctx, true, 50, 100, "seq1_o11");
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "seq2_o10");
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "seq3_o10again");
        return;
    }
    if arg == "rseq" {
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "rseq1_o10");
        one(&setting, &ms, &map, &pool, &ctx, true, 50, 100, "rseq2_o11");
        return;
    }
    let (kind, num) = arg.split_at(arg.find('_').unwrap_or(3));
    let n: usize = num.trim_start_matches('_').parse().unwrap_or(0);
    if kind == "o10" { one(&setting, &ms, &map, &pool, &ctx, false, 2000, n, &arg); }
    else { one(&setting, &ms, &map, &pool, &ctx, true, n, 100, &arg); }
}
