# -*- coding: utf-8 -*-
"""tlstree.py — 루트 심볼에서 game_ai IR(_gaibc) 호출 그래프를 BFS 로 따라가며
LocalKey::with(TLS) 접점을 함수별로 모은다. 산출: 콘솔 + json."""
import io, os, re, sys, json, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
CORP = r"C:\tfm2mods\_gaibc"
DEFIDX = r"C:\tfm2mods\MIG\_next\reach\defidx.json"
OUT = os.path.dirname(os.path.abspath(__file__))
idx = json.load(io.open(DEFIDX, encoding="utf-8"))
CALL = re.compile(rb'(?:call|invoke)[^@\n]*@([A-Za-z0-9_$.]+)\(')
_cache = {}
def lines(fn):
    if fn not in _cache:
        _cache[fn] = io.open(os.path.join(CORP, fn), "rb").read().split(b"\n")
    return _cache[fn]
def body(sym):
    e = idx.get(sym)
    if not e or e[2] is None: return None
    fn, a, b = e
    return lines(fn)[a-1:b]
def callees(sym):
    b = body(sym)
    if b is None: return None
    out = []
    for raw in b:
        m = CALL.search(raw)
        if m:
            c = m.group(1).decode()
            if c.startswith("llvm."): continue
            out.append(c)
    return out
def is_tls(c):
    return ("8LocalKey" in c and "4with" in c)
def tlsname(c):
    m = re.search(r'7RefCell(?:NtNt|Nt|INtNt|B[0-9a-zA-Z_]+_)?(?:Cs[0-9a-zA-Z_]+_)?(?:[0-9]+[a-z_]+)*?([0-9]+)([A-Za-z_]+)EE4with', c)
    return c
roots = sys.argv[1:]
maxd = 8
seen = {}
q = collections.deque((r, 0, "") for r in roots)
tls_hits = collections.defaultdict(set)
graph = {}
while q:
    sym, d, parent = q.popleft()
    if sym in seen: continue
    seen[sym] = d
    cs = callees(sym)
    if cs is None:
        graph[sym] = None; continue
    graph[sym] = sorted(set(cs))
    for c in cs:
        if is_tls(c):
            tls_hits[sym].add(c)
        if d < maxd and c not in seen:
            q.append((c, d+1, sym))
# 출력: TLS 접점이 있는 함수와 그 with 심볼
print("정의 있음 %d · 정의 없음 %d" % (sum(1 for v in graph.values() if v is not None), sum(1 for v in graph.values() if v is None)))
for sym in sorted(tls_hits, key=lambda s: seen[s]):
    print("\n[depth %d] %s" % (seen[sym], sym))
    for c in sorted(tls_hits[sym]):
        print("    ", c)
json.dump({"seen": seen, "graph": graph, "tls": {k: sorted(v) for k, v in tls_hits.items()}},
          io.open(os.path.join(OUT, "tlstree_%s.json" % roots[0][-40:]), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
