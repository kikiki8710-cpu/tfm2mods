#![allow(unused, dead_code, non_snake_case)]
//! 19차 배치B 오라클 1 — `#85 should_disengage_object_hunt`(pub, fight_check.rs:1361) 를 **실행**으로 확인한다.
//!  명세 독립 재구현(수법 ⓓ): 케이스마다 적/아군 챔피언 사본을 캠프 안팎·가시/비가시·HP% 로 배치하고
//!    enemies = 적 중 (visible_state[my_team]==Visible && dist_sq(e,camp) <= R²)
//!    if enemies.is_empty() → false
//!    nearest = min dist_sq(e, me) ; if check_favorable_engage_formation(0, player, data, nearest, 200000) → false
//!    allies = 아군 중 (id != me.id && dist_sq(a,camp) <= R² && a.hp*100/a.max_hp > 39).count()
//!    return enemies.len() >= allies
//!  check_favorable_engage_formation 은 같은 pub 함수를 **직접 호출**해 그 값을 쓴다(콜리 내부는 이 오라클 범위 밖).
//!  ★엔티티는 `ptr::read` 로 사본을 떠서(누수) 오프셋으로 고치고 `cache.player_champion[t][p]` 에 다시 꽂는다(TEMPLATE 수법 ⑦).
//!  ★케이스당 프로세스 1개(argv[1]) — 콜리 TLS 여부 미확인이라 보수적으로.
//! 케이스 = (nEV: 캠프 안 가시 적 0..3, nEI: 캠프 안 비가시 적 0..1, nA: 캠프 안 아군(나 제외) 0..3, hpv: 그 아군 HP 0=100%/1=39%/2=40%)
//!   case = nEV + 4*nEI + 8*nA + 32*hpv  (0..95)
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify19/B/oracle/v19B_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *const u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) }
}
/// Entity 사본(1728B) — 원본은 건드리지 않는다. 누수(drop 안 함).
fn clone_entity(e: &Entity) -> &'static Entity {
    unsafe {
        let b: Box<std::mem::ManuallyDrop<Entity>> = Box::new(std::mem::ManuallyDrop::new(std::ptr::read(e as *const Entity)));
        let p = Box::leak(b);
        &**p
    }
}
fn dist_sq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx); let dy = ay.abs_diff(by); dx * dx + dy * dy
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let nEV = case % 4; let nEI = (case / 4) % 2; let nA = (case / 8) % 4; let hpv = (case / 32) % 3;
    // mode 1(argv[2]) = 넓은 캠프(R=400000)·아군을 적 반대편 끝에 둬 check_favorable_engage_formation 의 200000 반경 밖으로 뺀다
    //   → 수적 비교(enemies.len() >= allies) 의 false 가지를 연다
    let mode: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    game.set_tick(1000);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);

    let (cx, cy, R) = (450_000u64, 350_000u64, if mode == 1 { 400_000u64 } else { 100_000u64 });
    let ally_off: u64 = if mode == 1 { 350_000 } else { 0 };
    let team = 0usize; let my_p = 1usize; // Jungle
    let far = (cx + 300_000, cy + 300_000);
    // 사본 배치
    let mut ents: [[Option<&'static Entity>; 5]; 2] = [[None; 5]; 2];
    for t in 0..2usize {
        for p in 0..5usize {
            let e = cache.player_champion[t][p].expect("champ");
            let c = clone_entity(e);
            let b = c as *const Entity as *const u8;
            let maxhp: u64 = rd(b, 0x628);
            let (x, y, vis, hp): (u64, u64, i64, u64) = if t == 1 {
                if p < nEV { (cx + 10_000 * (p as u64 + 1), cy, 0, maxhp) }
                else if p < nEV + nEI { (cx, cy + 10_000 * (p as u64 + 1), 1, maxhp) }
                else { (far.0, far.1, 0, maxhp) }
            } else if p == my_p {
                (cx, cy.saturating_sub(50_000), 0, maxhp)
            } else {
                let k = if p < my_p { p } else { p - 1 }; // 아군 인덱스 0..3
                if k < nA {
                    let hp = match hpv { 0 => maxhp,
                        1 => { let mut h = (39 * maxhp + 99) / 100; while h * 100 / maxhp > 39 { h -= 1; } h }
                        _ => { let mut h = (40 * maxhp + 99) / 100; while h * 100 / maxhp < 40 { h += 1; } h } };
                    (cx.saturating_sub(ally_off + 10_000 * (k as u64 + 1)), cy, 0, hp)
                } else { (far.0, far.1, 0, maxhp) }
            };
            wr::<u64>(b, 0x660, x); wr::<u64>(b, 0x668, y); wr::<u64>(b, 0x670, hp);
            // visible_state[team 0](+0x38) / [team 1](+0x50) 태그: 0 Visible / 1 Invisible
            wr::<i64>(b, 0x38, vis); wr::<i64>(b, 0x38 + 24, 0);
            ents[t][p] = Some(c);
            cache.player_champion[t][p] = Some(c);
        }
    }
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, Position::Jungle).expect("player");
    let me = ents[team][my_p].unwrap();
    let mb = me as *const Entity as *const u8;
    let me_id: usize = rd(mb, 0x5c0);
    let (mx, my): (u64, u64) = (rd(mb, 0x660), rd(mb, 0x668));

    // 독립 재구현
    let mut enemies: Vec<&Entity> = Vec::new();
    for p in 0..5 {
        let e = ents[1][p].unwrap(); let b = e as *const Entity as *const u8;
        let vis: i64 = rd(b, 0x38); let (x, y): (u64, u64) = (rd(b, 0x660), rd(b, 0x668));
        if vis == 0 && dist_sq(x, y, cx, cy) <= R * R { enemies.push(e); }
    }
    let mut cfef: i32 = -1;
    let pred = if enemies.is_empty() { false } else {
        let nearest = enemies.iter().copied().min_by_key(|e| { let b = *e as *const Entity as *const u8; dist_sq(rd(b, 0x660), rd(b, 0x668), mx, my) }).unwrap();
        let f = game_ai::check_favorable_engage_formation(0, player, &data, nearest, 200_000);
        cfef = f as i32;
        if f { false } else {
            let mut allies = 0usize;
            for p in 0..5 {
                let a = ents[0][p].unwrap(); let b = a as *const Entity as *const u8;
                let id: usize = rd(b, 0x5c0); let (x, y): (u64, u64) = (rd(b, 0x660), rd(b, 0x668));
                let hp: u64 = rd(b, 0x670); let maxhp: u64 = rd(b, 0x628);
                if id != me_id && dist_sq(x, y, cx, cy) <= R * R && hp * 100 / maxhp > 39 { allies += 1; }
            }
            enemies.len() >= allies
        }
    };
    let actual = game_ai::should_disengage_object_hunt(0, player, &data, (cx, cy), R);
    println!("case\t{}\tmode={}\tnEV={}\tnEI={}\tnA={}\thpv={}\tenemies={}\tcfef={}\tpred={}\tactual={}\t{}",
             case, mode, nEV, nEI, nA, hpv, enemies.len(), cfef, pred, actual, if pred == actual { "MATCH" } else { "MISMATCH" });
}
