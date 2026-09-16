#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치E · 260 resolve_fight_stake_roster 오라클 — `define hidden` 심볼을 link_name 으로 직접 진입(METHOD_MAP ⑥).
//!  구조 검증(콜리 resolve_fight_full 내부는 블랙박스):
//!   R1  roster=[(ally,0,false)]  → remaining 비어 → 반환 = absolute(콜리 값 그대로)
//!   R2  roster=[(ally,0,true)]   → remaining=[ally] → 반환 = diff · 예측: diff.line_absolute == R1.line(=absolute.line, 같은 allies 입력) ·
//!       line 이 갈리면 rescue_ally == Some(ally.id) 여야 하고 같으면 콜리 값
//!   R3  roster=[(champ,0,true)]  → 자기 자신은 bound 라도 id 필터로 제외 → remaining 비어 → 반환 == roster=[(champ,0,false)] 결과와 64B 동일(패딩 6B 제외)
//!   R4  roster=[]                 → allies 빈 슬라이스 · remaining 비어 → absolute
//!   R5  roster=[(a1,0,true),(a2,0,true)] 두 아군 · 갈리면 rescue_ally = 거리²(u64 절대차) 최소 아군 id
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify27/E/oracle/o260.rs
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

#[repr(C)]
#[derive(Clone, Copy)]
struct FP { focus_tag: u64, focus: u64, soaker_tag: u64, soaker: u64, rescue_tag: u64, rescue: u64, net_value: i64, line: u8, line_absolute: u8, pad: [u8; 6] }

extern "Rust" {
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model26resolve_fight_stake_roster"]
    fn stake_roster(version: usize, data: &OperationData, champ: &Entity, roster: &[(&Entity, i64, bool)],
                    near_enemies: &[&Entity], committed_dir: i8, tower: Option<&Entity>, judge_accuracy: usize) -> FP;
}


fn mkatk(damage: usize) -> Effect {
    let ae = AttackEffect { ty: AttackEffectType::EnemyTarget, damage, attack_ratio: 100, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false };
    Effect { ty: Arc::new(ae), range: 60000, growth_range: 0, start_timing: 10,
        target: CastingTarget::Enemy, attack_type: AttackType::BaseAttack, casting: CastingType::Targeting }
}

fn show(tag: &str, f: &FP) {
    println!("{}\tfocus=({},{})\tsoaker=({},{})\trescue=({},{})\tnet={}\tline={}\tline_abs={}",
             tag, f.focus_tag, f.focus, f.soaker_tag, f.soaker, f.rescue_tag, f.rescue, f.net_value, f.line, f.line_absolute);
}
fn same56(a: &FP, b: &FP) -> bool {
    let pa = a as *const FP as *const u8; let pb = b as *const FP as *const u8;
    unsafe { (0..58).all(|i| *pa.add(i) == *pb.add(i)) }   // 0x3a 까지(패딩 제외)
}

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() > 99 { let _ = game_ai::v48_projectile_profile as *const (); }
    let case = av.get(1).cloned().unwrap_or("R1".into());
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
    game.set_tick(1000);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].expect("champ");
    let a1 = cache.player_champion[0][1].expect("a1");
    let a2 = cache.player_champion[0][2].expect("a2");
    let e1 = cache.player_champion[1][0].expect("e1");
    let e2 = cache.player_champion[1][1].expect("e2");
    // 적을 챔프 근처로 옮겨 교전 상황을 만든다(write_volatile · TEMPLATE 함정 ⑦: raw 포인터로)
    unsafe {
        let p1 = e1 as *const Entity as *mut Entity; let p2 = e2 as *const Entity as *mut Entity;
        std::ptr::write_volatile(&mut (*p1).x, champ.x + 60000); std::ptr::write_volatile(&mut (*p1).y, champ.y);
        std::ptr::write_volatile(&mut (*p2).x, champ.x + 80000); std::ptr::write_volatile(&mut (*p2).y, champ.y + 20000);
        let q1 = a1 as *const Entity as *mut Entity; let q2 = a2 as *const Entity as *mut Entity;
        std::ptr::write_volatile(&mut (*q1).x, champ.x + 30000); std::ptr::write_volatile(&mut (*q1).y, champ.y);      // dist² = 9e8
        std::ptr::write_volatile(&mut (*q2).x, champ.x + 10000); std::ptr::write_volatile(&mut (*q2).y, champ.y + 5000); // dist² = 1.25e8 (더 가깝다)
    }
    let near: Vec<&Entity> = vec![e1, e2];
    let dir: i8 = av.get(2).and_then(|s| s.parse().ok()).unwrap_or(-1);
    let acc: usize = 100;
    println!("case\t{}\tchamp.id={}\ta1.id={}\ta2.id={}\tdir={}\tchamp=({},{})\ta1=({},{})\ta2=({},{})", case, champ.id, a1.id, a2.id, dir, champ.x, champ.y, a1.x, a1.y, a2.x, a2.y);
    unsafe {
        match case.as_str() {
            "R1R2" => {
                let r1 = stake_roster(55, &data, champ, &[(a1, 0, false)], &near, dir, None, acc);
                show("R1 unbound(pred = absolute)", &r1);
                let r2 = stake_roster(55, &data, champ, &[(a1, 0, true)], &near, dir, None, acc);
                show("R2 bound  (pred = diff · line_abs==R1.line)", &r2);
                println!("check\tR2.line_abs==R1.line\t{}\tline_diverged={}\trescue_expected={}",
                         r2.line_absolute == r1.line, r2.line != r1.line,
                         if r2.line != r1.line { format!("Some({})", a1.id) } else { "callee-value".to_string() });
            }
            "R3" => {
                let r3a = stake_roster(55, &data, champ, &[(champ, 0, true)], &near, dir, None, acc);
                let r3b = stake_roster(55, &data, champ, &[(champ, 0, false)], &near, dir, None, acc);
                show("R3a self bound", &r3a); show("R3b self unbound", &r3b);
                println!("check\tR3a==R3b(58B)\t{}", same56(&r3a, &r3b));
            }
            "R4" => {
                let r4 = stake_roster(55, &data, champ, &[], &near, dir, None, acc);
                show("R4 empty roster", &r4);
            }
            "R5" => {
                let r5u = stake_roster(55, &data, champ, &[(a1, 0, false), (a2, 0, false)], &near, dir, None, acc);
                show("R5u two unbound (absolute)", &r5u);
                let r5 = stake_roster(55, &data, champ, &[(a1, 0, true), (a2, 0, true)], &near, dir, None, acc);
                show("R5 two bound (diff)", &r5);
                println!("check\tline_abs==abs.line\t{}\tdiverged={}\tnearest_ally={}\trescue=({},{})",
                         r5.line_absolute == r5u.line, r5.line != r5u.line, a2.id, r5.rescue_tag, r5.rescue);
            }
            "R6" => {
                // 판별력 확보 시도: 전원 attack_effect 주입 + 적 공격력/HP 비대칭
                for (e, dmg) in [(champ, 150usize), (a1, 150), (a2, 150), (e1, 400), (e2, 400)] {
                    let p = e as *const Entity as *mut Entity;
                    std::ptr::write(&mut (*p).attack_effect, Some(mkatk(dmg)));
                    std::ptr::write_volatile(&mut (*p).stat_cached.attack, dmg);
                    std::ptr::write_volatile(&mut (*p).stat_cached.hp, if dmg > 200 { 3000 } else { 1200 });
                    std::ptr::write_volatile(&mut (*p).hp, if dmg > 200 { 3000 } else { 1200 });
                }
                let r6u = stake_roster(55, &data, champ, &[(a1, 0, false), (a2, 0, false)], &near, dir, None, acc);
                show("R6u two unbound (absolute)", &r6u);
                let r6 = stake_roster(55, &data, champ, &[(a1, 0, true), (a2, 0, true)], &near, dir, None, acc);
                show("R6 two bound (diff)", &r6);
                println!("check	line_abs==abs.line	{}	diverged={}	nearest_ally={}	rescue=({},{})",
                         r6.line_absolute == r6u.line, r6.line != r6u.line, a2.id, r6.rescue_tag, r6.rescue);
            }
            _ => println!("unknown"),
        }
    }
}
