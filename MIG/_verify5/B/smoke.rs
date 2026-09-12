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
//!  ③ ★★**TLS 메모를 쓰는 함수는 한 프로세스에서 반복 측정하면 안 된다.**
//!     (2026-09-11 4차 배치 D 실측 — 이 라운드 최대 발견)
//!     `fight_check::check_kill_die_tick` 은 `thread_local!(RefCell<DieTickCache>)` 이고
//!     **캐시 키 `DieTickKey` 에 엔티티 id 만 있고 hp·스탯이 없다.**
//!     ⟹ 한 프로세스 안에서 세계를 바꿔 가며 반복 호출하면 **첫 측정값이 계속 재생된다.**
//!     3차가 `single_tower_dive_is_viable` 을 「RNG·hp·공격력·방어력·사거리·거리·아군수·배치
//!     **9축 전부 무관**」으로 판정한 것이 전부 이 아티팩트였다.
//!     **케이스당 프로세스 1개**로 재면 `target hp 1999 → true / 2000 → false`
//!     (`kdt_tgt = hp × 60`, 경계 `< 120000`) — **대상 hp 는 판별 축이다.**
//!     ⟹ **규칙: 한 exe = 한 케이스.** 축을 쓸어보려면 인자로 케이스를 받아
//!        바깥에서 프로세스를 반복 실행하라(참조구현 `_verify4\D\body_o2.rs`).
//!     ⟹ **징후**: ①세계를 바꿨는데 결과가 안 바뀐다 ②호출 순서를 바꾸면 값이 뒤집힌다.
//!        둘 중 하나라도 보이면 「판별력 부재」라고 쓰기 전에 프로세스를 갈라 다시 재라.
//!
//!  ④ ★**실전 `ChampionInfo::default()` 는 스탯만이 아니라 액션 파라미터까지 0 이다.**
//!     (4차 배치 A) 챔피언 6종의 이펙트를 `expected_damage_target` 에 넣으면 **26/26 이 0**
//!     (조기반환). 판별력을 얻으려면 **`AttackEffect`(72B, 전 필드 pub)를 직접 조립**하라
//!     (`Effect` 도 전 필드 pub).
//!
//!  ⑤ ★**미니언은 자동으로 생기지 않는다.** (4차 배치 B) `minion_wave_setting` 16필드 +
//!     `melee/range_minion` 실전값을 주입하고 `run_tick` **600틱**을 돌려야 팀당 9마리가 선다.
//!     이걸로 이 프로젝트에서 `enemy_minion_line_action_damage_at > 0` 을 처음 얻었다.
//!
//! ## ⚠이 템플릿을 만들어도 안 쓴 프로브가 남는다
//! 3차가 이 파일을 정본화했는데, **3차 자신의 앵커 프로브**(`_verify3\D\D3_o10.rs`·`D3_o11.rs`)가
//! `real_setting()` 이 아니라 `Default::default()` + tps=60 만 세팅하고 있었다(4차 배치 D 적발).
//! ⟹ **과거 프로브의 결과를 인용할 때는 그 프로브가 `setting_ok()` 를 찍었는지 먼저 확인하라.**
//!
//! ## 빌드
//! ```
//! sh C:\tfm2mods\MIG\_verify3\build.sh <내프로브.rs>
//! → %TEMP%\tfm2_spanprobe\<이름>.exe
//! ```
//!
//! ## 검증 방법
//! `main()` 이 먼저 세팅 무결성을 찍는다. `setting_ok` 가 false 면 그 프로브의 결과는 믿지 마라.
//! TLS 메모 계열(③)을 재는 프로브라면 **케이스를 argv 로 받아 한 프로세스에 한 케이스만** 재라.
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

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    // ⚠`MapDef::moba` 는 setting 을 읽는다 — 세팅을 **먼저** 고친 뒤에 불러야 좌표가 정상이다.
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

    // 무결성 지표 — 이 세 줄이 기대값과 다르면 세팅/셋업이 잘못된 것이다
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
    println!("is_top_side_true\t{}/10\t(height=0 이면 10/10 로 붕괴한다)", top);
    println!("expect\ttowers=16 twin=2/2 is_top_side_true=8/10");
}
