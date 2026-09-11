#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 10단계 : 15 single_tower_dive_is_viable 의 판별 축 = **타워 hp** 인가
//  o8(타워 hp 를 100~5000 으로 낮춘 세계) = 전부 true / o9(타워 hp 디폴트) = 전부 false
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
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, ctx);
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
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let tp: TeamPlan = Default::default();
    let t1t0 = (272000u64, 48000u64);
    let mut g = mkgame(&setting, &ms, &map, &ctx);
    let ids: Vec<usize> = g.world.champion_ids.clone();
    let tw: Vec<usize> = g.world.tower_ids.clone();
    // 디폴트 타워 스탯 덤프
    for id in tw.iter().take(3) {
        let e = g.world.entity.get(*id).unwrap();
        println!("meta\ttower id={}\thp={}\tstat.hp={}\tstat_cached.hp={}\tattack={}\tradius={}",
                 id, e.hp, e.stat.hp, e.stat_cached.hp, e.stat_cached.attack, e.radius);
    }
    for id in ids.iter().take(1) {
        let e = g.world.entity.get(*id).unwrap();
        println!("meta\tchamp id={}\thp={}\tstat.hp={}\tstat_cached.hp={}\tattack={}",
                 id, e.hp, e.stat.hp, e.stat_cached.hp, e.stat_cached.attack);
    }
    // 배치 고정: 챔프 스탯 개방, target 을 T1#0 위, actor 20000 뒤
    for id in ids.iter() {
        let e = g.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 5000; e.stat_cached.attack = 5000;
        e.stat.defence = 0; e.stat_cached.defence = 0;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
    }
    { let e = g.world.entity.get_mut(23).unwrap(); e.x = t1t0.0; e.y = t1t0.1; }
    { let e = g.world.entity.get_mut(18).unwrap(); e.x = t1t0.0 + 20000; e.y = t1t0.1; }

    macro_rules! viable {
        () => {{
            let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let ps = g.get_player_by_position(0, Position::Top).unwrap();
            let e = cache.game.get_entity_by_id(23).unwrap();
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, e, &mut dbgf)
        }};
    }
    println!("15d\tdefault_tower_hp\tviable={}", viable!());
    // 타워 hp 를 낮춰가며 임계 찾기 (적팀 타워만 / 전체)
    for hp in [1usize, 100, 1000, 5000, 10000, 20000, 50000, 100000, 200000, 500000, 1000000] {
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                              e.stat.hp = hp; e.stat_cached.hp = hp; e.hp = hp; }
        println!("15d\tall_tower_hp={}\tviable={}", hp, viable!());
    }
    // 이분탐색
    let mut lo = 1usize; let mut hi = 2_000_000usize;
    let ok = |g: &mut Game, hp: usize| { for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                                          e.stat.hp = hp; e.stat_cached.hp = hp; e.hp = hp; } };
    ok(&mut g, lo); let vlo = viable!();
    ok(&mut g, hi); let vhi = viable!();
    println!("15d\tbisect endpoints: hp={} -> {} / hp={} -> {}", lo, vlo, hi, vhi);
    if vlo != vhi {
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            ok(&mut g, mid);
            if viable!() == vlo { lo = mid } else { hi = mid }
        }
        println!("15d\t★tower_hp 임계: {} -> {} / {} -> {}", lo, vlo, hi, vhi);
    }
    // 챔프 hp 도 같이 올려서 '내 hp vs 타워 dps' 비율인지 본다
    ok(&mut g, 100000);
    for chp in [2000usize, 10000, 50000, 200000, 1000000] {
        for id in ids.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                               e.stat.hp = chp; e.stat_cached.hp = chp; e.hp = chp; }
        println!("15d\ttower_hp=100000 champ_hp={}\tviable={}", chp, viable!());
    }
    // 타워 공격력
    for id in ids.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                           e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000; }
    ok(&mut g, 100000);
    for tatk in [0usize, 1, 10, 100, 1000, 10000] {
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
                              e.stat.attack = tatk; e.stat_cached.attack = tatk; }
        println!("15d\ttower_hp=100000 tower_atk={}\tviable={}", tatk, viable!());
    }
}
