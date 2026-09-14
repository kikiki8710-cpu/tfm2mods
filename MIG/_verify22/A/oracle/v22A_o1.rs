#![allow(unused, dead_code, non_snake_case)]
//! 22차 배치A 오라클 — r13 잎 6함수(112~117, 전부 pub)를 **명세 독립 재구현**(수법 ⓓ)으로 대조한다.
//!  한 프로세스 = 한 구조 케이스(argv). TLS 접촉 콜리(camp_pos CAMP_POS_MEMO · expected_damage_target)가 있는
//!  113/115 는 케이스당 프로세스 1개(TEMPLATE ③). 112/116/117/114 는 순수 계산이라 프로세스 안에서 소량 스윕.
//!  세계 = TEMPLATE mkgame(start_game 직후). 엔티티/캐시 변조는 수법 ⑦(raw 포인터 인자, `&Entity` 인자 금지).
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify22/A/oracle/v22A_o1.rs
//! 실행: v22A_o1.exe <fn> <args...>   (드라이버 = run22A.py)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_unaligned(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }

struct W<'a> { setting: &'a GameSetting, ctx: &'a GameContext<'a, 'a>, game: Game }

fn mkctx<'a>(setting: &'a GameSetting, ms: &'a MapSetting, map: &'a MapDef, pool: &'a bumpalo::Bump, mw: &'a MacroWeights,
             champs: &'a Vec<String>, items: &'a Vec<Box<dyn ItemInfo>>) -> GameContext<'a, 'a> {
    GameContext { pool, setting, macro_weights: mw, map_setting: ms, map, champion_list: champs, item_list: items,
                  ignore_minion: false, debug: false, tutorial: TutorialType::None, trace_level: TraceLevel::Off }
}

// ───────── 117 camp_idx ─────────
fn predict117(j: JungleType) -> Option<usize> {
    match j { JungleType::Rhino => Some(0), JungleType::Mushroom => Some(1), JungleType::Bee => Some(2), JungleType::Stump => Some(3), _ => None }
}
fn jt(i: i64) -> JungleType {
    match i { 0 => JungleType::Rhino, 1 => JungleType::Mushroom, 2 => JungleType::Stump, 3 => JungleType::Bee, 4 => JungleType::Morgard, _ => JungleType::Serpen }
}

// ───────── 112 nontarget_windup_perceived ─────────
fn predict112(player: &PlayerState, data: &OperationData, game: &Game, caster: *const Entity) -> (bool, String) {
    let team = player.info.team;
    let bb = &data.blackboard[1 - team];
    let c: &Entity = unsafe { &*caster };
    if !bb.is_recent_visible(game as &dyn AbstractGame, player, c) { return (false, "notvis".into()); }
    let cp = ep(caster);
    let ty: i64 = rd(cp, 0x68);
    let elapsed: usize = if ty == 13 {
        let st: i64 = rd(cp, 0x70);
        match st { 3 | 4 | 5 | 6 => rd::<usize>(cp, 0x78), _ => 0 }
    } else { 0 };
    let a = player.info.parameter.skill_avoid_base();
    let cid: usize = rd(cp, 0x5c0);
    let h = data.cache.player_by_champion_id(cid).map(|p| p.info.parameter.skill_hit_base()).unwrap_or(50);
    let tick = game.tick();
    let cast_start = tick.saturating_sub(elapsed);
    let seed = (player.info.id as u64) ^ ((cid as u64) << 20) ^ ((cast_start as u64) << 40);
    let react = skill_avoid_react_ticks(seed, a, h);
    (elapsed >= react, format!("el={} a={} h={} tick={} seed={:#x} react={}", elapsed, a, h, tick, seed, react))
}

// ───────── 113 v3_deadly_edge_cells ─────────
fn predict113(version: usize, player: &PlayerState, data: &OperationData, ctx: &GameContext) -> (u32, String) {
    if version < 2 { return (2, "ver".into()); }
    let team = player.info.team; let pos = player.info.position.as_index();
    let champ = match data.cache.player_champion[team][pos] { Some(e) => e, None => return (2, "nochamp".into()) };
    let c = data.cache; let e = 1 - team;
    let fixed = [c.top_tower[e], c.mid_tower[e], c.bottom_tower[e], c.top_tower2[e], c.mid_tower2[e], c.bottom_tower2[e]];
    let mut found: Option<(usize, usize, usize)> = None;
    for t in fixed.iter().flatten().copied().chain(c.twin_towers[e].iter().copied()) {
        if let Some(eff) = t.attack_effect.as_ref() {
            found = Some((eff.expected_damage_target(ctx, t as &dyn AbstractEntity, champ), t.attack_cooltime(), t.id));
            break;
        }
    }
    let (shot, cool, tid) = match found { Some(v) => v, None => return (2, "notower".into()) };
    let edge_ticks = std::cmp::max(32000 / std::cmp::max(champ.stat_cached.move_speed, 1), 1);
    let edge_dmg = shot.saturating_mul(edge_ticks) / std::cmp::max(cool, 1);
    let cells = ((edge_dmg.saturating_mul(30) / std::cmp::max(champ.hp, 1)) as u32).wrapping_add(2);
    (std::cmp::min(cells, 60), format!("shot={} cool={} tid={} ms={} hp={} edge_ticks={} edge_dmg={} cells_raw={}", shot, cool, tid, champ.stat_cached.move_speed, champ.hp, edge_ticks, edge_dmg, cells))
}

// ───────── 115 is_cleared ─────────
fn predict115(camp: JungleType, team: usize, player: &PlayerState, data: &OperationData, ctx: &GameContext, game: &Game,
              tp: &game_ai::plan_legacy::team_plan::TeamPlan, offset: usize) -> (Option<bool>, String) {
    let champ = match data.cache.player_champion[player.info.team][player.info.position.as_index()] { Some(e) => e, None => return (Some(false), "nochamp".into()) };
    let idx = match predict117(camp) { Some(i) => i, None => return (None, "camp_idx panic".into()) };
    let next = tp.next_respawn_tick[team][idx];
    let tick = game.tick();
    if !(tick < next) { return (Some(false), format!("alive tick={} next={}", tick, next)); }
    let (cx, cy) = ctx.map.camp_pos(camp, team == 0);
    let dist = game_core::utils::distance(champ.x, champ.y, cx, cy);
    if champ.stat_cached.move_speed == 0 { return (None, "div0 panic".into()); }
    let move_tick = dist / champ.stat_cached.move_speed as u64;
    let rhs = move_tick as usize + offset + tick + ctx.setting.tick_per_second;
    (Some(next > rhs), format!("tick={} next={} camp=({},{}) dist={} ms={} move_tick={} rhs={}", tick, next, cx, cy, dist, champ.stat_cached.move_speed, move_tick, rhs))
}

// ───────── 116 can_tower_focused_when_attack ─────────
fn predict116(ctx: &GameContext, cache: &AbstractGameWithCache, player: &PlayerState, tower: *const Entity) -> (bool, String) {
    let tick = cache.game.tick();
    if !(tick < ctx.setting.tower_attack_disable_tick) { return (false, "disabled".into()); }
    let team = player.info.team;
    let champ = match cache.player_champion[team][player.info.position.as_index()] { Some(e) => e, None => return (false, "nochamp".into()) };
    let tp = ep(tower);
    if rd::<i64>(tp, 0x68) != 2 { return (false, "nottower".into()); }
    let t: &Entity = unsafe { &*tower };
    let eff = t.attack_effect.as_ref().unwrap();
    let minions = || cache.top_minions[team].iter().copied().chain(cache.mid_minions[team].iter().copied()).chain(cache.bottom_minions[team].iter().copied());
    if rd::<i64>(tp, 0x88) == 1 {
        let id: usize = rd(tp, 0x98);
        if id == champ.id { return (true, "focused".into()); }
        let cnt = minions().filter(|m| eff.is_in_range(t, m)).count();
        (cnt < 2, format!("cnt={}", cnt))
    } else {
        let any = minions().any(|m| eff.is_in_range(t, m));
        (!any, format!("any={}", any))
    }
}

// ───────── 114 attack_summon_action ─────────
fn predict114(player: &PlayerState, data: &OperationData, game: &Game) -> (Vec<(u8, usize)>, String) {
    let team = player.info.team;
    let champ = data.cache.player_champion[team][player.info.position.as_index()].unwrap();
    let ms = champ.stat_cached.move_speed as u64;
    let mut out = Vec::new(); let mut log = String::new();
    for &e in data.cache.others[1 - team].iter() {
        if !data.can_target(game as &dyn AbstractGame, player, e) { log += "T"; continue; }
        if !e.is_visible_from(champ) { log += "V"; continue; }
        let d2 = champ.distance_sq(e);
        if d2 > 6400000000 { log += "D"; continue; }
        let r_of = |eff: &Effect| eff.range(champ) + ms * 30 + eff.range_adjust(champ, e) + champ.radius() as u64 + e.radius() as u64;
        if champ.can_attack() { if let Some(atk) = champ.attack_effect.as_ref() { let r = r_of(atk); if d2 <= r * r { out.push((15u8, e.id)); } log += &format!("a{} ", r); } }
        if let Some(sk) = champ.skill_effect.as_ref() { if champ.can_skill() && sk.target.check(champ, e) { let r = r_of(sk); if d2 <= r * r { out.push((16u8, e.id)); } log += &format!("s{} ", r); } }
        if let Some(sk2) = champ.skill2_effect().as_ref() { if champ.can_skill2() && sk2.target.check(champ, e) { let r = r_of(sk2); if d2 <= r * r { out.push((17u8, e.id)); } log += &format!("s2:{} ", r); } }
        log += &format!("d2={} ", d2);
    }
    (out, log)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let which = a.get(1).cloned().unwrap_or_default();
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = mkctx(&setting, &ms, &map, &pool, &mw, &champs, &items);

    if which == "117" {
        let j = jt(arg(&a, 2, 0));
        let mine = predict117(j);
        println!("117\tcase={}\tmine={:?}", arg(&a, 2, 0), mine);
        let g = game_ai::plan_legacy::team_plan::camp_idx(j);     // 4/5 는 여기서 패닉(기대)
        println!("117\tcase={}\tgame={}\tmine={:?}\tok={}", arg(&a, 2, 0), g, mine, Some(g) == mine);
        return;
    }

    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let tick = arg(&a, 2, 0) as usize;
    game.set_tick(tick);
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];

    if which == "112" {
        // argv: tick ty_tag as_tag lv mode t0 t1 ... ; player = team0 Mid, caster = team1 Mid 의 비트 복제
        let ty_tag = arg(&a, 3, 13); let as_tag = arg(&a, 4, 3); let lv = arg(&a, 5, -1);
        let player = game.get_player_by_position(0, Position::Mid).expect("player");
        let src = cache.player_champion[1][2].expect("enemy mid");
        if lv >= 0 { bb[1].last_visible[2] = lv as usize; }
        let data = OperationData::new(&cache, &ctx, &bb);
        // 캐스터 비트 복제(1728B) — 드롭하지 않는다
        let mut buf: Vec<u64> = vec![0u64; 1728 / 8];
        unsafe { std::ptr::copy_nonoverlapping(src as *const Entity as *const u8, buf.as_mut_ptr() as *mut u8, 1728); }
        let cp = buf.as_ptr() as *const Entity;
        wr(ep(cp), 0x68, ty_tag as i64);
        wr(ep(cp), 0x70, as_tag as i64);
        let mut times: Vec<i64> = a[6..].iter().filter_map(|s| s.parse().ok()).collect();
        if times.is_empty() { times = vec![0]; }
        // 상대 시간(react 기준) 지정: 값이 -1000 이하이면 react + (v+1000)
        let mut total = 0; let mut mism = 0;
        for tv in times {
            let mut t = tv;
            if tv <= -1000 {
                wr(ep(cp), 0x78, 0usize);
                let (_, info) = predict112(player, &data, &game, cp);
                let react: i64 = info.split("react=").nth(1).unwrap().parse().unwrap();
                t = react + (tv + 1000);
            }
            wr(ep(cp), 0x78, t as usize);
            let (mine, info) = predict112(player, &data, &game, cp);
            let g = game_ai::nontarget_windup_perceived(2, player, &data, unsafe { &*cp });
            total += 1; if g != mine { mism += 1; }
            println!("112\ttick={}\tty={}\tas={}\tlv={}\tt={}\tgame={}\tmine={}\tok={}\t{}", tick, ty_tag, as_tag, lv, t, g, mine, g == mine, info);
        }
        println!("112\tSUMMARY\ttotal={}\tmismatch={}", total, mism);
        std::mem::forget(buf);
        return;
    }

    if which == "113" {
        // argv: tick version hp ms nochamp(0/1) killtowers(0/1) tower_attack
        let version = arg(&a, 3, 2) as usize; let hp = arg(&a, 4, -1); let msv = arg(&a, 5, -1);
        let nochamp = arg(&a, 6, 0); let kill = arg(&a, 7, 0); let tatk = arg(&a, 8, -1);
        let player = game.get_player_by_position(0, Position::Mid).expect("player");
        let champ = cache.player_champion[0][2].expect("champ") as *const Entity;
        if hp >= 0 { wr(ep(champ), 0x670, hp as usize); }
        if msv >= 0 { wr(ep(champ), 0x640, msv as usize); }
        let perslot = arg(&a, 9, 0); let killmask = arg(&a, 10, 0);
        if tatk >= 0 {
            let slots = [cache.top_tower[1], cache.mid_tower[1], cache.bottom_tower[1], cache.top_tower2[1], cache.mid_tower2[1], cache.bottom_tower2[1]];
            let mut k = 0usize;
            for t in slots.iter().flatten() {
                // 타워 attack_effect.ty 를 TowerAttackEffect(damage=tatk(+슬롯별 가산), ratio=0) 로 교체(Arc 팻포인터 16B 덮어쓰기 · 옛 Arc 는 누수)
                let dmg = tatk as usize + if perslot == 1 { k * 1000 } else { 0 };
                let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
                let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
                wr(ep(*t as *const Entity), 0x490, raw[0]); wr(ep(*t as *const Entity), 0x498, raw[1]);
                if (killmask >> k) & 1 == 1 { wr(ep(*t as *const Entity), 0x4c0, -1i32); }
                k += 1;
            }
            let mut j = 0usize;
            for t in cache.twin_towers[1].iter() {
                let dmg = tatk as usize + if perslot == 1 { 10000 + j * 1000 } else { 0 };
                let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(dmg, 0));
                let raw: [usize; 2] = unsafe { std::mem::transmute(arc) };
                wr(ep(*t as *const Entity), 0x490, raw[0]); wr(ep(*t as *const Entity), 0x498, raw[1]);
                if (killmask >> (6 + j)) & 1 == 1 { wr(ep(*t as *const Entity), 0x4c0, -1i32); }
                j += 1;
            }
            println!("113\tslots ids={:?}\ttwin ids={:?}", slots.iter().map(|o| o.map(|e| e.id)).collect::<Vec<_>>(), cache.twin_towers[1].iter().map(|e| e.id).collect::<Vec<_>>());
        }
        if kill == 1 {
            for t in [cache.top_tower[1], cache.mid_tower[1], cache.bottom_tower[1], cache.top_tower2[1], cache.mid_tower2[1], cache.bottom_tower2[1]].iter().flatten() {
                wr(ep(*t as *const Entity), 0x4c0, -1i32);
            }
            for t in cache.twin_towers[1].iter() { wr(ep(*t as *const Entity), 0x4c0, -1i32); }
        }
        if nochamp == 1 { cache.player_champion[0][2] = None; }
        let data = OperationData::new(&cache, &ctx, &bb);
        let (mine, info) = predict113(version, player, &data, &ctx);
        let g = game_ai::v3_deadly_edge_cells(version, player, &data);
        println!("113\ttick={}\tver={}\thp={}\tms={}\tnochamp={}\tkill={}\ttatk={}\tgame={}\tmine={}\tok={}\t{}", tick, version, hp, msv, nochamp, kill, tatk, g, mine, g == mine, info);
        return;
    }

    if which == "115" {
        // argv: tick camp team next_mode(abs>=0 | -1=rhs -2=rhs+1 -3=rhs-1) ms offset who(0=team0 Mid,1=team1 Mid)
        let camp = jt(arg(&a, 3, 0)); let team = arg(&a, 4, 0) as usize; let nm = arg(&a, 5, -2);
        let msv = arg(&a, 6, -1); let offset = arg(&a, 7, 0) as usize; let who = arg(&a, 8, 0) as usize;
        let player = game.get_player_by_position(who, Position::Mid).expect("player");
        let champ = cache.player_champion[who][2].expect("champ") as *const Entity;
        if msv >= 0 { wr(ep(champ), 0x640, msv as usize); }
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
        let mut dbg: DebugFrameData = Default::default();
        let idx = predict117(camp).unwrap_or(0);
        if nm >= 0 { tp.next_respawn_tick[team][idx] = nm as usize; }
        else {
            tp.next_respawn_tick[team][idx] = usize::MAX;    // 일단 살아있지 않게 두고 rhs 를 계산
            let (_, info) = predict115(camp, team, player, &data, &ctx, &game, &tp, offset);
            let rhs: usize = info.split("rhs=").nth(1).map(|s| s.trim().parse().unwrap()).unwrap_or(0);
            tp.next_respawn_tick[team][idx] = match nm { -1 => rhs, -2 => rhs + 1, _ => rhs.saturating_sub(1) };
        }
        let (mine, info) = predict115(camp, team, player, &data, &ctx, &game, &tp, offset);
        println!("115\tpre\tmine={:?}\t{}", mine, info);
        let g = game_ai::plan_legacy::old::is_cleared(camp, team, 2, &mut rnd, player, &data, &tp, offset, &mut dbg);
        println!("115\ttick={}\tcamp={}\tteam={}\tnm={}\tms={}\toff={}\twho={}\tgame={}\tmine={:?}\tok={}\t{}", tick, arg(&a, 3, 0), team, nm, msv, offset, who, g, mine, Some(g) == mine, info);
        return;
    }

    if which == "116" {
        // argv: tick nearest(-1=None, 0=me, 1=other) fake_in(0..3) fake_out(0..3) nochamp(0/1) nottower(0/1)
        let near = arg(&a, 3, -1); let fin = arg(&a, 4, 0) as usize; let fout = arg(&a, 5, 0) as usize;
        let nochamp = arg(&a, 6, 0); let nott = arg(&a, 7, 0);
        let player = game.get_player_by_position(0, Position::Mid).expect("player");
        let champ = cache.player_champion[0][2].expect("champ");
        let tower = cache.mid_tower[1].expect("enemy mid tower") as *const Entity;
        let tp = ep(tower);
        if near < 0 { wr(tp, 0x88, 0i64); } else { wr(tp, 0x88, 1i64); wr(tp, 0x98, if near == 0 { champ.id } else { champ.id + 1000 }); }
        // 가짜 미니언 = 내 팀 챔피언들(Top·Jungle·Bottom·Support) — 사거리 안/밖으로 배치
        let t: &Entity = unsafe { &*tower };
        let pool_e: Vec<&Entity> = [0usize, 1, 3, 4].iter().map(|&p| cache.player_champion[0][p].expect("ally")).collect();
        let mut k = 0;
        for i in 0..fin { let e = pool_e[k]; k += 1; wr(ep(e as *const Entity), 0x660, t.x); wr(ep(e as *const Entity), 0x668, t.y + 1000); cache.top_minions[0].push(e); }
        for i in 0..fout { let e = pool_e[k]; k += 1; wr(ep(e as *const Entity), 0x660, setting.width / 2); wr(ep(e as *const Entity), 0x668, setting.height / 2); cache.bottom_minions[0].push(e); }
        if nochamp == 1 { cache.player_champion[0][2] = None; }
        let towerarg: *const Entity = if nott == 1 { cache.player_champion[1][2].expect("enemy") as *const Entity } else { tower };
        let (mine, info) = predict116(&ctx, &cache, player, towerarg);
        let g = game_ai::can_tower_focused_when_attack(&ctx, &cache, player, unsafe { &*towerarg });
        let inr: Vec<bool> = pool_e.iter().map(|e| t.attack_effect.as_ref().unwrap().is_in_range(t, e)).collect();
        println!("116\ttick={}\tnear={}\tfin={}\tfout={}\tnochamp={}\tnott={}\tgame={}\tmine={}\tok={}\t{}\tin_range={:?}\tdisable={}", tick, near, fin, fout, nochamp, nott, g, mine, g == mine, info, inr, setting.tower_attack_disable_tick);
        return;
    }

    if which == "114" {
        // argv: tick n_others dist vis(0/1) atk_range level skill_range
        let n = arg(&a, 3, 1) as usize; let dist = arg(&a, 4, 0) as u64; let vis = arg(&a, 5, 1);
        let atk_r = arg(&a, 6, -1); let level = arg(&a, 7, -1); let sk_r = arg(&a, 8, -1);
        let player = game.get_player_by_position(0, Position::Mid).expect("player");
        let champ = cache.player_champion[0][2].expect("champ") as *const Entity;
        let c: &Entity = unsafe { &*champ };
        if atk_r >= 0 { wr(ep(champ), 0x4a0, atk_r as u64); wr(ep(champ), 0x4a8, 0u64); }
        if sk_r >= 0 { wr(ep(champ), 0x4d8, sk_r as u64); wr(ep(champ), 0x4e0, 0u64); wr(ep(champ), 0x510, sk_r as u64); wr(ep(champ), 0x518, 0u64); }
        if level >= 0 { wr(ep(champ), 0x5c8, level as usize); }
        let pool_e: Vec<&Entity> = [0usize, 1, 3, 4].iter().map(|&p| cache.player_champion[1][p].expect("enemy")).collect();
        for i in 0..n.min(4) {
            let e = pool_e[i];
            wr(ep(e as *const Entity), 0x660, c.x + dist); wr(ep(e as *const Entity), 0x668, c.y);
            wr(ep(e as *const Entity), 0x38, if vis == 1 { 0i64 } else { 2i64 });   // visible_state[team0]
            cache.others[1].push(e);
        }
        let data = OperationData::new(&cache, &ctx, &bb);
        let (mine, info) = predict114(player, &data, &game);
        let g = game_ai::attack_summon_action(player, &data);
        let gp = &g as *const _ as *const u8;
        let ptr: *const u8 = rd(gp, 0); let cap: usize = rd(gp, 16); let len: usize = rd(gp, 24);
        let mut got: Vec<(u8, usize)> = Vec::new();
        for i in 0..len { let el = unsafe { ptr.add(i * 184) }; got.push((rd::<u8>(el, 0xb1), rd::<usize>(el, 0x8))); }
        // 페이로드 24B 대조: SmallActionAttack::new 의 결과와 비교(태그 15 만)
        let mut pay_ok = true;
        for i in 0..len {
            let el = unsafe { ptr.add(i * 184) };
            let tag: u8 = rd(el, 0xb1);
            let tid: usize = got[i].1;
            let mut ref24 = [0u8; 24];
            if tag == 15 { let s = game_ai::SmallActionAttack::new(&data, tid); unsafe { std::ptr::copy_nonoverlapping(&s as *const _ as *const u8, ref24.as_mut_ptr(), 24) }; std::mem::forget(s); }
            else if tag == 16 { let s = game_ai::SmallActionSkill::new(&data, tid); unsafe { std::ptr::copy_nonoverlapping(&s as *const _ as *const u8, ref24.as_mut_ptr(), 24) }; std::mem::forget(s); }
            else { let s = game_ai::SmallActionSkill2::new(&data, tid); unsafe { std::ptr::copy_nonoverlapping(&s as *const _ as *const u8, ref24.as_mut_ptr(), 24) }; std::mem::forget(s); }
            let mut cur = [0u8; 24]; unsafe { std::ptr::copy_nonoverlapping(el, cur.as_mut_ptr(), 24) };
            // 살아있는 바이트 = start_tick(8)+target(8)+is_act(1) = 17B. +0x11..+0x18 은 패딩(값 비결정)
            if cur[..17] != ref24[..17] { pay_ok = false; println!("114\tpayload mismatch i={} cur={:?} ref={:?}", i, cur, ref24); }
            println!("114\tel[{}] tag={} live17={:?} pad7={:?} start_tick={} target={} is_act={}", i, tag, &cur[..17], &cur[17..], rd::<usize>(el, 0), rd::<usize>(el, 8), rd::<u8>(el, 16));
        }
        let mine_ids: Vec<(u8, usize)> = mine.clone();
        println!("114\ttick={}\tn={}\tdist={}\tvis={}\tatk_r={}\tlevel={}\tsk_r={}\tgame={:?}\tmine={:?}\tok={}\tlen={}\tcap={}\tptr_dangling8={}\tpay_ok={}\t{}", tick, n, dist, vis, atk_r, level, sk_r, got, mine_ids, got == mine_ids, len, cap, ptr as usize == 8, pay_ok, info);
        std::mem::forget(g);
        return;
    }
    println!("unknown fn {}", which);
}
