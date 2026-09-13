#![allow(unused, dead_code, non_snake_case)]
//! 18차 배치A 오라클 1 — `#57 handle_press_epic`(pub) 를 **실행**으로 확인한다.
//!  (a) 21% 게이트: 시드별로 `StdRng::seed_from_u64(seed).gen_range(0..100) < 21` 을 **독립 계산**해
//!      실제 발령 여부와 1:1 대조 (임계 `< 21` 정확 검증 — 외연이 같은 `<=20` 은 못 가른다).
//!  (b) 명세 독립 재구현(수법 ⓓ): rng 를 복제해 `gen_range` 1회 소비 후 `player.strategy(rnd, game)`
//!      로 같은 전략을 얻고, 명세 logic 대로 (kind, line) 을 예측 → 실제 chats/objective 와 대조.
//!      is_top_side 극성 = `x <= height - y`(17차 배치C 오라클 9/9) 를 그대로 쓴다.
//!  (c) 튜플 3순위(minion_count, 부호 있는 i32) — blackboard 를 argv 로 받은 값으로 채워 분기 유도.
//! 케이스(argv[1]): 0 = Gather(top=5,mid=2,bottom=-3) / 1 = Gather(-1,0,9) / 2 = Gather(0,0,0)
//!   3 = Split14{Jungle} epic>serpen(far=Top) counts(5,2,-3) / 4 = Split14{Mid} epic<serpen(far=Bottom) counts(-1,0,9)
//!   5 = Split131{Top,Support} 전원 존재 / 6 = Split131{Top,Support} + player_champion[t][Top]=None(양팀)
//!   전략 = `game.world.strategy[t].morgard_use`(전부 pub) 를 직접 세팅 — PlayerState::strategy(g15.ll:130480)는
//!   is_solorank 가 아니면 vtable+0x108 `AbstractGame::strategy(team)` 을 복사할 뿐이라 rnd 를 안 쓴다.
//! TLS: handle_press_epic 본문엔 LocalKey 없음. strategy 내부는 미확인 → **케이스당 프로세스 1개**.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify18/A/oracle/v18A_o1.rs
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::team_plan::TeamPlan;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *const u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind { Press, Split }
#[derive(Clone, Copy, PartialEq, Debug)]
enum Line { Top, Mid, Bottom }
fn line_u8(l: Line) -> u8 { match l { Line::Top => 0, Line::Mid => 1, Line::Bottom => 2 } }
fn pos_idx(p: Position) -> usize {
    match p { Position::Top => 0, Position::Jungle => 1, Position::Mid => 2, Position::Bottom => 3, Position::Support => 4 }
}

/// 명세 logic 의 독립 재구현. tower(line) 은 전부 생존(tick 1000) 이라 (true,false) 고정.
/// minion_count 는 이 프로브가 blackboard[team] 에 직접 써 넣은 값.
fn spec_predict(strategy: &Strategy, my_pos: Position, champ_top_side: bool,
                counts: [i32; 3], epic_tick: usize, serpen_tick: usize,
                pos1_champ_some: bool) -> (Kind, Line) {
    // 튜플 (has_first, has_second, count) 사전식 비교 — has_first=true, has_second=false 는 공통
    let pick = |x: Line| -> Line {
        let cx = counts[line_u8(x) as usize];
        let cm = counts[1];
        if (true, false, cx) > (true, false, cm) { x } else { Line::Mid }
    };
    match strategy.morgard_use {
        MorgardUseStrategy::Gather => {
            if champ_top_side { (Kind::Press, pick(Line::Top)) } else { (Kind::Press, pick(Line::Bottom)) }
        }
        MorgardUseStrategy::Split14 { position } => {
            let far = if epic_tick > serpen_tick { Line::Top } else { Line::Bottom };
            if my_pos == position { return (Kind::Split, far); }
            if champ_top_side {
                if far == Line::Top { (Kind::Press, Line::Mid) } else { (Kind::Press, pick(Line::Top)) }
            } else {
                if far == Line::Top { (Kind::Press, pick(Line::Bottom)) } else { (Kind::Press, Line::Mid) }
            }
        }
        MorgardUseStrategy::Split131 { position1, position2 } => {
            if my_pos == position1 || (my_pos == position2 && !pos1_champ_some) {
                (Kind::Split, Line::Top)
            } else if my_pos == position2 {
                (Kind::Split, Line::Bottom)
            } else {
                (Kind::Press, Line::Mid)
            }
        }
    }
}

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let counts: [i32; 3] = match case { 1 | 4 => [-1, 0, 9], 2 => [0, 0, 0], _ => [5, 2, -3] };
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = GameContext {
        pool: &pool, setting: &setting, macro_weights: &mw, map_setting: &ms,
        map: &map, champion_list: &champs, item_list: &items,
        ignore_minion: false, debug: false,
        tutorial: TutorialType::None, trace_level: TraceLevel::Off,
    };
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    game.set_tick(1000);
    for t in 0..2usize {
        game.world.strategy[t].morgard_use = match case {
            3 => MorgardUseStrategy::Split14 { position: Position::Jungle },
            4 => MorgardUseStrategy::Split14 { position: Position::Mid },
            5 | 6 => MorgardUseStrategy::Split131 { position1: Position::Top, position2: Position::Support },
            _ => MorgardUseStrategy::Gather,
        };
    }
    match case { 3 => { game.mode.jungle_runner.epic.next_respawn_tick = 5000; game.mode.jungle_runner.serpen.next_respawn_tick = 100; }
                 4 => { game.mode.jungle_runner.epic.next_respawn_tick = 100; game.mode.jungle_runner.serpen.next_respawn_tick = 5000; }
                 _ => {} }
    let mut cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    if case == 6 { cache.player_champion[0][0] = None; cache.player_champion[1][0] = None; }
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    // blackboard[team].{top,mid,bottom}_minion_state.minion_count = +0x20 / +0x48 / +0x70 (tcxdict Blackboard)
    for t in 0..2usize {
        let p = &mut bb[t] as *mut Blackboard as *const u8;
        wr::<i32>(p, 0x20, counts[0]);
        wr::<i32>(p, 0x48, counts[1]);
        wr::<i32>(p, 0x70, counts[2]);
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let gm = game.get_game_mode();
    let moba = gm.as_moba().expect("moba");
    let mp = moba as *const MobaMode as *const u8;
    let epic_tick: usize = rd(mp, 0x1b0);
    let serpen_tick: usize = rd(mp, 0x1e0);
    println!("case\t{}\tcounts={:?}\ttowers={}\ttick={}\tepic_tick={}\tserpen_tick={}\tfar_line={}",
             case, counts, game.world.tower_ids.len(), game.tick(), epic_tick, serpen_tick,
             if epic_tick > serpen_tick { "Top" } else { "Bottom" });

    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut n = 0usize; let mut fired = 0usize; let mut gate_mism = 0usize; let mut pred_mism = 0usize;
    let mut kinds = [0usize; 2]; let mut lines = [0usize; 3]; let mut strat = [0usize; 3];
    let mut top_side_seen = [0usize; 2];
    let mut dbgf: DebugFrameData = Default::default();
    for seed in 0u64..600 {
        for t in 0..2usize {
            for p in 0..5usize {
                let player = game.get_player_by_position(t, poss[p]).expect("player");
                let (cx, cy) = match cache.player_champion[t][p] { Some(e) => (e.x, e.y), None => (0, 0) };
                let top_side = game_core::is_top_side(&ctx, cx, cy);
                top_side_seen[top_side as usize] += 1;
                // (a) 독립 게이트 계산
                let mut r0 = rand::rngs::StdRng::seed_from_u64(seed * 1000 + (t * 5 + p) as u64);
                let v = r0.gen_range(0..100);
                let expect_on = v < 21;
                // (b) 같은 rng 상태로 전략 재현
                let strategy = player.strategy(&mut r0, &game as &dyn AbstractGame);
                // 실제 호출
                let mut rng = rand::rngs::StdRng::seed_from_u64(seed * 1000 + (t * 5 + p) as u64);
                let mut tp: TeamPlan = Default::default();
                tp.handle_press_epic(1, &mut rng, player, &data, &mut dbgf);
                let tpp = &tp as *const TeamPlan as *const u8;
                let len: usize = rd(tpp, 0xd0);
                let on = len > 0;
                n += 1;
                if on != expect_on { gate_mism += 1;
                    if gate_mism <= 5 { println!("GATE_MISM\tseed={}\tt={}\tp={}\tv={}\ton={}", seed, t, p, v, on); }
                }
                if !on { continue; }
                fired += 1;
                let cp: *const u8 = rd(tpp, 0xc8);
                let tag: u8 = rd(cp, 0);
                let ln: u8 = rd(cp, 1);
                let aux: usize = rd(cp, 8);
                let obj_tag: u8 = rd(tpp, 0x41f);
                let obj_line: u8 = rd(tpp, 0x420);
                let kind = if tag == 21 { Kind::Press } else if tag == 20 { Kind::Split } else { panic!("tag {}", tag) };
                let line = match ln { 0 => Line::Top, 1 => Line::Mid, 2 => Line::Bottom, _ => panic!("line {}", ln) };
                kinds[kind as usize] += 1; lines[line as usize] += 1;
                let si = match strategy.morgard_use { MorgardUseStrategy::Gather => 0, MorgardUseStrategy::Split14{..} => 1, _ => 2 };
                strat[si] += 1;
                let pos1_some = match strategy.morgard_use {
                    MorgardUseStrategy::Split131 { position1, .. } => cache.player_champion[t][pos_idx(position1)].is_some(),
                    _ => true };
                let (pk, pl) = spec_predict(&strategy, poss[p], top_side, counts, epic_tick, serpen_tick, pos1_some);
                let exp_obj_tag = if pk == Kind::Press { 5u8 } else { 6u8 };
                let okk = pk == kind && pl == line && len == 1 && aux == 0
                       && obj_tag == exp_obj_tag && obj_line == line_u8(line);
                if !okk { pred_mism += 1;
                    if pred_mism <= 10 {
                        println!("PRED_MISM\tseed={}\tt={}\tp={}\tstrat={:?}\ttop_side={}\tactual=({:?},{:?}) aux={} obj=({},{})\tpred=({:?},{:?})",
                                 seed, t, p, strategy.morgard_use, top_side, kind, line, aux, obj_tag, obj_line, pk, pl);
                    }
                }
            }
        }
    }
    println!("calls\t{}\tfired\t{}\t({:.2}%)\tgate_mism\t{}\tpred_mism\t{}", n, fired, 100.0 * fired as f64 / n as f64, gate_mism, pred_mism);
    println!("kinds Press/Split\t{:?}\tlines Top/Mid/Bottom\t{:?}\tstrat Gather/Split14/Split131\t{:?}\ttop_side false/true(호출수)\t{:?}",
             kinds, lines, strat, top_side_seen);
}
