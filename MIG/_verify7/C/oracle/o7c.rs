#![allow(unused, dead_code, non_snake_case)]
//! 7차 배치 C 오라클 — `_verify3\TEMPLATE.rs` 를 복사해 시작했다(real_setting/setting_ok 포함).
//!
//! 목적 = `ev≥4` 를 **실행 확인**으로 내리기.
//!  §A `TutorialType` 9종의 `spawn_epic/spawn_serpen/spawn_*_minion/player_count` 전수
//!      → specs[11] consts[6..14]·knobs · specs[12] knobs[4]·knobs[5]
//!  §B `rule_scope::position_exists(ctx, pos)` tutorial 9 × pos 5 = 45칸
//!  §C `rule_scope::chat_allowed(ctx, &chat)` **Chat 태그 0..56 × tutorial 9 = 513칸**
//!      Chat 은 24B · 직접태그 @+0x0(유효 0..=56) 이라 0 으로 채운 24B 에 태그만 써서 전 variant 를 만든다
//!      (페이로드 usize/Position/LineType 은 0 이 유효값)  → specs[12] knobs[6]
//!  §D `PlayCall(47)`/`PlayPropose(49)` 의 코드 바이트(+0x1/+0x2) 스윕 → specs[12] knobs[7]
//!
//! ⚠TLS 메모(TEMPLATE ③) 대상이 아니다 — 순수 함수 4종이라 한 프로세스에서 쓸어도 된다.
//!   (`tlsscan` 대상인 `check_kill_die_tick`·`camp_pos` 는 여기서 안 부른다)
use game_core::*;
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

const TUTS: [TutorialType; 9] = [
    TutorialType::None, TutorialType::First, TutorialType::TopSolo, TutorialType::Bottom,
    TutorialType::MidSolo, TutorialType::MidBottom, TutorialType::JungleOnly,
    TutorialType::Line, TutorialType::Total];
const POSS: [Position; 5] = [Position::Top, Position::Jungle, Position::Mid,
                             Position::Bottom, Position::Support];

/// Chat 태그 t 의 값을 **0 으로 채운 24B + 태그 바이트**로 만든다.
/// tcx: `game_core::Chat` size=24 align=8, tag=Direct i8 @+0x0, valid 0..=56.
/// 페이로드는 usize / Position(i32,0 유효) / LineType(i8,0 유효) 뿐이라 0 이 전부 유효값이다.
unsafe fn mkchat(tag: u8, b1: u8, b2: u8, w8: usize) -> Chat {
    let mut raw = [0u8; 24];
    raw[0] = tag;
    raw[1] = b1;
    raw[2] = b2;
    raw[8..16].copy_from_slice(&w8.to_ne_bytes());
    std::mem::transmute::<[u8; 24], Chat>(raw)
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
    let mkctx = |t: TutorialType| GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: t, trace_level: TraceLevel::Off,
    };

    // ── §A TutorialType 술어 전수 ──────────────────────────────────────────
    println!("\n===== §O7C-A  TutorialType 9종 술어 전수 =====");
    println!("tag\tname\t\tepic\tserpen\ttop_min\tmid_min\tbot_min\tplayer_count");
    let (mut ep, mut se, mut tp, mut md, mut bt) =
        (vec![], vec![], vec![], vec![], vec![]);
    for (i, t) in TUTS.iter().enumerate() {
        let (a, b, c, d, e, n) = (t.spawn_epic(), t.spawn_serpen(), t.spawn_top_minion(),
                                  t.spawn_mid_minion(), t.spawn_bottom_minion(), t.player_count());
        if a { ep.push(i) } if b { se.push(i) } if c { tp.push(i) }
        if d { md.push(i) } if e { bt.push(i) }
        println!("{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}", i, t, a, b, c, d, e, n);
    }
    println!("allow\tepic={:?}\tserpen={:?}\ttop_minion={:?}\tmid_minion={:?}\tbottom_minion={:?}",
             ep, se, tp, md, bt);

    // ── §B position_exists ───────────────────────────────────────────────
    println!("\n===== §O7C-B  rule_scope::position_exists (tutorial 9 × pos 5) =====");
    println!("tut\tTop\tJungle\tMid\tBottom\tSupport");
    let mut perpos: Vec<Vec<usize>> = vec![vec![]; 5];
    for (i, t) in TUTS.iter().enumerate() {
        let c = mkctx(*t);
        let r: Vec<bool> = POSS.iter()
            .map(|p| game_ai::plan_legacy::rule_scope::position_exists(&c, *p)).collect();
        for (j, v) in r.iter().enumerate() { if *v { perpos[j].push(i) } }
        println!("{}\t{}\t{}\t{}\t{}\t{}", i, r[0], r[1], r[2], r[3], r[4]);
    }
    for (j, p) in POSS.iter().enumerate() {
        println!("allow\t{:?}\t{:?}", p, perpos[j]);
    }

    // ── §C chat_allowed 전수 (Chat 태그 0..=56 × tutorial 9) ───────────────
    println!("\n===== §O7C-C  rule_scope::chat_allowed (Chat 0..=56 × tutorial 9) =====");
    let mut blocked: Vec<Vec<u8>> = vec![vec![]; 9];
    let mut always_ok = 0usize;
    println!("chat\ttut0..8 (T=허용)");
    for tag in 0u8..=56 {
        let mut row = String::new();
        let mut nallow = 0;
        for (i, t) in TUTS.iter().enumerate() {
            let c = mkctx(*t);
            let ch = unsafe { mkchat(tag, 0, 0, 0) };
            let ok = game_ai::plan_legacy::rule_scope::chat_allowed(&c, &ch);
            row.push(if ok { 'T' } else { '.' });
            if ok { nallow += 1 } else { blocked[i].push(tag) }
        }
        if nallow == 9 { always_ok += 1 }
        println!("{}\t{}\t{}", tag, row, if nallow == 9 { "무조건 허용" } else { "" });
    }
    println!("always_allowed_tags\t{}/57", always_ok);
    for i in 0..9 { println!("blocked_in_tut{}\t{:?}", i, blocked[i]); }

    // ── §D PlayCall/PlayPropose 코드 바이트 스윕 ──────────────────────────
    println!("\n===== §O7C-D  PlayCall(47)/PlayPropose(49)/GankPlan(50) 코드 바이트 스윕 =====");
    println!("(tutorial 을 Line(7) 로 고정 — 일반경기 None 은 대부분 무조건 허용이라 게이트가 안 보인다)");
    for tag in [47u8, 48, 49, 50, 55] {
        for t in [TutorialType::None, TutorialType::Line, TutorialType::Total,
                  TutorialType::TopSolo, TutorialType::JungleOnly] {
            let c = mkctx(t);
            let mut r1 = String::new();
            for b1 in 0u8..8 {
                let ch = unsafe { mkchat(tag, b1, 0, 0) };
                r1.push(if game_ai::plan_legacy::rule_scope::chat_allowed(&c, &ch) { 'T' } else { '.' });
            }
            let mut r2 = String::new();
            for b2 in 0u8..8 {
                let ch = unsafe { mkchat(tag, 0, b2, 0) };
                r2.push(if game_ai::plan_legacy::rule_scope::chat_allowed(&c, &ch) { 'T' } else { '.' });
            }
            println!("tag{}\t{:?}\tbyte+1[0..7]={}\tbyte+2[0..7]={}", tag, t, r1, r2);
        }
    }
    println!("\nDONE");
}
