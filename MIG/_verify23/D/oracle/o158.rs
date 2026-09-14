#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #158 line_minion_action_candidates (pub · game_ai::line_minion_action_candidates 직접 호출) + 독립 재구현 대조
//! 미니언 = 1728B 제로버퍼 Entity(ty 태그 1 · line +0x11a · x/y · visible_state[0] · radius · id · team Player(1)) 를 cache.top/mid/bottom_minions[1] 에 raw 주입.
//! attack_effect 는 사거리 0 Effect 로 두어(None 이면 can_attack 이 true 라 :104 unwrap 패닉 실측) should_suppress_direct_minion_attack(잎, 미재현) 경로를 끊고 Skill/Skill2 후보만 대조한다.
//! 사용: o158.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400; s.visible_distance = 130000; s.tick_per_second = 60;
    s.champion_radius = 10000; s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40; s.kill_gold = 300;
    s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7; s.return_tick = 120; s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000; s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150; s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20; s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20; s.support_gold_reduction = 15; s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
    s
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default(); st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new())); pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn p32(base: *mut u8, off: usize, v: i32) { std::ptr::write_volatile(base.add(off) as *mut i32, v); }
unsafe fn p8(base: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(base.add(off), v); }
unsafe fn r64(base: *const u8, off: usize) -> i64 { std::ptr::read_volatile(base.add(off) as *const i64) }
unsafe fn r8(base: *const u8, off: usize) -> u8 { std::ptr::read_volatile(base.add(off)) }
fn mk_effect(range: u64, growth: u64, casting: CastingType, target: CastingTarget) -> Effect {
    Effect { range, growth_range: growth, start_timing: 0, casting, target,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, attack_type: AttackType::Skill }
}
#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);
/// 가짜 미니언: (line, dx, dy, visible_tag)
unsafe fn mk_minion(id: i64, line: u8, x: i64, y: i64, vis: i64) -> *const Entity {
    let b: &'static mut Buf<1728> = Box::leak(Box::new(Buf([0u8; 1728])));
    let bp = b.0.as_mut_ptr();
    p64(bp, 0x0, 0); p64(bp, 0x8, 1);           // team = Player(1)
    p64(bp, 0x38, vis); p64(bp, 0x38 + 24, 0);  // visible_state[0]=vis, [1]=Visible
    p64(bp, 0x68, 1); p8(bp, 0x11a, line);      // ty = Minion, info.line
    p64(bp, 0x5c0, id); p64(bp, 0x660, x); p64(bp, 0x668, y);
    p64(bp, 0x680, 5000); p32(bp, 0x470, 0);    // radius 5000, radius_mult 0
    p64(bp, 0x670, 500); p64(bp, 0x628, 500);   // hp / max_hp
    p8(bp, 0x6b9, 1); p64(bp, 0x6a0, 0);       // can_target=true · block_target_tick=0 (CastingTarget::check 선행 가드 g06.ll:86753~86759)
    bp as *const Entity
}
fn d2(a: &Entity, b: &Entity) -> u64 {
    let dx = (a.x as i64 - b.x as i64).unsigned_abs(); let dy = (a.y as i64 - b.y as i64).unsigned_abs();
    dx * dx + dy * dy
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms, map: &map,
        champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let ver = 57usize;

    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ: &Entity = cache.player_champion[0][2].expect("champ");
    let cx = champ.x as i64; let cy = champ.y as i64;
    // 케이스: skill(range,target) · skill2 · level · neutral · minions: (line, dx, dy, vis)
    let mut skill: Option<(u64, CastingTarget)> = None;
    let mut skill2: Option<(u64, CastingTarget)> = None;
    let mut level: i64 = 1; let mut neutral = false; let mut champ_ms: i64 = 0;
    let mut mins: Vec<(u8, i64, i64, i64)> = Vec::new();
    let line = LineType::Top;   // 인자 line = Top(0)
    match case {
        0 => { skill = Some((200000, CastingTarget::Enemy)); }                                         // 미니언 없음 → 빈
        1 => { skill = Some((200000, CastingTarget::Enemy)); mins = vec![(0, 25000, 0, 0), (0, 30000, 5000, 0), (0, 50000, -5000, 0)]; }   // Top 3 → Skill 3
        2 => { skill = Some((300000, CastingTarget::Enemy)); mins = vec![(1, 100000, 0, 0), (1, 50000, 0, 0), (2, 120000, 0, 0), (0, 150000, 0, 0), (0, 25000, 0, 0)]; }  // 타라인 80000 밖 제외 · 안이면 포함 · 우리 라인은 거리 무관
        3 => { skill = Some((300000, CastingTarget::Enemy)); mins = vec![(1, 80000, 0, 0), (1, 79999, 0, 0), (2, 0, 80000, 0), (2, 0, 79999, 0)]; }  // near 경계(엄격 <)
        4 => { skill = Some((200000, CastingTarget::Enemy)); mins = vec![(0, 25000, 0, 1), (0, 30000, 0, 2), (0, 40000, 0, 0)]; }  // 비가시(1/2) 제외
        5 => { skill = Some((20000, CastingTarget::Enemy)); mins = vec![(0, 25000, 0, 0), (0, 35000, 0, 0), (0, 36000, 0, 0), (0, 70000, 0, 0)]; }  // 사거리 20000+radius 10000+5000+30*ms(1)=35030 → 35000 포함·36000 제외
        6 => { skill = Some((20000, CastingTarget::Enemy)); skill2 = Some((60000, CastingTarget::Enemy)); level = 3; mins = vec![(0, 25000, 0, 0), (0, 50000, 0, 0), (0, 100000, 0, 0)]; }  // 둘 다
        7 => { skill = Some((200000, CastingTarget::Enemy)); neutral = true; mins = vec![(0, 25000, 0, 1), (0, 30000, 0, 2)]; }  // 챔프 Neutral → 가시성 무시
        8 => { skill = Some((200000, CastingTarget::Ally)); mins = vec![(0, 25000, 0, 0)]; }                   // check 실패 → 빈
        9 => { skill = Some((20000, CastingTarget::Enemy)); champ_ms = 500; mins = vec![(0, 35000, 0, 0), (0, 50000, 0, 0), (0, 50100, 0, 0)]; }  // walk 30*500=15000 → 50000 포함 50100 제외
        10 => { skill2 = Some((60000, CastingTarget::Enemy)); level = 1; mins = vec![(0, 25000, 0, 0)]; }    // 레벨 1 → skill2 None → 빈
        _ => {}
    }
    let chp = champ as *const Entity as *mut u8;
    unsafe {
        p64(chp, 0x5c8, level);
        if neutral { p64(chp, 0x0, 1); }
        if champ_ms != 0 { p64(chp, 0x640, champ_ms); }
        // ★attack_effect None 이면 can_attack() 이 여전히 true 라 lane_minion.rs:104 unwrap 패닉(실측) → 사거리 0 인 Effect 로 두어 Attack 후보가 안 나오게 한다
        std::ptr::write(chp.add(0x490) as *mut Option<Effect>, Some(mk_effect(0, 0, CastingType::Targeting, CastingTarget::Enemy)));
        match skill { None => p32(chp, 0x4c8 + 0x30, -1), Some((r, t)) => std::ptr::write(chp.add(0x4c8) as *mut Option<Effect>, Some(mk_effect(r, 0, CastingType::Targeting, t))) }
        match skill2 { None => p32(chp, 0x500 + 0x30, -1), Some((r, t)) => std::ptr::write(chp.add(0x500) as *mut Option<Effect>, Some(mk_effect(r, 0, CastingType::Targeting, t))) }
        // 미니언 주입: 라인별 Vec [team 1]
        let cp = &cache as *const AbstractGameWithCache as *mut u8;
        let mut per_line: [Vec<*const Entity>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (i, (ln, dx, dy, vis)) in mins.iter().enumerate() {
            per_line[*ln as usize].push(mk_minion(1000 + i as i64, *ln, cx + dx, cy + dy, *vis));
        }
        for ln in 0..3 {
            let arr: &'static mut Vec<*const Entity> = Box::leak(Box::new(std::mem::take(&mut per_line[ln])));
            let base = cp.add([0x10, 0x50, 0x90][ln] + 1 * 32);
            p64(base, 0, arr.as_ptr() as i64); p64(base, 24, arr.len() as i64);
        }
    }
    // ── 모델 ──
    let mut model: Vec<(u8, usize)> = Vec::new();
    let mv = champ.stat_cached.move_speed as u64;
    let champ_r = champ.radius() as u64;
    let can_attack = champ.can_attack(); let can_skill = champ.can_skill(); let can_skill2 = champ.can_skill2();
    let sk = champ.skill_effect.as_ref();
    let sk2 = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None };
    let team_tag = unsafe { r64(chp, 0) }; let team_idx = unsafe { r64(chp, 8) } as usize;
    for m in cache.iter_minions(1) {
        let mp = m as *const Entity as *const u8;
        if team_tag & 1 == 0 && unsafe { r64(mp, 0x38 + team_idx * 24) } != 0 { continue; }
        let dd = d2(champ, m);
        let near = dd < 6400000000;
        let is_min = unsafe { r64(mp, 0x68) } == 1; let mline = unsafe { r8(mp, 0x11a) };
        if !near && !(is_min && mline == line as u8) { continue; }
        if can_attack { if let Some(e) = champ.attack_effect.as_ref() {
            let base = e.range as u64 + champ_r + champ.stat_buff_cached.range as u64 + (champ.level as u64 - 1) * e.growth_range as u64;
            let maxd = base + mv * 30 + e.range_adjust(champ, m) as u64 + m.radius() as u64;
            if dd <= maxd * maxd { model.push((15, usize::MAX)); }   // should_suppress 미재현 → 이 표식이 나오면 케이스 설계 오류
        } }
        if can_skill { if let Some(e) = sk { if e.target.check(champ, m) {
            let base = e.range as u64 + champ_r + champ.stat_buff_cached.range as u64 + (champ.level as u64 - 1) * e.growth_range as u64;
            let maxd = base + mv * 30 + e.range_adjust(champ, m) as u64 + m.radius() as u64;
            if dd <= maxd * maxd { model.push((16, m.id)); }
        } } }
        if can_skill2 { if let Some(e) = sk2 { if e.target.check(champ, m) {
            let base = e.range as u64 + champ_r + champ.stat_buff_cached.range as u64 + (champ.level as u64 - 1) * e.growth_range as u64;
            let maxd = base + mv * 30 + e.range_adjust(champ, m) as u64 + m.radius() as u64;
            if dd <= maxd * maxd { model.push((17, m.id)); }
        } } }
    }
    let got: bumpalo::collections::Vec<game_ai::SmallActionPlay> = game_ai::line_minion_action_candidates(ver, &data, player, line);
    let mut got_v: Vec<(u8, usize)> = Vec::new();
    unsafe { let base = got.as_ptr() as *const u8; for i in 0..got.len() { let ep = base.add(i * 184); got_v.push((r8(ep, 0xb1), r64(ep, 8) as usize)); } }
    let ok = got_v == model;
    println!("case={}\tgot={:?}\tmodel={:?}\t{}\t(n_minions={} can_attack={} can_skill={} can_skill2={} lvl={} ms={} champ_r={})",
             case, got_v, model, if ok { "MATCH" } else { "DIFF" }, cache.iter_minions(1).count(), can_attack, can_skill, can_skill2, champ.level, mv, champ_r);
}
