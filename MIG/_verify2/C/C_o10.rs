#![allow(unused, dead_code, non_snake_case)]
// 2차 배치 C — #10 주변 pub 헬퍼 + *_lead 산출 규칙 실행 확인
//  ① objective_entity_id_for_main_objective : MainObjective 12 variant 전수
//  ② can_enemy_hit_objective : range_margin 25000 의 단위(거리 오프셋) 확인
//  ③ AbstractGameWithCache 의 *_lead / region_point 실측 + 산출 규칙 재현
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    // ① MainObjective 전수
    let objs: Vec<(&str, MainObjective)> = vec![
        ("Morgard/None", MainObjective::Morgard { phase: ObjectPhase::None, with_battle: false }),
        ("Morgard/Setup", MainObjective::Morgard { phase: ObjectPhase::Setup, with_battle: false }),
        ("Morgard/Assemble", MainObjective::Morgard { phase: ObjectPhase::Assemble, with_battle: false }),
        ("Morgard/Hunt", MainObjective::Morgard { phase: ObjectPhase::Hunt, with_battle: false }),
        ("Morgard/Hunt+battle", MainObjective::Morgard { phase: ObjectPhase::Hunt, with_battle: true }),
        ("Serpen/Hunt", MainObjective::Serpen { phase: ObjectPhase::Hunt, with_battle: false }),
        ("Serpen/None", MainObjective::Serpen { phase: ObjectPhase::None, with_battle: false }),
    ];
    for (n, o) in objs.iter() {
        println!("objid\t{}\t{:?}", n, old::objective_entity_id_for_main_objective(&data, *o));
    }

    // ② can_enemy_hit_objective — margin 을 훑어 전이점을 찾는다
    let a = cache.player_champion[0][0].unwrap();
    let b = cache.player_champion[1][0].unwrap();
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    println!("hitpair\tax={} ay={} bx={} by={}\tdx={} dy={}\td2={}", a.x, a.y, b.x, b.y, dx, dy, dx*dx+dy*dy);
    let mut lo = 0u64; let mut hi = 3_000_000u64;
    for m in [0u64, 25000, 100000, 500000, 1_000_000, 1_200_000, 1_260_000, 1_270_000, 1_300_000, 2_000_000] {
        println!("hit\tmargin={}\t{}", m, old::can_enemy_hit_objective(b, a, m));
    }
    // 이분탐색으로 임계 margin
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if old::can_enemy_hit_objective(b, a, mid) { hi = mid; } else { lo = mid; }
    }
    println!("hit_threshold\tmargin={}\t(=이 값 이상이면 true)", hi);

    // ③ *_lead / region_point
    println!("lead\ttop={:?}\tmid={:?}\tbottom={:?}", cache.top_lead, cache.mid_lead, cache.bottom_lead);
    println!("region_point\t{:?}", cache.region_point);
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top", "Mid", "Bottom"];
    for li in 0..3usize {
        for team in 0..2usize {
            // IR 재현: lane_seq(line, team) 을 k=0..6 순회, region_point[r] < -2 인 동안 전진, 아니면 그 k 에서 정지
            let seq = map.lane_seq(lines[li], team);
            let mut k = 0usize;
            while k < 6 {
                let r = seq[k];
                if r < 27 && cache.region_point[r] < -2 { k += 1; } else { break; }
            }
            let game_v = cache.line_lead(team, lines[li]);
            println!("leadcalc\tline={}\tteam={}\tseq={:?}\tmine={}\tgame={}\t{}",
                ln[li], team, seq, k, game_v, if k == game_v { "MATCH" } else { "★MISMATCH" });
        }
    }
}
