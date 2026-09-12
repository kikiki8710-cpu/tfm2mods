# -*- coding: utf-8 -*-
u"""9차 배치A 의 `patch.json` 생성 + **kind 안정성 사전검사**.

`meaning` 본문을 고치면 `mkspec3._kind` 가 낱말을 다시 읽어 **`consts.kind` 가 뒤집힐 수 있다**
(G15 가 다시 열린다). 그래서 patch 를 쓰기 전에 **고친 문면으로 `_kind` 를 다시 돌려** 비교한다.
"""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, MIG)
import mkspec3 as M

V2 = json.load(io.open(os.path.join(MIG, "_spec", "specs20.json"), encoding="utf-8"))

# (spec, const, old조각, 덧붙일 문면, found_by)
E = [
 (1, 10, u"그 외 모든 EntityType(Champion 포함!)의 계수 = 0 → 점수 0",
  u" ★src_line=536 을 IR 로 못 박았다(9차 배치A): 리터럴 0 은 m05.ll:40074 "
  u"`%32 = phi i64 [ 0, %45 ], [ -10, %46 ], [ 80, %37 ], [ 200, %22 ], [ %44, %42 ]` 의 "
  u"phi 인입이라 그 자리에 `!dbg` 가 없다 — 인입 블록 `%45`(switch default)의 종결자 "
  u"`br label %31, !dbg !44413` 이 `!DILocation(line: 536)` 이다", "new"),

 (2, 4, u"L50 = 58자 → 내용 57 = indent 6 + `SubPlan::AttackNexus(AttackNexusSubPlan::default())`(51자) ±0",
  u"L50 = 58자 → 내용 57 = indent 6 + `SubPlan::AttackNexus(AttackNexusSubPlan::default())`(51자) ±0"
  u" ⚠IR 로는 이 줄을 **확인할 수 없다**(9차 배치A): 16 은 m12.ll:34973 "
  u"`%68 = phi i64 [ 5, %49 ], [ 2, %61 ], [ 16, %55 ]` 의 인입이고 phi 자신에 `!dbg` 가 없다. "
  u"인입 블록 `%55` 의 종결자 `br i1 %60, label %67, label %61, !dbg !62988` 은 attack_nexus.rs:47 인데 "
  u"**else 팔 `%61` 의 종결자도 같은 !62988** 이라 팔을 못 가른다(= `if` 식 자체의 위치). "
  u"⟹ IR 귀속은 판별력이 없고 L50 은 rmeta_srcmap 근거로 유지한다", "reused"),

 (5, 2, u"(같은 리터럴 1 이 start_in_range 저장 시 `and i8 %103, 1` 불리언 정규화 마스크로도 쓰임)",
  u"(같은 리터럴 1 이 start_in_range 저장 시 `and i8 %103, 1` 불리언 정규화 마스크로도 쓰임) "
  u"★src_line=146 을 IR 로 못 박았다(9차 배치A): 1 은 m13.ll:29154 "
  u"`%74 = phi i8 [ %72, %71 ], [ 1, %33 ], …` 의 인입이라 그 자리에 `!dbg` 가 없고, "
  u"인입 블록 `%33` 의 종결자 `br i1 %18, label %73, label %34, !dbg !35929` 이 "
  u"`!DILocation(line: 146)` 이다", "new"),

 (5, 8, u"(end_reason 의 7 과 값만 같고 의미는 무관)",
  u"(end_reason 의 7 과 값만 같고 의미는 무관) "
  u"★src_line=152 를 IR 로 못 박았다(9차 배치A): 7 은 m13.ll:29154 phi 의 인입 `[ 7, %66 ]` 이고 "
  u"블록 `%66` 의 종결자 `br i1 %65, label %73, label %67, !dbg !35963` 이 `!DILocation(line: 152)` 이다. "
  u"본문에 남은 리터럴 7 (`%23 = icmp eq i8 %2, 7`, m13.ll:29017 → L139)은 **end_reason 쪽**이라 무관", "new"),

 (9, 0, u"IR 에선 shl i64 %30,1 로 접혀 리터럴 2 는 team bounds-check(icmp ult %7,2) 쪽에만 남아 있다",
  u"IR 에선 shl i64 %30,1 로 접혀 리터럴 2 는 team bounds-check(icmp ult %7,2) 쪽에만 남아 있다 "
  u"— 접힌 그 자리(m15.ll:35471 `%31 = shl i64 %30, 1, !dbg !46299`)의 `!46299` 가 "
  u"`!DILocation(line: 1209)` 이라 **src_line=1209 는 IR 로도 확정**된다(9차 배치A)", "reused"),

 (9, 4, u"같은 값이 1286행 cross_sq(=sin²) 판정에도 쓰임",
  u"같은 값이 1286행 cross_sq(=sin²) 판정에도 쓰임 — 두 곳이 CSE 로 합쳐져 "
  u"m15.ll:35754 `%184 = mul i128 %175, 9` 하나만 남았고 그 `!dbg !46436` 은 "
  u"`!DILocation(line: 0)`(병합 위치)다. 소비자 `%189 = icmp sgt i128 %188, %184, !dbg !46437` 이 "
  u"`line: 1283` 이라 **src_line=1283 확정**(9차 배치A)", "new"),

 (9, 5, u"IR 에선 shl i128 %176,2 로 접혔고, 리터럴 4 는 아군 루프 상한(icmp ult %89,4)으로만 본문에 남아 있다",
  u"IR 에선 shl i128 %176,2 로 접혔고, 리터럴 4 는 아군 루프 상한(icmp ult %89,4)으로만 본문에 남아 있다 "
  u"— 접힌 그 자리(m15.ll:35776 `%191 = shl i128 %176, 2, !dbg !46432`)의 `!46432` 가 "
  u"`!DILocation(line: 1280)` 이라 **src_line=1280 은 IR 로도 확정**된다(9차 배치A)", "reused"),

 (16, 5, u"range 누적 초기값(하나도 못 쓰면 그대로 반환) 겸 radius_mult==0 빠른 경로 비교값",
  u"range 누적 초기값(하나도 못 쓰면 그대로 반환) 겸 radius_mult==0 빠른 경로 비교값 "
  u"★src_line=2398 을 IR 로 못 박았다(9차 배치A): m10.ll:51931 "
  u"`#dbg_value(i64 0, !56148, !DIExpression(), !56262)` 의 `!56148` 이 "
  u"`!DILocalVariable(name: \"range\", file: !1808, line: 2398)` 이다. 본문에 남은 리터럴 0 "
  u"(`icmp eq i32 %N, 0` 8개)은 전부 entity.rs:1512 radius_mult 빠른 경로(L2403/2410/2417/2424)다", "new"),
]

EV = {
 (1, 10): u"m05.ll:40074 `%32 = phi i64 [ 0, %45 ], …`(phi 인입이라 `!dbg` 없음) + 인입 블록 %45 종결자 m05.ll:40086 `br label %31, !dbg !44413` = `!DILocation(line: 536, scope: !44344)` → action_score.rs:536 = 주장 그대로. G12 가 낸 후보 [542,548] 은 각각 based 보너스 `select i1 %55, i64 5, i64 0`(L542)과 `icmp eq i64 %27, 0`(L548)로 **다른 용도의 0**",
 (2, 4): u"G12 의 유일한 강한 후보는 m12.ll:34916 `%32 = getelementptr inbounds nuw i8, ptr %30, i64 16, !dbg !62965` = **필드 오프셋**이지 상수가 아니다(7차가 없앴다는 gep 잡음이 `i64` 타입접두를 달고 화이트리스트를 통과). 진짜 16 은 m12.ll:34973 `%68 = phi i64 [ 5, %49 ], [ 2, %61 ], [ 16, %55 ]` 인입. 인입 블록 %55 종결자 `!62988` = attack_nexus.rs:47 이나 else 팔 %61 종결자도 같은 !62988 이라 팔 구분 불가 ⇒ 기각 근거로 못 쓴다(귀속은 구제 전용)",
 (5, 2): u"m13.ll:29154 `%74 = phi i8 [ %72, %71 ], [ 1, %33 ], [ 2, %36 ], …`; 블록 %33 종결자 m13.ll:29038 `br i1 %18, label %73, label %34, !dbg !35929` = `!DILocation(line: 146)` → 주장 그대로. G12 후보 [157] 은 `and i8 %103, 1`(불리언 마스크)과 `add i64 %123, 1`(루프 증가)",
 (5, 8): u"m13.ll:29154 phi 인입 `[ 7, %66 ]`; 블록 %66 종결자 m13.ll:29135 `br i1 %65, label %73, label %67, !dbg !35963` = `!DILocation(line: 152)` → 주장 그대로. G12 후보 [139] 는 `%23 = icmp eq i8 %2, 7`(end_reason 쪽, consts[1])",
 (9, 0): u"m15.ll:35471 `%31 = shl i64 %30, 1, !dbg !46299`; `!46299 = !DILocation(line: 1209, scope: !46169)` → 주장 그대로. G12 후보 [1203,1280] 은 `icmp ult i64 %7, 2`(팀 bounds-check)와 `shl i128 %176, 2`(**시프트 자릿수** 2 — 실제로는 ×4)",
 (9, 4): u"m15.ll:35754 `%184 = mul i128 %175, 9, !dbg !46436`, `!46436 = !DILocation(line: 0)`(병합). 그 값의 소비자 m15.ll:35787 `%189 = icmp sgt i128 %188, %184, !dbg !46437`, `!46437 = !DILocation(line: 1283)` → 주장 그대로. G12 후보 [1286] 은 같은 %184 의 **다른 소비자** `%185`(!46438 = line 1286). 변수 정의도 일치: `!46224 = !DILocalVariable(name: \"is_rear_direction\", line: 1283)`",
 (9, 5): u"m15.ll:35776 `%191 = shl i128 %176, 2, !dbg !46432`; `!46432 = !DILocation(line: 1280, scope: !46221)`, 변수 `!46222 = !DILocalVariable(name: \"is_front\", line: 1280)` → 주장 그대로. G12 후보 [1233] 은 `icmp samesign ult i64 %89, 4`(아군 루프 상한)",
 (16, 5): u"m10.ll:51931 `#dbg_value(i64 0, !56148, !DIExpression(), !56262)`; `!56148 = !DILocalVariable(name: \"range\", scope: !56149, file: !1808, line: 2398)` → 주장 그대로. G12 후보 [2403,2410,2417,2424] 는 전부 `icmp eq i32 %N, 0` = entity.rs:1512 `radius_mult==0` 빠른 경로",
}

errors = []
bad = 0
for (i, j, old, add, fb) in E:
    sp2 = V2["specs"][i]
    c = sp2["constants"][j]
    m = c.get("meaning") or u""
    if old not in m:
        print(u"!! old 미일치 specs[%d] consts[%d]" % (i, j)); bad += 1; continue
    new = m.replace(old, add if add.startswith(old[:12]) else old + add, 1)
    # ── kind 안정성: 고친 문면으로 `_kind` 재계산 ────────────────────────────
    k0 = M._kind(sp2, c, M._EVTAIL.split(m)[0])
    c2 = dict(c); c2["meaning"] = new
    k1 = M._kind(sp2, c2, M._EVTAIL.split(new)[0])
    flag = u"OK" if k0 == k1 else u"**KIND 변동**"
    print(u"specs[%2d] consts[%-2d] kind %s -> %s  %s" % (i, j, k0, k1, flag))
    if k0 != k1:
        bad += 1
        continue
    errors.append({
        "path": "/specs[%d]/consts[%d]/meaning" % (i, j),
        "kind": u"오탐",
        "old": old,
        "new": (add if add.startswith(old[:12]) else old + add),
        "evidence": EV[(i, j)],
        "behavior_change": False,
        "found_by": fb,
    })

P = {
    "round": 9, "batch": "A",
    "errors": errors,
    "ev_up": [],
    "brief_errors": [
      u"[지시·도시에 §1] 「8차 배치C 제안 = 강한 후보가 0개인 상수는 「불일치」가 아니라 「검사 불가」로 분류」를 "
      u"고칠 거리로 제시했는데 **그 판정은 `srclinecheck.check_spec` 에 이미 구현돼 있다** "
      u"(`cands = sorted(x for x in found if x)` + `if cands and claim not in found …`). "
      u"적용해도 8건 중 **0건**이 바뀐다 — 8건 전부 강한 후보를 갖고 있었기 때문이다"
      u"(09c0=[1203,1280] · 09c5=[1233] · 02c4=[37] · 01c10=[542,548] · 05c2=[157] · 05c8=[139] · "
      u"09c4=[1286] · 16c5=[2403,2410,2417,2424]). 제안된 해법이 증상과 무관했다.",

      u"[8차 배치C 귀속의 오류 — 판정 반전 1건] `02 consts[4]` 를 「phi 인입이라 `!dbg` 가 없음」으로만 "
      u"설명했는데, 그건 **정답을 못 본 이유**이고 **게이트가 오답을 낸 이유는 따로 있다** — "
      u"유일한 강한 후보가 `getelementptr inbounds nuw i8, ptr %30, i64 16`(필드 오프셋)이었다. "
      u"7차가 「gep 오프셋 잡음을 화이트리스트로 없앴다」고 적었지만 gep 의 인덱스는 `i64` **타입 접두를 달고 있어** "
      u"그 화이트리스트를 그대로 통과한다. 원인을 phi 로만 적으면 고칠 자리를 못 찾는다.",

      u"[도시에 §1·§2 불일치] §1 은 「`MIG\\_gates\\src_line\\` 에 다른 배치들이 만든 검사기 후보가 모여 있다 — "
      u"전부 돌려 보고 왜 결과가 갈리는지 규명하라」고 하는데, 그 폴더의 두 파일(`B_`·`D_srclinecheck2.py`)은 "
      u"**7차에 이미 `MIG\\srclinecheck.py` 본체로 흡수된 선행 제안**이다(본체 docstring 의 교정표 ①②③ 이 그것). "
      u"「결과가 갈리는」 비교 대상이 아니라 **아직 흡수 안 된 잔여**를 보는 자리였고, 실제 잔여는 배치B 의 "
      u"「`invoke` 는 두 줄이라 리터럴 줄에 `!dbg` 가 없다」 하나뿐이었다(이번에 흡수 — 검사 가능 상수 145→153).",

      u"[도시에 §2 누락] 필독 정본 표에 **`MIG\\srclinecheck.py` 자신이 없다**. G12 의 본체이고 "
      u"지시문에서야 「전문을 읽어라」고 하는데, 도시에만 읽고 착수하면 본체를 안 읽고 시작하게 된다.",
    ],
}
io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
    json.dumps(P, ensure_ascii=False, indent=1))
print(u"\n항목 %d개 (거부 %d)" % (len(errors), bad))
