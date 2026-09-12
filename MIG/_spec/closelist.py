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
    (2, u"앞 24B(ptr/ptr/cap)", u"2차 배치A: bumpalo Vec 레이아웃 확정(두 번째 워드가 Bump 참조)"),

    # ── 03 defensive_crisis ───────────────────────────────────────────
    (3, u"반환값 die 가",
     u"1차 배치A: resolved 에서 die_tick 공식 확정 + 2차에서 game_ai::check_kill_die_tick 직접 호출(반환 60)"),

    # ── 04 handle_line_defense ────────────────────────────────────────
    (4, u"morgard_exists",
     u"1차 배치A + 2차: 본문 = context.tutorial.spawn_epic() 한 줄, L544/L549 표기 확정, "
     u"{0,7,8} 오라클 9/9"),
    (4, u"champions", u"2차 배치A: tcx sig + 본체 확인, 2번째 인자 = 팀. 오라클 len()==5"),
    (4, u"MobaMode", u"2차 배치A: MobaMode 640B, +0x240 epic_minion_buff_time[2], 오라클 600 반환"),
    (4, u"배열을 **적팀 인덱스", u"1차 배치A: _shared.is_recent_visible 로 인덱스 의미 확정"),

    # ── 05 v50_fold_dive_episode ──────────────────────────────────────
    (5, u"in_range_ticks==0", u"1차 배치B: L131 확정(132~137 은 한국어 주석 6줄)"),
    (5, u"BigPlan::get_name 내부", u"1차 배치B resolved[1] 전수 표 + 2차 오라클 8종 실행"),

    # ── 06 v2_response_retreat_stance ─────────────────────────────────
    (6, u"blackboard 인덱스가 1-team", u"_shared.is_recent_visible 확정(영향 목록에 이 함수 포함)"),
    (6, u"min_by_key", u"1차 배치B: is_le → 동점이면 먼저 나온 원소, IR 확정"),
    (6, u"두 개의 별도 클로저인지", u"1차 배치B: mangled 심볼이 같은 DefId 아래 클로저 #0/#2 로 확정"),
    (6, u"RunAway", u"1차 배치B resolved[3]: undef 페이로드 확정"),

    # ── 07 EpicHuntAndBattlePlan::sub_plan ────────────────────────────
    (7, u"epic_ally_tick", u"1차 배치B resolved[0] 확정 + 2차 tcxdict 오프셋 전건 일치"),
    (7, u"upgrade_item", u"1차 배치B resolved[1] 확정"),
    (7, u"Recall", u"1차 배치B resolved[2]: sret 페이로드 = ZST 확정"),
    (7, u"MobaMode 의 live_list 시작 오프셋",
     u"2차 배치B: tcxdict --deep 으로 cap/ptr/len/next_respawn_tick 확정"),

    # ── 08 is_end ─────────────────────────────────────────────────────
    (8, u"is_recent_visible 의 판정 조건", u"_shared 에 판정식·120틱 확정"),
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
    (12, u"chat_allowed(GameContext", u"2차 배치C: 45칸 전수 일치"),

    # ── 13 target_bush_v30 ───────────────────────────────────────────
    (13, u"is_top_side 의 극성", u"2차 배치C: 오라클 실행 확정(ry_lt_x == !is_top_side)"),
    (13, u"어느 부시 테이블의 인덱스", u"resolved[0]: MapDef.bushes(+0x1c98) 값으로 확정"),

    # ── 14 LineGankerPlan::update ────────────────────────────────────
    (14, u"ganker.rs:279 소스 조건의 극성", u"2차 배치C: 오라클 실행 확정"),
    (14, u"has_near_line_enemy", u"2차 배치C: IR 전수독해 + 오라클 3/3 MATCH 로 판정식 확정"),
    (14, u"is_tower2", u"1차/2차: 판별자==2 && Tower.ty>4 구조 확정"),

    # ── 15 single_try_engage ─────────────────────────────────────────
    (15, u"iter_towers_without_nexus 가 반환하는 6칸",
     u"1차 배치D: 6칸 순서 + twin_towers 꼬리 확정. ★3차 배치D 재측정 = **팀당 8개(6칸+twin 2), 좌표중복 0** — ~~2차 오라클 팀당 10개~~ 는 init_tower 재호출 오염값이었다(포인터 동일성으로 확정)"),
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
    (19, u"map 클로저(s_0)", u"resolved[2]: _gcbc/g09.ll:137367~137440 확정"),
    (19, u"GameMode", u"1차 배치D: tcxdict --enum 으로 판별자 확정(tag==0=Moba)"),
    (19, u"fold", u"resolved[1]: 두 fold 정체·키·임계상수 0건 확정"),
]


def closed_reason(i, txt, warn=None):
    u"""그 항목을 닫은 근거. ★**모든 일치 중 「가장 긴 needle」을 고른다.** (11차 배치A·B·C·D 전원 적발)

    초판은 **첫 일치**를 돌려줬는데, `CLOSE` 는 라운드순으로 `+=` 되므로
    **언제나 가장 이른 라운드의 근거가 이겼다.** 그래서:
      · `12` 의 근거가 2차 「45칸」(다른 함수 결과)으로 뜨고, 3차의 「297칸 전수 진리표」는 **영원히 가려졌다**
      · `(11,"passive_plan")` **하나**가 `closed[4]/[5]/[6]` 세 물음에 **같은 why** 를 달았다
      · `(15,"engage_requires_dive")` 가 그 이름을 **언급만 하는** 다른 항목에 붙었다
      · `06 closed[5]` 의 근거가 본문에 우연히 있던 `min_by_key` 때문에 바뀌었다
    실측: `closed` 151행 중 **26행**이 같은 명세 안에서 why 를 공유했고, 중복 `(i,needle)` 쌍이 **6개**였다.

    ★`closelist.py` 자신이 4차에 「needle 은 그 항목의 **고유 문면**으로 잡는다」는 교훈을 적어 뒀는데
      그 형태가 살아 있었다 — **규칙을 적는 것과 기계가 강제하는 것은 다르다.**

    ⟹ ①모든 일치를 모아 **최장 needle** 채택(짧은 것이 긴 것을 삼키지 못한다)
       ②2건 이상 일치하면 `warn` 에 실어 **가려진 근거가 있음을 드러낸다**(조용히 고르지 않는다)
    """
    hits = [(k, needle, why) for k, (j, needle, why) in enumerate(CLOSE)
            if j == i and needle in txt]
    if not hits:
        return None
    # ★①needle 이 길수록 **그 항목 고유**의 문면이다  ②길이가 같으면 **나중 라운드**가 이긴다.
    #   `CLOSE` 는 라운드순 `+=` 이라 뒤에 있을수록 최신이고, 중복 `(i,needle)` 쌍이 **6개** 있다
    #   (`12/chat_allowed(GameContext` 는 2차 「45칸」 ↔ 3차 「297칸 전수 진리표」).
    hits.sort(key=lambda t: (-len(t[1]), -t[0]))
    if warn is not None and len(hits) > 1:
        warn.append((i, hits[0][1], [n for _, n, _ in hits[1:]]))
    return hits[0][2]


def shadowed(i, txt):
    u"""그 항목에 **일치했지만 채택되지 않은** needle 들. `audit()` 이 이걸 봐야 한다."""
    hits = [n for j, n, _ in CLOSE if j == i and n in txt]
    if len(hits) <= 1:
        return []
    hits.sort(key=lambda n: -len(n))
    return hits[1:]

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
    (8, u"constants 에 넣지 않고 knobs 에만", u"3차 배치B: 오라클 13/13, 임계 정확히 900"),
    # ★B-E3 정정(4차 배치B): needle 이 넓어 **version 항목(unknown[0])에 잘못 붙었다**.
    #   unknown[0] 은 `... enemy_minion_line_action_danger_damage_at 로 그대로 전달만 된다` 라고
    #   이 이름을 **언급만** 하는데, 넓은 needle 이 그걸 먹어서 `closed[0].why` 가
    #   "전문 확정(임계표 전량)" = closed[1](미니언 함수)의 근거를 달고 있었다.
    #   ⟹ 교훈: **needle 은 그 항목의 고유 문면으로 잡는다**(함수명은 여러 항목이 언급한다).
    (9, u"enemy_minion_line_action_danger_damage_at 의 내부(반환 damage 의 단위",
        u"3차 배치B: 전문 확정(임계표 전량)"),
    (9, u"version(p1) 이 실제로 무엇을 가르는지",
        u"3차 배치B + 4차 배치B 재실측: 피호출자 2단 i64 poison + version 12종 diff=0 = 죽은 인자"),
    # ── 배치 C ────────────────────────────────────────────────────────
    (10, u"is_enemy_well_danger", u"3차 배치C: 전문 확정 + version 미사용(오라클 격자 전수)"),
    (10, u"can_enemy_hit_objective", u"3차 배치C: 25000 = 선형 거리 여유치 + 하드컷 19.6e9"),
    (10, u"objective_entity_id_for_main_objective", u"3차 배치C: 태그 전수 진리표"),
    (11, u"passive_plan", u"3차 배치C: tcx sig 가 (BigPlan, u8) — 뒤 8B 는 별도 반환값"),
    (11, u"player_count", u"3차 배치C: 값표 전수 → 비교값 5 확정(==5 vs >=5 는 표기 불가)"),
    (12, u"포맷", u"3차 배치C: shared.포맷템플릿_문법 + IR Debug::fmt store 3곳"),
    (13, u"is_top_side", u"3차 배치C: 오라클 실행 확정(ry_lt_x == !is_top_side)"),
    (14, u"MapDef.bushes 배열의 값 사전", u"3차 배치C: MapDef.bushes 값 사전(id 1~24 + 중심좌표)"),
    (14, u"has_near_line_enemy", u"3차 배치C: IR 전수 + 오라클 3/3"),
    (14, u"_lead", u"3차 배치C: 산출식 확정(team 부호 포함, 6/6)"),
    (14, u"positioning_score", u"3차 배치C: DWARF 로 확정(시그니처가 소스 39/40/41 3줄)"),
    # ── 배치 D ────────────────────────────────────────────────────────
    (15, u"TryKill", u"3차 배치D: __1 을 0~99999 로 흔들어도 산출 동일(노브 아님)"),
    (15, u"engage_requires_dive", u"3차 배치D: 합성 엔티티 2,073,600쌍 mismatch 0"),
    (16, u"battle.rs:2399", u"3차 배치D: 줄 길이 산술로 4블록 동시 일치 복원(재료 부재 아님)"),
    (17, u"base_sub_goal", u"3차 배치D: pub 전수 진리표(가시성 6축 배제)"),
    (17, u"Prepare", u"3차 배치D: 17 closed 와 같은 근거(Chat 튜플 variant = 이름 부재)"),
    (18, u"epic.rs", u"3차 배치D: closed 가 L654~657 복원 + 전제 반박(과열림)"),
    (18, u"v3_epic_group_line", u"3차 배치D: 진리표 80칸 + None 경로 도달 확인"),
    (19, u"get_camp_state", u"3차 배치D: 포인터 동일성 전수표 + 특례 2건"),
]

# ── 3차 후속: 본문이 스스로 "닫힘/확정"이라고 말하는데 open 에 남아 있던 3건 ──
# (v2 still_unknown 9건 중 3건. 배치 D 가 P-14 로 18 을 지적했고, 대조해 보니 11·14 도 같았다.
#  DONE_MARK 자동 판정이 "부재 확정"·"죽은 variant 로 닫힘"·"브리핑 오류 정정" 같은
#  다른 표현을 못 잡은 것이다 — 표현 변형을 못 쫓는 것은 2차의 stalecheck 과 같은 결함.)
CLOSE += [
    (11, u"MF_SRC_NAMES",
     u"부재 확정 · 재탐색 금지(2026-09-11 범위 확장 완료: SDK deps rlib/rmeta + IR 3 + exe + "
     u"게임 번들 2.95GB/zlib 1,625조각 → 히트 0). 어휘는 **재료 부재(전 범위)** 가 정확하다"),
    (14, u"lead < 3",
     u"이 항목은 미확정 질문이 아니라 **메인 세션 브리핑 오류의 정정 기록**이다"
     u"(LineGankerPlan::update 에 lead 오프셋 0건, 실제 소비처는 다른 4곳). history 로 가는 것이 맞다"),
    (18, u"CommitAfterJoin",
     u"죽은 variant 로 닫힘 — `_gaibc` 전량 gep 체인 누적 스캔 결과 값 1 을 쓰는 곳이 없다. "
     u"어휘는 **재료 부재(범위 = _gaibc·_gcbc·_gvbc)** 가 정확하다(3차 배치D P-14 지적)"),
]


# ══════════════════════════════════════════════════════════════════════
# ★감사기 — needle 이 **아무것도 못 닫으면 조용히 no-op** 이 되는 것을 막는다.
#
# 왜: 4차 배치 C 가 전수 확인한 결과 **CLOSE 103개 중 10개가 아무것도 닫지 않았다**
#     (index 오기 4 + 문면 불일치 6). 그런데 나는 "닫았다"고 보고했다.
#     `tcxaudit` 오탐 · `mem.chk` 449행 조회실패와 **같은 형태의 실패**다 —
#     도구가 조용히 아무것도 안 하는데 결과는 성공처럼 보인다.
# ⟹ 이제 `mkspec3.py` 가 매 빌드마다 이걸 돌리고, 죽은 needle 이 있으면 **크게 찍는다.**
# ══════════════════════════════════════════════════════════════════════
def audit(specs):
    u"""반환 = {dead, wrongidx, over}
    dead     = 어느 index 에서도 안 맞는 needle (문면 불일치)
    wrongidx = 등록한 index 에선 안 맞고 다른 index 에서 맞는 needle (index 오기)
    over     = 한 needle 이 같은 index 의 여러 항목을 동시에 닫는 것(감사 흔적이 뭉개진다)
    """
    dead, wrongidx, over = [], [], []
    for j, (i, needle, why) in enumerate(CLOSE):
        if i >= len(specs):
            dead.append((j, i, needle, u"index 범위 밖"))
            continue
        pool = [x.get("q") or u"" for x in specs[i]["open"] + specs[i]["closed"]]
        hit = [q for q in pool if needle in q]
        if not hit:
            others = [k for k in range(len(specs))
                      if any(needle in (x.get("q") or u"")
                             for x in specs[k]["open"] + specs[k]["closed"])]
            (wrongidx if others else dead).append((j, i, needle, others))
        elif len(hit) > 1:
            over.append((j, i, needle, len(hit)))

    # ★★`shadowed` — **일치했지만 채택되지 않은 근거.** (11차 배치B·C·D 적발)
    #   `audit()` 이 `dead`/`wrongidx`/`over` 만 보던 탓에, **더 강한 근거가 짧은 needle 에
    #   가려져 있는 것**을 아무도 못 봤다. 실측 **22건**(3차의 「297칸 전수 진리표」가
    #   2차의 「45칸」 뒤에 가려져 있던 것이 대표 사례).
    #   ⟹ 채택되지 않은 것이 **더 강한 근거일 수 있으니** 사람이 한 번 보게 만든다.
    #   ⛔중복 `(i, needle)` 쌍은 그 자체로 결함이다(실측 6개) — 여기서 같이 센다.
    shadow, dupkey = [], []
    seen = {}
    for j, (i, needle, why) in enumerate(CLOSE):
        if (i, needle) in seen:
            dupkey.append((j, i, needle, seen[(i, needle)]))
        else:
            seen[(i, needle)] = j
    for i, sp in enumerate(specs):
        for x in sp["open"] + sp["closed"]:
            q = x.get("q") or u""
            s = shadowed(i, q)
            if s:
                shadow.append((i, q[:40], s[:3]))
    return {"dead": dead, "wrongidx": wrongidx, "over": over,
            "shadowed": shadow, "dupkey": dupkey}


# ══════════ 4차 반증검증(2026-09-11)이 닫은 것 ══════════
# 근거 = RE\2026-09-11_20함수-4차반증검증-배치{A,B,C,D}.md
# ⚠needle 은 v3 원문에서 복사했고 위 audit() 가 매 빌드마다 검사한다.
CLOSE += [
    # ── 배치 A (00~04) — 실오류 0, 실질규명 6 + 오분류 11 ─────────────
    (1, u"value 의 단위",
     u"4차 배치A(ev5→ev2): 전량 복원 + game_core::utils::get_damage(pub·mir=True) MIR ±0. "
     u"반환은 HP와 같은 축의 절대 피해량 = max(ad*100/(100+def),1)+max(ap*100/(100+mr),1). 오라클 26/26"),
    (1, u"이펙트 종류별", u"4차 배치A: 차등의 소재 = Effect+0x0 ty vtable 3슬롯 + Effect+0x2c attack_type"),
    (1, u"어떤 액션 후보 루프에서 t 가 공급", u"4차 배치A: 유일 호출자 = JungleSubPlan::score(jungle.rs:152), 3사이트는 L159/171/183"),
    (1, u"writes 가 빈 배열인 이유", u"4차 배치A: 질문이 아니라 규칙 적용 기록(확정 서술)"),
    (2, u"gep 오프셋 0 이 접혀", u"4차 배치A: 판정 상수 아님으로 결론 = 판정 완료"),
    (2, u"vtable 디스패치를 하나도", u"4차 배치A: 확정 서술(질문 아님)"),
    (3, u"die_imminent 의 곱수 2", u"4차 배치A: 본문이 스스로 '확인된 사실'이라 말한다"),
    (3, u"명시적 is_empty 검사인지",
     u"4차 배치A: 소스에 명시적 is_empty() 확정 — inlinedAt buff_value.rs:30 + "
     u"check_kill_die_tick(invoke·&mut)이 건너뛰어지므로 LLVM 접힘일 수 없다"),
    (3, u"반환값 die 가", u"4차 배치A: history[2] 가 이미 확정(3차에서 die_tick 공식 전문)"),
    (4, u"get_game_mode",
     u"4차 배치A: 구현 정확히 4개(tcx), ExpectedGame 은 순수 위임이라 종단 3개이고 Game 만 Moba(MIR)"),
    (4, u"핸들러",
     u"4차 배치A: 「핸들러」 = TeamPlan(1064B). +0x41f objective 판별자 / +0x420 페이로드 태그. "
     u"true⟹DefenseLine(line)(3,1) / false⟹Defense(2)"),
    (4, u"min_by_key` 의 fold 본체", u"4차 배치A: history[0] 이 '★완전히 동일'로 이미 닫음"),
    # ── 배치 B (05~09) — 3차분이 닿지 않았던 범위 ────────────────────
    (5, u"_version(p2)/_tps(p3)", u"3차 배치B: 호출자 IR(v50_track_dive_episode)에서 0~8 의미 전량 확정"),
    (6, u"거리 임계 40000000001",
     u"3차/4차 배치B: 담당 범위 밖 상수 규칙 적용 기록(확정 서술) + 소스 표기 `<= 200000*200000` 확정"),
    (6, u"calls` 에 없다", u"3차 배치B: 규칙 적용 기록(확정 서술)"),
    (6, u"self(&LegacyPlanHandler)의 용도",
     u"4차 배치B: 본문·클로저 어디서도 LegacyPlanHandler 필드를 안 읽는다는 관측이 곧 답 — "
     u"판정 무영향. 소스에 self 표기가 있었는지는 표기 불가"),
    (7, u"champ 을 얻는 인덱싱",
     u"3차 배치B: resolved 에서 duration 으로 확정 + tcxdict 오프셋 전건 일치"),
    (7, u"vtable 슬롯 0x40/0x1f0", u"3차 배치B: resolved 확정"),
    (7, u"_debug(p7) 은 readnone", u"3차 배치B: ZST 확정"),
    (8, u"focus_epic_only", u"3차 배치B: 같은 플랜의 sub_plan 이 전부 읽는다"),
    (9, u"1321행", u"3차/4차 배치B: 바깥 || 순서만 재료 부재, 안쪽 && 는 select 로 순서 확정"),
    # ── 배치 C (10~14) — 전부 같은 명세 안에 답이 있다 ────────────────
    (10, u"25000", u"3차 배치C: 선형 거리 여유치(is_in_range_ex 8번째 인자) 확정"),
    (10, u"divtable", u"4차 배치C: 진짜 Game 의 &dyn 팻포인터로 런타임 대조 4/4 일치 — 슬롯 인덱스는 impl 무관"),
    (11, u"_ => true", u"4차 배치C: history[0] — IR switch 4·5·6 명시 case + default unreachable"),
    (11, u"ptr 부분(GameMode 페이로드)",
     u"4차 배치C: history[1] — 16B, tag 0/1/2 → &MobaMode/&SingleLaneMode/&DeathMatchMode"),
    (11, u"두 번 호출",
     u"4차 배치C(ev3): history[2] — 구체 impl 3개 전부 memory(none) ⟹ 2회 호출이 다를 수 없다. 사고 아님"),
    (12, u"player_count", u"4차 배치C: history[0] 값표 확정, 비교값 5(==5 vs >=5 만 표기 불가)"),
    (12, u"chat_allowed(GameContext", u"3차 배치C: 297칸 전수 진리표"),
    (12, u"handle_chat_inner(m13.ll",
     u"4차 배치C: history[2]/[10]/[12] 가 31-case 디스패치 + arm별 계산식 전부 기록. "
     u"★내 브리핑의 「여러 라운드째 미탐색」은 사실과 달랐다 — knobs 12개가 inner 본문을 값까지 서술한다"),
    (12, u"chat.rs:18 의 소스 형태",
     u"4차 배치C: 조기반환형 확정 — 줄 길이 독립 재확증(L18=47 = indent4 + `if !…is_enabled() {` 43자 정확)"),
    (12, u"misunderstood(p8)", u"4차 배치C: history[4] — 플랜 판정 불개입, chat.rs:598 기록부만"),
    (12, u"PendingTraceEvent 원소 크기", u"4차 배치C: history[6] — 순수 원소 크기, 규칙 적용 기록"),
    (14, u"setup_limit(0x18)/wait_limit(0x20)",
     u"4차 배치C(ev5→2): 소비처는 is_end **만**(next_plan gep 0건). "
     u"is_end = tick>=wait_limit ‖ phase==Cancel ‖ (phase==WaitResponse && tick>=setup_limit). 오라클 36/36"),
    (14, u"Chat 원소의 나머지 22바이트",
     u"4차 배치C: tcxdict --enum Chat — Cancel 은 태그@+0x0 + CancelReason@+0x1, 나머지는 그 variant 의 패딩. 24B 원시덤프 실측"),
    (13, u"세 포인터 인자가 fastcc 인자승격",
     u"4차 배치C(ev3): 간접증거→직접증거. 호출부 m10.ll:11778~11791 에 gep 그대로"),
    (13, u"blackboard",
     u"4차 배치C: 선택기 3종 중 target_bush 만 blackboard 를 본다(cover.rs:196 / ganker.rs:351)"),
    # ── 배치 D (15~19) ───────────────────────────────────────────────
    (15, u"sub_goal 을 어떤 조건으로", u"4차 배치D: single_tower_dive_is_viable 본체 전량 + 판별식 game==mine 12/12"),
    (17, u"AbstractGame vtable",
     u"4차 배치D: 구현체 정확히 3개(Game/SingleLaneGame/DeathMatchGame) · 정적 vtable 6슬롯 × 3구현체 18/18 일치"),
    (18, u"dienum 이 준 이름이 __0/__1", u"4차 배치D: /specs[17]/closed 와 동상(Chat 튜플 variant = 이름 부재)"),
    (19, u"map 클로저(s_0)", u"4차 배치A/C: 항등 사상 확인 기록(확정 서술)"),
    (19, u"배열 길이 4", u"4차: SPEC_GUIDE §3 규칙 적용 기록(확정 서술)"),
]

# ══════════ specgate **G7**(과열림 게이트)이 스스로 찾아낸 것 ══════════
# G7 을 4차 반영 중에 신설하고 처음 돌렸더니, 4차 배치 D 가 보고한 `open[2]` 와는
# **다른 항목**을 하나 더 잡았다 — `history` 에 「★확정」으로 답이 있는데 열려 있었다.
# 사람이 전수 확인한 뒤에도 남는 것이 있다는 뜻이고, 그게 이 게이트를 만든 이유다.
CLOSE += [
    # ★5차 착수 전 재감사(2026-09-11)에서 잡았다. **4차 배치A 가 이미 지목한 것**인데
    #   내 patch4 가 배치 A 항목을 통째로 빠뜨려서 5차 직전까지 열려 있었다.
    #   G7 이 못 잡은 이유 = 초판 G7 은 **같은 스펙의 `history`/`closed` 만** 봤고 `shared` 를 안 봤다.
    #   ⟹ `shared` 는 여러 함수가 공유하는 사실의 자리라, 함수별 이력만 보면
    #      「다른 함수가 이미 규명한 것」을 영원히 못 닫는다. G7 에 규칙 A 를 신설해 막았다.
    (3, u"self 가 '적팀이 보는 시야판'인지",
     u"4차 배치A 지적 → `shared.is_recent_visible.blackboard_인덱스_의미` 에 ★확정으로 답이 있다: "
     u"`Blackboard[T].last_visible[pos]` = **팀 (1−T) 가 팀 T 의 pos 챔피언을 마지막으로 본 틱** "
     u"(내 팀 블랙보드 = 적이 나에 대해 아는 것). 근거 = `Blackboard::update` 호출부 "
     u"`_gcbc/g15.ll:249663` 이 team 0..1 로 돌며 자기 팀 챔프를 `is_visible(1-team, …)` 으로 묻는다 "
     u"+ game_ai 소비처 2곳 교차검증(`path_finder::enemy_knows_my_position` m03.ll:144778 · "
     u"`v57_summon_command_score` _v57.ll:1~43)"),
    (18, u"어떤 필드가 판정에 쓰이는지는 이 범위에서 확정 불가",
     u"G7 자동적발 → history 에 답 있음: goal_data 는 `serpen.epic_enemy_tick`(+0xc0) 오직 1개"
     u"(m15.ll:53994, Serpen 분기 / `epic.epic_enemy_tick`+0x88 은 Morgard 분기라 이 호출부에선 죽은 경로), "
     u"plan 은 `BigPlan::goal()` 판별자 태그 하나(== BigGoal::Battle(5), m09.ll:65038~65044)"),
]
