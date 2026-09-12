#![allow(unused, dead_code, non_snake_case)]
//! A6-O2 — `specs[0] game_ai::ult` 실행 오라클 (6차).
//! argv: <case> <level> <eff_range> <casting 0..3> <version> <radius_mult>
//!   case = visible | invisible | same_pos
//! ★한 프로세스 = 한 케이스(TEMPLATE ③).
//! 5차(A5_o3) 대비 **확장분**:
//!   (N1) `radius_mult != 0` — 5차는 mult=0 이라 `100` 기준값과 `as usize`(sext)가 **비판별**이었다.
//!        mult=+50 / -50 을 넣어 `radius*(100+mult)/100` 과 **sext vs zext** 를 가른다.
//!   (N2) `same_pos` × casting 4값 — `open[0]`(sz==0 div-by-zero) 도달성 판정.
//!   (N3) CastingType / VisibleState 태그값 실측 인쇄.
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

fn predict_xy(ef: &Effect, champ: &Entity, target: &Entity, margin: u64, add_caster_r: bool,
              map: &MapDef, setting: &GameSetting) -> (u64, u64, u64) {
    let dx = (champ.x as i64).wrapping_sub(target.x as i64);
    let dy = (champ.y as i64).wrapping_sub(target.y as i64);
    let sz = game_core::utils::isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
    let caster_r: u64 = if add_caster_r { champ.radius() as u64 } else { 0 };
    let total = Effect::range(ef, champ).wrapping_add(caster_r)
        .wrapping_add(Effect::range_adjust(ef, champ, target))
        .wrapping_add(target.radius() as u64);
    let from_distance = total.saturating_sub(margin) as i64;
    if sz == 0 { return (u64::MAX, u64::MAX, total); }
    let x = (target.x as i64).wrapping_add(from_distance.wrapping_mul(dx) / sz);
    let y = (target.y as i64).wrapping_add(from_distance.wrapping_mul(dy) / sz);
    let (ax, ay) = Game::adjust_position(map, setting, x, y);
    (ax, ay, total)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: String = a.get(1).cloned().unwrap_or("visible".into());
    let lvl: usize = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let rng: u64 = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(200000);
    let ct: usize = a.get(4).map(|s| s.parse().unwrap()).unwrap_or(0);
    let ver: usize = a.get(5).map(|s| s.parse().unwrap()).unwrap_or(2);
    let rmult: i32 = a.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
    let casting = [CastingType::Targeting, CastingType::Position,
                   CastingType::Direction, CastingType::None][ct];

    println!("TAG\tCastingType\tTargeting={} Position={} Direction={} None={}",
             CastingType::Targeting as i32, CastingType::Position as i32,
             CastingType::Direction as i32, CastingType::None as i32);
    {
        let v = VisibleState::Visible;
        let t = unsafe { std::ptr::read_unaligned(&v as *const _ as *const i64) };
        let u = VisibleState::Unknown;
        let tu = unsafe { std::ptr::read_unaligned(&u as *const _ as *const i64) };
        println!("TAG\tVisibleState\tVisible={}\tUnknown={}\t(명세 consts[6]=0=Visible)", t, tu);
    }

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
        e.stat_buff_cached.radius_mult = rmult;      // ★N1
        if let EntityType::Champion(c) = &mut e.ty { c.ult_cooldown = 0; }
    }
    if case == "same_pos" {
        let (mx, my) = { let e = game.world.entity.get(my_id).unwrap(); (e.x, e.y) };
        let e = game.world.entity.get_mut(foe_id).unwrap();
        e.x = mx; e.y = my;
    }
    {
        let e = game.world.entity.get_mut(foe_id).unwrap();
        let (lx, ly) = (e.x, e.y);
        e.visible_state[0] = if case == "invisible" {
            VisibleState::Invisible { last_x: lx, last_y: ly }
        } else { VisibleState::Visible };
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let champ: &Entity = cache.player_champion[0][0].unwrap();
    let target: &Entity = cache.player_champion[1][0].unwrap();
    let psd: PositioningScoreData = Default::default();

    println!("CASE\t{}\tlevel={}\teff_range={}\tcasting={:?}\tversion={}\tradius_mult={}\tsetting_ok={}",
             case, lvl, rng, casting, ver, rmult, ok);

    // ── N1: radius() 의 100 기준 + sext 판별 ───────────────────────────
    {
        let raw = champ.radius_field_raw();
        let got = champ.radius();
        let mult_sext = rmult as i64 as usize;              // 명세: `as usize` = 부호확장
        let mult_zext = rmult as u32 as usize;              // 대립가설: 영확장
        let p_sext = if rmult == 0 { raw } else { raw.wrapping_mul(100usize.wrapping_add(mult_sext)) / 100 };
        let p_zext = if rmult == 0 { raw } else { raw.wrapping_mul(100usize.wrapping_add(mult_zext)) / 100 };
        // 대립가설 2: 기준값이 100 이 아니라 1000 이라면
        let p_1000 = if rmult == 0 { raw } else { raw.wrapping_mul(1000usize.wrapping_add(mult_sext)) / 1000 };
        println!("RADIUS\tmult={}\traw={}\tradius()={}\tpred_sext100={}\tpred_zext100={}\tpred_sext1000={}\t{}",
                 rmult, raw, got, p_sext, p_zext, p_1000,
                 if got == p_sext && (p_sext != p_zext || p_sext != p_1000) { "MATCH(판별)" }
                 else if got == p_sext { "MATCH" } else { "**MISMATCH**" });
    }

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
        for (tag, add_r) in [("caster_r_ON", true), ("caster_r_OFF", false)] {
            for m in [149_999u64, 150_000, 150_001] {
                let (px, py, total) = predict_xy(ef, champ, target, m, add_r, &map, &setting);
                if px == u64::MAX { println!("PREDICT\t{}\tmargin={}\ttotal={}\tsz==0 → 예측식 자체가 div-by-zero", tag, m, total); continue; }
                let exp = game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, px, py);
                println!("PREDICT\t{}\tmargin={}\ttotal={}\tpos=({},{})\tsafe_move={}\tMATCH_ULT={}",
                         tag, m, total, px, py, show(&exp),
                         if show(&exp) == show(&got) { "YES" } else { "no" });
            }
        }
        let exp340 = game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, target.x, target.y);
        println!("PREDICT_L340\tsafe_move(target.x,target.y)={}\tMATCH_ULT={}",
                 show(&exp340), if show(&exp340) == show(&got) { "YES" } else { "no" });
    }
}

// `Entity.radius` 필드 직독(= `radius()` 이전 원본값). 필드가 pub 이라 그대로 읽는다.
trait RawRadius { fn radius_field_raw(&self) -> usize; }
impl RawRadius for Entity { fn radius_field_raw(&self) -> usize { self.radius } }
