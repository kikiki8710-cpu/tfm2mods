#![allow(unused, dead_code, non_snake_case)]
//! 22차 배치D 오라클 — #130 v55_mark_value (hidden define, m10.ll:33063) — `#[link_name]` 직접 호출
//! Effect 는 56B 제로버퍼 + ty(Arc<dyn EffectType>) 팻포인터만 기록(함수가 ty 만 읽음: m10.ll:33087~33089).
//! Some 경로 = HitmanUltEffect(g10.ll:313142: base=damage·accum_pct=damage_ratio·per_hit=0·max_hits=0·window=mark_duration)
//! None 경로 = AddCastedEffect(기본 impl, g02.ll:309685 initializes((0,8)))
//! 사용: o130.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_mark_value"]
    fn mv(version: usize, effect: *const u8, data: *const OperationData, player: *const PlayerState, param: *const u8,
          champ: *const Entity, t: *const Entity, hp_value: i64, debug: *mut u8) -> i64;
}

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

#[repr(C, align(8))]
struct Buf<const N: usize>([u8; N]);
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_unaligned(base.add(off) as *mut i64, v); }

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::champion_hp_value as *const (); }
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

    // 케이스
    let (mut damage, mut ratio, mut dur, mut hp_value, mut maxhp, mut hp, mut cache_v, mut none_mark, mut t_tower, mut scatter, mut skip_ally)
        = (1000i64, 50i64, 120i64, 10i64, 2000i64, 1500i64, 100i64, false, false, false, false);
    let mut far_t = false;
    match case {
        0 => {}                                   // accum 500*2=1000 → explosion 1000+500=1500 → 1500*10/1500 = 10
        1 => { damage = 100000 }                  // explosion cap 4000 → 26
        2 => { hp_value = 1000 }                  // 1000 → cap 160
        3 => { dur = 30 }                         // window_sec=max(0,1)=1 → accum 500 → 1000+250=1250 → 8
        4 => { hp = 0 }                           // 분모 max(0,1)=1 → 1500*10 = 15000 → 160
        5 => { t_tower = true }                   // t 비플레이어 → 0
        6 => { none_mark = true }                 // expected_mark None → 0
        7 => { cache_v = 10000 }                  // accum_dps 50000*2 → cap 4000 → explosion 1000+2000=3000 → 20
        8 => { scatter = true }                   // 아군 4명 150000 밖 → accum 100*2=200 → 1000+100=1100 → 7
        9 => { ratio = 0; damage = 0 }            // explosion 0 → v=0 (v>0 게이트 false)
        10 => { hp_value = -10 }                  // v 음수 → -10 (smin 160 만, 하한 없음)
        11 => { dur = 600; cache_v = 100 }        // window 10 → accum 5000 → cap 4000 → 1000+2000 → 20
        12 => { far_t = true }                    // t 가 150000 밖(적 기지) → accum 0 → 6
        13 => { cache_v = 30; dur = 180 }          // accum 150*3=450 → 1000+225=1225 → 8
        _ => {}
    }
    let team0: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ: &Entity = cache.player_champion[0][2].expect("champ");
    let t: &Entity = if t_tower { cache.iter_towers_without_nexus(1).next().expect("tower") } else { cache.player_champion[1][0].expect("enemy") };
    let tp = cache.player_by_champion_id(t.id);
    let ti = tp.map(|p| p.info.position as usize).unwrap_or(0);
    unsafe {
        let ep = t as *const Entity as *mut u8;
        p64(ep, 0x628, maxhp); p64(ep, 0x670, hp);
        // ★t 를 아군 진영 옆으로 옮긴다(적 챔프는 자기 기지에 있어 150000 밖 — 1차 실행에서 accum 항이 전부 0 이었다)
        if !far_t { p64(ep, 0x660, champ.x as i64 + 1000); p64(ep, 0x668, champ.y as i64 + 1000); }
        // 아군(팀0) 5명 캐시: attack/skill/skill2/ult _per_sec[ti]
        for p in 0..5 { if let Some(e) = cache.player_champion[0][p] {
            let ap = cache.player_by_champion_id(e.id).expect("ally player");
            let cp = (&cache as *const AbstractGameWithCache as *mut u8).add(0x280 + (ap.info.team * 5 + ap.info.position as usize) * 800);
            p64(cp, 400 + ti * 8, cache_v); p64(cp, 440 + ti * 8, 0); p64(cp, 480 + ti * 8, 0); p64(cp, 520 + ti * 8, 0);
            if scatter && p != 2 { let x = std::ptr::read_unaligned((e as *const Entity as *mut u8).add(0x660) as *const i64);
                p64(e as *const Entity as *mut u8, 0x660, x + 200000 * (p as i64 + 1)); }
        } }
    }
    // Effect 조립
    let ty: Arc<dyn EffectType> = if none_mark {
        Arc::new(AddCastedEffect { effects: Vec::new(), duration: 0, period: 0, casted_type: CastedType::Bleed })
    } else {
        Arc::new(HitmanUltEffect { speed: 0, mark_duration: dur as usize, damage: damage as usize, damage_ratio: ratio as usize, invisible_duration: 0 })
    };
    let mut eb = Box::new(Buf([0u8; 56]));
    let raw: [usize; 2] = unsafe { std::mem::transmute(ty.clone()) };   // (ArcInner*, vtable*)
    eb.0[0..8].copy_from_slice(&raw[0].to_le_bytes()); eb.0[8..16].copy_from_slice(&raw[1].to_le_bytes());
    let param = Box::new(Buf([0u8; 5384]));
    let mut dbg = Box::new(Buf([0u8; 224]));

    // 모델(명세 logic)
    let tps = 60i64;
    let exp = if none_mark || tp.is_none() { 0 } else {
        let window_sec = (dur / tps).max(1);
        let mut accum = 0i64; let mut hits = 0i64;
        for p in 0..5 { if let Some(e) = cache.player_champion[0][p] {
            let dx = (e.x as i64 - t.x as i64).abs(); let dy = (e.y as i64 - t.y as i64).abs();
            if dx * dx + dy * dy > 22500000000 { continue }
            accum += cache_v; hits += tps * 100 / (e.attack_cooltime() as i64).max(1);
        } }
        let expected_accum = (accum * window_sec).min(maxhp * 2);
        let expected_hits = hits * window_sec / 100;   // max_hits=0 → 무시
        let explosion = (damage + expected_hits * 0 + expected_accum * ratio / 100).min(maxhp * 2);
        ((explosion * hp_value) / hp.max(1)).min(160)
    };
    let got = unsafe { mv(55, eb.0.as_ptr(), &data as *const OperationData, team0 as *const PlayerState, param.0.as_ptr(),
                          champ as *const Entity, t as *const Entity, hp_value, dbg.0.as_mut_ptr()) };
    let dbg_touched = dbg.0.iter().any(|b| *b != 0);
    println!("case={}\tgot={}\tmodel={}\t{}\t(ti={} tp={} dbg_touched={})", case, got, exp,
             if got == exp { "MATCH" } else { "DIFF" }, ti, tp.is_some(), dbg_touched);
}
