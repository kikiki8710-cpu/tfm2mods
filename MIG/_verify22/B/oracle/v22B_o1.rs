#![allow(unused, dead_code, non_snake_case)]
//! v22B-O1 — 22차 배치 B 오라클: specs[121] safe_move_avoiding_enemy_well · [122] convert_to_move_action_target · [123] attack_structure_skill_action
//! argv: <case> [p1 p2 ...]   ★한 프로세스 = 한 케이스(TEMPLATE ③).
//!   sm_safe <ver>            목표=안전점(챔프+100000,0)             → 기대 Some(Move(x,y)) (투사체 0 → 조향 없음)
//!   sm_near <ver>            목표=챔프+1000                        → 기대 Some(Move(x,y)) (:61 isqrt<2000)
//!   sm_tdanger <ver>         목표=적 우물 위험점 D, 챔프 안전       → v1: None / v2: dist_sq(champ,esc(D))>4e6 ? Some(Move(esc(D))) : None
//!   sm_sdanger <ver>         챔프를 D 로 이동, 목표=D               → Some(Move(esc(cx,cy))) (버전 무관)
//!   sm_sdanger_tsafe <ver>   챔프를 D 로 이동, 목표=안전점          → Some(Move(target)) (:123 이 먼저 — 목표가 안전하면 챔프 위험은 안 봄)
//!   cv_pos <dist> <maxr>     Position 캐스팅, 목표=(cx+dist,cy)     → dist_sq>range² || maxr ? Pos(투사점) : Pos(x,y)
//!   cv_dir <dx> <dy>         Direction 캐스팅                        → Dir(dx*sign, dy*sign) (AttackEffect 기본 sign=+1)
//!   as_default               스워드맨 기본 챔프                      → 빈 Vec (skill_effect None)
//!   as_siege <near>          player0 = SiegeBreakerChampionInfo, near=1 이면 적 1차 탑타워 옆으로 이동 → 술어 출력 + 결과
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn show(o: &Option<Input>) -> String {
    match o {
        None => "None".to_string(),
        Some(Input::Move { x, y }) => format!("Move({},{})", x, y),
        Some(other) => format!("{:?}", other),
    }
}
fn show_t(t: &InputTarget) -> String { format!("{:?}", t) }

fn mk_effect(range: u64, growth: u64, casting: CastingType) -> Effect {
    Effect {
        range, growth_range: growth, start_timing: 0, casting,
        target: CastingTarget::Enemy,
        ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>,
        attack_type: AttackType::Skill,
    }
}

/// 격자 스캔으로 적 우물 위험점을 하나 찾는다(is_enemy_well_danger 가 pub).
fn find_danger(ver: usize, player: &PlayerState, setting: &GameSetting) -> Option<(u64, u64)> {
    let step = 16000u64;
    let mut y = 8000u64;
    while y < setting.height {
        let mut x = 8000u64;
        while x < setting.width {
            if game_ai::is_enemy_well_danger(ver, player, x, y) { return Some((x, y)); }
            x += step;
        }
        y += step;
    }
    None
}

fn mkgame_ci(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext, siege0: bool) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = if siege0 && t == 0 && p == 0 {
                Arc::new(SiegeBreakerChampionInfo::default())
            } else {
                Arc::new(SwordmanChampionInfo::default())
            };
            let mut st: AthleteStat = Default::default();
            st.judgement = 80; st.mental = 60;
            let name = if siege0 && t == 0 && p == 0 { "siege_breaker" } else { "swordman" };
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, name, ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: String = a.get(1).cloned().unwrap_or("sm_safe".into());
    let p1: i64 = a.get(2).map(|s| s.parse().unwrap()).unwrap_or(2);
    let p2: i64 = a.get(3).map(|s| s.parse().unwrap()).unwrap_or(0);

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
    let siege = case == "as_siege";
    let mut game = mkgame_ci(&setting, &ms, &map, &ctx, siege);
    let my_id = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][0].unwrap().id
    };
    println!("CASE\t{}\tp1={}\tp2={}\tsetting_ok={}\tDM_NO_DODGE={:?}", case, p1, p2, ok, std::env::var("DM_NO_DODGE").ok());

    // ── 사전 조정(챔프 이동 등) ──
    let ver = p1 as usize;
    let mut danger: Option<(u64, u64)> = None;
    if case.starts_with("sm_") {
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        danger = find_danger(ver, player, &setting);
        println!("DANGER\t{:?}", danger);
        if case == "sm_sdanger" || case == "sm_sdanger_tsafe" {
            let (dx, dy) = danger.expect("danger point");
            let e = game.world.entity.get_mut(my_id).unwrap();
            e.x = dx; e.y = dy;
        }
    }
    if case == "as_siege" && p1 > 0 {
        // 적(팀1) 1차 탑 타워 옆으로 이동
        let (tx, ty) = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let t = cache.top_tower[1].expect("enemy top tower");
            (t.x, t.y)
        };
        let e = game.world.entity.get_mut(my_id).unwrap();
        e.x = tx.saturating_sub(p1 as u64); e.y = ty;
        if let EntityType::Champion(c) = &mut e.ty { c.skill_cooldown = 0; }
    }

    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
    let champ: &Entity = cache.player_champion[0][0].unwrap();
    println!("CHAMP\tid={}\tlvl={}\tx={}\ty={}\tradius={}\tteam={}", champ.id, champ.level, champ.x, champ.y, champ.radius(), player.info.team);

    if case.starts_with("sm_") {
        let (tx, ty) = match case.as_str() {
            "sm_safe" => (champ.x + 100000, champ.y),
            "sm_sdanger_tsafe" => (100000u64, 900000u64),
            "sm_near" => (champ.x + 1000, champ.y),
            _ => danger.expect("danger"),
        };
        let t_danger = game_ai::is_enemy_well_danger(ver, player, tx, ty);
        let c_danger = game_ai::is_enemy_well_danger(ver, player, champ.x, champ.y);
        let c_recent = game_ai::is_recent_enemy_well_damage_danger(ver, player, champ);
        println!("PRED_IN\ttarget=({},{})\tt_danger={}\tc_danger={}\tc_recent={}", tx, ty, t_danger, c_danger, c_recent);
        // 명세 logic 재구현(외곽만 · 투사체 0 가정)
        let expect: Option<Input> = if t_danger {
            if c_danger || c_recent {
                let (ex, ey) = game_ai::enemy_well_escape_position(&setting, &map, player, champ.x, champ.y);
                Some(Input::Move { x: ex, y: ey })
            } else if ver > 1 {
                let (bx, by) = game_ai::enemy_well_escape_position(&setting, &map, player, tx, ty);
                let d2 = game_core::utils::distance_sq(champ.x, champ.y, bx, by);
                println!("PRED_V2\tesc=({},{})\td2={}", bx, by, d2);
                if d2 > 4_000_000 { Some(Input::Move { x: bx, y: by }) } else { None }
            } else { None }
        } else {
            Some(Input::Move { x: tx, y: ty })
        };
        let got = game_ai::safe_move_avoiding_enemy_well(ver, player, &data, champ, tx, ty);
        println!("RESULT\tgot={}\texpect={}\tMATCH={}", show(&got), show(&expect), if show(&got) == show(&expect) { "YES" } else { "no" });
    }

    if case == "cv_pos" || case == "cv_dir" {
        let range = 150_000u64; let growth = 0u64;
        let ef = mk_effect(range, growth, if case == "cv_pos" { CastingType::Position } else { CastingType::Direction });
        let (x, y) = if case == "cv_pos" { ((champ.x as i64 + p1) as u64, champ.y) } else { ((champ.x as i64 + p1) as u64, (champ.y as i64 + p2) as u64) };
        let maxr = case == "cv_pos" && p2 != 0;
        let r_eff = Effect::range(&ef, champ);
        let d2 = game_core::utils::distance_sq(x, y, champ.x, champ.y);
        let expect: InputTarget = if case == "cv_pos" {
            if maxr || d2 > r_eff * r_eff {
                let dx = (x as i64).wrapping_sub(champ.x as i64); let dy = (y as i64).wrapping_sub(champ.y as i64);
                let sz = game_core::utils::isqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))).max(1);
                let nx = (champ.x as i64).wrapping_add((r_eff as i64).wrapping_mul(dx) / sz);
                let ny = (champ.y as i64).wrapping_add(dy.wrapping_mul(r_eff as i64) / sz);
                let (ax, ay) = Game::adjust_position(&map, &setting, nx, ny);
                InputTarget::Pos { x: ax, y: ay }
            } else { InputTarget::Pos { x, y } }
        } else {
            let sign = EffectType::expected_move_input_dir_sign(&*ef.ty);
            println!("SIGN\t{}", sign);
            InputTarget::Dir { dir_x: (x as i64).wrapping_sub(champ.x as i64).wrapping_mul(sign), dir_y: (y as i64).wrapping_sub(champ.y as i64).wrapping_mul(sign) }
        };
        println!("PRED_IN\trange_eff={}\td2={}\trange2={}\tmaxr={}", r_eff, d2, r_eff * r_eff, maxr);
        let got = game_ai::convert_to_move_action_target(&setting, &map, champ, &ef, x, y, maxr);
        println!("RESULT\tgot={}\texpect={}\tMATCH={}", show_t(&got), show_t(&expect), if show_t(&got) == show_t(&expect) { "YES" } else { "no" });
    }

    if case.starts_with("as_") {
        let sk = champ.skill_effect.as_ref();
        let sk2 = champ.skill2_effect();
        let eds = sk.map(|e| EffectType::expected_damage_structure(&*e.ty, &ctx, champ).is_some());
        println!("SKILL\tskill_effect_some={}\teds_some={:?}\tskill2_some={}\tcan_skill={}\tcan_skill2={}\tmove_speed={}",
                 sk.is_some(), eds, sk2.is_some(), champ.can_skill(), champ.can_skill2(), champ.stat_cached.move_speed);
        let enemy = 1 - player.info.team;
        let mut n_pred = 0usize;
        let mut towers: Vec<&Entity> = Vec::new();
        for t in [cache.top_tower[enemy], cache.mid_tower[enemy], cache.bottom_tower[enemy], cache.top_tower2[enemy], cache.mid_tower2[enemy], cache.bottom_tower2[enemy]] { if let Some(t) = t { towers.push(t); } }
        for t in cache.twin_towers[enemy].iter() { towers.push(t); }
        if let Some(n) = cache.nexus[enemy] { towers.push(n); }
        for t in towers.iter() {
            if !t.can_target() { continue; }
            let d2 = game_core::utils::distance_sq(t.x, t.y, champ.x, champ.y);
            if let (Some(e), Some(true)) = (sk, eds) {
                let r = Effect::range(e, champ) + (champ.stat_cached.move_speed as u64) * 30 + Effect::range_adjust(e, champ, t) + champ.radius() as u64 + t.radius() as u64;
                let chk = CastingTarget::check(&e.target, champ, t);
                let tc = champ.skill.target_constraint(&game, champ, t);
                let ca = champ.skill.can_activate(&game, champ);
                let hit = champ.can_skill() && chk && d2 <= r * r;
                println!("TOWER\tid={}\td2={}\tr2={}\tcheck={}\tin_range={}\ttc={}\tca={}", t.id, d2, r * r, chk, d2 <= r * r, tc, ca);
                if hit && tc && ca { n_pred += 1; }
            }
        }
        let got = game_ai::attack_structure_skill_action(player, &data);
        // SmallActionSkill 필드는 private → derive(Debug) 로 관통(TEMPLATE ⓑ)
        let tags: Vec<String> = got.iter().map(|p| match p { game_ai::SmallActionPlay::Skill(s) => format!("Skill({:?})", s), game_ai::SmallActionPlay::Skill2(s) => format!("Skill2({:?})", s), _ => "other".into() }).collect();
        println!("RESULT\tlen={}\tpred_len={}\tMATCH={}\t{:?}", got.len(), n_pred, if got.len() == n_pred { "YES" } else { "no" }, tags);
    }
}
