#![allow(unused, dead_code, non_snake_case)]
// 2차 배치 D — 오라클 5단계
//  ① setting.tick_per_second = 60 (1차의 미해결 숙제)
//  ② Effect::is_in_range 재현식 game==mine 대조 (신규 — g06.ll:51634 IR 독해 검증)
//  ③ 16 max_range_nearly_can_use 진리표 + "왜 0 이었나" 진단
//  ④ 19 best_jungle_goal 10/10 재확인 (tps=60)
//  ⑤ 15 engage_requires_dive (tps=60)
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;                       // ★1순위 숙제
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
    let pnames = ["Top", "Jungle", "Mid", "Bottom", "Support"];
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tp: TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();

    println!("meta\ttick_per_second\t{}", setting.tick_per_second);

    // ── 챔피언/타워 엔티티 수집 ───────────────────────────────────
    let mut ents: Vec<(&Entity, String)> = Vec::new();
    for t in 0..2usize { for p in 0..5usize {
        if let Some(e) = cache.player_champion[t][p] {
            ents.push((e, format!("C{}{}", t, pnames[p])));
        }
    }}
    let mut towers: Vec<(&Entity, String)> = Vec::new();
    for t in 0..2usize {
        for (i, e) in cache.iter_towers_without_nexus(t).enumerate() {
            towers.push((e, format!("T{}#{}", t, i)));
        }
    }
    println!("meta\tchamps\t{}\ttowers\t{}", ents.len(), towers.len());

    // ── 엔티티 덤프(무엇이 판별력을 죽였는지) ─────────────────────
    for (e, n) in ents.iter().chain(towers.iter()) {
        let ae = e.attack_effect.as_ref();
        println!("dump\t{}\tpos=({},{})\tlevel={}\tradius={}\tradius_mult={}\tsbc_range={}\tattack_effect={}\trange={}\tgrowth={}\tcasting={:?}\ttarget={:?}",
            n, e.x, e.y, e.level, e.radius, e.stat_buff_cached.radius_mult,
            e.stat_buff_cached.range,
            ae.is_some(),
            ae.map(|f| f.range as i64).unwrap_or(-1),
            ae.map(|f| f.growth_range as i64).unwrap_or(-1),
            ae.map(|f| format!("{:?}", f.casting)).unwrap_or("-".into()),
            ae.map(|f| format!("{:?}", f.target)).unwrap_or("-".into()));
    }

    // ── ② Effect::is_in_range 재현 대조 ──────────────────────────
    // mine(R) = eff.range(caster) + eff.range_adjust(caster,target)
    //         + if eff.casting == Targeting { caster.radius() } else { 0 }
    //         + target.radius()
    //   판정 = distance_sq(cx,cy,tx,ty) <= R*R
    let all: Vec<(&Entity, String)> = ents.iter().chain(towers.iter())
        .map(|(e, n)| (*e, n.clone())).collect();
    let mut ok = 0usize; let mut bad = 0usize;
    for (ce, cn) in all.iter() {
        let eff = match ce.attack_effect.as_ref() { Some(f) => f, None => continue };
        for (te, tn) in all.iter() {
            let game_r = eff.is_in_range(ce, te);
            let base = eff.range(ce) as u64;
            let adj = eff.range_adjust(ce, te) as u64;
            let cr = if matches!(eff.casting, CastingType::Targeting) { ce.radius() as u64 } else { 0u64 };
            let tr = te.radius() as u64;
            let r = base.wrapping_add(adj).wrapping_add(cr).wrapping_add(tr);
            let d = game_core::utils::distance_sq(ce.x, ce.y, te.x, te.y);
            let mine = d <= r.wrapping_mul(r);
            if mine == game_r { ok += 1; } else {
                bad += 1;
                println!("is_in_range\t{}->{}\td={}\tR={}\tmine={}\tgame={}\t★MISMATCH", cn, tn, d, r, mine, game_r);
            }
        }
    }
    println!("is_in_range\tSUMMARY\tok={}\tmismatch={}", ok, bad);

    // ── ②-b 대조군: casting 게이트를 무시한 식(=16 이 쓰는 식)과 비교 ──
    let mut diff_gate = 0usize;
    for (ce, cn) in all.iter() {
        let eff = match ce.attack_effect.as_ref() { Some(f) => f, None => continue };
        for (te, tn) in all.iter() {
            let base = eff.range(ce) as u64 + eff.range_adjust(ce, te) as u64;
            let with = base + ce.radius() as u64 + te.radius() as u64;      // 16 의 식
            let gated = base + (if matches!(eff.casting, CastingType::Targeting) { ce.radius() as u64 } else { 0 })
                             + te.radius() as u64;                          // is_in_range 의 식
            if with != gated { diff_gate += 1; }
        }
    }
    println!("gatediff\tSUMMARY\tdiff={}", diff_gate);

    // ── ③ 16 max_range_nearly_can_use ───────────────────────────
    let a = cache.player_champion[0][0].unwrap();
    let b = cache.player_champion[1][0].unwrap();
    for tk in [0usize, 1, 20, 39, 40, 49, 50, 59, 60, 61, 100, 200, 100000] {
        println!("max_range_nearly_can_use\ttick={}\t{}", tk, old::max_range_nearly_can_use(a, b, tk));
    }
    // 타워를 시전자로 (타워는 attack_effect 가 실제로 있다)
    if let Some((tw, tn)) = towers.first() {
        for tk in [0usize, 40, 60, 100000] {
            println!("max_range_nearly_can_use\tcaster={};tick={}\t{}", tn, tk,
                old::max_range_nearly_can_use(tw, b, tk));
        }
    }

    // ── ④ 19 best_jungle_goal (tps=60) ──────────────────────────
    let camps = [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump];
    let cn = ["Rhino", "Mushroom", "Bee", "Stump"];
    let mut m = 0usize; let mut mm = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let e = cache.player_champion[t][p].unwrap();
        let mut best = (u64::MAX, "?");
        for (c, n) in camps.iter().zip(cn.iter()) {
            let (cx, cy) = map.camp_pos(*c, t == 0);
            let d = game_core::utils::distance_sq(cx, cy, e.x, e.y);
            if d < best.0 { best = (d, n); }
        }
        let got = old::best_jungle_goal(3, &mut rnd, ps, &data, &tp, None, &mut dbg);
        let hit = format!("{:?}", got) == best.1;
        if hit { m += 1 } else { mm += 1 }
        println!("best_jungle_goal\tt{}{}\tmine={}\tgame={:?}\t{}", t, pnames[p], best.1, got,
            if hit { "MATCH" } else { "★MISMATCH" });
    }}
    println!("best_jungle_goal\tSUMMARY\tmatch={}\tmismatch={}", m, mm);
    for ti in 0..2usize { for ci in 0..4usize {
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        println!("is_cleared\tteam={};camp={}\t{}\tside={}", ti, cn[ci],
            old::is_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, 0, &mut dbg),
            old::is_side_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, 0, &mut dbg));
    }}

    // ── ⑤ 15 engage_requires_dive (tps=60) ──────────────────────
    let mut nt = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        for (te, tn) in all.iter() {
            let v = game_ai::engage_requires_dive(ps, &data, te);
            if v { nt += 1; println!("engage_requires_dive\tt{}{}->{}\ttrue", t, pnames[p], tn); }
        }
    }}
    println!("engage_requires_dive\tSUMMARY\ttrue_count={}\tpairs={}", nt, 10 * all.len());
}
