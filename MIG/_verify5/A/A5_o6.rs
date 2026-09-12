#![allow(unused, dead_code, non_snake_case)]
//! A5-O6 — specs[3] `game_ai::defensive_crisis` 의 `cc_threat` 축 실행 오라클.
//!  · consts[2]=2 (level>2 → skill2 슬롯 개방)
//!  · consts[3]=4 (level>4 → ult 슬롯 개방)
//!  · knobs[1]   (cool <= tps 창) — tps=60 에서 60 통과 / 61 탈락
//!  · consts[13] (EntityType 태그 13=Champion 이라야 쿨다운 3개를 실값으로 읽음)
//! argv: <slot=skill|skill2|ult> <level> <cooldown> <ty=champ|other>
//! ★한 프로세스 = 한 케이스(check_kill_die_tick TLS 메모 회피).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn cc_effect() -> Effect {
    Effect {
        range: 100000,
        growth_range: 0,
        start_timing: 0,
        casting: CastingType::Targeting,
        target: CastingTarget::Enemy,
        ty: Arc::new(StunEffect { duration: 120 }) as Arc<dyn EffectType>,
        attack_type: AttackType::Skill,
    }
}
fn nocc_effect() -> Effect {
    Effect {
        range: 100000,
        growth_range: 0,
        start_timing: 0,
        casting: CastingType::Targeting,
        target: CastingTarget::Enemy,
        ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>,
        attack_type: AttackType::Skill,
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let slot: String = a.get(1).cloned().unwrap_or("ult".into());
    let lvl: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let cool: usize = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(0);

    let setting = real_setting();
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
    let (mx, my) = {
        let e = game.world.entity.get(my_id).unwrap();
        (e.x, e.y)
    };
    // 적을 target 바로 옆으로 옮기고 슬롯/레벨/쿨을 세팅
    {
        let e = game.world.entity.get_mut(foe_id).unwrap();
        e.x = mx + 1000;
        e.y = my + 1000;
        e.level = lvl;
        // 세 슬롯 모두 비CC 로 깔고, 지정 슬롯만 CC 로
        e.skill_effect = Some(nocc_effect());
        e.skill2_effect = Some(nocc_effect());
        e.ult_effect = Some(nocc_effect());
        match slot.as_str() {
            "skill" => e.skill_effect = Some(cc_effect()),
            "skill2" => e.skill2_effect = Some(cc_effect()),
            _ => e.ult_effect = Some(cc_effect()),
        }
        if let EntityType::Champion(c) = &mut e.ty {
            // 지정 슬롯만 주어진 쿨, 나머지는 창 밖(9999)으로 밀어 교차오염 제거
            c.skill_cooldown = 9999;
            c.skill2_cooldown = 9999;
            c.ult_cooldown = 9999;
            match slot.as_str() {
                "skill" => c.skill_cooldown = cool,
                "skill2" => c.skill2_cooldown = cool,
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
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);

    let dsq = Entity::distance_sq(foe, target);
    let mr = game_ai::max_range(foe, target);
    println!("SETUP\tslot={}\tlevel={}\tcool={}\ttps={}\tdist_sq={}\tmax_range={}\t(mr+30000)^2={}\tfilter_dist_ok={}",
             slot, lvl, cool, setting.tick_per_second, dsq, mr,
             (mr + 30000).wrapping_mul(mr + 30000), dsq <= (mr + 30000).wrapping_mul(mr + 30000));
    println!("CCTIME\tskill={:?}\tskill2={:?}\tult={:?}",
             foe.skill_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)),
             foe.skill2_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)),
             foe.ult_effect.as_ref().map(|e| game_ai::effect_cc_time(1, e)));
    let r = game_ai::defensive_crisis(1, &mut rnd, player, &data, target, &mut dbg);
    let p = &r as *const game_ai::DefensiveCrisis as *const u8;
    unsafe {
        println!("RESULT\tdie_imminent(+0x0)={}\tcc_threat(+0x1)={}", *p, *p.add(1));
    }
}
