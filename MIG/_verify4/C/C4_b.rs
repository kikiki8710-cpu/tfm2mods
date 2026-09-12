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

// ── 4차 배치C: region_point 산출식 반증 ─────────────────────────────
use game_core::{AbstractGame, LineType};

fn calc(adj: &Vec<Vec<usize>>, blue: &[bool; 27], red: &[bool; 27]) -> [i32; 27] {
    let mut bc = [0i64; 27];
    let mut rc = [0i64; 27];
    for r in 0..27 {
        if blue[r] && !red[r] { for &n in adj[r].iter() { bc[n] += 1; } }
        if red[r] && !blue[r] { for &n in adj[r].iter() { rc[n] += 1; } }
    }
    let mut out = [0i32; 27];
    for r in 0..27 {
        out[r] = (if blue[r] { 5 } else { 0 }) + bc[r] as i32
               - (if red[r] { 5 } else { 0 }) - rc[r] as i32;
    }
    out
}

fn main() {
    let setting = real_setting();
    setting_ok(&setting);
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
    println!("towers\t{}", game.world.tower_ids.len());

    // region_adj
    println!("ADJ_LEN\t{}", map.region_adj.len());
    for (r, v) in map.region_adj.iter().enumerate() {
        println!("ADJ\t{}\t{:?}", r, v);
    }
    // lane_seq
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let mut seqs = [[[0usize; 7]; 3]; 2];
    for t in 0..2usize {
        for (li, l) in lines.iter().enumerate() {
            seqs[t][li] = map.lane_seq(*l, t);
            println!("SEQ\tteam{}\t{:?}\t{:?}", t, l, seqs[t][li]);
        }
    }
    // 측정값
    let meas = cache.region_point;
    println!("MEAS\t{:?}", meas);
    println!("LEAD\ttop={:?}\tmid={:?}\tbottom={:?}", cache.top_lead, cache.mid_lead, cache.bottom_lead);

    // 브루트포스: 각 팀·라인마다 prefix 길이 k(0=아무것도 안 마킹, 1..=7 = seq[0..k])
    let mut hits = 0usize;
    for bt in 0..8usize { for bm in 0..8usize { for bb in 0..8usize {
      let mut blue = [false; 27];
      for i in 0..bt { blue[seqs[0][0][i]] = true; }
      for i in 0..bm { blue[seqs[0][1][i]] = true; }
      for i in 0..bb { blue[seqs[0][2][i]] = true; }
      for rt in 0..8usize { for rm in 0..8usize { for rb in 0..8usize {
        let mut red = [false; 27];
        for i in 0..rt { red[seqs[1][0][i]] = true; }
        for i in 0..rm { red[seqs[1][1][i]] = true; }
        for i in 0..rb { red[seqs[1][2][i]] = true; }
        if calc(&map.region_adj, &blue, &red) == meas {
            println!("FIT\tblue=({},{},{})\tred=({},{},{})", bt, bm, bb, rt, rm, rb);
            hits += 1;
        }
      }}}
    }}}
    println!("FITS\t{}", hits);

    // *_lead 재현 (3차 산출식: team0 p>2 / team1 p<-2 동안 전진, 첫 실패에서 break)
    for t in 0..2usize {
        for (li, l) in lines.iter().enumerate() {
            let sq = seqs[t][li];
            let mut lead = 0usize;
            for i in 0..7usize {
                let p = meas[sq[i]];
                if (t == 0 && p > 2) || (t == 1 && p < -2) { lead = i; } else { break; }
            }
            let got = match li { 0 => cache.top_lead[t], 1 => cache.mid_lead[t], _ => cache.bottom_lead[t] };
            println!("LEADCHK\tteam{}\t{:?}\tmine={}\tgame={}\tmatch={}", t, l, lead, got, lead == got);
        }
    }
}
