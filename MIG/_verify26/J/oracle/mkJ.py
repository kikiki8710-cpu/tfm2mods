# -*- coding: utf-8 -*-
"""26차 배치J patch.json 생성 — mkpatch 참조구현 사용. 실행: python -X utf8 mkJ.py"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=26, batch="J")
ORC = u"오라클 oJ.exe(_verify26/J/oracle/oJ.rs · 케이스당 프로세스 1개 · 로그 oJ_cases.log)"

# ───────────────────────── 229 count_nearby_enemies ─────────────────────────
# G16 P4: role 이 `nonnull` 을 말하는데 define 엔 없다 → 속성어 제거(본문 assume 를 정확히 인용)
p.fix("/specs[229]/sig/params[3]/note", force=True,
      old=u"null 검사는 llvm.assume(nonnull) 뿐",
      new=u"IR `readonly captures(address_is_null)`(define 에 nonnull 속성 없음) · 본문 null 검사 = `%11 = icmp ne ptr %3, null` + `llvm.assume`(m14.ll:37857~37858)뿐 · +0x0/+0x8(game 팻포인터)·+0x1e0(player_champion) 로드",
      evidence=u"m14.ll:37842 define `ptr readonly captures(address_is_null) %3` · m14.ll:37857 `%11 = icmp ne ptr %3, null` · 37858 `tail call void @llvm.assume(i1 %11)` — nonnull 은 define 속성이 아니라 본문 assume 였다(G16 P4 문면 정정)",
      behavior_change=False, found_by="reused", kind=u"보강")
# G16 P3 / G5: params 5행 vs tcx 3 — internal fastcc ArgumentPromotion 이라 IR/exe ABI 가 5 인 것이 정상 → 오탐
p.fix("/specs[229]/sig/params[0]/note", force=True,
      old=u"DI: x2/other = %0 (entity.rs:2157 distance_sq 인라인)",
      new=u"DI: x2/other = %0 (entity.rs:2157 distance_sq 인라인) · ★ABI 5 = `define internal fastcc … (i64 %0, i64 %1, ptr %2, ptr %3, ptr %4)`(m14.ll:37842) — ArgumentPromotion 으로 &Entity→(x,y)·&OperationData→(cache,blackboard) 승격 · exe 도 5(argscan 0xe8f850: 레지스터 4 + 스택 1 `[rsp+0xa0]`) · tcx 3 과 다른 것이 정상(G5/G16-P3 오탐)",
      evidence=u"m14.ll:37842 `define internal fastcc noundef range(i16 0, 32) i16 @…count_nearby_enemies(i64 %0, i64 %1, ptr … %2, ptr readonly captures(address_is_null) %3, ptr captures(address, read_provenance) %4)` · argscan.py 0xe8f850 → 「스택 인자 1개 + 레지스터 4 = exe 인자 5」",
      behavior_change=False, found_by="reused", kind=u"오탐")

# ───────────────────────── 230/231 LineSafe/LineWait::score ─────────────────────────
DEAD = (u"★사장(도달 불가): evaluate_action(m11.ll:53179~54237) 은 anchor 가 Lane(tag 1) 일 때 SmallActionPlay 논리 idx 2·3·4·7·8·9·10(Around 계열)을 전부 %30 평가 경로로 보내고 그 경로의 유일한 출구 %433 이 Some(tag 1) 이다(%437 의 None 인입은 %8·%20 뿐) — 따라서 L{l1} 의 Around{{target_id}} arm(L{l1}~L{l2}: 같은 팀·dist²>39999999999·Minion/Tower 50·Nexus 100)은 이 함수에서 절대 실행되지 않는다. 오라클 SS4/SS5/SS6·SW4/SW5/SW6: Around(n0·t0·bad) 전부 evaluate_action=Some(0) → score 0(구경로였다면 t0 는 +50). 구경로는 idx 0·1·5·6·11·12·13·14·15·16 에서만 돌고 가산은 Attack/Skill/Skill2 의 calculate_action_score 또는 -99999, 그 외 0")
for i, (l1, l2, mfile) in ((230, (142, 150, u"m02.ll")), (231, (197, 205, u"m15.ll"))):
    d = DEAD.format(l1=l1, l2=l2)
    # one_line
    p.fix(u"/specs[%d]/one_line" % i,
          old=u"Around 아군구조물 원거리=50/100, 대상 소실=-99999)",
          new=u"대상 소실=-99999 · Around 계열은 evaluate_action 이 항상 Some 이라 구경로의 50/100 구조물 가산 arm 은 사장)",
          evidence=u"m11.ll:53200~53230 evaluate_action switch: idx 2·3·4·7·8·9·10 → %30, 그 외 → %437(None) · m11.ll:54231 `437: preds = %433, %20×10, %8` · 54233 `%439 = phi i64 [0,%8],[1,%433],[0,%20]…` — %30 경로의 출구는 %433(Some) 뿐 · " + ORC + u" SS4/SS5/SS6·SW4/SW5/SW6 eval=Some(0)",
          behavior_change=False, found_by="new", kind=u"실오류")
    # logic — Around arm 머리줄에 사장 표식
    p.fix(u"/specs[%d]/logic" % i,
          old=u"    // SmallAction::Around{target_id} ← SmallActionPlay idx 2 Around / 3 AroundHide / 10 LaneMinionPosition (모두 payload+0x8 = target)  (L%d)" % l1,
          new=u"    // SmallAction::Around{target_id} ← SmallActionPlay idx 2 Around / 3 AroundHide / 10 LaneMinionPosition (모두 payload+0x8 = target)  (L%d)\n    // %s" % (l1, d),
          evidence=u"m11.ll:53200~53230 switch · 54231~54236 반환 phi · " + ORC + u" S?4/S?5/S?6 eval=Some(0), game score=0, mine(구경로 재현) t0=50",
          behavior_change=False, found_by="new", kind=u"보강")
    # knobs 0~2 effect — 사장
    fname = u"line_safe.rs" if i == 230 else u"line_wait.rs"
    p.fix(u"/specs[%d]/knobs[0]/effect" % i,
          old=u"내리면 더 가까운 아군 구조물 주변 대기도 50/100 가산을 받아 구조물 근처 배회 선호가 강해짐. 올리면 멀리 떨어진 구조물로 갈 때만 가산",
          new=u"★효과 없음(사장 arm): Around 계열 후보는 evaluate_action 이 항상 Some 을 돌려줘 이 비교(L%d)에 도달하지 않는다 — 값을 바꿔도 점수가 안 변한다. 도달시키려면 evaluate_action 쪽을 바꿔야 한다" % (l1 + 5),
          evidence=u"m11.ll:53200~53230·54231~54236 · " + ORC + u" 사장 확인(S?4~S?6)",
          behavior_change=False, found_by="new", kind=u"실오류")
    p.fix(u"/specs[%d]/knobs[1]/effect" % i,
          old=u"올리면 LineSafe 중 원거리 아군 미니언/타워 쪽 Around 후보가 더 자주 선택됨 (헬퍼: is_any_type_minion — Tower 도 같은 50)",
          new=u"★효과 없음(사장 arm): 이 50 은 %s:%d phi 상수(%s `[ 50, %%123 ]` ×2 — Minion(1)/Tower(2) 케이스)이지만 Around 계열은 evaluate_action 이 항상 Some 이라 구경로에 도달하지 않는다 — 값을 바꿔도 점수가 안 변한다" % (mfile, 48591 if i == 230 else 22681, mfile),
          evidence=u"%s:%d `%%87 = phi i64 [ -99999, %%66 ], … [ 50, %%123 ], … [ 100, %%126 ] …`(!dbg 없는 phi — G19 창 밖) · m11.ll:53200~53230 사장 · " % (mfile, 48591 if i == 230 else 22681) + ORC,
          behavior_change=False, found_by="new", kind=u"실오류")
    p.fix(u"/specs[%d]/knobs[2]/effect" % i,
          old=u"올리면 넥서스로 후퇴 대기 선호↑ (Minion/Tower 50 의 2배)",
          new=u"★효과 없음(사장 arm): 이 100 은 %s:%d phi 상수(`[ 100, %%126 ]` — Nexus(3) 케이스)이지만 Around 계열은 evaluate_action 이 항상 Some 이라 구경로에 도달하지 않는다 — 값을 바꿔도 점수가 안 변한다" % (mfile, 48591 if i == 230 else 22681),
          evidence=u"%s:%d phi · m11.ll:53200~53230 사장 · " % (mfile, 48591 if i == 230 else 22681) + ORC,
          behavior_change=False, found_by="new", kind=u"실오류")
    # G19 오탐: 50/100 은 phi 상수 (consts[12]/[13] meaning 에 근거 부착)
    p.fix(u"/specs[%d]/consts[12]/meaning" % i,
          old=u"Around 대상이 아군 Minion/Tower 이고 dist≥200000 일 때 가산",
          new=u"Around 대상이 아군 Minion/Tower 이고 dist≥200000 일 때 가산 — IR 위치 = %s:%d `%%87 = phi i64 … [ 50, %%123 ] … [ 50, %%123 ]`(switch L%d 의 case 1·2 인입 · !dbg 없는 phi 라 줄창 스캔에 안 잡힘) · ★사장 arm(evaluate_action 항상 Some)" % (mfile, 48591 if i == 230 else 22681, l1 + 6),
          evidence=u"%s:%d phi 원문 · %s:%d~%d `switch i64 %%125 … i64 1, label %%86 / i64 2, label %%86 / i64 3, label %%126`" % (mfile, 48591 if i == 230 else 22681, mfile, (48665 if i == 230 else 22755), (48669 if i == 230 else 22759)),
          behavior_change=False, found_by="reused", kind=u"오탐")
    p.fix(u"/specs[%d]/consts[13]/meaning" % i,
          old=u"Around 대상이 아군 Nexus 이고 dist≥200000 일 때 가산 (그 외 EntityType → 0)",
          new=u"Around 대상이 아군 Nexus 이고 dist≥200000 일 때 가산 (그 외 EntityType → 0) — IR 위치 = %s:%d `%%87 = phi i64 … [ 100, %%126 ]`(case 3 → %%126 → L%d br) · ★사장 arm(evaluate_action 항상 Some)" % (mfile, 48591 if i == 230 else 22681, l2),
          evidence=u"%s:%d phi 원문 · %s:%d `126: … br label %%86 ;L%d`" % (mfile, 48591 if i == 230 else 22681, mfile, (48671 if i == 230 else 22761), l2),
          behavior_change=False, found_by="reused", kind=u"오탐")
    # consts[14] level>2 크래시 경로 — 실행 순서 보강
    p.fix(u"/specs[%d]/consts[14]/meaning" % i,
          old=u"Skill2 후보가 level≤2 에서 생성되면 크래시 경로",
          new=u"Skill2 후보가 level≤2 에서 생성되면 크래시 경로(오라클 S?3: level 1 Skill2 는 L%d interaction_score 안 action_score.rs:886 unwrap 이 먼저 패닉 — 이 줄의 unwrap 은 그 뒤 순서라 실제 첫 패닉 지점이 아니다)" % (109 if i == 230 else 164),
          evidence=ORC + u" SS3/SW3: EXIT=101 `panicked at game-ai\\src\\action_score.rs:886:131: called Option::unwrap() on a None value`",
          behavior_change=False, found_by="new", kind=u"보강")

# 231 문면 오류: mem[4]/[5]/[24] 의 줄번호가 LineSafe 것(126/134/142) 그대로
for k, old, new in ((4, u"L173/126/134/142 · dyn AbstractGame 팻포인터 데이터", u"L173/181/189/197 · dyn AbstractGame 팻포인터 데이터"),
                    (5, u"L173/126/134/142 · vtable+0x1f0(=496)", u"L173/181/189/197 · vtable+0x1f0(=496)"),
                    (24, u"간접 호출 4곳(L173/126/134/142)", u"간접 호출 4곳(L173/181/189/197)")):
    p.fix(u"/specs[231]/mem[%d]/note" % k, old=old, new=new,
          evidence=u"m15.ll:22646 `%64 = call ptr %63(…) ;L173` · 22661 `%74 … ;L181` · 22676 `%84 … ;L189` · 22631 `%54 … ;L197`(_next/reach/eb5840.ll 루트줄) — line_safe 의 126/134/142 는 +55 이동 전 값",
          behavior_change=False, found_by="new", kind=u"실오류")

# ───────────────────────── 232 v30 ─────────────────────────
p.fix("/specs[232]/notes[0]",
      old=u"L161 tower_attack_disable_tick 극성/의미 — v22 명세와 동일 미확정",
      new=u"L161 극성 확정: `%36 = icmp ult i64 %31(tick), %35(setting+0x13f8 tower_attack_disable_tick)`(m07.ll:52795) → tick < tower_attack_disable_tick 이어야 진행, 아니면 false. 오라클 V6(tad=0 → false) / V1(tad=9999999 → true)",
      evidence=u"m07.ll:52790~52796 `%31 = tail call i64 %16(ptr %12)` … `%34 = gep %33, i64 5112` `%36 = icmp ult i64 %31, %35` `br i1 %36, label %37, label %135` · " + ORC + u" V6/V1",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[232]/consts[17]/meaning",
      old=u"aux: Effect::is_in_range_ex 의 마지막 인자(사거리 여유 가산 15000 — 추정: v47 의 Effect::range 여유와 같은 값)",
      new=u"aux 클로저(any::check · m06.ll:34098 / m11.ll:29909): `is_in_range_ex(tower.attack_effect, tower, champ, tower.x, tower.y, stance_x, stance_y, 15000)` 의 마지막 인자 = 사거리 여유 임계 15000(리터럴 확정). 오라클 V1: 타워 range 0·dx 20000 에서 true / V13: dx 400000 false",
      evidence=u"m06.ll:34098 `… i64 noundef %47, i64 noundef %48, i64 noundef 15000), !dbg !49490` · m11.ll:29909 동형 · " + ORC + u" V1/V13",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[232]/logic",
      old=u"L191: (stance_x, stance_y) = v30_line_action_stance(Some(context), champ, target, effect)",
      new=u"L191: (stance_x, stance_y) = v30_line_action_stance(data.context /*IR ABI: internal fastcc ArgumentPromotion — tcx 의 &OperationData 인자가 context 포인터(nullable=Option 인코딩)로 승격, m07.ll:52985 `ptr nonnull %10`*/, champ, target, effect)",
      evidence=u"m07.ll:50193 `define internal fastcc { i64, i64 } @…v30_line_action_stance(ptr readonly captures(address_is_null) %0, ptr … dereferenceable(1728) %1, ptr … dereferenceable(1728) %2, ptr … dereferenceable(56) %3)` · m07.ll:52985 호출 1번 인자 `ptr nonnull %10`(=load data+8)",
      behavior_change=False, found_by="new", kind=u"보강")

# ───────────────────────── ev 상향 ─────────────────────────
# 233
E233 = ORC + u" F0~F10 11케이스: (nexus,twin빈,적챔프사거리) 진리표 전부 IR 예측과 일치(F2/F4/F8/F9/F10 true · 나머지 false)"
for k in range(5): p.ev(u"/specs[233]/consts[%d]" % k, evidence=E233, to=2, found_by="new")
for k in range(2): p.ev(u"/specs[233]/knobs[%d]" % k, evidence=E233, to=2, found_by="new")
for k in (0, 1, 2, 3, 4, 5, 6): p.ev(u"/specs[233]/mem[%d]" % k, evidence=E233, to=3, found_by="new")
# 232
E232 = ORC + u" V0~V16 20케이스: 라인전 경계(V14/V15)·tad 극성(V6)·팀(V2)·비챔프(V3)·한방(V4)·이펙트 None(V8/V10)·level>2(V11/V11b)·Around(V7)·대상 없음(V12)·타워 사거리+15000(V1/V13) 전부 IR 예측과 일치"
for k in (4, 5, 8, 9, 10, 14, 15, 17): p.ev(u"/specs[232]/consts[%d]" % k, evidence=E232, to=2, found_by="new")
for k in range(3): p.ev(u"/specs[232]/knobs[%d]" % k, evidence=E232, to=2, found_by="new")
for k in (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29, 30, 31, 32, 33):
    p.ev(u"/specs[232]/mem[%d]" % k, evidence=E232, to=3, found_by="new")
# 230/231
for i in (230, 231):
    E = ORC + u" S%s0~S%s7: Attack/Skill 대상 소실 -99999(game==mine) · Attack e0 = interaction_score+economy(Pull)+calculate_action_score(game==mine) · Around 는 evaluate_action Some(0) 조기반환 · LineSafe==LineWait 전 케이스 동일" % ((u"S", u"S") if i == 230 else (u"W", u"W"))
    for k in (0, 1, 6, 7): p.ev(u"/specs[%d]/consts[%d]" % (i, k), evidence=E, to=2, found_by="new")
    for k in (3, 4): p.ev(u"/specs[%d]/knobs[%d]" % (i, k), evidence=E, to=2, found_by="new")
    for k in (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 25, 26, 27): p.ev(u"/specs[%d]/mem[%d]" % (i, k), evidence=E, to=3, found_by="new")
# 228
E228 = ORC + u" H0~H4: line_exists Top/Mid/Bottom 전부 true → end_check 3회 호출(전부 false) → lines 빈 → None(-1) · &mut self(TeamPlan 1064B) 전후 diff 0"
for k in (0, 1, 2, 3, 4, 8): p.ev(u"/specs[228]/consts[%d]" % k, evidence=E228, to=2, found_by="new")
for k in (0, 1): p.ev(u"/specs[228]/mem[%d]" % k, evidence=E228, to=3, found_by="new")

# ───────────────────────── 지시문 오류 ─────────────────────────
p.brief_error(u"§4 G16-P3/G5 가 229 를 「params 5 vs tcx 3」으로 잡았지만 `define internal fastcc`(ArgumentPromotion) 함수는 IR/exe ABI 가 tcx 와 다른 것이 정상이다 — 게이트가 `internal fastcc` 를 면제하지 않아 오탐. 검사기: define 줄에 `internal fastcc` 가 있으면 params 행 수 ≠ tcx 인자 수를 통과시키고 `i` 열 매핑만 검사하라")
p.brief_error(u"§4 G19 가 230/231 knobs[1](50)·knobs[2](100)을 「관측 못 찾음」으로 잡았지만 두 값은 `!dbg` 없는 phi(m02.ll:48591 / m15.ll:22681)에 있다 — 줄창 스캔이 phi 상수를 못 본다(7차 배치A 가 G12 에서 적발한 것과 같은 결함). 검사기: knobs.where 줄에서 분기하는 블록의 후속 phi 인입 상수도 관측창에 넣어라")
p.brief_error(u"229·232 에는 `sig.tls` 키 자체가 없다(228/230/231/233 은 「없음 — …」문면이 있다). ④ 「dict 형식 통일」을 요구하면서 키가 빠진 항목은 정정 경로(`/specs[i]/sig/tls`)로 채울 수 없다 — mkspec3 가 tls 없는 함수에도 `없음` 문면을 생성해야 한다")
p.brief_error(u"지시문 ⑦ 「AttackNexus Around arm t 미사용」·「SubPlan::score 5 의 상수/region」은 배치 J 담당 6함수(228~233)에 없다 — 다른 배치 몫이 지시문에 섞여 있다")
p.brief_error(u"mkspec3.py:565 가 v3 `sig.params[].role` 을 v2 `note` 로 채우고 v2 `role`(IR 속성을 정확히 적은 칸 — 229 p[3] = 「readonly · captures(address_is_null) — …」)은 버린다. 그래서 G16 이 v2 `note` 의 「llvm.assume(nonnull)」을 define 속성 주장으로 읽어 오탐을 냈고, 정정 경로도 `/sig/params[j]/role` 이 아니라 v2 키 `note` 로 써야 통과한다(이 patch 는 그렇게 냈다). 검사기: G16 은 v2 `role` 을 보고, mkspec3 는 role·note 를 둘 다 실어라")
p.brief_error(u"§1 표는 229 vis 를 `in:game_ai` 로 적고 ★「pub 아니어도 오라클 직접 진입」이라 했지만 229 는 `internal fastcc`(심볼 없음)라 그 예외에 해당한다 — 표에 ABI/링키지 열이 없어 배치가 define 을 열어야 안다")

p.save()
