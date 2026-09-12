#![allow(unused, dead_code, non_snake_case)]
//! C6 프로브 ①  ⓐ`offset_of!` 일괄 대조 (5차 배치A 발명 — 배치C 는 이번이 처음 = `inherited`)
//! 배치 C 담당 `specs[10]`~`specs[14]` 의 `mem` 표 전량을 **런타임 실행**으로 교차검증한다.
//! 출력: `OFF\t<base>\t<offset>\t<field>\t<expect>\t<actual>\t<OK|MISMATCH>`
//!       `SZ \t<type>\t<expect>\t<actual>\t<OK|MISMATCH>`
use game_core::*;
use std::mem::{offset_of, size_of};

macro_rules! chk {
    ($base:literal, $exp:expr, $name:literal, $got:expr) => {{
        let e: usize = $exp;
        let g: usize = $got;
        println!("OFF\t{}\t0x{:x}\t{}\t{}\t{}\t{}", $base, e, $name, e, g,
                 if e == g { "OK" } else { "MISMATCH" });
        if e != g { unsafe { BAD += 1; } }
    }};
}
macro_rules! sz {
    ($t:ty, $exp:expr) => {{
        let e: usize = $exp;
        let g: usize = size_of::<$t>();
        println!("SZ\t{}\t{}\t{}\t{}", stringify!($t), e, g,
                 if e == g { "OK" } else { "MISMATCH" });
        if e != g { unsafe { BAD += 1; } }
    }};
}
static mut BAD: usize = 0;

fn main() {
    // ───────── 구조체 크기 (명세가 `dereferenceable(N)` 로 주장한 값) ─────────
    sz!(PlayerState, 2528);
    sz!(OperationData, 24);
    sz!(AbstractGameWithCache, 8840);
    sz!(GameContext, 64);
    sz!(Blackboard, 744);
    sz!(Strategy, 24);
    sz!(GameSetting, 5432);
    sz!(MapDef, 28112);
    sz!(Tower, 192);
    sz!(MobaMode, 640);
    sz!(BigGoal, 24);
    sz!(DebugFrameData, 224);
    sz!(PositioningScoreData, 2760);

    // ───────── specs[10] ─────────
    chk!("PlayerState", 0x930, "info.team", offset_of!(PlayerState, info.team));
    chk!("OperationData", 0x0, "cache", offset_of!(OperationData, cache));
    chk!("OperationData", 0x10, "blackboard", offset_of!(OperationData, blackboard));
    chk!("AbstractGameWithCache", 0x0, "game", offset_of!(AbstractGameWithCache, game));
    chk!("AbstractGameWithCache", 0x1e0, "player_champion",
         offset_of!(AbstractGameWithCache, player_champion));
    chk!("MobaMode", 0x198, "jungle_runner.epic.live_list(.cap)",
         offset_of!(MobaMode, jungle_runner.epic.live_list));
    chk!("MobaMode", 0x1c8, "jungle_runner.serpen.live_list(.cap)",
         offset_of!(MobaMode, jungle_runner.serpen.live_list));
    chk!("Entity", 0x0, "team", offset_of!(Entity, team));
    chk!("Entity", 0x660, "x", offset_of!(Entity, x));
    chk!("Entity", 0x668, "y", offset_of!(Entity, y));
    chk!("Strategy", 0xf, "object_finish", offset_of!(Strategy, object_finish));

    // ───────── specs[11] / specs[12] 공통 ─────────
    chk!("OperationData", 0x8, "context", offset_of!(OperationData, context));
    chk!("GameContext", 0x38, "tutorial", offset_of!(GameContext, tutorial));
    chk!("GameContext", 0x39, "trace_level", offset_of!(GameContext, trace_level));
    chk!("GameContext", 0x8, "setting", offset_of!(GameContext, setting));
    chk!("GameContext", 0x20, "map", offset_of!(GameContext, map));

    // ───────── specs[13] / specs[14] ─────────
    chk!("AbstractGameWithCache", 0x180, "top_tower", offset_of!(AbstractGameWithCache, top_tower));
    chk!("AbstractGameWithCache", 0x190, "top_tower2", offset_of!(AbstractGameWithCache, top_tower2));
    chk!("AbstractGameWithCache", 0x1a0, "mid_tower", offset_of!(AbstractGameWithCache, mid_tower));
    chk!("AbstractGameWithCache", 0x1b0, "mid_tower2", offset_of!(AbstractGameWithCache, mid_tower2));
    chk!("AbstractGameWithCache", 0x1c0, "bottom_tower", offset_of!(AbstractGameWithCache, bottom_tower));
    chk!("AbstractGameWithCache", 0x1d0, "bottom_tower2", offset_of!(AbstractGameWithCache, bottom_tower2));
    chk!("AbstractGameWithCache", 0x21c0, "top_lead", offset_of!(AbstractGameWithCache, top_lead));
    chk!("AbstractGameWithCache", 0x21d0, "mid_lead", offset_of!(AbstractGameWithCache, mid_lead));
    chk!("AbstractGameWithCache", 0x21e0, "bottom_lead", offset_of!(AbstractGameWithCache, bottom_lead));
    chk!("PlayerState", 0x9c0, "info.position", offset_of!(PlayerState, info.position));
    chk!("Entity", 0x68, "ty", offset_of!(Entity, ty));
    chk!("Entity", 0x628, "stat_cached.hp", offset_of!(Entity, stat_cached.hp));
    chk!("Entity", 0x670, "hp", offset_of!(Entity, hp));
    chk!("Tower", 0x18, "nearest_enemy(Tower 기준)", offset_of!(Tower, nearest_enemy));
    chk!("Tower", 0xb8, "ty(Tower 기준)", offset_of!(Tower, ty));
    chk!("GameSetting", 0x12c0, "height", offset_of!(GameSetting, height));
    chk!("MapDef", 0x1c98, "bushes", offset_of!(MapDef, bushes));

    // Entity.ty 페이로드(EntityType::Tower) 절대오프셋 = Entity.ty + 8  (tcxdict: 페이로드 시작 +0x8)
    let ety = offset_of!(Entity, ty);
    chk!("Entity", 0x70, "ty payload(&Tower) = ty+8", ety + 8);
    chk!("Entity", 0x88, "ty.Tower.nearest_enemy = ty+8+0x18", ety + 8 + offset_of!(Tower, nearest_enemy));
    chk!("Entity", 0x128, "ty.Tower.ty = ty+8+0xb8", ety + 8 + offset_of!(Tower, ty));

    println!("\nTOTAL_MISMATCH\t{}", unsafe { BAD });
}
