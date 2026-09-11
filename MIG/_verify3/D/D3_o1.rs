#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 1단계 : ★깨끗한 세계(start_game 만) 재실측
//  0) 세계 인구조사 — towerchk 결론(tower_ids 16 · twin 2/2 · 팀당 8 · 좌표중복 0) 자기확인
//  1) iter_towers_without_nexus 구성/순서 = [top,mid,bottom,top2,mid2,bottom2]+twin_towers 확인
//  2) Effect::is_in_range 재현 대조 (깨끗한 세계 · 전 조합)
//  3) ★합성 엔티티(반경/사거리/casting/레벨/거리 변주)로 판별력 있는 진리표 재대조
//  4) engage_requires_dive 진리표 (깨끗한 세계)
//  5) best_jungle_goal 10/10 재확인
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

fn main() {
    let pool = bumpalo::Bump::new();
    let mut setting: GameSetting = Default::default();
    setting.tick_per_second = 60;                       // MapDef::moba 이전
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
    // ★init_tower / init_nexus 를 부르지 않는다 (2차 오염원)

    // ── 0) 세계 인구조사 ─────────────────────────────────────────
    {
        let mut c = [0usize; 2];
        let mut xy: [Vec<(u64,u64)>; 2] = [Vec::new(), Vec::new()];
        for id in game.world.tower_ids.iter() {
            if let Some(e) = game.world.entity.get(*id) {
                if let TeamType::Player(t) = e.team {
                    if t < 2 { c[t] += 1; xy[t].push((e.x, e.y)); }
                }
            }
        }
        let dupe = |v: &Vec<(u64,u64)>| { let mut w = v.clone(); w.sort(); w.dedup(); v.len() - w.len() };
        println!("census\ttower_ids={}\tteam0={}\tteam1={}\tdupe0={}\tdupe1={}",
                 game.world.tower_ids.len(), c[0], c[1], dupe(&xy[0]), dupe(&xy[1]));
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    println!("census\ttwin_towers\t{}\t{}", cache.twin_towers[0].len(), cache.twin_towers[1].len());

    // ── 1) iter_towers_without_nexus 구성/순서 ──────────────────
    for t in 0..2usize {
        let it: Vec<&Entity> = cache.iter_towers_without_nexus(t).collect();
        let expect_named: Vec<Option<&Entity>> = vec![
            cache.top_tower[t], cache.mid_tower[t], cache.bottom_tower[t],
            cache.top_tower2[t], cache.mid_tower2[t], cache.bottom_tower2[t]];
        let mut expect: Vec<&Entity> = expect_named.into_iter().flatten().collect();
        for e in cache.twin_towers[t].iter() { expect.push(*e); }
        let same_len = it.len() == expect.len();
        let same_ptr = same_len && it.iter().zip(expect.iter())
                                    .all(|(a,b)| (*a as *const Entity) == (*b as *const Entity));
        println!("itertowers\tteam={}\tn={}\texpect_n={}\tPTR_IDENTICAL={}\tnexus_in_iter={}",
                 t, it.len(), expect.len(), same_ptr,
                 cache.nexus[t].map(|n| it.iter().any(|e| (*e as *const Entity)==(n as *const Entity))).unwrap_or(false));
        for (i, e) in it.iter().enumerate() {
            println!("itertowers\tteam={};i={}\tid={}\tpos=({},{})\tradius={}\tae={}\tcasting={:?}",
                     t, i, e.id, e.x, e.y, e.radius, e.attack_effect.is_some(),
                     e.attack_effect.as_ref().map(|f| format!("{:?}", f.casting)).unwrap_or("-".into()));
        }
    }

    // ── 실물 엔티티 목록 ────────────────────────────────────────
    let mut ents: Vec<(&Entity, String)> = Vec::new();
    for t in 0..2usize { for p in 0..5usize {
        if let Some(e) = cache.player_champion[t][p] { ents.push((e, format!("C{}{}", t, pnames[p]))); }
    }}
    let mut towers: Vec<(&Entity, String)> = Vec::new();
    for t in 0..2usize {
        for (i, e) in cache.iter_towers_without_nexus(t).enumerate() {
            towers.push((e, format!("T{}#{}", t, i)));
        }
    }
    println!("meta\ttick_per_second\t{}\tchamps\t{}\ttowers\t{}", setting.tick_per_second, ents.len(), towers.len());

    // 재현식
    fn mine_range(eff: &Effect, e: &Entity) -> u64 {
        eff.range.wrapping_add(eff.growth_range.wrapping_mul((e.level as u64).wrapping_sub(1)))
                 .wrapping_add(e.stat_buff_cached.range as u64)
    }
    fn mine_radius(e: &Entity) -> u64 {
        let m = e.stat_buff_cached.radius_mult as i64;
        if m == 0 { e.radius as u64 } else { (((e.radius as i64) * (100i64 + m)) / 100) as u64 }
    }
    fn mine_iir(eff: &Effect, c: &Entity, t: &Entity) -> bool {
        let r = mine_range(eff, c)
                    .wrapping_add(eff.range_adjust(c, t) as u64)
                    .wrapping_add(if matches!(eff.casting, CastingType::Targeting) { mine_radius(c) } else { 0 })
                    .wrapping_add(mine_radius(t));
        game_core::utils::distance_sq(c.x, c.y, t.x, t.y) <= r.wrapping_mul(r)
    }

    // ── 2) is_in_range 실물 전 조합 ─────────────────────────────
    let all: Vec<(&Entity, String)> = ents.iter().chain(towers.iter()).map(|(e,n)| (*e, n.clone())).collect();
    let mut ok = 0; let mut bad = 0; let mut nt = 0; let mut nf = 0;
    for (ce, cn) in all.iter() {
        let eff = match ce.attack_effect.as_ref() { Some(f) => f, None => continue };
        for (te, tn) in all.iter() {
            let g = eff.is_in_range(ce, te);
            let m = mine_iir(eff, ce, te);
            if g { nt += 1 } else { nf += 1 }
            if g == m { ok += 1 } else { bad += 1;
                println!("iir_real\t{}->{}\tmine={}\tgame={}\t★MISMATCH", cn, tn, m, g); }
        }
    }
    println!("iir_real\tSUMMARY\tok={}\tmismatch={}\tgame_true={}\tgame_false={}", ok, bad, nt, nf);
    let mut rok=0; let mut rbad=0; let mut aok=0; let mut abad=0;
    for (ce, cn) in all.iter() {
        let eff = match ce.attack_effect.as_ref() { Some(f) => f, None => continue };
        if eff.range(ce) as u64 == mine_range(eff, ce) { rok+=1 } else { rbad+=1;
            println!("eff_range\t{}\tmine={}\tgame={}\t★MISMATCH", cn, mine_range(eff,ce), eff.range(ce)); }
        if ce.radius() as u64 == mine_radius(ce) { aok+=1 } else { abad+=1;
            println!("ent_radius\t{}\tmine={}\tgame={}\t★MISMATCH", cn, mine_radius(ce), ce.radius()); }
    }
    println!("eff_range\tSUMMARY\tok={}\tmismatch={}", rok, rbad);
    println!("ent_radius\tSUMMARY\tok={}\tmismatch={}", aok, abad);

    // ── 3) ★합성 엔티티 진리표 (판별력 확보) ────────────────────
    let base = cache.player_champion[0][0].unwrap();
    let mut syn: Vec<Entity> = Vec::new();
    let castings = [CastingType::Targeting, CastingType::Position, CastingType::Direction, CastingType::None];
    let mut labels: Vec<String> = Vec::new();
    for (ci, cst) in castings.iter().enumerate() {
      for &rng in [0u64, 100, 1000].iter() {
        for &grow in [0u64, 50].iter() {
          for &lvl in [1usize, 4].iter() {
            for &rad in [0usize, 10, 300].iter() {
              for &rmul in [0usize, 50].iter() {
                for &(dx, dy) in [(0u64,0u64), (100,0), (0,1000), (700,700), (5000,0)].iter() {
                  let mut e = base.clone();
                  e.level = lvl; e.radius = rad;
                  e.stat_buff_cached.radius_mult = rmul as _;
                  e.stat_buff_cached.range = 7;
                  e.x = 400000 + dx; e.y = 400000 + dy;
                  if let Some(f) = e.attack_effect.as_mut() {
                      f.range = rng; f.growth_range = grow; f.casting = *cst;
                  }
                  labels.push(format!("c{};r{};g{};l{};rad{};m{};d({},{})", ci, rng, grow, lvl, rad, rmul, dx, dy));
                  syn.push(e);
                }}}}}}}
    let mut sok=0; let mut sbad=0; let mut st=0; let mut sf=0;
    for i in 0..syn.len() {
        let eff = match syn[i].attack_effect.as_ref() { Some(f) => f.clone(), None => continue };
        for j in 0..syn.len() {
            let g = eff.is_in_range(&syn[i], &syn[j]);
            let m = mine_iir(&eff, &syn[i], &syn[j]);
            if g { st+=1 } else { sf+=1 }
            if g==m { sok+=1 } else { sbad+=1;
                if sbad<=20 { println!("iir_syn\t{}->{}\tmine={}\tgame={}\t★MISMATCH", labels[i], labels[j], m, g); } }
        }
    }
    println!("iir_syn\tSUMMARY\tn={}\tok={}\tmismatch={}\tgame_true={}\tgame_false={}", syn.len(), sok, sbad, st, sf);
    let mut rok2=0; let mut rbad2=0; let mut aok2=0; let mut abad2=0;
    for i in 0..syn.len() {
        let eff = match syn[i].attack_effect.as_ref() { Some(f) => f, None => continue };
        if eff.range(&syn[i]) as u64 == mine_range(eff,&syn[i]) { rok2+=1 } else { rbad2+=1;
            if rbad2<=10 { println!("eff_range_syn\t{}\tmine={}\tgame={}\t★MISMATCH", labels[i], mine_range(eff,&syn[i]), eff.range(&syn[i])); } }
        if syn[i].radius() as u64 == mine_radius(&syn[i]) { aok2+=1 } else { abad2+=1;
            if abad2<=10 { println!("ent_radius_syn\t{}\tmine={}\tgame={}\t★MISMATCH", labels[i], mine_radius(&syn[i]), syn[i].radius()); } }
    }
    println!("eff_range_syn\tSUMMARY\tok={}\tmismatch={}", rok2, rbad2);
    println!("ent_radius_syn\tSUMMARY\tok={}\tmismatch={}", aok2, abad2);

    // ── 4) engage_requires_dive (깨끗한 세계) ──────────────────
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let tp: TeamPlan = Default::default();
    let mut dbgf: DebugFrameData = Default::default();
    let mut trues: Vec<String> = Vec::new();
    let mut pairs = 0usize;
    let mut mok=0; let mut mbad=0;
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        for (te, tn) in all.iter() {
            pairs += 1;
            let g = game_ai::engage_requires_dive(ps, &data, te);
            let m = cache.iter_towers_without_nexus(1 - t).any(|tw|
                        tw.attack_effect.as_ref().map(|f| mine_iir(f, tw, te)).unwrap_or(false));
            if g == m { mok+=1 } else { mbad+=1;
                println!("erd\tt{}{}->{}\tmine={}\tgame={}\t★MISMATCH", t, pnames[p], tn, m, g); }
            if g { trues.push(format!("t{}{}->{}", t, pnames[p], tn)); }
        }
    }}
    println!("erd\tSUMMARY\tpairs={}\ttrue_count={}\treprod_ok={}\treprod_mismatch={}", pairs, trues.len(), mok, mbad);
    let mut selfhit = 0usize;
    for s in trues.iter() {
        let tgt = s.split("->").nth(1).unwrap();
        let myteam: usize = if s.starts_with("t0") { 0 } else { 1 };
        if tgt.starts_with(&format!("T{}#", 1 - myteam)) { selfhit += 1; }
    }
    println!("erd\tSELFTOWER\ttrue={}\tenemy_tower_targets={}", trues.len(), selfhit);
    for s in trues.iter() { println!("erd_true\t{}", s); }

    // ── 5) best_jungle_goal ────────────────────────────────────
    let camps = [JungleType::Rhino, JungleType::Mushroom, JungleType::Bee, JungleType::Stump];
    let cn = ["Rhino", "Mushroom", "Bee", "Stump"];
    let mut m2=0; let mut mm2=0;
    for t in 0..2usize { for p in 0..5usize {
        let ps = game.get_player_by_position(t, poss[p]).unwrap();
        let e = cache.player_champion[t][p].unwrap();
        let mut best = (u64::MAX, "?");
        for (c, n) in camps.iter().zip(cn.iter()) {
            let (cx, cy) = map.camp_pos(*c, t == 0);
            let d = game_core::utils::distance_sq(cx, cy, e.x, e.y);
            if d < best.0 { best = (d, n); }
        }
        let got = old::best_jungle_goal(3, &mut rnd, ps, &data, &tp, None, &mut dbgf);
        let hit = format!("{:?}", got) == best.1;
        if hit { m2+=1 } else { mm2+=1 }
        println!("bjg\tt{}{}\tmine={}\tgame={:?}\t{}", t, pnames[p], best.1, got, if hit {"MATCH"} else {"★MISMATCH"});
    }}
    println!("bjg\tSUMMARY\tmatch={}\tmismatch={}", m2, mm2);
    for ti in 0..2usize { for ci in 0..4usize {
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        println!("is_cleared\tteam={};camp={}\t{}\tside={}", ti, cn[ci],
            old::is_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, 0, &mut dbgf),
            old::is_side_cleared(camps[ci], ti, 3, &mut rnd, ps, &data, &tp, 0, &mut dbgf));
    }}

    // ── 6) 16 max_range_nearly_can_use 표본 ────────────────────
    let a = cache.player_champion[0][0].unwrap();
    let b = cache.player_champion[1][0].unwrap();
    for tk in [0usize, 40, 60, 100000] {
        println!("mrncu\tchamp;tick={}\t{}", tk, old::max_range_nearly_can_use(a, b, tk));
    }
    if let Some((tw, tn)) = towers.first() {
        for tk in [0usize, 40, 60, 100000] {
            println!("mrncu\tcaster={};tick={}\t{}", tn, tk, old::max_range_nearly_can_use(tw, b, tk));
        }
    }
}
