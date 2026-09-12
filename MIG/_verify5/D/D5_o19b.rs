#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #6 — `/specs[19]` 보강 3건.
//!  (a) `MapDef::camp_pos` 의 `CAMP_POS_MEMO`(thread_local RefCell) 가 **거짓말을 하는지** —
//!      o19 에서 setting 이 다른 두 MapDef 가 같은 좌표를 냈다. `MapDef.camps`(pub) 를 직접 읽어
//!      메모를 우회해 대조한다. **argv 로 맵 순서를 바꿔 프로세스를 갈라서도** 재라(한 exe = 한 케이스).
//!  (b) 817~819 폴백 경로(`not_cleared_camps.len()==0`) **도달** — `game.mode.jungle_runner`(pub) 의
//!      4캠프 `next_respawn_tick` 을 올려 전부 cleared 로 만든 뒤 최소 리스폰 캠프 선택을 확인.
//!  (c) 제곱거리 **동점 first-wins** — 격자 브루트포스로 진짜 동점점을 찾아 배열 순서 승자를 확인.
//! 실행: `D5_o19b.exe [m1|m2]`  (인자 없으면 m1 먼저)
use game_core::*;
use game_ai::plan_legacy::old::{best_jungle_goal, is_cleared};
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000;
    s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24; s.nexus_heal_decay = 100;
    s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}", ok, s.width, s.height, s.tick_per_second);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
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
    } }
    game.start_game(&mut rnd, ctx);
    game
}
const CAMPS: [JungleType; 4] = [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump];
fn d2(a: u64, b: u64) -> u64 { let d = if a > b { a - b } else { b - a }; d * d }

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "m1".into());
    let s1 = real_setting();
    let mut s2 = real_setting(); s2.width = 480000; s2.height = 480000;
    let _ = setting_ok(&s1);

    // (a) 메모 우회 — MapDef.camps 를 직접 읽는다. argv 로 **어느 맵을 먼저 만들고 먼저 물을지** 바꾼다
    {
        let (first, second, fname, sname) = if which == "m2" { (&s2, &s1, "m2", "m1") } else { (&s1, &s2, "m1", "m2") };
        let mf = MapDef::moba(first);
        println!("rawcamps\t{}\tn={}", fname, mf.camps.len());
        for cd in mf.camps.iter() { println!("rawcamps\t{}\t{:?}\tpos={:?}", fname, cd.ty, cd.pos); }
        for c in CAMPS { println!("memo1st\t{}\t{:?}\tblue={:?}\tred={:?}", fname, c, mf.camp_pos(c, true), mf.camp_pos(c, false)); }
        let msd = MapDef::moba(second);
        println!("rawcamps\t{}\tn={}", sname, msd.camps.len());
        for cd in msd.camps.iter() { println!("rawcamps\t{}\t{:?}\tpos={:?}", sname, cd.ty, cd.pos); }
        for c in CAMPS { println!("memo2nd\t{}\t{:?}\tblue={:?}\tred={:?}", sname, c, msd.camp_pos(c, true), msd.camp_pos(c, false)); }
    }

    // 이하 본 실험은 m1(실전 설정)으로 고정
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);

    // (b) 전부 cleared 로 만들기 — next_respawn_tick 을 서로 다르게 줘서 최소 선택을 판별한다
    //     Rhino=900 Mushroom=500 Bee=700 Stump=600  → 최소 = Mushroom
    game.mode.jungle_runner.blue_rhino.next_respawn_tick = 900;
    game.mode.jungle_runner.blue_mushroom.next_respawn_tick = 500;
    game.mode.jungle_runner.blue_bee.next_respawn_tick = 700;
    game.mode.jungle_runner.blue_stump.next_respawn_tick = 600;
    game.mode.jungle_runner.red_rhino.next_respawn_tick = 900;
    game.mode.jungle_runner.red_mushroom.next_respawn_tick = 500;
    game.mode.jungle_runner.red_bee.next_respawn_tick = 700;
    game.mode.jungle_runner.red_stump.next_respawn_tick = 600;
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let tp: TeamPlan = Default::default();
        let data = OperationData::new(&cache, &ctx, &bb);
        let me = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut ncl = 0;
        for c in CAMPS {
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            let cl = is_cleared(c, 0, 1, &mut r, me, &data, &tp, 0, &mut dbg);
            if cl { ncl += 1; }
            println!("cleared\t{:?}\t{}", c, cl);
        }
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        println!("fallback\tcleared={}/4\tgame={:?}\texpect_min_respawn=Mushroom(500)", ncl, g);
    }
    // 동점 폴백 — Rhino 와 Mushroom 을 같은 최솟값으로
    game.mode.jungle_runner.blue_rhino.next_respawn_tick = 500;
    game.mode.jungle_runner.blue_mushroom.next_respawn_tick = 500;
    game.mode.jungle_runner.blue_bee.next_respawn_tick = 700;
    game.mode.jungle_runner.blue_stump.next_respawn_tick = 600;
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let tp: TeamPlan = Default::default();
        let data = OperationData::new(&cache, &ctx, &bb);
        let me = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        println!("fallback_tie\tRhino=500 Mushroom=500 Bee=700 Stump=600\tgame={:?}\texpect_firstwins=Rhino", g);
    }
    // Bee 와 Stump 를 동점 최솟값으로 (배열 순서 Bee(idx2) < Stump(idx3))
    game.mode.jungle_runner.blue_rhino.next_respawn_tick = 900;
    game.mode.jungle_runner.blue_mushroom.next_respawn_tick = 900;
    game.mode.jungle_runner.blue_bee.next_respawn_tick = 300;
    game.mode.jungle_runner.blue_stump.next_respawn_tick = 300;
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let tp: TeamPlan = Default::default();
        let data = OperationData::new(&cache, &ctx, &bb);
        let me = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        println!("fallback_tie2\tRhino=900 Mushroom=900 Bee=300 Stump=300\tgame={:?}\texpect_firstwins=Bee", g);
    }
    // 원상복구
    for f in [0usize] { let _ = f; }
    game.mode.jungle_runner.blue_rhino.next_respawn_tick = 0;
    game.mode.jungle_runner.blue_mushroom.next_respawn_tick = 0;
    game.mode.jungle_runner.blue_bee.next_respawn_tick = 0;
    game.mode.jungle_runner.blue_stump.next_respawn_tick = 0;
    game.mode.jungle_runner.red_rhino.next_respawn_tick = 0;
    game.mode.jungle_runner.red_mushroom.next_respawn_tick = 0;
    game.mode.jungle_runner.red_bee.next_respawn_tick = 0;
    game.mode.jungle_runner.red_stump.next_respawn_tick = 0;

    // (c) 제곱거리 동점 브루트포스 — 격자에서 최솟값이 2개 이상인 점을 찾는다
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let tp: TeamPlan = Default::default();
    let me = game.get_player_by_position(0, Position::Jungle).unwrap();
    let src = cache.player_champion[0][1].unwrap();
    let mut my: Entity = unsafe { std::ptr::read(src as *const Entity) };
    let myref: &Entity = unsafe { &*(&my as *const Entity) };
    cache.player_champion[0][1] = Some(myref);
    let cp: Vec<(u64, u64)> = CAMPS.iter().map(|c| map.camp_pos(*c, true)).collect();
    let mut ties: Vec<(u64, u64, Vec<usize>)> = Vec::new();
    let mut x = 0u64;
    while x <= 960000 && ties.len() < 6 {
        let mut y = 0u64;
        while y <= 960000 && ties.len() < 6 {
            let ds: Vec<u64> = cp.iter().map(|(cx, cy)| d2(*cx, x) + d2(*cy, y)).collect();
            let mn = *ds.iter().min().unwrap();
            let idx: Vec<usize> = (0..4).filter(|i| ds[*i] == mn).collect();
            if idx.len() >= 2 { ties.push((x, y, idx)); }
            y += 1000;
        }
        x += 1000;
    }
    println!("ties_found\t{}", ties.len());
    let mut nrun = 0usize; let mut nmatch = 0usize;
    for (tx, ty, idx) in ties.iter() {
        my.x = *tx; my.y = *ty;
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        let expect = CAMPS[idx[0]];   // 배열 앞쪽이 이긴다는 주장
        nrun += 1; if g == expect { nmatch += 1; }
        println!("tie\t({},{})\ttied_idx={:?}\tgame={:?}\texpect_firstwins={:?}\t{}",
            tx, ty, idx, g, expect, if g == expect { "MATCH" } else { "**DIFF**" });
    }
    println!("TIE_TOTAL\t{}/{}", nmatch, nrun);
    println!("DONE\twhich={}", which);
    std::mem::forget(my);
}
