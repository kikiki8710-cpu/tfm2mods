#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ⑤ — 18 의 **호출부 가드를 하나씩 계측**하고, 막는 값을 직접 바꿔 도달을 시도한다.
//!
//! 가드(IR, objective_handlers.rs:1116~1128):
//!   (a) `game.as_moba()` 가 Some          (b) `remain_epic_time(team) > tps*10`
//!   (c) `version > 1`                     (d) `my_alive >= enemy_alive`
//! `remain_epic_time` 은 `MobaMode+0x240+8*team` 에서 읽는다(IR gep 576). pub 게터로 값을 먼저 재고,
//! 막혀 있으면 그 워드를 raw 로 키워 다시 시도한다.
use game_core::*;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective};
use game_ai::plan_legacy::types::BigPlan;
use game_ai::GoalData;
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

unsafe fn snap(tp: &TeamPlan) -> Vec<u8> {
    std::slice::from_raw_parts(tp as *const TeamPlan as *const u8, std::mem::size_of::<TeamPlan>()).to_vec()
}
fn difflist(a: &[u8], b: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] { let s = i; while i < a.len() && a[i] != b[i] { i += 1 } out += &format!(" +0x{:x}..{:x}", s, i); }
        else { i += 1 }
    }
    if out.is_empty() { " (없음)".into() } else { out }
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

    // ── 가드 (a)(b) 계측 ────────────────────────────────────────────
    let tps = setting.tick_per_second;
    println!("GUARD\ttick={}\ttps={}\ttps10={}", game.tick(), tps, tps * 10);
    let gm = game.get_game_mode();
    println!("GUARD\tgame_mode_is_moba={}", gm.as_moba().is_some());
    let force: i64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(-1);
    {
        if let Some(mb) = gm.as_moba() {
            for t in 0..2usize {
                println!("GUARD\tremain_epic_time[{}]={}\t> tps*10 ? {}", t, mb.remain_epic_time(t),
                         mb.remain_epic_time(t) > tps * 10);
            }
            // IR gep 576 = MobaMode+0x240 + 8*team 이 remain 의 원천이라는 주장 검증 + 강제 세팅
            let base = mb as *const MobaMode as *const u8;
            unsafe {
                for t in 0..2usize {
                    let w = *(base.add(576 + 8 * t) as *const u64);
                    println!("RAW\tMobaMode+0x{:x}(team{})={}", 576 + 8 * t, t, w);
                }
                if force >= 0 {
                    for t in 0..2usize {
                        let p = base.add(576 + 8 * t) as *mut u64;
                        *p = force as u64;
                    }
                    for t in 0..2usize {
                        println!("FORCED\tremain_epic_time[{}]={}\t> tps*10 ? {}", t, mb.remain_epic_time(t),
                                 mb.remain_epic_time(t) > tps * 10);
                    }
                }
            }
        }
    }

    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut alive = [0usize; 2];
    for t in 0..2usize { for x in 0..5 { if cache.player_champion[t][x].is_some() { alive[t] += 1 } } }
    println!("GUARD\talive0={}\talive1={}", alive[0], alive[1]);

    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let gd: GoalData = Default::default();

    for ver in [1usize, 60] {
        for t in 0..2usize {
            let plr = match game.get_player_by_position(t, Position::Top) { Some(x) => x, None => continue };
            let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
            let mut dbg: DebugFrameData = Default::default();
            let mut tp: TeamPlan = Default::default();
            tp.objective = None;
            unsafe { *(&mut tp as *mut TeamPlan as *mut u8).add(0x41e) = 0u8; }   // 마커 Some(Top)
            let before = unsafe { snap(&tp) };
            let mut plan = BigPlan::ForcePassive;
            tp.update_objective(ver, &mut rnd, plr, &data, &gd, &mut plan, &mut dbg);
            let after = unsafe { snap(&tp) };
            println!("R\tforce={}\tver={}\tteam={}\tpcl=0->{}\tobjective={}\tchats_len={}\teo={}\tdiff={}",
                force, ver, t, after[0x41e], after[0x41f],
                u64::from_le_bytes(after[0xd0..0xd8].try_into().unwrap()),
                u64::from_le_bytes(after[0x410..0x418].try_into().unwrap()),
                difflist(&before, &after));
            for c in tp.chats.iter() {
                let cp = c as *const Chat as *const u8;
                unsafe { println!("  CHAT\ttag={}\tb1={}", *cp, *cp.add(1)); }
            }
        }
    }
}
