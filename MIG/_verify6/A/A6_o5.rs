#![allow(unused, dead_code, non_snake_case)]
//! A6-O5 — `specs[3] game_ai::defensive_crisis` 실행 오라클 (5차 A5_o6 과 같은 수법, 재확인 + 확장).
//! argv: <slot=skill|skill2|ult|none> <level> <cooldown> <tps>
//! ★한 프로세스 = 한 케이스(check_kill_die_tick TLS 메모 회피, TEMPLATE ③).
//! 확장분:
//!   (N1) `tps` 를 인자로 받아 `die_imminent = die < tps*2`(knobs[0]) 와 `cool <= tps`(knobs[1]) 를
//!        **같은 프로세스 안에서 한 번만** 재되, 프로세스를 갈라 tps 축을 쓴다.
//!   (N2) `rnd`(&mut StdRng) · `debug`(&mut DebugFrameData) 바이트 diff — `open[0]` 잔여 물음.
//!   (N3) `check_kill_die_tick` 을 직접 호출해 `die` 원값을 찍는다(임계 비교를 눈으로).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn cc_effect() -> Effect {
    Effect { range: 100000, growth_range: 0, start_timing: 0,
             casting: CastingType::Targeting, target: CastingTarget::Enemy,
             ty: Arc::new(StunEffect { duration: 120 }) as Arc<dyn EffectType>,
             attack_type: AttackType::Skill }
}
fn nocc_effect() -> Effect {
    Effect { range: 100000, growth_range: 0, start_timing: 0,
             casting: CastingType::Targeting, target: CastingTarget::Enemy,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>,
             attack_type: AttackType::Skill }
}
fn bytes<T>(t: &T) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()).to_vec() }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let slot: String = a.get(1).cloned().unwrap_or("ult".into());
    let lvl: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let cool: usize = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(0);
    let tps: usize = a.get(4).map(|s| s.parse().unwrap()).unwrap_or(60);

    let mut setting = real_setting();
    setting.tick_per_second = tps;
    let ok = setting_ok(&setting);
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
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let (my_id, foe_id) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c.player_champion[0][0].unwrap().id, c.player_champion[1][0].unwrap().id)
    };
    let (mx, my) = { let e = game.world.entity.get(my_id).unwrap(); (e.x, e.y) };
    {
        let e = game.world.entity.get_mut(foe_id).unwrap();
        e.x = mx + 1000; e.y = my + 1000; e.level = lvl;
        e.skill_effect = Some(nocc_effect());
        e.skill2_effect = Some(nocc_effect());
        e.ult_effect = Some(nocc_effect());
        match slot.as_str() {
            "skill" => e.skill_effect = Some(cc_effect()),
            "skill2" => e.skill2_effect = Some(cc_effect()),
            "none" => {}
            _ => e.ult_effect = Some(cc_effect()),
        }
        if let EntityType::Champion(c) = &mut e.ty {
            c.skill_cooldown = 9999; c.skill2_cooldown = 9999; c.ult_cooldown = 9999;
            match slot.as_str() {
                "skill" => c.skill_cooldown = cool,
                "skill2" => c.skill2_cooldown = cool,
                "none" => {}
                _ => c.ult_cooldown = cool,
            }
        }
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let target: &Entity = cache.player_champion[0][0].unwrap();
    let foe: &Entity = cache.player_champion[1][0].unwrap();

    let dsq = Entity::distance_sq(foe, target);
    let mr = game_ai::max_range(foe, target);
    println!("SETUP\tslot={}\tlevel={}\tcool={}\ttps={}\tdist_sq={}\tmax_range={}\t(mr+30000)^2={}\tfilter_dist_ok={}\tsetting_ok={}",
             slot, lvl, cool, tps, dsq, mr, (mr + 30000).wrapping_mul(mr + 30000),
             dsq <= (mr + 30000).wrapping_mul(mr + 30000), ok);
    println!("CCTIME\tskill={:?}\tskill2={:?}\tult={:?}",
             foe.skill_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)),
             foe.skill2_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)),
             foe.ult_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)));

    // (N2) &mut 인자 바이트 diff
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);
    let rb = bytes(&rnd);
    let db = bytes(&dbg);
    let r = game_ai::defensive_crisis(1, &mut rnd, player, &data, target, &mut dbg);
    let ra = bytes(&rnd);
    let da = bytes(&dbg);
    let p = &r as *const game_ai::DefensiveCrisis as *const u8;
    unsafe {
        println!("RESULT\tdie_imminent(+0x0)={}\tcc_threat(+0x1)={}\trnd_changed={}\tdebug_changed={}",
                 *p, *p.add(1), rb != ra, db != da);
    }
    println!("MEMOWARN\t이 아래 check_kill_die_tick 직접호출은 **같은 프로세스 두 번째 측정**이라 TLS 메모 값일 수 있다");
    // (N3) die 원값 — TLS 메모라 위 호출과 같은 키면 재생이다(경고 그대로)
    {
        let mut rnd2 = rand::rngs::StdRng::seed_from_u64(9);
        let mut dbg2: DebugFrameData = Default::default();
        let empty: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        let mut foes: bumpalo::collections::Vec<&Entity> = bumpalo::collections::Vec::new_in(&pool);
        foes.push(foe);
        let tp = cache.player_by_champion_id(target.id).unwrap();
        let die = game_ai::check_kill_die_tick(1, &mut rnd2, &data, tp, target, foes, empty, &mut dbg2);
        println!("DIETICK\tdie={}\ttps*2={}\tdie<tps*2={}\t(명세 knobs[0])", die, tps * 2, die < tps * 2);
    }
}
