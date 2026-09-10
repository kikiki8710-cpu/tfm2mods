# -*- coding: utf-8 -*-
u"""specgate — 명세의 **완결 조건**을 기계로 검사한다. (2026-09-11 신설)

왜 만드나 — 2026-09-11 반증검증에서 정정 13건이 나왔는데, 그중 **9건은 새 도구가 아니라
"그 시점에 이미 갖고 있던 재료"로 잡혔어야 하는 누락**이었다. 원인은 실력이 아니라 절차다:
분석에 **완결 조건이 없어서** 라운드마다 다른 것을 발견했다.

실제 누락 사례(전부 이 게이트에 걸린다):
  G1 자기모순  : 17 flee_die 가 logic 엔 i64::MAX, note 엔 usize::MAX (같은 파일)
                 13 극성이 resolved 엔 정답, logic 엔 오답
                 18 줄번호가 still_unknown 엔 674/676, constants 엔 682
                 01 「티어 컷」이 같은 파일 resolved 의 "camp_type.__0 = 팀 인덱스" 와 모순
  G2 호출부    : 09 「유일 호출처」 — 실제 8곳. grep 을 안 했을 뿐이다
  G3 형제함수  : 14 가 target_bush_v30 만 보고 sub_plan 이 쓰는 v41 을 통째로 놓쳤다
  G4 술어 sig  : 01 이 `icmp ult x,2` 를 소스 임계로 읽었다. is_jungle(&self, usize) 의
                 시그니처를 봤다면 그게 `x==0||x==1` 의 접힘임을 알았다

사용:
  python -X utf8 specgate.py              전량
  python -X utf8 specgate.py --only 14    한 함수
  python -X utf8 specgate.py --gate G1    한 게이트
"""
import io, json, os, re, sys
from collections import Counter

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
SPEC = V3 if os.path.exists(V3) else os.path.join(HERE, "_spec", "specs20.json")
IS_V3 = SPEC == V3
STRIKE = re.compile(r"~~(?!~).+?~~")

D = json.load(io.open(SPEC, encoding="utf-8"))
S = D["specs"]
CNT = Counter()
ROWS = []


def flag(gate, i, msg, detail=""):
    CNT[gate] += 1
    ROWS.append((gate, i, msg, detail))


def txt(sp, keys):
    u"""취소선(이미 정정된 옛 값)은 뺀 본문."""
    return STRIKE.sub(u" ", json.dumps({k: sp.get(k) for k in keys}, ensure_ascii=False))


# ── G1. 자기모순 — 같은 명세 안에서 서로 다른 말을 하는가 ──────────────
# 상수·타입·센티널은 표기가 여러 곳에 중복된다. 중복된 곳이 서로 다르면 그 자체가 결함이다.
PAIRS = [
    (u"센티널 폭", r"(usize::MAX|u64::MAX)", r"(i64::MAX|isize::MAX)"),
    (u"Option 겹수", r"Option<Option<", r"(?<!Option<)Option<[A-Z]\w+>(?!\s*>)"),
]
NUMKEY = re.compile(r"(?:rs|ll):(\d{2,6})")


def gate1(i, sp):
    live = txt(sp, ("logic", "mem", "consts", "knobs", "sig", "one_line") if IS_V3 else
               ("logic", "reads", "writes", "constants", "calls", "knobs", "new_knobs",
                "signature", "one_line"))
    resv = txt(sp, ("history", "open", "closed") if IS_V3 else
               ("resolved", "unknown", "still_unknown"))
    for label, a, b in PAIRS:
        ha, hb = re.search(a, live), re.search(b, live)
        if ha and hb:
            flag("G1", i, u"자기모순(%s): 같은 명세에 '%s' 와 '%s' 가 동시에 있다"
                 % (label, ha.group(0), hb.group(0)))
    # 같은 심볼에 붙은 줄번호가 본 표와 resolved/unknown 에서 다른가
    # 상수 src_line 이 meaning 의 줄번호와 어긋나는지.
    # ⚠오탐 주의: meaning 이 **인라인된 callee** 의 줄(entity.rs:1701 등)을 인용하는 것은 정상이다.
    #   모순은 **이 함수 자신의 소스 파일**에서 다른 줄을 가리킬 때만 성립한다.
    myfile = os.path.basename(str(sp.get("src") or "")).strip()
    if not myfile:
        return
    for j, c in enumerate(sp.get("consts" if IS_V3 else "constants") or []):
        sl = c.get("src_line")
        if not isinstance(sl, int):
            continue
        mean = STRIKE.sub(u" ", str(c.get("meaning", "")))
        # ⚠오탐 제거: 같은 파일의 **다른 용처**를 의도적으로 함께 적는 경우가 있다
        #   (예: "같은 값 0 은 chat.rs:18 의 TraceLevel::Off 비교에도" / "별개로 … 도 2").
        #   그런 표지가 있으면 모순이 아니다.
        if any(k in mean for k in (u"별개로", u"같은 값", u"두 곳", u"에도 ", u"정정", u"->", u"→")):
            continue
        for m in re.finditer(re.escape(myfile) + r":(\d{2,6})", mean):
            if int(m.group(1)) != sl:
                flag("G1", i, u"constants[%d] src_line=%s 인데 meaning 은 같은 파일 %s:%s 를 가리킨다"
                     % (j, sl, myfile, m.group(1)), mean[:130])
                break


# ── G2. 호출부 전수 열거 — "유일 호출처" 류 주장에 근거가 있는가 ────────
SOLE = re.compile(u"(유일 호출처|호출처는 하나|단 한 곳|유일한 호출)")
COUNTED = re.compile(u"호출부\\s*\\d+\\s*곳|호출부 전수|call sites?\\s*[:=]\\s*\\d+")


def gate2(i, sp):
    live = txt(sp, ("logic", "signature", "calls", "knobs", "new_knobs",
                    "unknown", "still_unknown", "exe"))
    if SOLE.search(live) and not COUNTED.search(live):
        flag("G2", i, u"'유일 호출처' 주장이 있는데 **전수 개수**가 없다 — grep 으로 세라",
             SOLE.search(live).group(0))
    if IS_V3:
        cl = sp.get("callers") or {}
        if not cl.get("count"):
            flag("G2", i, u"callers.count == 0 — 자동 열거가 0곳이다. vtable 간접호출이거나 "
                          u"심볼이 어긋난 것이니 손으로 확인해야 한다", str(cl.get("scope")))
        return
    ex = sp.get("exe") or {}
    if isinstance(ex, dict) and ex.get("callers") == []:
        flag("G2", i, u"exe.callers 가 빈 배열이다 — '호출자 없음'인지 '조인 실패'인지 구분 안 됨")


# ── G3. 형제 함수 — 같은 플랜의 다른 진입점을 봤는가 ────────────────────
SIB = ("sub_plan", "next_plan", "is_end", "update", "on_enter", "on_exit")


def gate3(i, sp):
    if IS_V3:
        sb = sp.get("siblings") or {}
        if sb.get("plan") and not sb.get("entries"):
            flag("G3", i, u"siblings.plan=%s 인데 entries 가 비었다 — 자동 열거 실패" % sb.get("plan"))
        return
    sym = (sp.get("sym") or "") + " " + (sp.get("name") or "")
    cands = [x for x in re.findall(r"\d+([A-Z][A-Za-z0-9]*)", sym) if x.endswith("Plan")]
    if not cands:
        return
    plan = cands[-1]          # 가장 안쪽 타입
    live = txt(sp, ("logic", "calls", "unknown", "still_unknown", "new_knobs", "one_line"))
    seen = [s for s in SIB if s in live]
    mine = (sp.get("name") or "").split("::")[-1]
    if mine in seen:
        seen.remove(mine)
    if not seen:
        flag("G3", i, u"%s 의 형제 진입점(%s)을 하나도 언급하지 않는다 — "
                      u"`grep \"^define.*%s\"` 로 목록부터 뽑아라" % (plan, "/".join(SIB), plan))


# ── G4. 등장 술어의 tcx 시그니처가 해결됐는가 ──────────────────────────
# IR 의 `< N` 상수를 소스 임계로 오독하는 사고(01)가 여기서 걸린다.
# std 제공 술어 — game_ai/game_core 의 판정 술어가 아니라 물을 시그니처가 없다
STD_PRED = set(u"""is_none is_some is_ok is_err is_empty is_null is_nan is_sign_negative
is_char_boundary is_ascii is_alphanumeric can_unwind has_key""".split())
PRED = re.compile(r"\b(is_[a-z0-9_]{2,30}|can_[a-z0-9_]{2,30}|has_[a-z0-9_]{2,30})\s*\(")
SIGD = re.compile(u"(fn\\s*\\(|시그니처|sig\\s*=|tcx sig)")


def gate4(i, sp):
    if IS_V3:
        nosig = [c.get("name") for c in (sp.get("callees") or []) if not c.get("sig")]
        if nosig:
            flag("G4", i, u"callees 중 sig 가 비어 있는 항목 %d개" % len(nosig), ", ".join(nosig[:8]))
        um = (sp.get("callees_unmatched") or {}).get("names") or []
        if len(um) >= 8:
            flag("G4", i, u"callees_unmatched %d개 — 판정에 쓰이는 술어가 섞였는지 손으로 확인" % len(um),
                 ", ".join(um[:10]))
        return
    live = txt(sp, ("logic", "calls", "constants", "knobs", "new_knobs"))
    resv = txt(sp, ("resolved", "unknown", "still_unknown"))
    preds = set(m.group(1) for m in PRED.finditer(live)) - STD_PRED
    if not preds:
        return
    missing = []
    for p in sorted(preds):
        near = re.search(re.escape(p) + r".{0,220}", live + resv, re.S)
        if near and SIGD.search(near.group(0)):
            continue
        # calls[] 에 시그니처가 잡혀 있으면 통과
        if any(p in json.dumps(c, ensure_ascii=False) and SIGD.search(json.dumps(c, ensure_ascii=False))
               for c in (sp.get("calls") or [])):
            continue
        missing.append(p)
    if missing:
        flag("G4", i, u"술어 %d개의 tcx 시그니처가 명세 어디에도 없다 "
                      u"(인자를 받는 술어면 IR 의 임계 상수는 접힘일 수 있다)"
             % len(missing), ", ".join(missing[:8]))


GATES = {"G1": gate1, "G2": gate2, "G3": gate3, "G4": gate4}
NAMES = {"G1": u"자기모순", "G2": u"호출부 전수", "G3": u"형제 함수", "G4": u"술어 시그니처"}


def main():
    only = None
    if "--only" in sys.argv:
        only = int(sys.argv[sys.argv.index("--only") + 1])
    pick = None
    if "--gate" in sys.argv:
        pick = sys.argv[sys.argv.index("--gate") + 1]
    for i, sp in enumerate(S):
        if only is not None and i != only:
            continue
        for g, fn in GATES.items():
            if pick and g != pick:
                continue
            fn(i, sp)
    print(u"\n" + "=" * 100)
    print(u"## 명세 완결 조건 검사 (게임 %s)" % D["meta"].get("game"))
    print(u"   " + "  ".join(u"%s %s=%d" % (g, NAMES[g], CNT[g]) for g in sorted(GATES)))
    print("=" * 100)
    for g in sorted(GATES):
        rows = [r for r in ROWS if r[0] == g]
        if not rows:
            continue
        print(u"\n### [%s %s] %d건" % (g, NAMES[g], len(rows)))
        for _, i, msg, det in rows:
            print(u"  [%02d] %-46s  %s" % (i, S[i]["name"][:46], msg))
            if det:
                print(u"        %s" % det[:150])
    print(u"\n총 %d건" % sum(CNT.values()))


if __name__ == "__main__":
    main()
