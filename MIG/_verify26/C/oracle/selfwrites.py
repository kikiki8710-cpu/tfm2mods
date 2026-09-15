# -*- coding: utf-8 -*-
"""PassiveLinePlan::update (m04.ll 19038~24726) 에서 인자 포인터 파생 쓰기 전수.
 - %0 self / %2 rnd / %7 debug / %5 team_plan / %3 player / %4 data / %6 pos_score
 - store · memcpy/memset · call 에 파생 포인터를 넘기는 사이트 · load(읽기) 도 집계
출력: 오프셋별 store 목록(줄번호·값·루트 소스줄)
"""
import io, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
RAW = r"C:\tfm2mods\_gaibc\m04.ll"
ANN = r"C:\tfm2mods\MIG\_next\reach\d28800.ll"
A, B = 19038, 24726
raw = io.open(RAW, encoding="utf-8", errors="replace").read().split("\n")
ann = {}
for l in io.open(ANN, encoding="utf-8", errors="replace"):
    m = re.match(r"\s*(\d+)\|(.*?)(;L[^;]*)?\s*$", l.rstrip("\n"))
    if m: ann[int(m.group(1))] = (m.group(3) or "").strip()

ARGS = {"%0": "self", "%2": "rnd", "%7": "debug", "%5": "team_plan", "%3": "player", "%4": "data", "%6": "pos_score", "%1": "version"}
# derived: ssa -> (argname, offset or None)
der = {k: (v, 0) for k, v in ARGS.items()}
GEP_I8 = re.compile(r"^\s*(%[\w.]+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr (%[\w.]+), i64 (-?\d+)")
GEP_ANY = re.compile(r"^\s*(%[\w.]+) = getelementptr[^,]*, ptr (%[\w.]+)(.*)$")
STORE = re.compile(r"^\s*store (?:volatile )?(\w+) (.+?), ptr (%[\w.]+)")
LOAD = re.compile(r"^\s*(%[\w.]+) = load (\w+), ptr (%[\w.]+)")
CALL = re.compile(r"^\s*(?:(%[\w.]+) = )?(?:tail )?(?:call|invoke) [^@%]*(@[\w.$\"]+|%[\w.]+)\((.*)\)")
SEL = re.compile(r"^\s*(%[\w.]+) = select i1 (%[\w.]+), ptr (%[\w.]+), ptr (%[\w.]+)")
PHI = re.compile(r"^\s*(%[\w.]+) = phi ptr (.*)$")

stores = collections.defaultdict(list)
loads = collections.defaultdict(list)
calls = []
unresolved = []
for i in range(A, B + 1):
    l = raw[i - 1]
    m = GEP_I8.match(l)
    if m:
        d, base, off = m.group(1), m.group(2), int(m.group(3))
        if base in der and der[base][1] is not None:
            der[d] = (der[base][0], der[base][1] + off)
        elif base in der:
            der[d] = (der[base][0], None)
        continue
    m = GEP_ANY.match(l)
    if m:
        d, base = m.group(1), m.group(2)
        if base in der:
            der[d] = (der[base][0], None)
            unresolved.append((i, l.strip()[:160]))
        continue
    m = SEL.match(l)
    if m and (m.group(3) in der or m.group(4) in der):
        a = der.get(m.group(3)); b = der.get(m.group(4))
        der[m.group(1)] = (a or b)[0], None
        unresolved.append((i, l.strip()[:160]))
        continue
    m = PHI.match(l)
    if m:
        srcs = re.findall(r"\[ (%[\w.]+), ", m.group(2))
        hit = [s for s in srcs if s in der]
        if hit:
            offs = set(der[s][1] for s in hit)
            der[m.group(1)] = (der[hit[0]][0], offs.pop() if len(offs) == 1 and len(hit) == len(srcs) else None)
            if der[m.group(1)][1] is None: unresolved.append((i, l.strip()[:160]))
        continue
    m = STORE.match(l)
    if m:
        ty, val, p = m.groups()
        if p in der:
            stores[der[p]].append((i, ty, val.strip()[:60], ann.get(i, "")))
        continue
    m = LOAD.match(l)
    if m:
        d, ty, p = m.groups()
        if p in der:
            loads[der[p]].append((i, ty, ann.get(i, "")))
            # loaded pointer from self (e.g. Vec.ptr) → mark as heap-derived
            if ty == "ptr":
                der[d] = (der[p][0] + f".heap@{der[p][1]}", 0)
        continue
    m = CALL.match(l)
    if m:
        args = m.group(3)
        hits = [(a, der[a]) for a in re.findall(r"(%[\w.]+)", args) if a in der]
        if hits:
            calls.append((i, m.group(2)[:90], hits, ann.get(i, "")))

def key(k): return (k[0], -1 if k[1] is None else k[1])
print("=== STORES (파생 포인터 기준) ===")
for k in sorted(stores, key=key):
    name, off = k
    offs = "?" if off is None else hex(off)
    for (i, ty, val, ch) in stores[k]:
        print(f"{name}+{offs}\t{i}\tstore {ty} {val}\t{ch}")
print("\n=== CALLS with derived args ===")
for (i, fn, hits, ch) in calls:
    print(f"{i}\t{fn}\t{[(a, n, (hex(o) if o is not None else '?')) for a, (n, o) in hits]}\t{ch}")
print("\n=== LOADS summary (self/debug/team_plan/rnd) ===")
cnt = collections.Counter()
for k in sorted(loads, key=key):
    if k[0] in ("self", "debug", "team_plan", "rnd", "pos_score", "player", "data"):
        print(f"{k[0]}+{'?' if k[1] is None else hex(k[1])}\t{len(loads[k])}\t{[x[0] for x in loads[k]][:12]}")
print("\n=== unresolved gep/select/phi (수동 확인) ===")
for u in unresolved: print(u)
