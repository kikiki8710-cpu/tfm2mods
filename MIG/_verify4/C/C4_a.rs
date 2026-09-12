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

// ── 4차 배치C 추가 ────────────────────────────────────────────────────
use game_ai::plan_legacy::old::{LineGankerPlan, LineGankerPhase};
use game_core::{AbstractGame, LineType, Chat, CancelReason};

/// &dyn AbstractGame 팻포인터에서 vtable 슬롯을 **바이트 오프셋으로** 꺼낸다.
unsafe fn vtslot(dg: &dyn AbstractGame, byte_off: usize) -> *const () {
    let raw: (*const (), *const *const ()) = std::mem::transmute(dg);
    *raw.1.add(byte_off / 8)
}
unsafe fn vtdata(dg: &dyn AbstractGame) -> *const () {
    let raw: (*const (), *const *const ()) = std::mem::transmute(dg);
    raw.0
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    // 무결성 지표
    println!("towers\t{}\ttwin0={}\ttwin1={}",
             game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
    let mut top = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                if game_core::is_top_side(&ctx, e.x, e.y) { top += 1; }
            }
        }
    }
    println!("is_top_side_true\t{}/10", top);
    println!("expect\ttowers=16 twin=2/2 is_top_side_true=8/10");

    // ═══ (1) vtable 슬롯 — **구체 impl `Game`** 기준으로 확인 (10 open[2] / 11 mem / 12 mem)
    //     divtable 이 준 이름은 ExpectedGame vtable 전역 기준이었다. 여기서는 진짜 Game 의
    //     &dyn 팻포인터에서 슬롯을 꺼내 간접호출하고 직접 트레이트 호출과 대조한다.
    let dg: &dyn AbstractGame = &game as &dyn AbstractGame;
    unsafe {
        let d = vtdata(dg);
        // 0x28 tick
        let f: extern "Rust" fn(*const ()) -> usize = std::mem::transmute(vtslot(dg, 0x28));
        let a = f(d); let b = AbstractGame::tick(&game);
        println!("VT\t0x28\ttick\tindirect={}\tdirect={}\tmatch={}", a, b, a == b);
        // 0x40 get_game_mode
        let g: extern "Rust" fn(*const ()) -> GameMode<'static> = std::mem::transmute(vtslot(dg, 0x40));
        let ga = g(d); let gb = AbstractGame::get_game_mode(&game);
        let tag = |m: &GameMode| match m { GameMode::Moba(_) => 0, GameMode::SingleLane(_) => 1, GameMode::DeathMatch(_) => 2 };
        println!("VT\t0x40\tget_game_mode\tindirect_tag={}\tdirect_tag={}\tmatch={}",
                 tag(&ga), tag(&gb), tag(&ga) == tag(&gb));
        // 0x1f0 get_entity_by_id
        let h: extern "Rust" fn(*const (), usize) -> Option<&'static Entity> = std::mem::transmute(vtslot(dg, 0x1f0));
        let id = game.world.tower_ids[0];
        let ha = h(d, id); let hb = AbstractGame::get_entity_by_id(&game, id);
        println!("VT\t0x1f0\tget_entity_by_id\tid={}\tindirect_some={}\tdirect_some={}\tsame_ptr={}",
                 id, ha.is_some(), hb.is_some(),
                 ha.map(|e| e as *const Entity) == hb.map(|e| e as *const Entity));
        // 0x108 strategy (3차 신규 주장 재확인)
        let s2: extern "Rust" fn(*const (), usize) -> Strategy = std::mem::transmute(vtslot(dg, 0x108));
        let sa = s2(d, 0); let sb = AbstractGame::strategy(&game, 0);
        println!("VT\t0x108\tstrategy\tindirect_objfin={:?}\tdirect_objfin={:?}\tmatch={}",
                 sa.object_finish, sb.object_finish, format!("{:?}", sa) == format!("{:?}", sb));
    }

    // ═══ (2) LineGankerPlan::new 의 두 usize 가 어느 필드로 가나 (14 open[2] 전제)
    let p = LineGankerPlan::new(LineType::Top, 111, 222);
    println!("NEW\tnew(Top,111,222)\t{:?}", p);
    unsafe {
        let raw = &p as *const LineGankerPlan as *const u8;
        println!("NEW_RAW\tsetup_limit@0x18={}\twait_limit@0x20={}\tline@0x28={}\tphase@0x29={}",
                 *(raw.add(0x18) as *const usize), *(raw.add(0x20) as *const usize),
                 *raw.add(0x28), *raw.add(0x29));
    }
    let p2 = LineGankerPlan::new_with_phase(LineType::Mid, 333, 444, LineGankerPhase::Cancel);
    println!("NEWP\tnew_with_phase(Mid,333,444,Cancel)\t{:?}", p2);
    unsafe {
        let raw = &p2 as *const LineGankerPlan as *const u8;
        println!("NEWP_RAW\t0x18={}\t0x20={}\tline={}\tphase_tag={}",
                 *(raw.add(0x18) as *const usize), *(raw.add(0x20) as *const usize),
                 *raw.add(0x28), *raw.add(0x29));
    }

    // ═══ (3) is_end 진리표 — setup_limit/wait_limit 의 실제 소비 규칙 (14 open[2], ev5→2)
    let tick = AbstractGame::tick(&game);
    println!("TICK\t{}", tick);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    let mut dbg: DebugFrameData = Default::default();
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let phases = [("WaitResponse", LineGankerPhase::WaitResponse),
                  ("Setup", LineGankerPhase::Setup),
                  ("Cancel", LineGankerPhase::Cancel),
                  ("ChangeJungle", LineGankerPhase::ChangeJungle(JungleType::Rhino))];
    println!("HDR\tis_end\tphase\tsetup_limit\twait_limit\tresult");
    for (nm, ph) in phases.iter() {
        for &sl in [tick, tick + 1, tick.wrapping_sub(1)].iter() {
            for &wl in [tick, tick + 1, tick.wrapping_sub(1)].iter() {
                let pp = LineGankerPlan::new_with_phase(LineType::Top, sl, wl, ph.clone());
                let r = pp.is_end(3, &mut rnd, player, &data, &mut dbg);
                println!("IS_END\t{}\t{}\t{}\t{}", nm, sl as i64 - tick as i64,
                         wl as i64 - tick as i64, r);
            }
        }
    }

    // ═══ (4) Chat::Cancel 24B 원시 덤프 (14 open[3])
    for (nm, c) in [("Cancel(LowHpSelf)", Chat::Cancel(CancelReason::LowHpSelf)),
                    ("Cancel(TargetMissing)", Chat::Cancel(CancelReason::TargetMissing))].iter() {
        unsafe {
            let raw = c as *const Chat as *const u8;
            let mut v = String::new();
            for i in 0..24 { v.push_str(&format!("{:02x} ", *raw.add(i))); }
            println!("CHATRAW\t{}\tsize={}\t{}", nm, std::mem::size_of::<Chat>(), v);
        }
    }

    // ═══ (5) MapDef.bushes 값 사전 재산출 (14 open[0])
    let mut acc: std::collections::BTreeMap<usize, (u64, u64, usize)> = Default::default();
    let mut zero = 0usize;
    for y in 0..30usize {
        for x in 0..30usize {
            let id = map.bushes[y][x];
            if id == 0 { zero += 1; continue; }
            let e = acc.entry(id).or_insert((0, 0, 0));
            e.0 += (x as u64) * 32000 + 16000;
            e.1 += (y as u64) * 32000 + 16000;
            e.2 += 1;
        }
    }
    println!("BUSH\tzero_cells={}\tdistinct={}", zero, acc.len());
    for (id, (sx, sy, n)) in acc.iter() {
        println!("BUSHID\t{}\t{}\t{}\tcells={}", id, sx / (*n as u64), sy / (*n as u64), n);
    }
}
