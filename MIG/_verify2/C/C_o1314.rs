#![allow(unused, dead_code, non_snake_case)]
// 2차 반증검증 배치 C — #13 LineGankCoverPlan::target_bush_v30 / #14 LineGankerPlan::update
// sub_plan 이 pub 이므로 target_bush_v30(cover) · target_bush_v41(ganker) 의 반환 부시를
// 실행으로 직접 뽑아 1차 IR 전표를 반증한다. *_lead 3필드는 캐시에 직접 써 넣는다.
use game_core::*;
use game_ai::plan_legacy::old::{LineGankerPlan, LineGankCoverPlan, LineGankerPhase};
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::GoalData;
use rand::SeedableRng;
use std::sync::Arc;

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
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let pn = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);

    // 챔피언을 흩어놓기 위해 N틱 굴린다 (Mid 분기의 is_top_side 양쪽을 모두 밟기 위함)
    let ticks: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    {
        for _ in 0..ticks { let mut ff: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd, &mut ff); }
    }

    let gd: GoalData = Default::default();
    let ps_score: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top", "Mid", "Bottom"];

    // 챔피언 좌표 + is_top_side 먼저 찍는다
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                println!("champ\tt{}\t{}\tx={}\ty={}\tis_top_side={}\tis_bottom_side={}",
                    t, pn[p], e.x, e.y,
                    map_regions::is_top_side(&ctx, e.x, e.y),
                    map_regions::is_bottom_side(&ctx, e.x, e.y));
            }
        }}
    }

    for lead in 0..8usize {
        for li in 0..3usize {
            for t in 0..2usize {
                for p in 0..5usize {
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    cache.top_lead = [lead, lead];
                    cache.mid_lead = [lead, lead];
                    cache.bottom_lead = [lead, lead];
                    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let ps = match game.get_player_by_position(t, poss[p]) { Some(x) => x, None => continue };

                    let gk = LineGankerPlan::new(lines[li], 100, 100);
                    let r1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        format!("{:?}", gk.sub_plan(3, &mut rand::rngs::StdRng::seed_from_u64(5), ps, &data, &mut dbg))
                    })).unwrap_or_else(|_| "PANIC".into());

                    let cv = LineGankCoverPlan::new(lines[li], 100);
                    let r2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        format!("{:?}", cv.sub_plan(3, &mut rand::rngs::StdRng::seed_from_u64(5), ps, &data, &mut dbg))
                    })).unwrap_or_else(|_| "PANIC".into());

                    println!("bush\tlead={}\tline={}\tteam={}\tpos={}\tGANKER={}\tCOVER={}",
                        lead, ln[li], t, pn[p], r1, r2);
                }
            }
        }
    }

    // update 의 도착판정 vs sub_plan 목표 부시 불일치 재현
    for lead in 0..7usize {
        for li in 0..3usize {
            for t in 0..2usize {
                let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                cache.top_lead = [lead, lead]; cache.mid_lead = [lead, lead]; cache.bottom_lead = [lead, lead];
                let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                let data = OperationData::new(&cache, &ctx, &bb);
                let ps = match game.get_player_by_position(t, Position::Jungle) { Some(x) => x, None => continue };
                for ph in [LineGankerPhase::WaitResponse, LineGankerPhase::Setup, LineGankerPhase::Cancel, LineGankerPhase::ChangeJungle(JungleType::Rhino)] {
                let mut gk = LineGankerPlan::new_with_phase(lines[li], 100, 100, ph);
                let before = format!("{:?}", gk);
                let mut r = rand::rngs::StdRng::seed_from_u64(5);
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    gk.update(3, &mut r, ps, &data, &gd, &ps_score, &mut dbg);
                })).is_ok();
                println!("update	lead={}	line={}	team={}	phase={:?}	ok={}	before={}	after={:?}	is_cancel={}",
                    lead, ln[li], t, ph, ok, before, gk, gk.is_cancel());
                }
        }
    }
}
}
