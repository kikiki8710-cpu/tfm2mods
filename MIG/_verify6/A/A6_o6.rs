#![allow(unused, dead_code, non_snake_case)]
//! A6-O6 — A6_o1 이 못 덮은 나머지 `mem` 행 보충(수법 ⓐ 의 실주소 차 변형).
//!   · `EntityType::Champion` 페이로드 3칸 (Entity+0xb8/0xc0/0xc8) — 열거형 variant 라 `offset_of!` 불가
//!   · `Entity+0x98` = `ty.Jungle.info.camp_type.__0`
//!   · `Effect+0x8` = `Arc<dyn EffectType>` 팻포인터 뒤 워드(vtable)
//!   · `PlayerState+0x218`(AthleteStat.judgement) · `+0x450`(judgement_mental_ratio) — specs[3] knobs[19]/[20]
//!   · `Option<Effect>::None` 니치 = `+0x30 == -1`
use game_core::*;
use rand::SeedableRng;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

static mut NOK: usize = 0;
static mut NNG: usize = 0;
fn chk(spec: &str, base: &str, name: &str, claim: usize, got: usize) {
    let ok = claim == got;
    unsafe { if ok { NOK += 1 } else { NNG += 1 } }
    println!("{}\t{}\t{}\tclaim=0x{:x}\tgot=0x{:x}\t{}", spec, base, name, claim, got,
             if ok { "MATCH" } else { "**MISMATCH**" });
}

fn main() {
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
    let cache = AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx);
    let e: &Entity = cache.player_champion[0][0].unwrap();
    let eb = e as *const Entity as usize;

    // EntityType::Champion 페이로드 (실주소 차)
    if let EntityType::Champion(c) = &e.ty {
        chk("3", "Entity", "ty.Champion.attack_cooldown", 0xb0, (&c.attack_cooldown as *const usize as usize) - eb);
        chk("3", "Entity", "ty.Champion.skill_cooldown", 0xb8, (&c.skill_cooldown as *const usize as usize) - eb);
        chk("3", "Entity", "ty.Champion.skill2_cooldown", 0xc0, (&c.skill2_cooldown as *const usize as usize) - eb);
        chk("3", "Entity", "ty.Champion.ult_cooldown", 0xc8, (&c.ult_cooldown as *const usize as usize) - eb);
    } else { println!("**Champion 아님**"); }
    // EntityType 태그
    println!("TAG\tEntityType(Champion)@Entity+0x68={}\t(명세 13)",
             unsafe { std::ptr::read_unaligned((eb + 0x68) as *const i64) });

    // Entity+0x98 = ty.Jungle.info.camp_type.__0 (정글 캠프를 pub spawn 으로 생성)
    for team in 0..2usize {
        let cs = game.mode.jungle_runner.get_camp_state(team, JungleType::Rhino);
        for j in cs.spawn(&setting, 0, 5000 + team * 100).into_iter().take(1) {
            let jb = &j as *const Entity as usize;
            let tag = unsafe { std::ptr::read_unaligned((jb + 0x68) as *const i64) };
            let c0 = unsafe { std::ptr::read_unaligned((jb + 0x98) as *const usize) };
            println!("JUNGLE\tteam={}\ttag@+0x68={}\t+0x98={}\t{}", team, tag, c0,
                     if tag == 4 && c0 == team { "MATCH(camp_type.__0 = 팀 인덱스)" } else { "**MISMATCH**" });
            unsafe { if tag == 4 && c0 == team { NOK += 1 } else { NNG += 1 } }
        }
    }

    // Effect+0x8 = Arc<dyn EffectType> 팻포인터 뒤 워드
    {
        let ef = Effect { range: 1, growth_range: 0, start_timing: 0,
                          casting: CastingType::Targeting, target: CastingTarget::Enemy,
                          ty: Arc::new(AttackEffect::new(1, 0)) as Arc<dyn EffectType>,
                          attack_type: AttackType::Skill };
        let b = &ef as *const Effect as usize;
        let w: &[usize] = unsafe { std::slice::from_raw_parts(b as *const usize, 7) };
        let raw: (*const (), *const ()) = unsafe { std::mem::transmute(&*ef.ty as *const dyn EffectType) };
        println!("EFFECT_ARC\tword0=0x{:x}\tword1=0x{:x}\tdyn_vtable=0x{:x}\t{}",
                 w[0], w[1], raw.1 as usize,
                 if w[1] == raw.1 as usize { "MATCH(+0x8 = vtable 포인터)" } else { "**MISMATCH**" });
        unsafe { if w[1] == raw.1 as usize { NOK += 1 } else { NNG += 1 } }
        // Option<Effect>::None 니치 = +0x30 == -1
        let n: Option<Effect> = None;
        let nv = unsafe { std::ptr::read_unaligned(((&n as *const _ as usize) + 0x30) as *const i32) };
        println!("OPT_EFFECT\tNone 의 +0x30 = {}\t(명세 -1)\t{}", nv,
                 if nv == -1 { "MATCH" } else { "**MISMATCH**" });
        unsafe { if nv == -1 { NOK += 1 } else { NNG += 1 } }
        std::mem::forget(n);
    }

    // PlayerState +0x218 / +0x450
    {
        let ps: &PlayerState = game.get_player_by_position(0, Position::Top).unwrap();
        let pb = ps as *const PlayerState as usize;
        let j = unsafe { std::ptr::read_unaligned((pb + 0x218) as *const usize) };
        let r = unsafe { std::ptr::read_unaligned((pb + 0x450) as *const usize) };
        println!("PS\t+0x218={}\t(mkgame 이 judgement=80 으로 만든다)\t{}", j,
                 if j == 80 { "MATCH" } else { "**MISMATCH**" });
        println!("PS\t+0x450={}\t(명세 judgement_mental_ratio 초기 1000)\t{}", r,
                 if r == 1000 { "MATCH" } else { "**MISMATCH**" });
        unsafe { if j == 80 { NOK += 1 } else { NNG += 1 } }
        unsafe { if r == 1000 { NOK += 1 } else { NNG += 1 } }
    }
    unsafe { println!("\nTOTAL\tMATCH={}\tMISMATCH={}", NOK, NNG); }
}
