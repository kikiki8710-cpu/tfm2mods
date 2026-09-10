# -*- coding: utf-8 -*-
u"""closelist — `open[]` 에서 **내려도 되는 항목의 명시 목록**. (2026-09-11)

## 왜 자동 판정이 아니라 목록인가
초판은 "resolved 와 식별자가 2개 이상 겹치면 닫힘"으로 자동 판정했는데, 그러면
`get_input_target 내부는 안 봄`(진짜 미탐색)·`base_sub_goal 내부`·`single_tower_dive_is_viable`
같은 **열린 항목까지 닫혔다.** 과하게 열어두는 것은 3차가 재확인만 하고 넘어가니 안전하지만,
**과하게 닫으면 3차가 그것을 '새 발견'으로 또 집는다** — 유저가 없애고 싶은 게 정확히 그것이다.
⟹ 기본값 = **열림**. 닫는 것은 ①본문에 해소 표기가 있거나 ②아래 목록에 있을 때만.

목록의 근거는 1·2차 반증검증 리포트가 "★STALE 삭제 / 해소" 로 명시한 항목뿐이다
(`_verify\\REPORT_{A,B,C,D}.md` · `RE\\2026-09-11_20함수-2차반증검증-배치*.md`).
키 = (함수 index, 그 항목을 특정하는 부분문자열).
"""

# (i, needle, 닫는 근거)
CLOSE = [
    # ── 00 ult ────────────────────────────────────────────────────────
    (0, u"param2 rnd(StdRng) 와 param6 positioning_score",
     u"1차 배치A: 통과 인자임이 확정(본문 역참조 0). 사실 서술이라 미확정이 아니다"),
    (0, u"Arc 페이로드 오프셋 계산", u"1차 배치A: ABI 접힘이라 constants 제외 결정 = 판정 완료"),
    (0, u"dyn EffectType vtable 슬롯 +0x118",
     u"1차 배치A: vt+0x110 = linear_cast_range_margin 을 L196 줄길이 ±0 으로 독립 재확증"),

    # ── 01 calculate_jungle_action_score ──────────────────────────────
    (1, u"camp_type.__0(usize, Entity+0x98)의 의미",
     u"1차 배치A: 팀 인덱스로 확정(JungleCampState::spawn 의 store 지점 확인)"),
    (1, u"is_jungle()·is_nexus()·is_minion() 의 소스 본문",
     u"2차 배치A: mirdump_game_core 에 네 술어 MIR 전문 — is_jungle 1377(인자 usize)·"
     u"is_any_type_minion 1260·is_nexus·is_tower"),
    (1, u"527·529·532",
     u"2차 배치A: 줄 길이로 전부 설명된다(소멸한 코드 없음)"),

    # ── 02 AttackNexusPlan::sub_plan ──────────────────────────────────
    (2, u"Position::as_index", u"2차 배치A: MIR entity.rs:580~581 = discriminant as usize, 재배치 없음"),
    (2, u"is_in_heal_area", u"2차 배치A: L38 ±0 복원(들여쓰기 4 보정)"),
    (2, u"buf.a", u"2차 배치A: bumpalo Vec 레이아웃 확정(두 번째 워드가 Bump 참조)"),

    # ── 03 defensive_crisis ───────────────────────────────────────────
    (3, u"check_kill_die_tick 반환값",
     u"1차 배치A: resolved 에서 die_tick 공식 확정 + 2차에서 game_ai::check_kill_die_tick 직접 호출(반환 60)"),

    # ── 04 handle_line_defense ────────────────────────────────────────
    (4, u"morgard_exists",
     u"1차 배치A + 2차: 본문 = context.tutorial.spawn_epic() 한 줄, L544/L549 표기 확정, "
     u"{0,7,8} 오라클 9/9"),
    (4, u"champions", u"2차 배치A: tcx sig + 본체 확인, 2번째 인자 = 팀. 오라클 len()==5"),
    (4, u"MobaMode", u"2차 배치A: MobaMode 640B, +0x240 epic_minion_buff_time[2], 오라클 600 반환"),
    (4, u"Blackboard 배열을", u"1차 배치A: _shared.is_recent_visible 로 인덱스 의미 확정"),

    # ── 05 v50_fold_dive_episode ──────────────────────────────────────
    (5, u"in_range_ticks==0", u"1차 배치B: L131 확정(132~137 은 한국어 주석 6줄)"),
    (5, u"BigPlan::get_name 내부", u"1차 배치B resolved[1] 전수 표 + 2차 오라클 8종 실행"),

    # ── 06 v2_response_retreat_stance ─────────────────────────────────
    (6, u"blackboard 인덱스가 1-team", u"_shared.is_recent_visible 확정(영향 목록에 이 함수 포함)"),
    (6, u"min_by_key", u"1차 배치B: is_le → 동점이면 먼저 나온 원소, IR 확정"),
    (6, u"별개 클로저", u"1차 배치B: mangled 심볼이 같은 DefId 아래 클로저 #0/#2 로 확정"),
    (6, u"RunAway", u"1차 배치B resolved[3]: undef 페이로드 확정"),

    # ── 07 EpicHuntAndBattlePlan::sub_plan ────────────────────────────
    (7, u"epic_ally_tick", u"1차 배치B resolved[0] 확정 + 2차 tcxdict 오프셋 전건 일치"),
    (7, u"upgrade_item", u"1차 배치B resolved[1] 확정"),
    (7, u"Recall", u"1차 배치B resolved[2]: sret 페이로드 = ZST 확정"),
    (7, u"MobaMode 의 live_list 시작 오프셋",
     u"2차 배치B: tcxdict --deep 으로 cap/ptr/len/next_respawn_tick 확정"),

    # ── 08 is_end ─────────────────────────────────────────────────────
    (8, u"is_recent_visible 본체", u"_shared 에 판정식·120틱 확정"),
    (8, u"blackboard 인덱스", u"resolved[1] 이 _shared 포인터로 확정"),
    (8, u"MainObjective", u"resolved[2] + 2차 오라클 objective 진리표"),
    (8, u"release_to_passive", u"resolved[3] 전문 확정"),
    (8, u"Vec<usize> 배치", u"2차 배치B: tcxdict --deep 으로 cap@+0x0 확정"),

    # ── 09 check_favorable_engage_formation ───────────────────────────
    (9, u"enemy_minion_line_action_danger_damage_at 의 내부",
     u"1차 배치B resolved[0] 전문 확정 + 2차 인자 전건 IR 재확인"),
    (9, u"fountains", u"2차 배치B: tcx 4-튜플 + 오라클 실측 좌표 + 재현 400/400"),

    # ── 10 should_end_object_finish_kill_priority_battle ──────────────
    (10, u"blackboard 인덱스의 의미", u"_shared.is_recent_visible 확정"),
    (10, u"Blackboard::is_recent_visible 본체",
     u"2차 배치C: _gcbc/g07.ll:157005~157050 에 있었다(_gaibc 에 declare 만)"),
    (10, u"strategy 가 난수를 소비", u"2차 배치C: RNG 무영향 실행 확정(난수열 5/5 동일)"),

    # ── 12 handle_chat ────────────────────────────────────────────────
    (12, u"position_exists", u"2차 배치C: 45칸 전수 일치"),

    # ── 13 target_bush_v30 ───────────────────────────────────────────
    (13, u"is_top_side 의 극성", u"2차 배치C: 오라클 실행 확정(ry_lt_x == !is_top_side)"),
    (13, u"어느 부시 테이블의 인덱스", u"resolved[0]: MapDef.bushes(+0x1c98) 값으로 확정"),

    # ── 14 LineGankerPlan::update ────────────────────────────────────
    (14, u"ganker.rs:279 소스 조건의 극성", u"2차 배치C: 오라클 실행 확정"),
    (14, u"has_near_line_enemy", u"2차 배치C: IR 전수독해 + 오라클 3/3 MATCH 로 판정식 확정"),
    (14, u"is_tower2", u"1차/2차: 판별자==2 && Tower.ty>4 구조 확정"),

    # ── 15 single_try_engage ─────────────────────────────────────────
    (15, u"iter_towers_without_nexus 가 반환하는 6칸",
     u"1차 배치D: 6칸 순서 + twin_towers 꼬리 확정, 2차 오라클 팀당 10개"),
    (15, u"rnd(p3)·debug(p7)", u"사실 서술(본문 역참조 0) — 미확정 항목이 아니다"),
    (15, u"블록 %53~%83 의 배열부 상한 6", u"판정 상수 아님으로 결론 = 판정 완료"),
    (15, u"min_by_key 의 fold 본체", u"1차: 추가 술어 없이 단순 최소로 확인"),

    # ── 16 max_range_nearly_can_use ──────────────────────────────────
    (16, u"cooldown", u"resolved: 남은 틱 확정 + 2차 attack_cooldown 14-arm MIR 전량"),
    (16, u"Effect::range_adjust 본체", u"resolved: 분기표 확정. 2차에서 vtable +0xe8 위임까지 확정"),
    (16, u"CastingTarget::check", u"resolved: 14종 규칙표 확정"),
    (16, u"%33", u"1차 배치D: GameSetting.tick_per_second(0x12f8) 확정"),

    # ── 17 DeathMatchBattle::new ─────────────────────────────────────
    (17, u"0x17c/0x17d", u"1차 배치D: MainObjective 페이로드(phase/with_battle) 확정"),
    (17, u"Vec::new 의 dangling", u"판정값 아님으로 결론 = 판정 완료"),
    (17, u"Chat::Battle 의 두 번째 필드",
     u"1차 배치D: Chat 은 튜플 variant 라 필드명이 애초에 없다(도구 한계 아님)"),
    (17, u"Prepare", u"동상 — 튜플 variant"),

    # ── 18 v3_epicops_buff_window ────────────────────────────────────
    (18, u"리터럴 true", u"2차 배치D: WavePriorityObject(0=Morgard/1=Serpen) 확정"),
    (18, u"player.strategy", u"1차 배치D: _gcbc/g15.ll:130480 define 확정(+rnd readnone)"),
    (18, u"player_champion 의 원소 타입", u"1차 배치D: tcxdict 로 [[Option<&Entity>;5];2] 확정"),
    (18, u"!dbg", u"resolved 가 '소실되지 않았다'로 정면 반박"),
    (18, u"v3_epic_group_line", u"resolved 가 두 함수 모두 확정"),

    # ── 19 best_jungle_goal ──────────────────────────────────────────
    (19, u"is_cleared 본문", u"resolved[0]: 판정식 전문 + camp_idx 확정"),
    (19, u"MapDef::camp_pos", u"resolved[3]: _gcbc 에서 확정(_gaibc 에 declare 만이었다)"),
    (19, u"SliceRandom", u"resolved[2]: _gcbc/g09.ll:137367~137440 확정"),
    (19, u"GameMode", u"1차 배치D: tcxdict --enum 으로 판별자 확정(tag==0=Moba)"),
    (19, u"fold", u"resolved[1]: 두 fold 정체·키·임계상수 0건 확정"),
]


def closed_reason(i, txt):
    for j, needle, why in CLOSE:
        if j == i and needle in txt:
            return why
    return None
