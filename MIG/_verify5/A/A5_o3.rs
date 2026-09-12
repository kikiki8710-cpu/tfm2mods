#![allow(unused, dead_code, non_snake_case)]
//! A5-O3 — specs[0] `game_ai::ult` **실행 오라클** (v2).
//! argv: <case> <level> <eff_range> <casting 0..3> <version>
//!   case = visible | invisible
//! ★한 프로세스 = 한 케이스(TEMPLATE ③ TLS 메모 함정 회피).
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn mk_effect(range: u64, growth: u64, casting: CastingType) -> Effect {
    Effect {
        range,
        growth_range: growth,
        start_timing: 0,
        casting,
        target: CastingTarget::Enemy,
        ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>,
        attack_type: AttackType::Skill,
    }
}

fn show(o: &Option<Input>) -> String {
    match o {
        None => "None".to_string(),
        Some(Input::Move { x, y }) => format!("Move({},{})", x, y),
        Some(Input::Return) => "Return".to_string(),
        Some(Input::Attack { target }) => format!("Attack({:?})", target),
        Some(Input::Skill { target }) => format!("Skill({:?})", target),
        Some(Input::Skill2 { target }) => format!("Skill2({:?})", target),
        Some(Input::Ult { target }) => format!("Ult({:?})", target),
    }
}

/// 명세 §logic 329~336 을 **pub API 로만** 재구현. caster_r 은 `Effect::range` **밖**에서 더한다.
fn predict_xy(
    ef: &Effect, champ: &Entity, target: &Entity, margin: u64, add_caster_r: bool,
    map: &MapDef, setting: &GameSetting,
) -> (u64, u64, u64, i64, i64) {
    let dx = (champ.x as i64).wrapping_sub(target.x as i64);
    let dy = (champ.y as i64).wrapping_sub(target.y as i64);
    let sz = game_core::utils::isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
    let caster_r: u64 = if add_caster_r { champ.radius() as u64 } else { 0 };
    let total = Effect::range(ef, champ)
        .wrapping_add(caster_r)
        .wrapping_add(Effect::range_adjust(ef, champ, target))
        .wrapping_add(target.radius() as u64);
    let from_distance = total.saturating_sub(margin) as i64;
    if sz == 0 { return (u64::MAX, u64::MAX, total, dx, dy); }
    let x = (target.x as i64).wrapping_add(from_distance.wrapping_mul(dx) / sz);
    let y = (target.y as i64).wrapping_add(from_distance.wrapping_mul(dy) / sz);
    let (ax, ay) = Game::adjust_position(map, setting, x, y);
    (ax, ay, total, dx, dy)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: String = a.get(1).cloned().unwrap_or("visible".into());
    let lvl: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let rng: u64 = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(200000);
    let ct: usize = a.get(4).map(|s| s.parse().unwrap()).unwrap_or(0);
    let ver: usize = a.get(5).map(|s| s.parse().unwrap()).unwrap_or(2);
    let casting = [CastingType::Targeting, CastingType::Position, CastingType::Direction, CastingType::None][ct];

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
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (cache.player_champion[0][0].unwrap().id, cache.player_champion[1][0].unwrap().id)
    };
    {
        let e = game.world.entity.get_mut(my_id).unwrap();
        e.level = lvl;
        e.ult_effect = Some(mk_effect(rng, 0, casting));
        if let EntityType::Champion(c) = &mut e.ty { c.ult_cooldown = 0; }
    }
    if case == "same_pos" {
        // specs[0] open[0]: sz==0 → 334행 div-by-zero 패닉 경로가 실제로 있나
        let (mx, my) = { let e = game.world.entity.get(my_id).unwrap(); (e.x, e.y) };
        let e = game.world.entity.get_mut(foe_id).unwrap();
        e.x = mx; e.y = my;
    }
    {
        let e = game.world.entity.get_mut(foe_id).unwrap();
        let (lx, ly) = (e.x, e.y);
        e.visible_state[0] = if case == "invisible" {
            VisibleState::Invisible { last_x: lx, last_y: ly }
        } else {
            VisibleState::Visible
        };
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let champ: &Entity = cache.player_champion[0][0].unwrap();
    let target: &Entity = cache.player_champion[1][0].unwrap();
    let psd: PositioningScoreData = Default::default();

    println!("CASE\t{}\tlevel={}\teff_range={}\tcasting={:?}\tversion={}\tsetting_ok={}",
             case, lvl, rng, casting, ver, ok);
    println!("CHAMP\tid={}\tlvl={}\tx={}\ty={}\tradius={}\tcan_ult={}\tult_eff_some={}",
             champ.id, champ.level, champ.x, champ.y, champ.radius(),
             champ.can_ult(), champ.ult_effect().is_some());
    println!("TARGET\tid={}\tx={}\ty={}\tradius={}\tvisible_from_champ={}",
             target.id, target.x, target.y, target.radius(), target.is_visible_from(champ));

    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let got = game_ai::ult(ver, &mut rnd, player, &data, &psd, target);
    println!("ULT_RESULT\t{}", show(&got));

    if let Some(ef) = champ.ult_effect().as_ref() {
        println!("EFFECT\trange_fn={}\trange_adjust={}\tis_in_range={}\tcheck={}\ton_caster={}\tlinear_move_speed={:?}",
                 Effect::range(ef, champ), Effect::range_adjust(ef, champ, target),
                 Effect::is_in_range(ef, champ, target),
                 CastingTarget::check(&ef.target, champ, target),
                 EffectType::on_caster(&*ef.ty), EffectType::linear_move_speed(&*ef.ty));
        // (B) margin 감도 + caster_r 포함 여부
        for (tag, add_r) in [("caster_r_ON", true), ("caster_r_OFF", false)] {
            for m in [149_999u64, 150_000, 150_001] {
                let (px, py, total, dx, dy) = predict_xy(ef, champ, target, m, add_r, &map, &setting);
                let exp = game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, px, py);
                println!("PREDICT\t{}\tmargin={}\ttotal={}\tpos=({},{})\tsafe_move={}\tMATCH_ULT={}",
                         tag, m, total, px, py, show(&exp),
                         if show(&exp) == show(&got) { "YES" } else { "no" });
            }
        }
        // (C) 비가시 경로: L340 = safe_move(target.x, target.y)
        let exp340 = game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, target.x, target.y);
        println!("PREDICT_L340\tsafe_move(target.x,target.y)={}\tMATCH_ULT={}",
                 show(&exp340), if show(&exp340) == show(&got) { "YES" } else { "no" });
    }
}
