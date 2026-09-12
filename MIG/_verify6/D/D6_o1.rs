#![allow(unused, dead_code, non_snake_case)]
//! 6차 배치D 프로브 ① — ⓐ `std::mem::offset_of!` **일괄 대조** (5차 배치A 수법 = 내겐 `inherited`)
//!
//! 목적: `specs[15]~[19]` 의 `mem` 행 전량을 **한 프로세스에서** 기계 대조한다.
//! 5차 배치D 는 `&x.f as *const _ as usize - base` 주소 산술로 **일부만** 쟀다.
//! 여기서는 `offset_of!` 로 **전 행**(ev3 인 것까지)을 쓸어 MISMATCH 를 센다.
//!
//! 출력 형식: `OFF\t<경로>\t<주장>\t<실측>\t<OK|MISMATCH>`
use game_core::*;
use game_ai::plan_legacy::team_plan::TeamPlan;
use game_ai::plan_legacy::old::{SinglePlanBattle, DeathMatchBattle, BattlePlanGoal, BattleSubPlanGoal};
use game_ai::plan_legacy::handler::LegacyPlanHandler;
use std::mem::{offset_of, size_of};

static mut OK: usize = 0;
static mut NG: usize = 0;

fn chk(path: &str, want: usize, got: usize) {
    let m = if want == got { "OK" } else { "MISMATCH" };
    unsafe { if want == got { OK += 1 } else { NG += 1 } }
    println!("OFF\t{}\t0x{:x}\t0x{:x}\t{}", path, want, got, m);
}
fn chksz(path: &str, want: usize, got: usize) {
    let m = if want == got { "OK" } else { "MISMATCH" };
    unsafe { if want == got { OK += 1 } else { NG += 1 } }
    println!("SIZE\t{}\t{}\t{}\t{}", path, want, got, m);
}

fn main() {
    // ── 구조체 크기 (명세가 괄호로 적은 값) ──────────────────────────
    chksz("OperationData", 24, size_of::<OperationData>());
    chksz("AbstractGameWithCache", 8840, size_of::<AbstractGameWithCache>());
    chksz("GameContext", 64, size_of::<GameContext>());
    chksz("PlayerState", 2528, size_of::<PlayerState>());
    chksz("Entity", 1728, size_of::<Entity>());
    chksz("Effect", 56, size_of::<Effect>());
    chksz("TeamPlan", 1064, size_of::<TeamPlan>());
    chksz("SinglePlanBattle", 144, size_of::<SinglePlanBattle>());
    chksz("DeathMatchBattle", 384, size_of::<DeathMatchBattle>());
    chksz("BattlePlanGoal", 24, size_of::<BattlePlanGoal>());
    chksz("BattleSubPlanGoal", 16, size_of::<BattleSubPlanGoal>());
    chksz("LegacyPlanHandler", 6168, size_of::<LegacyPlanHandler>());
    chksz("MapDef", 28112, size_of::<MapDef>());
    chksz("DebugFrameData", 224, size_of::<DebugFrameData>());
    chksz("StdRng", 320, size_of::<rand::rngs::StdRng>());
    chksz("MainObjective", 3, size_of::<game_ai::plan_legacy::team_plan::MainObjective>());
    chksz("Strategy", 24, size_of::<Strategy>());
    chksz("JungleCampState", 48, size_of::<JungleCampState>());
    chksz("Chat", 24, size_of::<Chat>());
    chksz("EntityType", 480, size_of::<EntityType>());

    // ── /specs[15] single_try_engage ────────────────────────────────
    chk("15/mem[0]  OperationData.cache",           0x0,   offset_of!(OperationData, cache));
    chk("15/mem[1]  AGWC.game(data ptr)",           0x0,   offset_of!(AbstractGameWithCache, game));
    // mem[2] = 팻포인터 둘째 워드 = game(+8). offset_of! 로는 못 잡으므로 크기로 갈음
    chksz("15/mem[2]  &dyn AbstractGame 팻포인터",  16,    size_of::<&dyn AbstractGame>());
    chk("15/mem[4]  PlayerState.info.team",         0x930, offset_of!(PlayerState, info.team));
    chk("15/mem[5]  LegacyPlanHandler.team_plan",   0xf8,  offset_of!(LegacyPlanHandler, team_plan));
    chk("15/mem[6]  LPH.positioning_score",         0x990, offset_of!(LegacyPlanHandler, positioning_score));
    chk("15/mem[7]  Entity.ty",                     0x68,  offset_of!(Entity, ty));
    chk("15/mem[9]  Entity.x",                      0x660, offset_of!(Entity, x));
    chk("15/mem[10] Entity.y",                      0x668, offset_of!(Entity, y));
    chk("15/mem[11] SinglePlanBattle.sub_goal",     0x58,  offset_of!(SinglePlanBattle, sub_goal));
    chk("15/mem[12] SinglePlanBattle.dive_tower",   0x8c,  offset_of!(SinglePlanBattle, dive_tower));
    chk("15/logic   SinglePlanBattle.chats",        0x68,  offset_of!(SinglePlanBattle, chats));

    // ── /specs[16] max_range_nearly_can_use ────────────────────────
    chk("16/mem[11] Entity.stat_buff_cached.range", 0x438, offset_of!(Entity, stat_buff_cached) + offset_of!(BuffState, range));
    chk("16/mem[12] Entity.sbc.radius_mult",        0x470, offset_of!(Entity, stat_buff_cached) + offset_of!(BuffState, radius_mult));
    chk("16/mem[13] Entity.attack_effect",          0x490, offset_of!(Entity, attack_effect));
    chk("16/mem[18] Entity.skill_effect",           0x4c8, offset_of!(Entity, skill_effect));
    chk("16/mem[23] Entity.skill2_effect",          0x500, offset_of!(Entity, skill2_effect));
    chk("16/mem[24] Entity.ult_effect",             0x538, offset_of!(Entity, ult_effect));
    chk("16/mem[25] Entity.level",                  0x5c8, offset_of!(Entity, level));
    chk("16/mem[26] Entity.radius",                 0x680, offset_of!(Entity, radius));
    chk("16/mem[27] Effect.range",                  0x10,  offset_of!(Effect, range));
    chk("16/mem[28] Effect.growth_range",           0x18,  offset_of!(Effect, growth_range));
    chk("16/mem[29] Effect.target",                 0x28,  offset_of!(Effect, target));
    chk("16/mem[30] Effect.casting",                0x30,  offset_of!(Effect, casting));

    // ── /specs[17] DeathMatchBattle::new (43행 전량) ────────────────
    chk("17/mem[6]  DMB.support_target",   0x0,   offset_of!(DeathMatchBattle, support_target));
    // [private 필드 — offset_of! 불가] chk("17/mem[7]  DMB.region",           0x10,  offset_of!(DeathMatchBattle, region));
    // [private 필드 — offset_of! 불가] chk("17/mem[8]  DMB.well_runaway",     0x30,  offset_of!(DeathMatchBattle, well_runaway));
    // [private 필드 — offset_of! 불가] chk("17/mem[9]  DMB.death_focus",      0x40,  offset_of!(DeathMatchBattle, death_focus));
    // [private 필드 — offset_of! 불가] chk("17/mem[10] DMB.seal_basis",       0x50,  offset_of!(DeathMatchBattle, seal_basis));
    // [private 필드 — offset_of! 불가] chk("17/mem[11] DMB.hold_scene_basis", 0x70,  offset_of!(DeathMatchBattle, hold_scene_basis));
    // [private 필드 — offset_of! 불가] chk("17/mem[12] DMB.repo_scene_basis", 0x90,  offset_of!(DeathMatchBattle, repo_scene_basis));
    // [private 필드 — offset_of! 불가] chk("17/mem[13] DMB.flee_dir",         0xa8,  offset_of!(DeathMatchBattle, flee_dir));
    // [private 필드 — offset_of! 불가] chk("17/mem[14] DMB.far_noout_since",  0xc0,  offset_of!(DeathMatchBattle, far_noout_since));
    chk("17/mem[15] DMB.main_goal",        0xd0,  offset_of!(DeathMatchBattle, main_goal));
    chk("17/mem[16] DMB.sub_goal",         0xe8,  offset_of!(DeathMatchBattle, sub_goal));
    chk("17/mem[18] DMB.chats",            0xf8,  offset_of!(DeathMatchBattle, chats));
    chk("17/mem[19] DMB.start_tick",       0x110, offset_of!(DeathMatchBattle, start_tick));
    // [private 필드 — offset_of! 불가] chk("17/mem[20] DMB.flee_die",         0x118, offset_of!(DeathMatchBattle, flee_die));
    chk("17/mem[21] DMB.trade_lean",       0x120, offset_of!(DeathMatchBattle, trade_lean));
    // [private 필드 — offset_of! 불가] chk("17/mem[22] DMB.lean_last_tick",   0x128, offset_of!(DeathMatchBattle, lean_last_tick));
    // [private 필드 — offset_of! 불가] chk("17/mem[23] DMB.scene_change_tick",0x130, offset_of!(DeathMatchBattle, scene_change_tick));
    chk("17/mem[24] DMB.last_act_tick",    0x138, offset_of!(DeathMatchBattle, last_act_tick));
    chk("17/mem[25] DMB.idle_spec_tick",   0x140, offset_of!(DeathMatchBattle, idle_spec_tick));
    chk("17/mem[26] DMB.last_swing_tick",  0x148, offset_of!(DeathMatchBattle, last_swing_tick));
    // [private 필드 — offset_of! 불가] chk("17/mem[27] DMB.ep_follow_until",  0x150, offset_of!(DeathMatchBattle, ep_follow_until));
    // [private 필드 — offset_of! 불가] chk("17/mem[28] DMB.idle_prev_pos",    0x158, offset_of!(DeathMatchBattle, idle_prev_pos));
    // [private 필드 — offset_of! 불가] chk("17/mem[29] DMB.last_unseal_tick", 0x168, offset_of!(DeathMatchBattle, last_unseal_tick));
    chk("17/mem[30] DMB.with_dive",        0x170, offset_of!(DeathMatchBattle, with_dive));
    chk("17/mem[31] DMB.help_called",      0x171, offset_of!(DeathMatchBattle, help_called));
    chk("17/mem[32] DMB.dive_abandoned",   0x172, offset_of!(DeathMatchBattle, dive_abandoned));
    // [private 필드 — offset_of! 불가] chk("17/mem[33] DMB.dodge_commit",     0x173, offset_of!(DeathMatchBattle, dodge_commit));
    chk("17/mem[34] DMB.last_stand",       0x174, offset_of!(DeathMatchBattle, last_stand));
    chk("17/mem[35] DMB.had_ult_ready",    0x175, offset_of!(DeathMatchBattle, had_ult_ready));
    chk("17/mem[36] DMB.stance",           0x176, offset_of!(DeathMatchBattle, stance));
    chk("17/mem[37] DMB.tactic",           0x177, offset_of!(DeathMatchBattle, tactic));
    chk("17/mem[38] DMB.scene",            0x178, offset_of!(DeathMatchBattle, scene));
    // [private 필드 — offset_of! 불가] chk("17/mem[39] DMB.scene_last_from",  0x179, offset_of!(DeathMatchBattle, scene_last_from));
    chk("17/mem[40] DMB.dive_tower",       0x17a, offset_of!(DeathMatchBattle, dive_tower));
    // [private 필드 — offset_of! 불가] chk("17/mem[41] DMB.main_objective",   0x17b, offset_of!(DeathMatchBattle, main_objective));
    // [private 필드 — offset_of! 불가] chk("17/mem[42] DMB.lean_last_sign",   0x17e, offset_of!(DeathMatchBattle, lean_last_sign));

    // ── /specs[18] v3_epicops_buff_window ──────────────────────────
    chk("18/mem[0]  PlayerState.info.team",     0x930, offset_of!(PlayerState, info.team));
    chk("18/mem[1]  PlayerState.info.position", 0x9c0, offset_of!(PlayerState, info.position));
    chk("18/mem[2]  OperationData.cache",       0x0,   offset_of!(OperationData, cache));
    chk("18/mem[5]  AGWC.player_champion",      0x1e0, offset_of!(AbstractGameWithCache, player_champion));
    chk("18/mem[6]  Strategy.morgard_use",      0x4,   offset_of!(Strategy, morgard_use));
    chk("18/mem[7]  TeamPlan.chats(+cap)",      0xc0,  offset_of!(TeamPlan, chats));
    chk("18/mem[10] TeamPlan.eo_serpen_punish_issues", 0x410, offset_of!(TeamPlan, eo_serpen_punish_issues));
    // [private 필드 — offset_of! 불가] chk("18/mem[11] TeamPlan.v3_press_chat_line",      0x41e, offset_of!(TeamPlan, v3_press_chat_line));
    chk("18/mem[12] TeamPlan.objective",               0x41f, offset_of!(TeamPlan, objective));
    chk("18/aux     TeamPlan.next_respawn_tick",       0x378, offset_of!(TeamPlan, next_respawn_tick));

    // ── /specs[19] best_jungle_goal ────────────────────────────────
    chk("19/mem[0]  OperationData.cache",   0x0,   offset_of!(OperationData, cache));
    chk("19/mem[1]  OperationData.context", 0x8,   offset_of!(OperationData, context));
    chk("19/mem[2]  GameContext.pool",      0x0,   offset_of!(GameContext, pool));
    chk("19/mem[3]  GameContext.map",       0x20,  offset_of!(GameContext, map));
    chk("19/mem[9]  AGWC.player_champion",  0x1e0, offset_of!(AbstractGameWithCache, player_champion));
    chk("19/mem[10] PlayerState.info.team",     0x930, offset_of!(PlayerState, info.team));
    chk("19/mem[11] PlayerState.info.position", 0x9c0, offset_of!(PlayerState, info.position));
    chk("19/mem[12] Entity.x", 0x660, offset_of!(Entity, x));
    chk("19/mem[13] Entity.y", 0x668, offset_of!(Entity, y));
    chk("19/mem[8]  JungleCampState.next_respawn_tick", 0x18, offset_of!(JungleCampState, next_respawn_tick));

    // ── 15 의 iter_towers_without_nexus 6칸 순서 (logic 249행 주장) ──
    chk("15/logic AGWC.top_tower",     0x180, offset_of!(AbstractGameWithCache, top_tower));
    chk("15/logic AGWC.mid_tower",     0x1a0, offset_of!(AbstractGameWithCache, mid_tower));
    chk("15/logic AGWC.bottom_tower",  0x1c0, offset_of!(AbstractGameWithCache, bottom_tower));
    chk("15/logic AGWC.top_tower2",    0x190, offset_of!(AbstractGameWithCache, top_tower2));
    chk("15/logic AGWC.mid_tower2",    0x1b0, offset_of!(AbstractGameWithCache, mid_tower2));
    chk("15/logic AGWC.bottom_tower2", 0x1d0, offset_of!(AbstractGameWithCache, bottom_tower2));
    chk("15/logic AGWC.twin_towers",   0x130, offset_of!(AbstractGameWithCache, twin_towers));
    chk("15/logic AGWC.nexus",         0x170, offset_of!(AbstractGameWithCache, nexus));

    // ── Vec<Chat> 3워드 순서 (18 mem[7..9] / 17 mem[18]) ────────────
    // 5차 배치D 가 (cap, ptr, len) 로 확정한 것을 재확인한다.
    let mut v: Vec<Chat> = Vec::new();
    let w = |v: &Vec<Chat>| unsafe {
        let p = v as *const Vec<Chat> as *const usize;
        (*p, *p.add(1), *p.add(2))
    };
    let e = w(&v);
    println!("VEC\tempty\tw0={}\tw1={}\tw2={}", e.0, e.1, e.2);
    v.push(Chat::Repair(0));
    let a = w(&v);
    println!("VEC\tpush1\tw0={}\tw1={}\tw2={}\tcap_api={}\tlen_api={}", a.0, a.1, a.2, v.capacity(), v.len());
    chk("18/mem[7]  Vec<Chat> cap 워드", 0x0, if a.0 == v.capacity() { 0x0 } else { 0xff });
    chk("18/mem[9]  Vec<Chat> len 워드", 0x10, if a.2 == v.len() { 0x10 } else { 0xff });

    // ── 열거형 판별자 (consts 행) — 실제 값으로 태그 바이트를 읽는다 ──
    unsafe fn tag1<T>(v: &T) -> u8 { *(v as *const T as *const u8) }
    unsafe fn tag8<T>(v: &T) -> u64 { *(v as *const T as *const u64) }
    unsafe {
        chk("18/consts[3] Chat::Repair=23",      23, tag1(&Chat::Repair(0)) as usize);
        chk("18/consts[4] Chat::SerpenSetup=25", 25, tag1(&Chat::SerpenSetup(0)) as usize);
        chk("18/consts[5] Chat::Press=21",       21, tag1(&Chat::Press(LineType::Top, 0)) as usize);
        chk("18/consts[6] Chat::PressChange=22", 22, tag1(&Chat::PressChange(LineType::Top, 0)) as usize);
        chk("17/consts[1] Chat::Battle=3",        3, tag1(&Chat::Battle(0, 0)) as usize);
        chk("15/consts[5] BattleSubPlanGoal::KitingBack=3", 3, tag8(&BattleSubPlanGoal::KitingBack { focus: 0 }) as usize);
        chk("15/consts[6] BattleSubPlanGoal::RunAway=4",    4, tag8(&BattleSubPlanGoal::RunAway) as usize);
        chk("15/consts[7] BattleSubPlanGoal::End=7",        7, tag8(&BattleSubPlanGoal::End) as usize);
        chk("15/consts[0] BattlePlanGoal::TryKill=0",       0, tag8(&BattlePlanGoal::TryKill(0, 60)) as usize);
        chk("18/consts[2] MainObjective::Repair=7", 7,
            tag1(&game_ai::plan_legacy::team_plan::MainObjective::Repair) as usize);
        // 18 mem[13]: Serpen{phase:Setup, with_battle:true} = 1 / 1 / 1
        let s = game_ai::plan_legacy::team_plan::MainObjective::Serpen {
            phase: game_ai::plan_legacy::team_plan::ObjectPhase::Setup, with_battle: true };
        let sp = &s as *const _ as *const u8;
        println!("TAG\tMainObjective::Serpen(Setup,true)\tb0={}\tb1={}\tb2={}", *sp, *sp.add(1), *sp.add(2));
        chk("18/mem[13] Serpen tag", 1, *sp as usize);
        chk("18/mem[13] phase=Setup", 1, *sp.add(1) as usize);
        chk("18/mem[13] with_battle", 1, *sp.add(2) as usize);
        // Option<LineType> None 니치 = 0xff (18 consts[7])
        let n: Option<LineType> = None;
        chk("18/consts[7] Option<LineType>::None=0xff", 0xff, tag1(&n) as usize);
        // Option<MainObjective> None 니치 = 0xff (17 consts[3])
        let n2: Option<game_ai::plan_legacy::team_plan::MainObjective> = None;
        chk("17/consts[3] Option<MainObjective>::None=0xff", 0xff, tag1(&n2) as usize);
        chksz("Option<MainObjective>", 3, size_of::<Option<game_ai::plan_legacy::team_plan::MainObjective>>());
    }

    unsafe { println!("TOTAL\tOK={}\tMISMATCH={}", OK, NG); }
}
