# -*- coding: utf-8 -*-
u"""**G18 — `logic` ↔ `mem`/`consts` 상호참조.** 9차 배치D 신설.

## 1. 왜 이 축인가 — `G6`·`G8` 이 안 보는 방향
`G6`(logic 미반영)·`G8`(표에 옛 값)은 **둘 다 `~~취소선~~` 을 앵커로 쓴다.**
「어느 한쪽이 **정정 선언을 했는데** 다른 쪽이 옛 값을 들고 있나」만 본다.
⟹ **정정이 없었던 것**은 둘 다 못 본다. 즉 **「아예 빠졌나」는 완전 무검사**다.

실제로 그래서 샌 것 — `13 target_bush_v30` 의 `logic` 은 `data.cache` · `data.context` 를 읽는데
`mem` 표에 **`OperationData` 행이 한 줄도 없다**(다른 19개 명세 중 16개가 갖고 있는 그 두 줄이다).
`G6`/`G8` 은 취소선이 없으니 침묵하고, `G14`(mem.dir)는 **있는 행만** 보므로 침묵한다.

## 2. ★판정식은 **반증형** (`G15` 의 설계를 이 축에 적용)
| | 분류식 | **반증식** |
|---|---|---|
| 규칙 | 후보가 있는데 주장이 없으면 불일치 | **주장을 지지하는 관측이 0건일 때만** 불일치 |
| 관측을 늘리면 | 오탐이 는다 | **오탐이 준다** |

여기서 「주장」 = *logic 이 이 자리를 인용했다*, 「지지 관측」 = *표·정본·사전 중 어디든 그 근거가 있다*.
구제 통로를 늘릴수록 적발이 **줄어든다.** 7차 `G12` 가 「귀속을 후보에 더했다가 16→22」로 늘린
함정을 **규칙이 아니라 판정식의 성질로** 막는다.

## 3. 세 탐침 (전역 인용 실측: P1 227 · P1b 58 · P2 297)
### P1 오프셋 인용 (`logic` 의 `+0xNN`) — 구제 6
  ① 같은 값이 `mem/consts/knobs` 의 `offset` 칸에 있다(`0x08`↔`0x8` **정수 정규화**)
  ② 표 문면(name/value/note) 어디든 그 표기가 있다
  ③ **범위 구제** — 어떤 `mem` 행이 `off ≤ x < off+span` 을 덮는다(span = 그 행 문면의 `NNNB`).
     `05 mem[27]`(`V50DiveEpisode(힙 원소) 0x0`, 104B) 한 줄이 `+0x08/0x50/0x54/0x5f` 를 덮고,
     그 행 note 가 「필드 대응은 logic 참조」로 **명시적으로 위임**한다
  ④ **유도 구제** — 같은 문장 안 인용들의 부분합이 표의 어떤 offset 과 같다.
     `15`: 「Entity+0x68 → 페이로드+0x8 → Tower+0xb8」 ⟹ 0x68+0x8+0xb8 = **0x128**(표에 있다)
  ⑤ **부정 문맥** — `05`「…(self+0x578)와 gap_ticks(self+0x5c0)는 **읽히지 않는다**」.
     안 읽는다고 적은 자리는 표에 행이 **없는 것이 옳다**
  ⑥ **IR 미접촉** — 그 오프셋이 담당 IR 범위에서 한 번도 만들어지지 않는다.
     `15` 의 `+0x180/0x1a0/…` 는 콜리 `iter_towers_without_nexus` 안의 자리다(이 범위 IR 에 `i64 384` **0건**)

### P1b **base 가 붙은** 오프셋 인용 (`Entity+0x660`) — P1 의 base 맹점 보완
P1 은 값만 보므로 `battle.chats /*+0x68*/` 이 **다른 구조체**의 `Entity+0x68` 때문에 구제된다.
P1b 는 그 base 의 행만 본다. ⚠소문자 변수(`self+0x5d0`)는 좌표계를 모르므로 **판정하지 않는다**.

### P2 필드 인용 (`logic` 의 `a.b.F` 사슬 잎) — 구제 7
⚠`logic` 은 자유 서술 의사코드라 잎이 **필드인지 메서드인지 의사표기인지** 모른다.
그래서 **「후보 제시형」**이고, 구제로 걸러 남은 것만 낸다:
  ① `mem[].name` 에 있다  ② 표 문면 어디든 있다  ③ 메서드꼴 이름(`is_*`·`len`·…)
  ④ ★`distruct.json` 에 **그 이름의 필드가 없다** ⟹ 필드가 아니다(메서드·지역변수)
  ⑤ 그 사슬이 **주석(`//` 뒤)에만** 나온다  ⑥ 뿌리가 `_`(=`_shared.` 교차참조)
  ⑦ ★**레지스터 승격** — `sig.params[].role` 이 그 필드를 「승격」으로 적었다(`_promoted`)

⛔**「부재」의 방향이 G14 와 반대다** — 이 게이트가 재는 것은 IR 의 부재가 아니라 **표의 부재**다.
   그래서 IR 은 **구제(⑥)에만** 쓰고 기각 근거로 쓰지 않는다.

## 4. 검출력 (변이 시험 — `mutate.py`) · 오탐 이력
`mem` 행을 하나씩 지워 `logic` 의 인용을 고아로 만들고 되잡는지 센다.
**포착 68/308 = 22.1%**(분모 = `logic` 이 인용하는 행) · 전 행 기준 68/451 = 15.1% ·
`offset` 교란 변이는 46/308 = 14.9%.

★**개발 중 후보 적발 누계 20건 중 IR 원문 반증을 통과한 것은 1건**(오탐 19 = 95%).
  폐기한 탐침 하나 = **상수 리터럴 대조**(6건 전수 오탐 — `-> 55`·`11 Revenant` 같은 화살표·열거
  나열을 비교연산자로 오인했다. `logic` 의 숫자는 줄번호가 대부분이라 이 축은 **재료 부재**).

사용: `python -X utf8 gate.py [specidx ...]` / `specgate` 진입점 = `check_spec(sp)`
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
if MIG not in sys.path:
    sys.path.insert(0, MIG)

STRIKE = re.compile(r"~~(?!~).+?~~")
# 주소 문맥의 오프셋 인용만 센다. 맨 `0x10` 은 상수일 수도 있어 대상이 아니다.
CITE = re.compile(r"(?:@\s*)?\+\s*(0x[0-9a-fA-F]+)|@\s*(0x[0-9a-fA-F]+)")
SPAN = re.compile(r"(\d{2,5})\s*B\b")
FLD = re.compile(r"\b([a-z_][a-z0-9_]*)((?:\.[a-z_][a-z0-9_]*)+)\b(?!\s*\()")
SENT = re.compile(r"[\n;{}]|(?<=[.!?])\s")
TXTKEYS = ("mem", "consts", "knobs")   # ★표만. sig·notes 까지 넓히면 검출력이 27.9→14.9% 로 반토막(REPORT §3)

_DS = [None]


def _fieldnames():
    u"""`distruct.json` 의 **모든 필드명 집합**. 읽기만 한다(쓰기 금지)."""
    if _DS[0] is None:
        s = set()
        try:
            d = json.load(io.open(os.path.join(MIG, "distruct.json"), encoding="utf-8"))
            for _sn, rec in d.items():
                for f in (rec or {}).get("fields") or []:
                    if f.get("name"):
                        s.add(f["name"])
        except Exception:
            pass
        _DS[0] = s
    return _DS[0]


def _blob(sp):
    return json.dumps({k: sp.get(k) for k in TXTKEYS}, ensure_ascii=False)


# ★부정 문맥 — `logic` 이 「읽히지 않는다」고 적은 자리는 **표에 행이 없는 것이 옳다**.
#   (`05`: 「live 의 prev_holder_hp 페이로드(self+0x578)와 gap_ticks(self+0x5c0)는 읽히지 않는다」)
NEGCTX = re.compile(u"읽히지\\s*않|안\\s*읽|쓰이지\\s*않|미사용|사용하지\\s*않|제외|버린다|poison|죽은")

_TOUCH = {}


def _ir_offsets(sp):
    u"""★**IR 구제 전용** — 이 함수의 IR 범위에서 실제로 만들어진 오프셋 집합.

    `mem` 은 **이 함수가 짚는 자리**의 표다. 콜리 안에서만 일어나는 접근은 행이 없는 것이 옳다
    (`15`: `iter_towers_without_nexus` 안의 `+0x180/0x1a0/…` — 실측 이 범위 IR 에 `i64 384` 0건).
    ⚠**기각에는 절대 쓰지 않는다.** IR 에 보인다고 결함이 되는 게 아니라, **안 보이면 구제**다."""
    ir = sp.get("ir") or {}
    key = (ir.get("file"), ir.get("frm"), ir.get("to"))
    if key[0] is None:
        return None
    if key not in _TOUCH:
        try:
            import memdir as MD
            _r, _w, _hr, _hw, touch = MD.scan(*key)
            _TOUCH[key] = {o for (_root, o) in touch}
        except Exception:
            _TOUCH[key] = None
    return _TOUCH[key]


def _rows(sp):
    return sp.get("mem") or []


def _promoted(sp):
    u"""★**레지스터 승격 구제** — `sig.params[].role` 이 「…로 승격」이라 적은 인자의 필드.

    `13 target_bush_v30` 의 `define` 은 `(i8 %0, i64 %1, i32 %2, ptr %3, ptr %4)` 다 —
    `data.cache`/`data.context` 는 **호출부가 이미 꺼내서 넘긴 포인터**이고 이 함수는
    `OperationData` 를 한 번도 짚지 않는다. ⟹ `mem` 에 행이 **없는 것이 옳다.**
    (초판이 이걸 실오류로 올렸다가 `define` 원문으로 뒤집었다 — REPORT §4)"""
    sig = sp.get("sig") or {}
    txt = u" ".join(str(p.get("role") or u"") for p in (sig.get("params") or []))
    return txt


def _tbl_offsets(sp):
    u"`mem`(및 표) 의 offset 칸 정수 집합."
    out = set()
    for f in ("mem", "consts", "knobs"):
        for x in sp.get(f) or []:
            o = str(x.get("offset") or u"")
            m = re.match(r"^\s*0x([0-9a-fA-F]+)\s*$", o)
            if m:
                out.add(int(m.group(1), 16))
    return out


def _spans(sp):
    u"""`[(off, span)]` — 행이 덮는 구간. span 은 그 행 문면의 `NNNB`(없으면 0)."""
    out = []
    for x in _rows(sp):
        m = re.match(r"^\s*0x([0-9a-fA-F]+)\s*$", str(x.get("offset") or u""))
        if not m:
            continue
        base = int(m.group(1), 16)
        txt = u" ".join(str(x.get(k) or u"") for k in ("name", "value", "note"))
        sizes = [int(s) for s in SPAN.findall(txt)]
        out.append((base, max(sizes) if sizes else 0))
    return out


def _subset_hits(vals, targets, cap=5):
    u"""`vals` 의 부분합 중 `targets` 에 드는 것이 있나(유도 구제). 항 수는 cap 으로 제한."""
    vals = list(vals)[:cap]
    sums = {0}
    for v in vals:
        sums |= {s + v for s in sums}
    return bool((sums - {0}) & targets)


def _p1(sp):
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        return []
    tbl = _tbl_offsets(sp)
    spans = _spans(sp)
    blob = _blob(sp)
    out = []
    for sent in SENT.split(lg):
        cited = [int((a or b), 16) for a, b in CITE.findall(sent or u"")]
        if not cited:
            continue
        for x in sorted(set(cited)):
            if x in tbl:                                             # 구제①
                continue
            if re.search(r"0x0*%x\b" % x, blob, re.I):               # 구제②
                continue
            if any(b <= x < b + s for (b, s) in spans if s):         # 구제③
                continue
            if _subset_hits([c for c in set(cited) if c != x] + [x], tbl):   # 구제④
                continue
            if NEGCTX.search(sent or u""):                                   # 구제⑤ 부정 문맥
                continue
            iro = _ir_offsets(sp)
            if iro is not None and x not in iro:                             # 구제⑥ IR 미접촉
                continue
            out.append((u"P1", x, sent.strip()[:110]))
    return out


METHODISH = re.compile(r"^(?:is_|as_|to_|has_|get_|set_|iter|len|unwrap|clone|count|min|max|"
                       r"abs|sqrt|rev|sum|map|filter|next|last|first|into|from|new|push|"
                       r"contains|any|all|some|none|cmp|eq|ne|and_then|or_else|expect)")
# ★의사코드 낱말 — `logic` 은 열거형 페이로드 추출을 `mode.payload` 로 적는다. 이건 필드 접근이
#   아니라 **표기**다(구제. 초판이 `10 mode.payload` 를 이 이유로 오탐했다 — REPORT §4).
PSEUDO = {u"payload", u"variant", u"discriminant", u"판별자", u"inner", u"value"}


def _p2(sp):
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        return []
    names = u" ".join(str(x.get("name") or u"") for x in _rows(sp))
    blob = _blob(sp)
    fields = _fieldnames()
    prom = _promoted(sp)
    # 주석 전용 여부 판정용: 코드 줄(주석 제거)만 모은 본문
    code = u"\n".join(re.sub(r"//.*$", u" ", ln) for ln in lg.split(u"\n"))
    out = []
    for m in FLD.finditer(lg):
        root, tail = m.group(1), m.group(2)
        leaf = tail.split(u".")[-1]
        if len(leaf) < 4:
            continue
        if root.startswith(u"_") or leaf in PSEUDO:                   # 구제⑥·의사코드 낱말
            continue
        if leaf in names:                                            # 구제①
            continue
        if re.search(r"(?<![A-Za-z0-9_])%s(?![A-Za-z0-9_])" % re.escape(leaf), blob):   # 구제②
            continue
        if METHODISH.match(leaf) or leaf not in fields:              # 구제③④
            continue
        if re.search(r"(?<![A-Za-z0-9_])%s(?![A-Za-z0-9_])" % re.escape(leaf), prom):   # 구제⑦
            continue
        whole = m.group(0)
        if whole not in code:                                        # 구제⑤ (주석에만 등장)
            continue
        out.append((u"P2", whole, leaf))
    # 같은 잎은 한 번만
    seen, uniq = set(), []
    for r in out:
        if r[2] in seen:
            continue
        seen.add(r[2])
        uniq.append(r)
    return uniq


# ── P1b. **base 가 붙은** 오프셋 인용 — `Entity+0x660` · `Tower+0xb8` ──────────
# P1 은 오프셋 값만 보므로 `battle.chats /*+0x68*/` 이 `Entity+0x68`(다른 구조체) 때문에 구제된다.
# P1b 는 **그 base 의 행에** 있는지를 본다. 대문자로 시작하는 낱말만 타입으로 인정한다
# (`self+0x5d0` 류 지역변수는 좌표계를 모르므로 **판정하지 않는다** — 재료 부재이지 오류가 아니다).
TYOFF = re.compile(r"\b([A-Z][A-Za-z0-9_]*)\s*\+\s*0x([0-9a-fA-F]+)")


def _head(b):
    s = re.sub(r"\s*\(.*?\)\s*", u" ", (b or u"").strip())
    s = re.sub(r"^dyn\s+|^bumpalo\s+|^반환\s+", u"", s)
    return re.sub(r"\s*(?:::)?\s*vtable$", u"", s).split(u".")[0].strip()


def _p1b(sp):
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        return []
    byhead = {}
    for x in _rows(sp):
        m = re.match(r"^\s*0x([0-9a-fA-F]+)\s*$", str(x.get("offset") or u""))
        if not m:
            continue
        txt = u" ".join(str(x.get(k) or u"") for k in ("name", "value", "note"))
        sizes = [int(s) for s in SPAN.findall(txt)]
        byhead.setdefault(_head(x.get("base")), []).append(
            (int(m.group(1), 16), max(sizes) if sizes else 0))
    blob = _blob(sp)
    out = []
    for m in TYOFF.finditer(lg):
        ty, x = m.group(1), int(m.group(2), 16)
        lst = byhead.get(ty)
        if lst is None:
            continue                       # 그 타입이 이 명세의 base 로 없다 → **판정하지 않는다**
        if any(b == x or (s and b <= x < b + s) for (b, s) in lst):      # 구제①②
            continue
        seg = blob
        if re.search(r"%s[^\"]{0,90}0x0*%x\b" % (re.escape(ty), x), seg, re.I):   # 구제③
            continue
        if _subset_hits([v for (v, _s) in lst] + [x], {v for (v, _s) in lst}):    # 구제④
            continue
        sent = lg[max(0, m.start() - 120):m.end() + 60]
        if NEGCTX.search(sent):                                                  # 구제⑤
            continue
        iro = _ir_offsets(sp)
        if iro is not None and x not in iro:                                     # 구제⑥
            continue
        out.append((u"P1b", ty, x, sent.strip()[:110]))
    seen, uniq = set(), []
    for r in out:
        if (r[1], r[2]) in seen:
            continue
        seen.add((r[1], r[2]))
        uniq.append(r)
    return uniq


def check_spec(sp):
    u"""★`specgate G18` 진입점 → `[(슬롯, 사유, 상세)]`.

    슬롯 = `logic/P1` · `logic/P2`. **표에 빠진 것**만 낸다(값 대조는 G6/G8 몫)."""
    out = []
    for (_t, x, sent) in _p1(sp):
        out.append((u"logic/P1", u"logic 이 `+0x%x` 를 인용하는데 mem 표에 그 오프셋이 없다" % x,
                    sent))
    for (_t, ty, x, sent) in _p1b(sp):
        out.append((u"logic/P1b", u"logic 이 `%s+0x%x` 를 인용하는데 base=%s 행에 그 오프셋이 없다"
                    % (ty, x, ty), sent))
    for (_t, whole, leaf) in _p2(sp):
        out.append((u"logic/P2", u"logic 이 필드 `%s` 를 읽는데 mem 표에 그 필드 행이 없다" % whole,
                    u"잎 `%s` 는 distruct 의 실재 필드명" % leaf))
    return out


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    n1 = n2 = 0
    for i in idxs:
        sp = D["specs"][i]
        rows = check_spec(sp)
        if not rows:
            continue
        print(u"\n===== specs[%d] %s" % (i, sp["name"]))
        for (slot, why, det) in rows:
            print(u"  ★%s %s\n        %s" % (slot, why, det))
            if "P1" in slot:
                n1 += 1
            else:
                n2 += 1
    print(u"\n---- P1 오프셋 누락 %d · P2 필드 누락 %d" % (n1, n2))


if __name__ == "__main__":
    main()
