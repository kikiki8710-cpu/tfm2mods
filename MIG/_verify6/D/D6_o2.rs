#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ② — `specs[18] v3_epicops_buff_window` 의 **pub 피호출자 진리표**
//!
//! 5차 배치D 판정: 「이 세계 구성으로는 도달 불가 · 미탐색 = 에픽/세르펜 스폰 + 부상 + group_line 성립 세계」.
//! 이번엔 **함수 자체가 아니라 그 판정 알맹이 3개가 `pub`** 이라는 점을 쓴다(tcx 확인):
//!   game_ai::plan_legacy::old::v3_epic_group_line        (pub, epic.rs:814)
//!   game_ai::plan_legacy::old::v3_epic_formation_role    (pub, epic.rs:781)
//!   game_ai::plan_legacy::old::v3_serpen_contest_clear_win(pub, serpen.rs:52)
//! + ⓔ 게임모드/TutorialType 을 바꿔가며 「group_line == None → false」 경로가 실제로 서는지 본다.
use game_core::*;
use game_ai::plan_legacy::old::{v3_epic_group_line, v3_epic_formation_role, v3_serpen_contest_clear_win};
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

const POS: [Position; 5] = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
const TUT: [TutorialType; 9] = [TutorialType::None, TutorialType::First, TutorialType::TopSolo,
    TutorialType::Bottom, TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
    TutorialType::Line, TutorialType::Total];

fn lt(o: Option<LineType>) -> String {
    match o { None => "None".into(), Some(l) => format!("{:?}", l) }
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\ttps={}\theight={}", setting.width != 0 && setting.tick_per_second != 0,
             setting.tick_per_second, setting.height);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();

    // morgard_use 3종 (tcx: 5=Gather / 6=Split14{position} / 암묵=Split131{position1,position2})
    let mus: Vec<(String, MorgardUseStrategy)> = vec![
        ("Gather".into(), MorgardUseStrategy::Gather),
        ("Split14(Top)".into(), MorgardUseStrategy::Split14 { position: Position::Top }),
        ("Split14(Bottom)".into(), MorgardUseStrategy::Split14 { position: Position::Bottom }),
        ("Split131(Top,Bottom)".into(), MorgardUseStrategy::Split131 { position1: Position::Top, position2: Position::Bottom }),
        ("Split131(Mid,Support)".into(), MorgardUseStrategy::Split131 { position1: Position::Mid, position2: Position::Support }),
    ];
    // 메모리 태그 직독 — history[0] 의 「니치 밀림」 주장 반증 시도
    for (n, m) in mus.iter() {
        let p = m as *const MorgardUseStrategy as *const u32;
        unsafe { println!("MUTAG\t{}\tlow32={}\thigh32={}", n, *p, *p.add(1)); }
    }

    for (ti, tut) in TUT.iter().enumerate() {
        let pool = bumpalo::Bump::new();
        let ctx = GameContext {
            pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false, tutorial: *tut, trace_level: TraceLevel::Off,
        };
        let game = mkgame(&setting, &ms, &map, &ctx);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        if ti == 0 {
            println!("world\ttowers={}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(),
                     cache.twin_towers[0].len(), cache.twin_towers[1].len());
        }
        for t in 0..2usize {
            for (pi, p) in POS.iter().enumerate() {
                let plr = match game.get_player_by_position(t, *p) { Some(x) => x, None => continue };
                for (mn, m) in mus.iter() {
                    let gl = v3_epic_group_line(*m, plr, &data);
                    let role = v3_epic_formation_role(*m, *p, plr, &data);
                    let rs = match role { None => "None".to_string(),
                        Some(f) => format!("Some(is_split={},line={:?})", f.is_split, f.line) };
                    // 호출부 술어: is_some_and(|f| !f.is_split)
                    let ok = matches!(role, Some(f) if !f.is_split);
                    // 그리고 그 바이트0 (0=Some(false) / 1=Some(true) / 2=None) — history 주장 검증
                    let b0 = unsafe { *(&role as *const Option<game_ai::plan_legacy::old::V3EpicFormation> as *const u8) };
                    println!("GL\ttut={:?}\tteam={}\tpos={:?}\tmu={}\tgroup_line={}\trole={}\tok={}\tb0={}",
                             tut, t, p, mn, lt(gl), rs, ok, b0);
                }
            }
        }
        // announcer = (0..5).position(|x| champ[team][x].is_some() && role(x) 가 Some && !is_split)
        for t in 0..2usize {
            for (mn, m) in mus.iter() {
                let plr = match game.get_player_by_position(t, Position::Top) { Some(x) => x, None => continue };
                let gl = v3_epic_group_line(*m, plr, &data);
                let mut ann: Option<usize> = None;
                for x in 0..5usize {
                    if cache.player_champion[t][x].is_none() { continue }
                    let pl2 = match game.get_player_by_position(t, POS[x]) { Some(v) => v, None => continue };
                    let r = v3_epic_formation_role(*m, POS[x], pl2, &data);
                    if matches!(r, Some(f) if !f.is_split) { ann = Some(x); break }
                }
                println!("ANN\ttut={:?}\tteam={}\tmu={}\tgroup_line={}\tannouncer={:?}", tut, t, mn, lt(gl), ann);
            }
        }
        // 세르펜 징벌 성립성 (18 knobs[1] 우측 항)
        if ti == 0 {
            let tp: TeamPlan = Default::default();
            for t in 0..2usize {
                for (pi, p) in POS.iter().enumerate() {
                    let plr = match game.get_player_by_position(t, *p) { Some(x) => x, None => continue };
                    for v in [2usize, 30, 50, 54, 58, 60] {
                        let w = v3_serpen_contest_clear_win(v, plr, &data, &tp);
                        println!("SCW\tver={}\tteam={}\tpos={:?}\tclear_win={}", v, t, p, w);
                    }
                }
            }
        }
    }
}
