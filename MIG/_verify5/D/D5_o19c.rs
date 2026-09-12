#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #7 — `/specs[19]` 의 **817~819 폴백 경로 도달** + 필터 술어 `is_cleared` 완전식.
//!
//! o19b 에서 `JungleRunner.next_respawn_tick` 을 올려도 `is_cleared` 가 false 였다. IR 을 읽으니
//! (`_gaibc\m04.ll:62404~62500`) `is_cleared` 가 보는 리스폰 시각은 **JungleRunner 가 아니라
//! `TeamPlan.next_respawn_tick[team][camp_idx]`(TeamPlan+0x378, `[[usize;4];2]`, pub)** 였다.
//! ⟹ 「재료를 바꿔라」 규칙대로 오라클 이분탐색을 멈추고 IR 을 봤고, 그게 맞았다.
//!
//! is_cleared 완전식(IR):
//!   champ = data.cache.player_champion[player.info.team(+0x930)][player.info.position(+0x9c0)]
//!   if champ.is_none() { return false }
//!   respawn = team_plan.next_respawn_tick[team][camp_idx(camp)]      // TeamPlan+0x378
//!   if !(game.tick() < respawn) { return false }                     // vtable+0x28
//!   eta = utils::distance(champ.x(+0x660), champ.y(+0x668), camp_pos.0, camp_pos.1)
//!         / champ.stat_cached.move_speed(+0x640)                     // ⚠ 0 이면 div-by-zero 패닉
//!   respawn > eta + offset + game.tick() + setting.tick_per_second(GameSetting+0x12f8)
use game_core::*;
use game_ai::plan_legacy::old::{best_jungle_goal, is_cleared};
use game_ai::plan_legacy::team_plan::{camp_idx, TeamPlan};
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
    let setting = real_setting();
    println!("setting_ok\ttrue\ttps={}\twidth={}", setting.tick_per_second, setting.width);
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
    game.world.tick = 100;

    for c in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee] {
        println!("camp_idx\t{:?}\t{}", c, camp_idx(c));
    }
    println!("camp_idx\tMorgard/Serpen\tunreachable!() panic at team_plan.rs:40 (실측: 이 프로브 1차 실행이 거기서 죽었다)");

    // JungleRunner 를 서로 다른 리스폰 틱으로 (폴백 정렬 키)
    game.mode.jungle_runner.blue_rhino.next_respawn_tick = 900;
    game.mode.jungle_runner.blue_mushroom.next_respawn_tick = 500;
    game.mode.jungle_runner.blue_bee.next_respawn_tick = 700;
    game.mode.jungle_runner.blue_stump.next_respawn_tick = 600;

    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let me = game.get_player_by_position(0, Position::Jungle).unwrap();
    println!("player\tteam={}\tposition={:?}({})", me.info.team, me.info.position, me.info.position as usize);

    // 내가 통제하는 Entity 를 캐시에 꽂는다 (move_speed != 0 보장 — 0 이면 is_cleared 가 div-by-zero 패닉)
    let src = cache.player_champion[0][1].unwrap();
    let mut my: Entity = unsafe { std::ptr::read(src as *const Entity) };
    println!("champ_speed_before\tstat_cached.move_speed={}", my.stat_cached.move_speed);
    my.stat_cached.move_speed = 1000;
    my.x = 480000; my.y = 480000;
    let myref: &Entity = unsafe { &*(&my as *const Entity) };
    cache.player_champion[0][1] = Some(myref);

    let tick = (&game as &dyn AbstractGame).tick();
    let tps = setting.tick_per_second;
    println!("world\ttick={}\ttps={}", tick, tps);

    // is_cleared 완전식 game==mine
    let mut nrun = 0usize; let mut nmatch = 0usize;
    for respawn in [0usize, 100, 101, 500, 2000, 100000] {
        for offset in [0usize, 1, 500] {
            let mut tp: TeamPlan = Default::default();
            for t in 0..2usize { for k in 0..4usize { tp.next_respawn_tick[t][k] = respawn; } }
            let data = OperationData::new(&cache, &ctx, &bb);
            for c in CAMPS {
                let mut r = rand::rngs::StdRng::seed_from_u64(7);
                let mut dbg: DebugFrameData = Default::default();
                let g = is_cleared(c, 0, 1, &mut r, me, &data, &tp, offset, &mut dbg);
                // 내 재현
                let m = {
                    let ch = cache.player_champion[0][1];
                    if ch.is_none() { false } else if !(tick < respawn) { false } else {
                        let e = ch.unwrap();
                        let (cx, cy) = map.camp_pos(c, true);
                        let eta = game_core::utils::distance(e.x, e.y, cx, cy) / (e.stat_cached.move_speed as u64);
                        (respawn as u64) > eta + (offset as u64) + (tick as u64) + (tps as u64)
                    }
                };
                nrun += 1; if g == m { nmatch += 1; }
                let (cx, cy) = map.camp_pos(c, true);
                println!("isc\trespawn={}\toffset={}\t{:?}\tgame={}\tmine={}\t{}\teta={}",
                    respawn, offset, c, g, m, if g == m { "MATCH" } else { "**DIFF**" },
                    game_core::utils::distance(my.x, my.y, cx, cy) / (my.stat_cached.move_speed as u64));
            }
        }
    }

    // ── 817~819 폴백 도달: 전부 cleared 가 되게 respawn 을 크게
    {
        let mut tp: TeamPlan = Default::default();
        for t in 0..2usize { for k in 0..4usize { tp.next_respawn_tick[t][k] = 100000; } }
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut ncl = 0;
        for c in CAMPS {
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            if is_cleared(c, 0, 1, &mut r, me, &data, &tp, 0, &mut dbg) { ncl += 1; }
        }
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        if let GameMode::Moba(m) = cache.game.get_game_mode() {
            let mine = *CAMPS.iter().min_by_key(|c| m.jungle_runner.get_camp_state(0, **c).next_respawn_tick).unwrap();
            nrun += 1; if g == mine { nmatch += 1; }
            println!("FALLBACK\tcleared={}/4\tgame={:?}\tmine={:?}\t{}\t(runner: Rhino900 Mushroom500 Bee700 Stump600)",
                ncl, g, mine, if g == mine { "MATCH" } else { "**DIFF**" });
        }
    }
    // 폴백 최소값 스윕 + 동점 first-wins
    let cases: [(usize, usize, usize, usize, &str); 6] = [
        (900, 500, 700, 600, "min=Mushroom"),
        (300, 500, 700, 600, "min=Rhino"),
        (900, 800, 400, 600, "min=Bee"),
        (900, 800, 700, 200, "min=Stump"),
        (500, 500, 700, 600, "tie Rhino/Mushroom -> first=Rhino"),
        (900, 900, 300, 300, "tie Bee/Stump -> first=Bee"),
    ];
    let gp = &game as *const Game as *mut Game;
    for (rh, mu, be, st, why) in cases {
        // ⚠ cache 가 &game 을 들고 있어 안전 코드로는 못 고친다 — raw ptr 로 살아있는 game 을 직접 갱신한다
        unsafe {
            (*gp).mode.jungle_runner.blue_rhino.next_respawn_tick = rh;
            (*gp).mode.jungle_runner.blue_mushroom.next_respawn_tick = mu;
            (*gp).mode.jungle_runner.blue_bee.next_respawn_tick = be;
            (*gp).mode.jungle_runner.blue_stump.next_respawn_tick = st;
        }
        let mut tp: TeamPlan = Default::default();
        for t in 0..2usize { for k in 0..4usize { tp.next_respawn_tick[t][k] = 100000; } }
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me, &data, &tp, None, &mut dbg);
        let mine = *CAMPS.iter().min_by_key(|c| {
            match **c { JungleType::Rhino => rh, JungleType::Mushroom => mu,
                        JungleType::Bee => be, _ => st } }).unwrap();
        let seen: Vec<usize> = CAMPS.iter().map(|c| {
            if let GameMode::Moba(m) = cache.game.get_game_mode() {
                m.jungle_runner.get_camp_state(0, *c).next_respawn_tick } else { 0 } }).collect();
        nrun += 1; if g == mine { nmatch += 1; }
        println!("fbk	Rhino{} Mushroom{} Bee{} Stump{}	seen={:?}	game={:?}	mine={:?}	{}	{}",
            rh, mu, be, st, seen, g, mine, if g == mine { "MATCH" } else { "**DIFF**" }, why);
    }

    println!("TOTAL\t{}/{}\tMATCH", nmatch, nrun);
    println!("DONE");
    std::mem::forget(my);
}
