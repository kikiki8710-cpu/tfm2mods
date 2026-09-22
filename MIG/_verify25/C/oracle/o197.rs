#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치C · 197 SerpenHuntSubPlan::action_candidates 오라클 (pub 직접 호출).
//!  케이스당 프로세스 1개(argv[1]). 빌드: sh C:/tfm2mods/MIG/_verify3/build.sh <이 파일>
//!  관측: sret Vec(len·원소 태그·필드) · &mut self(need_recall 1B) 전후 · rnd 320B 전후 diff · 원소 184B 헥스덤프
//!  케이스
//!   1  need_recall=1 · 세르펜 없음 · champ hp_ratio=29 (hp*100/max == 29) → L213 거짓 → Recall 1개 · self 1 유지
//!   2  need_recall=1 · hp_ratio=30 → L213 참 → self 0 · 이후 L226~ 경로(RunAway 예상)
//!   3  need_recall=0 · 세르펜 스폰(공격 있음) · champ 멀리 · champ.hp = 3*dmg     → L207 `hp > 3*dmg` 거짓 → self 1 · Recall
//!   4  need_recall=0 · 세르펜 스폰 · champ.hp = 3*dmg+1                            → self 0 · move 경로(Trace new_attack_range 예상)
//!   5  need_recall=0 · 세르펜 스폰 · champ 를 세르펜 옆(사거리 안)으로 · hp 최대 → objective_in_attack_range → Trace(margin) 또는 act Attack
//!   6  =5 + 거리 조정(dist+15000 < range · inner+dist > range) → L656 gen_range 소비 확인
//!   8  =6 + champ.attack_cooldown(+0xb0)=100 → can_attack=false → act_actions 비움 → move 경로 Trace(new_attack_range_margin) 관측(argv[3]=dist)
//!   7  need_recall=0 · 세르펜 없음 · 기본 → L661 RunAway(with_skill=false) 단독 → L506 clone 반환 (변형 baseline)
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

unsafe fn w8(p: *mut u8, off: usize, v: u8) { std::ptr::write_volatile(p.add(off), v); }
unsafe fn w64(p: *mut u8, off: usize, v: u64) { std::ptr::write_volatile(p.add(off) as *mut u64, v); }
unsafe fn r8(p: *const u8, off: usize) -> u8 { std::ptr::read_volatile(p.add(off)) }
unsafe fn r64(p: *const u8, off: usize) -> u64 { std::ptr::read_volatile(p.add(off) as *const u64) }
unsafe fn ri64(p: *const u8, off: usize) -> i64 { std::ptr::read_volatile(p.add(off) as *const i64) }
fn snap(p: *const u8, n: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(p, n).to_vec() } }
fn hexdump(b: &[u8]) -> String {
    let mut s = String::new();
    for (i, c) in b.chunks(16).enumerate() { s.push_str(&format!("\n      {:03x}: {}", i * 16, c.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(" "))); }
    s
}
fn mk_effect(range: u64) -> Effect {
    Effect { range, growth_range: 0, start_timing: 0, casting: CastingType::Targeting, target: CastingTarget::Enemy,
             ty: Arc::new(AttackEffect::new(100, 0)) as Arc<dyn EffectType>, attack_type: AttackType::BaseAttack }
}
fn setting_with_serpen() -> GameSetting {
    let mut s = real_setting();
    // 실전 game_setting.game_setting "serpen_jungle" 값
    s.serpen_jungle.stat.attack = 50; s.serpen_jungle.stat.hp = 5000; s.serpen_jungle.stat.defence = 60; s.serpen_jungle.stat.magic_resistance = 60;
    s.serpen_jungle.growth.attack = 20; s.serpen_jungle.growth.hp = 1000; s.serpen_jungle.growth.defence = 10; s.serpen_jungle.growth.magic_resistance = 10;
    s.serpen_jungle.attack.action_name = "attack".to_string();
    s.serpen_jungle.attack.attack_ratio = 100; s.serpen_jungle.attack.attack = 0; s.serpen_jungle.attack.range = 80000;
    s.serpen_jungle.attack.cooltime = 90; s.serpen_jungle.attack.duration = 24; s.serpen_jungle.attack.start_timing = 18;
    s.serpen_jungle.attack.cancelable = false; s.serpen_jungle.attack.attack_type = AttackType::BaseAttack;
    s.serpen_jungle.exp = 60; s.serpen_jungle.gold = 50; s.serpen_jungle.growth_gold = 10;
    s.serpen_jungle.respawn_tick = 7200; s.serpen_jungle.first_spawn_tick = 7200;
    s
}
const TAGN: [(&str, u8); 16] = [("RunAway", 3), ("Recall", 4), ("Around", 5), ("AroundHide", 6), ("AroundRegion", 7), ("AroundRunAway", 8), ("Positioning", 9), ("AroundPositionBush", 11), ("AroundBush", 12), ("LaneMinionPosition", 13), ("Trace", 14), ("Attack", 15), ("Skill", 16), ("Skill2", 17), ("Ult", 18), ("Stop", 19)];
fn tagname(t: u8) -> &'static str { for (n, v) in TAGN.iter() { if *v == t { return n; } } if t < 3 { "AroundPosition(untagged)" } else { "?" } }
fn describe(e: *const u8) -> String {
    unsafe {
        let t = r8(e, 0xb1);
        match t {
            3 => format!("RunAway start_tick={} goal=({},{}) end_delay={} goal_risk={} prog_best_dist_sq={:#x} prog_best_tick={} pf_tag@125={} with_skill@128={} with_ult={} dodge={} committed={}",
                        r64(e, 0), r64(e, 8), r64(e, 16), r64(e, 24), ri64(e, 32), r64(e, 40), r64(e, 48), r8(e, 125), r8(e, 128), r8(e, 129), r8(e, 130), r8(e, 131)),
            4 => format!("Recall pf_tag@69={} start_tick={} goal=({},{}) end_delay={} goal_risk={} prog_best_dist_sq={:#x} prog_best_tick={} committed@128={}",
                        r8(e, 69), r64(e, 72), r64(e, 80), r64(e, 88), r64(e, 96), ri64(e, 104), r64(e, 112), r64(e, 120), r8(e, 128)),
            14 => format!("Trace explicit_min_range_tag@0={} pf_tag@85={} start_tick={} target={} goal=({},{}) margin@120={} end_delay@128={} escape_commit_until={} avoid_tower@144={} attack_range_only@145={} abandoned={} avoid_zone={} dive_ignore={} last_escape_tag@149={}",
                        r64(e, 0), r8(e, 85), r64(e, 88), r64(e, 96), r64(e, 104), r64(e, 112), r64(e, 120), r64(e, 128), r64(e, 136), r8(e, 144), r8(e, 145), r8(e, 146), r8(e, 147), r8(e, 148), r8(e, 149)),
            15 | 16 | 17 | 18 => format!("{} start_tick={} target={} is_act@16={}", tagname(t), r64(e, 0), r64(e, 8), r8(e, 16)),
            19 => "Stop".to_string(),
            0 | 1 | 2 => format!("AroundPosition start_tick={} goal=({},{}) diff={} target=({},{}) radius@88={} end_delay@96={} pf_tag@173={} purpose@176={} outline@177={}",
                        r64(e, 0), r64(e, 8), r64(e, 16), ri64(e, 24), r64(e, 32), r64(e, 40), r64(e, 88), r64(e, 96), r8(e, 173), r8(e, 176), r8(e, 177)),
            _ => format!("tag {}", t),
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let case: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(7);
    let ver: usize = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(58);
    if a.len() > 99 { let _ = game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates as *const (); }
    let setting = setting_with_serpen();
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
    let team = 0usize;
    let mut rnd2 = rand::rngs::StdRng::seed_from_u64(9);
    let want_serpen = matches!(case, 3 | 4 | 5 | 6 | 8);
    let mut serpen_id: Option<usize> = None;
    if want_serpen {
        game.mode.jungle_runner.serpen.next_respawn_tick = 1;
        let mut spawned_at = 0usize;
        for k in 0..3000usize {
            let mut fd: Option<&mut GameFrameData> = None;
            game.run_tick(&ctx, &mut rnd2, &mut fd);
            if !game.mode.jungle_runner.serpen.live_list.is_empty() { spawned_at = k + 1; break; }
        }
        println!("serpen_spawned_after_ticks\t{}\tlive_list={:?}", spawned_at, game.mode.jungle_runner.serpen.live_list);
        serpen_id = game.mode.jungle_runner.serpen.live_list.get(0).copied();
    }
    let tick = 1000usize;
    game.set_tick(tick);
    // 챔피언(팀0 정글) 편집: attack_effect 주입 · 위치 · hp
    let player = game.get_player_by_position(team, Position::Jungle).expect("player");
    let pid = player.info.id;
    let cache0 = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let champ: &Entity = cache0.player_champion[team][1].expect("champ");
    let cp = champ as *const Entity as *mut u8;
    let serpen: Option<&Entity> = serpen_id.and_then(|id| game.get_entity_by_id(id));
    unsafe {
        std::ptr::write(cp.add(0x490) as *mut Option<Effect>, Some(mk_effect(80000)));
        let maxhp = r64(cp, 0x628);
        println!("champ\tid={}\tmax_hp={}\thp={}\tpos=({},{})\tlevel={}\tradius={}\tattack_effect_tag@0x4c0={}", r64(cp, 0x5c0), maxhp, r64(cp, 0x670), r64(cp, 0x660), r64(cp, 0x668), r64(cp, 0x5c8), r64(cp, 0x680), std::ptr::read_volatile(cp.add(0x4c0) as *const i32));
        match case {
            1 | 2 => {
                let maxhp = 1000u64; w64(cp, 0x628, maxhp);
                let hp: u64 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(if case == 1 { 299 } else { 300 });
                w64(cp, 0x670, hp);
                println!("set_hp\t{}\tmax={}\tratio={}", hp, maxhp, hp * 100 / maxhp);
            }
            3 | 4 => {
                let s = serpen.expect("serpen");
                let sp = s as *const Entity as *const u8;
                let eff = s.attack_effect.as_ref().expect("serpen attack_effect");
                let dmg = Effect::expected_damage_target(eff, &ctx, s as &dyn AbstractEntity, champ);
                let maxhp = 100000u64; w64(cp, 0x628, maxhp);
                let hp = if case == 3 { 3 * dmg as u64 } else { 3 * dmg as u64 + 1 };
                w64(cp, 0x670, hp.min(maxhp).max(1));
                println!("serpen\tid={}\tpos=({},{})\thp={}/{}\tatk_range={}\tdmg_to_champ={}\tset_champ_hp={}", r64(sp, 0x5c0), r64(sp, 0x660), r64(sp, 0x668), r64(sp, 0x670), r64(sp, 0x628), eff.range, dmg, r64(cp, 0x670));
            }
            5 | 6 | 8 => {
                if case == 8 { w64(cp, 0xb0, 100); println!("champ_attack_cooldown_set	100 (can_attack=false)"); }
                let s = serpen.expect("serpen");
                let sp = s as *const Entity as *const u8;
                let (sx, sy) = (r64(sp, 0x660), r64(sp, 0x668));
                let d: u64 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(if case == 5 { 30000 } else { 60000 });
                w64(cp, 0x660, sx + d); w64(cp, 0x668, sy);
                let maxhp = 100000u64; w64(cp, 0x628, maxhp); w64(cp, 0x670, maxhp);
                println!("serpen\tid={}\tpos=({},{})\thp={}/{}\tradius={}\tchamp_moved_to=({},{})\tdist={}", r64(sp, 0x5c0), sx, sy, r64(sp, 0x670), r64(sp, 0x628), r64(sp, 0x680), r64(cp, 0x660), r64(cp, 0x668), d);
            }
            _ => {}
        }
    }
    // 편집 후 캐시 재구성
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let player = game.get_player_by_position(team, Position::Jungle).expect("player");
    let mut sub = game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::default();
    let selfp = &mut sub as *mut _ as *mut u8;
    unsafe { w8(selfp, 0, if case == 1 || case == 2 { 1 } else { 0 }); }
    let self_before = unsafe { r8(selfp, 0) };
    let team_plan = game_ai::plan_legacy::team_plan::TeamPlan::default();
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let param: &game_ai::ScoreParameter = unsafe { &*spbuf.as_ptr() };
    let mut dbgf: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(77);
    let rb = snap(&rnd as *const _ as *const u8, 320);
    println!("before\tself={}\tversion={}\ttick={}", self_before, ver, tick);
    let res = sub.action_candidates(ver, &mut rnd, player, &data, param, &team_plan, &mut dbgf);
    let ra = snap(&rnd as *const _ as *const u8, 320);
    let rdiff = (0..320).filter(|&i| rb[i] != ra[i]).count();
    let self_after = unsafe { r8(selfp, 0) };
    println!("after\tself={}\tself_changed={}\tres_len={}\trnd_bytes_changed={}", self_after, self_after != self_before, res.len(), rdiff);
    for (i, e) in res.iter().enumerate() {
        let ep = e as *const _ as *const u8;
        let t = unsafe { r8(ep, 0xb1) };
        println!("elem[{}]\ttag={}({})\t{}", i, t, tagname(t), describe(ep));
        println!("   bytes{}", hexdump(&snap(ep, 184)));
    }
    let ninfo = dbgf.infos.len();
    println!("debug_infos_len\t{}", ninfo);
}
