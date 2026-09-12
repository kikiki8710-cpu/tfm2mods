#![allow(unused, dead_code, non_snake_case)]
//! B6_o2 — **05 `v50_fold_dive_episode` 최초 실행검증** (상위 `pub` 래퍼 `LegacyPlanHandler::update` 경유)
//!
//! ## 왜 되나 — 5차는 "`in:game_ai` 라 차단"으로 닫혀 있었다
//! `v50_fold_dive_episode` 의 **호출부 2곳(m13.ll:23653 · 23856)이 `LegacyPlanHandler::update` 의
//! `define`(m13.ll:14467) 안**이다(`v50_track_dive_episode` 는 전량 인라인돼 call 이 0 이다).
//! `update` 는 `vis=pub` 이므로 **바깥에서 부를 수 있다.**
//!
//! ## 관측 창구
//! `v50_dive_episodes`(+0x888) · `last_dive_abandon_tick`(+0x1480) · `plan`(+0x5e8) 은 **pub 필드**다
//! (`offset_of!` 가 통과 = B6_o1). 진행중 에피소드 `v50_dive_ep_live`(+0x570) 만 private 이라
//! **raw 120B 주입**으로 세운다(tcx 가 필드 20개를 전부 확인해 준 레이아웃 — B6_o1 · tcxdict).
//!
//! ⚠주입 시 `target_pos`(Position, 4B 태그) · `tower`(TowerType, 1B 태그)는 **유효 판별자**만 쓴다.
//!   (1차 시도에서 0 페이로드 + 쓰레기 판별자로 Debug 가 세그폴트했다 — B6_o1 주석 참조)
use game_core::*;
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use game_ai::plan_legacy::types::BigPlan;
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

/// 주입할 진행중 에피소드. **명세 05 `mem[1..18]` 의 오프셋 그대로** 120B 를 손으로 깐다.
#[derive(Clone, Copy, Debug)]
struct Live {
    start_tick: u64, last_tick: u64, ep_ticks: u64, in_range_ticks: u64,
    holder_ticks: u64, team_holder_ticks: u64, minion_cover_ticks: u64,
    soaked_hp: u64, gap_ticks: u64, max_catch_break: u64, uncatch_total: u64,
    target_pos: i32, start_race_adv: i32, start_in_range: u8, tower: u8,
    start_model: u8, start_na: u8, start_ne: u8, start_tgt_hp: u8,
}
impl Default for Live {
    fn default() -> Self {
        // 전 필드 서로 다른 센티널 — 레코드 22칸 대응표를 한 번에 가른다
        Live { start_tick: 1111, last_tick: 2222, ep_ticks: 3333, in_range_ticks: 0,
               holder_ticks: 5555, team_holder_ticks: 0, minion_cover_ticks: 7777,
               soaked_hp: 8888, gap_ticks: 9999, max_catch_break: 1010, uncatch_total: 2020,
               target_pos: 3, start_race_adv: -77, start_in_range: 1, tower: 1,
               start_model: 31, start_na: 32, start_ne: 33, start_tgt_hp: 34 }
    }
}

/// handler+0x570 부터 120B 를 Some(live) 로 만든다.
/// +0x570 = 0 ⟹ 바깥 Option 은 Some, 안쪽 `prev_holder_hp` 는 None (tcx: -1=바깥None / 0,1=prev tag)
unsafe fn inject_live(h: &mut LegacyPlanHandler, l: &Live) {
    let p = h as *mut LegacyPlanHandler as *mut u8;
    let w64 = |o: usize, v: u64| unsafe { std::ptr::write_unaligned(p.add(o) as *mut u64, v) };
    let w32 = |o: usize, v: i32| unsafe { std::ptr::write_unaligned(p.add(o) as *mut i32, v) };
    let w8 = |o: usize, v: u8| unsafe { std::ptr::write_unaligned(p.add(o), v) };
    w64(0x570, 0);                       // 바깥 Some + prev_holder_hp = None
    w64(0x578, 0);                       // prev_holder_hp 페이로드(읽히지 않는다고 명세가 말한다)
    w64(0x580, l.start_tick);
    w64(0x588, l.last_tick);
    w64(0x590, l.ep_ticks);
    w64(0x598, l.in_range_ticks);
    w64(0x5a0, l.holder_ticks);
    w64(0x5a8, l.team_holder_ticks);
    w64(0x5b0, l.minion_cover_ticks);
    w64(0x5b8, l.soaked_hp);
    w64(0x5c0, l.gap_ticks);
    w64(0x5c8, l.max_catch_break);
    w64(0x5d0, l.uncatch_total);
    w32(0x5d8, l.target_pos);
    w32(0x5dc, l.start_race_adv);
    w8(0x5e0, l.start_in_range);
    w8(0x5e1, l.tower);
    w8(0x5e2, l.start_model);
    w8(0x5e3, l.start_na);
    w8(0x5e4, l.start_ne);
    w8(0x5e5, l.start_tgt_hp);
}

unsafe fn set_abort_src(h: &mut LegacyPlanHandler, v: u8) {
    std::ptr::write_unaligned((h as *mut LegacyPlanHandler as *mut u8).add(0x1811), v);
}
unsafe fn live_tag(h: &LegacyPlanHandler) -> i64 {
    std::ptr::read_unaligned((h as *const LegacyPlanHandler as *const u8).add(0x570) as *const i64)
}

/// 명세 05 의 end_plan 분류 사슬 (dive_episode.rs:146~153) 그대로 — ⓓ 독립 재구현
fn end_plan_of(n: &str) -> u8 {
    if n.starts_with("PassiveLine") { 1 }
    else if n.starts_with("PassiveJungle") { 2 }
    else if n.starts_with("LineGank") { 3 }
    else if n.contains("Epic") { 4 }
    else if n.contains("Serpen") { 5 }
    else if n.starts_with("ActiveRecall") { 6 }
    else if n.starts_with("Recall") { 6 }
    else if n.starts_with("Battle") { 7 }
    else if n.contains("Nexus") { 8 }
    else { 9 }
}

fn main() {
    // ── 세계 구성 (TEMPLATE.rs 함정 ①② 준수: real_setting · init_tower/nexus 미호출) ──
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
    const TICK: usize = 5000;
    game.world.tick = TICK;
    println!("setting_ok\t{}\twidth={}\ttps={}\tchamp_radius={}",
        setting.width == 960000 && setting.tick_per_second == 60 && setting.champion_radius == 10000,
        setting.width, setting.tick_per_second, setting.champion_radius);

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = Default::default();
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();

    let mk = |tag: u64| -> BigPlan {
        let mut raw = [0u8; 384];
        raw[0..8].copy_from_slice(&tag.to_le_bytes());
        unsafe { std::mem::transmute(raw) }
    };

    // ══════════ A) C 블록 — 레코드 22필드 대응표 ══════════
    // plan = PassiveLine(태그3) ⟹ end_plan 1 이어야 한다(명세 consts[2]).
    println!("\n#A\tC블록 레코드 필드 대응 (plan=PassiveLine)");
    println!("#A\tfield\tgot\texpect\tverdict");
    {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(3);
        let l = Live::default();
        unsafe { inject_live(&mut h, &l); set_abort_src(&mut h, 0x5A); }
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        println!("A_meta\tepisodes_len\t{}\texp>=1\t{}", h.v50_dive_episodes.len(),
                 if h.v50_dive_episodes.len() >= 1 { "OK" } else { "NO-PUSH" });
        println!("A_meta\tlive_tag_after\t{}\texp=-1(take)\t{}", unsafe { live_tag(&h) },
                 if unsafe { live_tag(&h) } == -1 { "OK" } else { "MISMATCH" });
        if let Some(e) = h.v50_dive_episodes.first() {
            let mut bad = 0;
            macro_rules! f { ($n:expr, $g:expr, $e:expr) => {{
                let (g, ex) = ($g as i64, $e as i64);
                if g != ex { bad += 1; }
                println!("A\t{}\t{}\t{}\t{}", $n, g, ex, if g == ex { "MATCH" } else { "MISMATCH" });
            }}}
            f!("max_catch_break", e.max_catch_break, l.max_catch_break);
            f!("uncatch_total", e.uncatch_total, l.uncatch_total);
            f!("start_tick", e.start_tick, l.start_tick);
            f!("end_tick", e.end_tick, l.last_tick);
            f!("ep_ticks", e.ep_ticks, l.ep_ticks);
            f!("in_range_ticks", e.in_range_ticks, l.in_range_ticks);
            f!("holder_ticks", e.holder_ticks, l.holder_ticks);
            f!("team_holder_ticks", e.team_holder_ticks, l.team_holder_ticks);
            f!("minion_cover_ticks", e.minion_cover_ticks, l.minion_cover_ticks);
            f!("soaked_hp", e.soaked_hp, l.soaked_hp);
            f!("start_race_adv", e.start_race_adv, l.start_race_adv);
            f!("start_model", e.start_model, l.start_model);
            f!("start_na", e.start_na, l.start_na);
            f!("start_ne", e.start_ne, l.start_ne);
            f!("start_tgt_hp", e.start_tgt_hp, l.start_tgt_hp);
            f!("start_in_range", e.start_in_range as u8, l.start_in_range);
            println!("A\ttarget_pos\t{:?}\texp_tag={}\t", e.target_pos, l.target_pos);
            println!("A\ttower\t{:?}\texp_tag={}\t", e.tower, l.tower);
            println!("A\tend_reason\t{}\t(호출부가 정한다)\t", e.end_reason);
            println!("A\tend_plan\t{}\texp=1(PassiveLine)\t{}", e.end_plan,
                     if e.end_plan == 1 { "MATCH" } else { "MISMATCH" });
            println!("A\taborted\t{}\t(호출부가 정한다)\t", e.aborted);
            println!("A\tabort_src\t{}\t(aborted?0x5A:0)\t", e.abort_src);
            println!("A_SUMMARY\tmismatch={}", bad);
        }
    }

    // ══════════ B) end_plan 분류 사슬 — plan 태그를 바꿔 가며 ══════════
    // 5차는 `BigPlan::get_name` 16종만 실행했고 **사슬 적용부는 private 이라 ev4** 로 남겼다.
    // 이제 사슬 자체를 실행으로 가른다.
    // ★1차 시도에서 10/16 MISMATCH — 원인은 명세 오류가 아니라 **`update` 가 fold 전에 plan 을
    //   갈아치우기 때문**이었다(아래 B 표의 name_after_update 열이 그 증거).
    //   ⟹ 기대값을 **fold 시점의 이름**에서 세워 사슬 자체를 가른다. B2 는 이름이 안 바뀐 케이스만 뽑는다.
    println!("\n#B\ttag\tinjected\tname_after_update\tend_plan\texpect_from_name\tverdict");
    let names: [(u64, &str, u8); 16] = [
        (0,  "DeathMatchBattle(untagged)", 9), (2,  "ForcePassive", 9),
        (3,  "PassiveLine", 1), (4,  "SinglePlanLine", 9),
        (5,  "SinglePlanBattle", 9), (7,  "PassiveJungle", 2),
        (8,  "ActiveRecall", 6), (9,  "Battle", 7),
        (10, "LineGanker", 3), (11, "LineGankCover", 3),
        (12, "EpicHuntAndPoke", 4), (13, "EpicHuntAndBattle", 4),
        (14, "SerpenHuntAndPoke", 5), (15, "SerpenHuntAndBattle", 5),
        (16, "AttackNexus", 8), (17, "DefenseNexus", 8),
    ];
    let mut bbad = 0;
    let mut bn = 0;
    for (tag, vn, _exp) in names {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(tag);
        unsafe { inject_live(&mut h, &Live::default()); }
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        let after = h.plan.get_name();
        let exp = end_plan_of(&after);
        match h.v50_dive_episodes.first() {
            Some(e) => {
                bn += 1;
                if e.end_plan != exp { bbad += 1; }
                println!("B\t{}\t{}\t{:?}\t{}\t{}\t{}", tag, vn, after, e.end_plan, exp,
                         if e.end_plan == exp { "MATCH" } else { "MISMATCH" });
            }
            None => { bbad += 1; println!("B\t{}\t{}\t{:?}\t-\t{}\tNO-PUSH", tag, vn, after, exp); }
        }
    }
    println!("B_SUMMARY\tcases={}\tmismatch={}", bn, bbad);

    // ══════════ B2) `update` 가 plan 을 안 바꾼 케이스만 — 주입→분류 인과 ══════════
    println!("\n#B2\ttag\tname_before\tname_after\tend_plan\texpect\tverdict");
    let mut b2bad = 0;
    let mut b2n = 0;
    for (tag, _vn, exp) in names {
        let before_name = mk(tag).get_name();
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(tag);
        unsafe { inject_live(&mut h, &Live::default()); }
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        let after = h.plan.get_name();
        if after != before_name { continue; }
        if let Some(e) = h.v50_dive_episodes.first() {
            b2n += 1;
            if e.end_plan != exp { b2bad += 1; }
            println!("B2\t{}\t{:?}\t{:?}\t{}\t{}\t{}", tag, before_name, after, e.end_plan, exp,
                     if e.end_plan == exp { "MATCH" } else { "MISMATCH" });
        }
    }
    println!("B2_SUMMARY\tstable_cases={}\tmismatch={}", b2n, b2bad);

    // ══════════ C) A 블록 게이트 — knobs[1](in_range_ticks==0) · knobs[2](team_holder_ticks==0) ══════
    // last_dive_abandon_tick(+0x1480, pub) = max(prev, live.last_tick) 가 조건부로만 올라야 한다.
    println!("\n#C\tin_range\tteam_holder\tlast_tick\tabandon_after\texpect\tverdict");
    for (ir, th) in [(0u64, 0u64), (1, 0), (0, 1), (1, 1), (0, 0)] {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(3);
        let mut l = Live::default();
        l.in_range_ticks = ir; l.team_holder_ticks = th; l.last_tick = 4242;
        unsafe { inject_live(&mut h, &l); }
        let before = h.last_dive_abandon_tick;
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        let after = h.last_dive_abandon_tick;
        let exp = if ir == 0 && th == 0 { 4242usize } else { before };
        println!("C\t{}\t{}\t4242\t{}\t{}\t{}", ir, th, after, exp,
                 if after == exp { "MATCH" } else { "MISMATCH" });
    }

    // ══════════ D) max() 인지 — 기존값이 더 크면 안 내려가야 한다 ══════════
    println!("\n#D\tprev\tlive.last_tick\tafter\texpect=max\tverdict");
    for (prev, lt) in [(0usize, 4242u64), (9999, 4242), (4242, 4242), (1, 4242)] {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(3);
        h.last_dive_abandon_tick = prev;
        let mut l = Live::default(); l.last_tick = lt;
        unsafe { inject_live(&mut h, &l); }
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        let exp = prev.max(lt as usize);
        println!("D\t{}\t{}\t{}\t{}\t{}", prev, lt, h.last_dive_abandon_tick, exp,
                 if h.last_dive_abandon_tick == exp { "MATCH" } else { "MISMATCH" });
    }

    // ══════════ E) 라이브가 없으면 push 가 없다(take 는 무조건 실행) ══════════
    println!("\n#E\tlive\tepisodes_len\tabandon\tverdict");
    {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(3);
        let before_tag = unsafe { live_tag(&h) };
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        println!("E\tNone(tag={})\t{}\t{}\t{}", before_tag, h.v50_dive_episodes.len(),
                 h.last_dive_abandon_tick,
                 if h.v50_dive_episodes.is_empty() { "OK(무push)" } else { "MISMATCH" });
    }

    // ══════════ F) abort_src 게이트 — aborted=false 면 0 으로 지워지나 ══════════
    println!("\n#F\tabort_src_injected\taborted\tabort_src_in_record\tverdict");
    for src in [0u8, 0x5A, 0xFF] {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(3);
        unsafe { inject_live(&mut h, &Live::default()); set_abort_src(&mut h, src); }
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        match h.v50_dive_episodes.first() {
            Some(e) => println!("F\t{}\t{}\t{}\t{}", src, e.aborted, e.abort_src,
                                if !e.aborted && e.abort_src == 0 { "OK(지워짐)" }
                                else if e.aborted && e.abort_src == src { "OK(복사됨)" } else { "CHECK" }),
            None => println!("F\t{}\t-\t-\tNO-PUSH", src),
        }
    }

    // ══════════ G) end_plan 코드 2~6 을 실제로 내게 하기 ══════════
    // B 에서 `update` 가 plan 을 갈아치우는 것이 확인됐으니, **update 가 그 플랜을 고르도록**
    // 포지션·HP 를 바꿔 코드 2(PassiveJungle)·6(ActiveRecall/Recall) 등을 유도한다.
    println!("\n#G\tteam\tpos\thp\tname_after\tend_plan\texpect_from_name\tverdict");
    let mut gbad = 0; let mut gn = 0;
    for t in 0..2usize {
        for pi in 0..5usize {
            for hp_ratio in [100u64, 5u64] {
                let pl = match game.get_player_by_position(t, poss[pi]) { Some(x) => x, None => continue };
                // 챔프 hp 를 낮춰 회복/귀환 계열을 유도한다(Entity 전 필드 pub — TEMPLATE ⑦)
                let mut g2 = Game::new(1234u64, false, &setting, &ms, &map);
                let mut r0 = rand::rngs::StdRng::seed_from_u64(7);
                let mut id2 = 0usize;
                for tt in 0..2usize { for pp in 0..5usize {
                    let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
                    let mut st: AthleteStat = Default::default();
                    st.judgement = 80; st.mental = 60;
                    g2.add_player(GamePlayer::new(id2, "p", tt, poss[pp], st, "swordman", ci, Vec::new()));
                    id2 += 1;
                }}
                g2.start_game(&mut r0, &ctx);
                g2.world.tick = TICK;
                let c2 = AbstractGameWithCache::new(&g2 as &dyn AbstractGame, &ctx);
                if hp_ratio != 100 {
                    if let Some(e) = c2.player_champion[t][pi] {
                        let ep = e as *const Entity as *mut Entity;
                        unsafe { (*ep).hp = (*ep).stat_cached.hp * (hp_ratio as usize) / 100; }
                    }
                }
                let bb2: [Blackboard; 2] = Default::default();
                let d2 = OperationData::new(&c2, &ctx, &bb2);
                let pl2: &PlayerState = g2.get_player_by_position(t, poss[pi]).unwrap();
                let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, poss[pi]);
                h.plan = mk(3);
                unsafe { inject_live(&mut h, &Live::default()); }
                let mut dbg: DebugFrameData = Default::default();
                let mut r = rand::rngs::StdRng::seed_from_u64(11);
                h.update(3usize, &mut r, pl2, &d2, &mut dbg, false);
                let after = h.plan.get_name();
                let exp = end_plan_of(&after);
                if let Some(e) = h.v50_dive_episodes.first() {
                    gn += 1;
                    if e.end_plan != exp { gbad += 1; }
                    println!("G\t{}\t{:?}\t{}\t{:?}\t{}\t{}\t{}", t, poss[pi], hp_ratio, after,
                             e.end_plan, exp, if e.end_plan == exp { "MATCH" } else { "MISMATCH" });
                }
            }
        }
    }
    println!("G_SUMMARY\tcases={}\tmismatch={}", gn, gbad);

    // ══════════ H) 06 로 가는 pub 경로가 정말 없나 — 명세 06 history 「차단(실측)」 반증 ══════════
    //   IR: update(m13.ll:14467) → handle_interact_battle(호출 m13.ll:20242)
    //        → v2_response_retreat_stance(호출 m13.ll:38079·38761)
    //   `update` 는 pub 이고 위에서 실제로 실행됐다. sub_goal 은 BigPlan+0x8(Battle) +0x58 ⟹ handler+0x648.
    println!("\n#H\tplan\tsub_goal_tag\tfocus\tnote");
    for (tag, nm) in [(9u64, "Battle"), (3u64, "PassiveLine")] {
        let mut h = LegacyPlanHandler::new(3usize, &mut rnd0, 60usize, Position::Top);
        h.plan = mk(tag);
        let mut dbg: DebugFrameData = Default::default();
        let mut r = rand::rngs::StdRng::seed_from_u64(11);
        h.update(3usize, &mut r, player, &data, &mut dbg, false);
        let p = &h as *const LegacyPlanHandler as *const u8;
        let sg: i64 = unsafe { std::ptr::read_unaligned(p.add(0x5e8 + 0x8 + 0x58) as *const i64) };
        let fo: i64 = unsafe { std::ptr::read_unaligned(p.add(0x5e8 + 0x8 + 0x60) as *const i64) };
        println!("H\t{}\t{}\t{}\t after_name={:?}", nm, sg, fo, h.plan.get_name());
    }
    println!("H_NOTE\t3=KitingBack / 4=RunAway (tcxdict --enum BattleSubPlanGoal)");

    println!("\nDONE");
}
