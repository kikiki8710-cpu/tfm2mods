#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 5단계 : 15 open[3] update 게이트 **이분탐색 + tps 의존성**
//  D3_o4 실측(스탯 개방 후, tps=60, tick=0, 비다이브, actor↔target 동일 y):
//     d <= 100000        -> Kiting{focus:target}
//     d 199999..250000   -> Trace{focus:target}
//     d >= 299999        -> RunAway
//     tick <= 120 -> Kiting / tick >= 121 -> RunAway   (start_tick 을 0 으로 덮어도 동일)
//  여기서 ①두 거리 임계를 1 단위로 확정 ②tick 임계가 tick_per_second 파생인지 확인(tps 30/60/90)
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal, BattleSubPlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

const A: usize = 18;
const T: usize = 23;

fn scenario(tps: usize) {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = tps;
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
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    let pos_score: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();
    let ids: Vec<usize> = game.world.champion_ids.clone();
    for id in ids.iter() {
        let e = game.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 100; e.stat_cached.attack = 100;
        e.stat.defence = 30; e.stat_cached.defence = 30;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
        if let Some(f) = e.skill_effect.as_mut()  { f.range = 60000; }
    }
    macro_rules! setpos {
        ($id:expr, $x:expr, $y:expr) => {{ let e = game.world.entity.get_mut($id).unwrap(); e.x = $x; e.y = $y; }};
    }
    macro_rules! sgof {
        () => {{
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let player = game.get_player_by_position(0, Position::Top).unwrap();
            let mut b = SinglePlanBattle::new(3usize, BattlePlanGoal::TryKill(T, 60), &data, player);
            b.update(3usize, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
            match b.sub_goal { BattleSubPlanGoal::Kiting{..} => 1u8, BattleSubPlanGoal::Trace{..} => 2,
                               BattleSubPlanGoal::RunAway => 3, BattleSubPlanGoal::End => 4,
                               BattleSubPlanGoal::KitingBack{..} => 5, BattleSubPlanGoal::Protect{..} => 6,
                               BattleSubPlanGoal::Assassin{..} => 7, BattleSubPlanGoal::AssassinReady{..} => 8 }
        }};
    }
    let nm = |c: u8| match c { 1=>"Kiting",2=>"Trace",3=>"RunAway",4=>"End",5=>"KitingBack",6=>"Protect",7=>"Assassin",_=>"AssassinReady" };

    setpos!(T, 500000, 500000);
    game.world.tick = 0;

    // ── 거리 임계 이분탐색: Kiting -> Trace ─────────────────────
    let probe = |g: &mut Game, d: u64| -> u8 { 0 };  // placeholder (매크로가 game 을 잡으므로 아래 인라인)
    let mut lo = 0u64; let mut hi = 400000u64;
    // 1) Kiting(1) 구간 상한
    {
        let mut lo1 = 0u64; let mut hi1 = 400000u64;
        while lo1 + 1 < hi1 {
            let mid = (lo1 + hi1) / 2;
            setpos!(A, 500000u64 - mid, 500000);
            if sgof!() == 1 { lo1 = mid } else { hi1 = mid }
        }
        setpos!(A, 500000u64 - lo1, 500000); let a = sgof!();
        setpos!(A, 500000u64 - hi1, 500000); let b = sgof!();
        println!("BIS\ttps={}\tKiting_max_d={} ({}) / next_d={} ({})", tps, lo1, nm(a), hi1, nm(b));
    }
    // 2) RunAway(3) 시작점
    {
        let mut lo2 = 0u64; let mut hi2 = 800000u64;
        while lo2 + 1 < hi2 {
            let mid = (lo2 + hi2) / 2;
            setpos!(A, 500000u64.saturating_sub(mid), 500000);
            if sgof!() == 3 { hi2 = mid } else { lo2 = mid }
        }
        setpos!(A, 500000u64.saturating_sub(lo2), 500000); let a = sgof!();
        setpos!(A, 500000u64.saturating_sub(hi2), 500000); let b = sgof!();
        println!("BIS\ttps={}\tlast_nonRunAway_d={} ({}) / RunAway_from_d={} ({})", tps, lo2, nm(a), hi2, nm(b));
    }
    // ── tick 임계 이분탐색 (d = Kiting 구간 안: 30000) ──────────
    setpos!(A, 470000, 500000);
    {
        let mut lo3 = 0usize; let mut hi3 = 100000usize;
        while lo3 + 1 < hi3 {
            let mid = (lo3 + hi3) / 2;
            game.world.tick = mid;
            if sgof!() == 1 { lo3 = mid } else { hi3 = mid }
        }
        game.world.tick = lo3; let a = sgof!();
        game.world.tick = hi3; let b = sgof!();
        println!("BIS\ttps={}\tKiting_max_tick={} ({}) / next_tick={} ({})\ttps*2={}\ttps*2+1={}",
                 tps, lo3, nm(a), hi3, nm(b), tps*2, tps*2+1);
        game.world.tick = 0;
    }
    // ── y 축으로도 같은가(순수 거리인지 축 의존인지) ────────────
    for d in [100000u64, 200000, 250000, 260000, 270000, 280000].iter() {
        setpos!(A, 500000, 500000u64 - *d);
        let cy = sgof!();
        setpos!(A, 500000u64 - *d, 500000);
        let cx = sgof!();
        // 대각선: 같은 유클리드 거리
        let dd = (*d as f64 / 2f64.sqrt()) as u64;
        setpos!(A, 500000 - dd, 500000 - dd);
        let cd = sgof!();
        println!("AXIS\ttps={}\td={}\tx={}\ty={}\tdiag={}", tps, d, nm(cx), nm(cy), nm(cd));
    }
}

fn main() {
    for tps in [60usize, 30, 90] { scenario(tps); }
}
