#![allow(unused, dead_code, non_snake_case)]
//! C15_o1 — 15차 배치C 오라클: pub 3함수 **독립 재구현 대조(TEMPLATE 수법 ⓓ)**
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

    // ══════════════════ #31 v3_epic_formation_role ══════════════════
    {
        println!("\n#31\tcase\tteam\tpos\tmu\tfar\tabsent\tgather\tgot\texpect\tverdict");
        let mut n = 0usize; let mut bad = 0usize;
        let le = [rs::line_exists(&ctx, LineType::Top), rs::line_exists(&ctx, LineType::Mid), rs::line_exists(&ctx, LineType::Bottom)];
        println!("31\tline_exists\tTop={}\tMid={}\tBottom={}", le[0], le[1], le[2]);
        // far_line 축: (epic_tick, serpen_tick) 3벌 — epic>serpen → Top(0) / 그 외 Bottom(2)
        let fars: [(usize, usize, u8); 3] = [(1000, 500, 0), (500, 1000, 2), (700, 700, 2)];
        // 부재 축: 없음 / 팀 t 의 pos a 없음 (a = 0..5)
        for &(et, st_, far_exp) in fars.iter() {
            game.mode.jungle_runner.epic.next_respawn_tick = et;
            game.mode.jungle_runner.serpen.next_respawn_tick = st_;
            for team in 0..2usize {
                for absent in 0..6usize {          // 5 = 아무도 없음(전원 존재)
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    if absent < 5 { cache.player_champion[team][absent] = None; }
                    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    let data = OperationData::new(&cache, &ctx, &bb);
                    for pos in 0..5usize {
                        let ps = game.get_player_by_position(team, pos_from(pos)).unwrap();
                        // 기준선: Gather = press_line(None)
                        let gather = old::v3_epic_formation_role(MorgardUseStrategy::Gather, pos_from(pos), ps, &data);
                        let gs = fmt_v3(&gather);
                        // 1) Gather — is_split 은 반드시 false(None 이거나)
                        {
                            n += 1;
                            let ok = gather.as_ref().map(|f| !f.is_split).unwrap_or(true);
                            if !ok { bad += 1; }
                            println!("31\tG\t{}\t{}\tGather\t{}\t{}\t{}\t{}\t{}\t{}", team, pos, far_exp, absent, gs, gs, "is_split=false", if ok {"MATCH"} else {"MISMATCH"});
                        }
                        // 2) Split14{position: sp}
                        for sp in 0..5usize {
                            let got = old::v3_epic_formation_role(MorgardUseStrategy::Split14 { position: pos_from(sp) }, pos_from(pos), ps, &data);
                            let g = fmt_v3(&got);
                            let (exp, ok) = if sp == pos && le[far_exp as usize] {
                                (format!("(1,{})", far_exp), g == format!("(1,{})", far_exp))
                            } else {
                                // press_line(Some(far)) — is_split false, line != far
                                let ok = match &got { None => true, Some(f) => !f.is_split && lt_code(&f.line) != far_exp };
                                (format!("fallback!={}", far_exp), ok)
                            };
                            n += 1; if !ok { bad += 1; }
                            println!("31\tS14\t{}\t{}\tSplit14({})\t{}\t{}\t{}\t{}\t{}\t{}", team, pos, sp, far_exp, absent, gs, g, exp, if ok {"MATCH"} else {"MISMATCH"});
                        }
                        // 3) Split131{p1, p2}
                        for p1 in 0..5usize { for p2 in 0..5usize {
                            let got = old::v3_epic_formation_role(MorgardUseStrategy::Split131 { position1: pos_from(p1), position2: pos_from(p2) }, pos_from(pos), ps, &data);
                            let g = fmt_v3(&got);
                            let p1_absent = absent == p1; let p2_absent = absent == p2;
                            let exp: String = if le[0] && (p1 == pos || (p2 == pos && p1_absent)) { "(1,0)".into() }
                                else if le[2] && (p2 == pos || (p1 == pos && p2_absent)) { "(1,2)".into() }
                                else if le[1] { "(0,1)".into() }
                                else { gs.clone() };
                            let ok = g == exp;
                            n += 1; if !ok { bad += 1; }
                            println!("31\tS131\t{}\t{}\tSplit131({},{})\t{}\t{}\t{}\t{}\t{}\t{}", team, pos, p1, p2, far_exp, absent, gs, g, exp, if ok {"MATCH"} else {"MISMATCH"});
                        }}
                    }
                }
            }
        }
        println!("#31_SUMMARY\tn={}\tbad={}", n, bad);
        total_bad += bad;
    }

    // ══════════════════ #32 serpen_giveup_chat_reason ══════════════════
    {
        println!("\n#32\ttut\tlive\tteam\tmy\tenemy\tgot\texpect\tverdict");
        let mut n = 0usize; let mut bad = 0usize;
        for tut in 0u8..9 {
            let ctx2 = mkctx(tut_from(tut));
            let se = rs::serpen_exists(&ctx2);
            for live in [0usize, 1, 2] {
                game.mode.jungle_runner.serpen.live_list.clear();
                for k in 0..live { game.mode.jungle_runner.serpen.live_list.push(900000 + k); }
                for team in 0..2usize {
                    for my in 0usize..3 { for en in 0usize..3 {
                        game.mode.serpen_count[team] = my;
                        game.mode.serpen_count[1 - team] = en;
                        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx2);
                        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
                        let data = OperationData::new(&cache, &ctx2, &bb);
                        let ps = game.get_player_by_position(team, Position::Mid).unwrap();
                        let got = old::serpen_giveup_chat_reason(ps, &data);
                        let g = match &got { None => "None", Some(SerpenGiveUpReason::StackAhead) => "StackAhead", Some(SerpenGiveUpReason::Outnumbered) => "Outnumbered" };
                        let tut_ok = matches!(tut, 0 | 5 | 7 | 8);
                        let exp = if !tut_ok || live == 0 { "None" } else if my <= en { "Outnumbered" } else { "StackAhead" };
                        let ok = g == exp;
                        n += 1; if !ok { bad += 1; }
                        println!("32\t{}(se={})\t{}\t{}\t{}\t{}\t{}\t{}\t{}", tut, se, live, team, my, en, g, exp, if ok {"MATCH"} else {"MISMATCH"});
                    }}
                }
            }
        }
        game.mode.jungle_runner.serpen.live_list.clear();
        println!("#32_SUMMARY\tn={}\tbad={}", n, bad);
        total_bad += bad;
    }

    // ══════════════════ #34 has_line_defense_threat ══════════════════
    {
        println!("\n#34a\tteam\tline\tfrom_mid\tcount\ttower\tgot\texpect\tverdict");
        let mut n = 0usize; let mut bad = 0usize;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        // 적 미니언 관측표: (id, x, y, is_minion, nearest_enemy, near Top/Mid/Bottom)
        for team in 0..2usize {
            let mut cnt = 0usize;
            for m in cache.iter_minions(1 - team) {
                if cnt < 6 {
                    println!("34m\tenemy_of_{}\tid={}\tx={}\ty={}\tminion={}\tnearest={:?}\tnear=({},{},{})", team, m.id, m.x, m.y,
                        matches!(m.ty, EntityType::Minion { .. }), m.nearest_enemy(),
                        is_near_line(&ctx, m.x, m.y, LineType::Top) as u8, is_near_line(&ctx, m.x, m.y, LineType::Mid) as u8, is_near_line(&ctx, m.x, m.y, LineType::Bottom) as u8);
                }
                cnt += 1;
            }
            println!("34m\tenemy_of_{}\ttotal={}", team, cnt);
        }
        let mut towers: Vec<usize> = game.world.tower_ids.iter().cloned().collect();
        towers.push(usize::MAX - 1);   // 존재하지 않는 id
        for team in 0..2usize {
            for lc in 0u8..3 {
                let line = lt_from(lc);
                // 독립 재구현: any(near_line && Minion && nearest_enemy == Some(tower))
                let reimpl = |tower: usize| -> bool {
                    cache.iter_minions(1 - team).any(|m| is_near_line(&ctx, m.x, m.y, line)
                        && matches!(m.ty, EntityType::Minion { .. }) && m.nearest_enemy() == Some(tower))
                };
                for &(fm, mc) in [(-3001i64, 0i32), (-3000, 0), (-2999, 0), (0, -3), (0, -2), (0, -1), (-3001, -3), (5000, 5)].iter() {
                    let pushed = fm < -3000 || mc < -2;
                    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
                    {
                        let msx = match line { LineType::Top => &mut bb[team].top_minion_state, LineType::Mid => &mut bb[team].mid_minion_state, LineType::Bottom => &mut bb[team].bottom_minion_state };
                        msx.from_mid = fm; msx.minion_count = mc;
                    }
                    // 상대 팀 blackboard 는 반대로 세팅(팀 인덱스 오귀속 감지)
                    {
                        let msy = match line { LineType::Top => &mut bb[1 - team].top_minion_state, LineType::Mid => &mut bb[1 - team].mid_minion_state, LineType::Bottom => &mut bb[1 - team].bottom_minion_state };
                        msy.from_mid = if pushed { 9999 } else { -9999 }; msy.minion_count = if pushed { 9 } else { -9 };
                    }
                    // 다른 라인 blackboard 도 반대로(라인 오귀속 감지)
                    for oc in 0u8..3 { if oc != lc {
                        let mo = match lt_from(oc) { LineType::Top => &mut bb[team].top_minion_state, LineType::Mid => &mut bb[team].mid_minion_state, LineType::Bottom => &mut bb[team].bottom_minion_state };
                        mo.from_mid = if pushed { 9999 } else { -9999 }; mo.minion_count = if pushed { 9 } else { -9 };
                    }}
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let ps = game.get_player_by_position(team, Position::Mid).unwrap();
                    for &tw in towers.iter() {
                        let got = old::has_line_defense_threat(ps, &data, line, tw);
                        let exp = pushed && reimpl(tw);
                        let ok = got == exp;
                        n += 1; if !ok { bad += 1; }
                        if !ok || exp || tw == towers[0] || tw == usize::MAX - 1 {
                            println!("34\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", team, lc, fm, mc, tw, got, exp, if ok {"MATCH"} else {"MISMATCH"});
                        }
                    }
                }
            }
        }
        println!("#34_SUMMARY\tn={}\tbad={}", n, bad);
        total_bad += bad;
    }
    println!("\n#TOTAL_BAD\t{}", total_bad);
}
