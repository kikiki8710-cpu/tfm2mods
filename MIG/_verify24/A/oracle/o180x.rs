#![allow(unused, dead_code, non_snake_case)]
//! 24차 A · 180 position_eval_at_uncached — **탐색 프로브**(세계 관찰 전용). pub 호출자 position_eval_at 경유.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30;
    s.respawn_growth_term = 1800; s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20; s.support_gold_reduction = 15;
    s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}", setting.width != 0 && setting.tick_per_second != 0);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    // walls: first wall cells
    let mut nw = 0; let mut firstwall = (99usize, 99usize);
    for yi in 0..30 { for xi in 0..30 { if map.walls[yi][xi] != 0 { nw += 1; if firstwall.0 == 99 { firstwall = (xi, yi); } } } }
    println!("walls\tcount={}\tfirst(xi,yi)={:?}\tnexus_pos={:?}\tfountains={:?}", nw, firstwall, map.nexus_pos, map.fountains);
    for t in 0..2usize { for p in 0..5usize {
        if let Some(e) = cache.player_champion[t][p] {
            let ps = game.get_player_by_position(t, [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support][p]).unwrap();
            println!("champ\tt={}\tp={}\tid={}\tx={}\ty={}\thp={}\tmaxhp={}\tlevel={}\tradius={}\tpid={}\tacc={}", t, p, e.id, e.x, e.y, e.hp, e.stat_cached.hp, e.level, e.radius, ps.info.id, ps.info.parameter.positioning_accuracy());
        }
    } }
    println!("jungles\tn={}", cache.jungles.len());
    for j in cache.jungles.iter() {
        println!("jungle\tid={}\tx={}\ty={}\thp={}\tty={:?}", j.id, j.x, j.y, j.hp, std::mem::discriminant(&j.ty));
    }
    println!("towers\tn={}", game.world.tower_ids.len());
    for tid in game.world.tower_ids.iter() {
        if let Some(t) = game.get_entity_by_id(*tid) {
            println!("tower\tid={}\tx={}\ty={}\thp={}\tteam={:?}\tcooltime={}", t.id, t.x, t.y, t.hp, t.team, t.attack_cooltime());
        }
    }
    println!("tick={}\tseed={}", game.tick(), game.seed());
    let ps = game.get_player_by_position(0, Position::Top).unwrap();
    let e = cache.player_champion[0][0].unwrap();
    for (x, y) in [(e.x, e.y), (900000u64, 30000u64), (800000, 64000), (799999, 64000), (480000, 480000)] {
        let s = game_ai::position_eval_at(2, ps, &data, x, y, game_ai::PositionEvalPurpose::Recall);
        let wd = game_ai::is_enemy_well_danger(2, ps, x, y);
        println!("score\tx={}\ty={}\twell_danger={}\trisk={}\ttower_risk={}\tgain={}\tgain_me={}\tadjust={}\tunseen={}\ton_traj={}\ton_ptraj={}", x, y, wd, s.risk, s.tower_risk, s.gain, s.gain_me, s.adjust, s.unseen_champ_threat, s.on_trajectory, s.on_periodic_trajectory);
    }
}
