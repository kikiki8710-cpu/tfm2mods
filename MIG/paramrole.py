# -*- coding: utf-8 -*-
u"""`sig.params[].role` 기계 대조 — **G16 승격판(9차 배치B)**.

8차 배치C 통합판(`MIG\\paramrole.py`)을 그대로 물려받고, 그 판이 내던
**`P4` 6건 · `P6` 14건을 IR 원문으로 전수 반증한 결과 20/20 이 전부 오탐**이어서
그 오탐을 만든 6개 결함을 고쳤다. (반증 내역 = `_verify9/B/REPORT.md` §2)

| # | 결함 | 증상(8차 판) | 고침 |
|---|---|---|---|
| **F1** | P4 가 **인용문 안의 속성어**를 「주장」으로 읽음 | `02`·`17` p[1] 의 role 이 **%0(sret) 의 define 헤더를 백틱으로 인용**했는데 그 안의 `writeonly`/`noalias`/`captures(none)` 을 p[1] 의 주장으로 읽어 4건 | 백틱 스팬이 **IR 인용**이면(=`define`/opcode/`%N =` 포함) 주장 텍스트에서 **제거**. 인용은 근거이지 주장이 아니다 |
| **F2** | `sret` 주장을 아무 행에서나 검사 | `02`·`17` p[1] 의 「이 params 표는 **sret** out-ptr 을 빠뜨렸다」를 p[1] 이 sret 속성을 주장한 것으로 읽어 2건 | `sret` 속성 주장은 **sret 행(`name` 이 `(sret` 로 시작)에서만** 검사 |
| **F3** | ★백틱 **짝짓기 붕괴** | 인용 정규식이 `` `([^`]{6,90})` `` 이라 **90자 넘는 인용은 건너뛰는 게 아니라 짝이 한 칸 밀린다** — 열림/닫힘이 뒤바뀌어 **산문이 「인용」으로 날조**된다. `02`·`17` 의 P6 2건이 정확히 그것(날조된 인용 = `(i=0) 행을 params[0] 로 싣는 것인데`) | 짝짓기는 `` `([^`]*)` `` 로 **길이 무관**하게 하고, 길이 필터는 짝을 지은 **뒤에** 적용 |
| **F4** | P6 이 **인용 × 줄번호 교차곱** | `03` p[4] 는 인용 5군데·IR 조각 3개인데 조각을 **모든** 줄번호와 대조해 12건. 실제로는 5군데 전부 IR 과 **일치**한다 | 조각은 **직전 인용**(없으면 직후)에만 귀속. 사이의 산문이 40자를 넘으면 귀속 포기(=기각 안 함) |
| **F5** | 「IR 처럼 보임」 필터가 한글 산문을 통과 | `(i=0) 행을 params[0] 로…` 가 `[=;]` 하나로 통과 | **한글이 들어간 조각은 IR 이 아니다**(제외) + opcode/`%N =` 를 요구 |
| **F6** | 생략기호(`…`)·축약 인용을 불일치로 셈 | `define void @…AttackNexusPlan8sub_plan(…)` 처럼 심볼을 줄여 쓴 인용 | ① `…`/`...` 를 와일드카드로 보고 **조각 순서 일치** ② 그래도 안 되면 **토큰 순서부분수열**(≥80%) 로 구제 |
| **F7** | `33864~34294` 를 **줄 목록**으로 오독 | `~` 를 `/` 와 같은 구분자로 읽어 두 줄만 봄 | `/ · ,` = 목록 · `~` = **범위**(구간 전체를 훑음). 그 절이 **부정**(`0건`·`없다`·`거짓`)이면 귀속 포기 |

★설계 원칙은 8차 그대로다 — **기각에 쓸 수 있는 것만 기각에 쓴다.**
오프셋 인용(`+0x930` 류)은 8차에 29/29 오탐이 나와 `--weak` 전용으로 남아 있다.

| 코드 | 검사 | 왜 강한가 |
|---|---|---|
| **P1** 정렬 불변식 | `define` 인자 수 ↔ `params[]` 수(sret·소거·SROA 보정 후) | IR 헤더 원문과의 직접 대조 |
| **P2** sret 규약 | sret out-ptr 을 `params[0]`(`name`=`(sret…`, `i`=0)로 싣는가 | 8차 확정 규약. 안 실으면 P1 이 깨진다 |
| **P3** `i` 번호 규약 | `i` = 소스 시그니처 위치(1-based), sret=`0` — `sig.tcx` 인자 수로 검산 | `sig.tcx` 가 정본 |
| **P4** 속성 주장 | `role` **본인의 주장**이 `readnone`/`readonly`/`writeonly`/`captures(none)` 이면 `define` 의 그 인자에 실제로 있는가 | `define` 줄 원문. 반증이 결정적 |
| **P5** 전면 미사용 | **절 전체가** 인자를 부정하는데 본문에서 call 인자 밖에 쓰이는가 | 절 단위 + 오프셋·타객체 언급 없는 절만 |
| **P6** 인용 대조 | role 이 인용한 `mNN.ll:LINE` 과 그 옆 IR 조각이 실제로 그 줄에 있는가 | 줄 원문. 반증이 결정적 |

진입점: `check_spec(sp) -> [(params 인덱스, 사유, 상세)]`  (`specgate.py` G13 과 같은 계약)
용법:  python -X utf8 gate.py [specidx ...] [--weak] [--verbose]
"""
import io
import re, json, os, re, sys

MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

# ── role 어휘 ──────────────────────────────────────────────────────────
FULL_NEG = re.compile(
    u"전혀 안 (씀|쓰|읽)|한 번도 (참조|로드|안)|readnone|미사용|"
    u"본문에서 (직접 )?안 (씀|쓴다|읽|쓰임|읽힘|읽음)|이 본문에선 안 씀|"
    u"본문에서 전혀|직접 안 씀|직접 소비하지 않|직접 (읽|쓰)지 않|사용 0회|"
    u"안 읽음\\b|안 씀\\b|안 쓴다\\b|안 읽는다\\b|읽지 않는다\\b|쓰이지 않|"
    u"전달만|그대로 전달|로만 전달|넘기기만")
POS_USE = re.compile(
    u"읽는다|읽음|읽고|읽어|만 읽|쓴다\\b|쓰기|사용\\b|사용한다|접근|만진|승격|spill|"
    u"저장|담(?:아|겨|긴|는)|기록|갱신|로드|상태변경|갱신하는|를 씀|만 씀|둘만|"
    u"필드|오프셋|store|load|gep|getelementptr")
OTHER_BASE = re.compile(
    u"클로저|환경|캡처|캐시\\.|cache\\.|피호출자|레코드|반환값|슬롯|스탠스|팻포인터|"
    u"그 \\+0x|내부|안의|하위|필드 1개|필드 2개|정본|tcx|오라클|호출부|근거")
DROPPED = re.compile(u"poison|인자로 안 넘어옴|인자에서 제거|인자 없음")
ATTRS = ("readnone", "readonly", "writeonly", "captures(none)", "sret", "noalias", "nonnull")


def NEGATED(clause, at):
    u"""속성어가 부정 서술(`readonly 없음` · `noalias·readonly 없음` · `readonly 가 없다`) 안에만 나오나.
    ★19차 A([80] p[1] `&self`): 속성어 여러 개를 `·`/`,` 로 묶어 「… 없음」 이라 쓴 절도 부정으로 본다."""
    import re as _re
    attrw = u"(?:readnone|readonly|writeonly|noalias|nonnull|captures\\(none\\)|dereferenceable(?:\\(\\d+\\))?)"
    sep = u"[^\\uac00-\\ud7a3A-Za-z]{0,8}"
    # 20차 A/D: `readonly 아님` · `3속성 전부 없음` — 부정 어휘 확장 + 수량어(`3속성`·`세 속성`) 통과
    pat = _re.compile(u"(?:" + sep + attrw + u")*" + sep + u"(?:\\d+속성|세 속성|셋 다|전부|모두)?\\s*(없음|없다|없고|없어|빠짐|빠져|제외|부재|미부착|전부 없|아님|아니다|아니고|아니라)")
    # 비교 대상(다른 함수)의 속성을 말하는 자리 — `update_on_dead(&mut self) 는 noalias dereferenceable(6168)` — 도 이 인자의 주장이 아니다.
    cmp = _re.compile(u"\\((?:&mut self|&self|&mut [A-Za-z_]+|&[A-Za-z_]+)\\)\\s*(?:는|은|=|:)\\s*$")
    for m in _re.finditer(_re.escape(at), clause):
        if cmp.search(clause[max(0, m.start() - 40):m.start()]):
            continue
        if not pat.match(clause[m.end():m.end() + 80]):
            return False
    return True


OFFPAT = re.compile(r"\+?0x([0-9a-fA-F]{1,5})\b")
CLAUSE = re.compile(u"[.。]\\s+|(?<=[다움씀함김])\\s*,\\s*|\\s—\\s|\\s\\u2014\\s")

# ── F7. 인용 문법 ─────────────────────────────────────────────────────
#   `mNN.ll:1234`                 한 줄
#   `mNN.ll:12/34/56`             줄 목록 (`/` `·` `,`)
#   `mNN.ll:100~200`              줄 **범위** (`~` `〜` `–`)  ← 8차 판은 이걸 목록으로 읽었다
CITE = re.compile(r"(m\d+\.ll):(\d+)"
                  r"((?:\s*[/\u00b7,]\s*\d+)+)?"
                  r"(?:\s*[~\u301c\u2013]\s*(\d+))?")
# ── F5. 「IR 조각처럼 보임」 — 한글이 있으면 IR 이 아니다 ──────────────
HANGUL = re.compile(u"[\uac00-\ud7a3\u3131-\u318e]")
QIR = re.compile(r"\b(?:define|declare|getelementptr|store|load|call|invoke|icmp|fcmp|"
                 r"phi|select|br|ret|alloca|bitcast|zext|sext|trunc|switch)\b"
                 r"|%\d+\s*=")
NEGCLAUSE = re.compile(u"0건|없다|아니다|거짓|하나도|없음|미존재|안 나온다")

# ★F8 — 그 절이 **다른 크레이트/다른 IR 파일**을 인용하나. (12차 배치D)
#   `_gcbc`/`_gvbc` 경로나 `gNN.ll`/`vNN.ll`, 또는 이 명세의 IR 파일이 아닌 `mNN.ll` 이 나오면
#   그 절의 속성 주장은 **콜리의 것**이다.
_XCITE = re.compile(r"_g[cv]bc|[gv]\d+\.ll|(m\d+\.ll)")


def FOREIGN(clause, myfile):
    for m in _XCITE.finditer(clause or u""):
        if m.group(1) is None:          # `_gcbc`·`gNN.ll`·`vNN.ll` = 무조건 외부
            return True
        if myfile and m.group(1) != myfile:
            return True                 # 다른 `mNN.ll` = 같은 크레이트의 **다른 함수**
    return False
META = re.compile(u"params\\[|이 params 표|표는 |행 추가|applypatch|8차 확정 규약|"
                  u"규약 정본|도시에|명세 표")

REGPAT = re.compile(r"(?:IR\s*|IR에(?:는|서)\s*[^%]{0,40})?%(\d+)\b")
GEP = re.compile(r"^\s*%(\d+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr (%\d+), i64 (-?\d+)")
GEPD = re.compile(r"^\s*%(\d+) = getelementptr\b.*?, ptr (%\d+), i64 %")
LOAD = re.compile(r"^\s*%(\d+) = load [^,]+, ptr (%\d+)")
STORE = re.compile(r"^\s*store [^,]+, ptr (%\d+)")
CALLY = re.compile(r"\b(?:tail |musttail |notail )?(?:call|invoke)\b")

_CACHE = {}


def _src(f):
    if f not in _CACHE:
        _CACHE[f] = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                            errors="replace").read().split("\n")
    return _CACHE[f]


def split_args(ln):
    u"""`define` 헤더 인자 분해. ★`@심볼` **뒤**의 첫 `(` 부터 센다."""
    at = ln.find("@")
    i = ln.find("(", at if at >= 0 else 0)
    if i < 0:
        return []
    depth, j = 0, i
    while j < len(ln):
        if ln[j] == "(":
            depth += 1
        elif ln[j] == ")":
            depth -= 1
            if depth == 0:
                break
        j += 1
    inner = ln[i + 1:j]
    parts, d, cur = [], 0, ""
    for ch in inner:
        if ch in "([{":
            d += 1
        elif ch in ")]}":
            d -= 1
        if ch == "," and d == 0:
            parts.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def defhead(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a:
        return None, None, None
    src = _src(f)
    for k in range(a - 1, min(a + 6, len(src))):
        if src[k].lstrip().startswith("define"):
            return src[k], src[a - 1:b], k + 1
    return None, None, None


def is_sret_arg(a0):
    if not a0:
        return False
    return ("sret(" in a0) or ("dead_on_unwind" in a0 and "writable" in a0 and "writeonly" in a0)


def tcx_arity(sp):
    t = ((sp.get("sig") or {}).get("tcx") or u"")
    i = t.find("fn(")
    if i < 0:
        return None
    depth, j, start = 0, i + 2, i + 3
    while j < len(t):
        if t[j] in "(<[":
            depth += 1
        elif t[j] in ")>]":
            depth -= 1
            if depth == 0:
                break
        j += 1
    inner = t[start:j]
    if not inner.strip():
        return 0
    # ★P3 규약 = i 는 **소스 시그니처 위치**(팻포인터도 1행). IR 슬롯 수가 필요하면 ir_slots() 를 따로 쓴다(09-13).
    return len(split_top(inner))


def split_top(inner):
    u"""괄호 깊이 0 의 쉼표로 인자 문자열을 나눈다."""
    out, d, cur = [], 0, []
    for ch in inner:
        if ch in "(<[{":      # ★`{}` 도 깊이에 넣는다 — `&dyn [Binder { value: …, bound_vars: [] }]` 안의 쉼표(09-13)
            d += 1
        elif ch in ")>]}":
            d -= 1
        if ch == "," and d == 0:
            out.append("".join(cur).strip()); cur = []
        else:
            cur.append(ch)
    if "".join(cur).strip():
        out.append("".join(cur).strip())
    return out


def ir_slots(arg):
    u"""tcx 파라미터 1개 → IR 인자 슬롯 수. ★09-13(15차 배치A 적발): `&dyn Trait`·`&[T]`·`&str`·`Box<dyn>` 는
    **팻포인터 = (ptr, vtable|len) 2슬롯**이라 IR/명세 params 가 tcx 보다 한 행 많은 것이 정상이다(#21/#23 오탐의 근원)."""
    a = (arg or u"").strip()
    core = re.sub(r"^&(?:'\w+\s+)?(?:mut\s+)?", "", a)
    if core.startswith("dyn ") or core.startswith("[") or core == "str" or core.startswith("Box<dyn") or core.startswith("[Binder"):
        return 2
    if a.startswith("&") and ("dyn " in core[:6] or core.startswith("[")):
        return 2
    return 1


# ── F3. 백틱 짝짓기 — 길이 무관하게 짝을 짓고, 길이 필터는 **뒤에** ────
def backticks(role):
    u"""(시작위치, 내용) 목록. ★길이 상한을 정규식에 넣으면 **짝이 밀려 산문이 인용으로 날조**된다
    (8차 판 `` `([^`]{6,90})` `` 의 실제 사고). 짝은 먼저 짓고 거르는 것은 나중이다."""
    return [(m.start(1), m.group(1)) for m in re.finditer(r"`([^`]*)`", role)]


def is_ir_quote(q):
    u"""F5. 이 백틱 조각이 **IR 원문 인용**인가. 한글이 있으면 아니다."""
    if len(q.strip()) < 6:
        return False
    if HANGUL.search(q):
        return False
    return bool(QIR.search(q))


def strip_ir_quotes(role):
    u"""F1. **IR 인용은 근거이지 주장이 아니다** — 주장 스캔 전에 지운다."""
    return re.sub(r"`([^`]*)`",
                  lambda m: u" " if is_ir_quote(m.group(1)) else m.group(0), role)


def _norm(s):
    return re.sub(r"\s+", " ", s).strip()


_TOK = re.compile(r"[%@]?[\w.$]+")


def quote_in(q, lines):
    u"""F6. 인용 조각이 대상 줄들 중 하나에 있는가.
    ① `…`/`...` 를 와일드카드로 본 **조각 순서 일치** ② 토큰 **순서부분수열** ≥80% 로 구제."""
    nq = _norm(q)
    frags = [f for f in re.split(u"\u2026|\\.\\.\\.", nq) if f.strip()]
    for g in lines:
        ng = _norm(g)
        pos, ok = 0, True
        for f in frags:
            k = ng.find(f.strip(), pos)
            if k < 0:
                ok = False
                break
            pos = k + len(f.strip())
        if ok:
            return True
    # 구제 — 축약 인용(`player_by_champion_id(%18, i64 noundef %47)` 처럼 심볼·속성을 줄인 것).
    # ★단 **하드 토큰**(레지스터 `%N` · 홀로 선 정수 리터럴)은 **전부** 있어야 한다.
    #   비율만 보면(초판 ≥80%) 정수 하나를 위조해도 통과한다 — 변이 시험 C 가 0/4 였던 원인.
    toks = _TOK.findall(nq)
    if not toks:
        return True
    hard = [t for t in toks if re.match(r"^%\d+$", t) or re.match(r"^\d+$", t)]
    soft = [t for t in toks if t not in hard]
    for g in lines:
        ng = _norm(g)
        pos, ok = 0, True
        for t in hard:                       # 하드 토큰은 순서대로 **전부**
            k = ng.find(t, pos)
            if k < 0:
                ok = False
                break
            pos = k + len(t)
        if not ok:
            continue
        if not soft:
            return True
        hit, pos = 0, 0
        for t in soft:
            k = ng.find(t, pos)
            if k >= 0:
                hit += 1
                pos = k + len(t)
        if hit >= max(1, int(0.8 * len(soft))):
            return True
    return False


def cites(role):
    u"""F7. (시작, 끝, file, [줄...], is_range, 원문) 목록."""
    out = []
    for m in CITE.finditer(role):
        f, l0, lst, rng = m.group(1), int(m.group(2)), m.group(3), m.group(4)
        if rng:
            a, b = l0, int(rng)
            out.append((m.start(), m.end(), f, [a, b], True, m.group(0)))
        else:
            lines = [l0] + [int(x) for x in re.findall(r"\d+", lst or u"")]
            out.append((m.start(), m.end(), f, lines, False, m.group(0)))
    return out


def analyze(body, regs):
    u"""레지스터별 (총 사용, call 밖 사용, 스필, 깊이0 오프셋, 깊이1 오프셋, 동적여부)."""
    use = {r: 0 for r in regs}
    outcall = {r: 0 for r in regs}
    spill = {r: 0 for r in regs}
    off0 = {r: set() for r in regs}
    off1 = {r: set() for r in regs}
    dyn = {r: False for r in regs}
    pats = {r: re.compile(re.escape(r) + r"(?![\w.])") for r in regs}
    tag = {r: (r, 0, 0) for r in regs}

    for ln in body:
        s = ln.strip()
        if s.startswith("#dbg_") or s.startswith(";") or s.startswith("define"):
            continue
        iscall = bool(CALLY.search(ln))
        for r in regs:
            n = len(pats[r].findall(ln))
            if n:
                use[r] += n
                if not iscall:
                    outcall[r] += n
                    if re.match(r"^\s*store [^,]+ " + re.escape(r) + r"\s*,\s*ptr %\d+", ln):
                        spill[r] += n
        m = GEP.match(ln)
        if m:
            b = m.group(2)
            if b in tag:
                root, dep, o = tag[b]
                no = None if o is None else o + int(m.group(3))
                tag["%" + m.group(1)] = (root, dep, no)
                if root in off0:
                    if no is None:
                        dyn[root] = True
                    elif dep == 0:
                        off0[root].add(no)
                    else:
                        off1[root].add(no)
            continue
        m = GEPD.match(ln)
        if m:
            b = m.group(2)
            if b in tag:
                root, dep, o = tag[b]
                tag["%" + m.group(1)] = (root, dep, None)
                if root in dyn:
                    dyn[root] = True
            continue
        m = LOAD.match(ln)
        if m:
            b = m.group(2)
            if b in tag:
                root, dep, o = tag[b]
                if dep < 2:
                    tag["%" + m.group(1)] = (root, dep + 1, 0)
            continue
    return use, outcall, spill, off0, off1, dyn


def align(sp):
    u"""`params[j]` ↔ IR 레지스터 매핑 = (mapping, args, structural_findings)."""
    head, body, lno = defhead(sp)
    if head is None:
        return None, None, []
    args = split_args(head)
    ps = (sp.get("sig") or {}).get("params") or []
    fin = []
    regs = ["%%%d" % k for k in range(len(args))]
    sret = is_sret_arg(args[0]) if args else False
    listed = bool(ps) and (ps[0].get("name") or u"").strip().startswith(u"(sret")

    shift = 0
    if sret and not listed and len(args) >= len(ps) + 1:
        shift = 1
        fin.append((0, u"P2 sret 규약 위반 — sret out-ptr 이 `params[]` 에 없다",
                    u"IR %s:%d 첫 인자 `%s` 가 반환 out-ptr 인데 params[0] 은 `%s`. "
                    u"8차 확정 규약 = sret 를 params[0](name `(sret)`, i=0)로 싣는다"
                    % (sp["ir"]["file"], lno, args[0][:70], ps[0].get("name"))))
    elif sret and not listed and len(args) == len(ps):
        # 행은 있는데 **표식이 없다** — 자리는 맞아 보이지만 재구현자가 %0 을 소스 인자로 읽는다.
        fin.append((0, u"P2 sret 규약 — params[0] 가 sret out-ptr 인데 `(sret` 표식이 없다",
                    u"IR %s:%d 첫 인자 `%s` 가 반환 out-ptr 인데 params[0].name=`%s`. "
                    u"규약 = name 을 `(sret)` 로 두고 i=0"
                    % (sp["ir"]["file"], lno, args[0][:70], ps[0].get("name"))))
    need = len(args) - shift

    if need == len(ps):
        return {j: [regs[j + shift]] for j in range(len(ps))}, args, fin

    # ★09-13: 팻포인터(`&dyn`/`&[T]`/`&str`) 는 소스 1행 = IR 2슬롯 — 행의 type 으로 슬롯 수를 세어
    #   합이 IR 인자 수와 같으면 (data, vtable|len) 두 레지스터를 그 행에 매핑한다(#21/#23 · 15차 배치A 「오탐」 판정의 실체).
    widths = [ir_slots(p.get("type") or u"") for p in ps]
    if sum(widths) == need:
        m, pos = {}, shift
        for j, w in enumerate(widths):
            m[j] = regs[pos:pos + w]; pos += w
        return m, args, fin

    if need < len(ps):
        want = len(ps) - need
        cand = [j for j, p in enumerate(ps)
                if DROPPED.search(p.get("role") or u"")
                and not re.search(u"피호출자|호출 인자 자리|호출 시|arg\\d", p.get("role") or u"")]
        if len(cand) == want:
            live = [j for j in range(len(ps)) if j not in cand]
            m = {j: [] for j in cand}
            for n, j in enumerate(live):
                m[j] = [regs[n + shift]]
            return m, args, fin
        fin.append((None, u"P1 정렬 불변식 깨짐(소거 주장 수 불일치)",
                    u"IR 인자 %d개(sret보정 -%d) vs params %d개 ⟹ 소거 %d개가 필요한데 "
                    u"role 이 소거를 주장한 것은 %d개(p%s). 자리 매핑 보류"
                    % (len(args), shift, len(ps), want, len(cand), cand)))
        return None, args, fin

    explicit, union = {}, set()
    for j, p in enumerate(ps):
        rr = {"%" + m for m in re.findall(r"(?<![\w.])%(\d+)\b", p.get("role") or u"")}
        rr = {r for r in rr if r in regs[shift:]}
        if rr:
            explicit[j] = sorted(rr, key=lambda x: int(x[1:]))
            union |= rr
    if len(explicit) == len(ps) and union == set(regs[shift:]):
        return {j: explicit.get(j, []) for j in range(len(ps))}, args, fin
    fin.append((None, u"P1 정렬 불변식 깨짐(SROA 인자 승격 미기재)",
                u"IR 인자 %d개(sret보정 -%d) > params %d개인데 role 이 `%%k` 로 승격 대응을 "
                u"전부 적지 않았다(적은 것 p%s, 덮은 레지스터 %s). 자리 매핑 보류"
                % (len(args), shift, len(ps), sorted(explicit),
                   sorted(union, key=lambda x: int(x[1:])))))
    return None, args, fin


GAP = 40        # F4. 인용과 조각 사이 산문이 이보다 길면 귀속을 포기한다(=기각 안 함)


def check_spec(sp, weak=False):
    u"""★`specgate` 진입점. `[(params 인덱스, 사유, 상세)]`."""
    out = []
    ps = (sp.get("sig") or {}).get("params") or []
    if not ps:
        return out
    head, body, lno = defhead(sp)
    if head is None:
        return out
    mapping, args, fin = align(sp)
    myfile = (sp.get("ir") or {}).get("file")   # F8 — 외부 IR 인용 판별용
    for j, why, det in fin:
        out.append((j if j is not None else 0, why, det))

    # ── P3. `i` 번호 규약 ───────────────────────────────────────────────
    n_src = tcx_arity(sp)
    listed = (ps[0].get("name") or u"").strip().startswith(u"(sret")
    if n_src is not None:
        want = [0] + list(range(1, n_src + 1)) if listed else list(range(1, n_src + 1))
        got = [p.get("i") for p in ps]
        if len(want) != len(got):
            # ★행 수 자체가 안 맞으면 그 아래 모든 역할 서술이 한 칸씩 밀린다.
            #   (변이 시험 F 의 사각 5건 · G 의 5건이 전부 여기로 잡힌다)
            out.append((0, u"P3 `params[]` 행 수가 `sig.tcx` 소스 인자 수와 다르다",
                        u"`sig.tcx` 소스 인자 %d개(sret 행 %s) ⟹ %d행이어야 하는데 %d행. "
                        u"현재 i = %s"
                        % (n_src, u"있음" if listed else u"없음", len(want), len(got), got)))
        elif got != want:
            out.append((0, u"P3 `i` 번호 규약 위반",
                        u"`sig.tcx` 소스 인자 %d개 ⟹ i 는 %s 여야 한다(sret=0, 소스 1..n). 현재 %s. "
                        u"규약 정본 = 07·15" % (n_src, want, got)))

    if mapping is None:
        return out

    use, outcall, spill, off0, off1, dyn = analyze(body, ["%%%d" % k for k in range(len(args))])

    for j, p in enumerate(ps):
        role = p.get("role") or u""
        rl = mapping.get(j) or []
        is_sret_row = (p.get("name") or u"").strip().startswith(u"(sret")

        # ── P4. 속성 **주장** 대조 (F1 인용 제거 · F2 sret 행 한정) ────
        claim = strip_ir_quotes(role)
        for at in ATTRS:
            if at not in claim:
                continue
            if at == "sret" and not is_sret_row:
                continue                      # F2: sret 주장은 sret 행에서만 의미가 있다
            # F1-b: 명세 표 자체를 논하는 메타 절은 IR 속성 주장이 아니다
            cls = [c for c in CLAUSE.split(claim) if c and at in c]
            if cls and all(META.search(c) for c in cls):
                continue
            # ★★F8 — **콜리(다른 크레이트)의 `define` 인용**은 이 함수의 속성 주장이 아니다.
            #   (12차 배치D 적발. 도시에가 「G16 이 콜리 define 인용을 자기 함수에서 찾는다」고
            #    **오탐 모드를 예고까지 했는데** 게이트가 그대로 발화했다 — 예고할 수 있으면
            #    게이트가 걸러야 한다.)
            #   실례 `18 sig.params[2]`: 문면은 「**본체** `_gcbc/g15.ll:130480` 의 `rnd` 인자에
            #   `readnone` 이 붙어 있어」이고 `_gcbc` = **game_core 크레이트**다. 18 자신의
            #   `define`(m09.ll:6879)에 `readnone` 이 없는 것이 **맞다**.
            if cls and all(FOREIGN(c, myfile) for c in cls):
                continue
            # ★F9(19차 A · [80] p[1]): 「noalias·readonly **없음**」 처럼 **부재를 서술**한 절은 속성 주장이 아니라 그 반대다.
            #   절 안에서 속성어 뒤에 「없음/없다/없고/빠짐/제외」가 따르면 부정 서술로 보고 건너뛴다.
            if cls and all(NEGATED(c, at) for c in cls):
                continue
            txt = u" ".join(args[int(r[1:])] for r in rl) if rl else u""
            if rl and at not in txt:
                out.append((j, u"P4 role 이 주장한 IR 속성이 `define` 에 없다",
                            u"p[%d] %s `%s` — IR %s = %s"
                            % (j, p.get("name"), at, ",".join(rl), txt[:100])))

        # ── P5. 전면 미사용 ───────────────────────────────────────────
        partial = any(POS_USE.search(c or u"") for c in CLAUSE.split(role))
        for cl in CLAUSE.split(role):
            cl = (cl or u"").strip()
            if not cl or not FULL_NEG.search(cl):
                continue
            if OFFPAT.search(cl) or OTHER_BASE.search(cl):
                continue
            for r in rl:
                real = outcall[r] - spill[r]
                if real <= 0:
                    continue
                tier = u"[약함] " if partial else u""
                if partial and not weak:
                    continue
                out.append((j, u"%sP5 「미사용/전달만」인데 call 밖에서 쓰인다" % tier,
                            u"p[%d] %s %s — 절 「%s」 / call 밖 사용 %d회(스필 %d 제외) · "
                            u"총 %d회 · 오프셋 %s"
                            % (j, p.get("name"), r, cl[:46], real, spill[r], use[r],
                               sorted(hex(x) for x in off0[r]))))

        # ── P6. 인용 대조 (F3 짝짓기 · F4 근접 귀속 · F5~F7) ───────────
        cl_list = cites(role)
        if cl_list:
            for qpos, q in backticks(role):
                if not is_ir_quote(q):
                    continue
                # F4. 직전 인용(없으면 직후) 하나에만 귀속
                before = [c for c in cl_list if c[1] <= qpos]
                if before:
                    c = max(before, key=lambda x: x[1])
                    gap = qpos - c[1]
                else:
                    c = min(cl_list, key=lambda x: x[0])
                    gap = c[0] - (qpos + len(q))
                if gap > GAP:
                    continue                  # 귀속 불가 — 기각하지 않는다
                st, en, f, lines, isrng, raw = c
                seg = role[max(0, st - 90):min(len(role), qpos + len(q) + 30)]
                if isrng and NEGCLAUSE.search(seg):
                    continue                  # F7. 「0건이다」 류 부정 주장은 이 검사의 대상이 아니다
                src = _src(f)
                if isrng:
                    a, b = lines
                    if not (0 < a <= len(src)) or not (0 < b <= len(src)) or b < a:
                        out.append((j, u"P6 인용한 IR 줄이 파일 범위 밖",
                                    u"p[%d] %s — %s:%d~%d (파일 %d줄)"
                                    % (j, p.get("name"), f, a, b, len(src))))
                        continue
                    got = src[a - 1:b]
                    tgt = u"%s:%d~%d" % (f, a, b)
                else:
                    got, bad = [], False
                    for L in lines:
                        if not (0 < L <= len(src)):
                            out.append((j, u"P6 인용한 IR 줄이 파일 범위 밖",
                                        u"p[%d] %s — %s:%d (파일 %d줄)"
                                        % (j, p.get("name"), f, L, len(src))))
                            bad = True
                            continue
                        got.append(src[L - 1])
                    if bad or not got:
                        continue
                    tgt = u"%s:%s" % (f, lines)
                if not quote_in(q, got):
                    out.append((j, u"P6 인용한 IR 명령이 그 줄에 없다",
                                u"p[%d] %s — `%s` / %s = %s"
                                % (j, p.get("name"), _norm(q)[:70], tgt,
                                   _norm(got[0])[:90])))

        # ── E(약한 후보). 오프셋 인용 — **기각 근거가 아니다** ────────
        if weak:
            for cl in CLAUSE.split(role):
                cl = (cl or u"").strip()
                if not cl or OTHER_BASE.search(cl) or FULL_NEG.search(cl):
                    continue
                want = {int(x, 16) for x in OFFPAT.findall(cl)}
                if not want or not rl:
                    continue
                real = set()
                for r in rl:
                    real |= off0[r] | off1[r]
                    if dyn[r]:
                        want = set()
                miss = sorted(want - real)
                if miss:
                    out.append((j, u"[약함] 인용 오프셋이 인자 사슬(깊이≤1)에 없다",
                                u"p[%d] %s — %s / 실제 %s / 절 「%s」"
                                % (j, p.get("name"), [hex(x) for x in miss],
                                   [hex(x) for x in sorted(real)][:10], cl[:46])))
    return out


def main():
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    weak = "--weak" in sys.argv
    verb = "--verbose" in sys.argv
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    tot = rows = strong = 0
    for i in idxs:
        sp = D["specs"][i]
        r = check_spec(sp, weak=weak)
        ps = (sp.get("sig") or {}).get("params") or []
        rows += len(ps)
        tot += len(r)
        strong += len([x for x in r if not x[1].startswith(u"[약함]")])
        print(u"specs[%2d] %-46s params %-2d · 적발 %d"
              % (i, sp["name"], len(ps), len(r)))
        for (j, why, det) in r:
            print(u"    p[%s] %s\n         %s" % (j, why, det))
        if verb:
            m, args, _ = align(sp)
            if m is not None:
                print(u"    정렬: %s" % {k: v for k, v in sorted(m.items())})
    print(u"\n%s\n params %d행 · 적발 %d(강 %d / 약 %d)\n%s"
          % (u"=" * 88, rows, tot, strong, tot - strong, u"=" * 88))


if __name__ == "__main__":
    main()
