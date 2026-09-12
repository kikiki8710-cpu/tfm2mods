#![allow(unused, dead_code, non_snake_case)]
//! 15차 배치A 오라클 1 — #22 can_tower_focused · #24 v23_healthy_allies_near_point · #20 v27_active_objective_discipline
//! 한 프로세스에서 세 함수를 잰다(셋 다 TLS 메모 없음 — tlsscan 대상 아님, 순수 함수).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify15/A/oracle/o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn dist_sq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx);
    let dy = ay.abs_diff(by);
    dx * dx + dy * dy
}

/// #22 명세 L34/L42 의 사거리 식(명세 + 이번 라운드 정정: 15000 은 game_ai 리터럴)
fn tower_reach(t: &Entity, champ: &Entity) -> u64 {
    let eff = t.attack_effect.as_ref().unwrap();
    eff.range(t) + 15000 + t.radius() as u64 + champ.radius() as u64
}

/// 대립가설: Effect::range 에 15000 이 이미 포함 → 명세식에서 15000 을 빼면 같은 값?
fn tower_reach_alt(t: &Entity, champ: &Entity) -> u64 {
    let eff = t.attack_effect.as_ref().unwrap();
    eff.range(t) + t.radius() as u64 + champ.radius() as u64
}

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *mut u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut T, v) }
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
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        println!("towers\t{}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
    }

    // ───────────── #24 v23_healthy_allies_near_point ─────────────
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let allies: Vec<&Entity> = (0..5).filter_map(|p| cache.player_champion[0][p]).collect();
        let c0 = allies[0];
        for a in allies.iter() {
            println!("ALLY\tid={}\tx={}\ty={}\thp={}\tmax={}\tdist_sq_from_c0={}", a.id, a.x, a.y, a.hp, a.stat_cached.hp, dist_sq(c0.x, c0.y, a.x, a.y));
        }
        let mut n_match = 0; let mut n_mis = 0;
        // 반경 스윕: 각 아군까지의 거리 d 에 대해 range = d-1 / d / d+1 (경계 ule 판별)
        let mut ranges: Vec<u64> = vec![0, 1, 1000, 100000, 10_000_000];
        for a in allies.iter() {
            let d = (dist_sq(c0.x, c0.y, a.x, a.y) as f64).sqrt() as u64;
            for r in [d.saturating_sub(1), d, d + 1, d + 2] { ranges.push(r); }
        }
        for &r in ranges.iter() {
            for &mh in [0usize, 1, 50, 99, 100, 101].iter() {
                let got = game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point(player, &data, c0.x, c0.y, r, mh);
                let pred = allies.iter().filter(|a| {
                    a.hp * 100 / a.stat_cached.hp >= mh && dist_sq(c0.x, c0.y, a.x, a.y) <= r * r
                }).count();
                let pred_lt = allies.iter().filter(|a| {
                    a.hp * 100 / a.stat_cached.hp >= mh && dist_sq(c0.x, c0.y, a.x, a.y) < r * r
                }).count();
                let v = if got == pred { n_match += 1; if pred != pred_lt { "MATCH(판별 ule)" } else { "MATCH" } } else { n_mis += 1; "**MISMATCH**" };
                println!("V23\trange={}\tmin_hp={}\tgame={}\tmine(ule)={}\talt(ult)={}\t{}", r, mh, got, pred, pred_lt, v);
            }
        }
        println!("V23_SUMMARY\tmatch={}\tmismatch={}", n_match, n_mis);
    }
    // #24 — hp 조작: 아군 1명 hp 를 max 의 36% 로 내리고 min_hp 35/36/37 로 경계 판별
    {
        let ids: Vec<usize> = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            (0..5).filter_map(|p| cache.player_champion[0][p]).map(|e| e.id).collect()
        };
        let victim = ids[1];
        for id in ids.iter() {
            let e = game.world.entity.get_mut(*id).unwrap();
            e.stat_cached.hp = 1000; e.hp = 1000;
        }
        {
            let e = game.world.entity.get_mut(victim).unwrap();
            let mx = e.stat_cached.hp;
            e.hp = 360;
            println!("HPSET\tid={}\thp={}\tmax={}\tpct={}", victim, e.hp, mx, e.hp * 100 / mx);
        }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let allies: Vec<&Entity> = (0..5).filter_map(|p| cache.player_champion[0][p]).collect();
        let c0 = allies[0];
        for &mh in [0usize, 35, 36, 37, 100, 101].iter() {
            let got = game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point(player, &data, c0.x, c0.y, 10_000_000, mh);
            let pred = allies.iter().filter(|a| a.hp * 100 / a.stat_cached.hp >= mh).count();
            println!("V23HP\tmin_hp={}\tgame={}\tmine={}\t{}", mh, got, pred, if got == pred { "MATCH" } else { "**MISMATCH**" });
        }
    }
    // 원복
    {
        let ids: Vec<usize> = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            (0..5).filter_map(|p| cache.player_champion[0][p]).map(|e| e.id).collect()
        };
        let e = game.world.entity.get_mut(ids[1]).unwrap();
        e.hp = e.stat_cached.hp;
    }

    // ───────────── #22 can_tower_focused ─────────────
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let champ: &Entity = cache.player_champion[0][0].unwrap();
        let towers = cache.towers(1, &pool);
        println!("TOWERS\tenemy_count={}\tchamp_id={}\tchamp_radius={}\tradius_mult={}", towers.len(), champ.id, champ.radius(), champ.stat_buff_cached.radius_mult);
        let mut n_match = 0; let mut n_mis = 0; let mut n_disc = 0;
        for (k, t) in towers.iter().enumerate() {
            let tag: i64 = rd(*t as *const Entity as *const u8, 0x68);
            let ne_tag: i64 = rd(*t as *const Entity as *const u8, 0x88);
            let has_eff = t.attack_effect.is_some();
            if !has_eff { println!("TOWER\t#{}\tid={}\ttag={}\tattack_effect=None → skip", k, t.id, tag); continue; }
            let eff = t.attack_effect.as_ref().unwrap();
            let R = tower_reach(t, champ);
            let Ralt = tower_reach_alt(t, champ);
            println!("TOWER\t#{}\tid={}\ttag={}\tne_tag={}\tx={}\ty={}\teff.range={}\tgrowth={}\tlevel={}\tsb_range={}\tradius={}\trange(t)={}\tR={}\tRalt={}",
                     k, t.id, tag, ne_tag, t.x, t.y, eff.range, eff.growth_range, t.level, t.stat_buff_cached.range, t.radius(), eff.range(t), R, Ralt);
            // x 축으로 R-1, R, R+1, Ralt, Ralt+1 떨어진 점
            for (nm, dx) in [("R-1", R - 1), ("R", R), ("R+1", R + 1), ("Ralt", Ralt), ("Ralt+1", Ralt + 1), ("R+15000", R + 15000)] {
                let x = t.x + dx; let y = t.y;
                let got = game_ai::can_tower_focused(&ctx, &cache, player, x, y);
                let d2 = dist_sq(x, y, t.x, t.y);
                // 다른 타워가 같은 점을 덮을 수 있으니 predicted 는 전 타워 OR
                let pred = towers.iter().any(|u| {
                    let tg: i64 = rd(*u as *const Entity as *const u8, 0x68);
                    tg == 2 && u.attack_effect.is_some() && dist_sq(x, y, u.x, u.y) <= tower_reach(u, champ).pow(2)
                });
                let pred_alt = towers.iter().any(|u| {
                    let tg: i64 = rd(*u as *const Entity as *const u8, 0x68);
                    tg == 2 && u.attack_effect.is_some() && dist_sq(x, y, u.x, u.y) <= tower_reach_alt(u, champ).pow(2)
                });
                let v = if got == pred { n_match += 1; if pred != pred_alt { n_disc += 1; "MATCH(판별 15000)" } else { "MATCH" } } else { n_mis += 1; "**MISMATCH**" };
                println!("CTF\t#{}\t{}\tx={}\ty={}\tgame={}\tmine={}\talt(no15000)={}\t{}", k, nm, x, y, got, pred, pred_alt, v);
            }
        }
        println!("CTF_SUMMARY\tmatch={}\tmismatch={}\tdiscriminating={}", n_match, n_mis, n_disc);
    }
    // #22 — 2차: 타워 range/growth/level/sb_range + 챔프 radius_mult 주입 → 전체 식 판별
    {
        let (tids, champ_id): (Vec<usize>, usize) = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let x = (cache.towers(1, &pool).iter().map(|t| t.id).collect::<Vec<usize>>(), cache.player_champion[0][0].unwrap().id); x
        };
        for (k, tid) in tids.iter().enumerate() {
            let t = game.world.entity.get_mut(*tid).unwrap();
            if let Some(eff) = t.attack_effect.as_mut() { eff.range = 50000 + (k as u64) * 1000; eff.growth_range = 700; }
            t.level = 3;
            t.stat_buff_cached.range = 333;
            t.stat_buff_cached.radius_mult = 20;
        }
        { let c = game.world.entity.get_mut(champ_id).unwrap(); c.stat_buff_cached.radius_mult = 50; }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let champ: &Entity = cache.player_champion[0][0].unwrap();
        let towers = cache.towers(1, &pool);
        let mut n_match = 0; let mut n_mis = 0;
        for (k, t) in towers.iter().enumerate() {
            if t.attack_effect.is_none() { continue; }
            let eff = t.attack_effect.as_ref().unwrap();
            let R = tower_reach(t, champ);
            // 명세식 손계산(Effect::range·Entity::radius 를 안 부르고 필드로)
            let mine_range = eff.range + (t.level as u64 - 1) * eff.growth_range + t.stat_buff_cached.range as u64;
            let rad = |e: &Entity| -> u64 { if e.stat_buff_cached.radius_mult == 0 { e.radius as u64 } else { (e.radius as u64) * ((e.stat_buff_cached.radius_mult as i64 + 100) as u64) / 100 } };
            let R2 = mine_range + 15000 + rad(t) + rad(champ);
            println!("TOWER2	#{}	id={}	eff.range={}	growth={}	level={}	sb_range={}	t.radius={}	t.mult={}	c.radius={}	c.mult={}	R(api)={}	R(fields)={}	{}",
                     k, t.id, eff.range, eff.growth_range, t.level, t.stat_buff_cached.range, t.radius, t.stat_buff_cached.radius_mult, champ.radius, champ.stat_buff_cached.radius_mult, R, R2, if R == R2 { "same" } else { "**DIFF**" });
            for (nm, dx) in [("R-1", R - 1), ("R", R), ("R+1", R + 1)] {
                let x = t.x + dx; let y = t.y;
                let got = game_ai::can_tower_focused(&ctx, &cache, player, x, y);
                let pred = towers.iter().any(|u| {
                    let tg: i64 = rd(*u as *const Entity as *const u8, 0x68);
                    tg == 2 && u.attack_effect.is_some() && dist_sq(x, y, u.x, u.y) <= tower_reach(u, champ).pow(2)
                });
                let v = if got == pred { n_match += 1; "MATCH" } else { n_mis += 1; "**MISMATCH**" };
                println!("CTF2	#{}	{}	x={}	game={}	mine={}	{}", k, nm, x, got, pred, v);
            }
        }
        println!("CTF2_SUMMARY	match={}	mismatch={}", n_match, n_mis);
    }
    // #22 — nearest_enemy Some 경로: 타워 하나에 nearest_enemy=(0, champ.id) 를 써 넣고 먼 점에서도 true 인지
    {
        let (tid, champ_id, enemy_champ_id) = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let towers = cache.towers(1, &pool);
            (towers[0].id, cache.player_champion[0][0].unwrap().id, cache.player_champion[1][0].unwrap().id)
        };
        for (case, target_id) in [("me", champ_id), ("other", enemy_champ_id)] {
            {
                let t = game.world.entity.get_mut(tid).unwrap();
                let p = t as *mut Entity as *mut u8;
                wr::<i64>(p, 0x88, 1);            // nearest_enemy = Some
                wr::<usize>(p, 0x90, 0);          // .0 (미상 필드) = 0
                wr::<usize>(p, 0x98, target_id);  // .1 = id
            }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
            let champ: &Entity = cache.player_champion[0][0].unwrap();
            let t = cache.towers(1, &pool).into_iter().find(|u| u.id == tid).unwrap();
            let ne_tag: i64 = rd(t as *const Entity as *const u8, 0x88);
            let ne_id: usize = rd(t as *const Entity as *const u8, 0x98);
            let R = tower_reach(t, champ);
            for (nm, dx) in [("R", R), ("R+1", R + 1), ("far", 5_000_000u64)] {
                let x = t.x.saturating_sub(dx); let y = t.y;  // 반대 방향(다른 타워 간섭 최소화 시도)
                let got = game_ai::can_tower_focused(&ctx, &cache, player, x, y);
                let others = cache.towers(1, &pool).into_iter().filter(|u| u.id != tid).any(|u| {
                    let tg: i64 = rd(u as *const Entity as *const u8, 0x68);
                    tg == 2 && u.attack_effect.is_some() && dist_sq(x, y, u.x, u.y) <= tower_reach(u, champ).pow(2)
                });
                let pred = if case == "me" { true } else { dist_sq(x, y, t.x, t.y) <= R * R } || others;
                println!("CTF_NE\tcase={}\tne_tag={}\tne_id={}\t{}\tx={}\tgame={}\tmine={}\tothers={}\t{}", case, ne_tag, ne_id, nm, x, got, pred, others, if got == pred { "MATCH" } else { "**MISMATCH**" });
            }
        }
        // 원복
        {
            let t = game.world.entity.get_mut(tid).unwrap();
            let p = t as *mut Entity as *mut u8;
            wr::<i64>(p, 0x88, 0);
        }
    }
    // #22 — tower_attack_disable_tick 게이트: 0 으로 두면 전부 false
    {
        let mut s2 = real_setting();
        s2.tower_attack_disable_tick = 0;
        let ctx2 = GameContext {
            pool: &pool, setting: &s2, macro_weights: &mw, map_setting: &ms,
            map: &map, champion_list: &champs, item_list: &items,
            ignore_minion: false, debug: false,
            tutorial: TutorialType::None, trace_level: TraceLevel::Off,
        };
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx2);
        let player: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let t = cache.towers(1, &pool)[0];
        let got = game_ai::can_tower_focused(&ctx2, &cache, player, t.x, t.y);
        println!("CTF_GATE\tdisable_tick=0\ttick={}\tat_tower\tgame={}\tmine=false\t{}", game.tick(), got, if !got { "MATCH" } else { "**MISMATCH**" });
    }

    // ───────────── #20 v27_active_objective_discipline ─────────────
    {
        use game_ai::plan_legacy::team_plan::TeamPlan;
        let enemy_id = {
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[1][0].unwrap().id
        };
        // epic.live_list 에 적 챔피언 id 를 넣어 '오브젝트 엔티티' 로 쓴다(함수는 타입을 안 본다)
        println!("EPIC_LIVE\tbefore={}\tserpen_before={}", game.mode.jungle_runner.epic.live_list.len(), game.mode.jungle_runner.serpen.live_list.len());
        let mk = |wait: (u64, u64), until: usize, target: i8, kind: i8| -> TeamPlan {
            let mut tp: TeamPlan = Default::default();
            let p = &mut tp as *mut TeamPlan as *mut u8;
            wr::<u64>(p, 0x130, wait.0);
            wr::<u64>(p, 0x138, wait.1);
            wr::<usize>(p, 0x140, until);
            wr::<i8>(p, 0x148, target);
            wr::<i8>(p, 0x149, kind);
            tp
        };
        let tick = game.tick();
        let cases: Vec<(&str, (u64, u64), usize, i8, i8, JungleType, bool)> = vec![
            ("kind=2(None)",        (1, 2), tick + 10, 4, 2, JungleType::Morgard, false),
            ("target mismatch",     (1, 2), tick + 10, 4, 0, JungleType::Serpen,  false),
            ("until==tick",         (1, 2), tick,      4, 0, JungleType::Morgard, false),
            ("until=tick+1",        (1, 2), tick + 1,  4, 0, JungleType::Morgard, true),
            ("kind=1 HardDisengage",(7, 8), tick + 5,  5, 1, JungleType::Serpen,  true),
            ("Rhino target",        (3, 4), tick + 5,  0, 0, JungleType::Rhino,   true),
        ];
        for (nm, wait, until, tgt, kind, arg, exp) in cases.iter() {
            let tp = mk(*wait, *until, *tgt, *kind);
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let r = tp.v27_active_objective_discipline(2, &data, *arg);
            let got = r.is_some();
            let mut same = true;
            if let Some(st) = r.as_ref() {
                let p = st as *const _ as *const u8;
                let w0: u64 = rd(p, 0); let w1: u64 = rd(p, 8); let ut: usize = rd(p, 0x10); let tg: i8 = rd(p, 0x18); let kd: i8 = rd(p, 0x19);
                same = (w0, w1, ut, tg, kd) == (wait.0, wait.1, *until, *tgt, *kind);
                println!("V27_PAYLOAD\t{}\twait=({},{})\tuntil={}\ttarget={}\tkind={}\tsame_as_stored={}", nm, w0, w1, ut, tg, kd, same);
            }
            println!("V27\t{}\tgame={}\tmine={}\t{}", nm, got, exp, if got == *exp && same { "MATCH" } else { "**MISMATCH**" });
        }
        // 오브젝트 HP 경계: live_list 에 적 챔피언을 넣고 hp 를 35%/36% 로
        game.mode.jungle_runner.epic.live_list.push(enemy_id);
        for pct in [350usize, 359, 360, 361, 1000] {
            {
                let e = game.world.entity.get_mut(enemy_id).unwrap();
                e.stat_cached.hp = 1000;
                e.hp = pct;
            }
            let tp = mk((1, 2), tick + 10, 4, 0);
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let (hp, mx) = { let e = cache.game.get_entity_by_id(enemy_id).unwrap(); (e.hp, e.stat_cached.hp) };
            let got = tp.v27_active_objective_discipline(2, &data, JungleType::Morgard).is_some();
            let exp = !(hp * 100 / mx < 36);
            println!("V27_HP\tpct={}\thp={}\tmax={}\tratio={}\tgame={}\tmine={}\t{}", pct, hp, mx, hp * 100 / mx, got, exp, if got == exp { "MATCH" } else { "**MISMATCH**" });
            // Serpen 인자면 epic 리스트는 안 보므로 Some 이어야 함(serpen 리스트 비어있음)
            let tp2 = mk((1, 2), tick + 10, 5, 0);
            let got2 = tp2.v27_active_objective_discipline(2, &data, JungleType::Serpen).is_some();
            println!("V27_HP_SERPEN\tpct={}\tgame={}\tmine=true\t{}", pct, got2, if got2 { "MATCH" } else { "**MISMATCH**" });
        }
        game.mode.jungle_runner.epic.live_list.clear();
    }
    println!("DONE\tsetting_ok={}", ok);
}
