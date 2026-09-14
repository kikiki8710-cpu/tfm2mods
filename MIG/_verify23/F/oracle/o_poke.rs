#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치F 오라클 — #174 EpicPokeSubPlan::action_candidates_old / #175 SerpenPokeSubPlan::action_candidates_old (둘 다 pub)
//! 닿는 분기 = start_game 직후(에픽/세르펜 미스폰 · 적에게 안 보임): 명세대로면
//!   Epic  → [AroundRegion(target_region=7, end_delay=5)] + attack_summon_action(빈 것으로 기대)
//!   Serpen→ [AroundRegion(target_region=2, end_delay=5)] + attack_summon_action
//! ScoreParameter(5384B) 는 pub 생성자가 없어 **제로버퍼**로 넣는다 — 이 함수는 +0x9f0(PositioningScoreData) 주소만 콜리에 넘기고
//! 그 콜리(position_score_at_position)가 zeros 를 읽어도 패닉이 없음을 실행으로 확인한다(있으면 그 자체가 결과).
//! 검증: sret Vec 의 len · 원소 태그 바이트(+0xb1) · AroundRegion 페이로드 +0x8 target_region · +0x28 end_delay · +0x20 goal_risk(i64::MAX 기대)
//! 사용: o_poke.exe <case>   0=Epic 1=Serpen  (케이스당 프로세스 1개)
use game_core::*;
use game_ai::plan_legacy::sub_plan::{EpicPokeSubPlan, SerpenPokeSubPlan};
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
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
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(100);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).expect("player");
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let before = format!("{:?}", rnd);
    // ScoreParameter 제로버퍼(5384B align 8)
    let mut sp: Buf<5384> = Buf([0u8; 5384]);
    let param: &game_ai::ScoreParameter = unsafe { &*(sp.0.as_ptr() as *const game_ai::ScoreParameter) };
    let res: bumpalo::collections::Vec<game_ai::SmallActionPlay> = if case == 0 {
        let mut plan: EpicPokeSubPlan = Default::default();
        plan.action_candidates_old(2, &mut rnd, player, &data, param)
    } else {
        let mut plan: SerpenPokeSubPlan = Default::default();
        plan.action_candidates_old(2, &mut rnd, player, &data, param)
    };
    let after = format!("{:?}", rnd);
    println!("case={} ({}) len={} rnd_changed={}", case, if case == 0 { "Epic" } else { "Serpen" }, res.len(), before != after);
    for (i, e) in res.iter().enumerate() {
        let p = e as *const game_ai::SmallActionPlay as *const u8;
        let tag = unsafe { *p.add(0xb1) };
        let f8 = unsafe { std::ptr::read_unaligned(p.add(0x8) as *const u64) };
        let f20 = unsafe { std::ptr::read_unaligned(p.add(0x20) as *const i64) };
        let f28 = unsafe { std::ptr::read_unaligned(p.add(0x28) as *const u64) };
        let f75 = unsafe { *p.add(0x75) };
        let name = match e {
            game_ai::SmallActionPlay::RunAway(_) => "RunAway",
            game_ai::SmallActionPlay::Around(_) => "Around",
            game_ai::SmallActionPlay::AroundRegion(_) => "AroundRegion",
            game_ai::SmallActionPlay::Attack(_) => "Attack",
            game_ai::SmallActionPlay::Skill(_) => "Skill",
            _ => "other",
        };
        println!("  [{}] variant={} tag@0xb1={} +0x8={} +0x20={} +0x28={} +0x75={}", i, name, tag, f8, f20, f28, f75);
    }
    // 원시 Vec 32B 레이아웃(ptr/bump/cap/len)
    let vp = &res as *const _ as *const u64;
    unsafe { println!("  vec32: ptr={:#x} bump={:#x} cap={} len={} (pool={:#x})", *vp, *vp.add(1), *vp.add(2), *vp.add(3), &pool as *const _ as u64); }
    std::mem::forget(res);
}
