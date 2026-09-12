#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #8 — `/specs[15]`·`/specs[18]` 보강.
//! 두 함수 자체는 `vis=in:game_ai` 라 **직접 호출 불가**(tcx 정본). 그래서 다음만 실행으로 잡는다:
//!  A. `LegacyPlanHandler::new`(pub) → `team_plan`(+0xf8)·`positioning_score`(+0x990) 런타임 오프셋
//!  B. `PlayerState.info.team`(+0x930)·`info.position`(+0x9c0) 런타임 오프셋
//!  C. `SinglePlanBattle`(pub, Debug) — `sub_goal`(+0x58)·`dive_tower`(+0x8c) 오프셋 +
//!     ★`BattlePlanGoal::TryKill` 의 **두 번째 usize(=15 consts[1] 의 60)** 가 실제로 쓰이는지
//!  D. 실제 타워 엔티티의 `ty.Tower.info.ty`(+0x128)
//!  E. `TeamPlan`(pub) 의 `chats`(+0xc0)·`eo_serpen_punish_issues`(+0x410)·`objective`(+0x41f) 오프셋 +
//!     `TeamPlan::update_objective`(pub) 로 18 의 상위 진입점에 닿는지 시도
use game_core::*;
use game_ai::plan_legacy::old::{BattlePlanGoal, SinglePlanBattle, DeathMatchBattle};
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::plan_legacy::types::BigPlan;
use game_ai::GoalData;
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
fn ofs(base: usize, p: *const u8) -> usize { (p as usize) - base }

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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let me = game.get_player_by_position(0, Position::Top).unwrap();

    // ── A. LegacyPlanHandler
    {
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let h = LegacyPlanHandler::new(1usize, &mut r, 0usize, Position::Top);
        let b = &h as *const LegacyPlanHandler as usize;
        println!("A/size_of_LegacyPlanHandler\t{}", std::mem::size_of::<LegacyPlanHandler>());
        println!("A/team_plan\t+0x{:x}\tsize={}", ofs(b, &h.team_plan as *const _ as *const u8), std::mem::size_of::<TeamPlan>());
        println!("A/positioning_score\t+0x{:x}\tsize={}", ofs(b, &h.positioning_score as *const _ as *const u8),
            std::mem::size_of::<PositioningScoreData>());
        println!("A/version\t+0x{:x}\tplan\t+0x{:x}\tsub_plan\t+0x{:x}\tchats\t+0x{:x}",
            ofs(b, &h.version as *const _ as *const u8), ofs(b, &h.plan as *const _ as *const u8),
            ofs(b, &h.sub_plan as *const _ as *const u8), ofs(b, &h.chats as *const _ as *const u8));
    }

    // ── B. PlayerState
    {
        let b = me as *const PlayerState as usize;
        println!("B/size_of_PlayerState\t{}", std::mem::size_of::<PlayerState>());
        println!("B/info.team\t+0x{:x}\t={}", ofs(b, &me.info.team as *const _ as *const u8), me.info.team);
        println!("B/info.position\t+0x{:x}\t={:?}({})", ofs(b, &me.info.position as *const _ as *const u8),
            me.info.position, me.info.position as usize);
    }

    // ── C. SinglePlanBattle
    {
        let sp = SinglePlanBattle::new(1, BattlePlanGoal::TryKill(77, 60), &data, me);
        let b = &sp as *const SinglePlanBattle as usize;
        println!("C/size_of_SinglePlanBattle\t{}", std::mem::size_of::<SinglePlanBattle>());
        println!("C/main_goal\t+0x{:x}\tsub_goal\t+0x{:x}\tchats\t+0x{:x}\tstart_tick\t+0x{:x}\twith_dive\t+0x{:x}\tdive_tower\t+0x{:x}\ttactic\t+0x{:x}",
            ofs(b, &sp.main_goal as *const _ as *const u8), ofs(b, &sp.sub_goal as *const _ as *const u8),
            ofs(b, &sp.chats as *const _ as *const u8), ofs(b, &sp.start_tick as *const _ as *const u8),
            ofs(b, &sp.with_dive as *const _ as *const u8), ofs(b, &sp.dive_tower as *const _ as *const u8),
            ofs(b, &sp.tactic as *const _ as *const u8));
        println!("C/new_debug\t{:?}", sp);
        let spd = SinglePlanBattle::new_dive(1, BattlePlanGoal::TryKill(77, 60), &data, me);
        println!("C/new_dive_debug\t{:?}", spd);
        let spr = SinglePlanBattle::new_region(1, BattlePlanGoal::TryKill(77, 60), &data, me, 100, 200, 300);
        println!("C/new_region_debug\t{:?}", spr);
        // ★TryKill 두 번째 usize 축 — 60 이 무엇인가
        for x in [0usize, 1, 59, 60, 61, 1000, 999999] {
            let a = SinglePlanBattle::new(1, BattlePlanGoal::TryKill(77, x), &data, me);
            let d = DeathMatchBattle::new(1, BattlePlanGoal::TryKill(77, x), &data, me);
            println!("C/trykill1\t{}\tSPB={:?}\tDMB_sub={:?}\tDMB_chats={:?}", x, a, d.sub_goal(), d.chats);
        }
        // update 후 sub_goal — 채택/미채택 판정에 쓰는 태그
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut hh = LegacyPlanHandler::new(1usize, &mut r, 0usize, Position::Top);
        let mut dbg: DebugFrameData = Default::default();
        let mut sp2 = SinglePlanBattle::new(1, BattlePlanGoal::TryKill(77, 60), &data, me);
        println!("C/before_update_sub_goal\t{:?}", sp2.sub_goal());
        sp2.update(1, &mut r, me, &data, &hh.positioning_score, &hh.team_plan, &mut dbg);
        println!("C/after_update_sub_goal\t{:?}\tis_end={}", sp2.sub_goal(), sp2.is_end());
        // 실제 적 챔프를 target 으로
        let tid = cache.player_champion[1][0].unwrap().id;
        let mut sp3 = SinglePlanBattle::new(1, BattlePlanGoal::TryKill(tid, 60), &data, me);
        println!("C/real_target_before\t{:?}", sp3.sub_goal());
        sp3.update(1, &mut r, me, &data, &hh.positioning_score, &hh.team_plan, &mut dbg);
        println!("C/real_target_after\t{:?}\tis_end={}\tdebug={:?}", sp3.sub_goal(), sp3.is_end(), sp3);
    }

    // ── D. 실제 타워의 ty.Tower.info.ty
    for (i, id) in game.world.tower_ids.iter().enumerate().take(4) {
        if let Some(e) = cache.game.get_entity_by_id(*id) {
            let b = e as *const Entity as usize;
            let tag = unsafe { ((b as *const u8).add(0x68) as *const u64).read_unaligned() };
            if let EntityType::Tower { info } = &e.ty {
                println!("D/tower{}\tid={}\tty_tag={}\tinfo.ty@+0x{:x}\t={:?}",
                    i, id, tag, ofs(b, &info.ty as *const _ as *const u8), info.ty);
            }
        }
    }

    // ── E. TeamPlan
    {
        let mut tp: TeamPlan = Default::default();
        let b = &tp as *const TeamPlan as usize;
        println!("E/size_of_TeamPlan\t{}", std::mem::size_of::<TeamPlan>());
        println!("E/chats\t+0x{:x}\tobjective\t+0x{:x}\teo_serpen_punish_issues\t+0x{:x}\tnext_respawn_tick\t+0x{:x}",
            ofs(b, &tp.chats as *const _ as *const u8), ofs(b, &tp.objective as *const _ as *const u8),
            ofs(b, &tp.eo_serpen_punish_issues as *const _ as *const u8),
            ofs(b, &tp.next_respawn_tick as *const _ as *const u8));
        println!("E/objective_before\t{:?}\tchats_len={}\tpunish={}", tp.objective, tp.chats.len(), tp.eo_serpen_punish_issues);
        // 18 의 상위 진입점 시도
        let gd: GoalData = Default::default();
        let mut plan = BigPlan::ForcePassive;
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        tp.update_objective(1, &mut r, me, &data, &gd, &mut plan, &mut dbg);
        println!("E/after_update_objective\t{:?}\tchats_len={}\tpunish={}\tplan={:?}",
            tp.objective, tp.chats.len(), tp.eo_serpen_punish_issues, std::mem::discriminant(&plan));
        for c in tp.chats.iter() { println!("E/chat\t{:?}", c); }
        // 팀·포지션·**버전**을 바꿔 여러 번 (sig 상 18 의 version 은 range 2.. 라 1 은 v3 경로가 아니다)
        let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
        for v in [1usize, 2, 3, 30, 50, 54, 58, 60] {
            for t in 0..2usize { for p in poss.iter() {
                if let Some(ps) = game.get_player_by_position(t, *p) {
                    let mut tp2: TeamPlan = Default::default();
                    let mut r2 = rand::rngs::StdRng::seed_from_u64(7);
                    let mut dbg2: DebugFrameData = Default::default();
                    let mut plan2 = BigPlan::ForcePassive;
                    tp2.update_objective(v, &mut r2, ps, &data, &gd, &mut plan2, &mut dbg2);
                    let scw = game_ai::plan_legacy::old::v3_serpen_contest_clear_win(v, ps, &data, &tp2);
                    println!("E/uo\tv{}\tt{}\t{:?}\tobjective={:?}\tchats={}\tpunish={}\tserpen_contest_clear_win={}",
                        v, t, p, tp2.objective, tp2.chats.len(), tp2.eo_serpen_punish_issues, scw);
                }
            } }
        }
    }
    println!("DONE");
}
