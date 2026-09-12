#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ③ — ⓒ **`TeamPlan` 1064B 통째 바이트 diff** 로 `v3_epicops_buff_window` 도달 여부 판정
//!
//! 5차 배치D 는 `TeamPlan::update_objective` 를 80조합 돌리고 **`objective`/`chats`/`eo_serpen_punish_issues`
//! 세 필드만** 읽어 「전부 불변 ⟹ 도달 불가」로 닫았다. 그런데 18 의 압박채팅 경로는
//! **`v3_press_chat_line`(+0x41e) 을 발화 여부와 무관하게 항상 갱신**한다(logic 679행).
//! ⟹ 세 필드만 보면 그 경로가 **보이지 않는다.** 여기서는 1064B 를 통째로 떠서 diff 한다(5차 배치C 수법).
//!
//! 덤으로 부상 챔프를 주입해 `v3_epicops_repair_need` 1/2 경로를 세운다
//! (Entity 41필드 전부 pub + `cache.player_champion` pub — 5차 배치D N20).
use game_core::*;
use game_ai::plan_legacy::team_plan::TeamPlan;
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

fn diff(tag: &str, a: &[u8], b: &[u8]) {
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            let s = i;
            while i < a.len() && a[i] != b[i] { i += 1 }
            runs.push((s, i));
        } else { i += 1 }
    }
    if runs.is_empty() { println!("DIFF\t{}\t(변화 없음)", tag); return }
    for (s, e) in runs {
        let av: Vec<String> = a[s..e].iter().map(|x| format!("{:02x}", x)).collect();
        let bv: Vec<String> = b[s..e].iter().map(|x| format!("{:02x}", x)).collect();
        println!("DIFF\t{}\t+0x{:x}..0x{:x}\t{} -> {}", tag, s, e, av.join(""), bv.join(""));
    }
}

/// 부상 챔프 주입 — Entity 를 복제해 hp 를 낮추고 캐시에 다시 꽂는다(5차 D N20 수법).
unsafe fn injure(e: &Entity, hp_percent: usize) -> &'static Entity {
    let mut c: Entity = std::ptr::read(e as *const Entity);
    let maxhp = c.stat_cached.hp;
    c.hp = maxhp * hp_percent / 100;
    Box::leak(Box::new(c))
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\ttps={}", setting.width != 0 && setting.tick_per_second != 0, setting.tick_per_second);
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

    // 케이스: 부상 인원 수(0=건드리지 않음)
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let hp_pc: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    unsafe {
        for x in 0..case.min(5) {
            if let Some(e) = cache.player_champion[0][x] {
                cache.player_champion[0][x] = Some(injure(e, hp_pc));
            }
        }
    }
    // 주입 확인
    for t in 0..2usize {
        let mut s = String::new();
        for x in 0..5usize {
            if let Some(e) = cache.player_champion[t][x] {
                s += &format!(" [{}]{}%", x, if e.stat_cached.hp > 0 { e.hp * 100 / e.stat_cached.hp } else { 0 });
            } else { s += &format!(" [{}]None", x) }
        }
        println!("HP\tteam={}{}", t, s);
    }

    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let mut dbg: DebugFrameData = Default::default();
    let gd: GoalData = Default::default();

    for t in 0..2usize {
        for (pi, p) in POS.iter().enumerate() {
            let plr = match game.get_player_by_position(t, *p) { Some(x) => x, None => continue };
            let mut tp: TeamPlan = Default::default();
            let mut plan = BigPlan::ForcePassive;
            let before = unsafe { snap(&tp) };
            tp.update_objective(60, &mut rnd, plr, &data, &gd, &mut plan, &mut dbg);
            let after = unsafe { snap(&tp) };
            diff(&format!("case{}_t{}_{:?}", case, t, p), &before, &after);
            // 관심 필드 직독
            unsafe {
                let b = &after[..];
                println!("FLD\tcase{}_t{}_{:?}\tobjective(+0x41f)={}\tv3_press_chat_line(+0x41e)={}\teo_serpen(+0x410)={}\tchats(cap/ptr/len)={}/{:x}/{}",
                    case, t, p, b[0x41f], b[0x41e],
                    u64::from_le_bytes(b[0x410..0x418].try_into().unwrap()),
                    u64::from_le_bytes(b[0xc0..0xc8].try_into().unwrap()),
                    u64::from_le_bytes(b[0xc8..0xd0].try_into().unwrap()),
                    u64::from_le_bytes(b[0xd0..0xd8].try_into().unwrap()));
                let n = u64::from_le_bytes(b[0xd0..0xd8].try_into().unwrap()) as usize;
                if n > 0 {
                    for c in tp.chats.iter() {
                        let cp = c as *const Chat as *const u8;
                        println!("CHAT\tcase{}_t{}_{:?}\ttag={}\tb1={}\tw1={}", case, t, p,
                            *cp, *cp.add(1), u64::from_le_bytes(std::slice::from_raw_parts(cp.add(8), 8).try_into().unwrap()));
                    }
                }
            }
        }
    }
}
