# -*- coding: utf-8 -*-
u"""`mem[].dir`(읽기/쓰기 방향) 기계 대조 — **§4-b 무검사 축 451행의 첫 계측기.** (7차 배치C 신설)

## 무엇을 하나
명세가 「이 베이스의 +0xNN 을 **읽는다(r)/쓴다(w)**」고 적은 것을, IR 범위 안의 실제 `load`/`store`
집합과 대조한다. 판정은 **필요조건**이다:

- `dir=r` 인데 그 오프셋으로 가는 `load` 가 범위 안에 **하나도 없다** → 결함 후보
- `dir=w` 인데 그 오프셋으로 가는 `store`(또는 memcpy 목적지)가 **하나도 없다** → 결함 후보

## 포인터 오프셋 추적 모델
SSA 값 하나에 **가능한 오프셋 집합**을 붙인다(`None` = 미상).
- `gep i8, ptr %B, i64 N`  → {b+N | b ∈ off(B)}
- `gep <ty>, ptr %B, i64 %i`(가변 인덱스) → off(B) 를 **그대로** 물려준다(배열 원소 접근 — `player_champion` 류)
- `phi ptr [...]` / `select i1 .., ptr, ptr` → 들어오는 값들의 **합집합**(고정점까지 반복)
  ⚠**합집합이어야 한다.** 「전부 같을 때만」으로 만들면 `live_list.ptr` 처럼 두 갈래(epic/serpen)가
  합류한 뒤 load 되는 필드를 통째로 놓친다(첫 판이 그랬다).
- 함수 인자 포인터(`%0`..`%n`)는 오프셋 0

## ⚠한계 (범위 명시)
1. **베이스 구조체를 구분하지 않는다** — 오프셋 숫자만 본다. 서로 다른 구조체의 같은 오프셋이
   서로의 알리바이가 되어 **거짓 음성**(결함을 놓침)이 생긴다. 실측: specs[10] `MainObjective+0x0`
   이 `OperationData+0x0` 의 load 로 통과했다. ⟹ 다음 판은 **베이스별로 SSA 루트를 태그**해야 한다.
2. **레지스터 전달 인자는 원리적으로 안 보인다** — `i24 MainObjective` 처럼 스칼라 승격된 값의
   필드 「읽기」는 `load` 가 아니라 `and`/`icmp` 다. 이런 행은 `dir` 축이 아니라 **별도 값**(예 `reg`)
   이 필요하다.
3. **호출된 함수 안의 접근은 안 보인다** — `passive_plan(&mut plan)` 처럼 콜리에서 쓰는 필드.
   `dereferenceable(N)` 인자로는 잡히지만 오프셋은 안 남는다.
⟹ 거짓 양성은 위 2·3 유형에 몰리고, **그 유형을 걸러낸 나머지는 실오류 확률이 높다.**
"""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import srclinecheck as S

GEP = re.compile(r"^\s*(%[^\s,]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr (%[^\s,]+), i64 (-?\d+)")
GEPANY = re.compile(r"^\s*(%[^\s,]+) = getelementptr\b.*?, ptr (%[^\s,]+),")
LOAD = re.compile(r"^\s*(?:%[^\s,]+ = )?(?:tail )?load [^,]+, ptr (%[^\s,]+)")
STORE = re.compile(r"^\s*store .+?, ptr (%[^\s,]+)")
MEMCPY = re.compile(r"@llvm\.memcpy\.[^(]*\(ptr[^%]*(%[^\s,]+),\s*ptr[^%]*(%[^\s,]+),")
MEMSET = re.compile(r"@llvm\.memset\.[^(]*\(ptr[^%]*(%[^\s,]+),")
PHI = re.compile(r"^\s*(%[^\s,]+) = phi ptr (.*)$")
PHIIN = re.compile(r"\[\s*(%[^\s,\]]+),")
SEL = re.compile(r"^\s*(%[^\s,]+) = select i1 [^,]+, ptr (%[^\s,]+), ptr (%[^\s,]+)")
ARG = re.compile(r"^%\d+$")
# ★정수 phi/select 로 고른 **오프셋 상수 집합** — `target_bush_v30` 은 라인별 타워 배열 오프셋을
#   `phi i64 [416,..],[448,..],[384,..]` 로 고른 뒤 `gep i8, ptr %cache, i64 %16` 한다.
#   이걸 안 따라가면 `top/mid/bottom_tower(2)` 6행이 통째로 「load 없음」으로 오탐된다.
IPHI = re.compile(r"^\s*(%[^\s,]+) = phi i64 (.*)$")
ICONST = re.compile(r"\[\s*(-?\d+),")
ISEL = re.compile(r"^\s*(%[^\s,]+) = select i1 [^,]+, i64 (-?\d+), i64 (-?\d+)")
GEPV = re.compile(r"^\s*(%[^\s,]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr (%[^\s,]+), i64 (%[^\s,]+)")


def offsets(src, a, b):
    off = {}                       # ssa -> set[int] | None
    reads, writes = {}, {}
    lo, hi = a - 1, min(b, len(src))

    def res(p):
        if p in off:
            return off[p]
        return {0} if ARG.match(p) else None

    def add(d, s, k):
        if s:
            for o in s:
                d.setdefault(o, []).append(k + 1)

    ioff = {}
    for k in range(lo, hi):
        ln = src[k]
        m = IPHI.match(ln)
        if m:
            c = {int(x) for x in ICONST.findall(m.group(2))}
            if c:
                ioff[m.group(1)] = c
            continue
        m = ISEL.match(ln)
        if m:
            ioff[m.group(1)] = {int(m.group(2)), int(m.group(3))}

    for k in range(lo, hi):
        ln = src[k]
        m = GEP.match(ln)
        if m:
            bo = res(m.group(2))
            n = int(m.group(3))
            off[m.group(1)] = None if bo is None else {x + n for x in bo}
            continue
        m = GEPV.match(ln)
        if m:
            bo, iv = res(m.group(2)), ioff.get(m.group(3))
            off[m.group(1)] = ({x + y for x in bo for y in iv} if (bo and iv) else bo)
            continue
        m = GEPANY.match(ln)
        if m:
            off[m.group(1)] = res(m.group(2))

    for _ in range(6):                    # phi 고정점
        changed = False
        for k in range(lo, hi):
            ln = src[k]
            m = PHI.match(ln)
            if m:
                vals = [res(x) for x in PHIIN.findall(m.group(2))]
                dst = m.group(1)
            else:
                m = SEL.match(ln)
                if not m:
                    continue
                vals = [res(m.group(2)), res(m.group(3))]
                dst = m.group(1)
            u = set()
            for v in vals:
                if v:
                    u |= v
            if u and off.get(dst) != u:
                off[dst] = u
                changed = True
        if not changed:
            break

    for k in range(lo, hi):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = LOAD.match(ln)
        if m:
            add(reads, res(m.group(1)), k); continue
        m = STORE.match(ln)
        if m:
            add(writes, res(m.group(1)), k); continue
        m = MEMCPY.search(ln)
        if m:
            add(writes, res(m.group(1)), k); add(reads, res(m.group(2)), k); continue
        m = MEMSET.search(ln)
        if m:
            add(writes, res(m.group(1)), k)
    return reads, writes, off


OFFPAT = re.compile(r"0x([0-9a-fA-F]+)")


def check_spec(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    src, _meta = S.load(f)
    reads, writes, _ = offsets(src, a, b)
    out = []
    for j, m in enumerate(sp.get("mem") or []):
        d = (m.get("dir") or "").strip()
        mm = OFFPAT.search(str(m.get("offset") or ""))
        if not mm:
            continue
        o = int(mm.group(1), 16)
        if d == "r" and o not in reads:
            out.append((j, "r", o))
        elif d == "w" and o not in writes:
            out.append((j, "w", o))
    return out


if __name__ == "__main__":
    D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
    lo = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    hi = int(sys.argv[2]) if len(sys.argv) > 2 else 20
    tot = n = 0
    for i in range(lo, hi):
        sp = D["specs"][i]
        rows = check_spec(sp)
        n += len(sp.get("mem") or [])
        tot += len(rows)
        print(u"specs[%2d] %-46s mem %3d행 · 결함후보 %d"
              % (i, sp["name"], len(sp.get("mem") or []), len(rows)))
        for (j, d, o) in rows:
            mr = sp["mem"][j]
            print(u"    mem[%2d] %-34s +%-10s %-46s dir=%s 인데 %s 없음"
                  % (j, mr.get("base"), mr.get("offset"), mr.get("name"), d,
                     u"load" if d == "r" else u"store"))
    print(u"\n합계: mem %d행 중 결함후보 %d" % (n, tot))
