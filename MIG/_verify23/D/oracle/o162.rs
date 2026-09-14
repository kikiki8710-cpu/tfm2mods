#![allow(unused, dead_code, non_snake_case)]
//! 23차 배치D 오라클 — #162 v57_summon_command_score (define hidden, m10.ll:38354) — `#[link_name]` 직접 호출
//! 반환 = Option<i64> ScalarPair { i64 tag, i64 val }. 액션은 ChampionInfo::ult()/skill()/skill2() 로 실물 Box<dyn Action>.
//! near_visible_enemy: 적 챔프를 around 근처(≤200000)로 옮기고, tick 0 이면 blackboard last_seen(0)+120 >= tick 이라 true,
//!   set_tick(1000) 이면 false (g07.ll:157039~157043).  구울 = 1728B 제로버퍼 + ty 태그(+0x68)=7 을 others[team] 에 raw 주입.
//! 사용: o162.exe <case>  (케이스당 프로세스 1개)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value24v57_summon_command_score"]
    fn v57(data: *const OperationData, player: *const PlayerState, champ: *const Entity,
           action: *const Box<dyn Action>, t: *const Entity) -> Option<i64>;
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
unsafe fn p64(base: *mut u8, off: usize, v: i64) { std::ptr::write_volatile(base.add(off) as *mut i64, v); }
unsafe fn r64(base: *const u8, off: usize) -> i64 { std::ptr::read_volatile(base.add(off) as *const i64) }

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
    let mut game = mkgame(&setting, &ms, &map, &ctx);

    // 케이스 정의: (액션종류, t 가 아군?, 근처 적 배치?, tick, 구울 수, 비구울 추가 수)
    // kind: 0 IllusionistUlt · 1 NecroSkill · 2 NecroSkill2 · 3 NecroUlt · 4 기타(Swordman skill)
    let (kind, t_ally, near, tick, ghouls, junk, expect): (u8, bool, bool, usize, usize, usize, Option<i64>) = match case {
        0 => (0, false, false, 0, 0, 0, Some(0)),      // L795 t.team != champ.team
        1 => (0, true, false, 0, 0, 0, Some(5)),       // attack_effect None → dps 10 → atk 10, 근처 적 없음 → 5
        2 => (0, true, true, 0, 0, 0, Some(10)),       // 근처 적 + tick0(recent) → atk_power 10
        3 => (0, true, true, 1000, 0, 0, Some(5)),     // 근처 적이지만 last_seen 0+120 < 1000 → 5
        4 => (1, false, false, 0, 0, 0, Some(8)),      // 네크로 Q: champ 근처 적 없음 → 8
        5 => (1, false, true, 0, 0, 0, Some(25)),      // 네크로 Q: champ 근처 적 → 25
        6 => (2, false, false, 0, 0, 0, Some(-100)),   // 네크로 W: 구울 0 → -100
        7 => (2, false, false, 0, 2, 0, Some(36)),     // 구울 2, t 적 → 36
        8 => (2, false, false, 0, 4, 0, Some(60)),     // 구울 4 → 72 → 60 포화
        9 => (2, true, false, 0, 2, 0, Some(0)),       // 구울 2, t 아군 → 0
        10 => (2, false, false, 0, 0, 3, Some(-100)),  // 비구울 3 만 → 구울 0 → -100
        11 => (2, false, false, 0, 3, 2, Some(54)),    // 구울 3 + 비구울 2 → 54
        12 => (3, false, false, 0, 0, 0, Some(30)),    // 네크로 궁: 근처 적 없음 → 30
        13 => (3, false, true, 0, 0, 0, Some(90)),     // 근처 적 → 90
        14 => (4, false, false, 0, 0, 0, None),        // 기타 액션 → None
        15 => (3, false, true, 1000, 0, 0, Some(30)),  // 근처 적이지만 tick 1000 → 30
        16 => (2, true, false, 0, 4, 0, Some(0)),      // 구울 4 + 아군 → 0 (구울 수 무관)
        _ => (0, false, false, 0, 0, 0, None),
    };
    if tick > 0 { game.set_tick(tick); }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    let player: &PlayerState = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ: &Entity = cache.player_champion[0][2].expect("champ");
    let t: &Entity = if t_ally { cache.player_champion[0][0].expect("ally") } else { cache.player_champion[1][0].expect("enemy") };
    let around: &Entity = if kind == 0 { t } else { champ };
    unsafe {
        if near {
            // 적 챔프(팀1 pos1)를 around 옆 1000 으로
            let e = cache.player_champion[1][1].expect("enemy2");
            let ep = e as *const Entity as *mut u8;
            p64(ep, 0x660, r64(around as *const Entity as *const u8, 0x660) + 1000);
            p64(ep, 0x668, r64(around as *const Entity as *const u8, 0x668) + 1000);
        }
        // others[0] 에 구울/비구울 주입
        if ghouls + junk > 0 {
            let team = 0usize;
            let cp = &cache as *const AbstractGameWithCache as *mut u8;
            let ov = cp.add(0xf0 + team * 32);
            let optr = r64(ov, 0) as *const *const Entity;
            let olen = r64(ov, 24) as usize;
            let mut arr: Vec<*const Entity> = Vec::new();
            for i in 0..olen { arr.push(*optr.add(i)); }
            for i in 0..(ghouls + junk) {
                let b: &'static mut Buf<1728> = Box::leak(Box::new(Buf([0u8; 1728])));
                let bp = b.0.as_mut_ptr();
                p64(bp, 0x68, if i < ghouls { 7 } else { 2 });   // 7 Ghoul / 2 Tower(비구울)
                arr.push(bp as *const Entity);
            }
            let arr: &'static mut Vec<*const Entity> = Box::leak(Box::new(arr));
            p64(ov, 0, arr.as_ptr() as i64);
            p64(ov, 24, arr.len() as i64);
        }
    }
    let action: Box<dyn Action> = match kind {
        0 => IllusionistChampionInfo::default().ult(),
        1 => NecromancerChampionInfo::default().skill(),
        2 => NecromancerChampionInfo::default().skill2(),
        3 => NecromancerChampionInfo::default().ult(),
        _ => SwordmanChampionInfo::default().skill(),
    };
    let dist = unsafe {
        let e = cache.player_champion[1][1].expect("enemy2");
        let dx = (e.x as i64 - around.x as i64).abs(); let dy = (e.y as i64 - around.y as i64).abs();
        dx * dx + dy * dy
    };
    let got = unsafe { v57(&data as *const OperationData, player as *const PlayerState, champ as *const Entity,
                           &action as *const Box<dyn Action>, t as *const Entity) };
    println!("case={}\tkind={}\tgot={:?}\tmodel={:?}\t{}\t(tick={} t_ally={} near={} d2={} ghouls={} junk={} others_len={} atk_eff_none={})",
             case, kind, got, expect, if got == expect { "MATCH" } else { "DIFF" }, game.tick(), t_ally, near, dist, ghouls, junk,
             cache.others[0].len(), unsafe { std::ptr::read_volatile((t as *const Entity as *const u8).add(0x4c0) as *const i32) } == -1);
}
