#![allow(unused, dead_code, non_snake_case)]
//! 23차 E · 168 SmallActionAroundBush::get_input 오라클 — `define hidden` 심볼을 #[link_name] 로 직접 진입.
//! 목적 = sret Option<Input>(32B) 의 **살아있는 바이트** 런타임 확정: 출력 버퍼를 0xAA 로 채우고 호출 뒤 32B 덤프.
//!   기대(IR m08.ll:105617·105631·105642): None → [0,8)=-1 만 기록 · Some(Move) → tag 0 + x,y 24B · [24,32) 는 콜리 지역(%15) 사본(미정의).
//! sret 는 Rust ABI 에서 첫 인자(rcx) 이므로 extern 선언에 out ptr 을 첫 인자로 명시하고 -> () 로 받는다.
//! 한 프로세스 = 한 케이스(argv[1]).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvMs6_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_21SmallActionAroundBush9get_input"]
    fn ab_get_input(out: *mut [u8; 32], me: *mut game_ai::SmallActionAroundBush, version: usize, rnd: &mut rand::rngs::StdRng,
                    player: &PlayerState, data: &OperationData, ps: &PositioningScoreData, debug: &mut DebugFrameData);
}

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000; s.respawn_tick = 300; s.respawn_growth = 30;
    s.respawn_growth_term = 1800; s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10; s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100; s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200; s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150; s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999; s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400; s.well_damage = 600; s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1; s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20; s.support_gold_reduction = 15;
    s.support_exp_reduction = 30; s.stamina_zero_debuff_percent = 30;
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
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let n: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    if std::env::args().count() > 99 { let _ = game_ai::can1v1win as *const (); }   // rlib 링크 보장
    let setting = real_setting();
    println!("setting_ok\t{}\ttps={}", setting.width != 0 && setting.tick_per_second != 0 && setting.champion_radius != 0, setting.tick_per_second);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let g = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&g as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].unwrap();
    let enemy = cache.player_champion[1][0].unwrap();
    let player = g.get_player_by_position(0, Position::Top).unwrap();
    let ps: PositioningScoreData = Default::default();
    let mut debug: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(99);
    let version = 2usize;

    // 목표 = 케이스별. 0: 내 근처(도달 가능) · 1: 적 스폰(멀리) · 2: 맵 밖 좌표(경로 실패 기대) · 3: 자기 자리
    let cp = champ as *const Entity as *mut Entity;
    let (tx, ty) = match n {
        0 => (champ.x + 64000, champ.y - 64000),
        1 => (enemy.x, enemy.y),
        2 => (5_000_000u64, 5_000_000u64),
        _ => (champ.x, champ.y),
    };
    let mut tgt_ent: Entity = unsafe { std::ptr::read(champ) };   // new_with_target 는 &Entity 의 x,y 를 target 으로 쓴다
    let tp = &mut tgt_ent as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*tp).x, tx); std::ptr::write_volatile(&mut (*tp).y, ty); }
    let mut me = game_ai::SmallActionAroundBush::new_with_target(&data, unsafe { &*tp }, version, game_ai::AroundBushOutlineType::None);
    println!("self\ttarget=({},{})\tchamp=({},{})\tsizeof={}", tx, ty, champ.x, champ.y, std::mem::size_of::<game_ai::SmallActionAroundBush>());
    let sp = &mut me as *mut game_ai::SmallActionAroundBush as *mut u8;
    let before: Vec<u8> = unsafe { std::slice::from_raw_parts(sp, 120).to_vec() };
    let mut out = [0xAAu8; 32];
    for round in 0..2 {
        let rnd0 = rnd.clone();
        unsafe { ab_get_input(&mut out, &mut me, version, &mut rnd, player, &data, &ps, &mut debug); }
        { use rand::RngCore; let mut probe = rnd0.clone(); let mut k = 0usize; let mut hit = probe == rnd;
          while !hit && k < 4096 { let _ = probe.next_u32(); k += 1; hit = probe == rnd; }
          println!("rnd	round={}	changed={}	draws_u32={}", round, rnd0 != rnd, if hit { k as i64 } else { -1 }); }
        let tag = i64::from_le_bytes(out[0..8].try_into().unwrap());
        let a = u64::from_le_bytes(out[8..16].try_into().unwrap());
        let b = u64::from_le_bytes(out[16..24].try_into().unwrap());
        let tail: Vec<String> = out[24..32].iter().map(|x| format!("{:02x}", x)).collect();
        let after: Vec<u8> = unsafe { std::slice::from_raw_parts(sp, 120).to_vec() };
        let pf_tag = after[0x6d];
        let changed: Vec<usize> = (0..120).filter(|i| before[*i] != after[*i]).collect();
        println!("o168\tcase={}\tround={}\ttag={}\t+8={}\t+16={}\t+24..32={}\tself.path_finder_tag@0x6d={}\tself_changed_bytes={:?}",
                 n, round, tag, a, b, tail.join(""), pf_tag, changed);
        std::mem::forget(std::mem::replace(&mut out, [0xAAu8; 32]));
    }
    std::mem::forget(tgt_ent);
}
