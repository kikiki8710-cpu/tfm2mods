# -*- coding: utf-8 -*-
u"""`_prop.json` 생성 — v2 정본의 `meaning` 을 읽어 **최소 삽입**으로 수정안을 만든다.
(정본은 읽기만 한다. 실제 반영은 메인이 `applypatch.py` 로)"""
import io, json, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
V2 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))


def get(i, j):
    return (V2["specs"][i].get("constants") or [])[j].get("meaning") or u""


P = []


def add(i, j, new, want, why, ek=u"실오류"):
    P.append({"i": i, "j": j, "old": get(i, j), "new": new,
              "want": want, "why": why, "ekind": ek})


# ── 05 v50_fold_dive_episode — end_reason/end_plan 코드값 10행 ──────────
# 근거: m13.ll:29017 `%23 = icmp eq i8 %2, 7`(end_reason 동등비교) ·
#       m13.ll:29154 `%74 = phi i8 [ 1,%33 ],[2,%36],[3,%41],[4,%46],[7,%66],[5,%51],[6,%61],[6,%56]`
#       (end_plan 코드표를 phi 로 산출). 28946~29380 전 범위에 이 값들의 순서비교(icmp ult/slt 등)는 **0건**.
EV05 = (u"m13.ll:29017 `icmp eq i8 %2, 7`(end_reason 동등비교) · "
        u"m13.ll:29154 `%74 = phi i8 [ 1,%33 ],[ 2,%36 ],[ 3,%41 ],[ 4,%46 ],[ 7,%66 ],"
        u"[ 5,%51 ],[ 6,%61 ],[ 6,%56 ]`(end_plan 코드표). "
        u"IR 범위 28946~29380 에 이 값들의 순서비교는 **0건** ⟹ 임계일 수 없다")
for j in range(1, 11):
    m = get(5, j)
    head = u"end_reason 코드 " if j == 1 else u"end_plan 코드 "
    assert m.startswith(head), (j, m[:24])
    add(5, j, m.replace(head, head[:-1] + u"(판별자) ", 1), u"태그", EV05)

# ── 06 c0 — 부정문 맹점(「인덱스가 아니다」 때문에 kind=인덱스) ────────────
# ⚠머리쪽도 같이 고친다 — `applypatch.already()` 가 `new[:80]` 부분문자열로 「이미 적용」을
#   판정해서, **긴 문면의 꼬리만 고치면 조용히 버려진다**(이 패치 초판이 실제로 그렇게 삼켜졌다).
add(6, 0,
    get(6, 0).replace(u"AI 버전 게이트 —", u"AI 버전 비교 임계(게이트) —", 1)
             .replace(u"이지 인덱스가 아니다)", u"이지 팀 배열 첨자가 아니다)"),
    u"임계",
    u"m13.ll:45232 `%14 = icmp ult i64 %0, 2` — 첫 인자(version)의 **순서비교**다. "
    u"`meaning` 이 「인덱스가 **아니다**」라고 적었는데 `mkspec3` 의 낱말 매칭은 부정문을 못 봐 "
    u"그 「인덱스」에 걸려 kind=인덱스가 됐다")

# ── 07 c1 — 같은 부정문 맹점(「태그가 아니다」 때문에 kind=태그) ───────────
add(7, 1,
    get(7, 1).replace(u"★귀환 HP 임계.", u"★귀환 HP 임계(순서비교 문턱).", 1)
             .replace(u"이지 태그가 아니다)", u"이지 코드값이 아니다)"),
    u"임계",
    u"m02.ll:49087 `%94 = icmp ult i64 %33, 51` — **순서비교** 문턱이다. "
    u"`meaning` 이 「태그가 **아니다**」라 적었는데 부정문을 못 봐 kind=태그로 뒤집혔다")

# ── 10 c1 — 동등비교 기대값인데 잔여 버킷(임계) 으로 떨어짐 ────────────────
add(10, 1,
    get(10, 1).replace(u"마스크 결과 기대값 0x000300", u"마스크 결과의 **동등비교 기대 태그값** 0x000300", 1),
    u"태그",
    u"m10.ll:49633 `%9 = icmp eq i24 %8, 768` — 동등비교의 기대값이다(순서비교 0건). "
    u"`mkspec3` 에 이 종류를 가리키는 낱말이 없어 잔여 버킷 `임계` 로 떨어져 있었다")

# ── 11 — LineType/TutorialType 판별자 10행 ───────────────────────────────
EV11 = (u"m13.ll:12319~12327 `switch i8 %40, label %41 [ i8 0,… i8 6,… ]` · "
        u"m13.ll:12353~12356 `[ i8 0 · i8 2 · i8 7 · i8 8 ]` — **switch 케이스 라벨**이다. "
        u"IR 범위 12238~12479 의 순서비교는 `icmp ult i64 %1, 2`(version) · "
        u"`icmp ult i8 %53, -6` · `icmp ult i8 %66, -7` 셋뿐이고 이 값들과 무관 ⟹ 임계일 수 없다")
add(11, 4, get(11, 4).replace(u"LineType::Mid —", u"LineType::Mid 판별자 1 —", 1), u"태그",
    u"m13.ll:12290 `store i8 1`(SinglePlanLine 의 line 필드) — 열거형 판별자다. " + EV11)
for j, nm, v in ((6, u"None", 0), (7, u"First", 1), (8, u"TopSolo", 2), (9, u"Bottom", 3),
                 (10, u"MidSolo", 4), (11, u"MidBottom", 5), (12, u"JungleOnly", 6),
                 (13, u"Line", 7), (14, u"Total", 8)):
    m = get(11, j)
    src = u"TutorialType::%s —" % nm
    assert m.startswith(src), (j, m[:30])
    add(11, j, m.replace(src, u"TutorialType::%s 판별자 %d —" % (nm, v), 1), u"태그", EV11,
        ek=(u"보강" if j == 8 else u"실오류"))

# ── 13·14 — 除외 ──
# 7차 배치C 가 부시 ID 26행을 `임계 → 인덱스` 로 제안했고 나도 한번 넣었다가 **내 게이트로 반증하고 물렸다.**
#   적용 시뮬레이션: 강 16 → **28** (NEG 는 4→2 로 줄지만 순효과가 음수).
#   이유 = `select` 로 산출될 뿐 이 함수 IR 은 그 값으로 **첫자를 때리지 않는다**(GEP/MINMAX/ARITH 0건).
#   ⟹ `임계`(지지 0) 에서 `인덱스`(지지 0) 로 옮기는 것은 진전이 아니라 **라벨 교체**다.
#   이 26행의 참된 소속은 `산출값` 이고 그건 현행 4종 어휘에 칸이 없다 ⇒ **표기 불가**로 낸다.

json.dump(P, io.open(r"C:\tfm2mods\MIG\_verify8\B\_prop.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print(u"제안 %d행" % len(P))
for p in P:
    print(u"  [%02d]c%-2d %s" % (p["i"], p["j"], p["new"][:78]))
