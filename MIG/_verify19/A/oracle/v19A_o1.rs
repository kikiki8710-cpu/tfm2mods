#![allow(unused, dead_code, non_snake_case)]
//! 19차 배치A 오라클 1 — `#77 DefenseNexusPlan::sub_plan`(pub) 를 **실행**으로 확인한다.
//!  판정 축(명세 logic L82~L102):
//!   (a) L95 비위험: `hp_ratio < 31 || (hp < max && in_fountain)` → Recall(5) / 아니면 DefenseNexus(17)
//!   (b) L92 위험:   `hp_ratio > 20 || in_fountain` → DefenseNexus / 아니면 Recall
//!   (c) danger = (existing_lines_weak && nexus_near_enemy_champion) || nexus_under_direct_attack
//!       - existing_lines_weak: tutorial=JungleOnly → valid_lines=[] → all()=true (명세 L54 표)
//!         tutorial=None → 3라인 전부 front_minion 이 넥서스 120000 이내여야 true (blackboard[1-team] 0x0/0x28/0x50)
//!       - nexus_near_enemy_champion: 적 챔프 ↔ 넥서스 distance_sq < 14400000001 (=120000²+1)
//!   (d) version>1 && goal_data.heal_commit → Recall 즉시 (GoalData+0xf0)
//!  TLS: version>1 && !heal_commit 경로만 LAST_STAND_MEMO(seed,tick) 를 탄다 → 그 경로는 재지 않는다.
//!  엔티티 변경 = `game.world.entity.get_mut(id)`(&mut Entity, 15차 A 전례) 후 cache 재생성.
//! 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify19/A/oracle/v19A_o1.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

use game_ai::plan_legacy::old as old;
use game_ai::GoalData;

fn rd<T: Copy>(base: *const u8, off: usize) -> T {
    unsafe { std::ptr::read_unaligned(base.add(off) as *const T) }
}
fn wr<T: Copy>(base: *const u8, off: usize, v: T) {
    unsafe { std::ptr::write_unaligned(base.add(off) as *mut u8 as *mut T, v) }
}

fn mkctx<'a>(pool: &'a bumpalo::Bump, setting: &'a GameSetting, mw: &'a MacroWeights, ms: &'a MapSetting,
             map: &'a MapDef, champs: &'a Vec<String>, items: &'a Vec<Box<dyn ItemInfo>>, tut: TutorialType) -> GameContext<'a, 'a> {
    GameContext {
        pool, setting, macro_weights: mw, map_setting: ms,
        map, champion_list: champs, item_list: items,
        ignore_minion: false, debug: false,
        tutorial: tut, trace_level: TraceLevel::Off,
    }
}

const MAXHP: usize = 1000;

fn main() {
    let setting = real_setting();
    let ok = setting_ok(&setting);
    let ms: MapSetting = Default::default();
    let map = MapDef::moba(&setting);
    let pool = bumpalo::Bump::new();
    let mw: MacroWeights = Default::default();
    let champs: Vec<String> = Vec::new();
    let items: Vec<Box<dyn ItemInfo>> = Vec::new();
    let ctx = mkctx(&pool, &setting, &mw, &ms, &map, &champs, &items, TutorialType::None);
    let ctx_jo = mkctx(&pool, &setting, &mw, &ms, &map, &champs, &items, TutorialType::JungleOnly);
    let mut game = mkgame(&setting, &ms, &map, &ctx);
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let mut dbg: DebugFrameData = Default::default();
    let plan = old::DefenseNexusPlan::new(0);

    // 기준 좌표: 넥서스(팀0)·샘 rect(MapDef+0x6d70, 팀0)
    let (nx, ny, champ_id, enemy_id, max_hp, ex0, ey0) = {
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let nexus: &Entity = rd::<Option<&Entity>>(&cache as *const _ as *const u8, 0x170).expect("nexus[0]");
        let c = cache.player_champion[0][0].unwrap();
        let e = cache.player_champion[1][0].unwrap();
        (nexus.x, nexus.y, c.id, e.id, MAXHP, e.x, e.y)
    };
    let mp = &map as *const MapDef as *const u8;
    let (lx, ly, rx, ry): (u64, u64, u64, u64) = (rd(mp, 0x6d70), rd(mp, 0x6d78), rd(mp, 0x6d80), rd(mp, 0x6d88));
    println!("towers\t{}\tnexus0=({},{})\tfountain0=({},{})-({},{})\tchamp_id={}\tenemy_id={}\tmax_hp={}\tenemy0=({},{})",
             game.world.tower_ids.len(), nx, ny, lx, ly, rx, ry, champ_id, enemy_id, max_hp, ex0, ey0);
    let in_f = |x: u64, y: u64| lx <= x && x <= rx && ly <= y && y <= ry;
    let far = (480_000u64, 480_000u64);  // 맵 중앙(샘 밖)
    let fount = ((lx + rx) / 2, (ly + ry) / 2);
    println!("fount_center={:?}\tin_f={}\tfar={:?}\tin_f={}", fount, in_f(fount.0, fount.1), far, in_f(far.0, far.1));

    // 케이스 표: (이름, version, tutorial_jo, hp_pct(또는 절대 -k), 내 위치 in_fountain?, 적 dx from nexus(None=원위치), front_minion 3개 세팅?, heal_commit, 기대 태그)
    struct Case { name: &'static str, ver: usize, jo: bool, hp: i64, in_f: bool, edx: Option<u64>, fm: bool, heal: bool, exp: i64 }
    let cases = [
        Case { name: "A_base_full_fount",        ver: 1, jo: false, hp: 100, in_f: true,  edx: None,        fm: false, heal: false, exp: 17 },
        Case { name: "B_hp30_fount",             ver: 1, jo: false, hp: 30,  in_f: true,  edx: None,        fm: false, heal: false, exp: 5 },
        Case { name: "C1_hp30_far",              ver: 1, jo: false, hp: 30,  in_f: false, edx: None,        fm: false, heal: false, exp: 5 },
        Case { name: "C2_hp31_far",              ver: 1, jo: false, hp: 31,  in_f: false, edx: None,        fm: false, heal: false, exp: 17 },
        Case { name: "D1_hpmax-1_fount",         ver: 1, jo: false, hp: -1,  in_f: true,  edx: None,        fm: false, heal: false, exp: 5 },
        Case { name: "D2_hpmax-1_far",           ver: 1, jo: false, hp: -1,  in_f: false, edx: None,        fm: false, heal: false, exp: 17 },
        // danger 경로 (JungleOnly → existing_lines_weak=true)
        Case { name: "E1_jo_enemy50k_hp25_far",  ver: 1, jo: true,  hp: 25,  in_f: false, edx: Some(50000),  fm: false, heal: false, exp: 17 },
        Case { name: "E2_jo_enemy50k_hp20_far",  ver: 1, jo: true,  hp: 20,  in_f: false, edx: Some(50000),  fm: false, heal: false, exp: 5 },
        Case { name: "E3_jo_enemy50k_hp21_far",  ver: 1, jo: true,  hp: 21,  in_f: false, edx: Some(50000),  fm: false, heal: false, exp: 17 },
        Case { name: "E4_jo_enemy50k_hp20_fount",ver: 1, jo: true,  hp: 20,  in_f: true,  edx: Some(50000),  fm: false, heal: false, exp: 17 },
        Case { name: "E5_jo_enemy120000_hp25",   ver: 1, jo: true,  hp: 25,  in_f: false, edx: Some(120000), fm: false, heal: false, exp: 17 },
        Case { name: "E6_jo_enemy120001_hp25",   ver: 1, jo: true,  hp: 25,  in_f: false, edx: Some(120001), fm: false, heal: false, exp: 5 },
        Case { name: "E7_jo_enemyfar_hp25",      ver: 1, jo: true,  hp: 25,  in_f: false, edx: Some(400000), fm: false, heal: false, exp: 5 },
        // tutorial None: front_minion 없으면 existing_lines_weak=false → 비위험
        Case { name: "F1_none_enemy50k_hp25",    ver: 1, jo: false, hp: 25,  in_f: false, edx: Some(50000),  fm: false, heal: false, exp: 5 },
        Case { name: "F2_none_enemy50k_fm_hp25", ver: 1, jo: false, hp: 25,  in_f: false, edx: Some(50000),  fm: true,  heal: false, exp: 17 },
        // heal_commit 게이트
        Case { name: "G1_v2_heal_full_fount",    ver: 2, jo: false, hp: 100, in_f: true,  edx: None,        fm: false, heal: true,  exp: 5 },
        Case { name: "G2_v1_heal_full_fount",    ver: 1, jo: false, hp: 100, in_f: true,  edx: None,        fm: false, heal: true,  exp: 17 },
    ];
    // nexus_under_direct_attack 반경 스윕(open[3] 보조 관측): 적 챔프 1명을 넥서스 +dx 에 두고 under 를 본다
    {
        println!("\nunder_sweep\tdx\tunder");
        for dx in [0u64, 50000, 119999, 120000, 120001, 150000, 200000, 210000, 219999, 220000, 220001, 230000, 240000, 250000, 400000] {
            { let en = game.world.entity.get_mut(enemy_id).unwrap(); en.x = nx + dx; en.y = ny; }
            let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
            let bb: [Blackboard; 2] = [Default::default(), Default::default()];
            let data = OperationData::new(&cache, &ctx, &bb);
            let ps = game.get_player_by_position(0, Position::Top).unwrap();
            println!("under_sweep\t{}\t{}", dx, old::nexus_under_direct_attack(ps, &data));
        }
        { let en = game.world.entity.get_mut(enemy_id).unwrap(); en.x = ex0; en.y = ey0; }
    }
    let mut n = 0usize; let mut bad = 0usize;
    println!("\ncase\tver\ttut\thp\tin_f\tedx\tfm\theal\tunder\tpred_path\tgame\texp(spec)\tverdict");
    for c in cases.iter() {
        // 세계 세팅
        {
            let me = game.world.entity.get_mut(champ_id).unwrap();
            me.stat_cached.hp = MAXHP;   // ★기본 챔프는 stat_cached.hp==1 (hp% 전부 0 붕괴) → 실전 규모로
            me.hp = if c.hp < 0 { (max_hp as i64 + c.hp) as usize } else { max_hp * (c.hp as usize) / 100 };
            let (x, y) = if c.in_f { fount } else { far };
            me.x = x; me.y = y;
            let en = game.world.entity.get_mut(enemy_id).unwrap();
            match c.edx { Some(dx) => { en.x = nx + dx; en.y = ny; }, None => { en.x = ex0; en.y = ey0; } }
        }
        let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
        let mut bb: [Blackboard; 2] = [Default::default(), Default::default()];
        if c.fm {
            let p = &mut bb[1] as *mut Blackboard as *const u8;   // 적팀 판 = blackboard[1-team]
            for off in [0x0usize, 0x28, 0x50] { wr::<i64>(p, off, 1); wr::<usize>(p, off + 8, enemy_id); }
        }
        let cx = if c.jo { &ctx_jo } else { &ctx };
        let data = OperationData::new(&cache, cx, &bb);
        let mut gd: GoalData = Default::default();
        wr::<u8>(&mut gd as *mut GoalData as *const u8, 0xf0, if c.heal { 1 } else { 0 });
        let ps = game.get_player_by_position(0, Position::Top).unwrap();
        let under = old::nexus_under_direct_attack(ps, &data);
        let me = cache.player_champion[0][0].unwrap();
        // ── 명세 logic 독립 재구현(수법 ⓓ) — under 는 실측값을 그대로 쓴다(콜리, 담당 밖)
        let hp_ratio = me.hp * 100 / me.stat_cached.hp;
        let infount = in_f(me.x, me.y);
        let en = cache.player_champion[1][0].unwrap();
        let near = { let dx = en.x.abs_diff(nx); let dy = en.y.abs_diff(ny); dx*dx + dy*dy < 14400000001 };
        let lines_weak = if c.jo { true } else { c.fm && near };   // fm 은 near 인 적 챔프 id 를 3라인에 넣으므로 near 와 동치
        let danger = (lines_weak && near) || under;
        let (pred, path): (i64, &str) = if c.ver > 1 && c.heal { (5, "L83heal") }
            else if danger { if hp_ratio > 20 || infount { (17, "L92") } else { (5, "L92") } }
            else { if hp_ratio < 31 || (me.hp < me.stat_cached.hp && infount) { (5, "L95") } else { (17, "L95") } };
        let sp = plan.sub_plan(c.ver, &mut rnd, ps, &data, &gd, &mut dbg);
        let tag: i64 = rd(&sp as *const _ as *const u8, 0);
        n += 1; if tag != pred { bad += 1; }
        println!("{}\t{}\t{}\t{}({})\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}({:?})\t{}\t{}",
                 c.name, c.ver, if c.jo { "JO" } else { "None" }, c.hp, hp_ratio,
                 infount, c.edx, c.fm, c.heal, under, path, tag, sp, pred,
                 if tag == pred { "MATCH" } else { "MISMATCH" });
    }
    println!("\nTOTAL\t{}\tMISMATCH\t{}\tsetting_ok={}", n, bad, ok);
}
