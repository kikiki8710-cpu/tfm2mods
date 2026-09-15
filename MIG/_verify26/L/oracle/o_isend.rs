#![allow(unused, dead_code, non_snake_case)]
//! 26차 L · 243 Trace::is_end / 244 Recall::is_end / 248 Ult::is_end 오라클 — hidden 함수를 `#[link_name]` 로 직접 진입.
//! 한 프로세스 = 한 케이스(argv[1] = 함수 t/r/u, argv[2] = 케이스 번호).
//! 틱은 액션 생성(new) 뒤 캐시를 버리고 game.set_tick 으로 바꾼 다음 캐시를 다시 만들어 준다.
use game_core::*;
use game_ai::*;
use rand::SeedableRng;
use std::sync::Arc;
include!("common.rs");

extern "Rust" {
    #[link_name = "_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB4_16SmallActionTrace6is_end"]
    fn trace_is_end(s: &SmallActionTrace, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData) -> bool;
    #[link_name = "_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB5_17SmallActionRecall6is_end"]
    fn recall_is_end(s: &SmallActionRecall, rnd: &mut rand::rngs::StdRng, version: usize, player: &PlayerState, data: &OperationData) -> bool;
    #[link_name = "_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_14SmallActionUlt6is_end"]
    fn ult_is_end(s: &SmallActionUlt, rnd: &mut rand::rngs::StdRng, version: usize, player: &PlayerState, data: &OperationData) -> bool;
}

fn set_xy(game: &mut Game, id: usize, x: u64, y: u64) {
    let e = game.get_entity_by_id_mut(id).unwrap() as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*e).x, x); std::ptr::write_volatile(&mut (*e).y, y); }
}
fn set_vis(game: &mut Game, id: usize, team: usize, v: VisibleState) {
    let e = game.get_entity_by_id_mut(id).unwrap() as *mut Entity;
    unsafe { std::ptr::write_volatile(&mut (*e).visible_state[team], v); }
}

fn main() {
    if std::env::args().count() > 99 { let _ = game_ai::battle_action as *const (); }
    let which = std::env::args().nth(1).unwrap_or("t".into());
    let case: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
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
    let (my, tgt, myx, myy) = {
        let c = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let m = c.player_champion[0][1].unwrap(); let t = c.player_champion[1][0].unwrap();
        (m.id, t.id, m.x, m.y)
    };
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let (lx, ly, rx, ry) = map.fountain(0);
    println!("my={} tgt={} myxy=({},{}) fountain0=({},{},{},{}) tick0={}", my, tgt, myx, myy, lx, ly, rx, ry, game.tick());

    // ── 액션 생성(틱 100) ──
    let (mut tr, mut rc, mut ul) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        let player = game.get_player_by_position(0, Position::Jungle).unwrap();
        (SmallActionTrace::new(&data, tgt, 30), SmallActionRecall::new(&data, player, 5), SmallActionUlt::new(&data, tgt))
    };
    println!("trace={:?}", tr);
    println!("recall={:?}", rc);
    println!("ult={:?}", ul);
    let ult_bytes: [u64; 3] = unsafe { std::mem::transmute_copy(&ul) };
    println!("ult_raw start_tick={} target={} is_act={}", ult_bytes[0], ult_bytes[1], ult_bytes[2] & 0xff);

    // ── 케이스별 세계 조작 ──
    let mut tick = 100usize; let mut version = 2usize; let mut target_override: Option<usize> = None; let mut label = String::new();
    let mut vis = VisibleState::Visible;
    match (which.as_str(), case) {
        ("t", 0) => { label = "기본: 보임·goal 멀리·tick=100(<130)".into(); }
        ("t", 1) => { label = "abandoned".into(); tr.mark_abandoned(); }
        ("t", 2) => { label = "만료 tick=130(=start+end_delay)".into(); tick = 130; }
        ("t", 3) => { label = "만료 직전 tick=129".into(); tick = 129; }
        ("t", 4) => { label = "대상 소멸(target=999999)".into(); target_override = Some(999_999); }
        ("t", 5) => { label = "안 보임(Invisible)".into(); vis = VisibleState::Invisible { last_x: 0, last_y: 0 }; }
        ("t", 6) => { label = "안 보임(Unknown)".into(); vis = VisibleState::Unknown; }
        ("t", 7) => { label = "보임 · goal dx=10000 (dist²=1e8)".into(); tr.goal_x = myx + 10000; tr.goal_y = myy; }
        ("t", 8) => { label = "보임 · goal dx=10001".into(); tr.goal_x = myx + 10001; tr.goal_y = myy; }
        ("t", 9) => { label = "안 보임 · goal dx=0 (도달했어도 시야 없으면 false?)".into(); vis = VisibleState::Invisible { last_x: 0, last_y: 0 }; tr.goal_x = myx; tr.goal_y = myy; }
        ("t", 10) => { label = "보임 · goal dx=6000,dy=8000 (dist²=1e8)".into(); tr.goal_x = myx + 6000; tr.goal_y = myy + 8000; }
        ("r", 0) => { label = "기본 v2: tick=100(<105)·goal 멀리·우물 밖".into(); }
        ("r", 1) => { label = "만료 tick=105".into(); tick = 105; }
        ("r", 2) => { label = "만료 직전 tick=104".into(); tick = 104; }
        ("r", 3) => { label = "goal dx=10000".into(); rc.goal_x = 300000 + 10000; rc.goal_y = 500000; }
        ("r", 4) => { label = "goal dx=10001".into(); rc.goal_x = 300000 + 10001; rc.goal_y = 500000; }
        ("r", 11) => { label = "goal dx=6000,dy=8000 (dist²=1e8)".into(); rc.goal_x = 306000; rc.goal_y = 508000; }
        ("r", 12) => { label = "goal dx=0 · v1".into(); rc.goal_x = 300000; rc.goal_y = 500000; version = 1; }
        ("r", 5) => { label = "우물 안(lx,ly) v2".into(); }
        ("r", 6) => { label = "우물 안(lx,ly) v1".into(); version = 1; }
        ("r", 7) => { label = "우물 안(rx,ry) v2".into(); }
        ("r", 8) => { label = "우물 밖 x=lx-1 v2".into(); }
        ("r", 9) => { label = "우물 밖 y=ry+1 v2".into(); }
        ("r", 10) => { label = "우물 안 v2 · 만료(tick=105) 동시".into(); tick = 105; }
        ("u", 0) => { label = "기본: tick=100 · 보임 · is_act=false".into(); }
        ("u", 1) => { label = "tick=105 (start+5<=tick)".into(); tick = 105; }
        ("u", 2) => { label = "tick=104".into(); tick = 104; }
        ("u", 3) => { label = "is_act=true".into(); ul = unsafe { std::mem::transmute::<[u64; 3], SmallActionUlt>([100, tgt as u64, 1]) }; }
        ("u", 4) => { label = "대상 소멸".into(); ul = unsafe { std::mem::transmute::<[u64; 3], SmallActionUlt>([100, 999_999, 0]) }; }
        ("u", 5) => { label = "안 보임(Invisible)".into(); vis = VisibleState::Invisible { last_x: 0, last_y: 0 }; }
        ("u", 6) => { label = "안 보임(Unknown)".into(); vis = VisibleState::Unknown; }
        ("u", 7) => { label = "안 보임 + tick=104".into(); vis = VisibleState::Unknown; tick = 104; }
        _ => { println!("nocase"); return; }
    }
    if let Some(t) = target_override {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let data = OperationData::new(&cache, &ctx, &bb);
        tr = SmallActionTrace::new(&data, t, 30);   // 재생성: target 만 바꾼다(틱 100 유지)
    }
    if which == "r" {
        match case { 0 | 1 | 2 | 3 | 4 | 11 | 12 => set_xy(&mut game, my, 300000, 500000), 5 | 6 | 10 => set_xy(&mut game, my, lx, ly), 7 => set_xy(&mut game, my, rx, ry), 8 => set_xy(&mut game, my, lx - 1, ly), 9 => set_xy(&mut game, my, lx, ry + 1), _ => {} }
    }
    set_vis(&mut game, tgt, 0, vis);
    game.set_tick(tick);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(0, Position::Jungle).unwrap();
    let champ = cache.player_champion[0][1].unwrap();
    let tv = cache.player_champion[1][0].map(|e| format!("{:?}", e.visible_state[0])).unwrap_or("?".into());
    let r = unsafe { match which.as_str() {
        "t" => trace_is_end(&tr, &mut rnd, player, &data),
        "r" => recall_is_end(&rc, &mut rnd, version, player, &data),
        _ => ult_is_end(&ul, &mut rnd, version, player, &data),
    } };
    println!("RESULT\t{}{}\t{}\ttick={}\tchamp=({},{})\ttgt_vis={}\tversion={}\tis_end={}", which, case, label, game.tick(), champ.x, champ.y, tv, version, r);
}
