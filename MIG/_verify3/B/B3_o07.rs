#![allow(unused, dead_code, non_snake_case)]
// 3차 반증검증 배치 B — 오라클: 07 EpicHuntAndBattlePlan::sub_plan
// 목표: L36 귀환(Recall=태그5) 조건 · hp_ratio<51 임계 · L47 기본 EpicHunt(태그11)
// ⚠ target_bush 필드가 private 이라 Default(=None) 로만 만들 수 있어 Hide(9) 경로는 미도달.
// ⚠ BRIEF §3① — init_tower/init_nexus 재호출 금지
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;
use game_ai::plan_legacy::old::EpicHuntAndBattlePlan;

fn tag_of(sp: &game_ai::plan_legacy::sub_plan::SubPlan) -> u64 {
    unsafe { *(sp as *const _ as *const u64) }
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
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(7);
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut pid = 0usize;
    for t in 0..2usize { for p in 0..5usize {
        let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
        let st: AthleteStat = Default::default();
        game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
        pid += 1;
    }}
    game.start_game(&mut rnd0, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let f0 = map.fountains[0];
    println!("# fountain[team0]={:?}  fountain[team1]={:?}", f0, map.fountains[1]);

    // '에픽' 대역으로 쓸 실제 world 엔티티 = 적팀 서포터 챔피언
    let (epic_id, base_champ) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c.player_champion[1][4].unwrap().id, c.player_champion[0][1].unwrap().clone())
    };
    println!("# epic_id={} champ_base hp={} max={} pos=({},{})",
        epic_id, base_champ.hp, base_champ.stat_cached.hp, base_champ.x, base_champ.y);

    let plan: EpicHuntAndBattlePlan = Default::default();   // target_bush = None
    let gd: game_ai::GoalData = Default::default();

    // (hp, max, x, y, epic_full, live_ok, 기대태그 or None)
    let ins = f0;                     // 힐 영역 안 좌표(분수 사각형 좌상단)
    let cases: Vec<(&str, usize, usize, u64, u64, bool, bool)> = vec![
        // name, champ.hp, champ.max, x, y, epic 풀피?, live_list 에 epic 넣기?
        ("A ratio30 out epicfull",   30, 100, 500000, 500000, true,  true),
        ("B ratio50 out epicfull",   50, 100, 500000, 500000, true,  true),
        ("C ratio51 out epicfull",   51, 100, 500000, 500000, true,  true),
        ("D ratio100 out epicfull", 100, 100, 500000, 500000, true,  true),
        ("E ratio51 IN  epicfull",   51, 100, ins.0 + 1, ins.1 + 1, true, true),
        ("F ratio100 IN epicfull",  100, 100, ins.0 + 1, ins.1 + 1, true, true),
        ("G ratio30 out epicHURT",   30, 100, 500000, 500000, false, true),
        ("H ratio30 out epicNONE",   30, 100, 500000, 500000, true,  false),
    ];
    for (name, hp, mx, x, y, epic_full, live_ok) in cases {
        // epic HP 조작
        {
            let g: &mut dyn AbstractGame = &mut game;
            let e = g.get_entity_by_id_mut(epic_id).unwrap();
            let m = e.stat_cached.hp;
            e.hp = if epic_full { m } else { m - 1 };
        }
        game.mode.jungle_runner.epic.live_list = if live_ok { vec![epic_id] } else { Vec::new() };
        let mut ce = base_champ.clone();
        ce.hp = hp; ce.stat_cached.hp = mx; ce.x = x; ce.y = y;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][1] = Some(&ce);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut rnd = rand::rngs::StdRng::seed_from_u64(7);   // 매 호출 동일 시드
        let mut dbgf: DebugFrameData = Default::default();
        let sp = plan.sub_plan(3, &mut rnd, ps, &data, &gd, &mut dbgf);
        let ratio = hp * 100 / mx;
        println!("CASE\t{}\tratio={}\ttag={}\t{:?}", name, ratio, tag_of(&sp), sp);
    }
    // version 무영향 확인 (기본 경로)
    {
        game.mode.jungle_runner.epic.live_list = Vec::new();
        let mut ce = base_champ.clone(); ce.hp = 30; ce.stat_cached.hp = 100; ce.x = 500000; ce.y = 500000;
        let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        cache.player_champion[0][1] = Some(&ce);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps = game.get_player_by_position(0, Position::Jungle).unwrap();
        let mut base = None; let mut vdiff = 0usize;
        for v in [0usize,1,2,3,24,30,46,50,60] {
            let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
            let mut dbgf: DebugFrameData = Default::default();
            let t = tag_of(&plan.sub_plan(v, &mut rnd, ps, &data, &gd, &mut dbgf));
            match base { None => base = Some(t), Some(b) => if b != t { vdiff += 1; } }
        }
        println!("RESULT\tversion_diff={}\tbase_tag={:?}", vdiff, base);
    }
}
