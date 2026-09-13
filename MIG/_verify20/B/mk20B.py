# -*- coding: utf-8 -*-
"""20차 배치B patch.json 생성 (specs[105]~[107]). 실행: python -X utf8 mk20B.py"""
import sys; sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=20, batch="B")

# ───────────────────────── 105 passive_plan ─────────────────────────
p.fix("/specs[105]/sig/params[1]/note",
      old=u"③readonly 가 빠진 이유가 바로 그 원자 쓰기(내부 가변성)",
      new=u"③IR 인자 속성은 `ptr noundef nonnull align 8 %1`(readonly·noalias 없음) — 그 원자 쓰기(내부 가변성) 때문",
      evidence=u"m13.ll:6495 define `…, ptr noundef nonnull align 8 %1, …` (readonly 없음 = 명세 주장 그대로). G16 P4 가 「readonly 가 빠진」의 '빠진' 을 부정어(빠짐/빠져)로 못 잡아 오탐",
      behavior_change=False, found_by="new", kind=u"오탐", force=True)   # v3 키는 role, v2 키는 note(+stale role) → applypatch 는 v2 note 를 고친다
p.fix("/specs[105]/mem[50]/name",
      old=u"region_pos[27] (x,y)", new=u"region_centers[27] (x,y)",
      evidence=u"tcxdict MapDef 0x6ba0 = `region_centers[0].0 u64` (MapDef 필드 13개 중 region_pos 없음) · m13.ll:81764 `!13207 = !DIDerivedType(name: \"region_centers\", … offset: 220416)` (=0x6ba0)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[105]/logic",
      old=u"map.region_pos[map.line_region(line, team, lead[team])]",
      new=u"map.region_centers[map.line_region(line, team, lead[team])]",
      evidence=u"위 mem[50] 과 동일(tcxdict MapDef 0x6ba0 = region_centers)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[105]/consts[3]/meaning",
      old=u"gen_range(0..1000) < p 이면 PassiveLine(fallback_line) 채택(발원 6). :2096 점수 = -minion_count*1000 - from_mid 에도 사용",
      new=u"‰ 분모 계수: gen_range(0..1000) < p 이면 PassiveLine(fallback_line) 채택(발원 6). :2096 점수 가중 계수 = -minion_count*1000 - from_mid 에도 사용",
      evidence=u"kind 미상 → 계수. kindchk 강한 관측 0(gen_range 는 CALLARG, 점수 곱은 접힘) 이라 낱말이 그대로 kind 가 된다(mkspec3 규칙①)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[105]/consts[7]/meaning",
      old=u"epic_giveup_tick 이 tps*10 이내면 '최근 포기' (에픽옵스 커버 라인 진입 조건)",
      new=u"tps*10 계수(`%709 = mul i64 %708, 10` m13.ll:7975, 루트 :2043) — epic_giveup_tick 이 tps*10 이내면 '최근 포기'(에픽옵스 커버 라인 진입 조건). 같은 리터럴 10 = MainObjective::PressTower 태그(:1928 `icmp eq i8 %213, 10` m13.ll:6947)·BigPlan::LineGanker 태그(:1949 `store i64 10`)",
      evidence=u"m13.ll:7975 `mul i64 %708, 10`(ARITH) / m13.ll:6947 `icmp eq i8 %213, 10`(PressTower 판정). 이 행의 src_line 2043 은 tps*10 자리이므로 kind 는 태그가 아니라 계수",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[105]/consts[12]/meaning",
      old=u"v2_obj_part == Some(0x50 | phase) — Serpen 복원 코드(0x50 + ObjectPhase).",
      new=u"v2_obj_part == Some(0x50 | phase) — Serpen 복원 코드 태그(0x50 + ObjectPhase).",
      evidence=u"kind 미상 → 태그(낱말). m13.ll:7713 `or disjoint i8 %phase, 80` 뒤 `icmp eq` 비교 = 코드값",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[105]/consts[13]/meaning",
      old=u"v2_obj_part == Some(0x60 | phase) — Morgard 복원 코드",
      new=u"v2_obj_part == Some(0x60 | phase) — Morgard 복원 코드 태그",
      evidence=u"kind 미상 → 태그(낱말). m13.ll:7844 `or disjoint i8 %phase, 96` 뒤 `icmp eq` 비교 = 코드값",
      behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────── 106 v25_objective_posture ─────────────────────────
p.fix("/specs[106]/consts[15]/meaning",
      old=u"230000²+1 — focus 후보 필터: distance_sq(x, camp_pos) ≤ 230000²",
      new=u"230000²+1 임계 — focus 후보 필터: distance_sq(x, camp_pos) ≤ 230000² (오라클 v20B_o1: 229000/231000 배치가 focus 후보에 들고/빠짐 확인)",
      evidence=u"kind 미상 → 임계. 본문 범위에 리터럴 없음(aux call_mut m09.ll:42198 / fold 13726 `icmp ult …, 52900000001`) 이라 낱말이 kind 가 된다 · 오라클 v20B_o1.tsv 500/500 MATCH",
      behavior_change=False, found_by="new", kind=u"보강")

ORC = u"오라클 실행 확인(20차 B v20B_o1: 독립 재구현 predict==actual 500/500 MATCH · kind None/HoldCamp/Screen/WaitGroup/SoftDisengage · 카운트 4종·focus_enemy·camp_pos·wait_pos·target 전수 대조)"
for j, what in ((0, u"target Morgard(4)/Serpen(5)↔objective 태그 0/1 대조: 불일치·None(-1) 이면 None"),
                (1, u"target Serpen(5)"), (2, u"None 경로 sret+0 = -1"), (3, u"wait_pos = camp_pos(Stump,team==0) (Morgard)"),
                (4, u"wait_pos = camp_pos(Rhino,team==0) (Serpen)"), (5, u"hp*100/max_hp"),
                (8, u"hp% 39/40 경계 케이스 포함"), (9, u"내 챔프 거리 139000/141000 경계"), (10, u"165000/171000 경계"),
                (11, u"캠프 거리 179000/181000 경계"), (12, u"189000/191000 경계"), (13, u"hp 49/51 경계"),
                (14, u"closure#8: elapsed*move_speed >= distance(last_pos,camp)-180000, move_speed 1/500/5000 으로 camp_possible 0~5 갈림"),
                (15, u"229000/231000 경계"), (16, u"min_by_key 키 dist_sq(me)+dist_sq(camp)>>2 로 focus id 일치"),
                (17, u"내 챔프 149000/151000 경계"), (18, u"내 hp 34/35 경계(mode 2)"), (19, u"visible<3 · side_ally>3"),
                (20, u"≥2 비교 전부"), (21, u"179000/181000 경계 → WaitGroup")):
    p.ev("/specs[106]/consts[%d]" % j, evidence=ORC + u" — " + what, to=2, frm=(3 if j == 0 else 4), found_by="new")
for j in range(2, 16):
    p.ev("/specs[106]/knobs[%d]" % j, evidence=ORC, to=2, frm=4, found_by="new")
for j in range(28, 40):
    p.ev("/specs[106]/mem[%d]" % j, evidence=ORC + u" — sret 필드 값 대조", to=3, frm=(3 if j == 38 else 4), found_by="new")
for j in (0, 1, 4, 5, 11, 14, 19, 20, 21, 22, 23, 24, 25, 26, 27):
    p.ev("/specs[106]/mem[%d]" % j, evidence=ORC + u" — 그 필드를 raw write 로 바꾼 케이스가 predict 대로 갈림", to=3, frm=4, found_by="new")

# ───────────────────────── 107 handle_none_or_gank_objective ─────────────────────────
p.fix("/specs[107]/consts[51]/src_line", old=958, new=944,
      evidence=u"m09.ll:12284/12599/12676 `icmp ult i64 %n, 6` + `llvm.assume` — !dbg 사슬: assume_count_le_upper_bound(iterator) → count<…closure$5/6/7> → team_plan.rs:1479/1516/1525 → **objective_handlers.rs:944**(check_comeback_pick_opportunity 인라인). :954~958 already_on_line 에는 6 이 없다(루트 958 인 리터럴 6 명령 0건)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[107]/consts[51]/meaning",
      old=u"count < 6 assume(5칸 상한 힌트, 판정 아님). BigGoal 아님",
      new=u"assume 힌트(판정 아님): check_comeback_pick_opportunity 인라인의 (0..5).filter().count() 상한 `icmp ult %n, 6` 3곳(team_plan.rs:1479·1516·1525 → 루트 :944). 같은 리터럴 6 = BigPlan 태그 니치 구멍 `icmp ne %tag, 6` assume(:986·1024·1071) · spawn_epic 접힘 `ult 6`(:912 안). BigGoal 아님",
      evidence=u"위 src_line 과 동일 · tcxdict --enum BigPlan: 태그 6 은 untagged(DeathMatchBattle) 자리라 부재 → `!range`+assume",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[107]/logic",
      old=u"// :912  check_opportunistic_tower_push(version, player, data, &kill_line) 인라인(team_plan.rs:1273~1345) → Option<LineType>",
      new=u"// :912  check_opportunistic_tower_push(version, player, data, kill_line) 인라인(team_plan.rs:1273~1345) → bool (tcx sig `fn(usize, &PlayerState, &OperationData, LineType) -> bool`; 아래 '→ None' = false, 참이면 kill_line 그대로 채택)",
      evidence=u"_tcx/game_ai.json check_opportunistic_tower_push sig = `fn(usize,&PlayerState,&OperationData,LineType) -> bool` (callees[11] 과 동일). IR: 인라인 결과가 LineType phi 가 아니라 i1 분기이고 PressTower 라인 store(m09.ll:10767 `store i8 %422`) 의 %422 = check_recent_kill_lane 결과(kill_line)",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[107]/logic",
      old=u"//   else → Some(line)\n    if let Some(line) = check_opportunistic_tower_push(..) {",
      new=u"//   else → true\n    if check_opportunistic_tower_push(..) { let line = kill_line;",
      evidence=u"동일(tcx sig → bool)", behavior_change=False, found_by="new", kind=u"실오류")
p.fix("/specs[107]/sig/returns",
      old=u"() — ret void (L1131). %53 phi i1 은 죽은 값(사용처 없음). 결과는 전부 self/plan 부작용",
      new=u"bool(tcx sig) = 「목표/플랜을 정했는가」 — 목표 설정 후 return 하는 전 경로·L1000 ActiveRecall·L1013/L1060 giveup = true, L1006/L1053 should_delay return 과 L1123~1131 꼬리 = false, L1119 는 v3_epicops_buff_window 의 bool 을 그대로(%53 phi 인입값 m09.ll:9185). 유일 호출자 TeamPlan::update(m09.ll:30757)가 값을 버려 IR 은 `define … void` 로 축약(L1131 ret void, %53 죽은 값). 결과는 전부 self/plan 부작용",
      evidence=u"m09.ll:9185 `%53 = phi i1 [true,%40],[true,%2262],…,[false,%1871],…,[%2135,%2134],[false,%2136],[false,%2132],[false,%2128],…` — 인입 블록 루트: true=L833·840·852·884·894·903·916·935·965·1000·1013·1046·1060·1094·1108, false=L1006·1053·1123·1124·1126, %2135=L1120 v3_epicops_buff_window 반환. tcx sig `… -> bool`",
      behavior_change=False, found_by="new", kind=u"보강", force=True)   # v3 sig.ret ↔ v2 signature.returns
p.fix("/specs[107]/sig/params[1]/role",
      old=u"그 외는 콜리에 그대로 전달",
      new=u"나머지 사용은 콜리 인자로 넘김(call 밖 사용 = 위 4곳뿐)",
      evidence=u"G16 P5 가 절 「그 외는 콜리에 그대로 전달」을 전면 미사용 주장으로 읽음(FULL_NEG '그대로 전달'). 실측 call 밖 사용 4회 = 역할문의 분기 4곳(L876 m09.ll:9814 `icmp ugt i64 %1, 1` 등) 과 일치 → 오탐",
      behavior_change=False, found_by="new", kind=u"오탐")
p.fix("/specs[107]/consts[24]/meaning",
      old=u"KillLog stride 48B (rfind 포인터 -48)",
      new=u"KillLog 원소 길이(stride) 48B — rfind 포인터 -48, 판정값 아님",
      evidence=u"kind 미상 → 길이(낱말). m09.ll:10061 `mul nuw nsw i64 %391, 48` = len*48 바이트",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[107]/consts[26]/meaning",
      old=u"1300: ms.from_mid < judgement_penalty*1000 이면 None",
      new=u"1300: ms.from_mid < judgement_penalty*1000 이면 None — 페널티→from_mid 환산 계수",
      evidence=u"kind 미상 → 계수. m09.ll:10525 `%592 = mul nsw i64 %591, 1000`(ARITH, 루트 :912)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[107]/consts[39]/meaning",
      old=u"1505: can_near_enemies_range(line_center, range=300000) — MIA 적 탐색 반경",
      new=u"1505: can_near_enemies_range(line_center, range=300000) — MIA 적 탐색 반경 임계(콜리 인자)",
      evidence=u"kind 미상 → 임계(낱말). m09.ll:12392 invoke can_near_enemies_range(…, i64 300000, …) CALLARG 뿐이라 낱말이 kind 가 된다",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[107]/open[3]",
      old=u"poison 3개(2번째=rnd?, 5번째=goal_data?, 7번째=debug?)는 콜리가 그 인자를 안 읽는다는 뜻. 실제 파라미터 이름은 콜리 define(m09.ll 53000대) 미열람",
      new=u"poison 3개 = 2번째 rnd(&mut StdRng) · 5번째 team_plan(&TeamPlan, goal_data 아님) · 7번째 debug(&mut DebugFrameData) — tcx sig `fn(usize, &mut StdRng, &PlayerState, &OperationData, &TeamPlan, Option<LineType>, &mut DebugFrameData) -> Option<LineType>`(callees[12]). 콜리가 그 인자를 안 읽는다는 뜻(콜리 본문은 미열람)",
      evidence=u"_tcx/game_ai.json check_press_tower_opportunity sig(=callees[12] 행). 5번째 인자는 &TeamPlan 이지 GoalData 가 아니다",
      behavior_change=False, found_by="new", kind=u"실오류")

# ───────────────────────── 도시에 오류 ─────────────────────────
p.brief_error(u"§4 「이 배치 몫 미해소 = 0건」 이 틀렸다 — `specgate.py --only 105/106/107` 실행 결과 105 G16=1 · 106 G7=2 · 107 G12=1·G16=1·G20(R1)=1 = 6건이 열려 있었다(mkdossier 의 게이트 집계가 G7/G16/G20 을 빠뜨리거나 다른 판을 읽는다)")
p.brief_error(u"§0 신선도 명령은 정상. 단 §1 표의 `ev≥4(미실행)` 수(105=88·106=75·107=141)가 무엇의 합인지(mem+consts+knobs? open 포함?) 정의가 없어 실행 대상 계획에 못 쓴다")
p.brief_error(u"§5 스키마 예시의 `ev_up.from/to` 만 있고 `guard`(applypatch 가 받는 선택 필드) 언급이 없다 — 삽입과 섞이면 도장이 밀리는데 도시에는 안내하지 않는다")
p.brief_error(u"§4-b 「callees ev4 행 = 판정 보류」라 했는데 107 callees 에는 보류가 아니라 **명백히 다른 함수**(Staff::serialize::push · nightmare::tick_per_second · ClientData::team · MinionWaveSnapshot::find · EntityInfo::gold · Strategy::tutorial · CCState::tick · PatchSetting::is_empty) 가 실려 있다 — leaf 이름 후보 열거가 '판정 보류' 로 읽히면 재구현자가 그 시그니처를 집는다. 자동 생성기(calls→tcx 조인)가 std 이름(push/find/is_empty/tick)과 필드 이름(gold/team/tutorial/tick_per_second)을 걸러야 한다")

out = p.save()
