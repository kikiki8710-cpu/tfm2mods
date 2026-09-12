#![allow(unused, dead_code, non_snake_case)]
//! A5-O1 — 배치 A(specs[0]~[4]) `mem` 행 오프셋 **런타임 실측** 오라클.
//! `std::mem::offset_of!` 는 컴파일러가 최종 레이아웃으로 계산한 값을 **실행 시점 숫자**로 낸다.
//! ⟹ 명세의 `mem[].offset` 을 IR 독해(ev4) 가 아니라 **실행(ev2)** 으로 대조한다.
//! 열거형 variant 페이로드(ty.Champion.*)는 offset_of 가 안 되므로 **실주소 차**로 잰다.
use game_core::*;
use std::mem::offset_of;

fn chk(spec: &str, base: &str, name: &str, claim: usize, got: usize) {
    println!(
        "{}\t{}\t{}\tclaim=0x{:x}\tgot=0x{:x}\t{}",
        spec, base, name, claim, got,
        if claim == got { "MATCH" } else { "**MISMATCH**" }
    );
}

fn main() {
    println!("== A5-O1 offset oracle ==");

    // ---- OperationData (spec0/1/2/3/4) ----
    chk("0,1,2,3,4", "OperationData", "cache", 0x0, offset_of!(OperationData, cache));
    chk("0,1,2,3,4", "OperationData", "context", 0x8, offset_of!(OperationData, context));
    chk("3,4", "OperationData", "blackboard", 0x10, offset_of!(OperationData, blackboard));

    // ---- GameContext ----
    chk("3,4", "GameContext", "pool", 0x0, offset_of!(GameContext, pool));
    chk("0,3,4", "GameContext", "setting", 0x8, offset_of!(GameContext, setting));
    chk("0,2,4", "GameContext", "map", 0x20, offset_of!(GameContext, map));
    chk("4", "GameContext", "tutorial", 0x38, offset_of!(GameContext, tutorial));

    // ---- AbstractGameWithCache ----
    chk("3,4", "AbstractGameWithCache", "game", 0x0, offset_of!(AbstractGameWithCache, game));
    chk("2,4", "AbstractGameWithCache", "twin_towers", 0x130, offset_of!(AbstractGameWithCache, twin_towers));
    chk("4", "AbstractGameWithCache", "top_tower", 0x180, offset_of!(AbstractGameWithCache, top_tower));
    chk("4", "AbstractGameWithCache", "top_tower2", 0x190, offset_of!(AbstractGameWithCache, top_tower2));
    chk("4", "AbstractGameWithCache", "mid_tower", 0x1a0, offset_of!(AbstractGameWithCache, mid_tower));
    chk("4", "AbstractGameWithCache", "mid_tower2", 0x1b0, offset_of!(AbstractGameWithCache, mid_tower2));
    chk("4", "AbstractGameWithCache", "bottom_tower", 0x1c0, offset_of!(AbstractGameWithCache, bottom_tower));
    chk("4", "AbstractGameWithCache", "bottom_tower2", 0x1d0, offset_of!(AbstractGameWithCache, bottom_tower2));
    chk("0,1,2,3", "AbstractGameWithCache", "player_champion", 0x1e0, offset_of!(AbstractGameWithCache, player_champion));

    // ---- GameSetting ----
    chk("3", "GameSetting", "tick_per_second", 0x12f8, offset_of!(GameSetting, tick_per_second));

    // ---- MapDef ----
    chk("2", "MapDef", "fountains", 0x6d70, offset_of!(MapDef, fountains));

    // ---- MobaMode ----
    chk("4", "MobaMode", "epic_minion_buff_time", 0x240, offset_of!(MobaMode, epic_minion_buff_time));

    // ---- Strategy ----
    chk("4", "Strategy", "morgard_defense", 0xe, offset_of!(Strategy, morgard_defense));

    // ---- Blackboard ----
    chk("4", "Blackboard", "top_minion_state", 0x0, offset_of!(Blackboard, top_minion_state));
    chk("4", "Blackboard", "mid_minion_state", 0x28, offset_of!(Blackboard, mid_minion_state));
    chk("4", "Blackboard", "bottom_minion_state", 0x50, offset_of!(Blackboard, bottom_minion_state));

    // ---- BrainMinionParameter ----
    chk("4", "BrainMinionParameter", "from_mid", 0x10, offset_of!(BrainMinionParameter, from_mid));
    chk("4", "BrainMinionParameter", "minion_count", 0x20, offset_of!(BrainMinionParameter, minion_count));

    // ---- Effect ----
    chk("0", "Effect", "range", 0x10, offset_of!(Effect, range));
    chk("0", "Effect", "growth_range", 0x18, offset_of!(Effect, growth_range));
    chk("0", "Effect", "target", 0x28, offset_of!(Effect, target));
    chk("0,3", "Effect", "casting", 0x30, offset_of!(Effect, casting));
    chk("0", "Effect", "ty", 0x0, offset_of!(Effect, ty));

    // ---- Entity (직접 필드) ----
    chk("0", "Entity", "team", 0x0, offset_of!(Entity, team));
    chk("0", "Entity", "visible_state", 0x38, offset_of!(Entity, visible_state));
    chk("1,3", "Entity", "ty", 0x68, offset_of!(Entity, ty));
    chk("3", "Entity", "skill_effect", 0x4c8, offset_of!(Entity, skill_effect));
    chk("3", "Entity", "skill2_effect", 0x500, offset_of!(Entity, skill2_effect));
    chk("0,3", "Entity", "ult_effect", 0x538, offset_of!(Entity, ult_effect));
    chk("3,4", "Entity", "id", 0x5c0, offset_of!(Entity, id));
    chk("0,3", "Entity", "level", 0x5c8, offset_of!(Entity, level));
    chk("0,2,4", "Entity", "x", 0x660, offset_of!(Entity, x));
    chk("0,2,4", "Entity", "y", 0x668, offset_of!(Entity, y));
    chk("1,2", "Entity", "hp", 0x670, offset_of!(Entity, hp));
    chk("0", "Entity", "radius", 0x680, offset_of!(Entity, radius));
    // 중첩 필드
    chk("0", "Entity", "stat_buff_cached.range", 0x438, offset_of!(Entity, stat_buff_cached.range));
    chk("0", "Entity", "stat_buff_cached.radius_mult", 0x470, offset_of!(Entity, stat_buff_cached.radius_mult));
    chk("2", "Entity", "stat_cached.hp", 0x628, offset_of!(Entity, stat_cached.hp));
    // 구조체 크기 (명세가 괄호로 적은 값)
    println!("size\tEntity\t{}\t(명세 1728B)", std::mem::size_of::<Entity>());
    println!("size\tPlayerState\t{}\t(명세 2528B)", std::mem::size_of::<PlayerState>());
    println!("size\tOperationData\t{}\t(명세 24B)", std::mem::size_of::<OperationData>());
    println!("size\tGameContext\t{}\t(명세 64B)", std::mem::size_of::<GameContext>());
    println!("size\tAbstractGameWithCache\t{}\t(명세 8840B)", std::mem::size_of::<AbstractGameWithCache>());
    println!("size\tGameSetting\t{}\t(명세 5432B)", std::mem::size_of::<GameSetting>());
    println!("size\tMapDef\t{}\t(명세 28112B)", std::mem::size_of::<MapDef>());
    println!("size\tEffect\t{}\t(명세 56B)", std::mem::size_of::<Effect>());
    println!("size\tMobaMode\t{}\t(명세 640B)", std::mem::size_of::<MobaMode>());
    println!("size\tStrategy\t{}\t(명세 24B)", std::mem::size_of::<Strategy>());
    println!("size\tBlackboard\t{}\t(명세 744B)", std::mem::size_of::<Blackboard>());
    println!("size\tBrainMinionParameter\t{}\t(명세 40B)", std::mem::size_of::<BrainMinionParameter>());
    println!("size\tDefensiveCrisis\t{}\t(명세 2B)", std::mem::size_of::<game_ai::DefensiveCrisis>());
    println!("size\tOption<Input>\t{}\t(명세 32B)", std::mem::size_of::<Option<Input>>());
    println!("size\tInputTarget\t{}\t(명세 24B)", std::mem::size_of::<InputTarget>());
    println!("size\tPositioningScoreData\t{}\t(명세 2760B)", std::mem::size_of::<PositioningScoreData>());
    println!("size\tSubPlan\t{}\t(명세 72B)", std::mem::size_of::<game_ai::plan_legacy::sub_plan::SubPlan>());
    println!("size\tDebugFrameData\t{}\t(명세 224B)", std::mem::size_of::<DebugFrameData>());
    println!("size\tScoreParameter\t{}\t(명세 5384B)", std::mem::size_of::<game_ai::ScoreParameter>());
    println!("size\tStdRng\t{}\t(명세 320B)", std::mem::size_of::<rand::rngs::StdRng>());
}
