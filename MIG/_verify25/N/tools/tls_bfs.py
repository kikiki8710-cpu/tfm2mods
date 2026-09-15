# -*- coding: utf-8 -*-
"""cg.pkl 로 attack 에서 도달 가능한 함수 전체를 BFS(직접 call 만 · 간접 호출은 표시) — TLS 참조 함수와 경로를 낸다."""
import pickle, os, re, sys, collections
D = pickle.load(open(os.path.join(os.path.dirname(__file__), "cg.pkl"), "rb"))
G, TG = D["graph"], D["tls_globals"]
ROOT = "_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input6attack"
MAXD = int(sys.argv[1]) if len(sys.argv) > 1 else 64
def short(s):
    s = re.sub(r"^_R[A-Za-z0-9]*?(7game_ai|9game_core|4core|5alloc|3std|4rand|7bumpalo)", r"\1:", s)
    return s[:120]
par = {ROOT: None}; dist = {ROOT: 0}
q = collections.deque([ROOT])
missing = set()
while q:
    u = q.popleft()
    if dist[u] >= MAXD: continue
    if u not in G:
        missing.add(u); continue
    for v in G[u]["callees"]:
        if v not in dist:
            dist[v] = dist[u] + 1; par[v] = u; q.append(v)
print("reachable", len(dist), "missing defs(declare only)", len(missing))
def path(v):
    out = []
    while v: out.append(short(v)); v = par[v]
    return " <- ".join(out)
tls_hit = [(v, G[v]["tls"]) for v in dist if v in G and G[v]["tls"]]
print("--- TLS 참조 함수", len(tls_hit))
for v, t in sorted(tls_hit, key=lambda x: dist[x[0]]):
    print(f"d={dist[v]} {short(v)}  file={G[v]['file']}:{G[v]['line']}")
    for g in sorted(t): print("     TLS", g[:120], TG.get(g, ""))
    print("     path:", path(v))
ind = [(v, G[v]["indirect"]) for v in dist if v in G and G[v]["indirect"]]
print("--- 간접 호출 있는 함수", len(ind), "(간접 호출 수 합", sum(x[1] for x in ind), ")")
for v, n in sorted(ind, key=lambda x: dist[x[0]])[:40]:
    print(f"d={dist[v]} n={n} {short(v)}")
print("--- missing(선언만) 예:", [short(m) for m in list(missing)[:20]])
# 모든 TLS 전역 목록 중 이름에 CACHE/MEMO/CTX 포함
print("--- 전체 TLS 전역", len(TG))
for g in sorted(TG):
    if re.search(r"CACHE|MEMO|CTX|BEAMS|STANCE|MASKS", g): print("  ", g, TG[g])
