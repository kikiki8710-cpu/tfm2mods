#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치 F 오라클 — r18 지도 밖 판단 로직 4함수(262 target_score · 265 v54_aoe_ally_heal_value · 266 can_near_enemies_range · 267 expected_dps).
//! 264 push_candidate 는 `define internal fastcc`(심볼 없음) → 직접 진입 불가(IR 로만). 263 은 from_iter_in 인스턴스(Map<IntoIter,closure_env> 80B by-value) → 이번 라운드 미시도.
//! 한 프로세스 = 한 케이스. 사용: o27F.exe <family> <args...>
//!   dps  a s s2 lv        → 267: champ.attack/skill/skill2 데미지·레벨 주입 → expected_dps(ctx, champ, enemy) vs Σ dmg×1000/cd
//!   ts   hp maxhp dx dmg  → 262: target(적 챔프) hp/maxhp/x=champ.x+dx · atk=mkeff(dmg) · wave None → target_score vs 명세식(+rnd clone)
//!   cnr  x y d tick pat   → 266: TeamPlan::default + vision 세팅(pat: 0=전부 lvt 0 · 1=슬롯0,1 lvt=tick-100 나머지 0 · 2=전부 tick-100) · set_tick(tick)
//!   v54  amount range ex  → 265: HealEffect(AllyAll{range}) · param=calculate_score_parameter · anchor=champ · exclude_id=ex(0=없음,1=자기 id) · action=champ.skill2()
use game_core::*;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use rand::Rng;
use std::sync::Arc;

extern "Rust" {
    #[link_name = "_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition12target_score"]
    fn target_score(version: usize, data: &OperationData, player: &PlayerState, champ: &Entity, atk: &Effect, target: &Entity,
                    wave: Option<&game_ai::MinionWaveSnapshot>, rnd: &mut rand::rngs::StdRng) -> Option<i64>;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23v54_aoe_ally_heal_value"]
    fn v54(version: usize, effect: &Effect, data: &OperationData, param: &game_ai::ScoreParameter, champ: &Entity, anchor: &Entity,
           exclude_id: usize, action: &Box<dyn Action>) -> i64;
}

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000; s.height = 960000;
    s.respawn_tick = 300; s.respawn_growth = 30; s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180; s.respawn_max = 2400;
    s.visible_distance = 130000; s.tick_per_second = 60; s.champion_radius = 10000;
    s.nexus_heal = 10; s.nexus_heal_2v2 = 10; s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24; s.nexus_heal_decay = 100;
    s.kill_exp = 30; s.kill_exp_growth = 30; s.assist_exp_ratio = 40;
    s.kill_gold = 300; s.assist_gold = 100; s.start_gold = 500; s.gold_per_second = 7;
    s.return_tick = 120; s.epic_minion_buff_duration = 5400; s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400; s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200; s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200; s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800; s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600; s.well_damage_tick = 30; s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700; s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20; s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15; s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}
pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}", ok, s.width, s.height, s.tick_per_second, s.champion_radius);
    ok
}
pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let mut st: AthleteStat = Default::default();
        st.judgement = 80; st.mental = 60;
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    } }
    game.start_game(&mut rnd, ctx);
    game
}
fn mkeff(damage: usize, atk_ty: AttackType, start_timing: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 100000, growth_range: 0, start_timing,
        target: CastingTarget::Enemy, attack_type: atk_ty, casting: CastingType::Targeting }
}
fn mkheal(amount: usize, range: u64) -> Effect {
    let he = HealEffect { ty: HealEffectType::AllyAll { range }, amount, attack_ratio: 0, ap_ratio: 0,
        single_amount: amount, single_attack_ratio: 0, single_ap_ratio: 0 };   // ★expected_heal(heal.rs:187~189) 은 single_* 필드만 읽는다(g04.ll:236299) — amount 는 apply 전용
    Effect { ty: Arc::new(he), range: 100000, growth_range: 0, start_timing: 0,
        target: CastingTarget::Ally, attack_type: AttackType::Skill, casting: CastingType::Targeting }
}
unsafe fn rd<T: Copy>(b: *const u8, off: usize) -> T { std::ptr::read_unaligned(b.add(off) as *const T) }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fam = args.get(1).cloned().unwrap_or("dps".into());
    let n = |k: usize, d: i64| -> i64 { args.get(k).and_then(|v| v.parse::<i64>().ok()).unwrap_or(d) };
    if args.len() > 99 { let _ = game_ai::expected_dps as *const (); }   // game_ai rlib 링크 보장(link_name 심볼용)

    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    if fam == "cnr" { let t = n(5, 0) as usize; if t > 0 { game.set_tick(t); } }
    if fam == "v54" { let t = n(6, 3000) as usize; if t > 0 { game.set_tick(t); } }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    if fam == "v54" {   // near_allies 는 아군 블랙보드 small_action 이 있어야 채워진다(26차 A o204 C2/C4 실측 aact=0)
        for p in 1..5usize { bb[0].small_actions[p] = Some(SmallAction::RunAway); }
        let edist = n(9, 0) as u64;   // 9번째 인자: 적 5명을 champ.x+edist 에 세우고 공격 이펙트·가시성 부여(champion_hp_value 판별력용)
        if edist > 0 {
            let c0 = cache.player_champion[0][0].unwrap();
            let myid = c0.id;
            for p in 0..5usize {
                if let Some(e) = cache.player_champion[1][p] {
                    let epp = e as *const Entity as *mut Entity;
                    unsafe {
                        std::ptr::write_volatile(&mut (*epp).x, c0.x + edist + 5000 * p as u64);
                        std::ptr::write_volatile(&mut (*epp).y, c0.y);
                        std::ptr::write(&mut (*epp).attack_effect, Some(mkeff(100 + 50 * p, AttackType::BaseAttack, 10)));
                        std::ptr::write_volatile(&mut (*epp).hp, 900); std::ptr::write_volatile(&mut (*epp).stat_cached.hp, 1000);
                        std::ptr::write(&mut (*epp).visible_state[0], VisibleState::Visible);
                    }
                    bb[1].last_visible[p] = cache.game.tick().saturating_sub(10);
                    bb[1].small_actions[p] = Some(SmallAction::Attack { target_id: myid });
                }
            }
        }
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let team = 0usize; let pos = Position::Top;
    let player = game.get_player_by_position(team, pos).unwrap();
    let champ0 = cache.player_champion[team][pos as usize].unwrap();
    let enemy0 = cache.player_champion[1][0].unwrap();
    let cp = champ0 as *const Entity as *mut Entity;
    let ep = enemy0 as *const Entity as *mut Entity;
    println!("world\ttowers={}\ttick={}\tseed={}\tchamp(id={},hp={},lv={},x={},y={},ms={})\tenemy(id={},hp={},x={},y={})",
        game.world.tower_ids.len(), cache.game.tick(), cache.game.seed(), champ0.id, champ0.hp, champ0.level, champ0.x, champ0.y, champ0.stat_cached.move_speed,
        enemy0.id, enemy0.hp, enemy0.x, enemy0.y);

    if fam == "dps" {
        let (a, s, s2, lv) = (n(2, 0) as usize, n(3, 0) as usize, n(4, 0) as usize, n(5, 1) as usize);
        unsafe {
            std::ptr::write(&mut (*cp).attack_effect, if a > 0 { Some(mkeff(a, AttackType::BaseAttack, 10)) } else { None });
            std::ptr::write(&mut (*cp).skill_effect, if s > 0 { Some(mkeff(s, AttackType::Skill, 10)) } else { None });
            std::ptr::write(&mut (*cp).skill2_effect, if s2 > 0 { Some(mkeff(s2, AttackType::Skill, 10)) } else { None });
            std::ptr::write_volatile(&mut (*cp).level, lv);
        }
        let champ: &Entity = unsafe { &*cp }; let enemy: &Entity = unsafe { &*ep };
        let dmg = |eff: &Option<Effect>| -> usize { eff.as_ref().map(|e| e.expected_damage_target(&ctx, champ as &dyn AbstractEntity, enemy)).unwrap_or(0) };
        let (da, ds, ds2) = (dmg(&champ.attack_effect), dmg(&champ.skill_effect), if champ.level > 2 { dmg(&champ.skill2_effect) } else { 0 });
        let (ca, cs, cs2) = (champ.attack_cooltime(), champ.skill_cooltime(), champ.skill2_cooltime());
        let mine = da * 1000 / ca.max(1) + ds * 1000 / cs.max(1) + ds2 * 1000 / cs2.max(1);
        let game_v = game_ai::expected_dps(&ctx, champ, enemy);
        println!("DPS\ta={} s={} s2={} lv={}\tdmg=({},{},{})\tcd=({},{},{})\tgame={}\tmine={}\t{}",
            a, s, s2, lv, da, ds, ds2, ca, cs, cs2, game_v, mine, if game_v == mine { "MATCH" } else { "DIFF" });
        return;
    }
    if fam == "ts" {
        let (hp, maxhp, dx, dmg) = (n(2, 1000) as usize, n(3, 1000) as usize, n(4, 0) as i64, n(5, 0) as usize);
        let atk = mkeff(dmg, AttackType::BaseAttack, 10);
        unsafe {
            std::ptr::write_volatile(&mut (*ep).hp, hp);
            std::ptr::write_volatile(&mut (*ep).stat_cached.hp, maxhp);
            let nx = ((*cp).x as i64 + dx).max(0) as u64;
            std::ptr::write_volatile(&mut (*ep).x, nx);
            std::ptr::write_volatile(&mut (*ep).y, (*cp).y);
        }
        let champ: &Entity = unsafe { &*cp }; let target: &Entity = unsafe { &*ep };
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut rnd2 = rnd.clone();
        // ── 명세식 재현 ──
        let attack_damage = atk.expected_damage_target(&ctx, champ as &dyn AbstractEntity, target) as i64;
        let hp_ratio = target.hp * 100 / target.stat_cached.hp.max(1);
        let range = atk.range + champ.stat_buff_cached.range as u64 + (champ.level - 1) as u64 * atk.growth_range;
        let attack_range = range + atk.range_adjust(champ, target) + champ.radius() as u64 + target.radius() as u64;
        let walk = target.distance(champ).saturating_sub(attack_range);
        let walk_tick = walk / (champ.stat_cached.move_speed as u64).max(1);
        let start_tick = atk.start_timing * 100 / champ.attack_speed_mult().max(1);
        let mut score: i64 = 0;   // wave None → 타이밍 점수 0
        if (target.hp as i64) > attack_damage + 5 {
            if hp_ratio < 26 { score += 50 } else if hp_ratio < 51 { score += 25 }
        } else { score += 90 }
        // target 은 챔피언(ty != Minion) → +80/+25 분기 없음
        if target.distance_sq(champ) <= attack_range * attack_range { score += 20 }
        let dp = game_core::utils::distance(champ.x, champ.y, target.x, target.y) as i64;
        score = score + dp / -3000 + rnd2.gen_range(0..=10i64);
        let mine = score.max(1);
        let game_v = unsafe { target_score(2, &data, player, champ, &atk, target, None, &mut rnd) };
        let after_g: u64 = rnd.clone().gen(); let after_m: u64 = rnd2.clone().gen();
        println!("TS\thp={} max={} dx={} dmg={}\tatk_dmg={} hp_ratio={} range={} dist={} dsq<=r2={} dp={} start={} walk={}\tgame={:?}\tmine={}\trnd_same_after={}\t{}",
            hp, maxhp, dx, dmg, attack_damage, hp_ratio, attack_range, target.distance(champ), target.distance_sq(champ) <= attack_range * attack_range, dp, start_tick, walk_tick,
            game_v, mine, after_g == after_m, if game_v == Some(mine) { "MATCH" } else { "DIFF" });
        return;
    }
    if fam == "cnr" {
        let (x, y, d, tick, pat, ms) = (n(2, 0) as u64, n(3, 0) as u64, n(4, 0) as u64, n(5, 0) as usize, n(6, 0), n(7, 0) as usize);
        let mut tp: TeamPlan = Default::default();
        for i in 0..5usize {
            let e = cache.player_champion[1][i].unwrap();
            if ms > 0 { unsafe { std::ptr::write_volatile(&mut (*(e as *const Entity as *mut Entity)).stat_cached.move_speed, ms * (i + 1)); } }
            tp.vision.last_visible_pos[i] = (e.x / 2 + 100000 * i as u64, e.y / 2);   // 마지막 목격점을 실제 위치와 다르게
            tp.vision.last_visible_ticks[i] = match pat { 1 => if i < 2 { tick.saturating_sub(100) } else { 0 }, 2 => tick.saturating_sub(100), _ => 0 };
        }
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut rnd2 = rnd.clone();
        let mut dbg: DebugFrameData = Default::default();
        // ── 명세식 재현 ──
        let lapse = game_ai::player_awareness_lapse(player, &data);
        let jb = player.info.parameter.judgement_base();
        let ja = player.info.parameter.judge_accuracy();
        let seed = cache.game.seed(); let tk = cache.game.tick(); let tps = setting.tick_per_second as usize;
        let mut mine: Vec<usize> = Vec::new();
        let mut trace = String::new();
        if !lapse {
            for i in 0..5usize {
                let e = match cache.player_champion[1 - team][i] { Some(e) => e, None => continue };
                let vis = cache.game.is_visible(player.info.team, e.id);
                if vis { trace += &format!("i{}:vis ", i); continue; }
                let msr = e.stat_cached.move_speed as u64;
                let elapsed = tk.saturating_sub(tp.vision.last_visible_ticks[i]);
                let keep;
                if elapsed > tps * 3 {
                    let err = game_core::unseen_error_radius(jb, elapsed, tps);
                    if err > 300000 { trace += &format!("i{}:err{}>300k ", i, err); continue; }
                    let (ex, ey) = game_core::unseen_estimated_pos(seed, player.info.id, i, tk, tps, e.x, e.y, err);
                    let dist = game_core::utils::distance(ex, ey, x, y);
                    keep = dist <= err + msr * 3 * tps as u64 + d;
                    trace += &format!("i{}:E(el={},err={},est=({},{}),dist={},lim={}) ", i, elapsed, err, ex, ey, dist, err + msr * 3 * tps as u64 + d);
                } else {
                    let can_move = elapsed as u64 * msr;
                    let dfl = game_core::utils::distance(x, y, tp.vision.last_visible_pos[i].0, tp.vision.last_visible_pos[i].1).saturating_sub(d);
                    keep = can_move >= dfl;
                    trace += &format!("i{}:D(el={},cm={},dfl={}) ", i, elapsed, can_move, dfl);
                }
                if keep { mine.push(e.id); }
            }
        }
        let out = tp.can_near_enemies_range(2, &mut rnd, player, &data, x, y, d, &mut dbg);
        let got: Vec<usize> = out.iter().map(|e| e.id).collect();
        let after_g: u64 = rnd.clone().gen(); let after_m: u64 = rnd2.clone().gen();
        println!("CNR\tx={} y={} d={} tick={} pat={}\tlapse={} jb={} ja={} tps={}\t{}\tgame={:?}\tmine={:?}\trnd_untouched={}\t{}",
            x, y, d, tick, pat, lapse, jb, ja, tps, trace, got, mine, after_g == after_m, if got == mine { "MATCH" } else { "DIFF" });
        return;
    }
    if fam == "v54" {
        let (amount, range, ex) = (n(2, 100) as usize, n(3, 200000) as u64, n(4, 0));
        let eff = mkheal(amount, range);
        let spread = n(5, 20000) as u64;
        // 아군 4명을 champ 근처(x+spread*i)에 세우고 hp 를 채운다(기본 챔프는 hp=1) — near_allies 판별력 확보
        for i in 1..5usize {
            if let Some(a) = cache.player_champion[team][i] {
                let ap = a as *const Entity as *mut Entity;
                unsafe {
                    std::ptr::write_volatile(&mut (*ap).x, (*cp).x + spread * i as u64);
                    std::ptr::write_volatile(&mut (*ap).y, (*cp).y);
                    std::ptr::write_volatile(&mut (*ap).hp, 200 + 150 * i);
                    std::ptr::write_volatile(&mut (*ap).stat_cached.hp, 1000);
                    let aatk = n(8, 0) as usize;   // 8번째 인자: 아군 공격 이펙트 데미지(champion_hp_value 판별력용)
                    if aatk > 0 { std::ptr::write(&mut (*ap).attack_effect, Some(mkeff(aatk * i, AttackType::BaseAttack, 10))); std::ptr::write_volatile(&mut (*ap).level, 3 + i); }
                }
            }
        }
        unsafe { std::ptr::write_volatile(&mut (*cp).hp, 700); std::ptr::write_volatile(&mut (*cp).stat_cached.hp, 1000); }
        let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
        let mut dbg: DebugFrameData = Default::default();
        let ver = n(7, 55) as usize;
        let param = game_ai::calculate_score_parameter(ver, &mut rnd, player, &data, &mut dbg);
        let pb = &param as *const _ as *const u8;
        let (ap_ptr, alen): (*const u8, usize) = unsafe { (rd(pb, 0x14b8), rd(pb, 0x14d0)) };
        let champ: &Entity = unsafe { &*cp };
        let anchor = champ;
        let exclude_id = if ex == 1 { champ.id } else if ex == 2 && alen > 0 { unsafe { rd::<usize>(ap_ptr, 0x58) } } else { usize::MAX };
        let action: &Box<dyn Action> = champ.skill2();
        // ── 명세식 재현 ──
        let radius_opt = eff.ty.expected_ally_area_radius();
        let mut mine: i64 = 0; let mut trace = String::new();
        if let Some(radius) = radius_opt {
            for k in 0..alen {
                let apb = unsafe { ap_ptr.add(k * 216) };
                let apr: &game_ai::ChampionScoreParameter = unsafe { &*(apb as *const game_ai::ChampionScoreParameter) };
                let id: usize = unsafe { rd(apb, 0x58) };
                if id == exclude_id { trace += &format!("k{}:excl ", k); continue; }
                let ally = match cache.game.get_entity_by_id(id) { Some(a) => a, None => { trace += &format!("k{}:noent ", k); continue; } };
                let r = ally.radius() as u64 + radius;
                let dx = ally.x.abs_diff(anchor.x); let dy = ally.y.abs_diff(anchor.y);
                if dx * dx + dy * dy > r * r { trace += &format!("k{}:far ", k); continue; }
                let heal = eff.expected_heal_target(&ctx, champ as &dyn AbstractEntity, ally as &dyn AbstractEntity) as i64;
                let applyed: i64 = unsafe { rd(apb, 0x70) }; let risk_epic: i64 = unsafe { rd(apb, 0x88) };
                let incoming = apr.possible_risk(&data, action.duration() + 30) + applyed + risk_epic;
                let shield = (incoming * 3).min(eff.expected_shield_target(&ctx, champ as &dyn AbstractEntity, ally as &dyn AbstractEntity) as i64);
                if heal + shield > 0 {
                    let hv = game_ai::champion_hp_value(&data, &param, apr);
                    mine += hv * (heal + shield) / (ally.hp as i64).max(1);
                    trace += &format!("k{}:id{} heal={} sh={} hv={} hp={} ", k, id, heal, shield, hv, ally.hp);
                } else { trace += &format!("k{}:zero(id{} heal={} sh={} inc={} hp={}) ", k, id, heal, shield, incoming, ally.hp); }
            }
        } else { trace += "radius=None "; }
        let game_v = unsafe { v54(ver, &eff, &data, &param, champ, anchor, exclude_id, action) };
        println!("V54\tamount={} range={} ex={} tick={} ver={}\tna.len={} radius={:?} dur={}\t{}\tgame={}\tmine={}\t{}",
            amount, range, ex, cache.game.tick(), ver, alen, radius_opt, action.duration(), trace, game_v, mine, if game_v == mine { "MATCH" } else { "DIFF" });
        return;
    }
    println!("unknown family {}", fam);
}
