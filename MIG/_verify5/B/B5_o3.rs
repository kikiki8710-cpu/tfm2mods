#![allow(unused, dead_code, non_snake_case)]
//! B5_o3 — 08 `EpicHuntAndPokePlan::is_end` 의 **미도달 경로 (c)(d) 공략**
//!
//! 3차가 남긴 미탐색: 「setup 경로는 `v24_...=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달」.
//! v24 의 IR(m09.ll:22104~22160) 을 읽어 **v24==false 이면서 is_visible_cell==true** 인 창을 찾았다:
//!   v24 exit phi = [ %18 -> false (v23 != 0), %31 -> true (is_visible_cell), %41 -> pressure_line ]
//!   ⟹ v23_recent_visible_enemies_near_point(player,data,cx,cy,180000,40) != 0 이면 v24=false 로 나간다.
//! 그러면 is_end 는 L179 로 흐르고 (c)/(d) 가 살아난다.
//!
//! ★부수 발견 검증: v23 의 6번째 인자는 **틱 창이 아니라 `min_hp_ratio`(HP%)** 다
//!   (DWARF `!61645 = DILocalVariable(name:"min_hp_ratio", arg:6)` · IR m15.ll:55688
//!    `hp*100/stat_cached.hp < %5 -> skip`). 명세 08 노브 「"최근 가시" 창 = 40틱」이 오독이다.
//!
//! ★세계를 바꿀 때마다 **게임을 새로 만든다**(TLS/캐시 오염 방지 — TEMPLATE.rs 함정 ③ 정신).
use game_core::*;
use game_ai::plan_legacy::old::EpicHuntAndPokePlan;
use game_ai::plan_legacy::team_plan::{TeamPlan, MainObjective, ObjectPhase};
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
    s
}

const TICK: usize = 5000;

/// 한 케이스 = 세계 하나. (v24, is_end, camp, cell, vis_cell) 반환
fn trial(d: i64, hp: usize, maxhp: usize, keep_slot: bool, phase: ObjectPhase,
         nrt_rem: usize, open_cell: bool, epic_alive: bool)
         -> (bool, bool, (u64,u64), (u64,u64), bool) {
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };

    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd0, &ctx);

    let camp = map.camp_pos(JungleType::Morgard, true);
    let ccx = (camp.0 / 32000) as usize;
    let ccy = (camp.1 / 32000) as usize;

    game.world.tick = TICK;
    for t in 0..2usize { for y in 0..30usize { for x in 0..30usize { game.world.visible_map[t][y][x] = 0; } } }
    if open_cell { game.world.visible_map[0][ccy][ccx] = 1; }
    game.mode.jungle_runner.epic.live_list.clear();
    if epic_alive {
        // 실존 엔티티 id 를 넣어 get_entity_by_id 가 Some 을 내게 한다(생존 판정만 씀)
        let tid = game.world.tower_ids[0];
        game.mode.jungle_runner.epic.live_list.push(tid);
    }
    game.mode.jungle_runner.epic.next_respawn_tick = TICK + nrt_rem;

    let vis = (&game as &dyn AbstractGame).is_visible_cell(0, ccx, ccy);
    let e_proto: Entity = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        c.player_champion[1][0].unwrap().clone()
    };
    let mut e = e_proto.clone();
    e.x = (camp.0 as i64 + d) as u64;
    e.y = camp.1;
    e.hp = hp; e.stat_cached.hp = maxhp;

    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    for p in 0..5usize { cache.player_champion[1][p] = None; }
    if keep_slot { cache.player_champion[1][0] = Some(&e); }
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    for p in 0..5usize { bb[0].last_visible[p] = 0; bb[1].last_visible[p] = 0; }
    bb[1].last_visible[0] = TICK;
    bb[1].last_visible_pos[0] = (e.x, e.y);
    let data = OperationData::new(&cache, &ctx, &bb);

    let mut tp: TeamPlan = Default::default();
    tp.objective = Some(MainObjective::Morgard { phase, with_battle: false });

    let v24 = tp.v24_objective_setup_should_release_to_passive(3, player, &data, JungleType::Morgard);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(5);
    let mut dbg: DebugFrameData = Default::default();
    let end = plan_is_end(&data, player, &tp, &mut rnd, &mut dbg);
    (v24, end, camp, (ccx as u64, ccy as u64), vis)
}

fn plan_is_end(data: &OperationData, player: &PlayerState, tp: &TeamPlan,
               rnd: &mut rand::rngs::StdRng, dbg: &mut DebugFrameData) -> bool {
    let plan: EpicHuntAndPokePlan = Default::default();
    plan.is_end(3, rnd, player, data, tp, dbg)
}

fn main() {
    let s = real_setting();
    println!("setting_ok\t{}", s.width!=0 && s.height!=0 && s.tick_per_second!=0 && s.champion_radius!=0);
    let (v24, end, camp, cell, vis) = trial(100000, 100, 100, true, ObjectPhase::Setup, 0, true, false);
    println!("camp\t{}\t{}\tcell=({},{})\tis_visible_cell={}", camp.0, camp.1, cell.0, cell.1, vis);

    let mut bad = 0;
    // ── (d) 경계 격리: 22500000000 = 150000^2
    println!("\n#D\td\tv24\tis_end\texp_v24\texp_end\tverdict\tpath");
    for d in [100000i64, 149999, 150000, 150001, 179999, 180000, 180001, 250000] {
        let (v24, end, _, _, _) = trial(d, 100, 100, true, ObjectPhase::Setup, 0, true, false);
        let v23nz = d <= 180000;
        let exp_v24 = !v23nz;
        let exp_end = if exp_v24 { true } else { (d as i128)*(d as i128) > 22500000000i128 };
        let ok = v24 == exp_v24 && end == exp_end;
        if !ok { bad += 1; }
        let path = if exp_v24 { "(b)v24=true" } else if exp_end { "(d)L184" } else { "폴스루 L194=false" };
        println!("D\t{}\t{}\t{}\t{}\t{}\t{}\t{}", d, v24, end, exp_v24, exp_end,
                 if ok {"MATCH"} else {"MISMATCH"}, path);
    }

    // ── ★v23 6번째 인자 = min_hp_ratio(40) 확인: d 고정, 적 HP% 만 흔든다
    println!("\n#H\thp%\tv24\tis_end\texp_v24(=hp%<40)\tverdict");
    for hp in [1usize, 20, 38, 39, 40, 41, 60, 100] {
        let (v24, end, _, _, _) = trial(100000, hp, 100, true, ObjectPhase::Setup, 0, true, false);
        let exp = hp < 40;
        let ok = v24 == exp; if !ok { bad += 1; }
        println!("H\t{}\t{}\t{}\t{}\t{}", hp, v24, end, exp, if ok {"MATCH"} else {"MISMATCH"});
    }
    // 정수나눗셈 축(max=1000)
    println!("#H2\thp/1000\tv24\texp_v24");
    for hp in [399usize, 400] {
        let (v24, _, _, _, _) = trial(100000, hp, 1000, true, ObjectPhase::Setup, 0, true, false);
        let exp = hp*100/1000 < 40;
        let ok = v24 == exp; if !ok { bad += 1; }
        println!("H2\t{}\t{}\t{}\t{}", hp, v24, exp, if ok {"MATCH"} else {"MISMATCH"});
    }

    // ── (c) 도달 시도 3종
    println!("\n#C\tdesc\tv24\tis_end\t해석");
    let (a1, b1, _, _, _) = trial(100000, 100, 100, false, ObjectPhase::Setup, 0, true, false);
    println!("C\tslots_all_None(v23=0)\t{}\t{}\tv24=true => (b) 가 먼저 나간다", a1, b1);
    let (a2, b2, _, _, _) = trial(100000, 39, 100, false, ObjectPhase::Setup, 0, true, false);
    println!("C\tslots_None+lowHP\t{}\t{}\t동일", a2, b2);
    // 시야 닫으면 v24 는 pressure_line 으로 간다 — 그때는 is_visible_cell=false 라 L179 블록 자체를 못 탄다
    let (a3, b3, _, _, v3) = trial(100000, 100, 100, false, ObjectPhase::Setup, 0, false, false);
    println!("C\tcell_closed,slots_None\t{}\t{}\tis_visible_cell={} (L179 미진입)", a3, b3, v3);

    // ── 에픽 생존/리젠 축 (L193/L194) — setup 경로를 끄고(Hunt) 잔여만 흔든다
    println!("\n#T\tnrt_rem\tepic_alive\tis_end\texp\tverdict");
    for (rem, alive) in [(0usize,true),(899,true),(900,true),(901,true),(1000,true),
                         (0,false),(899,false),(900,false),(901,false),(1000,false)] {
        let (_, end, _, _, _) = trial(100000, 100, 100, true, ObjectPhase::Hunt, rem, true, alive);
        let exp = if alive { false } else { rem > 900 };
        let ok = end == exp; if !ok { bad += 1; }
        println!("T\t{}\t{}\t{}\t{}\t{}", rem, alive, end, exp, if ok {"MATCH"} else {"MISMATCH"});
    }

    // ── phase 진리표 (setup 전용 종료경로가 phase 로만 열리는지)
    println!("\n#P\tphase\tv24\tis_end\t(d=250000: v23=0)");
    for (nm, ph) in [("None",ObjectPhase::None),("Setup",ObjectPhase::Setup),
                     ("Assemble",ObjectPhase::Assemble),("Hunt",ObjectPhase::Hunt)] {
        let (v24, end, _, _, _) = trial(250000, 100, 100, true, ph, 0, true, false);
        println!("P\t{}\t{}\t{}", nm, v24, end);
    }

    println!("\nSUMMARY\tmismatch={}", bad);
}
