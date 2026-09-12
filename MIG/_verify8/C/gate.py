# -*- coding: utf-8 -*-
u"""`sig.params[].role` 기계 대조 — **G16 승격 후보**. 8차 배치C 통합판.

후보 4벌(`_gates/params_role/{A_paramuse,B_rolecheck,C_rolechk,D_paramrole}.py`)을 전부 돌리고
**결과가 갈린 이유를 규명한 뒤** 하나로 합친 것이다. 갈림의 원인은 셋이었다.

| 갈림 | 누가 틀렸나 | 무엇 |
|---|---|---|
| ① `define` 헤더 파싱 | **B** | 반환 타입의 `range(i64 …, 206)` 를 인자 괄호로 잡아 인자를 +1 로 셌다. `@심볼` **뒤의** 첫 `(` 를 잡아야 한다(A·D 는 이미 고쳤다). B 의 적발 5건 중 `01`·`19` 두 건이 순전히 이 버그였고 `13` 은 수치가 틀렸다 |
| ② sret 정렬 | **C** | `params[0].i == 0` 인지로 sret 형을 판정했는데, sret 를 **아예 안 싣는 명세**(`02`·`17`)는 그 판정을 통과해 **인자 전체가 한 칸 밀린 채** 검사됐다. C 적발 15건 중 7건이 이 밀림이다 |
| ③ 절(clause) 분해 | **D**(+A) | 「cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) **는 안 씀**」 한 행에서 ⓐ「안 씀」을 인자 전체 부정으로 ⓑ부정절의 0x10 을 「있어야 하는데 없다」로 읽어 **한 행에 오탐 2건**. C 만 이걸 고쳤다 |

## ★★설계 원칙 — **기각에 쓸 수 있는 것만 기각에 쓴다**
`SPEC_RUNBOOK §S5-b` 의 G12 교훈(*귀속은 명세를 구제할 수는 있어도 기각할 수는 없다*)을 그대로 적용했다.
8차에 **D 의 적발 14건 · C 의 적발 15건을 IR 원문으로 전수 반증한 결과 29/29 가 전부 오탐**이었고,
오탐의 근원은 하나였다 — **오프셋 인용(`+0x930` 류)은 그 인자의 gep 오프셋이라는 보장이 없다.**
실제로는 ⓐ한 단계 로드한 하위 객체의 오프셋(`cache.player_champion(+0x1e0)`, `18`)
ⓑ클로저 환경의 오프셋(`filter 클로저 환경 +0x20`, `19`)
ⓒ구조체 레이아웃 서술(`epic(0x78) 스탠스의 tick`, `07`)이 섞여 있다.
⟹ **오프셋 인용은 `약한 후보`로 강등**했다(`--weak` 로만 출력, 기각 근거 아님).

기각에 쓰는 **강한 후보 4종**만 남긴다:

| 코드 | 검사 | 왜 강한가 |
|---|---|---|
| **P1** 정렬 불변식 | `define` 인자 수 ↔ `params[]` 수 (sret·소거·SROA 보정 후) | IR 헤더 원문과의 직접 대조. 어긋나면 **그 명세의 인자 역할 전체가 한 칸 밀린다** |
| **P2** sret 규약 | sret out-ptr 을 `params[0]`(`name`=`(sret…`, `i`=0)로 싣는가 | 8차 확정 규약(§규약). 안 실으면 P1 이 깨지고 재구현자가 ABI 를 못 읽는다 |
| **P3** `i` 번호 규약 | `i` = 소스 시그니처 위치(1-based), sret 는 `0` — `sig.tcx` 의 인자 수로 검산 | `sig.tcx` 가 정본이라 기계로 검산된다 |
| **P4** 속성 주장 | `role` 이 `readnone`/`readonly`/`captures(none)`/`writeonly` 를 주장하면 `define` 의 그 인자에 실제로 있는가 | `define` 줄 원문. 반증이 결정적 |
| **P5** 전면 미사용 | **절 전체가** 인자를 부정(`전혀 안 씀`/`한 번도`/`readnone`)하는데 본문에서 그 레지스터가 call 인자 밖에서 쓰이는가 | 절 단위 + 오프셋·타객체 언급이 없는 절만 본다 |

## 규약 (8차 확정) — sret 은 `params[]` 에 **싣는다**
근거: ① 실측 20함수 중 sret 형은 `00 02 07 15 17` 다섯인데 **`00 07 15` 셋이 이미 싣고 있다**(다수).
② 싣는 쪽이 `params[j]` ↔ IR `%j` 를 자명하게 만든다(SROA 없는 함수에서 1:1).
③ 안 싣는 쪽은 `sig.tcx` 와 중복일 뿐인데, 싣는 쪽의 `(sret)` 행은 **반환값 레이아웃**이라는
   다른 데 없는 정보를 담는다(`00`: `+0x0=태그(-1=None,5=Input::Ult), +0x8=InputTarget(24B)`).
④ 되돌리려면 행 **삭제** 3건이 필요한데 `applypatch` 에는 삭제도 추가도 없다 — 싣는 쪽이 손실이 작다.
`i` 번호는 `07`·`15` 식(**sret=0, 소스 인자 1..n**)이 정본이다. `00` 이 유일한 이탈(1..7)이다.

진입점: `check_spec(sp) -> [(params 인덱스, 사유, 상세)]`  (`specgate.py` G13 과 같은 계약)
용법:  python -X utf8 gate.py [specidx ...] [--weak] [--verbose]
"""
import io, json, os, re, sys

MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

# ── role 어휘 ──────────────────────────────────────────────────────────
# ★**전면 부정**(절 전체가 인자를 부정)에만 쓰는 좁은 목록.
#   C 초판이 `없다` 를 여기 넣어 「쓰기는 **없다**」·「부작용은 **없다**」를 인자 미사용으로 읽고
#   오탐 2건을 냈다(`08`·`15`). 「쓰기/부작용이 없다」는 **인자 미사용 주장이 아니다**.
FULL_NEG = re.compile(
    u"전혀 안 (씀|쓰|읽)|한 번도 (참조|로드|안)|readnone|미사용|"
    u"본문에서 (직접 )?안 (씀|쓴다|읽|쓰임|읽힘|읽음)|이 본문에선 안 씀|"
    u"본문에서 전혀|직접 안 씀|직접 소비하지 않|직접 (읽|쓰)지 않|사용 0회|"
    u"안 읽음\\b|안 씀\\b|안 쓴다\\b|안 읽는다\\b|읽지 않는다\\b|쓰이지 않|"
    u"전달만|그대로 전달|로만 전달|넘기기만")
# 인자가 **아닌 다른 베이스**를 가리키는 절의 신호어 — 이런 절에선 오프셋·부정 판정을 하지 않는다.
# 긍정 사용 서술 — 이게 role 안에 하나라도 있으면 그 role 의 부정절은 **부분 부정**이다.
POS_USE = re.compile(
    u"읽는다|읽음|읽고|읽어|만 읽|쓴다\\b|쓰기|사용\\b|사용한다|접근|만진|승격|spill|"
    u"저장|담(?:아|겨|긴|는)|기록|갱신|로드|상태변경|갱신하는|를 씀|만 씀|둘만|"
    u"필드|오프셋|store|load|gep|getelementptr")
OTHER_BASE = re.compile(
    u"클로저|환경|캡처|캐시\\.|cache\\.|피호출자|레코드|반환값|슬롯|스탠스|팻포인터|"
    u"그 \\+0x|내부|안의|하위|필드 1개|필드 2개|정본|tcx|오라클|호출부|근거")
DROPPED = re.compile(u"poison|인자로 안 넘어옴|인자에서 제거|인자 없음")
ATTRS = ("readnone", "readonly", "writeonly", "captures(none)", "sret", "noalias", "nonnull")
OFFPAT = re.compile(r"\+?0x([0-9a-fA-F]{1,5})\b")
# role 산문 안의 IR 인용: `m13.ll:18805 / 35679 / 37141` 형태의 연속 줄번호까지 받는다.
CITE = re.compile(r"(m\d+\.ll):(\d+)((?:\s*[/~·,]\s*\d+)*)")
REGPAT = re.compile(r"(?:IR\s*|IR에(?:는|서)\s*[^%]{0,40})?%(\d+)\b")
# 절 분해 — 한국어 서술이므로 구두점으로 쪼갠다(C 의 설계를 그대로 흡수).
CLAUSE = re.compile(u"[.。]\\s+|(?<=[다움씀함김])\\s*,\\s*|\\s—\\s|\\s\\u2014\\s")

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
    u"""`define` 헤더 인자 분해. ★`@심볼` **뒤**의 첫 `(` 부터 센다 —
    반환 타입의 `range(i64 .., 206)` 를 인자 괄호로 잡는 것이 B 의 오탐 3건의 원인이었다."""
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
    u"""첫 인자가 반환 out-ptr 인가. ⚠`internal fastcc` 는 `sret(..)` 속성을 떼지만
    슬롯은 그대로다(`15 single_try_engage` 가 그 형태) — 그래서 두 번째 판정이 필요하다."""
    if not a0:
        return False
    return ("sret(" in a0) or ("dead_on_unwind" in a0 and "writable" in a0 and "writeonly" in a0)


def tcx_arity(sp):
    u"""`sig.tcx` 의 소스 인자 수. 못 읽으면 None."""
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
    d, n = 0, 1
    for ch in inner:
        if ch in "(<[":
            d += 1
        elif ch in ")>]":
            d -= 1
        elif ch == "," and d == 0:
            n += 1
    return n


def analyze(body, regs):
    u"""레지스터별 (call 밖 사용 횟수, 총 사용 횟수, 정적 오프셋 집합(깊이 0), 깊이1 오프셋, 동적여부)."""
    gep, owner = {}, {}
    use = {r: 0 for r in regs}
    outcall = {r: 0 for r in regs}
    spill = {r: 0 for r in regs}
    off0 = {r: set() for r in regs}
    off1 = {r: set() for r in regs}
    dyn = {r: False for r in regs}
    pats = {r: re.compile(re.escape(r) + r"(?![\w.])") for r in regs}
    # (root, depth, offset) 추적: gep 는 offset 누적, load 는 depth+1 · offset 리셋
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
                    # ★인자를 alloca 에 그대로 스필하는 store 는 **「전달」의 구현 수단**이다
                    #   (`&version` 으로 넘기려면 주소가 필요하다 — 03 이 정확히 그 형태).
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
    u"""`params[j]` ↔ IR 레지스터 매핑. 돌려주는 것 = (mapping, args, structural_findings).

    ★순서가 중요하다 — **정렬이 틀리면 그 아래 모든 판정이 잡음**이다(C 의 오탐 7건이 그것).
      ① role 이 `IR %k` 로 **명시**하면 그것이 정본(SROA·소거 함수 `05 06 13` 이 그렇게 적혀 있다)
      ② 소거 주장(`poison`)은 레지스터 없음. ⚠「**피호출자**가 poison 을 받는다」는 소거가 아니다
      ③ 나머지는 자리순. sret 가 IR 에만 있으면 한 칸 민다(+ P2 로 보고)
    """
    head, body, lno = defhead(sp)
    if head is None:
        return None, None, []
    args = split_args(head)
    ps = (sp.get("sig") or {}).get("params") or []
    fin = []
    regs = ["%%%d" % k for k in range(len(args))]
    sret = is_sret_arg(args[0]) if args else False
    listed = bool(ps) and (ps[0].get("name") or u"").strip().startswith(u"(sret")

    # ── sret 보정 ──────────────────────────────────────────────────────
    shift = 0
    if sret and not listed and len(args) >= len(ps) + 1:
        shift = 1
        fin.append((0, u"P2 sret 규약 위반 — sret out-ptr 이 `params[]` 에 없다",
                    u"IR %s:%d 첫 인자 `%s` 가 반환 out-ptr 인데 params[0] 은 `%s`. "
                    u"8차 확정 규약 = sret 를 params[0](name `(sret)`, i=0)로 싣는다 "
                    u"(20함수 중 sret 형 00/02/07/15/17 가운데 00·07·15 가 이미 싣는다)"
                    % (sp["ir"]["file"], lno, args[0][:70], ps[0].get("name"))))
    need = len(args) - shift          # 명세가 덮어야 할 IR 인자 수

    # ── ① 자리순 (IR 인자 수 == params 수) ─────────────────────────────
    if need == len(ps):
        return {j: [regs[j + shift]] for j in range(len(ps))}, args, fin

    # ── ② 소거(LLVM 이 미사용 인자를 지운 경우) ────────────────────────
    # ★**소거는 「주장」이 아니라 「잔차」로 센다.** 주장으로 세면 `19` 처럼 role 이
    #   「**피호출자** 호출 인자 자리에 poison 이 들어간다」고 적은 것을 이 함수 인자 소거로
    #   오독해 정렬을 깨뜨린다(B 초판이 자체 적발한 오탐이자, 잔차 방식이면 애초에 안 생긴다).
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

    # ── ③ SROA (IR 인자 수 > params 수) — role 의 명시 `%k` 를 정본으로 ──
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
    for j, why, det in fin:
        out.append((j if j is not None else 0, why, det))

    # ── P3. `i` 번호 규약 ───────────────────────────────────────────────
    n_src = tcx_arity(sp)
    listed = (ps[0].get("name") or u"").strip().startswith(u"(sret")
    if n_src is not None:
        want = [0] + list(range(1, n_src + 1)) if listed else list(range(1, n_src + 1))
        got = [p.get("i") for p in ps]
        if len(want) == len(got) and got != want:
            out.append((0, u"P3 `i` 번호 규약 위반",
                        u"`sig.tcx` 소스 인자 %d개 ⟹ i 는 %s 여야 한다(sret=0, 소스 1..n). 현재 %s. "
                        u"규약 정본 = 07·15" % (n_src, want, got)))

    if mapping is None:
        return out

    regs_used = sorted({r for v in mapping.values() for r in v}, key=lambda x: int(x[1:]))
    use, outcall, spill, off0, off1, dyn = analyze(body, ["%%%d" % k for k in range(len(args))])

    for j, p in enumerate(ps):
        role = p.get("role") or u""
        rl = mapping.get(j) or []
        # ── P4. 속성 주장 대조 ────────────────────────────────────────
        for at in ATTRS:
            if at in role:
                txt = u" ".join(args[int(r[1:])] for r in rl) if rl else u""
                if rl and at not in txt:
                    out.append((j, u"P4 role 이 주장한 IR 속성이 `define` 에 없다",
                                u"p[%d] %s `%s` — IR %s = %s"
                                % (j, p.get("name"), at, ",".join(rl), txt[:100])))
        # ── P5. 전면 미사용 ───────────────────────────────────────────
        # ★두 등급(G12 교훈 그대로). **강함만 기각 근거**다.
        #   role 어딘가에 긍정 사용 서술이 있으면 그 부정절은 **부분 부정**이다 —
        #   「본문은 line 만 읽는다(team **은 안 씀**)」(02) ·
        #   「self.line 만 %0 로 인자승격돼 전달. chats/wait_limit **는 안 씀**」(13) ·
        #   「%0 을 alloca 에 한 번 **spill** 하고 … 넘길 뿐」(03) 이 전부 그 형태였고,
        #   절만 보고 기각하면 8차 초판처럼 **3/3 오탐**이 된다.
        partial = any(POS_USE.search(c or u"") for c in CLAUSE.split(role))
        for cl in CLAUSE.split(role):
            cl = (cl or u"").strip()
            if not cl or not FULL_NEG.search(cl):
                continue
            if OFFPAT.search(cl) or OTHER_BASE.search(cl):
                continue            # 타객체·부분 부정 절 — 인자 전면 부정이 아니다
            for r in rl:
                real = outcall[r] - spill[r]     # 스필 store 는 「전달」의 구현 수단이다
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
        # ── P6. role 이 인용한 IR 줄·명령 대조 (G13 `knobs.where` 와 같은 수법) ──
        # ★후보 4벌 중 아무도 안 본 축이다. `role` 산문 안에도 `m10.ll:33874 `store i64 %0, ptr %11``
        #   같은 **결정적으로 반증 가능한 인용**이 있고, 이건 절 분해·오프셋 귀속 문제가 없다.
        for f, l0, rest in CITE.findall(role):
            lines = [int(l0)] + [int(x) for x in re.findall(r"\d+", rest or u"")]
            got = []
            for L in lines:
                s = _src(f)
                if not (0 < L <= len(s)):
                    out.append((j, u"P6 인용한 IR 줄이 파일 범위 밖",
                                u"p[%d] %s — %s:%d (파일 %d줄)" % (j, p.get("name"), f, L, len(s))))
                    continue
                got.append(s[L - 1])
            # 같은 인용 안의 백틱 조각이 그 줄들 중 하나에 실제로 있는가
            for q in re.findall(r"`([^`]{6,90})`", role):
                if not re.search(r"[=;]|getelementptr|store |load |call |icmp|br ", q):
                    continue            # IR 명령처럼 안 보이는 백틱(심볼명 등)은 대상 아님
                nq = re.sub(r"\s+", " ", q).strip()
                if got and not any(nq in re.sub(r"\s+", " ", g) for g in got):
                    out.append((j, u"P6 인용한 IR 명령이 그 줄에 없다",
                                u"p[%d] %s — `%s` / %s:%s = %s"
                                % (j, p.get("name"), nq[:60], f, lines,
                                   re.sub(r"\s+", " ", got[0]).strip()[:90])))
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
    tot = rows = 0
    strong = 0
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
