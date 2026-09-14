#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #161 lane_minion_position_action (pub · game_ai::lane_minion_position_action 직접 호출) — None 조건 4곳은 모델 대조, Some 경로는 관측(target_score/choose_goal 미재현)
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
    let psd: PositioningScoreData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let ver = 57usize;
    // (position, attack None?, minions(line,dx,dy,vis), end_delay, purpose_tag)
    let (pos, atk_none, mins, end_delay): (Position, bool, Vec<(u8, i64, i64, i64)>, usize) = match case {
        0 => (Position::Support, false, vec![(0, 25000, 0, 0)], 7),          // Support → None
        1 => (Position::Mid, true, vec![(0, 25000, 0, 0)], 7),               // attack_effect None → None
        2 => (Position::Mid, false, vec![], 7),                              // 미니언 없음 → None
        3 => (Position::Mid, false, vec![(1, 200000, 0, 0), (0, 25000, 0, 1)], 7),  // 타라인 + 비가시 → 필터 0 → None
        4 => (Position::Mid, false, vec![(0, 25000, 0, 0), (0, 40000, 3000, 0)], 9),  // 후보 있음 → 관측
        5 => (Position::Top, false, vec![(0, 30000, 0, 0)], 3),              // Top 포지션 · 관측
        _ => (Position::Mid, false, vec![], 0),
    };
    let player: &PlayerState = game.get_player_by_position(0, pos).expect("player");
    let champ: &Entity = cache.player_champion[0][player.info.position as usize].expect("champ");
    let cx = champ.x as i64; let cy = champ.y as i64;
    let chp = champ as *const Entity as *mut u8;
    unsafe {
        if atk_none { p32(chp, 0x4c0, -1); } else {
            std::ptr::write(chp.add(0x490) as *mut Option<Effect>, Some(mk_effect(30000, 0, CastingType::Targeting, CastingTarget::Enemy)));
        }
        let cp = &cache as *const AbstractGameWithCache as *mut u8;
        let mut per_line: [Vec<*const Entity>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (i, (ln, dx, dy, vis)) in mins.iter().enumerate() { per_line[*ln as usize].push(mk_minion(1000 + i as i64, *ln, cx + dx, cy + dy, *vis)); }
        for ln in 0..3 {
            let arr: &'static mut Vec<*const Entity> = Box::leak(Box::new(std::mem::take(&mut per_line[ln])));
            let base = cp.add([0x10, 0x50, 0x90][ln] + 1 * 32);
            p64(base, 0, arr.as_ptr() as i64); p64(base, 24, arr.len() as i64);
        }
    }
    let got: Option<game_ai::SmallActionPlay> = game_ai::lane_minion_position_action(ver, &mut rnd, &data, player, LineType::Top, &psd, None, end_delay, game_ai::PositionEvalPurpose::General);
    let gp = &got as *const Option<game_ai::SmallActionPlay> as *const u8;
    let tag = unsafe { r8(gp, 0xb1) };
    let model_none = case <= 3;
    let ok = if model_none { tag == 0xff } else { tag == 13 || tag == 0xff };
    let mut fields = String::new();
    if tag == 13 { unsafe {
        fields = format!("start_tick={} target={} goal=({},{}) score={} end_delay={} pf_tag={} purpose={}",
            r64(gp, 0), r64(gp, 8), r64(gp, 16), r64(gp, 24), r64(gp, 32), r64(gp, 40), r8(gp, 0x75), r8(gp, 0x78));
    } }
    println!("case={}	tag={}	model={}	{}	({} pos={:?} n_minions={} champ=({},{}))", case, tag,
             if model_none { "None(0xff)" } else { "관측" }, if ok { "MATCH" } else { "DIFF" }, fields, pos, cache.iter_minions(1).count(), cx, cy);
}
