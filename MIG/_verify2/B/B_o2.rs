#![allow(unused, dead_code, non_snake_case)]
// 배치 B 2차 반증검증 — 오라클 2: 09 check_favorable_engage_formation 전수 진리표
// cache.player_champion 이 pub 필드라 Entity 를 복제해 좌표/HP 를 완전히 통제한다.
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

fn dsq(ax: u64, ay: u64, bx: u64, by: u64) -> u64 {
    let dx = ax.abs_diff(bx);
    let dy = ay.abs_diff(by);
    dx * dx + dy * dy
}

#[derive(Clone, Copy, Debug)]
struct A { x: u64, y: u64, hp: u64, mx: u64, alive: bool }

// 명세(specs20 #09 logic)를 그대로 옮긴 재현
fn mine(team: usize, pos: usize, allies: &[A; 5], ex: u64, ey: u64,
        fount: (u64, u64, u64, u64), engage_range: u64) -> (bool, i32, i32, i32) {
    let ebx = (fount.0 + fount.2) / 2;
    let eby = (fount.1 + fount.3) / 2;
    let rdx = ebx as i128 - ex as i128;
    let rdy = eby as i128 - ey as i128;
    let rlen = rdx * rdx + rdy * rdy;
    if rlen < 1 { return (true, -1, -1, -1); }
    let e2b = dsq(ex, ey, ebx, eby);
    let maxd = engage_range + 100000;
    let (mut front, mut flank, mut rear) = (0i32, 0i32, 0i32);
    for ap in 0..5usize {
        if !allies[ap].alive { continue; }
        if ap == pos { continue; }                       // ally.id == champ.id
        let a = allies[ap];
        if a.hp * 100 / a.mx < 40 { continue; }
        if dsq(a.x, a.y, ex, ey) > maxd * maxd { continue; }
        let adx = ex as i128 - a.x as i128;
        let ady = ey as i128 - a.y as i128;
        let alen = adx * adx + ady * ady;
        if alen < 1 { front += 1; continue; }
        let dot = adx * rdx + ady * rdy;
        let cross = adx * rdy - ady * rdx;
        let lp = alen * rlen;
        let dq = dot * dot;
        let cq = cross * cross;
        let is_front = dot > 0 && dq * 4 > lp;
        let is_rear = dot < 0 && dq * 100 > lp * 9;
        let is_flank = cq * 100 > lp * 9;
        if is_front { front += 1; }
        else if is_rear {
            if dsq(a.x, a.y, ebx, eby) < e2b { rear += 1; } else { flank += 1; }
        } else if is_flank { flank += 1; }
        else { front += 1; }
    }
    if rear > 0 { return (true, front, flank, rear); }
    if flank > 1 || (flank > 0 && front > 0) { return (true, front, flank, rear); }
    if front > 1 {
        let c2b = dsq(allies[pos].x, allies[pos].y, ebx, eby);
        return (c2b as u128 * 5 <= e2b as u128 * 6, front, flank, rear);
    }
    (false, front, flank, rear)
}

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    game.init_tower(&ctx);
    game.init_nexus(&setting, &map);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];

    // 기준 엔티티 6개 복제 (팀0 5명 + 타깃 1)
    let base: Vec<Entity> = {
        let c0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut v: Vec<Entity> = Vec::new();
        for p in 0..5usize { v.push(c0.player_champion[0][p].unwrap().clone()); }
        v.push(c0.player_champion[1][0].unwrap().clone());
        v
    };
    let team = 0usize; let pos = 0usize;
    let fount = map.fountains[1];
    println!("# fount_enemy\t{:?}\tcenter=({},{})", fount, (fount.0 + fount.2) / 2, (fount.1 + fount.3) / 2);

    // 결정적 의사난수
    let mut s: u64 = 0x1234_5678_9abc_def0;
    let mut nxt = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };

    let ex_base = 500000u64; let ey_base = 500000u64;
    let mut ok = 0; let mut bad = 0; let mut dmgnz = 0;
    let mut cov = std::collections::BTreeMap::<String, usize>::new();
    let mut vdiff = 0usize;
    for it in 0..400 {
        let ex = ex_base + (nxt() % 200000) - 100000;
        let ey = ey_base + (nxt() % 200000) - 100000;
        let er = if it % 4 == 0 { 200000u64 } else if it % 4 == 1 { 100000 } else if it % 4 == 2 { 400000 } else { 50000 };
        let mut aa = [A { x: 0, y: 0, hp: 100, mx: 100, alive: true }; 5];
        for k in 0..5usize {
            let r = nxt();
            aa[k].x = (ex as i64 + ((r % 700001) as i64 - 350000)).max(1) as u64;
            let r2 = nxt();
            aa[k].y = (ey as i64 + ((r2 % 700001) as i64 - 350000)).max(1) as u64;
            aa[k].mx = 100;
            aa[k].hp = match nxt() % 6 { 0 => 39, 1 => 40, 2 => 41, 3 => 100, 4 => 20, _ => 80 };
            aa[k].alive = nxt() % 10 != 0;
        }
        aa[pos].alive = true;                     // champ 은 반드시 존재(unwrap)
        if aa[pos].hp == 0 { aa[pos].hp = 100; }

        let mut ents = base.clone();
        for k in 0..5usize { ents[k].x = aa[k].x; ents[k].y = aa[k].y; ents[k].hp = aa[k].hp as usize; ents[k].stat_cached.hp = aa[k].mx as usize; }
        ents[5].x = ex; ents[5].y = ey; ents[5].hp = 100usize; ents[5].stat_cached.hp = 100usize;

        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for k in 0..5usize { cache.player_champion[0][k] = if aa[k].alive { Some(&ents[k]) } else { None }; }
        cache.player_champion[1][0] = Some(&ents[5]);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(team, poss[pos]).unwrap();

        // 미니언 게이트가 정말 0 인지 같이 잰다 (09 의 1208~1209)
        let dmg = game_ai::enemy_minion_line_action_danger_damage_at(
            3, &data, cache.player_champion[0][pos].unwrap(), ex, ey, setting.tick_per_second * 2, true, false);
        if dmg != 0 { dmgnz += 1; }

        let got = game_ai::check_favorable_engage_formation(3, ps, &data, &ents[5], er);
        for v in [0usize, 1, 2, 3, 10, 20, 30, 40, 46, 50, 54, 60] {
            if game_ai::check_favorable_engage_formation(v, ps, &data, &ents[5], er) != got { vdiff += 1; }
        }
        let (m, f, fl, r) = mine(team, pos, &aa, ex, ey, fount, er);
        let key = if r < 0 { "zero_retreat".to_string() }
            else if r > 0 { "rear>0".to_string() }
            else if fl > 1 { "flank>1".to_string() }
            else if fl > 0 && f > 0 { "flank&front".to_string() }
            else if f > 1 { format!("front>1:{}", m) }
            else { format!("none:{}f{}fl{}", f, fl, m) };
        *cov.entry(key).or_insert(0) += 1;
        if got == m { ok += 1; } else {
            bad += 1;
            if bad <= 12 {
                println!("MISMATCH\tit={}\ter={}\tE=({},{})\tgame={}\tmine={}\tfront={} flank={} rear={}\tallies={:?}",
                    it, er, ex, ey, got, m, f, fl, r, aa);
            }
        }
    }
    println!("RESULT\tok={}\tbad={}\tdmg_nonzero={}\tversion_diff={}", ok, bad, dmgnz, vdiff);
    for (k, v) in &cov { println!("COV\t{}\t{}", k, v); }
}
