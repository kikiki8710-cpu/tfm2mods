#![allow(unused, dead_code, non_snake_case)]
//! B5_o5 — 05 `BigPlan::get_name` 16 variant 전수 + 06 피호출 술어 4종 실행검증
//!
//! ## A. 05 — end_plan 분류표 완성
//! 2차는 8 variant 만 실행했고 나머지 8 은 「private 필드라 구성 불가」로 남겼다.
//! `BigPlan`(384B, 태그 8B @+0x0, 니치 untagged=DeathMatchBattle) 을 **0 페이로드 + 태그만** 세워
//! `transmute` 로 만들어 16종 전부 실행한다. 검증 대상:
//!   · variant → 이름 문자열 (특히 ForcePassive 의 원문 오타 "ForcePasive")
//!   · 명세의 분류 사슬을 그대로 돌려 end_plan 1..9
//!   · ★「`starts_with("Recall")` 분기는 죽은 가지」 — 16종 중 "Recall" 로 시작하는 이름이 하나도 없는지
//!   · ★「SinglePlanBattle/DeathMatchBattle 은 starts_with("Battle") 이 아니라 9」
//! ⚠0 페이로드는 UB 가능성이 있어 `catch_unwind` + `forget` 으로 감싼다(검증 전용).
//!
//! ## B. 06 — 담당 함수가 private 이므로 **피호출 술어를 개별 실행**한다
//!   · `Entity::distance_sq` = |dx|²+|dy|² (abs_diff)
//!   · `Blackboard::is_recent_visible` 의 120틱 창 + blackboard 인덱스 의미
//!   · `is_ignored_well_enemy(version, player, entity)`
//!   · `check_kill_die_tick` 의 towers 인자 영향 (06 은 항상 빈 Vec 을 넘긴다)
//!     ⚠TLS 메모(캐시 키 = 엔티티 id)를 피하려 **케이스마다 focus 엔티티 id 를 다르게** 준다.
use game_core::*;
use game_ai::plan_legacy::types::BigPlan;
use rand::SeedableRng;
use std::panic;
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

/// 명세 05 의 end_plan 분류 사슬 (dive_episode.rs:146~153) 그대로
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
    panic::set_hook(Box::new(|_| {}));

    // ─────────────────────────── A. 05 BigPlan::get_name ×16
    println!("#A\ttag\tvariant\tname\tend_plan\tstarts_Recall\tnote");
    let names = [
        (0u64,  "DeathMatchBattle(untagged)"),
        (2,  "ForcePassive"), (3,  "PassiveLine"), (4,  "SinglePlanLine"),
        (5,  "SinglePlanBattle"), (7,  "PassiveJungle"), (8,  "ActiveRecall"),
        (9,  "Battle"), (10, "LineGanker"), (11, "LineGankCover"),
        (12, "EpicHuntAndPoke"), (13, "EpicHuntAndBattle"),
        (14, "SerpenHuntAndPoke"), (15, "SerpenHuntAndBattle"),
        (16, "AttackNexus"), (17, "DefenseNexus"),
    ];
    let mut any_recall_prefix = false;
    let mut got = 0usize;
    for (tag, vn) in names {
        let r = panic::catch_unwind(|| {
            let mut raw = [0u8; 384];
            raw[0..8].copy_from_slice(&tag.to_le_bytes());
            let bp: BigPlan = unsafe { std::mem::transmute(raw) };
            let s = bp.get_name();
            std::mem::forget(bp);
            s
        });
        match r {
            Ok(s) => {
                let ep = end_plan_of(&s);
                let sr = s.starts_with("Recall");
                if sr { any_recall_prefix = true; }
                got += 1;
                println!("A\t{}\t{}\t{:?}\t{}\t{}\t", tag, vn, s, ep, sr);
            }
            Err(_) => println!("A\t{}\t{}\t<PANIC>\t-\t-\t0 페이로드로는 구성 불가", tag, vn),
        }
    }
    println!("A_SUMMARY\tvariants_ok={}/16\tany_name_starts_with_Recall={}", got, any_recall_prefix);
    println!("A_NOTE\tany_name_starts_with_Recall=false 이면 `starts_with(\"Recall\")->6` 은 죽은 가지");

    // ─────────────────────────── B. 06 피호출 술어
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
    for t in 0..2usize { for y in 0..30usize { for x in 0..30usize { game.world.visible_map[t][y][x] = 0; } } }

    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let proto: Entity = cache.player_champion[0][0].unwrap().clone();
    let eproto: Entity = cache.player_champion[1][0].unwrap().clone();

    // B1) Entity::distance_sq = |dx|^2 + |dy|^2
    println!("\n#B1\tax,ay\tbx,by\tgame\ttheory\tverdict");
    let mut bad = 0;
    for (ax, ay, bx2, by2) in [(0u64,0u64,3u64,4u64), (100,200,100,200), (960000,960000,0,0),
                               (500000,100000,200000,400000), (1,2,4,6)] {
        let mut a = proto.clone(); a.x = ax; a.y = ay;
        let mut b = proto.clone(); b.x = bx2; b.y = by2;
        let g = a.distance_sq(&b);
        let dx = ax.abs_diff(bx2); let dy = ay.abs_diff(by2);
        let t = dx*dx + dy*dy;
        if g != t { bad += 1; }
        println!("B1\t{},{}\t{},{}\t{}\t{}\t{}", ax,ay,bx2,by2,g,t, if g==t {"MATCH"} else {"MISMATCH"});
    }
    // utils::distance_sq 와 동일한지
    println!("B1b\tutils::distance_sq(0,0,3,4)={}\tEntity::distance_sq={}",
        game_core::utils::distance_sq(0,0,3,4),
        { let mut a=proto.clone(); a.x=0;a.y=0; let mut b=proto.clone(); b.x=3;b.y=4; a.distance_sq(&b) });

    // B2) Blackboard::is_recent_visible — 120틱 창 + 인덱스 의미
    println!("\n#B2\telapsed\tbb_index\tgame\ttheory(<=120)\tverdict");
    for idx in [0usize, 1] {
        for elapsed in [0usize, 119, 120, 121, 5000] {
            let mut bb: Blackboard = Default::default();
            for p in 0..5usize { bb.last_visible[p] = 0; }
            if elapsed <= TICK { bb.last_visible[0] = TICK - elapsed; }
            let g = bb.is_recent_visible(&game as &dyn AbstractGame, player, &eproto);
            // bb[1] = team0 이 team1 을 본 기록. eproto = team1 Top(position idx 0)
            let t = if idx == 1 { elapsed <= 120 } else { elapsed <= 120 };
            if g != t { bad += 1; }
            println!("B2\t{}\t{}\t{}\t{}\t{}", elapsed, idx, g, t, if g==t {"MATCH"} else {"MISMATCH"});
            break;
        }
    }
    // 위 루프는 구조가 헷갈리니 평탄하게 다시
    println!("#B2b\telapsed\tgame\texp(elapsed<=120)\tverdict");
    for elapsed in [0usize, 60, 119, 120, 121, 200, 5000] {
        let mut bb: Blackboard = Default::default();
        for p in 0..5usize { bb.last_visible[p] = 0; }
        bb.last_visible[0] = TICK.saturating_sub(elapsed);
        let g = bb.is_recent_visible(&game as &dyn AbstractGame, player, &eproto);
        let t = elapsed <= 120;
        if g != t { bad += 1; }
        println!("B2b\t{}\t{}\t{}\t{}", elapsed, g, t, if g==t {"MATCH"} else {"MISMATCH"});
    }
    // 다른 슬롯을 올려도 무영향(= owner.info.position 으로 인덱싱)
    {
        let mut bb: Blackboard = Default::default();
        for p in 0..5usize { bb.last_visible[p] = TICK; }
        bb.last_visible[0] = 0;
        let g = bb.is_recent_visible(&game as &dyn AbstractGame, player, &eproto);
        println!("B2c\tonly_slot0_stale\t{}\texp=false\t{}", g, if !g {"MATCH"} else {"MISMATCH"});
        if g { bad += 1; }
    }

    // B3) is_ignored_well_enemy — 적이 자기 분수대 안에 있으면?
    println!("\n#B3\tenemy_xy\tresult\tin_enemy_fountain");
    let f1 = map.fountains[1];
    for (x, y) in [(f1.0, f1.1), ((f1.0+f1.2)/2, (f1.1+f1.3)/2), (f1.2, f1.3),
                   (480000, 480000), (f1.0 - 1, f1.1), (0, 0)] {
        let mut e = eproto.clone(); e.x = x; e.y = y;
        let r = game_ai::plan_legacy::old::is_ignored_well_enemy(3, player, &e);
        let inf = x >= f1.0 && x <= f1.2 && y >= f1.1 && y <= f1.3;
        println!("B3\t{},{}\t{}\t{}", x, y, r, inf);
    }

    // B4) check_kill_die_tick — towers 인자 영향 (focus id 를 매번 바꿔 TLS 메모 회피)
    let _ = panic::take_hook();
    panic::set_hook(Box::new(|i| { println!("B4_PANIC_INFO\t{}", i); }));
    println!("\n#B4\tcase\tdie_tick\tnote");
    let mut mkfocus = |id: usize, hp: usize| { let mut e = proto.clone(); e.id = id; e.hp = hp; e };
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(3);
    // ★focus 는 **실제 등록된 챔피언 엔티티**여야 한다(가짜 id 를 주면 fight_check.rs:979 의
    //   Option::unwrap 이 None 으로 패닉한다 — 1회차 실측). 케이스마다 다른 슬롯을 써
    //   TLS 메모(캐시 키 = 엔티티 id) 충돌도 함께 피한다.
    for (i, (ntow, nen)) in [(0usize,0usize),(0,1),(1,0),(1,1),(2,1)].iter().enumerate() {
        let f: Entity = cache.player_champion[0][i].unwrap().clone();
        let mut ens: Vec<Entity> = Vec::new();
        for k in 0..*nen { let mut e = cache.player_champion[1][k].unwrap().clone(); e.x = f.x + 20000; e.y = f.y; ens.push(e); }
        let mut tws: Vec<Entity> = Vec::new();
        for k in 0..*ntow { let mut e = cache.player_champion[1][3+k].unwrap().clone(); e.x = f.x + 20000; e.y = f.y; tws.push(e); }
        let mut ev: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        for e in ens.iter() { ev.push(e); }
        let mut tv: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        for e in tws.iter() { tv.push(e); }
        let bb2: [Blackboard; 2] = [Default::default(), Default::default()];
        let od = OperationData::new(&cache, &ctx, &bb2);
        let r = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let mut rr = rand::rngs::StdRng::seed_from_u64(3);
            let mut dd: DebugFrameData = Default::default();
            game_ai::check_kill_die_tick(3, &mut rr, &od, player, &f, ev, tv, &mut dd)
        }));
        match r {
            Ok(d) => println!("B4\ttowers={} enemies={}\t{}\tfocus_id={}", ntow, nen, d, f.id),
            Err(_) => println!("B4\ttowers={} enemies={}\t<PANIC>\tfocus_id={}", ntow, nen, f.id),
        }
    }

    println!("\nSUMMARY\tmismatch={}", bad);
}
