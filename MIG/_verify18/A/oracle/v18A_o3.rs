#![allow(unused, dead_code, non_snake_case)]
//! 18차 배치A 오라클 3 — `#59 is_unreasonable_tower_dive_enemy`(pub) 를 **명세 독립 재구현**(수법 ⓓ)으로 대조한다.
//!  콜리 중 pub 인 것(max_range_cached · can_tower_focused_when_battle · can_trace_without_tower ·
//!  Blackboard::is_recent_visible)은 그대로 부르고, `ready_damage_to_target` 은 **in:game_ai 라 못 부른다** →
//!  default 챔프(액션 파라미터 0)라 0 으로 가정한다. ⚠이 가정이 틀리면 target.hp=1 케이스에서 불일치로 드러난다(자기검증).
//!  축: my_hp_ratio {0,24,25,34,35,44,45,100} × target_hp_ratio {0,1,25,26,50,51,100} × with_declared_dive {F,T}
//!      × 아군 근접 {0,1(=나만),2} × 적 근접(시야) {0,1,3} × 거리 {in_range, far} × target 자리 {적 미드타워(진입 필요), 맵 중앙(불필요)}
//!  수법 ⑦: Entity 는 전 필드 pub 이라 raw 포인터로 hp/stat_cached.hp/x/y 를 직접 쓴다. blackboard[1].last_visible[pos] 로 시야 유도.
//! TLS: MAX_RANGE_CACHE(max_range_cached) — 양쪽(실제·재구현)이 같은 캐시를 보므로 대조엔 무해. 단 값은 첫 호출 것이 재생된다(범위 명시).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify18/A/oracle/v18A_o3.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_unaligned(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: &Entity) -> *const u8 { e as *const Entity as *const u8 }
// ⚠함정(18차 배치A 실측): `fn set_pos(e: &Entity, ..)` 처럼 **`&Entity` 인자를 받아 그 안에서 raw store** 하면
//   인자가 `readonly noalias` 라 LLVM 이 그 store 를 UB 로 보고 **통째로 지운다**(volatile 로 읽어도 옛 값). 포인터로 받아라.
fn set_pos(e: *const Entity, x: u64, y: u64) { let p = e as *const u8; wr(p, 0x660, x); wr(p, 0x668, y); }
fn set_hp(e: *const Entity, hp: usize, max: usize) { let p = e as *const u8; wr(p, 0x670, hp); wr(p, 0x628, max); }
fn d2(a: &Entity, b: &Entity) -> u64 { let dx = a.x.abs_diff(b.x); let dy = a.y.abs_diff(b.y); dx * dx + dy * dy }

fn spec(ctx: &GameContext, cache: &AbstractGameWithCache, data: &OperationData, bb: &[Blackboard; 2], game: &Game,
        player: &PlayerState, target: &Entity, wdd: bool) -> (bool, String) {
    let team = player.info.team; let enemy = 1 - team;
    let champ = cache.player_champion[team][2].unwrap();
    let approach = game_ai::plan_legacy::old::max_range_cached(data, champ, target) + 25000;
    let needs = game_ai::can_tower_focused_when_battle(ctx, cache, player, target.x, target.y, approach)
             && !game_ai::can_trace_without_tower(ctx, cache, player.info.id, target.x, target.y, approach);
    if !needs { return (false, format!("needs=false approach={}", approach)); }
    let my_r = champ.hp * 100 / std::cmp::max(champ.stat_cached.hp, 1);
    let t_r = target.hp * 100 / std::cmp::max(target.stat_cached.hp, 1);
    let ready: usize = 0; // 가정(default 챔프)
    let in_range = d2(champ, target) <= approach * approach;
    let allies = cache.player_champion[team].iter().flatten().filter(|a| d2(a, target) < 19600000001).count();
    let enemies = cache.player_champion[enemy].iter().flatten()
        .filter(|e| d2(e, target) < 19600000001 && bb[enemy].is_recent_visible(game as &dyn AbstractGame, player, e)).count();
    let ally_adv = allies > 1 && allies > enemies;
    let a = ready >= target.hp && my_r > 24;
    let b = t_r < 26 && my_r > 34 && (in_range || ally_adv);
    let c = wdd && my_r > 44 && t_r < 51 && ally_adv;
    (!(a || b || c), format!("approach={} my_r={} t_r={} in_range={} allies={} enemies={} A={} B={} C={}", approach, my_r, t_r, in_range, allies, enemies, a, b, c))
}

fn main() {
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Mid).expect("player");
    let champ = cache.player_champion[0][2].expect("champ");
    let target = cache.player_champion[1][2].expect("target");
    let ally2 = cache.player_champion[0][0].expect("ally top");
    let en_a = cache.player_champion[1][0].expect("enemy top");
    let en_b = cache.player_champion[1][3].expect("enemy bottom");
    let en_c = cache.player_champion[1][4].expect("enemy support");
    let tower = cache.mid_tower[1].expect("enemy mid tower");
    let far_pt = (setting.width / 2, setting.height / 2);
    println!("towers\t{}\ttick={}\tenemy_mid_tower=({},{})\tcenter=({},{})", game.world.tower_ids.len(), game.tick(), tower.x, tower.y, far_pt.0, far_pt.1);

    // ★2차 시도: 적 타워 attack_effect.range(+0x4a0, tag +0x4c0 ≠ -1 이면 Some) 를 300000 으로 키워 초점 반경을 넓힌다
    let mut tw_n = 0usize;
    for arr in [&cache.top_tower, &cache.top_tower2, &cache.mid_tower, &cache.mid_tower2, &cache.bottom_tower, &cache.bottom_tower2] {
        if let Some(e) = arr[1] {
            let tag: i32 = rd(ep(e), 0x4c0); let r0: u64 = rd(ep(e), 0x4a0);
            if tw_n == 0 { println!("tower0	attack_effect.tag={}	range={}	growth_range={}", tag, r0, rd::<u64>(ep(e), 0x4a8)); }
            wr::<u64>(ep(e), 0x4a0, 300000); tw_n += 1;
        }
    }
    for e in cache.twin_towers[1].iter() { wr::<u64>(ep(e), 0x4a0, 300000); tw_n += 1; }
    println!("towers_widened	{}", tw_n);
    // ★타워 주변 스캔: approach 링 전체가 타워 초점 안에 드는 target 자리를 찾는다(없으면 needs 는 영원히 false)
    let approach0 = { set_pos(champ as *const Entity, tower.x + 20000, tower.y); game_ai::plan_legacy::old::max_range_cached(&data, champ, target) + 25000 };
    let mut found: Option<(u64, u64)> = None; let mut scan_n = 0usize; let mut scan_tf = 0usize; let mut scan_ct = 0usize;
    for gx in -8i64..=8 { for gy in -8i64..=8 {
        let (sx, sy) = ((tower.x as i64 + gx * 10000) as u64, (tower.y as i64 + gy * 10000) as u64);
        let tf = game_ai::can_tower_focused_when_battle(&ctx, &cache, player, sx, sy, approach0);
        let ct = game_ai::can_trace_without_tower(&ctx, &cache, player.info.id, sx, sy, approach0);
        scan_n += 1; if tf { scan_tf += 1; } if ct { scan_ct += 1; }
        if tf && !ct && found.is_none() { found = Some((sx, sy)); }
    } }
    println!("scan	approach={}	cells={}	tower_focused_when_battle=true:{}	can_trace=true:{}	needs_true_first={:?}", approach0, scan_n, scan_tf, scan_ct, found);
    let tower_pt = found.unwrap_or((tower.x, tower.y));
    // 쓰기 유효성 디버그
    set_pos(target as *const Entity, tower_pt.0, tower_pt.1); set_hp(champ as *const Entity, 34, 100);
    println!("dbg_write	target.x={}	rd(0x660)={}	volatile={}	champ.hp={}	rd(0x670)={}", target.x, rd::<u64>(ep(target), 0x660),
             unsafe { std::ptr::read_volatile(ep(target).add(0x660) as *const u64) }, champ.hp, rd::<usize>(ep(champ), 0x670));
    let my_rs = [0usize, 24, 25, 34, 35, 44, 45, 100];
    let t_rs = [0usize, 1, 25, 26, 50, 51, 100];
    let mut n = 0usize; let mut mism = 0usize; let mut trues = 0usize; let mut needs_false = 0usize;
    let mut cA = 0usize; let mut cB = 0usize; let mut cC = 0usize; let mut cAdv = 0usize; let mut cIn = 0usize; let mut cEn = 0usize;
    let mut cA = 0usize; let mut cB = 0usize; let mut cC = 0usize; let mut cAdv = 0usize; let mut cIn = 0usize; let mut cEn = 0usize;
    for &at_tower in &[true, false] {
        let (tx, ty) = if at_tower { tower_pt } else { far_pt };
        set_pos(target as *const Entity, tx, ty);
        for &dist in &[20000u64, 300000u64] {
            set_pos(champ as *const Entity, tx + dist, ty);
            for &allies_near in &[0usize, 1, 2] {
                // 0 = 나도 멀리(300000 밖으로 뺀다 → 그럼 in_range 도 false), 1 = 나만, 2 = 나 + Top 아군
                if allies_near == 0 { set_pos(champ as *const Entity, tx + 300000, ty + 300000); } else { set_pos(champ as *const Entity, tx + dist, ty); }
                if allies_near == 2 { set_pos(ally2 as *const Entity, tx, ty + 30000); } else { set_pos(ally2 as *const Entity, 50000, 50000); }
                for &enemies_near in &[0usize, 1, 3] {
                    // 적 Top/Bottom/Support 를 근접 + 시야(last_visible=950 → +120 ≥ 1000)
                    let ens = [en_a, en_b, en_c]; let eposs = [0usize, 3, 4];
                    for (k, e) in ens.iter().enumerate() {
                        if k < enemies_near { set_pos(*e as *const Entity, tx + 20000 * (k as u64 + 1), ty + 10000); wr::<usize>(&bb[1] as *const Blackboard as *const u8, 0x1e0 + 8 * eposs[k], 950); }
                        else { set_pos(*e as *const Entity, 900000, 900000); wr::<usize>(&bb[1] as *const Blackboard as *const u8, 0x1e0 + 8 * eposs[k], 0); }
                    }
                    for &wdd in &[false, true] {
                        for &mr in &my_rs { for &tr in &t_rs {
                            set_hp(champ as *const Entity, mr, 100); set_hp(target as *const Entity, tr, 100);
                            let actual = game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy(1, player, &data, target, wdd);
                            let (pred, why) = spec(&ctx, &cache, &data, &bb, &game, player, target, wdd);
                            n += 1; if actual { trues += 1; } if why.starts_with("needs=false") { needs_false += 1; }
                            if why.contains("A=true") { cA += 1; } if why.contains("B=true") { cB += 1; } if why.contains("C=true") { cC += 1; }
                            if why.contains("in_range=true") { cIn += 1; } if why.contains("allies=2 enemies=0") || why.contains("allies=2 enemies=1") { cAdv += 1; }
                            if why.contains("enemies=3") { cEn += 1; }
                            if why.contains("A=true") { cA += 1; } if why.contains("B=true") { cB += 1; } if why.contains("C=true") { cC += 1; }
                            if why.contains("in_range=true") { cIn += 1; } if why.contains("allies=2 enemies=0") || why.contains("allies=2 enemies=1") { cAdv += 1; }
                            if why.contains("enemies=3") { cEn += 1; }
                            if actual != pred {
                                mism += 1;
                                if mism <= 12 { println!("MISM\tat_tower={}\tdist={}\tallies={}\tenemies={}\twdd={}\tmr={}\ttr={}\tactual={}\tpred={}\t{}", at_tower, dist, allies_near, enemies_near, wdd, mr, tr, actual, pred, why); }
                            }
                        } }
                    }
                }
            }
        }
    }
    println!("cases\t{}\tmism\t{}\ttrue(무리)\t{}\tfalse\t{}\tneeds_false\t{}", n, mism, trues, n - trues, needs_false);
    println!("coverage	A_true={}	B_true={}	C_true={}	in_range_true={}	ally_adv_true(allies2,enemies0/1)={}	enemies3_seen={}", cA, cB, cC, cIn, cAdv, cEn);
    // 대표 케이스 몇 개
    set_pos(target as *const Entity, tower_pt.0, tower_pt.1); set_pos(champ as *const Entity, tower_pt.0 + 20000, tower_pt.1); set_pos(ally2 as *const Entity, 50000, 50000);
    for e in [en_a, en_b, en_c] { set_pos(e as *const Entity, 900000, 900000); }
    for &(mr, tr, wdd) in &[(100usize, 100usize, false), (100, 25, false), (34, 25, false), (35, 25, false), (100, 26, false), (45, 50, true), (45, 50, false)] {
        set_hp(champ as *const Entity, mr, 100); set_hp(target as *const Entity, tr, 100);
        let (pred, why) = spec(&ctx, &cache, &data, &bb, &game, player, target, wdd);
        println!("rep\tmr={}\ttr={}\twdd={}\tactual={}\tpred={}\t{}", mr, tr, wdd,
                 game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy(1, player, &data, target, wdd), pred, why);
    }
}
