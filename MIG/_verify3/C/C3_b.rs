#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 C — probe B
//  ★GameSetting::default() 의 width/height 가 0 이라 is_top_side 가 늘 true 였다.
//    실전값(bundle_unpacked_full/setting/game_setting.game_setting) = width/height 960000, champion_radius 10000.
//  (1) 13/14 미검증 분기 개방: 타워 전멸 / 2차타워 / nearest_enemy 스왑 / Mid 봇사이드
//  (2) 14 TargetMissing 취소 경로 도달 가능성 (v30 == 도착 부시 인가)
//  (3) 14 hp_ratio<41 임계 확인
//  (4) can_enemy_hit_objective 하드 상한(19_600_000_000 = 140000^2) 실측
// ★init_tower/init_nexus 재호출 없음
use game_core::*;
use game_ai::plan_legacy::old::{LineGankerPlan, LineGankCoverPlan, LineGankerPhase, can_enemy_hit_objective};
use game_ai::GoalData;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
    setting.width = 960000;
    setting.height = 960000;
    setting.champion_radius = 10000;
    let mw: MacroWeights = Default::default();
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = Game::new(1234u64, false, &setting, &ms, &map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let pn = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    println!("SANITY\ttower_ids={}\tnexus={}\tw={}\th={}",
        game.world.tower_ids.len(), game.world.nexus_ids.len(), setting.width, setting.height);

    // bush id -> 대표 셀 (cy,cx)
    let mut cell_of: Vec<Option<(usize, usize)>> = vec![None; 40];
    for cy in 0..30usize { for cx in 0..30usize {
        let b = map.bushes[cy][cx];
        if b < 40 && cell_of[b].is_none() { cell_of[b] = Some((cy, cx)); }
    }}
    let present: Vec<usize> = (0..40).filter(|i| cell_of[*i].is_some()).collect();
    println!("bushids\tpresent={:?}", present);

    // 챔프 엔티티 id 확보 + 자연상태 lead
    let (jid0, jid1) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] {
                println!("champ\tt{}\t{}\tid={}\tx={}\ty={}\thp={}/{}\tis_top_side={}",
                    t, pn[p], e.id, e.x, e.y, e.hp, e.stat_cached.hp, map_regions::is_top_side(&ctx, e.x, e.y));
            }
        }}
        println!("lead\ttop={:?}\tmid={:?}\tbottom={:?}\trp={:?}",
            cache.top_lead, cache.mid_lead, cache.bottom_lead, cache.region_point);
        (cache.player_champion[0][1].unwrap().id, cache.player_champion[1][1].unwrap().id)
    };
    let jids = [jid0, jid1];

    // 라인 타워 엔티티 id (line, team)
    let mut tw: [[Option<usize>; 2]; 3] = [[None; 2]; 3];
    let mut tw2: [[Option<usize>; 2]; 3] = [[None; 2]; 3];
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        for t in 0..2usize {
            tw[0][t] = cache.top_tower[t].map(|e| e.id);
            tw[1][t] = cache.mid_tower[t].map(|e| e.id);
            tw[2][t] = cache.bottom_tower[t].map(|e| e.id);
            tw2[0][t] = cache.top_tower2[t].map(|e| e.id);
            tw2[1][t] = cache.mid_tower2[t].map(|e| e.id);
            tw2[2][t] = cache.bottom_tower2[t].map(|e| e.id);
        }
        println!("towers\ttw={:?}\ttw2={:?}", tw, tw2);
    }

    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top", "Mid", "Bottom"];
    let gd: GoalData = Default::default();
    let ps_score: PositioningScoreData = Default::default();
    let mut dbg: DebugFrameData = Default::default();

    // ---------- (1) v30 진리표 : cover.sub_plan 이 bush 를 그대로 노출한다 ----------
    for li in 0..3usize {
     for t in 0..2usize {
      for tstate in 0..3usize {
       for ne in [false, true] {
        for side in 0..2usize {
            let (bx, by) = if side == 0 { (15000u64, 913000u64) } else { (600000u64, 600000u64) };
            { let e = game.world.entity.get_mut(jids[t]).unwrap(); e.x = bx; e.y = by; }
            for l2 in 0..3usize { for t2 in 0..2usize {
                for id in [tw[l2][t2], tw2[l2][t2]] {
                    if let Some(id) = id { if let Some(e) = game.world.entity.get_mut(id) {
                        if let EntityType::Tower { info: ref mut ti } = e.ty { ti.nearest_enemy = None; }
                    }}
                }
            }}
            if ne { if let Some(id) = tw[li][t] { if let Some(e) = game.world.entity.get_mut(id) {
                if let EntityType::Tower { info: ref mut ti } = e.ty { ti.nearest_enemy = Some((0usize, 0usize)); }
            }}}
            let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            if tstate >= 1 {
                match li { 0 => cache.top_tower[t] = None, 1 => cache.mid_tower[t] = None, _ => cache.bottom_tower[t] = None };
            }
            if tstate >= 2 {
                match li { 0 => cache.top_tower2[t] = None, 1 => cache.mid_tower2[t] = None, _ => cache.bottom_tower2[t] = None };
            }
            let twdesc = match li {
                0 => cache.top_tower[t].or(cache.top_tower2[t]),
                1 => cache.mid_tower[t].or(cache.mid_tower2[t]),
                _ => cache.bottom_tower[t].or(cache.bottom_tower2[t]),
            }.map(|e| if let EntityType::Tower { info: ref i } = e.ty { format!("{:?}/ne={}", i.ty, i.nearest_enemy.is_some()) } else { "notTower".to_string() })
             .unwrap_or("None".to_string());
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
            let cv = LineGankCoverPlan::new(lines[li], 100);
            let r2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                format!("{:?}", cv.sub_plan(3, &mut rand::rngs::StdRng::seed_from_u64(5), ps, &data, &mut dbg))
            })).unwrap_or_else(|_| "PANIC".to_string());
            let gk = LineGankerPlan::new(lines[li], 100, 100);
            let r1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                format!("{:?}", gk.sub_plan(3, &mut rand::rngs::StdRng::seed_from_u64(5), ps, &data, &mut dbg))
            })).unwrap_or_else(|_| "PANIC".to_string());
            println!("v30\tline={}\tteam={}\ttstate={}\tne={}\tside={}\ttop_side={}\ttower={}\tCOVERv30={}\tGANKERv41={}",
                ln[li], t, tstate, ne, side, map_regions::is_top_side(&ctx, bx, by), twdesc, r2, r1);
        }}}}}

    { let e = game.world.entity.get_mut(jid0).unwrap(); e.x = 15000; e.y = 913000; }
    { let e = game.world.entity.get_mut(jid1).unwrap(); e.x = 913000; e.y = 15000; }
    for l2 in 0..3usize { for t2 in 0..2usize {
        for id in [tw[l2][t2], tw2[l2][t2]] {
            if let Some(id) = id { if let Some(e) = game.world.entity.get_mut(id) {
                if let EntityType::Tower { info: ref mut ti } = e.ty { ti.nearest_enemy = None; }
            }}
        }
    }}

    // ---------- (2) TargetMissing 도달성 : Top/Bottom 라인은 v30 이 챔프좌표와 무관 ----------
    for &li in [0usize, 2usize].iter() {
     for t in 0..2usize {
      for &bid in present.iter() {
        let (cy, cx) = cell_of[bid].unwrap();
        let (px, py) = (cx as u64 * 32000 + 16000, cy as u64 * 32000 + 16000);
        { let e = game.world.entity.get_mut(jids[t]).unwrap(); e.x = px; e.y = py; }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
        let mut gk = LineGankerPlan::new(lines[li], 100, 100);
        let mut r = rand::rngs::StdRng::seed_from_u64(5);
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            gk.update(3, &mut r, ps, &data, &gd, &ps_score, &mut dbg);
        })).is_ok();
        let s = format!("{:?}", gk);
        if s.contains("Cancel") || !ok {
            println!("arrive\tline={}\tteam={}\tchamp_bush={}\tcell=({},{})\tok={}\tafter={}", ln[li], t, bid, cy, cx, ok, s);
        }
      }
     }
    }
    { let e = game.world.entity.get_mut(jid0).unwrap(); e.x = 15000; e.y = 913000; }
    { let e = game.world.entity.get_mut(jid1).unwrap(); e.x = 913000; e.y = 15000; }

    // ---------- (3) hp_ratio<41 임계 ----------
    {
        let maxhp = { let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            cache.player_champion[0][1].unwrap().stat_cached.hp };
        for &pct in [38usize, 39, 40, 41, 42, 45].iter() {
            let hp = maxhp * pct / 100;
            { let e = game.world.entity.get_mut(jid0).unwrap(); e.hp = hp; }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
            let mut gk = LineGankerPlan::new(LineType::Top, 100, 100);
            let mut r = rand::rngs::StdRng::seed_from_u64(5);
            let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                gk.update(3, &mut r, ps, &data, &gd, &ps_score, &mut dbg);
            })).is_ok();
            println!("hpgate\tmaxhp={}\tpct={}\thp={}\tratio={}\tok={}\tafter={:?}", maxhp, pct, hp, hp * 100 / maxhp, ok, gk);
        }
        { let e = game.world.entity.get_mut(jid0).unwrap(); e.hp = maxhp; }
    }

    // ---------- (4) can_enemy_hit_objective 하드 상한 ----------
    {
        let base = { let e = game.world.entity.get(jid1).unwrap(); (e.x, e.y) };
        let mut probe = |g: &mut Game, d: u64| -> bool {
            { let e = g.world.entity.get_mut(jid0).unwrap(); e.x = base.0.wrapping_add(d); e.y = base.1; }
            let ea = g.world.entity.get(jid0).unwrap();
            let eb = g.world.entity.get(jid1).unwrap();
            can_enemy_hit_objective(ea, eb, u64::MAX / 4)
        };
        let mut lo = 0u64; let mut hi = 1_000_000u64;
        while lo + 1 < hi { let mid = lo + (hi - lo) / 2; if probe(&mut game, mid) { lo = mid } else { hi = mid } }
        println!("cap\tmax_true_dx={}\tfirst_false_dx={}\tlo_sq={}", lo, hi, (lo as u128) * (lo as u128));
        for &d in [139999u64, 140000, 140001].iter() {
            println!("cap_chk\td={}\tres={}", d, probe(&mut game, d));
        }
    }
}
