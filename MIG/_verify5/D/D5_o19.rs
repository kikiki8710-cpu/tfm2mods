#![allow(unused, dead_code, non_snake_case)]
//! 5차 배치D 오라클 #5 — `/specs[19]` `best_jungle_goal`(passive_jungle.rs:806) 진리표.
//! `pub` 이고 인자 7개 전부 구성 가능. 4차는 「10/10 MATCH」만 남겼고 축별 판별을 안 했다.
//! 여기서 재는 축: 후보집합·순서 / 제곱거리 키 / 동점 first-wins / champ=None 폴백 / now_camp /
//!                 not_cleared 전멸 폴백(next_respawn_tick) / version·rnd·team_plan·debug 무영향.
//! ★`cache.player_champion` 이 `pub` 이라 **내가 만든 Entity 를 꽂아** 좌표를 자유롭게 흔들 수 있다.
//! ★★TLS 주의: `MapDef::camp_pos` 도 `CAMP_POS_MEMO`(thread_local RefCell) 를 쓴다 — 맵을 바꿔 가며
//!    한 프로세스에서 재면 오염될 수 있다. 그래서 맵 교체 축은 **마지막에** 재고 별도로 표시한다.
use game_core::*;
use game_ai::plan_legacy::old::{best_jungle_goal, is_cleared};
use game_ai::plan_legacy::team_plan::TeamPlan;
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
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
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
const CAMPS: [JungleType; 4] = [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump];
fn d2(a: u64, b: u64) -> u64 { let d = if a > b { a - b } else { b - a }; d * d }

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
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
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let tp: TeamPlan = Default::default();

    // 캠프 좌표표 (knobs[7] / logic 834)
    for c in [JungleType::Rhino, JungleType::Mushroom, JungleType::Stump, JungleType::Bee,
              JungleType::Morgard, JungleType::Serpen] {
        let b = map.camp_pos(c, true);
        let r = map.camp_pos(c, false);
        let jb = c.camp_pos(true);
        println!("camp_pos\t{:?}\tblue={:?}\tred={:?}\tJungleType::camp_pos(blue)={:?}", c, b, r, jb);
    }
    // 리스폰 상태 (폴백 키 mem[8] JungleCampState+0x18)
    if let GameMode::Moba(m) = cache.game.get_game_mode() {
        for t in 0..2usize { for c in CAMPS {
            let st = m.jungle_runner.get_camp_state(t, c);
            println!("campstate\tteam{}\t{:?}\tnext_respawn_tick={}\trespawn_count={}\tlive={}\tis_blue_side={}",
                t, c, st.next_respawn_tick, st.respawn_count, st.live_list.len(), st.is_blue_side);
        } }
    } else { println!("campstate\tNOT_MOBA"); }

    let me0 = game.get_player_by_position(0, Position::Jungle).unwrap();
    // is_cleared 초기 상태
    {
        let data = OperationData::new(&cache, &ctx, &bb);
        for c in CAMPS {
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            println!("is_cleared\t{:?}\t{}", c, is_cleared(c, me0.info.team, 1, &mut r, me0, &data, &tp, 0, &mut dbg));
        }
    }

    let mut nrun = 0usize; let mut nmatch = 0usize;

    // 내 재현
    fn mine(version: usize, player: &PlayerState, data: &OperationData, tp: &TeamPlan,
            now_camp: Option<JungleType>, map: &MapDef, cache: &AbstractGameWithCache) -> JungleType {
        let team = player.info.team;
        let mut nc: Vec<JungleType> = Vec::new();
        for c in CAMPS {
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            if !is_cleared(c, team, version, &mut r, player, data, tp, 0, &mut dbg) { nc.push(c); }
        }
        if nc.is_empty() {
            if let GameMode::Moba(m) = cache.game.get_game_mode() {
                return *CAMPS.iter().min_by_key(|c| m.jungle_runner.get_camp_state(team, **c).next_respawn_tick).unwrap();
            }
            unreachable!();
        }
        let pos = player.info.position as usize;
        let champ = cache.player_champion[team][pos];
        if champ.is_none() {
            return now_camp.unwrap_or(JungleType::Rhino); // 랜덤 경로는 별도 취급
        }
        let e = champ.unwrap();
        *nc.iter().min_by_key(|c| {
            let (cx, cy) = map.camp_pos(**c, team == 0);
            d2(cx, e.x) + d2(cy, e.y)
        }).unwrap()
    }

    // ── (1) 챔프 좌표 축. cache.player_champion 에 내 Entity 를 꽂는다
    let src = cache.player_champion[0][1].unwrap();
    let mut my: Entity = unsafe { std::ptr::read(src as *const Entity) };
    let myref: &Entity = unsafe { &*(&my as *const Entity) };
    cache.player_champion[0][1] = Some(myref);
    let pts: [(u64, u64); 12] = [
        (15000, 913000), (480000, 480000), (100000, 800000), (800000, 100000),
        (0, 0), (959000, 959000), (300000, 700000), (700000, 300000),
        (200000, 200000), (600000, 600000), (50000, 500000), (500000, 50000)];
    for (x, y) in pts {
        my.x = x; my.y = y;
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, None, &mut dbg);
        let m = mine(1, me0, &data, &tp, None, &map, &cache);
        // 거리표
        let mut ds = String::new();
        for c in CAMPS {
            let (cx, cy) = map.camp_pos(c, me0.info.team == 0);
            ds.push_str(&format!("{:?}={} ", c, d2(cx, x) + d2(cy, y)));
        }
        nrun += 1; if g == m { nmatch += 1; }
        println!("pos\t({},{})\tgame={:?}\tmine={:?}\t{}\td2[{}]", x, y, g, m,
            if g == m { "MATCH" } else { "**DIFF**" }, ds.trim());
    }

    // ── (2) 동점 first-wins — 두 캠프에서 제곱거리가 같은 점을 계산해 넣는다
    {
        let (ax, ay) = map.camp_pos(JungleType::Rhino, true);
        let (bx, by) = map.camp_pos(JungleType::Mushroom, true);
        let mx = (ax + bx) / 2; let myy = (ay + by) / 2;
        my.x = mx; my.y = myy;
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, None, &mut dbg);
        let m = mine(1, me0, &data, &tp, None, &map, &cache);
        nrun += 1; if g == m { nmatch += 1; }
        let mut ds = String::new();
        for c in CAMPS { let (cx, cy) = map.camp_pos(c, true); ds.push_str(&format!("{:?}={} ", c, d2(cx, mx) + d2(cy, myy))); }
        println!("tie\tmid(Rhino,Mushroom)=({},{})\tgame={:?}\tmine={:?}\t{}\td2[{}]",
            mx, myy, g, m, if g == m { "MATCH" } else { "**DIFF**" }, ds.trim());
    }

    // ── (3) version / rnd 시드 / team_plan / debug 축 (무영향 주장)
    my.x = 480000; my.y = 480000;
    for v in [0usize, 1, 2, 3, 30, 50, 60] {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(v, &mut r, me0, &data, &tp, None, &mut dbg);
        println!("ver\t{}\tgame={:?}", v, g);
    }
    for sd in [0u64, 1, 7, 999, 123456789] {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(sd);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, None, &mut dbg);
        println!("seed\t{}\tgame={:?}", sd, g);
    }
    for nc in [None, Some(JungleType::Rhino), Some(JungleType::Mushroom), Some(JungleType::Stump),
               Some(JungleType::Bee), Some(JungleType::Morgard), Some(JungleType::Serpen)] {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, nc, &mut dbg);
        println!("nowcamp_champPRESENT\t{:?}\tgame={:?}", nc, g);
    }

    // ── (4) champ = None 폴백 (824~829)
    cache.player_champion[0][1] = None;
    for nc in [Some(JungleType::Rhino), Some(JungleType::Mushroom), Some(JungleType::Stump),
               Some(JungleType::Bee), Some(JungleType::Morgard), Some(JungleType::Serpen)] {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(7);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, nc, &mut dbg);
        let okk = g == nc.unwrap();
        nrun += 1; if okk { nmatch += 1; }
        println!("champNone_nowcamp\t{:?}\tgame={:?}\t{}", nc, g, if okk { "MATCH(그대로 유지)" } else { "**DIFF**" });
    }
    for sd in [0u64, 1, 2, 3, 7, 99, 12345] {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut r = rand::rngs::StdRng::seed_from_u64(sd);
        let mut dbg: DebugFrameData = Default::default();
        let g = best_jungle_goal(1, &mut r, me0, &data, &tp, None, &mut dbg);
        println!("champNone_random\tseed={}\tgame={:?}", sd, g);
    }
    cache.player_champion[0][1] = Some(myref);

    // ── (5) not_cleared 전멸 폴백: JungleRunner 상태를 못 만지므로 tick 을 미래로 옮겨 재현 시도
    //     (여기서는 is_cleared 값만 찍어 경로 도달 여부를 기록한다)
    {
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut cleared = 0;
        for c in CAMPS {
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            if is_cleared(c, 0, 1, &mut r, me0, &data, &tp, 0, &mut dbg) { cleared += 1; }
        }
        println!("fallback_reach\tcleared_camps={}/4\t(4 여야 817줄 폴백에 도달)", cleared);
    }

    // ── (6) 10 플레이어 × 양팀 (info.team/info.position 축)
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    for t in 0..2usize { for (pi, p) in poss.iter().enumerate() {
        if let Some(ps) = game.get_player_by_position(t, *p) {
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut r = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbg: DebugFrameData = Default::default();
            let g = best_jungle_goal(1, &mut r, ps, &data, &tp, None, &mut dbg);
            let ch = cache.player_champion[t][pi];
            let m = mine(1, ps, &data, &tp, None, &map, &cache);
            nrun += 1; if g == m { nmatch += 1; }
            println!("plr\tt{}\t{:?}\tchamp=({:?})\tgame={:?}\tmine={:?}\t{}",
                t, p, ch.map(|e| (e.x, e.y)), g, m, if g == m { "MATCH" } else { "**DIFF**" });
        }
    } }

    // ── (7) ★TLS 검사 — camp_pos 가 CAMP_POS_MEMO 를 쓰는지. 다른 setting 으로 두번째 MapDef
    {
        let mut s2 = real_setting(); s2.width = 480000; s2.height = 480000;
        let map2 = MapDef::moba(&s2);
        for c in CAMPS {
            println!("tlsmemo\t{:?}\tmap1_blue={:?}\tmap2_blue={:?}\tsame={}",
                c, map.camp_pos(c, true), map2.camp_pos(c, true),
                map.camp_pos(c, true) == map2.camp_pos(c, true));
        }
        // 역순으로 다시 — 순서 의존이 있으면 값이 뒤집힌다
        for c in CAMPS {
            println!("tlsmemo_rev\t{:?}\tmap2_blue={:?}\tmap1_blue={:?}",
                c, map2.camp_pos(c, true), map.camp_pos(c, true));
        }
    }

    println!("TOTAL\t{}/{}\tMATCH", nmatch, nrun);
    println!("DONE\tsetting_ok={}", ok);
    std::mem::forget(my);
}
