#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치C 오라클 #3 — specs[10] 의 pub 피호출자로 상수 검증
//!  · can_enemy_hit_objective(pub, fight_model.rs:1188) → 25000(선형 여유치) · 19600000000(=140000² 하드컷)
//!  · objective_entity_id_for_main_objective(pub, :1178) → 태그 0..11 전수
//!  · Blackboard::is_recent_visible(pub, blackboard.rs:346) → 120틱 시간창
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::{MainObjective, ObjectPhase};
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
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
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

fn main() {
    let setting = real_setting();
    println!("setting_ok\ttrue\twidth={} height={} tps={} radius={}", setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
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

    // ================ ① can_enemy_hit_objective ================
    // 실전 AttackEffect 를 직접 조립해 판별력을 준다(shared.오라클_레시피_함정 ④)
    let mut ents: Vec<Entity> = Vec::new();
    {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let base = c0.player_champion[0][0].expect("champ 없음").clone();
        // 0 = 적(캐스터) / 1 = 오브젝트
        let mut a = base.clone(); a.x = 0; a.y = 0; a.level = 5;
        let mut b = base.clone(); b.x = 0; b.y = 0;
        ents.push(a); ents.push(b);
        println!("attack_effect_is_some\t{}\tskill={}\tskill2={}\tult={}",
            ents[0].attack_effect.is_some(), ents[0].skill_effect.is_some(),
            ents[0].skill2_effect.is_some(), ents[0].ult_effect.is_some());
    }
    println!("\n### O10-A can_enemy_hit_objective(enemy@(0,0), obj@(d,0), margin)");
    println!("margin\tboundary_d\t(true 인 최대 d)");
    for &margin in [0u64, 1, 25000, 50000, 100000, 200000, 400000].iter() {
        // 이분탐색으로 true 최대 거리를 찾는다
        let f = |d: u64| -> bool {
            let mut e = ents[0].clone(); let mut o = ents[1].clone();
            o.x = d; o.y = 0;
            old::can_enemy_hit_objective(&e, &o, margin)
        };
        if !f(0) { println!("{}\tNONE(d=0 에서도 false)", margin); continue }
        let mut lo = 0u64; let mut hi = 1_000_000u64;
        if f(hi) { println!("{}\t>=1000000", margin); continue }
        while lo + 1 < hi { let m = (lo + hi) / 2; if f(m) { lo = m } else { hi = m } }
        println!("{}\t{}\t(d={} true / d={} false)", margin, lo, lo, lo + 1);
    }

    // ================ ② objective_entity_id_for_main_objective ================
    println!("\n### O10-B objective_entity_id_for_main_objective(tag 0..11)");
    println!("tag\tname\tresult");
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let objs: Vec<(&str, MainObjective)> = vec![
            ("Morgard(Hunt,false)", MainObjective::Morgard { phase: ObjectPhase::Hunt, with_battle: false }),
            ("Serpen(Hunt,false)", MainObjective::Serpen { phase: ObjectPhase::Hunt, with_battle: false }),
            ("Defense", MainObjective::Defense),
            ("DefenseLine(Top)", MainObjective::DefenseLine(LineType::Top)),
            ("Nexus(Top)", MainObjective::Nexus(LineType::Top)),
            ("PressEpic(Top)", MainObjective::PressEpic(LineType::Top)),
            ("SplitEpic(Top)", MainObjective::SplitEpic(LineType::Top)),
            ("Repair", MainObjective::Repair),
            ("Gank(Top)", MainObjective::Gank { line: LineType::Top }),
            ("Dive(Top)", MainObjective::Dive { line: LineType::Top }),
            ("PressTower(Top)", MainObjective::PressTower { line: LineType::Top }),
            ("ComebackPick(Top,0)", MainObjective::ComebackPick { line: LineType::Top, ready: false }),
        ];
        for (i, (nm, o)) in objs.iter().enumerate() {
            let tag = unsafe { *(o as *const _ as *const u8) };
            let r = old::objective_entity_id_for_main_objective(&data, *o);
            println!("{}\t{}\t{:?}", tag, nm, r);
        }
        // 에픽/서펜이 살아 있는 상태는 live_list 가 비어 있으면 None 이다 — 실제 상태 출력
        println!("(참고) 게임 시작 직후엔 epic/serpen live_list 가 비어 있을 수 있다");
    }

    // ================ ③ Blackboard::is_recent_visible 120틱 ================
    println!("\n### O10-C Blackboard::is_recent_visible — last_visible[pos] + W >= tick");
    println!("tick\tlast_visible\tresult\texpect(W=120)");
    {
        // 관측자 = team0 Top 의 PlayerState, 대상 = team1 Top 의 챔피언(멀어서 안 보임)
        for &(tk, lv) in [(1000usize, 880usize), (1000, 879), (1000, 881), (1000, 1000),
                          (5000, 4880), (5000, 4879), (300, 179), (300, 180)].iter() {
            game.set_tick(tk);
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let target = cache.player_champion[1][0].expect("team1 champ 없음");
            let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
            // 대상(team1)의 pos0 슬롯
            bb[1].last_visible[0] = lv;
            let player = game.get_player_by_position(0usize, Position::Top).unwrap();
            let vis = game.is_visible(0usize, target.id);
            let r = bb[1].is_recent_visible(&game as &dyn AbstractGame, player, target);
            println!("{}\t{}\t{}\texpect={}\t(is_visible={})", tk, lv, r, lv + 120 >= tk, vis);
        }
    }

    // ================ ④ is_enemy_well_danger — version 무영향 재확인 ================
    println!("\n### O10-D is_enemy_well_danger(version, player, x, y) — version 0..5 동일한가");
    {
        let player0 = game.get_player_by_position(0usize, Position::Top).unwrap();
        let player1 = game.get_player_by_position(1usize, Position::Top).unwrap();
        let mut diff = 0usize; let mut n = 0usize;
        let mut sig0: Vec<bool> = Vec::new();
        for ver in 0..6usize {
            let mut v: Vec<bool> = Vec::new();
            for p in [player0, player1] {
                for xi in 0..9usize { for yi in 0..9usize {
                    let x = (xi as u64) * 120000; let y = (yi as u64) * 120000;
                    v.push(game_ai::is_enemy_well_danger(ver, p, x, y));
                }}
            }
            if ver == 0 { sig0 = v.clone() } else { if v != sig0 { diff += 1 } }
            n += 1;
        }
        println!("version 0..5 격자 {}칸 — 다른 버전 수 = {}", sig0.len(), diff);
        // 경계 정밀 확인: team0 의 적팀(=1) 우물 사각형 (0,800000)-(64000,960000)
        for &(x, y) in [(0u64, 800000u64), (0, 799999), (64000, 960000), (64001, 960000),
                        (0, 960000), (160000, 896000), (160001, 896000)].iter() {
            println!("  team0 player, ({}, {}) -> {}", x, y, game_ai::is_enemy_well_danger(0, player0, x, y));
        }
    }
    println!("\nDONE");
}
