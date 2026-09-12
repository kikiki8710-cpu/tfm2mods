#![allow(unused, dead_code, non_snake_case)]
//! 15차 배치 D 오라클 #1 — `_verify3\TEMPLATE.rs` 를 그대로 따른다(real_setting · init_tower 안 부름).
//!
//! 재는 것:
//!  A. 36 calculate_nexus_defense_count (pub) — 시작 상태(미니언 0·에픽 0) = 0 인지,
//!     `game.mode.epic_minion_buff_time[enemy]` 를 raw write 로 켰을 때 **enemy = 1-team 극성**대로
//!     team 쪽만 weak_lead_state 값이 되는지, tutorial=First(태그1) 이면 morgard_exists=false 로 0 인지.
//!  B. 37 i_am_chosen_defender (pub) — 미니언·푸셔가 없을 때 키 = (false, 0, dist²(ally,nexus), id) 라
//!     독립 재구현(수법 ⓓ)과 need_count 0..=5 × exclude 3종 × 팀2 × 포지션5 = 전수 대조.
//!  C. 38 can_tower_focused_when_battle (pub) — 적 타워마다 임계 = range+(lv-1)*growth+stat_range
//!     + t.radius + champ.radius + d + 15000 을 독립 계산해 dist = 임계-1 / 임계 / 임계+1 에서
//!     true/true/false 인지(`<=` 극성), 그리고 tower_attack_disable_tick=0 이면 전부 false 인지.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000;
    s.height = 960000;
    s.respawn_tick = 300;
    s.respawn_growth = 30;
    s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180;
    s.respawn_max = 2400;
    s.visible_distance = 130000;
    s.tick_per_second = 60;
    s.champion_radius = 10000;
    s.nexus_heal = 10;
    s.nexus_heal_2v2 = 10;
    s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100;
    s.kill_exp = 30;
    s.kill_exp_growth = 30;
    s.assist_exp_ratio = 40;
    s.kill_gold = 300;
    s.assist_gold = 100;
    s.start_gold = 500;
    s.gold_per_second = 7;
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400;
    s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200;
    s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600;
    s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700;
    s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15;
    s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}

const POSS: [Position; 5] = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];

pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80;
            st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, POSS[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

fn dist_sq(a: &Entity, b: &Entity) -> u64 {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    dx * dx + dy * dy
}

/// 37 독립 재구현(미니언 0 · 푸셔 0 가정 → 키 = (false, 0, dist², id))
fn my_chosen(cache: &AbstractGameWithCache, team: usize, pos: usize, need: usize, exclude: &[usize]) -> bool {
    if need == 0 { return false; }
    let nexus = match cache.nexus[team] { Some(n) => n, None => return false };
    let me = match cache.player_champion[team][pos] { Some(e) => e, None => return false };
    if exclude.contains(&me.id) { return false; }
    let my_key = (dist_sq(me, nexus), me.id);
    let mut better = 0usize;
    for p in 0..5 {
        let ally = match cache.player_champion[team][p] { Some(e) => e, None => continue };
        if ally.id == me.id { continue; }
        if exclude.contains(&ally.id) { continue; }
        if (dist_sq(ally, nexus), ally.id) < my_key { better += 1; }
    }
    better < need
}

fn ent_radius(e: &Entity) -> u64 {
    let m = e.stat_buff_cached.radius_mult as i64;
    if m == 0 { e.radius as u64 } else { ((e.radius as u64) * ((m + 100) as u64)) / 100 }
}

/// 세계 하나 = (setting, tutorial, epic 주입) 조합. 한 번에 만든다.
fn run_world(tag: &str, mut setting: GameSetting, tutorial: TutorialType, epic_enemy_of_team0: bool, disable_tick0: bool, weak_team: Option<usize>, ctx_tut_first: bool) {
    if disable_tick0 { setting.tower_attack_disable_tick = 0; }
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
        tutorial, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    if epic_enemy_of_team0 {
        // Game+0xed00 = mode(MobaMode) · MobaMode+0x240 = epic_minion_buff_time[2] (tcxdict 정본)
        // team0 의 적 = team1 → [1] 만 켠다 ⟹ team0 에서만 enemy_has_epic 이어야 한다(극성 검증)
        unsafe {
            let base = (&mut game as *mut Game as *mut u8).add(0xed00 + 0x240) as *mut usize;
            *base.add(1) = 1000;
        }
    }
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    if let Some(wt) = weak_team {
        // AbstractGameWithCache+0x21c0 = top_lead[2] (tcxdict 정본). 그 팀의 Top 라인만 0 = '약한 라인'
        unsafe { *((&mut cache as *mut AbstractGameWithCache as *mut u8).add(0x21c0 + wt * 8) as *mut usize) = 0; }
    }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    // ctx_tut_first: 세계는 None(넥서스 있음)으로 만들고 **data.context 만** tutorial=First 로 바꿔
    //   L2329 morgard_exists 게이트만 격리한다(First 세계는 넥서스가 없어 L2330 unwrap 에서 죽는다 — case2 실측)
    let ctx2 = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: if ctx_tut_first { TutorialType::First } else { tutorial }, trace_level: TraceLevel::Off,
    };
    let data = OperationData::new(&cache, &ctx2, &bb);
    println!("[{}]\tINTEG\ttowers={}\ttwin0={}\ttwin1={}\ttick={}\t(기대 16/2/2)",
             tag, game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len(), game.tick());
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);

    // ---------- A. 36 ----------
    for team in 0..2usize {
        let lead: Vec<usize> = [LineType::Top, LineType::Mid, LineType::Bottom].iter()
            .map(|l| cache.line_lead(team, *l)).collect();
        let weak = lead.iter().any(|v| *v == 0);
        let epic_enemy = epic_enemy_of_team0 && team == 0;
        let spawn_epic = matches!(ctx2.tutorial, TutorialType::None) ; // First~JungleOnly(1..=6) 이면 false, Line/Total 은 이 프로브에서 안 씀
        let expect = if spawn_epic && epic_enemy { if weak { 1 } else { 0 } } else { 0 };
        let player = game.get_player_by_position(team, Position::Mid).unwrap();
        let got = game_ai::plan_legacy::handler::calculate_nexus_defense_count(0, &mut rnd, player, &data, &mut dbg);
        println!("[{}]\tA36\tteam={}\tlead={:?}\tweak={}\tepic_enemy={}\tspawn_epic={}\texpect={}\tgot={}\t{}",
                 tag, team, lead, weak, epic_enemy, spawn_epic, expect, got, if got == expect { "MATCH" } else { "MISMATCH" });
    }

    // ---------- B. 37 ----------
    let mut b_match = 0usize; let mut b_total = 0usize;
    for team in 0..2usize {
        let nexus = cache.nexus[team].unwrap();
        let mut ids: Vec<(u64, usize, usize)> = Vec::new();
        for p in 0..5 { if let Some(e) = cache.player_champion[team][p] { ids.push((dist_sq(e, nexus), e.id, p)); } }
        ids.sort();
        println!("[{}]\tB37\tteam={}\t(dist²,id,pos) 오름차순={:?}", tag, team, ids);
        let exclude_sets: Vec<Vec<usize>> = vec![vec![], vec![ids[0].1], vec![ids[0].1, ids[1].1]];
        for pos in 0..5usize {
            let player = game.get_player_by_position(team, POSS[pos]).unwrap();
            let me_id = cache.player_champion[team][pos].unwrap().id;
            for (ei, ex) in exclude_sets.iter().enumerate() {
                for need in 0..=5usize {
                    let got = game_ai::plan_legacy::old::i_am_chosen_defender(player, &data, need, ex);
                    let exp = my_chosen(&cache, team, pos, need, ex);
                    b_total += 1; if got == exp { b_match += 1; }
                    if got != exp || (need <= 1 && ei == 0) {
                        println!("[{}]\tB37\tteam={} pos={} me={} ex={:?} need={}\tgot={}\texp={}\t{}",
                                 tag, team, pos, me_id, ex, need, got, exp, if got == exp { "MATCH" } else { "MISMATCH" });
                    }
                }
            }
            // 내 id 가 exclude 에 있으면 즉시 false
            let ex_me = vec![me_id];
            let got = game_ai::plan_legacy::old::i_am_chosen_defender(player, &data, 5, &ex_me);
            b_total += 1; if got == false { b_match += 1; }
            println!("[{}]\tB37\tteam={} pos={} exclude=[me] need=5\tgot={}\texp=false\t{}", tag, team, pos, got, if !got { "MATCH" } else { "MISMATCH" });
        }
    }
    println!("[{}]\tB37\tSUMMARY\tmatch={}/{}", tag, b_match, b_total);

    // ---------- C. 38 ----------
    let mut c_match = 0usize; let mut c_total = 0usize;
    for team in 0..2usize {
        let enemy = 1 - team;
        let player = game.get_player_by_position(team, Position::Mid).unwrap();
        let champ = cache.player_champion[team][Position::Mid.as_index()].unwrap();
        let towers = cache.towers(enemy, &pool);
        let n_wo = cache.iter_towers_without_nexus(enemy).count();
        let mut n_nexus = 0usize;
        for t in towers.iter() { if format!("{:?}", t.ty).starts_with("Nexus") { n_nexus += 1; } }
        println!("[{}]\tC38\tteam={}\ttowers({})={}\titer_towers_without_nexus={}\tnexus_in_towers={}",
                 tag, team, enemy, towers.len(), n_wo, n_nexus);
        for (ti, t) in towers.iter().enumerate() {
            let att = match &t.attack_effect { Some(a) => a, None => { println!("[{}]\tC38\ttower#{} attack_effect=None", tag, ti); continue; } };
            for d in [0u64, 5000u64] {
                let thr = att.range + (t.level as u64 - 1) * att.growth_range + t.stat_buff_cached.range as u64
                    + ent_radius(t) + ent_radius(champ) + d + 15000;
                // 타워에서 +x 방향으로 정확히 thr-1 / thr / thr+1 떨어진 점 (y 동일 → distance 가 정수 정확)
                for (k, off) in [(0i64, -1i64), (1, 0), (2, 1)] {
                    let x = (t.x as i64 + thr as i64 + off) as u64;
                    let y = t.y;
                    let dd = game_core::utils::distance(x, y, t.x, t.y);
                    let got = game_ai::can_tower_focused_when_battle(&ctx, &cache, player, x, y, d);
                    let exp = if disable_tick0 { false } else { dd <= thr };
                    c_total += 1; if got == exp { c_match += 1; }
                    if ti < 2 || got != exp {
                        println!("[{}]\tC38\tteam={} tower#{} lv={} range={} growth={} statr={} trad={} crad={} d={}\tthr={}\tdist={}\tgot={}\texp={}\t{}",
                                 tag, team, ti, t.level, att.range, att.growth_range, t.stat_buff_cached.range, ent_radius(t), ent_radius(champ), d, thr, dd, got, exp,
                                 if got == exp { "MATCH" } else { "MISMATCH" });
                    }
                }
            }
        }
    }
    println!("[{}]\tC38\tSUMMARY\tmatch={}/{}", tag, c_match, c_total);
}

fn main() {
    let which: usize = std::env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(0);
    match which {
        0 => run_world("base", real_setting(), TutorialType::None, false, false, None, false),
        1 => run_world("epic1", real_setting(), TutorialType::None, true, false, None, false),
        2 => run_world("tutFirst+epic", real_setting(), TutorialType::First, true, false, None, false),
        3 => run_world("disable0", real_setting(), TutorialType::None, false, true, None, false),
        4 => run_world("epic1+weak0", real_setting(), TutorialType::None, true, false, Some(0), false),
        5 => run_world("epic1+weak1", real_setting(), TutorialType::None, true, false, Some(1), false),
        6 => run_world("noepic+weak0", real_setting(), TutorialType::None, false, false, Some(0), false),
        7 => run_world("ctxFirst+epic1+weak0", real_setting(), TutorialType::None, true, false, Some(0), true),
        _ => {}
    }
}
