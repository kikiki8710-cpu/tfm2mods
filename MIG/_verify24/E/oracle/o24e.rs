#![allow(unused, dead_code, non_snake_case)]
//! 24차 배치E 오라클 — 184 SmallActionRecall::get_input (define hidden → link_name 직접 링크).
//!  한 프로세스 = 한 케이스(argv). 세계 = TEMPLATE mkgame(실전 세팅 · 타워 16). 엔티티 변조 = raw write_volatile(함정 ⑦).
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify24/E/oracle/o24e.rs
//!  실행: o24e.exe 184 <dist> <speed> <version> <well(0/1)> <call2(0/1)>   (드라이버 = run24e.py)
//!   dist   = 내 챔피언(팀0 Top)을 우물 좌표 healp=(32000,928000) 에서 +x 방향으로 dist 만큼 떨어뜨린다
//!   speed  = Entity.stat_cached.move_speed(+0x640)
//!   well=1 → 챔피언을 우물 사각형 안(healp 자체)에 둔다(L691 None 경로)
//!   call2=1 → 같은 self 로 get_input 을 한 번 더 부른다(path_finder 재사용 · drop 경로 관찰)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }
fn ep(e: *const Entity) -> *const u8 { e as *const u8 }
fn arg(a: &[String], i: usize, d: i64) -> i64 { a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d) }

extern "Rust" {
    #[link_name = "_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB5_17SmallActionRecall9get_input"]
    fn recall_get_input(s: &mut game_ai::SmallActionRecall, version: usize, rnd: &mut rand::rngs::StdRng,
                        player: &PlayerState, data: &OperationData, ps: &PositioningScoreData,
                        dbg: &mut DebugFrameData) -> Option<Input>;
    #[link_name = "_RNvNtNtCshdEBA0ozCnw_7game_ai12small_action4cast14is_safe_recall"]
    fn is_safe_recall(version: usize, rnd: &mut rand::rngs::StdRng, player: &PlayerState, data: &OperationData,
                      ps: &PositioningScoreData) -> bool;
}

fn snap(s: &game_ai::SmallActionRecall) -> [u8; 136] { unsafe { std::ptr::read(s as *const _ as *const [u8; 136]) } }
fn hex_diff(a: &[u8], b: &[u8]) -> String {
    let mut out = String::new();
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() { if x != y { out += &format!("+{:#x}:{:02x}->{:02x} ", i, x, y); } }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let which = a.get(1).cloned().unwrap_or_default();
    if a.len() > 99 { let _ = game_ai::v22_lane_tower_pressure_attack_allowed as *const (); }
    let setting = real_setting();
    let ok = setting_ok(&setting);
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
    let mut rnd = rand::rngs::StdRng::seed_from_u64(9);

    if which == "184" {
        let dist = arg(&a, 2, 480000) as u64; let speed = arg(&a, 3, 1000) as u64;
        let version = arg(&a, 4, 55) as usize; let well = arg(&a, 5, 0); let call2 = arg(&a, 6, 0);
        let tick = arg(&a, 7, 1000) as usize; let enemy = arg(&a, 8, 0);
        game.set_tick(tick);
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let cp = ep(champ);
        // 우물 사각형(MapDef+0x6d70 · [(lx,ly,rx,ry);2])
        let mp = &map as *const _ as *const u8;
        let (lx, ly, rx, ry): (u64, u64, u64, u64) = (rd(mp, 0x6d70), rd(mp, 0x6d78), rd(mp, 0x6d80), rd(mp, 0x6d88));
        let (hx, hy) = (32000u64, 928000u64);
        let (cx, cy) = if well == 1 { (hx, hy) } else { (hx + dist, hy) };
        wr(cp, 0x660, cx); wr(cp, 0x668, cy);
        wr(cp, 0x640, speed);
        // 적 챔피언 5명은 자기 우물(원래 위치) 그대로 — 가시 적 0. enemy=1 이면 적 Top 을 내 옆 50000 에 두고 Visible
        if enemy == 1 {
            let ec = cache.player_champion[1][0].expect("enemy champ"); let ecp = ep(ec);
            wr(ecp, 0x660, cx + 50000); wr(ecp, 0x668, cy);
            unsafe { std::ptr::write_volatile(&mut (*(ecp as *mut Entity)).visible_state[0], VisibleState::Visible); }
        }
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps: PositioningScoreData = Default::default();
        let mut dbg: DebugFrameData = Default::default();
        let d = game_core::utils::distance(cx, cy, hx, hy);
        let time = if speed == 0 { u64::MAX } else { d / speed };
        let thr = (setting.tick_per_second * 3 + 60) as u64;
        let safe = { let mut r2 = rand::rngs::StdRng::seed_from_u64(9); unsafe { is_safe_recall(version, &mut r2, player, &data, &ps) } };
        let mut s = game_ai::SmallActionRecall::new(&data, player, version);
        let before = snap(&s);
        let rnd_before: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let r = unsafe { recall_get_input(&mut s, version, &mut rnd, player, &data, &ps, &mut dbg) };
        let rnd_after: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
        let after = snap(&s);
        let tag: i64 = rd(&r as *const _ as *const u8, 0);
        let in_well = lx <= cx && cx <= rx && ly <= cy && cy <= ry;
        // 명세(현행 문면) 예측: safe && time < thr → Return ;  IR 예측: safe && time >= thr → Return
        let spec_ret = safe && time < thr; let ir_ret = safe && time >= thr;
        println!("RESULT184\tenemy={}\tdist={}\tspeed={}\tver={}\twell={}\tin_well={}\tfountain=({},{},{},{})\td={}\ttime={}\tthr={}\tsafe={}\tgame={:?}\ttag={}\tspec_ret={}\tir_ret={}\trnd_changed={}\tself_diff={}\tgoal=({},{})\tcommitted={}\tpf_tag={}\tprog=({},{})\tgoal_risk={}",
                 enemy, dist, speed, version, well, in_well, lx, ly, rx, ry, d, time, thr, safe, r, tag, spec_ret, ir_ret,
                 rnd_before != rnd_after, hex_diff(&before, &after),
                 rd::<u64>(after.as_ptr(), 0x50), rd::<u64>(after.as_ptr(), 0x58), rd::<u8>(after.as_ptr(), 0x80), rd::<u8>(after.as_ptr(), 0x45),
                 rd::<u64>(after.as_ptr(), 0x70), rd::<u64>(after.as_ptr(), 0x78), rd::<i64>(after.as_ptr(), 0x68));
        if call2 == 1 {
            let path_ptr: u64 = rd(after.as_ptr(), 0x28); let pv_ptr: u64 = rd(after.as_ptr(), 0x30);
            let r2 = unsafe { recall_get_input(&mut s, version, &mut rnd, player, &data, &ps, &mut dbg) };
            let rnd_after2: [u8; 320] = unsafe { std::ptr::read(&rnd as *const _ as *const [u8; 320]) };
            let after2 = snap(&s);
            println!("RESULT184b\tgame2={:?}\trnd_changed={}\tself_diff={}\tpf_tag={}\tpath_ptr_same={}\tpv_ptr_same={}",
                     r2, rnd_after != rnd_after2, hex_diff(&after, &after2), rd::<u8>(after2.as_ptr(), 0x45),
                     rd::<u64>(after2.as_ptr(), 0x28) == path_ptr, rd::<u64>(after2.as_ptr(), 0x30) == pv_ptr);
        }
        return;
    }
    if which == "pe" {
        // POS_EVAL_CACHE 히트/스테일 실험: 같은 (id,x,y,purpose,version)·같은 tick·같은 seed 로 2회 부르면 두 번째는 캐시값(세계 변화 무시).
        //   tick 을 바꾸면 슬롯 tick 불일치 → 재계산. argv: pe <mode 0=Recall/1=AttackStance>
        let purpose = match arg(&a, 2, 0) { 1 => game_ai::PositionEvalPurpose::AttackStance, _ => game_ai::PositionEvalPurpose::Recall };
        let version = 55usize;
        game.set_tick(1000);
        let gp = &game as *const Game as *mut Game;
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let bb: [Blackboard; 2] = [Default::default(), Default::default()];
        let player = game.get_player_by_position(0, Position::Top).expect("player");
        let champ = cache.player_champion[0][0].expect("champ");
        let cp = ep(champ);
        wr(cp, 0x660, 480000u64); wr(cp, 0x668, 480000u64);
        let tower = cache.top_tower[1].or(cache.mid_tower[1]).or(cache.bottom_tower[1]).expect("enemy tower");
        let tp = ep(tower);
        let (t0x, t0y): (u64, u64) = (rd(tp, 0x660), rd(tp, 0x668));
        // 타워에 공격력 부여(22차 A 수법: Entity+0x490 Arc<dyn EffectType> 교체 · 옛 Arc 누수)
        { let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(500, 100));
          let raw: [usize; 2] = unsafe { std::mem::transmute(arc) }; wr(tp, 0x490, raw[0]); wr(tp, 0x498, raw[1]); }
        // 적 챔피언(Top)도 셀 옆에 두고 Visible + 공격력
        let ec = cache.player_champion[1][0].expect("enemy champ"); let ecp = ep(ec);
        { let arc: Arc<dyn EffectType> = Arc::new(TowerAttackEffect::new(300, 100));
          let raw: [usize; 2] = unsafe { std::mem::transmute(arc) }; wr(ecp, 0x490, raw[0]); wr(ecp, 0x498, raw[1]); }
        unsafe { std::ptr::write_volatile(&mut (*(ecp as *mut Entity)).visible_state[0], VisibleState::Visible); }
        wr(ecp, 0x660, 100000u64); wr(ecp, 0x668, 100000u64);
        let data = OperationData::new(&cache, &ctx, &bb);
        let ps: PositioningScoreData = Default::default();
        let (xi, yi) = (15i64, 15i64);   // 셀 (15,15) 중심 = (496000,496000)
        let dump = |s: &PositioningScore| format!("risk={} tower_risk={} gain={} gain_me={} adjust={} unseen={} traj={} ptraj={}",
            s.risk, s.tower_risk, s.gain, s.gain_me, s.adjust, s.unseen_champ_threat, s.on_trajectory, s.on_periodic_trajectory);
        let s1 = game_ai::position_score_at_cell(version, player, &data, &ps, xi, yi, purpose);
        // 세계 변화: 적 타워를 셀 중심으로 옮긴다(tower_risk 가 달라져야 함)
        wr(tp, 0x660, 496000u64); wr(tp, 0x668, 496000u64);
        wr(ecp, 0x660, 500000u64); wr(ecp, 0x668, 500000u64);
        let s2 = game_ai::position_score_at_cell(version, player, &data, &ps, xi, yi, purpose);       // 같은 tick → 캐시 히트 기대(=s1)
        unsafe { (*gp).set_tick(1001); }
        let s3 = game_ai::position_score_at_cell(version, player, &data, &ps, xi, yi, purpose);       // tick 변경 → 재계산 기대(≠s1)
        let s4 = game_ai::position_score_at_cell(version, player, &data, &ps, xi, yi, purpose);       // 재히트(=s3)
        unsafe { (*gp).set_tick(1000); }
        let s5 = game_ai::position_score_at_cell(version, player, &data, &ps, xi, yi, purpose);       // tick 1000 으로 복귀 — 슬롯은 1001 로 덮였으므로 재계산(=s3 값)
        println!("RESULTPE\tpurpose={:?}\ttower0=({},{})\n s1(tick1000,tower far) {}\n s2(tick1000,tower moved) {}\n s3(tick1001) {}\n s4(tick1001) {}\n s5(tick1000 again) {}\n hit_stale(s2==s1)={}\trecalc(s3!=s1)={}\ts4==s3={}\ts5==s3={}",
                 purpose, t0x, t0y, dump(&s1), dump(&s2), dump(&s3), dump(&s4), dump(&s5),
                 dump(&s2) == dump(&s1), dump(&s3) != dump(&s1), dump(&s4) == dump(&s3), dump(&s5) == dump(&s3));
        return;
    }
    println!("unknown fn");
}
