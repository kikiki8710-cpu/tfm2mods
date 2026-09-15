#![allow(unused, dead_code, non_snake_case)]
//! 26차 L · 246 SerpenCheckSubPlan::score 오라클(pub 직접 호출). 한 프로세스 = 한 케이스(argv[1]) · argv[2]=1 이면 interaction_score(base)만 찍는다.
//! 세르펜 live_list 에 상대 Top 챔피언 id 를 꽂고, 그 엔티티의 visible_state[0] 을 케이스별로 바꾼다.
use game_core::*;
use game_ai::*;
use game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan;
use rand::SeedableRng;
use std::sync::Arc;
include!("common.rs");

fn set_vis(game: &mut Game, id: usize, team: usize, v: VisibleState) {
    let e = game.get_entity_by_id_mut(id).unwrap() as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*e).visible_state[team], v); }
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let base_only: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    game.set_tick(100);
    let (idA, idB) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        (c.player_champion[1][0].unwrap().id, c.player_champion[1][2].unwrap().id)
    };
    // 케이스: (액션, 시야, live_list)
    //  0 Around(A) 안보임 · 1 Around(A) 보임 · 2 Around(B) 안보임 · 3 Attack(A) 안보임 · 4 Recall 안보임 · 5 Around(A) 안보임+live_list 비어있음
    //  6 AroundHide(A) 안보임 · 7 Around(A) Unknown · 8 Around(A) 안보임 + live_list=[B,A](첫 원소 B)
    let empty = case == 5;
    game.mode.jungle_runner.serpen.live_list = if empty { vec![] } else if case == 8 { vec![idB, idA] } else { vec![idA, idB] };
    let vis = match case { 1 => VisibleState::Visible, 7 => VisibleState::Unknown, _ => VisibleState::Invisible { last_x: 1, last_y: 1 } };
    set_vis(&mut game, idA, 0, vis.clone());
    set_vis(&mut game, idB, 0, vis.clone());
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let zp: std::mem::MaybeUninit<ScoreParameter> = std::mem::MaybeUninit::zeroed();
    let parameter: ScoreParameter = unsafe { zp.assume_init() };
    let mut dbg: DebugFrameData = Default::default();
    let sp: SerpenCheckSubPlan = Default::default();
    let (name, action): (&str, SmallActionPlay) = match case {
        0 | 1 | 5 | 7 | 8 => ("Around(A)", SmallActionPlay::Around(SmallActionAround::new(2, &mut rnd, &data, player, idA, 0))),
        2 => ("Around(B)", SmallActionPlay::Around(SmallActionAround::new(2, &mut rnd, &data, player, idB, 0))),
        3 => ("Attack(A)", SmallActionPlay::Attack(SmallActionAttack::new(&data, idA))),
        4 => ("Recall", SmallActionPlay::Recall(SmallActionRecall::new(&data, player, 5))),
        6 => ("AroundHide(A)", SmallActionPlay::AroundHide(SmallActionAroundHide::new(2, &mut rnd, &data, player, idA, 0))),
        _ => { println!("nocase"); return; }
    };
    let tag = unsafe { *((&action as *const SmallActionPlay as *const u8).add(0xb1)) };
    let visq = game.is_visible(0, idA);
    if base_only == 1 {
        let b = interaction_score(2, &mut rnd, player, &data, &parameter, &action, &mut dbg);
        println!("BASE\tcase={}\taction={}\ttag={}\tis_visible(0,A)={}\tbase={}", case, name, tag, visq, b);
    } else {
        let sc = SerpenCheckSubPlan::score(&sp, 2, &parameter, &mut rnd, player, &data, &action, &mut dbg);
        println!("SCORE\tcase={}\taction={}\ttag={}\tis_visible(0,A)={}\tlive_list={:?}\tscore={}", case, name, tag, visq, game.mode.jungle_runner.serpen.live_list, sc);
    }
    std::mem::forget(action);
}
