#![allow(unused, dead_code, non_snake_case)]
// 3차 배치 D — 오라클 2단계 : ★15 single_try_engage 체인 전체를 오라클로 돌린다
//  근거: tcx 상 SinglePlanBattle::{new,new_dive,update} · single_tower_dive_is_viable(single_battle::)
//        · BattlePlanGoal · BattleSubPlanGoal 이 **전부 pub** ⟹ single_try_engage 본문을 그대로 재현 가능
//  목표:
//   (A) open[1] version 분기 — version 을 쓸어 결과가 바뀌는지 실행으로 확인
//   (B) open[2] single_tower_dive_is_viable 진리표 (single_battle:: 쪽. death_battle:: 아님)
//   (C) open[3] update 가 sub_goal 을 KitingBack/RunAway/End 로 떨구는 조건 — 거리 스윕으로 임계 실측
//   (D) open[0] TryKill.__1 (=60) 이 결과에 영향을 주는가 — 값을 바꿔 A/B
use game_core::*;
use game_ai::plan_legacy::old as old;
use game_ai::plan_legacy::old::{SinglePlanBattle, BattlePlanGoal, BattleSubPlanGoal};
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

// ★실측: BattleSubPlanGoal 의 페이로드 variant 는 **struct variant `{ focus }`** 다(tuple 아님).
fn sg(g: &BattleSubPlanGoal) -> String { format!("{:?}", g) }
fn committed(g: &BattleSubPlanGoal) -> bool {
    !matches!(g, BattleSubPlanGoal::KitingBack { .. } | BattleSubPlanGoal::RunAway | BattleSubPlanGoal::End)
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
    let pnames = ["Top", "Jungle", "Mid", "Bottom", "Support"];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd, &ctx);          // ★ init_tower/init_nexus 재호출 없음

    let pos_score: PositioningScoreData = Default::default();
    let tp: TeamPlan = Default::default();

    // ── 대상 id 목록 수집 ─────────────────────────────────────
    let (champ_ids, tower_ids, c0_id, c1_id, c1_pos) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut ci = Vec::new();
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] { ci.push((t, p, e.id)); }
        }}
        let mut ti = Vec::new();
        for t in 0..2usize { for (i, e) in cache.iter_towers_without_nexus(t).enumerate() { ti.push((t, i, e.id)); } }
        let c0 = cache.player_champion[0][0].unwrap();
        let c1 = cache.player_champion[1][0].unwrap();
        (ci, ti, c0.id, c1.id, (c1.x, c1.y))
    };
    println!("meta\tchamp_ids={:?}", champ_ids);
    println!("meta\ttower_ids={:?}", tower_ids);
    println!("meta\tc0_id={}\tc1_id={}\tc1_pos={:?}", c0_id, c1_id, c1_pos);

    // ── single_try_engage 재현 (본문 그대로) ───────────────────
    // in_tower / viable / sub_goal / committed 를 전부 뽑아 준다.
    macro_rules! run {
        ($ver:expr, $t:expr, $p:expr, $tid:expr, $goal1:expr) => {{
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let mut dbgf: DebugFrameData = Default::default();
            let player = game.get_player_by_position($t, poss[$p]).unwrap();
            let g = cache.game;
            let ent = g.get_entity_by_id($tid);
            let in_tower = ent.map(|e| game_ai::engage_requires_dive(player, &data, e)).unwrap_or(false);
            let mut viable = None;
            let mut res: Option<(bool, String, Option<String>)> = None;
            if in_tower {
                match g.get_entity_by_id($tid) {
                    None => { res = Some((false, "-".into(), None)); }
                    Some(target) => {
                        let v = old::single_tower_dive_is_viable($ver, &mut rnd, player, &data, &tp, target, &mut dbgf);
                        viable = Some(v);
                        if !v { res = Some((false, "-".into(), None)); }
                        else {
                            let mut b = SinglePlanBattle::new_dive($ver, BattlePlanGoal::TryKill($tid, $goal1), &data, player);
                            b.dive_tower = cache.iter_towers_without_nexus(1 - $t)
                                .min_by_key(|t2| t2.distance_sq(target))
                                // ★실측: EntityType::Tower 는 **struct variant `{ info: Tower }`** 이고
                                //   TowerType 은 `info.ty`(Tower+0xb8 ⟹ Entity+0x128) 다.
                                .and_then(|e| if let EntityType::Tower { info } = &e.ty { Some(info.ty) } else { None });
                            let dt = b.dive_tower.map(|x| format!("{:?}", x));
                            b.update($ver, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
                            res = Some((committed(&b.sub_goal), sg(&b.sub_goal), dt));
                        }
                    }
                }
            } else if ent.is_some() || true {
                let mut b = SinglePlanBattle::new($ver, BattlePlanGoal::TryKill($tid, $goal1), &data, player);
                b.update($ver, &mut rnd, player, &data, &pos_score, &tp, &mut dbgf);
                res = Some((committed(&b.sub_goal), sg(&b.sub_goal), None));
            }
            let (c, s, dt) = res.unwrap();
            (in_tower, viable, c, s, dt)
        }};
    }

    // ── (A) version 스윕 ──────────────────────────────────────
    println!("--- A: version sweep (actor=t0Top, target=C1Top champ / T1#0 tower) ---");
    for ver in [0usize, 1, 2, 3, 4, 5, 10, 30, 31, 32, 33, 50, 100] {
        let (it1, v1, c1, s1, d1) = run!(ver, 0, 0, c1_id, 60usize);
        let ttid = tower_ids.iter().find(|(t, i, _)| *t == 1 && *i == 0).unwrap().2;
        let (it2, v2, c2, s2, d2) = run!(ver, 0, 0, ttid, 60usize);
        println!("A\tver={}\tTGT=champ in_tower={} viable={:?} sub_goal={} committed={}\t|\tTGT=tower in_tower={} viable={:?} sub_goal={} committed={} dive_tower={:?}",
                 ver, it1, v1, s1, c1, it2, v2, s2, c2, d2);
    }

    // ── (B) single_tower_dive_is_viable 진리표 ─────────────────
    println!("--- B: single_tower_dive_is_viable (single_battle::) ---");
    {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let data = OperationData::new(&cache, &ctx, &bb);
        let mut dbgf: DebugFrameData = Default::default();
        let mut allents: Vec<(&Entity, String)> = Vec::new();
        for t in 0..2usize { for p in 0..5usize {
            if let Some(e) = cache.player_champion[t][p] { allents.push((e, format!("C{}{}", t, pnames[p]))); } }}
        for t in 0..2usize { for (i, e) in cache.iter_towers_without_nexus(t).enumerate() {
            allents.push((e, format!("T{}#{}", t, i))); }}
        let mut nt = 0; let mut nf = 0;
        for ver in [3usize, 32, 50] {
            for t in 0..2usize { for p in 0..5usize {
                let player = game.get_player_by_position(t, poss[p]).unwrap();
                for (e, n) in allents.iter() {
                    let v = old::single_tower_dive_is_viable(ver, &mut rnd, player, &data, &tp, e, &mut dbgf);
                    if v { nt += 1; println!("B\tver={} actor=t{}{} target={}\tTRUE", ver, t, pnames[p], n); } else { nf += 1; }
                }
            }}
        }
        println!("B\tSUMMARY\ttrue={}\tfalse={}", nt, nf);
        // death_battle:: 쌍둥이는 in:death_battle 이라 호출 불가(참조용 기록)
    }

    // ── (C) update 거리 게이트 스윕 (actor=t0Top, target=C1Top) ─
    println!("--- C: distance sweep (actor t0Top moved toward/away from target C1Top) ---");
    let ds: Vec<u64> = vec![0, 1000, 10000, 50000, 100000, 150000, 190000, 199000, 199999,
                            200000, 200001, 210000, 250000, 290000, 299999, 300000, 300001,
                            350000, 400000, 500000, 800000];
    for d in ds.iter() {
        // 대상(C1Top) 기준 x 축으로 d 만큼 떨어진 위치에 actor 배치
        {
            let e = game.world.entity.get_mut(c0_id).unwrap();
            e.x = c1_pos.0.saturating_sub(*d);
            e.y = c1_pos.1;
        }
        let (it, v, c, s, dt) = run!(3usize, 0, 0, c1_id, 60usize);
        println!("C\td={}\tin_tower={}\tsub_goal={}\tcommitted={}", d, it, s, c);
    }
    // 원위치 복구
    {
        let e = game.world.entity.get_mut(c0_id).unwrap();
        e.x = 15000; e.y = 913000;
    }

    // ── (C-2) hp 스윕 ─────────────────────────────────────────
    println!("--- C2: actor hp sweep (d=100000) ---");
    {
        let e = game.world.entity.get_mut(c0_id).unwrap();
        e.x = c1_pos.0.saturating_sub(100000); e.y = c1_pos.1;
    }
    let hp_max = { game.world.entity.get(c0_id).unwrap().stat_cached.hp };
    for frac in [100usize, 75, 50, 30, 20, 10, 5, 1] {
        { let e = game.world.entity.get_mut(c0_id).unwrap(); e.hp = hp_max * frac / 100; }
        let (it, v, c, s, dt) = run!(3usize, 0, 0, c1_id, 60usize);
        println!("C2\thp%={}\thp={}\tsub_goal={}\tcommitted={}", frac, hp_max * frac / 100, s, c);
    }
    { let e = game.world.entity.get_mut(c0_id).unwrap(); e.hp = hp_max; e.x = 15000; e.y = 913000; }

    // ── (D) TryKill.__1 A/B ───────────────────────────────────
    println!("--- D: TryKill.__1 A/B (60 vs 0 vs 1 vs 99999) ---");
    for g1 in [60usize, 0, 1, 30, 120, 99999] {
        let (it, v, c, s, dt) = run!(3usize, 0, 0, c1_id, g1);
        let ttid = tower_ids.iter().find(|(t, i, _)| *t == 1 && *i == 0).unwrap().2;
        let (it2, v2, c2, s2, d2) = run!(3usize, 0, 0, ttid, g1);
        println!("D\tgoal1={}\tchamp:sub_goal={} committed={}\t|\ttower:sub_goal={} committed={} dive_tower={:?}",
                 g1, s, c, s2, c2, d2);
    }

    // ── (E) 없는 id / 아군 id / 넥서스 id ─────────────────────
    println!("--- E: target id 변주 ---");
    let nexus_id = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.nexus[1].map(|e| e.id)
    };
    let mut ids: Vec<(String, usize)> = vec![("bogus".into(), 999999usize)];
    ids.push(("ally_champ".into(), champ_ids.iter().find(|(t,p,_)| *t==0 && *p==1).unwrap().2));
    ids.push(("enemy_champ".into(), c1_id));
    ids.push(("own_tower".into(), tower_ids.iter().find(|(t,i,_)| *t==0 && *i==0).unwrap().2));
    ids.push(("enemy_tower".into(), tower_ids.iter().find(|(t,i,_)| *t==1 && *i==0).unwrap().2));
    if let Some(n) = nexus_id { ids.push(("enemy_nexus".into(), n)); }
    for (nm, id) in ids.iter() {
        let (it, v, c, s, dt) = run!(3usize, 0, 0, *id, 60usize);
        println!("E\t{}\tid={}\tin_tower={}\tviable={:?}\tsub_goal={}\tcommitted={}\tdive_tower={:?}", nm, id, it, v, s, c, dt);
    }
}
