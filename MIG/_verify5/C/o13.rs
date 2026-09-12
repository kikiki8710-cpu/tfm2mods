#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치C 오라클 #1 — specs[13] target_bush_v30 전표 + specs[14] update / target_bush_v41
//! 정본 템플릿 `_verify3\TEMPLATE.rs` 복사 기반(real_setting + init_tower 미호출).
//!
//! 관측 경로
//!  · specs[13] : `LineGankCoverPlan::sub_plan`(pub) 은 `target_bush_v30` 반환값을
//!                `SubPlan::Hide{bush}`(태그 9, bush@+0x8) 로 그대로 내보내는 얇은 래퍼다
//!                (`_gaibc/m10.ll:11755~11810`, tail call 11791) ⟹ 사설 함수의 반환을 직접 읽는다.
//!  · specs[14] : `LineGankerPlan::update`(pub) — phase/chats 가 pub 이라 직접 관측.
//!                `LineGankerPlan::sub_plan`(pub) → target_bush_v41 (lead 기반).
use game_core::*;
use game_ai::plan_legacy::old::{LineGankCoverPlan, LineGankerPlan, LineGankerPhase};
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
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}", ok, s.width, s.height, s.tick_per_second, s.champion_radius);
    ok
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

/// SubPlan(72B) 의 태그(+0x0)와 두번째 워드(+0x8)를 날바이트로 읽는다.
fn sp_raw(sp: &game_ai::plan_legacy::sub_plan::SubPlan) -> (u64, u64, u8, u8, u8) {
    unsafe {
        let p = sp as *const _ as *const u8;
        (*(p as *const u64), *(p.add(8) as *const u64), *p.add(16), *p.add(17), *p.add(18))
    }
}

/// specs[13] / specs[14] 이 주장하는 target_bush_v30 전표.
/// tower=None → 타워 전멸 / Some((is_t2, near_enemy))
/// s = !is_top_side (= 봇 사이드)
fn predict_v30(line: u8, team: usize, tower: Option<(bool, bool)>, s: bool) -> u64 {
    match tower {
        None => match line {
            0 => if team == 0 { 2 } else { 16 },
            1 => if team == 0 { 4 } else { 17 },
            _ => if team == 0 { 9 } else { 21 },
        },
        Some((t2, ne)) => match line {
            0 => if t2 { if team == 0 { 3 } else { 6 } }
                 else if ne { if team == 0 { 6 } else { 3 } }
                 else { if team == 0 { 3 } else { 6 } },
            1 => if t2 { if s { if team == 0 { 13 } else { 18 } } else { if team == 0 { 8 } else { 12 } } }
                 else { if s { 14 } else { 11 } },
            _ => if t2 { if team == 0 { 15 } else { 20 } }
                 else if ne { if team == 0 { 20 } else { 15 } }
                 else { if team == 0 { 15 } else { 20 } },
        },
    }
}

/// specs[14] knobs 의 target_bush_v41 전표 주장
fn predict_v41(line: u8, team: usize, lead: usize, s: bool) -> u64 {
    match line {
        0 => { let t0 = [16u64, 6, 3, 3, 3, 2, 2]; let t1 = [2u64, 3, 6, 6, 6, 16, 16];
               if team == 0 { t0[lead] } else { t1[lead] } }
        2 => { let t0 = [21u64, 20, 15, 15, 15, 9, 7]; let t1 = [9u64, 15, 20, 20, 20, 21, 23];
               if team == 0 { t0[lead] } else { t1[lead] } }
        _ => {
            if lead == 0 || lead >= 5 {
                if team == 0 { if s { 21 } else { 17 } } else { if s { 9 } else { 4 } }
            } else { if s { 14 } else { 11 } }
        }
    }
}
/// Mid 의 lead 5·6 은 team 이 반대쪽으로 붙는다는 주장(표 원문): table[5],[6] = team0 ? (s?9:4) : (s?21:17)
fn predict_v41_mid(team: usize, lead: usize, s: bool) -> u64 {
    match lead {
        0 => if team == 0 { if s { 21 } else { 17 } } else { if s { 9 } else { 4 } },
        1..=4 => if s { 14 } else { 11 },
        _ => if team == 0 { if s { 9 } else { 4 } } else { if s { 21 } else { 17 } },
    }
}

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
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
    let game = mkgame(&setting, &ms, &map, &ctx);

    // ---- 셀 표본 고르기: bush id 별 첫 셀 + bush 0 셀 몇 개
    let mut cells: Vec<(usize, usize, u64)> = Vec::new();
    let mut seen = [false; 64];
    for cy in 0..30usize {
        for cx in 0..30usize {
            let b = map.bushes[cy][cx] as u64;
            if b != 0 && (b as usize) < 64 && !seen[b as usize] { seen[b as usize] = true; cells.push((cx, cy, b)); }
        }
    }
    // 봇사이드/탑사이드 각각 bush 0 셀 하나
    for &(cx, cy) in [(1usize, 1usize), (28, 28)].iter() {
        cells.push((cx, cy, map.bushes[cy][cx] as u64));
    }
    println!("cells\t{}", cells.len());
    for &(cx, cy, b) in cells.iter() { print!("{}:{} ", b, format!("({},{})", cx, cy)); }
    println!();

    // ---- 엔티티 풀: 타워 4종 + 셀별 챔피언
    let mut ents: Vec<Entity> = Vec::new();
    {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bt = c0.top_tower[0].expect("top_tower[0] 없음").clone();
        let bc = c0.player_champion[0][0].expect("player_champion[0][0] 없음").clone();
        for t2 in [false, true] {
            for ne in [false, true] {
                let mut e = bt.clone();
                if let EntityType::Tower { info: ref mut i } = e.ty {
                    i.ty = if t2 { TowerType::Top2 } else { TowerType::Top };
                    i.nearest_enemy = if ne { Some((1usize, 2usize)) } else { None };
                } else { panic!("top_tower 가 EntityType::Tower 가 아니다"); }
                ents.push(e);
            }
        }
        println!("tower_variants\t{}\t(0=t1/ne0 1=t1/ne1 2=t2/ne0 3=t2/ne1)", ents.len());
        for &(cx, cy, _b) in cells.iter() {
            let mut e = bc.clone();
            e.x = (cx as u64) * 32000 + 16000;
            e.y = (cy as u64) * 32000 + 16000;
            e.stat_cached.hp = 1000;
            e.hp = 1000; // 100%
            ents.push(e);
        }
    }
    let ncell = cells.len();
    let champ0 = 4usize; // ents[4..] = 셀별 챔피언

    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let gd: game_ai::GoalData = Default::default();
    let ps: PositioningScoreData = Default::default();

    // =============== specs[13] : target_bush_v30 전표 ===============
    println!("\n### S13 target_bush_v30 (LineGankCoverPlan::sub_plan 경유)");
    println!("line\tteam\ttower\ts\tcx\tcy\tgame\tmine\tmatch\ttag\toutline\tcheckmove\tspotted");
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let mut m13 = 0usize; let mut n13 = 0usize;
    // tower 상태: None + 4변형
    for li in 0..3usize {
        for team in 0..2usize {
            for tst in 0..5usize {
                for ci in 0..ncell {
                    // Mid 가 아니면 셀은 무관 → 첫 2개만 돌려 시간 절약
                    if li != 1 && ci >= 2 { continue; }
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    // 라인별 타워 슬롯 초기화
                    for t in 0..2 {
                        cache.top_tower[t] = None; cache.top_tower2[t] = None;
                        cache.mid_tower[t] = None; cache.mid_tower2[t] = None;
                        cache.bottom_tower[t] = None; cache.bottom_tower2[t] = None;
                    }
                    let towdesc: Option<(bool, bool)> = if tst == 0 { None } else {
                        Some(((tst - 1) >= 2, ((tst - 1) & 1) == 1))
                    };
                    if let Some(_) = towdesc {
                        let e = &ents[tst - 1];
                        match li { 0 => cache.top_tower[team] = Some(e),
                                   1 => cache.mid_tower[team] = Some(e),
                                   _ => cache.bottom_tower[team] = Some(e) }
                    }
                    let champ = &ents[champ0 + ci];
                    cache.player_champion[team][0] = Some(champ);
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let player = game.get_player_by_position(team, Position::Top).expect("player 없음");
                    let plan = LineGankCoverPlan::new(lines[li], 0);
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
                    let mut dbg: DebugFrameData = Default::default();
                    let sp = plan.sub_plan(3usize, &mut rnd, player, &data, &mut dbg);
                    let (tag, bush, ol, cm, sm) = sp_raw(&sp);
                    let s = !game_core::is_top_side(&ctx, champ.x, champ.y);
                    let mine = predict_v30(li as u8, team, towdesc, s);
                    let hit = bush == mine;
                    if hit { m13 += 1 } ; n13 += 1;
                    if !hit || ci < 2 {
                        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                            li, team, tst, s, cells[ci].0, cells[ci].1, bush, mine,
                            if hit { "OK" } else { "***DIFF***" }, tag, ol, cm, sm);
                    }
                }
            }
        }
    }
    println!("S13_MATCH\t{}/{}", m13, n13);

    // =============== specs[14] : update HP 임계 ===============
    println!("\n### S14 update HP 임계(41) — hp_ratio = hp*100/max_hp");
    println!("hp\tratio\tphase\tchats\texpect_cancel");
    {
        let mut hpents: Vec<Entity> = Vec::new();
        {
            let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bc = c0.player_champion[0][0].unwrap().clone();
            for hp in [0u64, 1, 399, 400, 405, 409, 410, 411, 500, 1000] {
                let mut e = bc.clone();
                e.stat_cached.hp = 1000; e.hp = hp as usize;
                // bush 가 절대 안 맞는 자리(bush 0)로 둬서 취소사유②를 배제
                e.x = 1 * 32000 + 16000; e.y = 1 * 32000 + 16000;
                hpents.push(e);
            }
        }
        for (k, hp) in [0usize, 1, 399, 400, 405, 409, 410, 411, 500, 1000].iter().enumerate() {
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][0] = Some(&hpents[k]);
            let data = OperationData::new(&cache, &ctx, &bb);
            let player = game.get_player_by_position(0usize, Position::Top).unwrap();
            let mut g = LineGankerPlan::new(LineType::Top, 100, 200);
            let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
            let mut dbg: DebugFrameData = Default::default();
            g.update(3usize, &mut rnd, player, &data, &gd, &ps, &mut dbg);
            let ratio = hp * 100 / 1000;
            println!("{}\t{}\t{:?}\t{:?}\t{}", hp, ratio, g.phase, g.chats, ratio < 41);
        }
    }

    // =============== specs[14] : 취소사유② (bush 도착 + 적없음) ===============
    println!("\n### S14 update 취소사유2 — champ_bush == target_bush_v30(ganker판) && !has_near_line_enemy");
    println!("line\tteam\ttower\ts\tcx\tcy\tcellbush\tmine_v30\tpred_cancel\tphase\tchats\tmatch");
    let mut m14 = 0usize; let mut n14 = 0usize;
    for li in 0..3usize {
        for team in 0..2usize {
            for tst in 0..5usize {
                for ci in 0..ncell {
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    for t in 0..2 {
                        cache.top_tower[t] = None; cache.top_tower2[t] = None;
                        cache.mid_tower[t] = None; cache.mid_tower2[t] = None;
                        cache.bottom_tower[t] = None; cache.bottom_tower2[t] = None;
                    }
                    let towdesc: Option<(bool, bool)> = if tst == 0 { None } else {
                        Some(((tst - 1) >= 2, ((tst - 1) & 1) == 1))
                    };
                    if towdesc.is_some() {
                        let e = &ents[tst - 1];
                        match li { 0 => cache.top_tower[team] = Some(e),
                                   1 => cache.mid_tower[team] = Some(e),
                                   _ => cache.bottom_tower[team] = Some(e) }
                    }
                    let champ = &ents[champ0 + ci];
                    cache.player_champion[team][0] = Some(champ);
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let player = game.get_player_by_position(team, Position::Top).unwrap();
                    let mut g = LineGankerPlan::new(lines[li], 100, 200);
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
                    let mut dbg: DebugFrameData = Default::default();
                    g.update(3usize, &mut rnd, player, &data, &gd, &ps, &mut dbg);
                    let s = !game_core::is_top_side(&ctx, champ.x, champ.y);
                    let mine = predict_v30(li as u8, team, towdesc, s);
                    let cellbush = cells[ci].2;
                    let pred = cellbush == mine;
                    let got = format!("{:?}", g.phase) == "Cancel";
                    let hit = pred == got;
                    if hit { m14 += 1 }; n14 += 1;
                    if !hit || pred {
                        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}\t{}",
                            li, team, tst, s, cells[ci].0, cells[ci].1, cellbush, mine, pred,
                            g.phase, g.chats, if hit { "OK" } else { "***DIFF***" });
                    }
                }
            }
        }
    }
    println!("S14_CANCEL2_MATCH\t{}/{}", m14, n14);

    // =============== specs[14] : target_bush_v41 (lead) — Mid s=true 공백 채우기 ===============
    println!("\n### S14 target_bush_v41 (LineGankerPlan::sub_plan 경유) — lead 0..6 × s");
    println!("line\tteam\tlead\ts\tgame\tmine\tmatch");
    let mut m41 = 0usize; let mut n41 = 0usize;
    // s=false 셀 / s=true 셀 하나씩 찾기
    let mut ci_top = usize::MAX; let mut ci_bot = usize::MAX;
    for ci in 0..ncell {
        let e = &ents[champ0 + ci];
        let s = !game_core::is_top_side(&ctx, e.x, e.y);
        if s && ci_bot == usize::MAX { ci_bot = ci }
        if !s && ci_top == usize::MAX { ci_top = ci }
    }
    println!("ci_top={} ci_bot={}", ci_top, ci_bot);
    for li in 0..3usize {
        for team in 0..2usize {
            for lead in 0..7usize {
                for &(ci, s) in [(ci_top, false), (ci_bot, true)].iter() {
                    if ci == usize::MAX { continue }
                    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
                    let champ = &ents[champ0 + ci];
                    cache.player_champion[team][0] = Some(champ);
                    cache.top_lead[team] = lead; cache.mid_lead[team] = lead; cache.bottom_lead[team] = lead;
                    let data = OperationData::new(&cache, &ctx, &bb);
                    let player = game.get_player_by_position(team, Position::Top).unwrap();
                    let g = LineGankerPlan::new(lines[li], 100, 200);
                    let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
                    let mut dbg: DebugFrameData = Default::default();
                    let sp = g.sub_plan(3usize, &mut rnd, player, &data, &mut dbg);
                    let (tag, bush, ol, cm, sm) = sp_raw(&sp);
                    let mine = if li == 1 { predict_v41_mid(team, lead, s) } else { predict_v41(li as u8, team, lead, s) };
                    let hit = bush == mine;
                    if hit { m41 += 1 }; n41 += 1;
                    println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\ttag={} ol={} cm={} sm={}",
                        li, team, lead, s, bush, mine, if hit { "OK" } else { "***DIFF***" }, tag, ol, cm, sm);
                }
            }
        }
    }
    println!("S14_V41_MATCH\t{}/{}", m41, n41);
    println!("\nDONE setting_ok={}", ok);
}
