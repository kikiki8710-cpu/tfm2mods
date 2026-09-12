#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ⑥ — `specs[15]` 의 **산문 주장 2건**을 실행으로 반증 시도
//!  (1) `logic:249` — `iter_towers_without_nexus` 가 내놓는 **앞 6칸 순서**
//!      주장: `[top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2]` 그대로
//!            + 꼬리 = `twin_towers[team]`, `nexus` 는 제외
//!  (2) `open[0]` — `version` 축. 5차가 `new`/`new_dive`/`new_region` 은 무영향으로 닫았고
//!      **`SinglePlanBattle::update`(pub) 와 `single_tower_dive_is_viable` 는 남겼다.**
//!      여기서 `update` 의 version 축을 쓸어 open 범위를 더 좁힌다.
use game_core::*;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
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
const POS: [Position; 5] = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, POS[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let pool = bumpalo::Bump::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false, tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    println!("EType\tsize_of_EntityType={}", std::mem::size_of::<EntityType>());

    // (1) iter_towers_without_nexus 순서 — 주장 배열과 포인터 동일성으로 대조
    for t in 0..2usize {
        let named: [(&str, Option<&Entity>); 6] = [
            ("top_tower",     cache.top_tower[t]),
            ("mid_tower",     cache.mid_tower[t]),
            ("bottom_tower",  cache.bottom_tower[t]),
            ("top_tower2",    cache.top_tower2[t]),
            ("mid_tower2",    cache.mid_tower2[t]),
            ("bottom_tower2", cache.bottom_tower2[t]),
        ];
        let mut expect: Vec<(String, usize)> = Vec::new();
        for (n, o) in named.iter() {
            if let Some(e) = o { expect.push((n.to_string(), *e as *const Entity as usize)); }
        }
        for (i, e) in cache.twin_towers[t].iter().enumerate() {
            expect.push((format!("twin[{}]", i), *e as *const Entity as usize));
        }
        let got: Vec<usize> = cache.iter_towers_without_nexus(t).map(|e| e as *const Entity as usize).collect();
        println!("ITER\tteam={}\tn_expect={}\tn_got={}", t, expect.len(), got.len());
        let n = expect.len().max(got.len());
        let mut ok = 0usize; let mut ng = 0usize;
        for i in 0..n {
            let ex = expect.get(i);
            let gt = got.get(i);
            let m = match (ex, gt) { (Some(a), Some(b)) => a.1 == *b, _ => false };
            if m { ok += 1 } else { ng += 1 }
            println!("  IT[{}]\texpect={}\tmatch={}", i,
                ex.map(|x| x.0.clone()).unwrap_or("<없음>".into()), m);
        }
        // nexus 가 섞였는지
        let nx = cache.nexus[t].map(|e| e as *const Entity as usize);
        println!("ITER\tteam={}\tOK={}\tNG={}\tnexus_in_iter={}", t, ok, ng,
                 nx.map(|p| got.contains(&p)).unwrap_or(false));
    }

    // (2) SinglePlanBattle::update 의 version 축
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let plr = game.get_player_by_position(0, Position::Top).unwrap();
    let tgt = cache.player_champion[1][0].unwrap().id;
    let ps: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();
    for v in [0usize, 1, 2, 3, 30, 50, 54, 58, 60] {
        let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
        let mut dbg: DebugFrameData = Default::default();
        let mut b = SinglePlanBattle::new(v, BattlePlanGoal::TryKill(tgt, 60), &data, plr);
        let s0 = unsafe { std::slice::from_raw_parts(&b as *const SinglePlanBattle as *const u8, 144).to_vec() };
        b.update(v, &mut rnd, plr, &data, &ps, &tp, &mut dbg);
        let s1 = unsafe { std::slice::from_raw_parts(&b as *const SinglePlanBattle as *const u8, 144).to_vec() };
        let subgoal = u64::from_le_bytes(s1[0x58..0x60].try_into().unwrap());
        let mut ch = String::new();
        for i in 0..144 { if s0[i] != s1[i] { ch += &format!(" {:x}", i) } }
        println!("UPD\tver={}\tsub_goal_tag={}\tchanged_bytes={}", v, subgoal, ch.trim());
    }
}
