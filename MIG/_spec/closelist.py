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

# ══════════ 3차 반증검증(2026-09-11)이 닫은 것 ══════════
# 근거 = RE\2026-09-11_20함수-3차반증검증-배치{A,B,C,D}.md
CLOSE += [
    # ── 배치 A ────────────────────────────────────────────────────────
    (0, u"linear_cast_range_with_margin", u"3차 배치A: history[4](a) 에 이미 답이 있었다(과열림)"),
    (0, u"get_input_target", u"3차 배치A: abstract_input.rs 345~655 확정, None 반환 정확히 3곳"),
    (0, u"safe_move_avoiding_enemy_well", u"3차 배치A: history[0]+[1] 에 이미 답이 있었다(과열림)"),
    (0, u"Effect::range_adjust", u"3차 배치D: vtable +0xe8 위임 + 반환 u64 확정"),
    (0, u"Entity::can_ult", u"3차 배치A: history[4](c) 에 이미 답이 있었다(과열림)"),
    (3, u"is_ignored_well_enemy", u"3차: shared.is_recent_visible + 우물 판정 확정"),
    (3, u"effect_cc_time", u"3차 배치A: 단일 vtable 디스패치 + version 완전 미사용(오라클 1,150회)"),
    (4, u"is_recent_visible", u"shared.is_recent_visible.blackboard_인덱스_의미 = 확정(과열림)"),
    # ── 배치 B ────────────────────────────────────────────────────────
    (5, u"end_reason", u"3차 배치B: 호출자 IR(v50_track_dive_episode)에서 0~8 의미 전량 확정"),
    (5, u"end_plan", u"3차 배치B: 문자열 매칭이 원문 + 오라클 get_name 8종"),
    (5, u"prev_holder_hp", u"3차 배치B: tcx 에 그 필드가 아예 없다"),
    (5, u"last_dive_abandon_tick", u"3차 배치B: 소비 게이트 8곳 전부 tick > 값+1+4×tps"),
    (7, u"hp_ratio", u"3차 배치B: 오라클 8/8, 경계 정확히 50/51"),
    (8, u"is_recent_visible", u"shared 에 완전 규정 + 영향 목록에 is_end 포함(과열림)"),
    (8, u"as_moba", u"3차 배치B: <Game as AbstractGame>::get_game_mode MIR 이 순수 함수"),
    (8, u"vtable", u"3차 배치B: 슬롯 = 0x20 + 8×(트레이트 선언 순서), divtable 6/6"),
    (8, u"15", u"3차 배치B: 오라클 13/13, 임계 정확히 900"),
    (9, u"enemy_minion_line_action_danger_damage_at", u"3차 배치B: 전문 확정(임계표 전량)"),
    (9, u"version", u"3차 배치B: 피호출자 2단 i64 poison + 오라클 diff 0 = 죽은 인자"),
    # ── 배치 C ────────────────────────────────────────────────────────
    (10, u"is_enemy_well_danger", u"3차 배치C: 전문 확정 + version 미사용(오라클 격자 전수)"),
    (10, u"can_enemy_hit_objective", u"3차 배치C: 25000 = 선형 거리 여유치 + 하드컷 19.6e9"),
    (10, u"objective_entity_id_for_main_objective", u"3차 배치C: 태그 전수 진리표"),
    (11, u"passive_plan", u"3차 배치C: tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값"),
    (11, u"player_count", u"3차 배치C: 값표 전수 → 비교값 5 확정(==5 vs >=5 는 표기 불가)"),
    (12, u"포맷", u"3차 배치C: shared.포맷템플릿_문법 + IR Debug::fmt store 3곳"),
    (13, u"is_top_side", u"3차 배치C: 오라클 실행 확정(ry_lt_x == !is_top_side)"),
    (13, u"bushes", u"3차 배치C: MapDef.bushes 값 사전(id 1~24 + 중심좌표)"),
    (14, u"has_near_line_enemy", u"3차 배치C: IR 전수 + 오라클 3/3"),
    (14, u"_lead", u"3차 배치C: 산출식 확정(team 부호 포함, 6/6)"),
    (14, u"positioning_score", u"3차 배치C: DWARF 로 확정(시그니처가 소스 39/40/41 3줄)"),
    # ── 배치 D ────────────────────────────────────────────────────────
    (15, u"TryKill", u"3차 배치D: __1 을 0~99999 로 흔들어도 산출 동일(노브 아님)"),
    (15, u"engage_requires_dive", u"3차 배치D: 합성 엔티티 2,073,600쌍 mismatch 0"),
    (16, u"battle.rs:2399", u"3차 배치D: 줄 길이 산술로 4블록 동시 일치 복원(재료 부재 아님)"),
    (17, u"base_sub_goal", u"3차 배치D: pub 전수 진리표(가시성 6축 배제)"),
    (18, u"Prepare", u"3차 배치D: 17 closed 와 같은 근거(Chat 튜플 variant = 이름 부재)"),
    (18, u"epic.rs", u"3차 배치D: closed 가 L654~657 복원 + 전제 반박(과열림)"),
    (18, u"v3_epic_group_line", u"3차 배치D: 진리표 80칸 + None 경로 도달 확인"),
    (19, u"get_camp_state", u"3차 배치D: 포인터 동일성 전수표 + 특례 2건"),
]
