#![allow(unused, dead_code, non_snake_case)]
//! B5_o4 — 07 `EpicHuntAndBattlePlan::sub_plan` 진리표 (3분기 전부 + 게이트 경계 격리)
//!
//! 목표(ev4 → ev2):
//!  · L36 에픽 무손상 조건 `epic.hp == epic.stat_cached.hp`
//!  · L36 3번째 OR 항 `champ.hp < max && is_in_heal_area` (fountains[team] 사각형 4경계)
//!  · L41 은신 게이트 `tick_per_second + epic_ally_tick > epic_ally_killed_tick` (계수 1 / `>` 엄격)
//!  · GoalData+0x98 epic_ally_tick / +0xa0 epic_ally_killed_tick 의 역할
//!  · MobaMode live_list.get(0).and_then(get_entity_by_id) — len==0 / 없는 id
//!  · SubPlan 태그 5/9/11 + Hide 페이로드 4필드(bush / out_line=1 / check_move=0 / enemy_spotted_me=0)
//!
//! target_bush 는 private 이라 `transmute` 로 만든다(4차 배치B 경로). None 쌍을 같이 재서 레이아웃을 자기검증한다.
use game_core::*;
use game_ai::GoalData;
use game_ai::plan_legacy::old::EpicHuntAndBattlePlan;
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

#[derive(Clone, Copy, Debug)]
struct C {
    champ_hp: usize, champ_max: usize,
    champ_xy: Option<(u64,u64)>,        // None = 기본 위치 유지
    epic: u8,                          // 0=live_list 비움 / 1=풀피 / 2=1 깎임 / 3=없는 id
    bush: Option<usize>,
    ally_tick: usize, killed_tick: usize,
    tps: usize,
}

fn trial(c: C) -> (u64, [u8; 72], usize, bool) {
    let mut setting = real_setting();
    setting.tick_per_second = c.tps;
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

    // 내 챔피언(team0 Top) 실체 id 찾기
    let (my_id, epic_id) = {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c0.player_champion[0][0].unwrap().id, c0.player_champion[1][4].unwrap().id)
    };
    // 내 챔피언 상태
    if let Some(e) = game.world.entity.get_mut(my_id) {
        e.stat_cached.hp = c.champ_max;
        e.hp = c.champ_hp;
        if let Some((x,y)) = c.champ_xy { e.x = x; e.y = y; }
    }
    // 에픽 슬롯
    game.mode.jungle_runner.epic.live_list.clear();
    match c.epic {
        1 => { if let Some(e) = game.world.entity.get_mut(epic_id) { let m = e.stat_cached.hp; e.hp = m; }
               game.mode.jungle_runner.epic.live_list.push(epic_id); }
        2 => { if let Some(e) = game.world.entity.get_mut(epic_id) { let m = e.stat_cached.hp; e.hp = m - 1; }
               game.mode.jungle_runner.epic.live_list.push(epic_id); }
        3 => { game.mode.jungle_runner.epic.live_list.push(999_999_999); }
        _ => {}
    }

    let mut gd: GoalData = Default::default();
    gd.epic.epic_ally_tick = c.ally_tick;
    gd.epic.epic_ally_killed_tick = c.killed_tick;
    // 오프셋 실측
    let gd_off_ally = (&gd.epic.epic_ally_tick as *const usize as usize) - (&gd as *const GoalData as usize);
    let gd_off_killed = (&gd.epic.epic_ally_killed_tick as *const usize as usize) - (&gd as *const GoalData as usize);

    let plan: EpicHuntAndBattlePlan = match c.bush {
        None => unsafe { std::mem::transmute([0u64, 0u64]) },
        Some(b) => unsafe { std::mem::transmute([1u64, b as u64]) },
    };

    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let mut dbg: DebugFrameData = Default::default();
    let sp = plan.sub_plan(3, &mut rnd, player, &data, &gd, &mut dbg);
    let raw: [u8; 72] = unsafe { std::mem::transmute_copy(&sp) };
    let tag = u64::from_le_bytes(raw[0..8].try_into().unwrap());
    let in_heal = {
        let f = map.fountains[0];
        let e = cache.player_champion[0][0].unwrap();
        e.x >= f.0 && e.x <= f.2 && e.y >= f.1 && e.y <= f.3
    };
    (tag, raw, gd_off_ally * 0x1000 + gd_off_killed, in_heal)
}

fn name(tag: u64) -> &'static str {
    match tag { 5 => "Recall", 9 => "Hide", 11 => "EpicHunt", _ => "???" }
}

fn main() {
    let s = real_setting();
    println!("setting_ok\t{}", s.width!=0 && s.height!=0 && s.tick_per_second!=0 && s.champion_radius!=0);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&s);
    println!("fountains[0]\t{:?}", map.fountains[0]);
    let base = C { champ_hp: 100, champ_max: 100, champ_xy: None, epic: 0, bush: None,
                   ally_tick: 0, killed_tick: 1_000_000, tps: 60 };
    let (_, _, offs, _) = trial(base);
    println!("GoalData_offsets\tepic_ally_tick=0x{:x}\tepic_ally_killed_tick=0x{:x}", offs >> 12, offs & 0xfff);

    let mut bad = 0usize;
    let mut chk = |label: &str, c: C, exp: u64| {
        let (tag, raw, _, heal) = trial(c);
        let ok = tag == exp;
        if !ok { bad += 1; }
        println!("{}\ttag={}({})\texp={}({})\tin_heal={}\t{}", label, tag, name(tag), exp, name(exp), heal,
                 if ok {"MATCH"} else {"MISMATCH"});
        raw
    };

    // ── G1 귀환 HP 경계 (에픽 풀피)
    println!("\n#G1 귀환 HP% 경계 (hp_ratio < 51)");
    // ⚠ 챔피언 기본 스폰 위치가 **자기 분수대 안**이라, 그대로 두면 3번째 OR 항
    //    (hp<max && in_heal_area) 이 먼저 참이 돼 HP% 임계를 격리할 수 없다(1회차 실측).
    //    따라서 맵 중앙으로 옮긴다.
    for hp in [49usize, 50, 51, 52] {
        let c = C { champ_hp: hp, champ_max: 100, champ_xy: Some((480000, 480000)), epic: 1, ..base };
        chk(&format!("G1_hp{}", hp), c, if hp < 51 { 5 } else { 11 });
    }
    for hp in [399usize, 400, 509, 510] {
        let c = C { champ_hp: hp, champ_max: 1000, champ_xy: Some((480000, 480000)), epic: 1, ..base };
        chk(&format!("G1b_hp{}/1000", hp), c, if hp*100/1000 < 51 { 5 } else { 11 });
    }

    // ── G2 에픽 무손상 조건
    println!("\n#G2 에픽 무손상 (epic.hp == epic.stat_cached.hp)");
    let mid = Some((480000u64, 480000u64));
    chk("G2_full", C { champ_hp: 10, champ_max: 100, champ_xy: mid, epic: 1, ..base }, 5);
    chk("G2_dmg1", C { champ_hp: 10, champ_max: 100, champ_xy: mid, epic: 2, ..base }, 11);
    chk("G2_nolist", C { champ_hp: 10, champ_max: 100, champ_xy: mid, epic: 0, ..base }, 11);
    chk("G2_badid", C { champ_hp: 10, champ_max: 100, champ_xy: mid, epic: 3, ..base }, 11);

    // ── G3 3번째 OR 항: champ.hp < max && is_in_heal_area  (fountains[0] = (0,896000,64000,960000))
    println!("\n#G3 회복지역 사각형 (hp_ratio>=51 이라 1번째 항은 거짓)");
    let hz = |x: u64, y: u64, hp: usize| C { champ_hp: hp, champ_max: 100, champ_xy: Some((x,y)), epic: 1, ..base };
    chk("G3_in_LT",      hz(0, 896000, 60), 5);
    chk("G3_in_RB",      hz(64000, 960000, 60), 5);
    chk("G3_out_x+1",    hz(64001, 960000, 60), 11);
    chk("G3_out_y-1",    hz(0, 895999, 60), 11);
    chk("G3_out_center", hz(480000, 480000, 60), 11);
    chk("G3_fullhp_in",  hz(0, 896000, 100), 11);   // hp<max 가 거짓 => && 실패

    // ── G4 은신 게이트 경계: tps + ally_tick > killed_tick  (에픽 없음 => Recall 미발화)
    println!("\n#G4 은신 게이트 (tps + ally_tick > killed_tick), bush=Some(7)");
    for (a, k, tps) in [(1000usize, 1058usize, 60usize), (1000,1059,60), (1000,1060,60), (1000,1061,60),
                        (0, 59, 60), (0, 60, 60), (0, 0, 1), (0, 1, 1), (0, 29, 30), (0, 30, 30)] {
        let c = C { champ_hp: 100, champ_max: 100, epic: 0, bush: Some(7),
                    ally_tick: a, killed_tick: k, tps, ..base };
        let exp = if tps + a > k { 9 } else { 11 };
        chk(&format!("G4_a{}_k{}_tps{}", a, k, tps), c, exp);
    }

    // ── G5 bush=None 이면 게이트가 참이어도 EpicHunt
    println!("\n#G5 target_bush");
    chk("G5_bushNone_gateT", C { champ_hp: 100, champ_max: 100, epic: 0, bush: None,
                                 ally_tick: 1000, killed_tick: 0, tps: 60, ..base }, 11);
    chk("G5_bushSome_gateT", C { champ_hp: 100, champ_max: 100, epic: 0, bush: Some(3),
                                 ally_tick: 1000, killed_tick: 0, tps: 60, ..base }, 9);

    // ── G6 Hide 페이로드 4필드
    println!("\n#G6 Hide 페이로드");
    for b in [0usize, 7, 26] {
        let c = C { champ_hp: 100, champ_max: 100, epic: 0, bush: Some(b),
                    ally_tick: 1000, killed_tick: 0, tps: 60, ..base };
        let (tag, raw, _, _) = trial(c);
        let bush = u64::from_le_bytes(raw[8..16].try_into().unwrap());
        println!("G6\tbush_in={}\ttag={}\tbush_out={}\tout_line={}\tcheck_move={}\tenemy_spotted_me={}\t{}",
            b, tag, bush, raw[16], raw[17], raw[18],
            if tag==9 && bush==b as u64 && raw[16]==1 && raw[17]==0 && raw[18]==0 {"MATCH"} else {"MISMATCH"});
        if !(tag==9 && bush==b as u64 && raw[16]==1 && raw[17]==0 && raw[18]==0) { bad += 1; }
    }
    // EpicHunt 페이로드 need_recall
    let (tag, raw, _, _) = trial(C { champ_hp: 100, champ_max: 100, epic: 0, bush: None,
                                     ally_tick: 0, killed_tick: 1_000_000, tps: 60, ..base });
    println!("G6\tEpicHunt tag={}\tneed_recall(+0x8 i8)={}\t{}", tag, raw[8], if tag==11 && raw[8]==0 {"MATCH"} else {"MISMATCH"});
    if !(tag==11 && raw[8]==0) { bad += 1; }

    println!("\nSUMMARY\tmismatch={}", bad);
}
