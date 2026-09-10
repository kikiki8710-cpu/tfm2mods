#![allow(unused, dead_code, non_snake_case)]
// 2차 반증검증 배치 A — 오라클 1단계
//  ① tick_per_second=60 을 실제로 세팅한다(1차의 미해결 숙제)
//  ② judge_accuracy 진리표 (judgement x judgement_mental_ratio)
//  ③ AthleteParameter::update 가 judgement_mental_ratio 를 어떻게 흔드는가
//  ④ Position::as_index / Entity::radius / Entity::is_visible_from / MapDef fountains
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    println!("setting\ttps_default\t{}", setting.tick_per_second);
    setting.tick_per_second = 60;                       // ★1차 숙제
    println!("setting\ttps_now\t{}", setting.tick_per_second);
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

    // ---------- ② judge_accuracy 진리표 (jmr = 1000 기본값) ----------
    for j in [0usize, 1, 10, 11, 50, 99, 100, 101, 111, 200, 1000] {
        let mut st: AthleteStat = Default::default();
        st.judgement = j;
        let p = AthleteParameter::new(&st);
        println!("judge_acc\tjudgement={}\tacc={}\tmine={}", j, p.judge_accuracy(),
                 std::cmp::min(j.wrapping_mul(1000) / 1000, 100) * 9 + 100);
    }

    // ---------- ③ update 로 judgement_mental_ratio 를 흔든다 ----------
    // sig: update(&mut self, rnd, _team, tick, statistics, team_gold_diff, dm_score_gap,
    //             team_death_delta, lane_cs_behind)
    let stats: PlayerStatistics = Default::default();
    for mental in [0usize, 30, 50, 80, 100, 200] {
        for gap in [0usize, 1, 2, 3, 4, 10] {
            let mut st: AthleteStat = Default::default();
            st.judgement = 100;
            st.mental = mental;
            let mut p = AthleteParameter::new(&st);
            let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
            let before = p.judge_accuracy();
            p.update(&mut rnd, 0, 1, &stats, 0, gap, 0, false);
            // 손계산: gap_norm = min(gap,3)*1000/3
            //         degradation = (100-min(mental,100))*gap_norm/100*110/100
            //         target = max(1000 - degradation, 10);  jmr = min(target, 1000)
            let m = std::cmp::min(mental, 100);
            let gap_norm = std::cmp::min(gap, 3) * 1000 / 3;
            let degr = (100 - m) * gap_norm / 100 * 110 / 100;
            let target = std::cmp::max(1000usize.saturating_sub(degr), 10);
            let jmr = if gap == 0 { 1000 } else { std::cmp::min(target, 1000) };
            let mine = std::cmp::min(jmr * 100 / 1000, 100) * 9 + 100;
            let got = p.judge_accuracy();
            println!("upd\tmental={}\tgap={}\tbefore={}\tafter={}\tmine={}\t{}",
                     mental, gap, before, got, mine,
                     if got == mine { "MATCH" } else { "*MISMATCH" });
        }
    }
    // update 반복 호출 = 단조 감소인가?
    {
        let mut st: AthleteStat = Default::default();
        st.judgement = 100; st.mental = 50;
        let mut p = AthleteParameter::new(&st);
        let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
        let mut seq = Vec::new();
        for k in 0..6 {
            seq.push(p.judge_accuracy());
            p.update(&mut rnd, 0, (k + 1) * 7, &stats, 0, if k % 2 == 0 { 3 } else { 0 }, 0, false);
        }
        seq.push(p.judge_accuracy());
        println!("upd_seq\tmental=50\t{:?}", seq);
    }

    // ---------- ④ Position::as_index ----------
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    for (i, p) in poss.iter().enumerate() {
        println!("as_index\tdecl={}\t{:?}\tidx={}", i, p, p.as_index());
    }

    // ---------- ④ fountains / nexus / camp ----------
    for t in 0..2usize {
        let f = map.fountain(t);
        println!("fountain\tteam={}\t{:?}", t, f);
    }

    // ---------- 게임 구성 ----------
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 60 + (pid as usize) * 3;
        st.mental = 50;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);

    // ---------- ④ Entity::radius / is_visible_from ----------
    for t in 0..2usize { for p in 0..5usize {
        if let Some(e) = cache.player_champion[t][p] {
            println!("ent\tt{}p{}\tx={}\ty={}\tradius()={}\tteam={:?}", t, p, e.x, e.y, e.radius(), e.team);
        }
    }}
    let a = cache.player_champion[0][0].unwrap();
    let b = cache.player_champion[1][0].unwrap();
    println!("vis\ta_from_b={}\tb_from_a={}\ta_from_a={}", a.is_visible_from(b), b.is_visible_from(a), a.is_visible_from(a));
}
