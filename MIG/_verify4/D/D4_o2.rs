#![allow(unused, dead_code, non_snake_case)]
//! ★★SDK 실행 오라클 **정본 템플릿** (2026-09-11 확립) — 새 프로브는 이걸 복사해서 시작하라.
//!
//! ## 왜 이 파일이 필요한가
//! 1·2차·3차의 오라클 프로브들이 두 가지를 잘못했고, 그 때문에 "재료 부재 / 입력 판별력 부재"
//! 판정이 여러 건 잘못 나왔다:
//!
//!  ① **`GameSetting::default()` 는 거의 전부 0 이다.** 실전 설정 파일과 비교하면
//!     **숫자 필드 44개가 0 이 아니다**(width/height 960000 · tick_per_second 60 ·
//!     champion_radius 10000 · visible_distance 130000 · respawn_tick 300 · well_damage 600 …).
//!     ⟹ `height == 0` 이면 `height - y` 가 u64 **언더플로**해서 `is_top_side` 가 상수 true 가 된다.
//!        2026-09-11 2차가 「챔프 10명이 전부 탑 사이드 = 재료 부재」로 닫은 것이 이 오진이었고,
//!        실전 height 를 넣자 같은 초기 좌표에서 Support 2명이 false 로 갈렸다(3차 배치 C).
//!     ⟹ `champion_radius == 0` 이면 사거리 계열이 전 구간 0 이 된다
//!        (「default 챔피언은 이펙트가 비었다」로 오진했던 증상의 실제 원인 중 하나).
//!
//!  ② **`start_game()` 이 이미 타워·넥서스를 만든다.** 그 뒤 `init_tower()`/`init_nexus()` 를
//!     또 부르면 전부 2배가 된다 — 실측(`_verify2\towerchk.rs`):
//!       `start_game` 만      : tower_ids 16 · twin_towers 2/2 · 팀당 8  · 좌표중복 0
//!       `+init_tower/nexus`  : tower_ids 32 · twin_towers 4/4 · 팀당 16 · 좌표중복 8
//!     2차 배치 D 의 「타워 팀당 10개, twin 2쌍 좌표 중복」이 이 오염이었다.
//!
//! ## 빌드
//! ```
//! sh C:\tfm2mods\MIG\_verify3\build.sh <내프로브.rs>
//! → %TEMP%\tfm2_spanprobe\<이름>.exe
//! ```
//!
//! ## 검증 방법
//! `main()` 이 먼저 세팅 무결성을 찍는다. `setting_ok` 가 false 면 그 프로브의 결과는 믿지 마라.
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

/// ★실전 설정값. 아래 44줄은 게임 설치본의
///   `<게임설치>\bundle_unpacked_full\setting\game_setting.game_setting`(JSON)에서 **자동 생성**했다
///   (`_verify3\_setting_gen.txt`, 재생성은 그 옆 주석 참조). 44줄 전부 rustc 컴파일 통과 = 필드명·타입 유효.
/// ⚠`serde_json::from_str` 로 직접 읽는 방법은 **쓸 수 없다** — SDK deps 에 serde rlib 이 두 개 있고
///   `game_core` 가 링크한 판과 `serde_json` 이 링크한 판이 달라 `Deserialize` 바운드가 안 맞는다(실측 E0277).
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

/// 세팅이 0 으로 남아 있지 않은지 확인한다. **프로브마다 이걸 찍어라.**
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}

/// ★게임 하나를 만든다. **`init_tower`/`init_nexus` 를 부르지 않는다**(위 ②).
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80;
            st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    // ⛔ game.init_tower(ctx);      ← 부르지 마라(타워 2배)
    // ⛔ game.init_nexus(setting, map);
    game
}


// D4_o2 — `single_tower_dive_is_viable` 판별축 검증.
//  ★한 번 실행에 **한 케이스만** 측정하고 끝낸다(인자로 케이스 선택).
//    이유 = `check_kill_die_tick`(fight_check.rs:917)은 **캐시된 진입점**(DieTickKey)이라
//    같은 프로세스에서 입력을 흔들면 캐시가 먼저 답할 수 있다. 3차의 「9축 전부 무관」이
//    그 아티팩트인지 가리려면 프로세스를 분리해야 한다.
//  케이스: o10_<towerhp>   = D3_o10 세계(아군 스폰 그대로, boost, target hp 2000)
//          o11_<targethp>  = D3_o11 세계(아군 4명 한 점 적층, tower hp 100)
//          seq             = 한 프로세스 안에서 o11 -> o10 순서로 두 번(캐시 오염 재현용)

fn one(setting: &GameSetting, ms: &MapSetting, map: &MapDef, pool: &bumpalo::Bump,
       ctx: &GameContext, stack_allies: bool, thp: usize, twr_hp: usize, tag: &str) {
    let tp: TeamPlan = Default::default();
    let t1t0 = (272000u64, 48000u64);
    let mut g = mkgame(setting, ms, map, ctx);
    let ids: Vec<usize> = g.world.champion_ids.clone();
    let tw: Vec<usize> = g.world.tower_ids.clone();
    for id in ids.iter() {
        let e = g.world.entity.get_mut(*id).unwrap();
        e.stat.hp = 2000; e.stat_cached.hp = 2000; e.hp = 2000;
        e.stat.attack = 5000; e.stat_cached.attack = 5000;
        e.stat.defence = 0; e.stat_cached.defence = 0;
        e.stat.move_speed = 3000; e.stat_cached.move_speed = 3000;
        e.radius = 5000;
        if let Some(f) = e.attack_effect.as_mut() { f.range = 50000; }
    }
    { let e = g.world.entity.get_mut(23).unwrap();
      e.stat.hp = thp; e.stat_cached.hp = thp; e.hp = thp; e.x = t1t0.0; e.y = t1t0.1; }
    { let e = g.world.entity.get_mut(18).unwrap(); e.x = t1t0.0 + 20000; e.y = t1t0.1; }
    if twr_hp != 0 {
        for id in tw.iter() { let e = g.world.entity.get_mut(*id).unwrap();
            e.stat.hp = twr_hp; e.stat_cached.hp = twr_hp; e.hp = twr_hp; }
    }
    if stack_allies {
        for id in ids.iter().skip(1).take(4) {
            let e = g.world.entity.get_mut(*id).unwrap(); e.x = 15000; e.y = 913000; }
    }
    let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, ctx, &bb);
    let mut dbgf: DebugFrameData = Default::default();
    let ps = g.get_player_by_position(0, Position::Top).unwrap();
    let tgt = cache.game.get_entity_by_id(23).unwrap();
    let me = cache.player_champion[0][0].unwrap();
    let mut r = rand::rngs::StdRng::seed_from_u64(7);
    let v = old::single_tower_dive_is_viable(3, &mut r, ps, &data, &tp, tgt, &mut dbgf);
    // ★ 내 재현: return check_kill_die_tick(target) < check_kill_die_tick(me)   (single_battle.rs:944)
    let mut a6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    a6.push(tgt);
    let mut a7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    for t in cache.iter_towers_without_nexus(1) { a7.push(t); break; }
    let mut b6: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    b6.push(me);
    let b7: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(pool);
    let mut r1 = rand::rngs::StdRng::seed_from_u64(7);
    let ka = game_ai::check_kill_die_tick(3, &mut r1, &data, ps, me, a6, a7, &mut dbgf);
    let mut r2 = rand::rngs::StdRng::seed_from_u64(7);
    let kb = game_ai::check_kill_die_tick(3, &mut r2, &data, ps, tgt, b6, b7, &mut dbgf);
    println!("{}\tstack={}\tthp={}\ttwr_hp={}\tviable={}\tkdt_me={}\tkdt_tgt={}",
             tag, stack_allies, thp, twr_hp, v, ka, kb);
}

fn main() {
    let arg: String = std::env::args().nth(1).unwrap_or("o10_0".to_string());
    let setting = real_setting();
    setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };

    if arg == "seq" {
        one(&setting, &ms, &map, &pool, &ctx, true, 50, 100, "seq1_o11");
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "seq2_o10");
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "seq3_o10again");
        return;
    }
    if arg == "rseq" {
        one(&setting, &ms, &map, &pool, &ctx, false, 2000, 0, "rseq1_o10");
        one(&setting, &ms, &map, &pool, &ctx, true, 50, 100, "rseq2_o11");
        return;
    }
    let (kind, num) = arg.split_at(arg.find('_').unwrap_or(3));
    let n: usize = num.trim_start_matches('_').parse().unwrap_or(0);
    if kind == "o10" { one(&setting, &ms, &map, &pool, &ctx, false, 2000, n, &arg); }
    else { one(&setting, &ms, &map, &pool, &ctx, true, n, 100, &arg); }
}
