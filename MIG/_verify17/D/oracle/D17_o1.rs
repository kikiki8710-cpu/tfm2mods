#![allow(unused, dead_code, non_snake_case)]
//! 17차 배치 D 오라클 #1 — `_verify3\TEMPLATE.rs` 를 그대로 따른다(real_setting · init_tower 안 부름).
//! 대상: 56 `PassiveLinePlan::sub_plan`(pub · 타입 pub · `PassiveLinePlan::new(LineType)` pub · `TeamPlan: Default`)
//! 케이스당 프로세스 1개(argv[1]). 각 케이스는 self/TeamPlan/Blackboard 를 raw write 로 조립하고 반환 SubPlan(72B) 의
//! 태그(+0x0 i64)·페이로드(+0x8/+0x9/+0xa i8) 를 찍는다. 기대값은 명세 logic 을 독립적으로 손으로 계산한 것(수법 ⓓ).
//!
//!  C0  self.in_recall=1                                   → tag 5 Recall
//!  C1  v46_flee=1 · cover=0 · acute=1                     → tag 4 LineWait  · +8 = line(2 Bottom)
//!  C2  v46_flee=1 · cover=0 · acute=0                     → tag 3 LineSafe  · +8 = 2
//!  C3  v46_flee=1 · cover=1 (게이트 불통과) · 그 외 기본     → tag 2 LineDefense{style 0, line 2, action 2 Push}  (front_minion None)
//!  C4  기본(모두 0)                                        → tag 2 {0,2,2}
//!  C5  objective=Gank(8) line=Bottom · from_mid=2000       → tag 2 {0,2,1 Normal}   (정글러 미준비 · from_mid<2001)
//!  C6  objective=Gank(8) line=Bottom · from_mid=2001       → tag 2 {0,2,0 Pull}
//!  C7  objective=Gank(8) line=Top(라인 불일치) · from_mid=2001 → tag 2 {0,2,2 Push}  (갱크 대상 라인 아님 → 비갱크 경로)
//!  C8  objective=Dive(9) line=Bottom · from_mid=2001       → tag 2 {0,2,2 Push}  (Dive 는 gank_or_dive_here 만 true, is_gank_target_line 은 false)
//!  C9  front_minion=Some(자기 챔프 id) · 기본                → tag 2 {0,2,2}  (near_allies>=1 > can_near_enemy 0 → LineDefense)
//!  C10 self.line=Bottom · position=Jungle 플레이어로 호출    → tag 2 {1 Defensive,2,2}  (L899 line_style = position==Jungle)
use game_core::*;
use game_ai::plan_legacy::old::PassiveLinePlan;
use game_ai::plan_legacy::team_plan::TeamPlan;
use rand::SeedableRng;
use std::sync::Arc;

pub fn real_setting() -> GameSetting {
    let mut s: GameSetting = Default::default();
    s.width = 960000;
    s.height = 960000;
    s.respawn_tick = 300;
    s.respawn_growth = 30;
    s.respawn_growth_term = 1800;
    s.respawn_growth_per_level = 180;
    s.respawn_max = 2400;
    s.visible_distance = 130000;
    s.tick_per_second = 60;
    s.champion_radius = 10000;
    s.nexus_heal = 10;
    s.nexus_heal_2v2 = 10;
    s.nexus_heal_3v3 = 10;
    s.nexus_heal_tick = 24;
    s.nexus_heal_decay = 100;
    s.kill_exp = 30;
    s.kill_exp_growth = 30;
    s.assist_exp_ratio = 40;
    s.kill_gold = 300;
    s.assist_gold = 100;
    s.start_gold = 500;
    s.gold_per_second = 7;
    s.return_tick = 120;
    s.epic_minion_buff_duration = 5400;
    s.epic_minion_buff_range = 140000;
    s.tower_reduce_tick_2v2 = 5400;
    s.tower_reduce_tick_3v3 = 7200;
    s.gold_ratio_2v2 = 200;
    s.gold_ratio_3v3 = 150;
    s.exp_ratio_2v2 = 200;
    s.exp_ratio_3v3 = 150;
    s.tower_attack_disable_tick = 9999999;
    s.tower_attack_disable_tick_2v2 = 10800;
    s.tower_attack_disable_tick_3v3 = 14400;
    s.well_damage = 600;
    s.well_damage_tick = 30;
    s.top_hp_regen_percent = 1;
    s.jungle_execute_threshold = 700;
    s.jungle_move_speed_bonus = 20;
    s.mid_exp_bonus = 20;
    s.bottom_gold_bonus = 20;
    s.support_gold_reduction = 15;
    s.support_exp_reduction = 30;
    s.stamina_zero_debuff_percent = 30;
    s
}

pub fn setting_ok(s: &GameSetting) -> bool {
    let ok = s.width != 0 && s.height != 0 && s.tick_per_second != 0 && s.champion_radius != 0;
    println!("setting_ok\t{}\twidth={}\theight={}\ttps={}\tchamp_radius={}\tvisible={}",
             ok, s.width, s.height, s.tick_per_second, s.champion_radius, s.visible_distance);
    ok
}

pub fn mkgame(setting: &GameSetting, ms: &MapSetting, map: &MapDef, ctx: &GameContext) -> Game {
    let poss = [Position::Top, Position::Jungle, Position::Mid, Position::Bottom, Position::Support];
    let mut game = Game::new(1234u64, false, setting, ms, map);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut pid = 0usize;
    for t in 0..2usize {
        for p in 0..5usize {
            let ci: Arc<dyn ChampionInfo> = Arc::new(SwordmanChampionInfo::default());
            let mut st: AthleteStat = Default::default();
            st.judgement = 80;
            st.mental = 60;
            game.add_player(GamePlayer::new(pid, "p", t, poss[p], st, "swordman", ci, Vec::new()));
            pid += 1;
        }
    }
    game.start_game(&mut rnd, ctx);
    game
}

unsafe fn w8(base: *mut u8, off: usize, v: u8) { std::ptr::write(base.add(off), v); }
unsafe fn w64(base: *mut u8, off: usize, v: u64) { std::ptr::write_unaligned(base.add(off) as *mut u64, v); }
unsafe fn r8(base: *const u8, off: usize) -> u8 { std::ptr::read(base.add(off)) }
unsafe fn r64(base: *const u8, off: usize) -> u64 { std::ptr::read_unaligned(base.add(off) as *const u64) }

fn main() {
    let case: usize = std::env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(0);
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
    println!("towers\t{}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(), cache.twin_towers[0].len(), cache.twin_towers[1].len());
    println!("sizeof\tPassiveLinePlan={}\tTeamPlan={}\tBlackboard={}\tSubPlan={}",
             std::mem::size_of::<PassiveLinePlan>(), std::mem::size_of::<TeamPlan>(),
             std::mem::size_of::<Blackboard>(), std::mem::size_of::<game_ai::plan_legacy::sub_plan::SubPlan>());

    // ── 케이스 조립 ──
    let team = 0usize;
    let mut pos = Position::Bottom;
    let mut plan = PassiveLinePlan::new(LineType::Bottom);
    let mut tp: TeamPlan = Default::default();
    let champ_id = cache.player_champion[team][3].map(|e| e.id).unwrap_or(0);
    let jg = cache.player_champion[team][1].map(|e| (e.id, e.x, e.y)).unwrap_or((0, 0, 0));
    let ch = cache.player_champion[team][3].map(|e| (e.id, e.x, e.y)).unwrap_or((0, 0, 0));
    println!("champ\tid={}\tx={}\ty={}\tjungler\tid={}\tx={}\ty={}\tdist2={}", ch.0, ch.1, ch.2, jg.0, jg.1, jg.2,
             (ch.1.abs_diff(jg.1)).pow(2) + (ch.2.abs_diff(jg.2)).pow(2));
    unsafe {
        let pp = &mut plan as *mut PassiveLinePlan as *mut u8;
        let tpp = &mut tp as *mut TeamPlan as *mut u8;
        let bbp = &mut bb[team] as *mut Blackboard as *mut u8;
        println!("self.line@0x116\t{}\tin_recall@0x110\t{}", r8(pp, 0x116), r8(pp, 0x110));
        match case {
            0 => { w8(pp, 0x110, 1); }
            1 => { w8(pp, 0x112, 1); w8(pp, 0x115, 0); w8(pp, 0x113, 1); }
            2 => { w8(pp, 0x112, 1); w8(pp, 0x115, 0); w8(pp, 0x113, 0); }
            3 => { w8(pp, 0x112, 1); w8(pp, 0x115, 1); w8(pp, 0x113, 1); }
            4 => {}
            5 => { w8(tpp, 0x41f, 8); w8(tpp, 0x420, 2); w64(bbp, 0x60, 2000); }
            6 => { w8(tpp, 0x41f, 8); w8(tpp, 0x420, 2); w64(bbp, 0x60, 2001); }
            7 => { w8(tpp, 0x41f, 8); w8(tpp, 0x420, 0); w64(bbp, 0x60, 2001); }
            8 => { w8(tpp, 0x41f, 9); w8(tpp, 0x420, 2); w64(bbp, 0x60, 2001); }
            9 => { w64(bbp, 0x50, 1); w64(bbp, 0x58, champ_id as u64); }   // bottom.front_minion = Some(champ_id)
            10 => { pos = Position::Jungle; }
            _ => {}
        }
        println!("tp.objective@0x41f\t{}\tline@0x420\t{}\tbb.bottom.from_mid@0x60\t{}\tfront_minion@0x50\t{}/{}",
                 r8(tpp, 0x41f), r8(tpp, 0x420), r64(bbp, 0x60), r64(bbp, 0x50), r64(bbp, 0x58));
    }
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, pos).unwrap();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(11);
    let mut dbg: DebugFrameData = Default::default();
    let sp = plan.sub_plan(58, &mut rnd, player, &data, &tp, &mut dbg);
    unsafe {
        let p = &sp as *const _ as *const u8;
        println!("RESULT\tcase={}\ttag={}\t+8={}\t+9={}\t+10={}\t{:?}", case, r64(p, 0), r8(p, 8), r8(p, 9), r8(p, 10), sp);
    }
}
