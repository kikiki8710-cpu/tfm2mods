# -*- coding: utf-8 -*-
u"""`consts[].src_line` 기계 대조 — 그 리터럴이 실제로 어느 소스 줄에서 왔는지 **inlinedAt 사슬 전체**로 본다.

판정: 주장한 `src_line` 이 그 리터럴을 쓰는 명령의 사슬(담당 `.rs` 파일 프레임만) 어디에도 없으면 **틀린 값**.
※ 사슬 전체를 보는 이유 = 인라인된 accessor 는 최내곽이 `entity.rs` 이고, 진짜 소스 줄은 중간 프레임에 있다.

## ★7차 교정 — 초판은 **적발 47건 중 대부분이 오탐**이었다
배치 A·D 가 **독립적으로** 담당분을 줄 원문으로 전수 반증했고, 둘 다 같은 결론을 냈다:
「지시대로 고쳤으면 판정 반전 오류」. 새는 곳이 셋이었다.

| # | 샌 곳 | 고친 법 |
|---|---|---|
| ① | 리터럴 정규식이 `%2`(SSA)·`!2`(메타데이터)·`align 2`·`dereferenceable(816)`·gep 오프셋을 상수로 착각 | **화이트리스트**(`litpat`) — 타입 접두(`i8 3`)나 쉼표 뒤(`, 4`)만 진짜 상수 |
| ② | `switch` case 라벨 줄엔 `!dbg` 가 없다(헤더·푸터·arm 에 붙는다) | **귀속 추론**(`_switch_dbg` arm 종결자 · `_switch_own` 헤더/푸터) |
| ③ | `phi` 의 상수 인입도 `!dbg` 가 없어 `line 0` 으로 버려진다 | **귀속 추론**(`_term_dbg` 인입 블록 종결자) |

교정 직후 실측 **47 → 10건**(그 시점 값 — **현재값은 `python -X utf8 specgate.py` 로 재라**).

⚠**게이트 건수를 문서에 적지 마라.** 명세가 자라면 변한다. 실제로 8차 배치A 가
「`G12` 수치가 이 docstring(10)·`SPEC_RUNBOOK §S5-b`(13)·도시에(8)에서 **셋 다 다르다**」를 적발했다.
`SPEC_RUNBOOK` 이 이미 「기준선은 숫자 대신 명령」이라고 적어 두고 있었는데 내가 그걸 어겼다.

⚠★**귀속은 「약한 후보」다 — 구제에만 쓰고 기각에는 쓰지 않는다.**
교정 도중 ②③의 귀속을 그냥 후보에 더했더니 **16 → 22 로 늘었다.** 판정식이
「후보가 있는데 claim 이 없으면 불일치」라서 **후보를 늘리는 것이 곧 오탐을 늘린다.**
⟹ 강한 후보(그 줄 자체의 `!dbg`)만 불일치 근거로 쓰고, 약한 후보는 claim 확인에만 쓴다.
추론으로 얻은 근거로 남의 주장을 뒤집으면 그건 게이트가 아니라 또 하나의 추측이다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = HERE   # ★MIG 루트로 승격(6차 배치A 산출물을 정식 도구로 편입)
IRDIR = r"C:\tfm2mods\_gaibc"
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')
DBG = re.compile(r"!dbg !(\d+)")

_cache = {}

# ★리터럴 매칭은 **화이트리스트**로 한다. (7차 배치A·D 가 독립적으로 같은 오탐을 짚었고,
#   배치D 의 「타입 접두가 붙은 것만 진짜 상수」 쪽이 더 정밀해서 그걸 채택했다.)
#
#   초판의 `(?<![\w.\-])2(?![\w.])` 는 `%2`(SSA 레지스터)·`!2`(메타데이터 id)·`align 2`·
#   `dereferenceable(816)`·`getelementptr .. i64 16`(필드 오프셋)을 **전부 상수로 착각**했다.
#   「실제 후보 = [310, 336, 338]」 같은 목록이 통째로 잡음이었고,
#   그걸 믿고 고쳤다면 배치A 담당 8건이 **전부 판정 반전 오류**가 났을 것이다.
#
#   ⟹ LLVM 에서 진짜 상수 피연산자는 **타입 접두**(`i8 3` · `i64 -1`)가 붙거나
#      **쉼표 뒤**(`icmp ugt i64 %30, 4`)에 온다. 그 둘만 센다.
_TY = r"(?:i1|i8|i16|i32|i64|i128|float|double)"


def litpat(val):
    return re.compile(r"(?:\b" + _TY + r"\s+|,\s*)" + re.escape(str(val)) + r"(?![\w.])")


def load(f):
    if f in _cache:
        return _cache[f]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    meta = {}
    for ln in src:
        if ln.startswith("!"):
            m = re.match(r"^!(\d+) = (.*)$", ln)
            if m:
                meta[m.group(1)] = m.group(2)
    _cache[f] = (src, meta)
    return _cache[f]


def scope_file(meta, sid, d=0):
    cur = sid
    while cur and cur in meta and d < 40:
        t = meta[cur]
        m = re.search(r"file: !(\d+)", t)
        if m:
            fn = FILEOF.search(meta.get(m.group(1), ""))
            if fn:
                return fn.group(1).split("\\")[-1]
        m2 = re.search(r"scope: !(\d+)", t)
        if not m2:
            return "?"
        cur = m2.group(1)
        d += 1
    return "?"


def chain(meta, n):
    out, cur, d = [], n, 0
    while cur and cur in meta and d < 48:
        m = LOC.search(meta[cur])
        if not m:
            break
        line, scope, inl = m.groups()
        out.append((scope_file(meta, scope), int(line)))
        if not inl:
            break
        cur, d = inl, d + 1
    return out


def _term_dbg(src, a, b, label):
    u"""블록 `%label` 의 **종결자 `!dbg`** 를 찾는다.

    ★왜 필요한가 — `phi` 의 상수 인입(`[ 5, %49 ]`)과 `switch` 의 case 라벨 줄에는
    `!dbg` 가 **아예 없다**(종결자에 붙는다). 그래서 G12 가 진짜 정답 줄을 **못 보고**,
    대신 잡음 후보만 내놨다. 상수가 어디서 왔는지는 **그 인입 블록의 종결자**가 말해준다."""
    lab = re.compile(r"^%s:" % re.escape(label))
    st = None
    for k in range(a - 1, min(b, len(src))):
        if lab.match(src[k].strip()):
            st = k
            break
    if st is None:
        return None
    for k in range(st, min(st + 400, b, len(src))):
        s = src[k].strip()
        if s.startswith(("br ", "switch ", "ret ", "unreachable", "indirectbr ")):
            m = DBG.search(src[k])
            if m:
                return m.group(1)
            # ★09-15(23차 E·F 적발 · 169/170 consts[2] · 174/175 case 라벨 3건 오탐): 여러 줄 `switch` 는
            #   `!dbg` 가 **닫는 `]` 줄**에 붙는다. 헤더에 없으면 `]` 줄까지 내려가 본다. `invoke` 도 2줄(`to label`).
            if s.startswith("switch ") or " invoke " in (" " + s):
                for j in range(k + 1, min(k + 400, b, len(src))):
                    t = src[j].strip()
                    m2 = DBG.search(src[j])
                    if t.startswith("]") or t.startswith("to label"):
                        return m2.group(1) if m2 else None
                    if t.endswith(":") or not t:
                        break
            return None
    return None


CASELBL = re.compile(r"^i\d+\s+-?\d+\s*,\s*label\s+%([\w.$]+)\s*$")


def _switch_own(src, k, a):
    u"""case 라벨 줄이 속한 `switch` 의 **헤더 또는 푸터 `]`** 에 붙은 `!dbg`. (7차 배치D)

    arm 종결자(`_switch_dbg`)가 「그 case 를 타면 무엇을 하는가」를 말한다면,
    이쪽은 「그 `match` 문 자체가 소스 어디인가」를 말한다. **둘 다 정답일 수 있어** 둘 다 본다
    — 단 어느 쪽도 **약한 후보**다(귀속은 기각 근거가 못 된다)."""
    hdr = None
    for j in range(k - 1, max(a - 2, k - 400), -1):
        s = src[j].strip()
        if s.startswith("switch ") and s.endswith("["):
            hdr = j
            break
        if s.startswith("]"):
            return None
    if hdr is None:
        return None
    for j in range(k, min(k + 400, len(src))):
        if src[j].strip().startswith("]"):
            m = DBG.search(src[j]) or DBG.search(src[hdr])
            return m.group(1) if m else None
    m = DBG.search(src[hdr])
    return m.group(1) if m else None


def _switch_dbg(src, k, a, b):
    u"""case 라벨 줄 `i64 2, label %37` 이면 **그 arm 블록 `%37` 의 종결자** `!dbg` 를 쓴다.

    ⚠닫는 `]` 줄의 `!dbg` 가 아니다 — 그건 switch 문 자체의 위치이고,
    **case 값이 어느 소스 줄에서 왔는지는 그 arm 이 말한다**(7차 배치A 실측:
    `i64 2, label %37` → 블록 `%37` 종결자 `!dbg !44410` = `jungle.rs:528`)."""
    m = CASELBL.match(src[k].strip())
    if not m:
        return None
    return _term_dbg(src, a, b, m.group(1))


def _in_switch(src, k, a):
    for j in range(k - 1, max(a - 2, k - 400), -1):
        s = src[j].strip()
        if s.startswith("switch ") and s.endswith("["):
            return True
        if s.startswith("]"):
            return False
    return False


PHIIN = re.compile(r"\[\s*(-?\d+)\s*,\s*%([\w.$]+)\s*\]")


def dbg_ids(src, meta, k, a, b, val):
    u"""k 번째 줄에서 상수 `val` 의 `!dbg` 후보. **(강한, 약한)** 두 벌로 돌려준다.

    ★등급을 나누는 이유 — 처음엔 `switch` arm·`phi` 인입 귀속을 그냥 후보에 **더했더니
    오탐이 16→22 로 늘었다.** 귀속은 *추론*이라 엉뚱한 줄을 끌고 올 수 있는데,
    G12 의 판정식이 「후보가 있는데 claim 이 없으면 불일치」라서
    **후보를 늘리는 것이 곧 오탐을 늘린다.** (배치A 의 8건에 맞춰 규칙을 넣다 과대적합했다.)

    ⟹ **강한 후보**(그 줄 자체에 `!dbg` 가 붙은 명령)만 불일치 판정의 근거로 쓰고,
    **약한 후보**(귀속 추론)는 *claim 이 맞는지 확인하는 데만* 쓴다.
    귀속은 명세를 **구제할 수는 있어도 기각할 수는 없다.**"""
    ln = src[k]
    strong, weak = [], []
    m = DBG.search(ln)
    if m:
        strong.append(m.group(1))
    s = ln.strip()
    if " phi " in ln or s.startswith("phi ") or "= phi" in ln:
        for v, lab in PHIIN.findall(ln):
            if v == str(val):
                d = _term_dbg(src, a, b, lab)
                if d:
                    weak.append(d)
    elif not m and _in_switch(src, k, a):
        for d in (_switch_dbg(src, k, a, b), _switch_own(src, k, a)):
            if d:
                weak.append(d)
    return strong, weak


def check_spec(sp):
    u"""★`specgate G12` 진입점. 불일치만 `[(consts 인덱스, 주장한 줄, 실제 후보)]` 로 돌려준다.

    「리터럴 미검출」은 **불일치가 아니다** — 상수가 접혀 그 자리에 리터럴이 없는 정상 경우다.
    실제 후보가 **있는데** 주장한 줄이 거기 없을 때만 결함으로 본다.
    """
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    own = str(sp.get("src") or "").split("\\")[-1]
    try:
        src, meta = load(f)
    except Exception:
        return []
    out = []
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        if not isinstance(claim, int):
            continue
        pat = litpat(val)
        found, soft = {}, set()
        for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            if "#dbg_" in ln or not pat.search(ln):
                continue
            st, wk = dbg_ids(src, meta, k, a, b, val)
            for did in st:
                for (fn, li) in chain(meta, did):
                    if fn == own:
                        found[li] = found.get(li, 0) + 1
            for did in wk:
                for (fn, li) in chain(meta, did):
                    if fn == own:
                        soft.add(li)
        # ★줄 `0` 은 함수 진입 `!dbg` 라 **신호가 아니다.** 후보가 `{0}` 뿐이면
        #   「리터럴이 접혔거나 범위 밖」과 같은 판정보류이지 불일치가 아니다.
        #   (초판이 이걸 후보로 세서 56건 중 상당수가 오탐이었다.)
        cands = sorted(x for x in found if x)
        # ★`soft`(switch arm·phi 인입 귀속)에 claim 이 있으면 **명세가 옳다** — 통과시킨다.
        #   귀속은 구제에만 쓰고 기각에는 쓰지 않는다(위 `dbg_ids` 주석).
        if cands and claim not in found and claim not in soft:
            out.append((j, claim, cands))
    return out


if __name__ != "__main__":
    # ★import 될 때는 아래 리포트 루프를 돌리지 않는다(게이트에서 부르므로).
    import sys as _s
    _s.modules[__name__].__dict__.setdefault("_asmod", True)
else:
    _asmod = False

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
for i in (range(20) if __name__ == "__main__" else []):          # ★전 20함수 (초판은 range(5) = 배치 A 범위만이었다)
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = load(f)
    print(u"\n===== specs[%d] %s  (%s  %s:%d~%d) =====" % (i, sp["name"], own, f, a, b))
    # 범위 안 모든 명령의 사슬에서 '담당 .rs' 프레임 줄 집합
    ownlines = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = DBG.search(ln)
        if not m:
            continue
        for (fn, li) in chain(meta, m.group(1)):
            if fn == own:
                ownlines.setdefault(li, 0)
                ownlines[li] += 1
    print(u"  본문이 실제로 참조하는 %s 줄 = %s" % (own, sorted(ownlines)))
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        pat = re.compile(r"(?<![\w.\-])" + re.escape(str(val)) + r"(?![\w.])")
        found = {}
        for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            if "#dbg_" in ln or not pat.search(ln):
                continue
            m = DBG.search(ln)
            if not m:
                continue
            for (fn, li) in chain(meta, m.group(1)):
                if fn == own:
                    found.setdefault(li, 0)
                    found[li] += 1
        ok = claim in found
        print(u"  consts[%d] value=%-12s claim=L%-5s 실제후보=%-28s %s"
              % (j, val, claim, sorted(found) if found else u"(리터럴 미검출)",
                 u"OK" if ok else (u"**불일치**" if found else u"(판정보류: 리터럴이 접혔거나 범위 밖)")))
