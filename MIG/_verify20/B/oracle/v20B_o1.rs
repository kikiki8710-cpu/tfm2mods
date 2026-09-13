#![allow(unused, dead_code, non_snake_case)]
//! 20차 배치B 오라클 1 — `#106 TeamPlan::v25_objective_posture`(pub, objective_discipline.rs:207) 를 **실행**으로 확인한다.
//!  수법 ⓓ 독립 재구현: 명세 `logic` 을 그대로 Rust 로 옮겨(predict) 실제 호출(actual)과 kind·카운트·focus_enemy 를 전부 대조한다.
//!  콜리(is_ignored_well_enemy / is_recent_visible / far_split_pressure / is_top_side·is_bottom_side·is_near_mid_line / utils::distance)는
//!  같은 pub 함수를 직접 부른다(콜리 내부는 범위 밖).
//!  ★엔티티는 `ptr::read` 사본(누수)을 오프셋으로 고쳐 `cache.player_champion` 에 다시 꽂는다(TEMPLATE 수법 ⑦ · *mut 로 씀).
//!  ★self(TeamPlan) 은 Default 후 objective 태그(+0x41f)/phase(+0x420)만 raw write. vision 은 0(last_visible_pos=(0,0), last_checked=0).
//!  ★케이스당 프로세스 1개(argv[1]) — 콜리 TLS 여부 미확인이라 보수적으로.
//!  ★한계: 에픽/세르펜 엔티티는 tick 1000 에 존재하지 않아 object_hp_ratio=100 고정 → execute_window(<26) 경로(Commit·L285/287 의 !ew 절)는 못 연다.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify20/B/oracle/v20B_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::team_plan::{TeamPlan, ObjectPhase, v25_objective_far_split_pressure};
use game_ai::plan_legacy::old::is_ignored_well_enemy;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *mut u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut T, v) }
}
fn clone_entity(e: &Entity) -> &'static mut Entity {
    unsafe {
        let b: Box<std::mem::ManuallyDrop<Entity>> = Box::new(std::mem::ManuallyDrop::new(std::ptr::read(e as *const Entity)));
        let p = Box::leak(b);
        &mut **p
    }
}
fn dsq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 { let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by); dx * dx + dy * dy }

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); self.0 >> 33 }
    fn pick(&mut self, n: u64) -> u64 { self.next() % n }
}
/// (cx,cy) 에서 거리 d, 방향 k(0..8) 로 떨어진 점. 맵 안으로 clamp.
fn at(cx: u64, cy: u64, d: u64, k: u64) -> (u64, u64) {
    let (sx, sy): (i64, i64) = match k % 8 { 0 => (1, 0), 1 => (0, 1), 2 => (-1, 0), 3 => (0, -1), 4 => (1, 1), 5 => (-1, 1), 6 => (1, -1), _ => (-1, -1) };
    let diag = k % 8 >= 4;
    let dd = if diag { (d as f64 / 1.41421356) as i64 } else { d as i64 };
    let x = (cx as i64 + sx * dd).clamp(20_000, 940_000) as u64;
    let y = (cy as i64 + sy * dd).clamp(20_000, 940_000) as u64;
    (x, y)
}

fn main() {
    let case: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    // mode(argv[2]): 1 = 내 챔프 hp 30% 강제(low_hp_contact 경로) · 2 = 내 hp 34%/35% 경계
    let mode: u64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut g = Lcg(case.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(12345));
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
    let tick: usize = 1000;
    game.set_tick(tick);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);

    // ── 케이스 구성 ──
    let team = (g.pick(2)) as usize;
    let enemy = 1 - team;
    let my_p = 1usize; // Jungle
    let target_is_morgard = g.pick(2) == 0;
    let target = if target_is_morgard { JungleType::Morgard } else { JungleType::Serpen };
    let target_i8: i8 = if target_is_morgard { 4 } else { 5 };
    // objective: 0=일치 1=불일치(다른 오브젝트) 2=None — 일치가 대부분이 되게
    let obj_mode = match g.pick(10) { 0 => 1, 1 => 2, _ => 0 };
    let phase_i = g.pick(4) as u8; // 0 None 1 Setup 2 Assemble 3 Hunt
    let (cx, cy) = map.camp_pos(target, team == 0);
    let my_d = [80_000u64, 140_000, 149_000, 151_000, 160_000, 179_000, 181_000, 220_000][g.pick(8) as usize];
    let my_k = g.pick(8);
    let (mx, my) = at(cx, cy, my_d, my_k);

    let mut ents: [[Option<&'static Entity>; 5]; 2] = [[None; 5]; 2];
    let mut desc = String::new();
    for t in 0..2usize {
        for p in 0..5usize {
            let e0 = cache.player_champion[t][p].expect("champ");
            let c = clone_entity(e0);
            let b = c as *mut Entity as *mut u8;
            // ★default 챔프의 stat_cached.hp == 1 (TEMPLATE 함정 ⑥ 계열) → 1000 으로 세팅해 HP% 축을 연다
            let maxhp: u64 = 1000; wr::<u64>(b, 0x628, maxhp);
            let mut hp_pct = [100u64, 100, 100, 60, 51, 49, 45, 40, 39, 30][g.pick(10) as usize];
            if t == team && p == my_p { if mode == 1 { hp_pct = 30; } else if mode == 2 { hp_pct = 34 + g.pick(2); } }
            let mut hp = maxhp * hp_pct / 100;
            if hp == 0 { hp = 1; }
            let (x, y, vis): (u64, u64, i64);
            if t == team && p == my_p {
                x = mx; y = my; vis = 0;
            } else {
                // 배치 종류: 0 내 근처 / 1 캠프 근처 / 2 멀리
                let kind = g.pick(3);
                let k = g.pick(8);
                let (px, py) = match kind {
                    0 => { let d = [40_000u64, 120_000, 139_000, 141_000, 165_000, 171_000, 179_000, 181_000][g.pick(8) as usize]; at(mx, my, d, k) }
                    1 => { let d = [30_000u64, 100_000, 150_000, 179_000, 181_000, 189_000, 191_000, 229_000, 231_000][g.pick(9) as usize]; at(cx, cy, d, k) }
                    _ => { at(cx, cy, 400_000 + 50_000 * g.pick(4), k) }
                };
                x = px; y = py;
                vis = if t == enemy { if g.pick(4) == 0 { 1 } else { 0 } } else { 0 };
            }
            wr::<u64>(b, 0x660, x); wr::<u64>(b, 0x668, y); wr::<u64>(b, 0x670, hp);
            // visible_state[0](+0x38) / [1](+0x50) 태그: 0 Visible / 1 Invisible(last_x,last_y 는 그대로)
            wr::<i64>(b, 0x38, vis); wr::<i64>(b, 0x38 + 24, vis);
            // move_speed 는 default 1 → closure#8 이 갈리도록 일부에 큰 값
            if t == enemy { let msp = [1u64, 1, 500, 5000][g.pick(4) as usize]; wr::<u64>(b, 0x640, msp); }
            desc.push_str(&format!("[{}{}:({},{}) hp{}%({}/{}) v{}]", t, p, x, y, hp_pct, hp, maxhp, vis));
            ents[t][p] = Some(&*c);
            cache.player_champion[t][p] = Some(&*c);
        }
    }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, Position::Jungle).expect("player");

    // ── self(TeamPlan) ──
    let mut tp: TeamPlan = Default::default();
    let tpp = &mut tp as *mut TeamPlan as *mut u8;
    let obj_tag: i8 = match obj_mode { 0 => if target_is_morgard { 0 } else { 1 }, 1 => if target_is_morgard { 1 } else { 0 }, _ => -1 };
    wr::<i8>(tpp, 0x41f, obj_tag);
    wr::<u8>(tpp, 0x420, phase_i);
    wr::<u8>(tpp, 0x421, 1);

    // ── 독립 재구현(predict) ──
    let me = ents[team][my_p].unwrap();
    let mb = me as *const Entity as *const u8;
    let hp_pct = |e: &Entity| -> u64 { let b = e as *const Entity as *const u8; let hp: u64 = rd(b, 0x670); let mh: u64 = rd(b, 0x628); hp * 100 / mh };
    let pos = |e: &Entity| -> (u64, u64) { let b = e as *const Entity as *const u8; (rd(b, 0x660), rd(b, 0x668)) };
    let vis_from_me = |e: &Entity| -> bool { let b = e as *const Entity as *const u8; let tag: i64 = rd(b, 0x38 + 24 * team); tag == 0 };
    let allies: Vec<&Entity> = (0..5).filter_map(|p| ents[team][p]).collect();
    let enemies: Vec<&Entity> = (0..5).filter_map(|p| ents[enemy][p]).collect();
    let (mxx, myy) = pos(me);
    let wait_pos = if target_is_morgard { map.camp_pos(JungleType::Stump, team == 0) } else { map.camp_pos(JungleType::Rhino, team == 0) };

    let mut pred_none = false;
    if obj_mode != 0 { pred_none = true; }
    let object_hp_ratio: u64 = 100; // 에픽/세르펜 없음
    let execute_window = object_hp_ratio < 26;
    let objective_untouched = object_hp_ratio > 84;
    let near_ally = allies.iter().filter(|x| hp_pct(x) > 39 && { let (x1, y1) = pos(x); dsq(x1, y1, mxx, myy) < 140_000u64 * 140_000 + 1 }).count();
    let near_enemy = enemies.iter().filter(|x| vis_from_me(x) && !is_ignored_well_enemy(0, player, x) && { let (x1, y1) = pos(x); dsq(x1, y1, mxx, myy) < 170_000u64 * 170_000 + 1 }).count();
    let camp_ally = allies.iter().filter(|x| hp_pct(x) > 39 && { let (x1, y1) = pos(x); dsq(x1, y1, cx, cy) < 180_000u64 * 180_000 + 1 }).count();
    let camp_vis_enemy = enemies.iter().filter(|x| hp_pct(x) > 39 && vis_from_me(x) && !is_ignored_well_enemy(0, player, x) && { let (x1, y1) = pos(x); dsq(x1, y1, cx, cy) < 190_000u64 * 190_000 + 1 }).count();
    let mut camp_possible = 0usize;
    for (p, x) in enemies.iter().enumerate() {
        let b = *x as *const Entity as *const u8;
        let hp: u64 = rd(b, 0x670); let mh: u64 = rd(b, 0x628);
        if hp * 100 / mh < 50 { continue; }
        if bb[enemy].is_recent_visible(&game as &dyn AbstractGame, player, x) { continue; }
        let last: (u64, u64) = (rd(tpp as *const u8, 0x230 + 16 * p), rd(tpp as *const u8, 0x238 + 16 * p));
        let d = game_core::utils::distance(last.0, last.1, cx, cy).saturating_sub(180_000);
        let last_checked: usize = rd(tpp as *const u8, 0x2d0 + 8 * p);
        let elapsed = tick.saturating_sub(last_checked) as u64;
        let msp: u64 = rd(b, 0x640);
        if elapsed * msp >= d { camp_possible += 1; }
    }
    let pressure = camp_vis_enemy + camp_possible;
    let side_ally = allies.iter().filter(|x| hp_pct(x) > 39 && { let (x1, y1) = pos(x);
        if target_is_morgard { game_core::is_top_side(&ctx, x1, y1) || game_core::is_near_mid_line(&ctx, x1, y1) }
        else { game_core::is_bottom_side(&ctx, x1, y1) || game_core::is_near_mid_line(&ctx, x1, y1) } }).count();
    let focus: Option<usize> = enemies.iter().filter(|x| vis_from_me(x) && !is_ignored_well_enemy(0, player, x) && { let (x1, y1) = pos(x);
            dsq(x1, y1, mxx, myy) < 180_000u64 * 180_000 + 1 && dsq(x1, y1, cx, cy) < 230_000u64 * 230_000 + 1 })
        .min_by_key(|x| { let (x1, y1) = pos(x); dsq(x1, y1, mxx, myy).saturating_add(dsq(x1, y1, cx, cy) >> 2) })
        .map(|e| { let b = *e as *const Entity as *const u8; rd::<usize>(b, 0x5c0) });
    let far_split = v25_objective_far_split_pressure(player, &data, target);
    let split_screen_ready = camp_ally >= 2 && far_split && pressure <= camp_ally + 2;
    let champ_camp = dsq(mxx, myy, cx, cy);
    let approach = champ_camp > 150_000u64 * 150_000;
    let isolated = near_enemy >= 2 && approach && near_ally < 2;
    let overloaded = !(camp_vis_enemy < camp_ally + 2 || camp_vis_enemy < 3);
    let low_hp = approach && hp_pct(me) < 35 && near_enemy != 0;
    let pred_kind: i8 = if pred_none { -1 }
        else if !execute_window && (low_hp || (isolated && overloaded)) { 4 }
        else if !execute_window && champ_camp > 180_000u64 * 180_000 && camp_ally < 2 && near_ally < 2 && near_enemy != 0 { 3 }
        else if objective_untouched && approach && pressure <= camp_ally + 1 && focus.is_some() && !isolated
                && (camp_ally >= 2 || near_ally >= 2 || side_ally >= 4 || split_screen_ready) { 2 }
        else if execute_window && camp_ally >= 2 && pressure <= camp_ally + 1 { 0 }
        else if (phase_i == 1 || phase_i == 3) && camp_ally >= 2 && pressure <= camp_ally + 1 { 1 }
        else { -1 };

    // ── 실제 호출 ──
    let r = tp.v25_objective_posture(0, player, &data, target);
    let rb = &r as *const _ as *const u8;
    let tag0: i64 = rd(rb, 0);
    let act_none = tag0 == -1;
    let (a_focus_tag, a_focus, a_cp, a_wp, a_na, a_ne, a_ca, a_pr, a_kind, a_tgt): (i64, usize, (u64, u64), (u64, u64), u64, u64, u64, u64, u8, i8) =
        (tag0, rd(rb, 8), (rd(rb, 0x10), rd(rb, 0x18)), (rd(rb, 0x20), rd(rb, 0x28)), rd(rb, 0x30), rd(rb, 0x38), rd(rb, 0x40), rd(rb, 0x48), rd(rb, 0x50), rd(rb, 0x51));

    let mut ok_all = true;
    let mut why = String::new();
    if act_none != (pred_kind == -1) { ok_all = false; why.push_str("none;"); }
    if !act_none {
        if a_kind as i8 != pred_kind { ok_all = false; why.push_str("kind;"); }
        if a_na as usize != near_ally { ok_all = false; why.push_str("near_ally;"); }
        if a_ne as usize != near_enemy { ok_all = false; why.push_str("near_enemy;"); }
        if a_ca as usize != camp_ally { ok_all = false; why.push_str("camp_ally;"); }
        if a_pr as usize != pressure { ok_all = false; why.push_str("pressure;"); }
        let af = if a_focus_tag == 1 { Some(a_focus) } else { None };
        if af != focus { ok_all = false; why.push_str("focus;"); }
        if a_cp != (cx, cy) { ok_all = false; why.push_str("camp_pos;"); }
        if a_wp != wait_pos { ok_all = false; why.push_str("wait_pos;"); }
        if a_tgt != target_i8 { ok_all = false; why.push_str("target;"); }
    }
    println!("case\t{}m{}\tteam={}\ttgt={}\tobj={}\tphase={}\tmy_d={}\t| pred kind={} na={} ne={} ca={} cve={} cpe={} sa={} focus={:?} fs={} appr={} iso={} ovl={} lowhp={} | act none={} kind={} na={} ne={} ca={} pr={} focus_tag={} focus={} tgt={}\t{}\t{}",
        case, mode, team, target_i8, obj_tag, phase_i, my_d, pred_kind, near_ally, near_enemy, camp_ally, camp_vis_enemy, camp_possible, side_ally, focus, far_split, approach, isolated, overloaded, low_hp,
        act_none, a_kind, a_na, a_ne, a_ca, a_pr, a_focus_tag, a_focus, a_tgt, if ok_all { "MATCH" } else { "MISMATCH" }, why);
    println!("cfg\t{}\t{}", case, desc);
}
