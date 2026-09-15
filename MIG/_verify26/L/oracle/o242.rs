#![allow(unused, dead_code, non_snake_case)]
//! 26차 L · 242 StealSubPlan::score 오라클(pub 직접 호출). 한 프로세스 = 한 케이스(argv[1]).
//! 세르펜/에픽 live_list 는 Game.mode.jungle_runner.{serpen,epic}.live_list(전부 pub) 에 챔피언 id 를 직접 꽂는다.
use game_core::*;
use game_ai::*;
use game_ai::plan_legacy::sub_plan::StealSubPlan;
use rand::SeedableRng;
use std::sync::Arc;
include!("common.rs");

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let setting = real_setting();
    setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext { pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items, ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    // 엔티티 id 수집
    let (idA, idB) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c.player_champion[1][0].unwrap().id, c.player_champion[1][2].unwrap().id)
    };
    // target 종류: 0 None · 1 Epic · 2 Serpen · 3 Serpen(live_list 비어있음) · 4 Epic(live_list 에 존재하지 않는 id)
    let tkind = case / 16;
    let akind = case % 16;
    game.mode.jungle_runner.serpen.live_list = if tkind == 3 { vec![] } else { vec![idA, idB] };
    game.mode.jungle_runner.epic.live_list   = if tkind == 4 { vec![999_999] } else { vec![idB, idA] };
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let zp: std::mem::MaybeUninit<ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let parameter: ScoreParameter = unsafe { zp.assume_init() };
    let mut dbg: DebugFrameData = Default::default();
    let sp = StealSubPlan { last_vision_tick: 0, target: match tkind { 0 => None, 1 | 4 => Some(StealTarget::Epic), _ => Some(StealTarget::Serpen) }, commit: true };
    // 기대 대상 id: Serpen → idA · Epic → idB
    let (name, action): (&str, SmallActionPlay) = match akind {
        0 => ("Attack(A)", SmallActionPlay::Attack(SmallActionAttack::new(&data, idA))),
        1 => ("Attack(B)", SmallActionPlay::Attack(SmallActionAttack::new(&data, idB))),
        2 => ("Skill(A)", SmallActionPlay::Skill(SmallActionSkill::new(&data, idA))),
        3 => ("Skill2(B)", SmallActionPlay::Skill2(SmallActionSkill2::new(&data, idB))),
        4 => ("Ult(A)", SmallActionPlay::Ult(SmallActionUlt::new(&data, idA))),
        5 => ("Trace(A)", SmallActionPlay::Trace(SmallActionTrace::new(&data, idA, 30))),
        6 => ("Trace(B)", SmallActionPlay::Trace(SmallActionTrace::new(&data, idB, 30))),
        7 => ("AroundRegion", SmallActionPlay::AroundRegion(SmallActionAroundRegion::new(2, &mut rnd, &data, player, 0, 0))),
        8 => ("Around(A)", SmallActionPlay::Around(SmallActionAround::new(2, &mut rnd, &data, player, idA, 0))),
        9 => ("Recall", SmallActionPlay::Recall(SmallActionRecall::new(&data, player, 5))),
        10 => ("Stop", SmallActionPlay::Stop),
        11 => ("AroundPosition", SmallActionPlay::AroundPosition(SmallActionAroundPosition::new(&mut rnd, &data, 100000, 100000, 0))),
        12 => ("AroundBush", SmallActionPlay::AroundBush(SmallActionAroundBush::new(&mut rnd, &data, player, 0))),
        _ => { println!("nocase"); return; }
    };
    let tag = unsafe { *((&action as *const SmallActionPlay as *const u8).add(0xb1)) };
    let sc = StealSubPlan::score(&sp, 2, &parameter, &mut rnd, player, &data, &action, &mut dbg);
    println!("case={}\ttkind={}\taction={}\ttag={}\tidA={}\tidB={}\tscore={}", case, tkind, name, tag, idA, idB, sc);
    std::mem::forget(action);
}
