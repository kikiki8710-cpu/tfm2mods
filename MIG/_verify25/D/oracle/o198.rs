#![allow(unused, dead_code, non_snake_case)]
//! 25차 배치D · 198 EpicHuntSubPlan::action_candidates 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv k=v). 세계 = TEMPLATE mkgame + (epic=1 이면 epic_jungle 스탯 주입 후 run_tick 으로 에픽 스폰).
//!  검증 표적: L203~L221 need_recall 갱신(극성·경계) 와 Recall 단일 후보 조기반환 · &mut self 쓰기 · 반환 variant 태그.
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify25/D/oracle/o198.rs
//!  실행: o198.exe nr=<0|1> epic=<0|1> focus=<none|me|other> hp=<n|3d|3d+1|d|d+1|d-1> max=<n> ticks=<n>
use game_core::*;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::collections::HashMap;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }
fn wr<T: Copy>(base: *const u8, off: usize, v: T) { unsafe { std::ptr::write_volatile(base.add(off) as *mut u8 as *mut T, v) } }

fn mkeff(damage: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 100000, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

fn args() -> HashMap<String, String> {
    let mut m = HashMap::new();
    for a in std::env::args().skip(1) {
        if let Some((k, v)) = a.split_once('=') { m.insert(k.to_string(), v.to_string()); }
    }
    m
}

fn main() {
    let a = args();
    let g = |k: &str, d: &str| a.get(k).cloned().unwrap_or(d.to_string());
    let nr0 = g("nr", "0") == "1";
    let want_epic = g("epic", "0") == "1";
    let focus = g("focus", "none");
    let hp_expr = g("hp", "500");
    let max_hp: usize = g("max", "1000").parse().unwrap();
    let ticks: usize = g("ticks", "30").parse().unwrap();
    let epic_dmg: usize = g("edmg", "137").parse().unwrap();

    let mut setting = real_setting();
    if want_epic {
        setting.epic_jungle.first_spawn_tick = 5;
        setting.epic_jungle.respawn_tick = 99999;
        setting.epic_jungle.stat.hp = 5000;
        setting.epic_jungle.stat.attack = 100;
        setting.epic_jungle.stat.move_speed = 1;
        setting.epic_jungle.attack.attack_ratio = 100;
        setting.epic_jungle.attack.attack = 100;
        setting.epic_jungle.attack.range = 20000;
        setting.epic_jungle.attack.cooltime = 60;
        setting.epic_jungle.attack.duration = 24;
        setting.epic_jungle.attack.start_timing = 16;
    }
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
    let mut rnd0 = rand::rngs::StdRng::seed_from_u64(99);
    if want_epic {
        for _ in 0..ticks { let mut fd: Option<&mut GameFrameData> = None; game.run_tick(&ctx, &mut rnd0, &mut fd); }
    }
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let gm: &dyn AbstractGame = &game;

    let team = 0usize; let pos = Position::Jungle;
    let player = game.get_player_by_position(team, pos).expect("player");
    let champ: &Entity = cache.player_champion[team][1].expect("champ");
    let cp = champ as *const Entity as *mut Entity;
    println!("champ\tid={}\thp={}\tmax={}\tatk_some={}\tx={}\ty={}", champ.id, champ.hp, champ.stat_cached.hp, champ.attack_effect.is_some(), champ.x, champ.y);
    // 챔피언 attack_effect 가 None 이면 objective_attack_range(L567) 의 unwrap 이 패닉하므로 채운다
    unsafe { if (*cp).attack_effect.is_none() { std::ptr::write(std::ptr::addr_of_mut!((*cp).attack_effect), Some(mkeff(50))); } }

    // 에픽
    let mut epic: Option<&Entity> = None;
    if let GameMode::Moba(m) = gm.get_game_mode() {
        let ll = &m.jungle_runner.epic.live_list;
        println!("epic_live_list\tlen={}\t{:?}", ll.len(), ll);
        if let Some(id) = ll.first() { epic = gm.get_entity_by_id(*id); }
    }
    let mut dmg: usize = 0;
    if let Some(e) = epic {
        let ep = e as *const Entity as *mut Entity;
        unsafe {
            // 에픽 공격 이펙트를 확정값으로 덮어 dmg 를 통제한다
            std::ptr::write(std::ptr::addr_of_mut!((*ep).attack_effect), Some(mkeff(epic_dmg)));
            if let EntityType::Epic { info } = &mut (*ep).ty {
                info.focused = match focus.as_str() { "me" => Some(champ.id), "other" => Some(champ.id + 1000), _ => None };
            }
        }
        let tag: i64 = rd(ep as *const u8, 0x68);
        let ftag: i64 = rd(ep as *const u8, 0x88); let fval: usize = rd(ep as *const u8, 0x90);
        println!("epic\tid={}\tty_tag={}\thp={}\tmax={}\tfocused_tag={}\tfocused_val={}\tx={}\ty={}", e.id, tag, e.hp, e.stat_cached.hp, ftag, fval, e.x, e.y);
        dmg = e.attack_effect.as_ref().unwrap().expected_damage_target(&ctx, e as &dyn AbstractEntity, champ);
        println!("dmg_before_hp\t{}", dmg);
    }
    // 챔피언 hp/max 설정
    let hp: usize = match hp_expr.as_str() {
        "3d" => 3 * dmg, "3d+1" => 3 * dmg + 1, "3d-1" => 3 * dmg - 1, "d" => dmg, "d+1" => dmg + 1, "d-1" => dmg - 1,
        s => s.parse().unwrap(),
    };
    wr::<usize>(cp as *const u8, 0x628, max_hp);
    wr::<usize>(cp as *const u8, 0x670, hp);
    if let Some(e) = epic {
        let d2 = e.attack_effect.as_ref().unwrap().expected_damage_target(&ctx, e as &dyn AbstractEntity, champ);
        println!("dmg_after_hp\t{}", d2);
        dmg = d2;
    }
    println!("champ_set\thp={}\tmax={}\tratio={}", champ.hp, champ.stat_cached.hp, if max_hp > 0 { champ.hp * 100 / max_hp } else { 0 });
    // near=1: 챔피언을 에픽 사거리 안(+dx)으로 옮긴다 → L569 objective_in_attack_range 경로
    if let Some(e) = epic {
        if g("near", "0") == "1" {
            let dx: u64 = g("dx", "15000").parse().unwrap();
            wr::<u64>(cp as *const u8, 0x660, e.x + dx);
            wr::<u64>(cp as *const u8, 0x668, e.y);
            println!("champ_moved\tx={}\ty={}\tepic_dist={}", champ.x, champ.y, champ.distance(e));
        }
    }
    // enemy=1: 적 챔피언 [1][1] 을 내 옆(+edx)으로 옮기고 team0 에 가시화 → L598 near_enemies · L607~ range_misjudge 롤(rnd 소비)
    if g("enemy", "0") == "1" {
        let en: &Entity = cache.player_champion[1][1].expect("enemy");
        let np = en as *const Entity as *mut Entity;
        let edx: u64 = g("edx", "30000").parse().unwrap();
        wr::<u64>(np as *const u8, 0x660, champ.x + edx);
        wr::<u64>(np as *const u8, 0x668, champ.y);
        wr::<i64>(np as *const u8, 0x38, 0);   // visible_state[0] = Visible
        unsafe { if (*np).attack_effect.is_none() { std::ptr::write(std::ptr::addr_of_mut!((*np).attack_effect), Some(mkeff(50))); } }
        println!("enemy_moved\tid={}\tx={}\ty={}\tdist={}\tvis0={}", en.id, en.x, en.y, champ.distance(en), rd::<i64>(np as *const u8, 0x38));
    }

    // 예측 (명세 logic L203~L221)
    let mut pred_nr = nr0;
    if !nr0 {
        if let Some(e) = epic {
            let focused: Option<usize> = if let EntityType::Epic { info } = &e.ty { info.focused } else { None };
            if (focused == Some(champ.id) && dmg * 3 >= champ.hp) || champ.hp <= dmg { pred_nr = true; }
        }
    } else {
        let ratio = champ.hp * 100 / champ.stat_cached.hp;
        if ratio > 29 { pred_nr = false; }
    }
    let pred_recall = pred_nr;

    // 호출
    let mut sp: game_ai::plan_legacy::sub_plan::EpicHuntSubPlan = Default::default();
    let spb = &mut sp as *mut _ as *mut u8;
    unsafe { std::ptr::write_volatile(spb, if nr0 { 1u8 } else { 0u8 }); }
    let mut spbuf = std::mem::MaybeUninit::<game_ai::ScoreParameter>::zeroed();
    let sp_ptr = spbuf.as_mut_ptr();
    unsafe { std::ptr::write(std::ptr::addr_of_mut!((*sp_ptr).positioning_score), Default::default()); }
    let param: &game_ai::ScoreParameter = unsafe { &*sp_ptr };
    let tp: game_ai::plan_legacy::team_plan::TeamPlan = Default::default();
    let mut dbg: DebugFrameData = Default::default();
    let mut rnd = rand::rngs::StdRng::seed_from_u64(7);
    let rnd_before = rnd.clone();
    let version = 60usize;
    let res = sp.action_candidates(version, &mut rnd, player, &data, param, &tp, &mut dbg);
    let nr_after = unsafe { std::ptr::read_volatile(spb) };
    let rnd_changed = rnd != rnd_before;
    println!("need_recall\tbefore={}\tafter={}\tpred={}\t{}", nr0 as u8, nr_after, pred_nr as u8, if (nr_after == 1) == pred_nr { "MATCH" } else { "MISMATCH" });
    println!("res_len\t{}", res.len());
    for (i, e) in res.iter().enumerate() {
        let b = e as *const game_ai::SmallActionPlay as *const u8;
        let tag: u8 = rd(b, 177);
        let mut hex = String::new();
        for k in 0..184 { hex.push_str(&format!("{:02x}", rd::<u8>(b, k))); }
        println!("elem[{}]\ttag={}\t{}", i, tag, hex);
        match tag {
            3 => println!("  RunAway\tstart={}\tgoal=({},{})\tend_delay={}\tpf_tag@125={}\twith_skill@128={}\twith_ult={}\tdodge={}\tcommitted={}",
                          rd::<u64>(b, 0), rd::<u64>(b, 8), rd::<u64>(b, 16), rd::<u64>(b, 24), rd::<u8>(b, 125), rd::<u8>(b, 128), rd::<u8>(b, 129), rd::<u8>(b, 130), rd::<u8>(b, 131)),
            4 => println!("  Recall\tpf_tag@69={}\tstart@72={}\tgoal=({},{})\tend_delay@96={}\trisk={}\tbest_dist={}\tbest_tick={}\tcommitted@128={}",
                          rd::<u8>(b, 69), rd::<u64>(b, 72), rd::<u64>(b, 80), rd::<u64>(b, 88), rd::<u64>(b, 96), rd::<i64>(b, 104), rd::<u64>(b, 112), rd::<u64>(b, 120), rd::<u8>(b, 128)),
            14 => println!("  Trace\tmin_range_tag@0={}\tpf_tag@85={}\tstart@88={}\ttarget@96={}\tgoal=({},{})\tmargin@120={}\tend_delay@128={}\tescape_until={}\tflags@144..150={:?}",
                          rd::<i64>(b, 0), rd::<u8>(b, 85), rd::<u64>(b, 88), rd::<u64>(b, 96), rd::<u64>(b, 104), rd::<u64>(b, 112), rd::<u64>(b, 120), rd::<u64>(b, 128), rd::<u64>(b, 136),
                          (0..6).map(|k| rd::<u8>(b, 144 + k)).collect::<Vec<u8>>()),
            15 | 16 | 17 | 18 => println!("  Cast\tstart={}\ttarget@8={}\tis_act@16={}", rd::<u64>(b, 0), rd::<u64>(b, 8), rd::<u8>(b, 16)),
            0 => println!("  AroundPosition(untagged)\tstart={}\tgoal=({},{})\tdiff={}\ttarget=({},{})\tinput=({},{},{},{},{})\tradius@88={}\tend_delay@96={}\tpf_tag@173={}\tpurpose@176={}",
                          rd::<u64>(b, 0), rd::<u64>(b, 8), rd::<u64>(b, 16), rd::<i64>(b, 24), rd::<u64>(b, 32), rd::<u64>(b, 40),
                          rd::<u64>(b, 48), rd::<u64>(b, 56), rd::<u64>(b, 64), rd::<u64>(b, 72), rd::<u64>(b, 80), rd::<u64>(b, 88), rd::<u64>(b, 96), rd::<u8>(b, 173), rd::<u8>(b, 176)),
            _ => {}
        }
    }
    let got_recall = res.len() == 1 && rd::<u8>(&res[0] as *const _ as *const u8, 177) == 4;
    println!("recall\tpred={}\tgot={}\t{}", pred_recall as u8, got_recall as u8, if pred_recall == got_recall { "MATCH" } else { "MISMATCH" });
    println!("rnd_changed\t{}", rnd_changed as u8);
    println!("debug_infos\t{}", dbg.infos.len());
}
