#![allow(unused, dead_code, non_snake_case)]
//! 22차 C · 124 v55_seal_value / 126 v55_banish_penalty 오라클 — hidden 함수를 `#[link_name]` 로 직접 진입.
//! 커스텀 EffectType(apply 만 필수) 로 expected_seal / has_banish_deep / expected_cc_time_deep 을 원하는 값으로 준다.
//! 한 프로세스 = 한 케이스(argv[1]).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_seal_value"]
    fn seal_value(version: usize, effect: &Effect, data: &OperationData, champ: &Entity, t: &Entity, hp_value: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value18v55_banish_penalty"]
    fn banish_penalty(version: usize, effect: &Effect, data: &OperationData, player: &PlayerState, champ: &Entity, t: &Entity, hp_value: i64) -> i64;
}

#[derive(Debug)]
struct MyFx { seal: Option<SealProfile>, banish: bool, cc: Option<usize> }
impl EffectType for MyFx {
    fn apply(&self, _r: &mut rand::rngs::StdRng, _g: &mut dyn AbstractGame, _c: &GameContext, _u: usize, _t: InputTarget, _a: AttackType, _o: Option<EffectOptionalInfo>, _f: &mut Option<&mut GameFrameData>) {}
    fn expected_seal(&self, _c: &GameContext, _e: &dyn AbstractEntity) -> Option<SealProfile> { self.seal }
    fn has_banish_deep(&self) -> bool { self.banish }
    fn expected_cc_time_deep(&self) -> Option<usize> { self.cc }
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
fn mkeffect(fx: MyFx) -> Effect {
    Effect { ty: Arc::new(fx), range: 0, growth_range: 0, start_timing: 0, target: CastingTarget::Ally, attack_type: AttackType::Skill, casting: CastingType::Targeting }
}

fn main() {
    let case: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    println!("setting_ok\ttps={}\twidth={}", setting.tick_per_second, setting.width);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let game = mkgame(&setting, &ms, &map, &ctx);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    // ★캐시 DPS 주입: 팀1 pos0(=t, Top) 의 캐시 = t 가 상대 5포지션에 넣는 DPS. 팀0 pos1..4 = 아군 캐시(126 용).
    for team in 0..2 { for pos in 0..5 { for i in 0..5 {
        let c = &mut cache.player_champion_cache[team][pos];
        c.attack_per_sec[i] = 10 * (i as usize + 1) + 100 * pos + 1000 * team;
        c.skill_per_sec[i]  = 100 * (i as usize + 1);
        c.skill2_per_sec[i] = 30 * (i as usize + 1);
        c.ult_per_sec[i]    = 70 * (i as usize + 1);
    } } }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);

    let champ = cache.player_champion[0][0].unwrap();      // team0 Top
    let t = cache.player_champion[1][0].unwrap();          // team1 Top → tp.info.position = Top(0) → ti = 0
    let player = game.get_player_by_position(0, Position::Top).unwrap();
    let tp = t as *const Entity as *mut Entity;
    let cp = champ as *const Entity as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*tp).stat_cached.hp, 100000usize); std::ptr::write_volatile(&mut (*tp).hp, 50000usize); }
    println!("t\tid={} hp={} max_hp={} x={} y={}", t.id, unsafe { std::ptr::read_volatile(&(*tp).hp) }, unsafe { std::ptr::read_volatile(&(*tp).stat_cached.hp) }, t.x, t.y);

    // 124 케이스 정의: (seal, hp_value, t 를 타워로 바꿀지)
    let (seal, hp_value, t_is_tower, banish, cc) = match case {
        0 => (None, 1000, false, false, None),
        11 => (Some(SealProfile{ticks:179, blocks_attack:true, blocks_skill:true}), 1000, false, false, None),  // sec=2
        12 => (Some(SealProfile{ticks:180, blocks_attack:true, blocks_skill:true}), 1000, false, false, None),  // sec=3
        1 => (Some(SealProfile{ticks:120, blocks_attack:true, blocks_skill:false}), 1000, false, false, None),
        2 => (Some(SealProfile{ticks:120, blocks_attack:false, blocks_skill:true}), 1000, false, false, None),
        3 => (Some(SealProfile{ticks:120, blocks_attack:true, blocks_skill:true}), 1000, false, false, None),
        4 => (Some(SealProfile{ticks:59,  blocks_attack:true, blocks_skill:true}), 1000, false, false, None),   // sec=max(0,1)=1
        5 => (Some(SealProfile{ticks:119, blocks_attack:true, blocks_skill:true}), 1000, false, false, None),   // sec=1
        6 => (Some(SealProfile{ticks:120, blocks_attack:true, blocks_skill:true}), 100000, false, false, None), // cap 80
        7 => (Some(SealProfile{ticks:120, blocks_attack:true, blocks_skill:true}), -1000, false, false, None),  // 음수
        8 => (Some(SealProfile{ticks:120, blocks_attack:true, blocks_skill:true}), 10, true, false, None),    // t = 타워 → 0
        9 => (Some(SealProfile{ticks:6000, blocks_attack:true, blocks_skill:true}), 10, false, false, None),  // erased 캡 = max_hp
        10 => (Some(SealProfile{ticks:120, blocks_attack:false, blocks_skill:false}), 1000, false, false, None),// 둘 다 false → stopped 0
        // 126
        20 => (None, 1000, false, false, Some(120)),         // has_banish_deep false → 0
        21 => (None, 1000, false, true, None),               // cc None → 0
        22 => (None, 1000, false, true, Some(120)),          // t 가 멀리(스폰) → 아군 0 → 0
        23 | 24 | 25 | 26 | 27 | 28 => (None, 1000, false, true, Some(120)),
        _ => (None, 1000, false, false, None),
    };
    let tower_id = game.world.tower_ids[0];
    let tower = game.get_entity_by_id(tower_id).unwrap();
    let t_ref: &Entity = if t_is_tower { tower } else { t };
    unsafe {
        match case {
            23 => { (*tp).x = (*cp).x; (*tp).y = (*cp).y; }   // t 를 champ 자리로 → 아군 4명(자신 제외) 전부 150000 이내
            24 => { (*tp).x = (*cp).x; (*tp).y = (*cp).y; (*tp).hp = 1; }                   // hp 분모 max(1,1)
            25 => { (*tp).x = (*cp).x; (*tp).y = (*cp).y; (*tp).hp = 0; }                   // hp 0 → max(0,1)=1
            26 => { // 아군 pos1 을 t 에서 정확히 150000, pos2 를 150001 로
                let a1 = cache.player_champion[0][1].unwrap() as *const Entity as *mut Entity;
                let a2 = cache.player_champion[0][2].unwrap() as *const Entity as *mut Entity;
                let a3 = cache.player_champion[0][3].unwrap() as *const Entity as *mut Entity;
                let a4 = cache.player_champion[0][4].unwrap() as *const Entity as *mut Entity;
                (*tp).x = 500000; (*tp).y = 500000;
                (*a1).x = 650000; (*a1).y = 500000;     // dist2 = 150000² → 포함
                (*a2).x = 650001; (*a2).y = 500000;     // 150001² → 제외
                (*a3).x = 0; (*a3).y = 0;               // 제외
                (*a4).x = 0; (*a4).y = 0;               // 제외
            }
            27 => { (*tp).x = (*cp).x; (*tp).y = (*cp).y; }   // cc 59 → sec 1
            28 => { (*tp).x = (*cp).x; (*tp).y = (*cp).y; }   // hp_value 큰 값 → cap 80
            _ => {}
        }
    }
    let cc2 = match case { 27 => Some(59usize), _ => cc };
    let hp_value2 = match case { 28 => 100000i64, _ => hp_value };
    let eff = mkeffect(MyFx{ seal, banish, cc: cc2 });
    // ★game_ai 크레이트를 LTO 단위에 끌어들이기 위한 pub 참조(없으면 --extern 이 링크 안 돼 LNK2019)
    println!("effect_cc_time	{:?}", game_ai::effect_cc_time(55, &eff));

    if case < 20 {
        let r = unsafe { seal_value(55, &eff, &data, champ, t_ref, hp_value2) };
        // 독립 재구현(명세 logic)
        let mine = match seal {
            None => 0i64,
            Some(sp) => {
                if t_is_tower { 0 } else {
                    let c = &cache.player_champion_cache[1][0];
                    let sec = (sp.ticks / 60).max(1);
                    let mut stopped = 0usize;
                    if sp.blocks_attack { stopped += c.attack_per_sec.iter().sum::<usize>() / 5; }
                    if sp.blocks_skill { stopped += (c.skill_per_sec.iter().sum::<usize>() + c.skill2_per_sec.iter().sum::<usize>() + c.ult_per_sec.iter().sum::<usize>()) / 5; }
                    let erased = (stopped * sec).min(t.stat_cached.hp as usize);
                    ((erased as i64 * hp_value2) / (t.hp as i64).max(1) / 2).min(80)
                }
            }
        };
        println!("RESULT124\tcase={}\tgame={}\tmine={}\tmatch={}", case, r, mine, r == mine);
    } else {
        let r = unsafe { banish_penalty(55, &eff, &data, player, champ, t, hp_value2) };
        let mine = if !banish { 0 } else if cc2.is_none() { 0 } else {
            let sec = (cc2.unwrap() / 60).max(1);
            let ti = 0usize;
            let mut dps = 0usize;
            for pos in 0..5 {
                let a = cache.player_champion[0][pos].unwrap();
                if a.id == champ.id { continue; }
                let d2 = a.x.abs_diff(t.x).pow(2) + a.y.abs_diff(t.y).pow(2);
                if d2 > 22500000000 { continue; }
                let c = &cache.player_champion_cache[0][pos];
                dps += c.attack_per_sec[ti] + c.skill_per_sec[ti] + c.skill2_per_sec[ti] + c.ult_per_sec[ti];
            }
            let lost = (dps * sec).min(t.stat_cached.hp as usize);
            ((lost as i64 * hp_value2) / (t.hp as i64).max(1) / 2).min(80)
        };
        println!("RESULT126\tcase={}\tgame={}\tmine={}\tmatch={}", case, r, mine, r == mine);
    }
}
