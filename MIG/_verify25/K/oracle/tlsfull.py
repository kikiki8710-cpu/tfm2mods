# -*- coding: utf-8 -*-
"""tlsfull.py — 깊이 제한 없는 전이적 TLS 도달 검사(tlsreach.py 의 memo-깊이 결함 교차검증).
전 corpus(_gaibc m*.ll + _gcbc g*.ll)를 1회 스트리밍해 define→(callees, tls refs) 그래프를 만들고
캐시(callgraph.json)한 뒤, 루트에서 SCC 없이 반복 fixpoint 로 전이 TLS 집합을 구한다."""
import io, os, re, sys, json, glob, collections
HERE = os.path.dirname(os.path.abspath(__file__))
GA = r"C:\tfm2mods\_gaibc"; GC = r"C:\tfm2mods\_gcbc"
CACHE = os.path.join(HERE, "callgraph.json")
DEFRE = re.compile(r"^define[^@]*@([A-Za-z0-9_$.]+)\(")
CALLRE = re.compile(r"\b(?:call|invoke)\b[^@\n]*@([A-Za-z0-9_$.]+)\(")
SYMRE = re.compile(r"@([A-Za-z0-9_$.]+)")
INDIRECT = re.compile(r"\b(?:call|invoke)\b[^@\n]*%\d+\(")

def build():
    files = sorted(glob.glob(os.path.join(GA, "m*.ll"))) + sorted(glob.glob(os.path.join(GC, "g*.ll")))
    tlsglob = {}; grefs = {}
    for f in files:
        with io.open(f, encoding="utf-8", errors="replace") as fh:
            for l in fh:
                if l.startswith("@"):
                    nm = l.split(" = ")[0].strip("@").strip('"')
                    if "thread_local" in l:
                        tlsglob[nm] = os.path.basename(f)
                    rs = [m.group(1) for m in SYMRE.finditer(l.split(" = ", 1)[1])] if " = " in l else []
                    rs = [r for r in rs if r != nm]
                    if rs: grefs[nm] = rs
    print("thread_local globals:", len(tlsglob), "globals with symbol refs:", len(grefs))
    graph = {}
    for f in files:
        mod = os.path.basename(f); cur = None
        with io.open(f, encoding="utf-8", errors="replace") as fh:
            for i, l in enumerate(fh, 1):
                if cur is None:
                    if l.startswith("define"):
                        m = DEFRE.match(l)
                        if m:
                            cur = [m.group(1), i, set(), set(), 0]
                else:
                    if l.startswith("}"):
                        graph[cur[0]] = [mod, cur[1], i, sorted(cur[2]), sorted(cur[3]), cur[4]]
                        cur = None; continue
                    for m in CALLRE.finditer(l):
                        c = m.group(1)
                        if not c.startswith("llvm."): cur[2].add(c)
                    if "@" in l:
                        for m in SYMRE.finditer(l):
                            g = m.group(1)
                            if g in tlsglob: cur[3].add(g)
                            elif g in grefs:
                                # 전역 상수(@anon fn-ptr 표 · LocalKey 등)를 거친 간접 참조: 그 상수가 가리키는 심볼을 콜리로 편입(전이)
                                st = list(grefs[g]); sn = set()
                                while st:
                                    r = st.pop()
                                    if r in sn: continue
                                    sn.add(r)
                                    if r in tlsglob: cur[3].add(r)
                                    elif r in grefs: st.extend(grefs[r])
                                    else: cur[2].add(r)
                    if INDIRECT.search(l) and "@" not in l.split("(")[0]:
                        cur[4] += 1
        print("scanned", mod, len(graph))
    json.dump({"tls": tlsglob, "graph": graph}, io.open(CACHE, "w", encoding="utf-8"))
    return tlsglob, graph

def load():
    if os.path.exists(CACHE):
        d = json.load(io.open(CACHE, encoding="utf-8")); return d["tls"], d["graph"]
    return build()

def tlsname(sym):
    nm = None
    for mm in re.finditer(r"(?<![0-9])(?=(\d+)([A-Za-z_][A-Za-z0-9_]*))", sym):
        n, w = int(mm.group(1)), mm.group(2)
        if 3 <= n <= len(w) and w[:n].isupper() and w[0] != "_" and "_" in w[:n]:
            nm = w[:n]
    return nm or sym[-40:]

def closure(graph, root):
    """루트에서 도달 가능한 모든 define 과, 각 define 의 전이 TLS 집합(fixpoint)."""
    reach = set(); st = [root]
    while st:
        s = st.pop()
        if s in reach or s not in graph: continue
        reach.add(s)
        st.extend(graph[s][3])
    tl = {s: set(graph[s][4]) for s in reach}
    changed = True
    while changed:
        changed = False
        for s in reach:
            for c in graph[s][3]:
                if c in tl and not tl[c] <= tl[s]:
                    tl[s] |= tl[c]; changed = True
    return reach, tl

def path_to(graph, src, tls, tl):
    """src 에서 tls 를 직접 참조하는 define 까지의 한 경로(BFS)."""
    prev = {src: None}; q = collections.deque([src])
    while q:
        s = q.popleft()
        if tls in graph[s][4]:
            p = [];
            while s is not None: p.append(s); s = prev[s]
            return p[::-1]
        for c in graph[s][3]:
            if c in tl and c not in prev and tls in tl[c]:
                prev[c] = s; q.append(c)
    return None

def main():
    tlsglob, graph = load()
    for root_sub in sys.argv[1:]:
        roots = [s for s in graph if root_sub in s]
        if len(roots) != 1:
            print("root candidates", len(roots), roots[:5]); continue
        root = roots[0]
        reach, tl = closure(graph, root)
        print("\nROOT", root, graph[root][0], graph[root][1], "reachable defines:", len(reach))
        print("direct TLS refs:", sorted(tlsname(x) for x in graph[root][4]), "indirect call sites:", graph[root][5])
        print("transitive TLS (전체, 깊이 무제한):", sorted(set(tlsname(x) for x in tl[root])))
        # 본문 순서의 직접 콜리(첫 등장) + 각각의 전이 TLS
        mod, a, b = graph[root][0], graph[root][1], graph[root][2]
        base = GA if mod.startswith("m") else GC
        L = io.open(os.path.join(base, mod), encoding="utf-8", errors="replace").read().split("\n")[a-1:b]
        seen = set(); n = 0
        for i, l in enumerate(L):
            for m in CALLRE.finditer(l):
                c = m.group(1)
                if c.startswith("llvm.") or c in seen: continue
                seen.add(c); n += 1
                t = sorted(set(tlsname(x) for x in tl.get(c, set())))
                loc = graph.get(c)
                print("%2d. L%-6d %-80s %s" % (n, a + i, c[-80:], ("%s:%d" % (loc[0], loc[1])) if loc else "(no define)"))
                if t:
                    print("      TLS:", ", ".join(t))
                    for x in sorted(tl.get(c, set()), key=tlsname):
                        p = path_to(graph, c, x, tl)
                        if p: print("        %-24s via %s" % (tlsname(x), " > ".join(y[-60:] for y in p)))
if __name__ == "__main__":
    main()
