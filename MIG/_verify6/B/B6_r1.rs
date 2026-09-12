#![allow(unused, dead_code, non_snake_case)]
//! B5_o1 — 09 `check_favorable_engage_formation` **임계 격리 진리표**
//!
//! 목적: 2·4차가 "합성 동작"을 400/400·1800/1800 으로 맞춘 것과 달리,
//!       **상수 하나하나의 경계를 단독으로 갈라** ev2 근거를 만든다.
//!  - 40   (HP%)          : fight_check.rs:1240
//!  - 100000 (여유거리)    : 1246
//!  - 4    (is_front cos²) : 1280
//!  - 9    (rear cos²)     : 1283
//!  - 9    (flank sin²)    : 1286
//!  - 5/6  (최종 거리비)   : 1340
//!  - 1    (제로벡터)      : 1223 / 1256
//!  - 인원조건 rear>0 / flank>1 / flank>0&&front>0 / front>1 : 1316/1321/1331
//!  - ★1307 `else -> front` 가 **도달 불가**인지 (대수 증명의 실행 확인)
//!
//! 템플릿 정본(_verify3/TEMPLATE.rs) 함정 ①②를 지킨다: real_setting + init_tower/nexus 미호출.
//! 케이스별 프로세스 분리도 지원(argv 에 케이스명) — TLS 메모 오염 여부 교차확인용.
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

pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

/// 아군 슬롯 기술서: (dx_from_enemy, dy_from_enemy, hp, max_hp) — None = 빈 슬롯
#[derive(Clone, Copy)]
struct Ally { dx: i64, dy: i64, hp: usize, maxhp: usize }

struct Case {
    name: &'static str,
    ex: u64, ey: u64,                 // target_enemy 좌표
    champ: (i64, i64),                // champ 좌표 = base + (dx,dy)  (기준: 적 분수 중심)
    allies: [Option<Ally>; 4],        // 슬롯 1..4 (슬롯0 = champ)
    engage_range: u64,
    expect: Option<bool>,             // 이론 예측 (있으면 대조)
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let only: Option<&str> = if argv.len() > 1 { Some(argv[1].as_str()) } else { None };

    let setting = real_setting();
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}",
        setting.width != 0 && setting.height != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0,
        setting.width, setting.height, setting.tick_per_second, setting.champion_radius);
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

    // ── 적(1팀) 분수 중심 = 코드가 쓰는 enemy_base
    let f = map.fountains[1];
    let bx = (f.0 + f.2) / 2;
    let by = (f.1 + f.3) / 2;
    println!("fountains[0]\t{:?}", map.fountains[0]);
    println!("fountains[1]\t{:?}", map.fountains[1]);
    println!("enemy_base(team0 기준)\t{}\t{}", bx, by);

    // ── 원본 엔티티 스냅샷(클론해서 좌표/HP 를 통제)
    let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let proto: Entity = cache0.player_champion[0][0].unwrap().clone();
    let penemy: Entity = cache0.player_champion[1][0].unwrap().clone();
    println!("entity_offsets\tx={:#x}\ty={:#x}\thp={:#x}\tstat_cached.hp={:#x}\tid={:#x}",
        (&proto.x as *const u64 as usize) - (&proto as *const Entity as usize),
        (&proto.y as *const u64 as usize) - (&proto as *const Entity as usize),
        (&proto.hp as *const usize as usize) - (&proto as *const Entity as usize),
        (&proto.stat_cached.hp as *const usize as usize) - (&proto as *const Entity as usize),
        (&proto.id as *const usize as usize) - (&proto as *const Entity as usize));

    // E 기본 위치: base 에서 -x 로 500000 (retreat vector = (+500000, 0))
    let e0x: u64 = bx - 500000;
    let e0y: u64 = by;

    // ─────────────────────────────────────────────────────────────
    let mut cases: Vec<Case> = Vec::new();
    let far_champ = (0i64, 900000i64);       // ratio 게이트를 항상 false 로 만드는 champ 위치
    let near_champ = (0i64, 100000i64);      // ratio 게이트 통과
    let none4: [Option<Ally>; 4] = [None, None, None, None];

    // A) HP% 임계 40 — 단독 rear 아군의 HP 만 흔든다
    for hp in [38usize, 39, 40, 41] {
        cases.push(Case { name: "A_hp", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx:100000, dy:0, hp, maxhp:100}), None, None, None],
            engage_range: 200000, expect: Some(hp >= 40) });
    }
    // A2) 정수나눗셈 확인: max=1000, hp=399/400
    for hp in [399usize, 400] {
        cases.push(Case { name: "A2_hpdiv", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx:100000, dy:0, hp, maxhp:1000}), None, None, None],
            engage_range: 200000, expect: Some(hp*100/1000 >= 40) });
    }

    // B) 여유거리 100000 — rear 아군을 +x 로 밀어 max_engage_dist 경계를 넘긴다
    for (er, d) in [(200000u64, 299999i64), (200000, 300000), (200000, 300001),
                    (100000, 199999), (100000, 200000), (100000, 200001),
                    (400000, 500000), (400000, 500001)] {
        cases.push(Case { name: "B_slack", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx:d, dy:0, hp:100, maxhp:100}), None, None, None],
            engage_range: er, expect: Some((d as u64) <= er + 100000) });
    }

    // C) is_front cos² 임계 4 — front 아군 1명 고정 + 시험 아군의 dy 를 흔든다
    //    front 2명 -> ratio(false) / front1+flank1 -> true
    for dy in [173203i64, 173204, 173205, 173206, 173207] {
        let dx = 100000i64;
        let is_front = 3*dx*dx > dy*dy;
        cases.push(Case { name: "C_front4", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}),   // 항상 front
                     Some(Ally{dx:-dx, dy, hp:100, maxhp:100}),         // 시험 (ally_dx=+dx)
                     None, None],
            engage_range: 400000, expect: Some(!is_front) });
    }

    // D) rear cos² 임계 9/100 — 단독 아군, rear(true) vs flank(false)
    for dy in [158987i64, 158988, 158989, 158990, 158991] {
        let dx = 50000i64;
        let is_rear = 91*dx*dx > 9*dy*dy;
        cases.push(Case { name: "D_rear9", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx, dy, hp:100, maxhp:100}), None, None, None],
            engage_range: 200000, expect: Some(is_rear) });
    }

    // E) 최종 거리비 5:6 — front 2명 고정, champ 를 base 에서 멀리/가까이
    //    경계: champ_to_base_sq*5 <= enemy_to_base_sq*6, enemy_to_base_sq = 500000^2
    for d in [547721i64, 547722, 547723, 547724] {
        cases.push(Case { name: "E_ratio56", ex: e0x, ey: e0y, champ: (0, d),
            allies: [Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}),
                     Some(Ally{dx:-200000, dy:0, hp:100, maxhp:100}), None, None],
            engage_range: 400000, expect: Some((d as i128)*(d as i128)*5 <= 500000i128*500000*6) });
    }

    // F) 제로벡터 1223 — 적이 정확히 적 분수 중심 위 => 아군 0명이어도 true
    cases.push(Case { name: "F_zeroR", ex: bx, ey: by, champ: far_champ,
        allies: none4, engage_range: 200000, expect: Some(true) });
    cases.push(Case { name: "F_zeroR_off1", ex: bx+1, ey: by, champ: far_champ,
        allies: none4, engage_range: 200000, expect: Some(false) });

    // G) 아군이 적과 완전히 겹침(1256) => front 취급
    //    겹친아군 + front아군 => front 2 => ratio(false).  만약 flank 취급이면 true 가 된다.
    cases.push(Case { name: "G_overlap_front", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:0, dy:0, hp:100, maxhp:100}),
                 Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}), None, None],
        engage_range: 400000, expect: Some(false) });

    // H) 인원 조건 표
    cases.push(Case { name: "H_rear1", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:100000, dy:0, hp:100, maxhp:100}), None, None, None],
        engage_range: 200000, expect: Some(true) });          // rear>0
    cases.push(Case { name: "H_front1", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}), None, None, None],
        engage_range: 400000, expect: Some(false) });         // front==1
    cases.push(Case { name: "H_front2", ex: e0x, ey: e0y, champ: near_champ,
        allies: [Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}),
                 Some(Ally{dx:-200000, dy:0, hp:100, maxhp:100}), None, None],
        engage_range: 400000, expect: Some(true) });          // front>1 && ratio ok
    // flank 1명 (dx=0, dy 크게) => false
    cases.push(Case { name: "H_flank1", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:0, dy:150000, hp:100, maxhp:100}), None, None, None],
        engage_range: 200000, expect: Some(false) });
    // flank 2명 => true
    cases.push(Case { name: "H_flank2", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:0, dy:150000, hp:100, maxhp:100}),
                 Some(Ally{dx:0, dy:-150000, hp:100, maxhp:100}), None, None],
        engage_range: 200000, expect: Some(true) });
    // flank1 + front1 => true
    cases.push(Case { name: "H_flank1front1", ex: e0x, ey: e0y, champ: far_champ,
        allies: [Some(Ally{dx:0, dy:150000, hp:100, maxhp:100}),
                 Some(Ally{dx:-150000, dy:0, hp:100, maxhp:100}), None, None],
        engage_range: 400000, expect: Some(true) });
    // rear 판정인데 base 거리 타이브레이크 실패 => flank 로 흐름 => 단독이면 false
    //   ally_dx<0 (A.x>E.x) 인데 base 에서 더 먼 위치: dy 를 크게 줘서 to_base > enemy_to_base
    //   dx=50000(=A.x=E.x+50000), dy=400000: to_base_sq=(450000)^2+400000^2=3.625e11 > 2.5e11
    //   rear 조건 91*dx^2 > 9*dy^2 => 2.275e11 > 1.44e12  false  => 이건 flank. (rear-tiebreak 실패 케이스는
    //   기하적으로 구성 불가능한지 자체가 관찰 대상이므로 sweep 로 따로 본다)

    // I) version 무영향 재확인
    for v in [0usize, 1, 2, 3, 46, 50, 60] {
        cases.push(Case { name: "I_version", ex: e0x, ey: e0y, champ: far_champ,
            allies: [Some(Ally{dx:100000, dy:0, hp:100, maxhp:100}), None, None, None],
            engage_range: 200000, expect: Some(true) });
        let _ = v;
    }

    // ─────────────────────────────────────────────────────────────
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    println!("player_team\t{}\tpos={:?}", player.info.team, player.info.position);

    let mut n = 0; let mut mismatch = 0;
    println!("#CASE\tname\tparams\tgame\ttheory\tverdict");
    for (i, c) in cases.iter().enumerate() {
        if let Some(o) = only { if c.name != o { continue; } }
        // 엔티티 조립
        let mut ents: Vec<Entity> = Vec::new();
        let mut champ = proto.clone();
        champ.id = 9000;
        champ.x = (bx as i64 + c.champ.0) as u64;
        champ.y = (by as i64 + c.champ.1) as u64;
        champ.hp = 1000; champ.stat_cached.hp = 1000;
        ents.push(champ);
        for (k, a) in c.allies.iter().enumerate() {
            if let Some(a) = a {
                let mut e = proto.clone();
                e.id = 9100 + k as usize;
                e.x = (c.ex as i64 + a.dx) as u64;
                e.y = (c.ey as i64 + a.dy) as u64;
                e.hp = a.hp; e.stat_cached.hp = a.maxhp;
                ents.push(e);
            }
        }
        let mut tgt = penemy.clone();
        tgt.id = 9500; tgt.x = c.ex; tgt.y = c.ey; tgt.hp = 1000; tgt.stat_cached.hp = 1000;

        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for t in 0..2usize { for p in 0..5usize { cache.player_champion[t][p] = None; } }
        cache.player_champion[0][0] = Some(&ents[0]);
        let mut sl = 1usize;
        for k in 0..4 { if c.allies[k].is_some() { cache.player_champion[0][1+k] = Some(&ents[sl]); sl += 1; } }
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);

        let ver = if c.name == "I_version" { (i % 7) as usize } else { 3usize };
        let got = game_ai::check_favorable_engage_formation(ver, player, &data, &tgt, c.engage_range);
        let th = c.expect;
        let verdict = match th { Some(t) => if t == got { "MATCH" } else { mismatch += 1; "MISMATCH" }, None => "-" };
        let ps = format!("E=({},{}) champ=base+({},{}) er={} allies={:?} ver={}",
            c.ex, c.ey, c.champ.0, c.champ.1, c.engage_range,
            c.allies.iter().filter_map(|a| a.map(|a| (a.dx, a.dy, a.hp, a.maxhp))).collect::<Vec<_>>(), ver);
        println!("CASE\t{}\t{}\t{}\t{:?}\t{}", c.name, ps, got, th, verdict);
        n += 1;
    }
    println!("SUMMARY\tcases={}\tmismatch={}", n, mismatch);

    // ─────────────────────────────────────────────────────────────
    // J) ★1307 `else -> front` 도달 가능성: 난수 스윕으로 분기 카운터를 센다(재현 측 계산)
    //    dot_sq + cross_sq == lp (라그랑주 항등식) 이 정수에서 정확히 성립하는지도 같이 검사한다.
    let mut rng = rand::rngs::StdRng::seed_from_u64(424242);
    use rand::Rng;
    let mut cnt = [0u64; 4]; // front / rear / flank / else1307
    let mut lagrange_bad = 0u64;
    for _ in 0..2_000_000u64 {
        let rdx: i128 = rng.gen_range(-960000i128..=960000);
        let rdy: i128 = rng.gen_range(-960000i128..=960000);
        let adx: i128 = rng.gen_range(-960000i128..=960000);
        let ady: i128 = rng.gen_range(-960000i128..=960000);
        let rl = rdx*rdx + rdy*rdy; if rl < 1 { continue; }
        let al = adx*adx + ady*ady; if al < 1 { continue; }
        let dot = adx*rdx + ady*rdy;
        let cross = adx*rdy - ady*rdx;
        let lp = al * rl;
        if dot*dot + cross*cross != lp { lagrange_bad += 1; }
        let is_front = dot > 0 && dot*dot*4 > lp;
        let is_rear  = dot < 0 && dot*dot*100 > lp*9;
        let is_flank = cross*cross*100 > lp*9;
        if is_front { cnt[0]+=1 } else if is_rear { cnt[1]+=1 } else if is_flank { cnt[2]+=1 } else { cnt[3]+=1 }
    }
    println!("J_sweep\tfront={}\trear={}\tflank={}\tELSE_1307={}\tlagrange_violations={}",
        cnt[0], cnt[1], cnt[2], cnt[3], lagrange_bad);
    // 작은 값 전수 (경계·퇴화 포함)
    let mut cnt2 = [0u64; 4]; let mut lb2 = 0u64;
    for rdx in -6i128..=6 { for rdy in -6i128..=6 { for adx in -6i128..=6 { for ady in -6i128..=6 {
        let rl = rdx*rdx + rdy*rdy; if rl < 1 { continue; }
        let al = adx*adx + ady*ady; if al < 1 { continue; }
        let dot = adx*rdx + ady*rdy; let cross = adx*rdy - ady*rdx; let lp = al*rl;
        if dot*dot + cross*cross != lp { lb2 += 1; }
        let is_front = dot > 0 && dot*dot*4 > lp;
        let is_rear  = dot < 0 && dot*dot*100 > lp*9;
        let is_flank = cross*cross*100 > lp*9;
        if is_front { cnt2[0]+=1 } else if is_rear { cnt2[1]+=1 } else if is_flank { cnt2[2]+=1 } else { cnt2[3]+=1 }
    }}}}
    println!("J_exhaust(-6..6)^4\tfront={}\trear={}\tflank={}\tELSE_1307={}\tlagrange_violations={}",
        cnt2[0], cnt2[1], cnt2[2], cnt2[3], lb2);
}
