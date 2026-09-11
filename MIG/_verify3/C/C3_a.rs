#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 C — probe A
//  (1) *_lead / region_point / MapDef::lane_seq 실측  (spec14 open[5], knobs)
//  (2) can_enemy_hit_objective 의 3번째 인자(25000) 단위 규명  (spec10 open[0])
//  (3) is_enemy_well_danger 진리표 (spec10 open[1])
// ★init_tower/init_nexus 재호출 없음 (BRIEF §3① 실측 함정)
use game_core::*;
use game_ai::plan_legacy::old::{can_enemy_hit_objective, is_ignored_well_enemy};
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;
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
    let pn = ["Top","Jungle","Mid","Bottom","Support"];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);
    println!("SANITY\ttower_ids={}\tnexus_ids={}\tchampion_ids={}\twidth={}\theight={}",
        game.world.tower_ids.len(), game.world.nexus_ids.len(), game.world.champion_ids.len(),
        setting.width, setting.height);

    // ---------- (1) lane_seq ----------
    let lines = [LineType::Top, LineType::Mid, LineType::Bottom];
    let ln = ["Top","Mid","Bottom"];
    for li in 0..3 { for t in 0..2usize {
        println!("lane_seq\tline={}\tteam={}\tseq={:?}", ln[li], t, map.lane_seq(lines[li], t));
    }}

    // ---------- (1) region_point / lead : 틱을 굴려가며 ----------
    for &tk in [0usize, 60, 600, 1800, 3600, 7200].iter() {
        while game.world.tick < tk {
            let mut ff: Option<&mut GameFrameData> = None;
            game.run_tick(&ctx, &mut rnd, &mut ff);
        }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        println!("lead\ttick={}\ttop={:?}\tmid={:?}\tbottom={:?}\trp_nonzero={}\trp={:?}",
            game.world.tick, cache.top_lead, cache.mid_lead, cache.bottom_lead,
            cache.region_point.iter().filter(|v| **v != 0).count(), cache.region_point);
    }

    // ---------- (2) can_enemy_hit_objective ----------
    // 엔티티 id 확보
    let (eid, oid, tid2) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let e = cache.player_champion[1][0].unwrap();
        let o = cache.player_champion[0][0].unwrap();
        let tw = cache.top_tower[0].unwrap();
        println!("ENT\tenemy_champ id={} x={} y={} radius={} nontarget_avoid={}", e.id, e.x, e.y, e.radius, e.nontarget_avoid_range);
        println!("ENT\tally_champ  id={} x={} y={} radius={}", o.id, o.x, o.y, o.radius);
        println!("ENT\ttower0      id={} x={} y={} radius={} ty={:?}", tw.id, tw.x, tw.y, tw.radius, tw.ty);
        (e.id, o.id, tw.id)
    };

    // 사거리 있는 엔티티(타워)를 시전자로도 시험한다
    let margin_probe = |game: &Game, a: usize, b: usize| -> Option<u64> {
        // 최소 margin 이분탐색: false..true 경계
        let f = |m: u64| -> bool {
            let ea = game.world.entity.get(a).unwrap();
            let eb = game.world.entity.get(b).unwrap();
            can_enemy_hit_objective(ea, eb, m)
        };
        if f(0) { return Some(0); }
        let mut hi: u64 = 1;
        let mut cnt = 0;
        while !f(hi) { if hi > (1u64<<62) { return None; } hi <<= 1; cnt += 1; if cnt > 70 { return None; } }
        let mut lo = hi >> 1;
        while lo + 1 < hi { let mid = lo + (hi - lo)/2; if f(mid) { hi = mid } else { lo = mid } }
        Some(hi)
    };

    for &(ca, cb, tag) in [(0usize,1usize,"champ->champ"), (0,2,"champ->tower"), (2,0,"tower->champ")].iter() {
        let ids = [eid, oid, tid2];
        let (a, b) = (ids[ca], ids[cb]);
        // 거리를 x 축으로 바꿔가며 경계 margin 을 잰다
        let base = { let e = game.world.entity.get(b).unwrap(); (e.x, e.y) };
        for &d in [0u64, 1000, 5000, 10000, 25000, 50000, 100000, 200000, 400000].iter() {
            {
                let e = game.world.entity.get_mut(a).unwrap();
                e.x = base.0 + d; e.y = base.1;
            }
            let m = margin_probe(&game, a, b);
            println!("hit\t{}\td={}\tmin_margin={:?}", tag, d, m);
        }
    }

    // ---------- (3) is_enemy_well_danger ----------
    let step = setting.width / 20;
    for ver in 0..6usize {
        for t in 0..2usize {
            let ps = game.get_player_by_position(t, Position::Jungle).unwrap();
            let mut trues: Vec<(u64,u64)> = Vec::new();
            let mut yy = 0u64;
            while yy <= setting.height {
                let mut xx = 0u64;
                while xx <= setting.width {
                    if game_ai::is_enemy_well_danger(ver, ps, xx, yy) { trues.push((xx, yy)); }
                    xx += step;
                }
                yy += step;
            }
            println!("well\tver={}\tteam={}\tn_true={}\tfirst10={:?}", ver, t, trues.len(), &trues[..trues.len().min(10)]);
        }
    }
}
