#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ④ — `v3_epicops_buff_window` **도달 판정 마커**
//!
//! IR 로 얻은 호출부 가드(`objective_handlers.rs:1116~1128`, m09.ll 14780~14832):
//!   1116  if game.as_moba()?.remain_epic_time(team) > tps*10 {
//!   1119     if version > 1 {
//!   1123        if my_alive_count >= enemy_alive_count {
//!   1120           v3_epicops_buff_window(...)          ← 표적
//!             }
//!          } else { handle_press_epic(...) }            // 1126
//!   1128     self.v3_press_chat_line = None             // ★무조건 실행되는 마커
//!
//! ⟹ `v3_press_chat_line`(+0x41e, private)을 **호출 전에 raw 로 Some(Top)=0 으로 세팅**해 두면
//!    호출 후 값이 255 로 바뀌었는지로 **이 블록에 도달했는지**를 가릴 수 있다.
//!    (5차 배치D 는 이 필드를 아예 안 읽어 「도달 불가」로 닫았다.)
use game_core::*;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};
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

unsafe fn raw(tp: &mut TeamPlan) -> *mut u8 { tp as *mut TeamPlan as *mut u8 }
unsafe fn snap(tp: &TeamPlan) -> Vec<u8> {
    std::slice::from_raw_parts(tp as *const TeamPlan as *const u8, std::mem::size_of::<TeamPlan>()).to_vec()
}
fn difflist(a: &[u8], b: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            let s = i;
            while i < a.len() && a[i] != b[i] { i += 1 }
            out += &format!(" +0x{:x}..{:x}", s, i);
        } else { i += 1 }
    }
    if out.is_empty() { " (없음)".into() } else { out }
}

/// 부상 챔프 주입 — stat_cached.hp 를 실전값으로 세우고 hp 를 비율대로 낮춘다.
unsafe fn mkchamp(e: &Entity, maxhp: usize, hp_percent: usize) -> &'static Entity {
    let mut c: Entity = std::ptr::read(e as *const Entity);
    c.stat_cached.hp = maxhp;
    c.hp = maxhp * hp_percent / 100;
    Box::leak(Box::new(c))
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

    // argv: <hurt_n> <hurt_pc> <healthy_pc>
    let hurt_n: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let hurt_pc: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    let well_pc: usize = std::env::args().nth(3).and_then(|s| s.parse().ok()).unwrap_or(100);

    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    unsafe {
        for x in 0..5usize {
            if let Some(e) = cache.player_champion[0][x] {
                let pc = if x < hurt_n { hurt_pc } else { well_pc };
                cache.player_champion[0][x] = Some(mkchamp(e, 1000, pc));
            }
            if let Some(e) = cache.player_champion[1][x] {
                cache.player_champion[1][x] = Some(mkchamp(e, 1000, 100));
            }
        }
    }
    let mut s = String::new();
    for x in 0..5usize {
        if let Some(e) = cache.player_champion[0][x] { s += &format!(" [{}]{}%", x, e.hp * 100 / e.stat_cached.hp); }
    }
    println!("HP\tteam0{}\t(hurt_n={} hurt_pc={} well_pc={})", s, hurt_n, hurt_pc, well_pc);

    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let gd: GoalData = Default::default();

    let objs: Vec<(String, Option<MainObjective>)> = vec![
        ("None".into(), None),
        ("Gank(Top)".into(), Some(MainObjective::Gank { line: LineType::Top })),
        ("Repair".into(), Some(MainObjective::Repair)),
        ("Defense".into(), Some(MainObjective::Defense)),
    ];

    for ver in [1usize, 2, 60] {
        for (on, ov) in objs.iter() {
            for t in 0..2usize {
                for p in [Position::Top, Position::Mid].iter() {
                    let plr = match game.get_player_by_position(t, *p) { Some(x) => x, None => continue };
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
                    let mut dbg: DebugFrameData = Default::default();
                    let mut tp: TeamPlan = Default::default();
                    tp.objective = ov.clone();
                    // ★마커: v3_press_chat_line(+0x41e, private) 을 Some(Top)=0 으로 raw 세팅
                    unsafe { *raw(&mut tp).add(0x41e) = 0u8; }
                    let before = unsafe { snap(&tp) };
                    let mut plan = BigPlan::ForcePassive;
                    tp.update_objective(ver, &mut rnd, plr, &data, &gd, &mut plan, &mut dbg);
                    let after = unsafe { snap(&tp) };
                    let reached = after[0x41e] != 0;      // 1128 줄이 돌았으면 255, 18 이 돌았으면 group_line
                    println!("R\tver={}\tobj={}\tteam={}\tpos={:?}\tpcl={}->{}\tobjective={}\tchats_len={}\teo={}\treached={}\tdiff={}",
                        ver, on, t, p, 0, after[0x41e], after[0x41f],
                        u64::from_le_bytes(after[0xd0..0xd8].try_into().unwrap()),
                        u64::from_le_bytes(after[0x410..0x418].try_into().unwrap()),
                        reached, difflist(&before, &after));
                    for c in tp.chats.iter() {
                        let cp = c as *const Chat as *const u8;
                        unsafe { println!("  CHAT\ttag={}\tb1={}", *cp, *cp.add(1)); }
                    }
                }
            }
        }
    }
}
