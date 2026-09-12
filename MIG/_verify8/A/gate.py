# -*- coding: utf-8 -*-
u"""**G14 — `mem[].dir`(읽기/쓰기 방향) 기계 대조.** 8차 배치A 통합판.

`_gates\\mem_dir\\` 의 후보 4벌(A·B·C·D)을 합치고, 합본이 낸 적발을 **IR 원문으로 전수 반증**해
오탐을 거르길 세 번 반복한 결과다. **전역 적발 = 0.**

## 1. 후보 4벌이 왜 갈렸나 (전역 실행 실측: **A=7 · B=3 · C=46 · D=7**)

독해력 차이가 아니라 **판정식**이 갈렸다.

| 갈림점 | A | B | C | D | 통합판 |
|---|---|---|---|---|---|
| **오프셋이 IR 에 안 보일 때** | 보류 | 보류 | **결함** | 보류 | **보류** |
| 호출 인자로 주소만 넘김(`addr`) | 없음 | r 만 | 없음 | 없음 | **`dereferenceable(N)` × `readonly` 로 방향별** |
| `0x860[len]` 같은 **인덱스 표기** | 파싱실패→보류 | 0x860 으로 오인 | 0x860 으로 오인 | 예외→스킵 | **대상 제외**(구조체 오프셋이 아니다) |
| `dir=-`(방향 미기재) | 불일치로 셈 | 스킵 | 스킵 | 불일치로 셈 | **스킵** |
| 접근 폭(`memcpy 24B`·`memset 80B`·`store i64`) | 무시 | 무시 | 무시 | 무시 | **커버리지 구제** |
| ptr `phi`/`select` 합류 | 미추적 | 미추적 | **합집합** | 미추적 | **합집합** |
| 가변 인덱스 gep 의 정수 `phi` 상수 | 미추적 | 미추적 | **추적** | 미추적 | **추적** |
| 베이스 구분 | 없음(접음) | 없음 | 없음 | pair 표는 만드나 판정은 접음 | **(루트, 시프트) 동정** |

★**C 의 46건은 계측이 예민해서가 아니라 판정식이 다른 것**이다. C 는 「그 오프셋으로 가는 load 가
하나도 없으면 결함 후보」라, **콜리 안에서만 접근하는 필드**·**레지스터 승격된 인자**
(specs[10] `MainObjective i24`)·**폭이 넓은 한 번의 쓰기**(specs[17] `memset(+0x120, 0, 80)` 이
덮는 8행)가 전부 적발로 나온다. C 자신의 docstring 이 한계 2·3 으로 그걸 적어 놨다 —
46건 중 상당수는 C 도 오탐임을 알고 있었다.
A 와 D 는 7 로 **수만 같고 집합이 다르다**(A 는 specs[11] `plan+0x5e8`·specs[12] `GameContext+0x38` 을,
D 는 specs[0] `sret+0x8`·specs[12] `CallHandled+0x38` 을 잡는다). 둘 다 「모순만 적발」인데
오프셋 해석 범위가 달라서다 — D 는 `0x860[len]` 을 예외로 떨궈 보고, A 는 `int(,16)` 실패로 버렸다.

⟹ **부재는 결함이 아니다.** 이 게이트가 잡으려는 실패 모드는 **방향 뒤집힘(r↔w)** 뿐이고,
그건 「그 자리가 반대 방향으로만 관측된다」로만 말할 수 있다.

## 2. ★판정식 — 귀속·추론은 **구제에만**

```
0) base 동정: 같은 base 의 다른 행들이 주장대로 확인되는 (루트, 시프트)를 찾는다.
   못 찾으면 → 보류(그 구조체가 이 범위에 없다)
1) claim=r → 그 자리를 덮는 load/memcpy-src 가 있으면 OK
   claim=w → 그 자리를 덮는 store/memcpy-dst/memset 이 있으면 OK
2) 그 주소가 호출 인자로 나갔고 `dereferenceable(N)` 이 그 자리를 덮으면
   · `readonly` 없음(=&mut) → 양방향 보류   · `readonly` 있음 → **r 주장만** 구제
3) 그러고도 남으면 **반대 방향 관측을 증인으로** 불일치를 선언한다. 증인은 0) 으로 동정된 자리여야 한다.
```
7차 `G12` 의 교훈(「후보를 늘리는 것이 곧 오탐을 늘린다」)을 이 축의 고유 실패 모드에 적용한 것이
**base 동정**이다. 오프셋만 접어서 보면 **클로저 환경 구조체의 `+0x20` 저장**이
`BrainMinionParameter.minion_count` 의 반증으로 둔갑한다 — 통합 1판이 실제로 그 오탐을 냈고,
IR 을 따라가 보니 m04.ll:58336 `%76 = getelementptr inbounds nuw i8, ptr %7, i64 32` →
`store ptr %2, ptr %76` 로 **클로저 캡처 구조체**였다(`minion_count` 는 콜리
`has_line_defense_threat`(defense_nexus.rs:588) 안에서 읽힌다).

추가 계측(phi 합집합·정수 phi 오프셋·접근 폭·dereferenceable·base 동정)은 **전부 구제 방향으로만**
작동하게 배치했다. 계측을 켜면 적발이 **줄어야** 정상이다:
접기만 한 1판 **7** → 폭·addr 구제 **3** → base 동정·인덱스표기 제외 **0**.

## 3. 검출력 — 적발 0 이 **무능**이 아니라는 반증
적발 0 은 ①축이 깨끗하다 ②게이트가 아무것도 못 잡는다 로 읽힌다. 갈라내려고 **변이 시험**을 둔다
(`mutate.py` — 행마다 `dir` 을 뒤집어 넣고 되잡는지 센다. 정본은 건드리지 않는다).

| 구성 | 변이 포착 | 실제 명세 적발 |
|---|---|---|
| 접기만(1판) | — | 7 (전수 오탐) |
| + 폭·addr 구제 | 10.0% (45/448) | 3 (전수 오탐) |
| + `readonly` 분리 | 34.6% (155/448) | 0 |
| + base 동정(루트만) | 57.8% (259/448) | 0 |
| **+ 시프트 정렬(최종)** | **53.1% (238/448)** | **0** |

★`readonly` 분리가 결정적이다. `dereferenceable(N)` 만 보고 보류하면 `Entity`(deref 1728)를
넘기는 호출 하나가 **그 구조체 전 필드를 보류**로 만들어 게이트가 무력화된다(10%).
마지막 행에서 포착이 살짝 내려간 것은 시프트 정렬이 **구제도 늘렸기** 때문이고, 그 대가로
`MapDef.fountains[team]` 처럼 **gep 로 도달하는 base** 를 제대로 읽게 됐다(§4 ③ 참조).

## 4. 한계 (범위 명시)
1. **콜리 안의 접근은 안 보인다.** `dereferenceable` 로 보류한다 — 범위 표기가 없는 호출은 놓친다.
2. **레지스터 전달 인자**(`i24 MainObjective`)의 필드 읽기는 `load` 가 아니라 `and`/`lshr` 다.
   부재로 빠져 보류된다. 이 축(`dir`)으로는 **표기 불가**이고, 별도 값(예 `reg`)이 필요하다.
3. 명세는 오프셋을 **그 base 기준 상대값**으로 적고 IR 추적은 **루트 기준 절대값**이다.
   포인터 load 로 도달하는 base(`Effect`·`Entity`)는 둘이 같지만 gep 로 도달하는 base
   (`MapDef.fountains[team]`)는 상수만큼 밀려 있다 — m12.ll:34916 의 `rx`(+0x10)는 실제로
   `%30+16`(=루트 기준 28032)이다. 시프트 정렬로 맞추지만 **형제 행이 2개 이상 있어야** 맞춘다.
4. **형제 행이 없는 base**(그 함수에 그 구조체 필드를 한 줄만 적은 경우)는 동정이 안 돼 판정하지 못한다.
   실측 보류 91행 중 절반 이상이 이것이다(`GameSetting+0x12f8 tick_per_second` 류).
5. 담당 IR 범위 밖(aux 클로저·비인라인 헬퍼)의 접근은 안 본다.

사용: `python -X utf8 gate.py [specidx ...]`  /  `specgate` 진입점 = `check_spec(sp)`
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"
if MIG not in sys.path:
    sys.path.insert(0, MIG)

GEP_B = re.compile(r"^\s*(%[\w.$-]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr ([%@][\w.$-]+), i64 (-?\d+)")
GEP_V = re.compile(r"^\s*(%[\w.$-]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr ([%@][\w.$-]+), i64 (%[\w.$-]+)")
# ⚠타입 인덱싱 gep 의 타입은 쉼표를 품는다(`{ i64, i64 }`) — `[^,]+` 로 자르면 안 된다.
GEP_ANY = re.compile(r"^\s*(%[\w.$-]+) = getelementptr\b.*?, ptr ([%@][\w.$-]+),")
LOAD = re.compile(r"^\s*(%[\w.$-]+) = (?:tail )?load ([^,]+), ptr ([%@][\w.$-]+)")
STORE = re.compile(r"^\s*store ([^,]+?) [^,]*, ptr ([%@][\w.$-]+)")
MEMCPY = re.compile(r"@llvm\.memcpy\.[^(]*\(ptr[^%@]*([%@][\w.$-]+),\s*ptr[^%@]*([%@][\w.$-]+),\s*i64 (\d+)")
MEMSET = re.compile(r"@llvm\.memset\.[^(]*\(ptr[^%@]*([%@][\w.$-]+),\s*i8[^,]*,\s*i64 (\d+)")
PHI_P = re.compile(r"^\s*(%[\w.$-]+) = phi ptr (.*)$")
PHI_IN = re.compile(r"\[\s*([%@][\w.$-]+)\s*,")
SEL_P = re.compile(r"^\s*(%[\w.$-]+) = select i1 [^,]+, ptr ([%@][\w.$-]+), ptr ([%@][\w.$-]+)")
PHI_I = re.compile(r"^\s*(%[\w.$-]+) = phi i64 (.*)$")
CONST_IN = re.compile(r"\[\s*(-?\d+)\s*,")
SEL_I = re.compile(r"^\s*(%[\w.$-]+) = select i1 [^,]+, i64 (-?\d+), i64 (-?\d+)")
ARG = re.compile(r"^%\d+$")
CALL = re.compile(r"^\s*(?:%[\w.$-]+ = )?(?:tail |musttail |notail )?(?:call|invoke)\b")
# ★호출 인자: `ptr <속성들> %64`. 속성에 `readonly` 가 있으면 **콜리는 읽기만 한다** —
#   이게 이 게이트의 검출력을 살린다. `dereferenceable(N)` 만 보고 무조건 보류하면
#   `Entity`(deref 1728) 를 넘기는 순간 그 구조체 전 필드가 보류가 돼 게이트가 무력해진다
#   (변이 시험 실측 = docstring §3 표: 안 가르면 10.0%, 가르면 34.6% → 최종 53.1%).
#   `[^%@]` 로 끊어 함수 이름(`@foo`)·다른 인자를 넘어가지 않게 한다.
ARGPTR = re.compile(r"\bptr([^%@]*?)(%[\w.$-]+)")
DEREFN = re.compile(r"dereferenceable\((\d+)\)")

_W = {"i1": 1, "i8": 1, "i16": 2, "i24": 3, "i32": 4, "i64": 8, "i128": 16,
      "float": 4, "double": 8, "ptr": 8}


def width(ty):
    u"""한 번의 load/store 가 덮는 바이트 수. 모르면 1(=구제를 과하게 하지 않는다)."""
    t = (ty or "").strip()
    if t in _W:
        return _W[t]
    m = re.match(r"^<?\s*\{?\s*(.*?)\s*\}?\s*>?$", t)
    if m and "," in m.group(1):
        return sum(_W.get(p.strip(), 1) for p in m.group(1).split(","))
    m = re.match(r"^\[(\d+) x (\S+)\]$", t)
    if m:
        return int(m.group(1)) * _W.get(m.group(2), 1)
    return 1


_src_cache = {}


def load_ir(f):
    if f not in _src_cache:
        _src_cache[f] = io.open(os.path.join(IRDIR, f), encoding="utf-8",
                                errors="replace").read().split("\n")
    return _src_cache[f]


_scan_cache = {}


def scan(f, a, b):
    u"""(캡슐 래퍼 — 변이 시험이 같은 범위를 수백 번 부른다)"""
    if (f, a, b) not in _scan_cache:
        _scan_cache[(f, a, b)] = _scan(f, a, b)
    return _scan_cache[(f, a, b)]


def _scan(f, a, b):
    u"""담당 IR 범위를 훑어 **(SSA 루트, 오프셋) 단위 접근표**를 만든다.

    돌려주는 것 — 전부 `(root, off)` 를 키로 하는 구간 목록/집합:
      reads[(root,off)]  = [폭]      load · memcpy 원본
      writes[(root,off)] = [폭]      store · memcpy 목적지 · memset
      held[(root,off)]   = [폭]      호출 인자로 주소가 나간 구간(방향 미상 → 보류)
      touch              = {(root,off)}  gep 로 주소가 실제로 만들어진 자리
    """
    src = load_ir(f)
    lo, hi = a - 1, min(b, len(src))
    off = {}                       # ssa -> set[(root, int)] | None

    def res(p):
        if p in off:
            return off[p] or set()
        return {(p, 0)} if ARG.match(p) or p.startswith("@") else {(p, 0)}

    # ① 정수 phi/select 상수집합 — 가변 인덱스 gep 용(`target_bush_v30` 의 라인별 타워 배열)
    ioff = {}
    for k in range(lo, hi):
        ln = src[k]
        m = PHI_I.match(ln)
        if m:
            c = {int(x) for x in CONST_IN.findall(m.group(2))}
            if c:
                ioff[m.group(1)] = c
            continue
        m = SEL_I.match(ln)
        if m:
            ioff[m.group(1)] = {int(m.group(2)), int(m.group(3))}

    def prop():
        ch = False
        for k in range(lo, hi):
            ln = src[k]
            m = GEP_B.match(ln)
            if m:
                n = int(m.group(3))
                v = {(r, o + n) for (r, o) in res(m.group(2))}
            else:
                m = GEP_V.match(ln)
                if m:
                    base, iv = res(m.group(2)), ioff.get(m.group(3))
                    v = {(r, o + y) for (r, o) in base for y in iv} if iv else set(base)
                else:
                    m = GEP_ANY.match(ln)
                    if m:
                        v = set(res(m.group(2)))   # 타입/가변 인덱싱: 베이스 오프셋 유지
                    else:
                        m = PHI_P.match(ln)
                        if m:
                            v = set()
                            for x in PHI_IN.findall(m.group(2)):
                                v |= res(x)
                        else:
                            m = SEL_P.match(ln)
                            if not m:
                                continue
                            v = res(m.group(2)) | res(m.group(3))
            if v and off.get(m.group(1)) != v:
                off[m.group(1)] = v
                ch = True
        return ch

    for _ in range(8):             # phi 고정점 (합집합 — 「전부 같을 때만」이면 합류 필드를 놓친다)
        if not prop():
            break

    reads, writes, held_r, held_rw, touch = {}, {}, {}, {}, set()

    def put(d, keys, w):
        for kk in keys:
            d.setdefault(kk, []).append(w)
            touch.add(kk)

    for s in off.values():
        if s:
            touch |= s

    for k in range(lo, hi):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = MEMCPY.search(ln)
        if m:
            n = int(m.group(3))
            put(writes, res(m.group(1)), n)
            put(reads, res(m.group(2)), n)
            continue
        m = MEMSET.search(ln)
        if m:
            put(writes, res(m.group(1)), int(m.group(2)))
            continue
        m = LOAD.match(ln)
        if m:
            put(reads, res(m.group(3)), width(m.group(2)))
            continue
        m = STORE.match(ln)
        if m:
            put(writes, res(m.group(2)), width(m.group(1)))
            continue
        if CALL.match(ln):
            # ★`dereferenceable(N)` = 콜리의 사정거리. `readonly` 면 **읽기 전용 보류**로 가른다.
            for attrs, nm in ARGPTR.findall(ln):
                dn = DEREFN.search(attrs)
                n = int(dn.group(1)) if dn else 1
                put(held_r if "readonly" in attrs else held_rw, res(nm), n)
    return reads, writes, held_r, held_rw, touch


_cov_cache = {}


def _index(tbl):
    u"""`{root: [(base, width)]}` — `covers` 를 선형탐색에서 루트 버킷으로 낮춘다."""
    k = id(tbl)
    if k not in _cov_cache:
        d = {}
        for (r, bo), ws in tbl.items():
            d.setdefault(r, []).append((bo, max(ws)))
        _cov_cache[k] = d
    return _cov_cache[k]


def covers(tbl, root, o):
    u"""`tbl[(root,*)]` 의 접근 구간이 오프셋 `o` 를 덮는가. ★폭은 **구제 전용**이다 —
    `memcpy 24B` 로 통째 쓰는 sret 슬롯의 `+0x8`(specs[0] mem[27]) 이나
    `store i64` 한 방에 들어가는 하위 필드가 「없음」으로 오탐되는 것을 막는다."""
    d = _index(tbl)
    buckets = [d.get(root, ())] if root is not None else d.values()
    for lst in buckets:
        for (base, w) in lst:
            if base <= o < base + max(w, 1):
                return True
    return False


# ★오프셋 칸이 **순수 16진수**일 때만 구조체 바이트 오프셋이다.
#   `0x860[len]` 은 「그 포인터가 가리키는 버퍼의 len 번째」라는 뜻이라 바이트 오프셋이 아니다
#   (specs[12] mem[13] — 통합 1판이 이걸 0x860 으로 읽어 오탐을 냈다. IR m13.ll:29646
#    `%97 = load ptr, ptr %96`(=+0x860) → `%98 = gep {..}, ptr %97, i64 %88` → `memcpy(%98,..,184)`).
PUREOFF = re.compile(r"^\s*0x([0-9a-fA-F]+)\s*$")



def off_of(m):
    u"""명세 행의 오프셋 칸 → 정수(순수 16진수일 때만)."""
    mm = PUREOFF.match(str(m.get("offset") or ""))
    return int(mm.group(1), 16) if mm else None


def base_anchors_for(sp, j, base, reads, writes, held_r, held_rw):
    u"""★**루트 동정(同定)** — `{(root, shift)}`. 그 base 가 이 함수에서
    어느 SSA 루트의 어느 자리로 나타나는가.

    명세는 오프셋을 **그 base 기준 상대값**으로 적는데 IR 추적은 **루트 기준 절대값**이다.
    포인터 load 로 도달하는 base(`Effect`·`Entity`)는 둘이 같지만, gep 로 도달하는 base
    (`MapDef.fountains[team]`)는 **상수만큼 밀려 있다** — m12.ll:34916 에서 `rx`(+0x10)는
    실제로 `%30+16`(=루트 기준 28032) 이다.

    시프트는 **같은 base 의 다른 행 2개 이상**이 그 자리에서 주장대로 확인될 때만 인정한다
    (시프트 0 은 1개로 충분). 루트당 **가장 많이 들어맞는 시프트 하나** + 시프트 0 만 남긴다
    (변이 시험 실측: 자격 시프트 전부 220/448 · 최선+0 **238/448**).
    ⚠이 동정은 **구제와 증인 자격 부여에만** 쓴다 — 동정 자체로 명세를 기각하지 않는다."""
    mem = sp.get("mem") or []
    sb = []
    for j2, m2 in enumerate(mem):
        if j2 == j or m2.get("base") != base:
            continue
        d2, o2 = (m2.get("dir") or "").strip(), off_of(m2)
        if d2 in ("r", "w") and o2 is not None:
            sb.append((d2, o2))
    if not sb:
        return set()

    def ok_on(root, ao, d):
        return covers(reads if d == "r" else writes, root, ao)

    allkeys = set(reads) | set(writes) | set(held_r) | set(held_rw)
    cands = {}
    for (r, bo) in allkeys:
        c = cands.setdefault(r, set())
        c.add(0)
        for (_d2, o2) in sb:
            c.add(bo - o2)
    best = {}
    for r, ss in cands.items():
        for sh in ss:
            n = sum(1 for (d2, o2) in sb if ok_on(r, o2 + sh, d2))
            if n < (1 if sh == 0 else 2):
                continue
            cur = best.setdefault(r, {})
            if sh == 0:
                cur[0] = n
            if n > cur.get("top", (0, 0))[0]:
                cur["top"] = (n, sh)
    res = set()
    for r, cur in best.items():
        if 0 in cur:
            res.add((r, 0))
        if "top" in cur:
            res.add((r, cur["top"][1]))
    return res


def check_spec(sp):
    u"""★`specgate G14` 진입점. `[(mem 인덱스, 사유, 상세)]` — **방향이 IR 과 모순인 행만.**

    ⛔부재(그 오프셋이 IR 에 안 보임)는 결함이 아니다 — 콜리 안 접근·레지스터 승격·상수 접힘.
    ⛔`dir` 이 `r`/`w` 가 아닌 행, 오프셋이 순수 16진수가 아닌 행은 대상이 아니다.
    """
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    try:
        reads, writes, held_r, held_rw, touch = scan(f, a, b)
    except Exception:
        return []
    mem = sp.get("mem") or []

    _bc = {}

    def base_anchors(j, base):
        if (j, base) not in _bc:
            _bc[(j, base)] = base_anchors_for(sp, j, base, reads, writes, held_r, held_rw)
        return _bc[(j, base)]

    def ok_on(root, ao, d):
        u"""루트 `root` 의 **절대** 오프셋 `ao` 가 방향 `d` 로 관측됐는가."""
        return covers(reads if d == "r" else writes, root, ao)

    out = []
    for j, m in enumerate(mem):
        d = (m.get("dir") or "").strip()
        o = off_of(m)
        if d not in ("r", "w") or o is None:
            continue
        base = m.get("base")
        anc = base_anchors(j, base)
        if not anc:
            continue                    # 그 base 를 이 범위에서 짚지 못했다 → 보류
        if any(ok_on(r, o + sh, d) for (r, sh) in anc):
            continue                                # 주장대로 관측 → OK
        if any(covers(held_rw, r, o + sh) for (r, sh) in anc):
            continue                # &mut 로 콜리에 넘어갔다 — 읽든 쓰든 콜리가 한다 → 보류
        if d == "r" and any(covers(held_r, r, o + sh) for (r, sh) in anc):
            continue                # readonly 로 넘어갔다 — 콜리가 **읽는다** → r 주장 구제
        # 반대 방향으로만 쓰는 증인을 찾는다. 증인은 **그 base 로 동정된 자리**여야 한다
        # — 안 그러면 클로저 환경 구조체의 같은 오프셋이 반증으로 둔갑한다(1판 오탐 실례).
        opp = "w" if d == "r" else "r"
        witness = None
        for (r, sh) in anc:
            if covers(reads if opp == "r" else writes, r, o + sh):
                witness = (r, sh)
                break
        if witness is None:
            continue
        out.append((j, u"dir=%s 인데 그 자리는 %s 로만 쓰인다(base 동정 루트 %s shift %+d)"
                    % (d, opp, witness[0], witness[1]),
                    u"%s %s +%s" % (base, m.get("name"), m.get("offset"))))
    return out


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    tot = bad = hold = skip = 0
    why = {}
    for i in idxs:
        sp = D["specs"][i]
        ir = sp.get("ir") or {}
        if not ir.get("file"):
            continue
        reads, writes, held_r, held_rw, touch = scan(ir["file"], ir["frm"], ir["to"])
        rows = check_spec(sp)
        print(u"\n===== specs[%d] %s (%s:%d~%d) · 접근 pair %d"
              % (i, sp["name"], ir["file"], ir["frm"], ir["to"], len(touch)))
        for (j, why, det) in rows:
            print(u"  ★mem[%-2d] %s\n           %s" % (j, det, why))
        for j, m in enumerate(sp.get("mem") or []):
            d = (m.get("dir") or "").strip()
            mm = PUREOFF.match(str(m.get("offset") or ""))
            if d not in ("r", "w") or not mm:
                skip += 1
                continue
            tot += 1
            o = int(mm.group(1), 16)
            # ★보류 집계도 **판정과 같은 경로**로 센다(base 동정 기준).
            #   접은 표로 세면 실제보다 적게 나와 「거의 다 확인됐다」는 착시를 준다(33 vs 91).
            anc = base_anchors_for(sp, j, m.get("base"), reads, writes, held_r, held_rw)
            if not anc:
                hold += 1
                why[u"base 미동정"] = why.get(u"base 미동정", 0) + 1
            elif not any(covers(reads if d == "r" else writes, r, o + sh) for (r, sh) in anc):
                hold += 1
                k = (u"held_rw(&mut 전달)" if any(covers(held_rw, r, o + sh) for (r, sh) in anc)
                     else u"held_r(readonly 전달)" if d == "r" and any(covers(held_r, r, o + sh) for (r, sh) in anc)
                     else u"증인 없음(그 자리 관측 없음)")
                why[k] = why.get(k, 0) + 1
        bad += len(rows)
    print(u"\n---- 검사 %d행 · **불일치 %d** · 보류 %d · 대상외 %d" % (tot, bad, hold, skip))
    for k in sorted(why, key=lambda x: -why[x]):
        print(u"     보류 사유 · %-24s %d" % (k, why[k]))


if __name__ == "__main__":
    main()
