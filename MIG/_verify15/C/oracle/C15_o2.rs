#![allow(unused, dead_code, non_snake_case)]
//! C15_o2 — 15차 배치C 오라클②: #34 has_line_defense_threat **양성 케이스 주입**(수법 ⑦ 엔티티 복제·캐시 재삽입)
//!   #31 v3_epic_formation_role · #32 serpen_giveup_chat_reason · #34 has_line_defense_threat
//! (#30 objective_is_damaged = in:game_ai, #33 v2_obj_restore_safe = in:handler → pub 진입 경로 없음, 제외)
//!
//! 세팅 = TEMPLATE.rs `real_setting()` + 4차 배치B 미니언 레시피(B7_o2). `init_tower/init_nexus` 안 부름(TEMPLATE ②).
//! TLS 메모: 세 함수 모두 `thread_local` 호출 없음(IR 본문 grep 0건) — 한 프로세스 반복 측정 가능.
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::rule_scope as rs;
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
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
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
    // 미니언을 실제로 세우는 값 (4차 배치B 레시피)
    s.minion_wave_setting.start_tick = 10;
    s.minion_wave_setting.tick_per_wave = 660;
    s.minion_wave_setting.melee_count = 2;
    s.minion_wave_setting.range_count = 1;
    s.minion_wave_setting.tick_per_spawn = 30;
    s.minion_wave_setting.growth_start_tick = 1800;
    s.minion_wave_setting.growth_tick = 1800;
    s.minion_wave_setting.growth_tick_2v2 = 400;
    s.minion_wave_setting.growth_tick_3v3 = 800;
    s.minion_wave_setting.growth_end_tick = 24000;
    s.minion_wave_setting.growth_end_tick_2v2 = 30000;
    s.minion_wave_setting.growth_end_tick_3v3 = 24000;
    s.minion_wave_setting.exp_range = 150000;
    s.minion_wave_setting.exp_decay2 = 100;
    s.minion_wave_setting.exp_decay3 = 80;
    s.minion_wave_setting.exp_decay4 = 60;
    s.melee_minion.stat.attack = 10; s.melee_minion.stat.hp = 400;
    s.melee_minion.stat.move_speed = 800;
    s.melee_minion.growth.attack = 1; s.melee_minion.growth.hp = 30;
    s.melee_minion.growth.move_speed = 10;
    s.melee_minion.attack.attack_ratio = 100; s.melee_minion.attack.range = 3000;
    s.melee_minion.attack.cooltime = 30; s.melee_minion.attack.duration = 24;
    s.melee_minion.attack.start_timing = 16;
    s.melee_minion.exp = 40; s.melee_minion.gold = 20;
    s.range_minion.stat.attack = 15; s.range_minion.stat.hp = 250;
    s.range_minion.stat.move_speed = 800;
    s.range_minion.growth.attack = 1; s.range_minion.growth.hp = 20;
    s.range_minion.growth.move_speed = 10;
    s.range_minion.attack.range = 35000; s.range_minion.attack.speed = 3000;
    s.range_minion.attack.cooltime = 40; s.range_minion.attack.duration = 24;
    s.range_minion.attack.start_timing = 16;
    s.range_minion.exp = 30; s.range_minion.gold = 20;
    s
}

fn lt_code(l: &LineType) -> u8 { match l { LineType::Top => 0, LineType::Mid => 1, LineType::Bottom => 2 } }
fn lt_from(c: u8) -> LineType { match c { 0 => LineType::Top, 1 => LineType::Mid, _ => LineType::Bottom } }
fn pos_from(p: usize) -> Position {
    [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support][p]
}
fn tut_from(t: u8) -> TutorialType {
    match t {
        0 => TutorialType::None, 1 => TutorialType::First, 2 => TutorialType::TopSolo,
        3 => TutorialType::Bottom, 4 => TutorialType::MidSolo, 5 => TutorialType::MidBottom,
        6 => TutorialType::JungleOnly, 7 => TutorialType::Line, _ => TutorialType::Total,
    }
}
fn fmt_v3(r: &Option<old::V3EpicFormation>) -> String {
    match r { None => "None".into(), Some(f) => format!("({},{})", f.is_split as u8, lt_code(&f.line)) }
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}",
             setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0,
             setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let mkctx = |tut: TutorialType| GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off,
    };
    let ctx = mkctx(TutorialType::None);

    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, pos_from(p), st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(9);
    for _ in 0..600usize {
        let mut fd: Option<&mut GameFrameData> = None;
        game.run_tick(&ctx, &mut rnd2, &mut fd);
    }
    println!("towers\t{}\tminions\t{}", game.world.tower_ids.len(), game.world.minion_ids.len());

    let mut total_bad = 0usize;


    // ══════════════════ #34b 양성 주입: 복제 미니언의 nearest_enemy = tower_id ══════════════════
    // o1 에서 자연 발생 미니언은 전부 nearest=None/미니언 id 라 any-분기가 한 번도 true 가 안 됐다.
    // 여기서는 적 미니언 하나를 복제해 nearest_enemy 와 좌표를 조작한 뒤 cache.mid_minions[enemy] 에 꽂는다.
    {
        println!("\n#34b\tteam\tline\tinj\ttower\tgot\texpect\tverdict");
        let mut n = 0usize; let mut bad = 0usize;
        let towers: Vec<usize> = game.world.tower_ids.iter().cloned().collect();
        let tw0 = towers[0]; let tw1 = towers[1];
        for team in 0..2usize {
            let enemy = 1 - team;
            // 기준 미니언(적팀) 하나 + 각 라인 근처 좌표 표본
            let (base, near_pts) = {
                let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                let b = c0.iter_minions(enemy).next().unwrap().clone();
                let mut pts: [Option<(u64, u64)>; 3] = [None, None, None];
                for m in c0.iter_minions(enemy) {
                    for lc in 0u8..3 { if pts[lc as usize].is_none() && is_near_line(&ctx, m.x, m.y, lt_from(lc)) { pts[lc as usize] = Some((m.x, m.y)); } }
                }
                (b, pts)
            };
            println!("34b\tteam={}\tnear_pts={:?}", team, near_pts);
            // 주입 변형 6종
            //  0: Minion, nearest=Some(tw0), 좌표=라인 근처            → 그 라인에서 tw0 이면 true
            //  1: Minion, nearest=Some(tw1), 좌표=라인 근처            → tw1 이면 true, tw0 이면 false
            //  2: Minion, nearest=None,      좌표=라인 근처            → false
            //  3: Minion, nearest=Some(tw0), 좌표=(0,0) 라인 밖         → false (is_near_line 게이트)
            //  4: Tower 타입 복제, nearest=Some(tw0), 좌표=라인 근처    → false (ty != Minion)
            for lc in 0u8..3 {
                let line = lt_from(lc);
                let Some((nx, ny)) = near_pts[lc as usize] else { println!("34b\tline {} 근처 표본 없음 — 건너뜀", lc); continue; };
                let tower_base = { let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx); c0.twin_towers[enemy][0].clone() };
                for inj in 0u8..5 {
                    let mut e = if inj == 4 { tower_base.clone() } else { base.clone() };
                    let (ex, ey) = if inj == 3 { (0u64, 0u64) } else { (nx, ny) };
                    e.x = ex; e.y = ey;
                    let want = match inj { 0 | 3 | 4 => Some(tw0), 1 => Some(tw1), _ => None };
                    match &mut e.ty {
                        EntityType::Minion { info } => { info.nearest_enemy = want; }
                        EntityType::Tower { info } => { info.nearest_enemy = want.map(|t| (t, 0usize)); }
                        _ => {}
                    }
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    cache.mid_minions[enemy].push(&e);
                    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    { let msx = match line { LineType::Top => &mut bb[team].top_minion_state, LineType::Mid => &mut bb[team].mid_minion_state, LineType::Bottom => &mut bb[team].bottom_minion_state };
                      msx.from_mid = -3001; }
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let ps = game.get_player_by_position(team, Position::Mid).unwrap();
                    let reimpl = |tower: usize| -> bool {
                        cache.iter_minions(enemy).any(|m| is_near_line(&ctx, m.x, m.y, line)
                            && matches!(m.ty, EntityType::Minion { .. }) && m.nearest_enemy() == Some(tower))
                    };
                    for &tw in [tw0, tw1].iter() {
                        let got = old::has_line_defense_threat(ps, &data, line, tw);
                        let exp = reimpl(tw);
                        let ok = got == exp;
                        n += 1; if !ok { bad += 1; }
                        println!("34b\t{}\t{}\t{}\t{}\t{}\t{}\t{}", team, lc, inj, tw, got, exp, if ok {"MATCH"} else {"MISMATCH"});
                    }
                    // 같은 주입 상태에서 웨이브 게이트 OFF → 반드시 false
                    {
                        let mut bb2: [Blackboard; 2] = [Default::default(), Default::default()];
                        let data2 = OperationData::new(&cache, &ctx, &bb2);
                        let got = old::has_line_defense_threat(ps, &data2, line, tw0);
                        n += 1; if got { bad += 1; }
                        println!("34b\t{}\t{}\t{}(gateOFF)\t{}\t{}\t{}\t{}", team, lc, inj, tw0, got, false, if !got {"MATCH"} else {"MISMATCH"});
                    }
                }
            }
        }
        println!("#34b_SUMMARY\tn={}\tbad={}", n, bad);
        total_bad += bad;
    }
    println!("\n#TOTAL_BAD\t{}", total_bad);
}
