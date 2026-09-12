#![allow(unused, dead_code, non_snake_case)]
//! A5-O2 — 남은 `mem` 행: 중첩(PlayerState.info.*) · 배열 stride · 열거형 페이로드 오프셋을
//! **실주소 차**로 잰다(offset_of 가 안 되는 것들).
use game_core::*;
use rand::SeedableRng;
use std::mem::offset_of;
use std::sync::Arc;

#[path = "tmpl.rs"]
mod tmpl;
use tmpl::*;

fn chk(spec: &str, base: &str, name: &str, claim: usize, got: usize) {
    println!(
        "{}\t{}\t{}\tclaim=0x{:x}\tgot=0x{:x}\t{}",
        spec, base, name, claim, got,
        if claim == got { "MATCH" } else { "**MISMATCH**" }
    );
}

fn main() {
    println!("== A5-O2 nested/stride/variant offsets ==");
    // ---- PlayerState.info(GamePlayer) 중첩 ----
    chk("0,1,2,3,4", "PlayerState", "info.team", 0x930, offset_of!(PlayerState, info.team));
    chk("0,1,2", "PlayerState", "info.position", 0x9c0, offset_of!(PlayerState, info.position));
    println!("size\tGamePlayer\t{}", std::mem::size_of::<GamePlayer>());
    println!("offset\tPlayerState.info\t0x{:x}", offset_of!(PlayerState, info));

    // ---- stride ----
    println!("stride\tplayer_champion[team]\t{}\t(명세 40)", std::mem::size_of::<[Option<&Entity>; 5]>());
    println!("stride\tplayer_champion[t][p]\t{}\t(명세 8)", std::mem::size_of::<Option<&Entity>>());
    println!("stride\ttwin_towers[team]\t{}\t(명세 32)", std::mem::size_of::<bumpalo::collections::Vec<&Entity>>());
    println!("stride\tvisible_state[i]\t{}\t(명세 24)", std::mem::size_of::<VisibleState>());
    println!("stride\tfountains[team]\t{}\t(명세 32)", std::mem::size_of::<(u64, u64, u64, u64)>());
    println!("stride\tepic_minion_buff_time\t{}\t(명세 16=2*8)", std::mem::size_of::<[usize; 2]>());
    println!("size\tLineType\t{}\t(명세 1B)", std::mem::size_of::<LineType>());
    println!("size\tCastingType\t{}\t(명세 4B)", std::mem::size_of::<CastingType>());
    println!("size\tCastingTarget\t{}\t(명세 4B)", std::mem::size_of::<CastingTarget>());
    println!("size\tPosition\t{}\t(명세 4B)", std::mem::size_of::<Position>());
    println!("size\tOption<Effect>\t{}\t(명세 56B = 니치)", std::mem::size_of::<Option<Effect>>());
    println!("size\tEntityType\t{}", std::mem::size_of::<EntityType>());

    // ---- 니치 판별자: Option<Effect> 의 None 표식이 정말 Effect+0x30 인가 ----
    {
        let none: Option<Effect> = None;
        let p = &none as *const Option<Effect> as *const u8;
        let mut nz: Vec<(usize, i32)> = Vec::new();
        unsafe {
            // i32 단위로 읽어 -1 이 어디 있는지
            for off in (0..56usize).step_by(4) {
                let v = std::ptr::read_unaligned(p.add(off) as *const i32);
                if v != 0 {
                    nz.push((off, v));
                }
            }
        }
        println!("niche\tOption<Effect>=None non-zero i32 words: {:?}\t(명세: +0x30 == -1)", nz);
    }

    // ---- TeamType 태그/페이로드 ----
    {
        let tp = TeamType::Player(1usize);
        let p = &tp as *const TeamType as *const u8;
        unsafe {
            println!("TeamType::Player(1)\ttag@+0x0={}\tpayload@+0x8={}\tsize={}",
                     std::ptr::read_unaligned(p as *const i64),
                     std::ptr::read_unaligned(p.add(8) as *const usize),
                     std::mem::size_of::<TeamType>());
        }
        let nt = TeamType::Neutral;
        let p2 = &nt as *const TeamType as *const u8;
        unsafe {
            println!("TeamType::Neutral\ttag@+0x0={}\t(명세 0=Player,1=Neutral)",
                     std::ptr::read_unaligned(p2 as *const i64));
        }
    }

    // ---- VisibleState 태그 ----
    {
        for (nm, v) in [("Visible", VisibleState::Visible), ("Unknown", VisibleState::Unknown)] {
            let p = &v as *const VisibleState as *const i64;
            unsafe { println!("VisibleState::{}\ttag={}\t(명세 Visible=0)", nm, std::ptr::read_unaligned(p)); }
        }
        let iv = VisibleState::Invisible { last_x: 7, last_y: 9 };
        let p = &iv as *const VisibleState as *const i64;
        unsafe { println!("VisibleState::Invisible\ttag={}", std::ptr::read_unaligned(p)); }
    }

    // ---- CastingType / CastingTarget 판별자 ----
    for (nm, v) in [
        ("Targeting", CastingType::Targeting), ("Position", CastingType::Position),
        ("Direction", CastingType::Direction), ("None", CastingType::None),
    ] {
        let p = &v as *const CastingType as *const i32;
        unsafe { println!("CastingType::{}\t{}\t(명세 0/1/2/3)", nm, std::ptr::read_unaligned(p)); }
    }

    // ---- 실게임에서 Champion variant 페이로드 오프셋 ----
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
    println!("towers\t{}\ttwin0={}\ttwin1={}", game.world.tower_ids.len(),
             cache.twin_towers[0].len(), cache.twin_towers[1].len());

    let e: &Entity = cache.player_champion[0][0].unwrap();
    let base = e as *const Entity as usize;
    println!("EntityType tag(i64)@Entity+0x68 = {}\t(명세 Champion=13)", unsafe {
        std::ptr::read_unaligned((base + 0x68) as *const i64)
    });
    if let EntityType::Champion(c) = &e.ty {
        chk("3", "Entity", "ty.Champion.attack_cooldown", 0xb0, (&c.attack_cooldown as *const _ as usize) - base);
        chk("3", "Entity", "ty.Champion.skill_cooldown", 0xb8, (&c.skill_cooldown as *const _ as usize) - base);
        chk("3", "Entity", "ty.Champion.skill2_cooldown", 0xc0, (&c.skill2_cooldown as *const _ as usize) - base);
        chk("3", "Entity", "ty.Champion.ult_cooldown", 0xc8, (&c.ult_cooldown as *const _ as usize) - base);
    } else {
        println!("**ty is not Champion**");
    }
    // Entity+0x4f8 = skill_effect 의 Option 니치(= skill_effect + 0x30)
    chk("3", "Entity", "skill_effect+0x30(casting)", 0x4f8, offset_of!(Entity, skill_effect) + 0x30);

    // ---- Tower entity 의 태그 ----
    for id in game.world.tower_ids.iter().take(1) {
        if let Some(t) = game.world.entity.get(*id) {
            println!("tower EntityType tag = {}\t(명세 Tower=2)", unsafe {
                std::ptr::read_unaligned(((t as *const Entity as usize) + 0x68) as *const i64)
            });
        }
    }
    for id in game.world.nexus_ids.iter().take(1) {
        if let Some(t) = game.world.entity.get(*id) {
            println!("nexus EntityType tag = {}\t(명세 Nexus=3)", unsafe {
                std::ptr::read_unaligned(((t as *const Entity as usize) + 0x68) as *const i64)
            });
        }
    }
}
