#![allow(unused, dead_code, non_snake_case)]
//! 27차 배치E · 259 v48_projectile_profile 오라클 (pub 직접 호출).
//!  한 프로세스 = 한 케이스(argv[1]) — TLS 메모(V48_PROJ_PROFILE · 키 = caster.name × slot) 때문(TEMPLATE 함정 ③).
//!  케이스:
//!   A  slot0 · LinearProjectileEffect(Circle r=7000 · speed 4321) · Direction  → 예측 Some((4321, 7000, 0))
//!   B  slot0 · LinearProjectileEffect(Rect 3000×5000 · speed 999) · Position  → 예측 Some((999, 20000, 0))  (비-Circle → halfwidth 20000)
//!   C  slot1 · level 2 · Linear …                                              → 예측 None(레벨 게이트 level>2 실패) · 같은 프로세스에서 level 9 로 재호출 → 여전히 None(캐시 재생)
//!   D  slot2 · level 5 · RangeProjectileEffect(Circle r=1234 · delay 17)      → 예측 Some((0, 1234, 17))  (Delayed → (0, applyed))  ※applyed 가 delay 필드인지는 실행이 답
//!   E  slot0 · casting Targeting                                              → 예측 None(is_nontarget 실패)
//!   F  slot0 · AttackEffect(투사체 0개) · Direction                            → 예측 None(스폰 투사체 없음)
//!   G  slot0 · 케이스 A 와 같되 두 캐스터(같은 이름 "p")를 번갈아 호출 — 두 번째 캐스터는 다른 effect(speed 1)인데 첫 값 재생(이름 키 공유) 예측
//!  빌드: sh C:/tfm2mods/MIG/_verify3/build.sh C:/tfm2mods/MIG/_verify27/E/oracle/o259.rs
//!  실행: o259.exe <A|B|C|D|E|F|G>
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "../../../_verify3/TEMPLATE.rs"]
mod tmpl;
use tmpl::*;

fn rd<T: Copy>(base: *const u8, off: usize) -> T { unsafe { std::ptr::read_volatile(base.add(off) as *const T) } }

fn mk_effect(ty: Arc<dyn EffectType>, casting: CastingType, range: u64) -> Effect {
    Effect { ty, range, growth_range: 0, start_timing: 10, target: CastingTarget::Enemy,
             attack_type: AttackType::Skill, casting }
}

fn linear(shape: ProjectileShape, speed: u64) -> Arc<dyn EffectType> {
    Arc::new(LinearProjectileEffect { shape, name: "probe_lin".to_string(), applyed_effect: Vec::new(),
        end_effect: Vec::new(), speed, range: 90000, y_offset: 0, applyed_target: CastingTarget::Enemy, penetrate: false })
}

fn ranged(shape: ProjectileShape, delay: u64) -> Arc<dyn EffectType> {
    Arc::new(RangeProjectileEffect { shape, name: "probe_rng".to_string(), applyed_effect: Vec::new(),
        delay, apply: 5, applyed_target: CastingTarget::Enemy })
}

fn attack() -> Arc<dyn EffectType> {
    Arc::new(AttackEffect { ty: AttackEffectType::EnemyTarget, damage: 100, attack_ratio: 0, hp_ratio: 0,
        target_hp_ratio: 0, cc_damage: 0, cc_damage_attack_ratio: 0, shared: false })
}

fn show(tag: &str, r: &Option<(u64, u64, u64)>) {
    let p = r as *const Option<(u64, u64, u64)> as *const u8;
    let t: i64 = rd(p, 0);
    match r {
        Some((s, h, d)) => println!("{}\tSome\tspeed={}\thalf={}\tdelay={}\traw_tag={}", tag, s, h, d, t),
        None => println!("{}\tNone\traw_tag={}\t(+8..+32 undef — 대조 금지)", tag, t),
    }
}

unsafe fn set_slot(e: *mut Entity, slot: u8, eff: Option<Effect>) {
    match slot {
        0 => std::ptr::write(&mut (*e).skill_effect, eff),
        1 => std::ptr::write(&mut (*e).skill2_effect, eff),
        _ => std::ptr::write(&mut (*e).ult_effect, eff),
    }
}
unsafe fn set_level(e: *mut Entity, lv: usize) { std::ptr::write_volatile(&mut (*e).level as *mut usize, lv); }

fn main() {
    let av: Vec<String> = std::env::args().collect();
    let case = av.get(1).cloned().unwrap_or("A".into());
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
    let game = mkgame(&setting, &ms, &map, &ctx);
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let bb: [Blackboard; 2] = [Default::default(), Default::default()];
    let data = OperationData::new(&cache, &ctx, &bb);
    let champ = cache.player_champion[0][0].expect("champ");
    let cp = champ as *const Entity as *mut Entity;
    let champ2 = cache.player_champion[0][1].expect("champ2");
    let cp2 = champ2 as *const Entity as *mut Entity;
    println!("case\t{}\tseed={}\tchamp.id={}\tname={:?}\tname.len={}\tlevel={}\tx={}\ty={}",
             case, (&game as &dyn AbstractGame).seed(), champ.id, champ.name, champ.name.len(), champ.level, champ.x, champ.y);
    unsafe {
        match case.as_str() {
            "A" => {
                set_slot(cp, 0, Some(mk_effect(linear(ProjectileShape::Circle { radius: 7000 }, 4321), CastingType::Direction, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("A.call1(pred Some(4321,7000,0))", &r);
                let r2 = game_ai::v48_projectile_profile(&data, champ, 0);
                show("A.call2(cache hit · same)", &r2);
            }
            "B" => {
                set_slot(cp, 0, Some(mk_effect(linear(ProjectileShape::Rect { width: 3000, height: 5000 }, 999), CastingType::Position, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("B.call1(pred Some(999,20000,0))", &r);
            }
            "C" => {
                set_level(cp, 2);
                set_slot(cp, 1, Some(mk_effect(linear(ProjectileShape::Circle { radius: 7000 }, 4321), CastingType::Direction, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 1);
                show("C.call1 lv2 (pred None: level>2 gate)", &r);
                set_level(cp, 9);
                println!("  level now {}", champ.level);
                let r2 = game_ai::v48_projectile_profile(&data, champ, 1);
                show("C.call2 lv9 (pred None: cache replay ★)", &r2);
            }
            "C2" => {
                set_level(cp, 9);
                set_slot(cp, 1, Some(mk_effect(linear(ProjectileShape::Circle { radius: 7000 }, 4321), CastingType::Direction, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 1);
                show("C2.call1 lv9 fresh process (pred Some(4321,7000,0))", &r);
            }
            "D" => {
                set_level(cp, 5);
                set_slot(cp, 2, Some(mk_effect(ranged(ProjectileShape::Circle { radius: 1234 }, 17), CastingType::Position, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 2);
                show("D.call1 lv5 ult (pred Some(0,1234,17) if Delayed.applyed==delay)", &r);
            }
            "D2" => {
                set_level(cp, 4);
                set_slot(cp, 2, Some(mk_effect(ranged(ProjectileShape::Circle { radius: 1234 }, 17), CastingType::Position, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 2);
                show("D2.call1 lv4 ult (pred None: level>4 gate)", &r);
            }
            "E" => {
                set_slot(cp, 0, Some(mk_effect(linear(ProjectileShape::Circle { radius: 7000 }, 4321), CastingType::Targeting, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("E.call1 Targeting (pred None)", &r);
            }
            "F" => {
                set_slot(cp, 0, Some(mk_effect(attack(), CastingType::Direction, 50000)));
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("F.call1 AttackEffect no projectile (pred None)", &r);
            }
            "G" => {
                set_slot(cp, 0, Some(mk_effect(linear(ProjectileShape::Circle { radius: 7000 }, 4321), CastingType::Direction, 50000)));
                set_slot(cp2, 0, Some(mk_effect(linear(ProjectileShape::Circle { radius: 100 }, 1), CastingType::Direction, 50000)));
                println!("  champ2.id={} name={:?}", champ2.id, champ2.name);
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("G.champ1 (pred Some(4321,7000,0))", &r);
                let r2 = game_ai::v48_projectile_profile(&data, champ2, 0);
                show("G.champ2 same name (pred Some(4321,7000,0) = 이름 키 공유 · 자기 값 (1,100,0) 아님)", &r2);
            }
            "H" => {
                // slot 0 에 effect 없음(None) → None
                set_slot(cp, 0, None);
                let r = game_ai::v48_projectile_profile(&data, champ, 0);
                show("H.call1 skill None (pred None)", &r);
            }
            _ => println!("unknown case"),
        }
    }
}
