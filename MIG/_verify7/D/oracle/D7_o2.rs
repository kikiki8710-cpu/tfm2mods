#![allow(unused, dead_code, non_snake_case)]
//! 7차 배치 D 오라클 #2 — 19 `knobs[5]`(정글 클리어 여유 = tps) · `knobs[6]`(짝캠프 마진 = tps*5) 실측.
//!
//! 방법: `TeamPlan::next_respawn_tick`(**pub**, +0x378)을 1단위 이분탐색해
//!       `is_cleared` / `is_side_cleared` 가 false→true 로 바뀌는 **정확한 경계**를 찾는다.
//!       경계를 tps 여러 값에서 재면 계수(×1 / ×5)가 직접 나온다.
//! ★TLS: `MapDef::camp_pos` 는 `CAMP_POS_MEMO`(thread_local) 이므로 **한 프로세스 = 한 tps**.
//!       (프로세스 안에서 next_respawn 만 바꾸는 것은 그 메모의 키가 아니라 안전하다)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting(tps: usize) -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000;
    s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = tps; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24; s.nexus_heal_decay = 100;
    s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

fn main() {
    let tps: usize = std::env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(60);
    let setting = real_setting(tps);
    println!("setting_ok\ttps={}\twidth={}\tchamp_radius={}", setting.tick_per_second, setting.width, setting.champion_radius);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bbd: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bbd);
    println!("INTEG\ttowers={}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(),
             cache.twin_towers[0].len(), cache.twin_towers[1].len());
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let champ = cache.player_champion[0][Position::Jungle.as_index()].unwrap();
    println!("INFO\ttick={}\tchamp=({},{})\tmove_speed={}", game.tick(), champ.x, champ.y,
             champ.stat_cached.move_speed);
    for camp in [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump] {
        let (cx, cy) = map.camp_pos(camp, true);
        println!("INFO\tcamp={:?}\tpos=({},{})\tdist={}", camp, cx, cy,
                 game_core::utils::distance(champ.x, champ.y, cx, cy));
    }
    // 이분탐색 — `next_respawn_tick[0][i]` 를 올리며 false→true 경계를 찾는다
    for (i, camp) in [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump]
        .iter().enumerate() {
        for mode in 0..2usize {
            let f = |v: usize| -> bool {
                let mut tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
                for k in 0..4 { tp.next_respawn_tick[0][k] = v; }
                let mut dbg: DebugFrameData = Default::default();
                let mut r2 = rand::rngs::StdRng::seed_from_u64(11);
                if mode == 0 {
                    game_ai::plan_legacy::old::is_cleared(*camp, 0, 0, &mut r2, player, &data, &tp, 0, &mut dbg)
                } else {
                    game_ai::plan_legacy::old::is_side_cleared(*camp, 0, 0, &mut r2, player, &data, &tp, 0, &mut dbg)
                }
            };
            let (mut lo, mut hi) = (0usize, 40_000_000usize);
            if !f(hi) { println!("BS\ttps={}\tcamp={:?}\tmode={}\t경계없음(hi 에서도 false)", tps, camp, mode); continue; }
            while lo + 1 < hi {
                let mid = lo + (hi - lo) / 2;
                if f(mid) { hi = mid } else { lo = mid }
            }
            println!("BS\ttps={}\tcamp={:?}\tmode={}\t최소true={}\t(직전false={})",
                     tps, camp, if mode == 0 { "is_cleared" } else { "is_side_cleared" }, hi, lo);
        }
    }
}
