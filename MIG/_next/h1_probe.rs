#![allow(unused, dead_code, non_snake_case)]
//! h1_probe — ★**차기 대상 `LegacyPlanHandler::update` 를 오라클로 실제 호출할 수 있는가?**
//!
//! 이 한 가지가 계획 전체를 결정한다:
//!  - 호출된다면 → 14,476줄을 **입출력 진리표**로 공략할 수 있다(IR 독해는 보조).
//!  - 막힌다면   → IR + DWARF + 줄 길이 산술만으로 가야 하고 일정이 배 이상 늘어난다.
//!
//! `_verify3\TEMPLATE.rs` 의 세팅을 그대로 쓴다(실전 GameSetting 44줄 · `init_tower` 재호출 금지).
//! 관측은 2026-09-11 3차 배치C 가 확립한 **구조체 스냅샷 바이트 diff**(`LegacyPlanHandler` 6,168B) —
//! 반환형이 `()` 라도, 필드가 pub 이 아니라도 `writes` 를 오프셋 단위로 읽어낼 수 있다.
//!
//! tcx 시그니처(정본):
//!   fn(&mut LegacyPlanHandler, usize, &mut StdRng, &PlayerState, &OperationData,
//!      &mut DebugFrameData, bool)
//!
//! 빌드: sh C:\tfm2mods\MIG\_verify3\build.sh C:\tfm2mods\MIG\_next\h1_probe.rs
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use rand::SeedableRng;
use std::sync::Arc;

// ── 실전 설정(TEMPLATE.rs 와 동일) ───────────────────────────────────
fn real_setting() -> GameSetting {
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

fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
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
    game.start_game(&mut rnd, ctx);   // ⛔init_tower/init_nexus 재호출 금지(타워 2배)
    game
}

/// 6,168B 스냅샷. 필드가 pub 이 아니어도 바이트로 읽는다(3차 배치C 기법).
unsafe fn snap(h: &LegacyPlanHandler) -> Vec<u8> {
    let n = std::mem::size_of::<LegacyPlanHandler>();
    std::slice::from_raw_parts(h as *const _ as *const u8, n).to_vec()
}

/// 바뀐 바이트 구간을 [start,end) 로 묶어 돌려준다.
fn diff_runs(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < a.len().min(b.len()) {
        if a[i] != b[i] {
            let s = i;
            while i < a.len().min(b.len()) && a[i] != b[i] {
                i += 1;
            }
            out.push((s, i));
        } else {
            i += 1;
        }
    }
    out
}

fn main() {
    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}",
             setting.width != 0 && setting.tick_per_second != 0,
             setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
    println!("sizeof_LegacyPlanHandler\t{}\t(기대 6168)", std::mem::size_of::<LegacyPlanHandler>());

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
    println!("towers\t{}\ttwin0={}\ttwin1={}",
             game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());

    // ── version 을 흔들며 update 를 부르고 상태 변화를 본다 ─────────────
    println!("\nversion\tret\tdiff_runs\tdiff_bytes\tfirst_runs");
    for ver in [0usize, 1, 2, 3, 40, 50, 60] {
        let Some(player) = game.get_player_by_position(0, Position::Top) else {
            println!("{}\tplayer 없음", ver);
            continue;
        };
        // tcx sig: fn(usize version, &mut StdRng, usize team, Position) -> LegacyPlanHandler
        let mut hrnd = rand::rngs::StdRng::seed_from_u64(5);
        let mut h = LegacyPlanHandler::new(ver, &mut hrnd, 0usize, Position::Top);
        let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
        let mut dbg: DebugFrameData = Default::default();
        let before = unsafe { snap(&h) };
        h.update(ver, &mut rnd, player, &data, &mut dbg, false);
        let after = unsafe { snap(&h) };
        let runs = diff_runs(&before, &after);
        let bytes: usize = runs.iter().map(|(s, e)| e - s).sum();
        let head: Vec<String> = runs.iter().take(8)
            .map(|(s, e)| format!("0x{:x}..0x{:x}", s, e)).collect();
        println!("{}\tok\t{}\t{}\t{}", ver, runs.len(), bytes, head.join(","));
    }
    println!("\n★update 가 실제로 호출됐고 상태 변화를 오프셋 단위로 읽을 수 있다 = 오라클 공략 가능");
}
