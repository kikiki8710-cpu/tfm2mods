#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #2 — `/specs[17]` 보강 3건을 **실행**으로 확정한다.
//!  (a) `start_tick = data.cache.game.tick()` (mem[19]·mem[5] vtable+0x28) — `world.tick` 을 0 이 아닌 값으로
//!      직접 세팅해 반환값이 따라오는지 본다. (o17 에서는 tick 이 0 이라 판별력이 없었다)
//!  (b) `chats` Vec 3워드의 정체(cap/ptr/len 순서)와 **push 1회 후 cap 실측** —
//!      `logic` 은 "cap 0 -> 1" 이라고 적었다.
//!  (c) 4차 R2(`base_sub_goal` 축 = 적 우물 위험)를 **실행으로** 확인 — 적팀 챔프 id(적 우물 안) vs
//!      아군 챔프 id vs 존재하지 않는 id.
use game_core::*;
use game_ai::plan_legacy::old::{BattlePlanGoal, DeathMatchBattle};
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
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
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
    game.start_game(&mut rnd, ctx);
    game
}
unsafe fn rd_u64(p: *const u8, off: usize) -> u64 { (p.add(off) as *const u64).read_unaligned() }
unsafe fn rd_i64(p: *const u8, off: usize) -> i64 { (p.add(off) as *const i64).read_unaligned() }

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);

    // (b) Vec<Chat> 3워드의 정체 — 우리가 직접 만든 Vec 으로 대조한다
    {
        let empty: Vec<Chat> = Vec::new();
        let p = &empty as *const Vec<Chat> as *const u8;
        unsafe { println!("vec_empty\tw0={}\tw1={}\tw2={}", rd_u64(p,0), rd_u64(p,8), rd_u64(p,16)); }
        let mut one: Vec<Chat> = Vec::new();
        one.push(Chat::Battle(77, 0));
        let p = &one as *const Vec<Chat> as *const u8;
        unsafe {
            println!("vec_push1\tw0={}\tw1={}\tw2={}\tcap_api={}\tlen_api={}",
                rd_u64(p,0), rd_u64(p,8), rd_u64(p,16), one.capacity(), one.len());
        }
        let mut two: Vec<Chat> = Vec::new();
        two.push(Chat::Battle(1, 0)); two.push(Chat::Battle(2, 0));
        println!("vec_push2\tcap_api={}\tlen_api={}", two.capacity(), two.len());
    }

    // (a) world.tick 을 0 이 아닌 값으로 세팅 → start_tick 이 따라오는가
    for t in [0usize, 1, 4321, 999999] {
        game.world.tick = t;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player = game.get_player_by_position(0, Position::Top).unwrap();
        let b = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(77, 88), &data, player);
        let p = &b as *const DeathMatchBattle as *const u8;
        unsafe {
            println!("tick\tworld.tick={}\tgame.tick()={}\tstart_tick=0x110->{}\tchats_cap={}\tchats_len={}",
                t, (&game as &dyn AbstractGame).tick(), rd_i64(p, 0x110),
                rd_u64(p, 0xf8), rd_u64(p, 0x108));
        }
    }

    // (c) base_sub_goal 축 — 적팀 챔프(적 우물 안) / 아군 챔프 / 없는 id
    game.world.tick = 0;
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    for t in 0..2usize {
        for (pi, ps) in poss.iter().enumerate() {
            if let Some(e) = cache.player_champion[t][pi] {
                println!("champ\tteam{}\t{:?}\tid={}\tx={}\ty={}\thp={}", t, ps, e.id, e.x, e.y, e.hp);
            }
        }
    }
    let me = game.get_player_by_position(0, Position::Top).unwrap();
    let mut ids: Vec<(String, usize)> = Vec::new();
    for t in 0..2usize {
        for pi in 0..5usize {
            if let Some(e) = cache.player_champion[t][pi] {
                ids.push((format!("t{}p{}", t, pi), e.id));
            }
        }
    }
    ids.push(("nonexistent".into(), 77));
    ids.push(("tower0".into(), *game.world.tower_ids.get(0).unwrap_or(&0)));
    for (nm, id) in ids.iter() {
        let sub = BattlePlanGoal::TryKill(*id, 0).base_sub_goal(1, me, &data);
        let b = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(*id, 0), &data, me);
        println!("bsg\t{}\tid={}\tdirect={:?}\tvia_new={:?}\tis_ignored_well_enemy={:?}",
            nm, id, sub, b.sub_goal(),
            cache.game.get_entity_by_id(*id).map(|e| game_ai::plan_legacy::old::is_ignored_well_enemy(1, me, e)));
    }
    // 적 우물 위험 술어 자체를 좌표 축으로 쓸어본다
    for (nm, x, y) in [("t0_spawn", 0u64, 0u64), ("center", 480000, 480000),
                       ("t1_spawn", 950000, 950000), ("t1_spawn2", 900000, 900000)] {
        println!("well\t{}\t({},{})\tis_enemy_well_danger(team0 Top)={}",
            nm, x, y, game_ai::is_enemy_well_danger(1, me, x, y));
    }
    println!("DONE\tsetting_ok={}", ok);
}
