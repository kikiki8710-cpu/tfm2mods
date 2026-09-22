# -*- coding: utf-8 -*-
"""tlsreach.py — 루트 define 에서 직접 호출 콜리를 순서대로 열거하고, 각 콜리가 전이적으로 닿는 TLS 전역을 BFS 로 센다.
사용: python -X utf8 tlsreach.py <root symbol substring> [--depth N]
인덱스: _gaibc 는 MIG\_next\reach\defidx.json, _gcbc 는 직접 스캔(캐시 gcidx.json).
"""
import io, os, re, sys, json, glob, collections

HERE = os.path.dirname(os.path.abspath(__file__))
GA = r"C:\tfm2mods\_gaibc"
GC = r"C:\tfm2mods\_gcbc"
DEFRE = re.compile(r"^define[^@]*@([A-Za-z0-9_$.]+)\(")
CALLRE = re.compile(r"\b(?:call|invoke)\b[^@]*@([A-Za-z0-9_$.]+)\(")
SYMRE = re.compile(r"@([A-Za-z0-9_$.]+)")

_FILES = {}
def lines_of(path):
    if path not in _FILES:
        _FILES[path] = io.open(path, encoding="utf-8", errors="replace").read().split("\n")
    return _FILES[path]

def build_gcidx():
    p = os.path.join(HERE, "gcidx.json")
    if os.path.exists(p):
        return json.load(io.open(p, encoding="utf-8"))
    idx = {}
    for f in sorted(glob.glob(os.path.join(GC, "g*.ll"))):
        mod = os.path.basename(f)
        cur = None
        with io.open(f, encoding="utf-8", errors="replace") as fh:
            for i, l in enumerate(fh, 1):
                if cur is None:
                    if l.startswith("define"):
                        m = DEFRE.match(l)
                        if m:
                            cur = (m.group(1), i)
                elif l.startswith("}"):
                    idx[cur[0]] = [mod, cur[1], i]
                    cur = None
    io.open(p, "w", encoding="utf-8").write(json.dumps(idx))
    return idx

def tlsname_of(sym):
    nm = None
    for mm in re.finditer(r"(?<![0-9])(?=(\d+)([A-Za-z_][A-Za-z0-9_]*))", sym):
        n, w = int(mm.group(1)), mm.group(2)
        if 3 <= n <= len(w) and w[:n].isupper() and w[0] != "_" and "_" in w[:n]:
            nm = w[:n]
    return nm

def build_tls():
    """TLS 전역 심볼 + 그 이름 토큰을 담은 모든 전역(@anon 상수 · call_once 등) → TLS 이름."""
    p = os.path.join(HERE, "tlsset2.json")
    if os.path.exists(p):
        return json.load(io.open(p, encoding="utf-8"))
    tls = {}
    names = set()
    for f in sorted(glob.glob(os.path.join(GA, "m*.ll"))) + sorted(glob.glob(os.path.join(GC, "g*.ll"))):
        with io.open(f, encoding="utf-8", errors="replace") as fh:
            for l in fh:
                if l.startswith("@") and "thread_local" in l:
                    sym = l.split(" = ")[0].strip("@").strip('"')
                    nm = tlsname_of(sym)
                    if nm:
                        names.add(nm); tls[sym] = nm
    # 2차: 이름 토큰을 담은 전역 상수(@anon…)도 TLS 참조로 본다
    for f in sorted(glob.glob(os.path.join(GA, "m*.ll"))) + sorted(glob.glob(os.path.join(GC, "g*.ll"))):
        with io.open(f, encoding="utf-8", errors="replace") as fh:
            for l in fh:
                if l.startswith("@anon.") and " constant " in l:
                    for nm in names:
                        if nm in l:
                            sym = l.split(" = ")[0].strip("@").strip('"')
                            tls[sym] = nm
                            break
    io.open(p, "w", encoding="utf-8").write(json.dumps({"tls": tls, "names": sorted(names)}))
    return {"tls": tls, "names": sorted(names)}

def main():
    root_sub = sys.argv[1]
    depth = 8
    if "--depth" in sys.argv:
        depth = int(sys.argv[sys.argv.index("--depth") + 1])
    ga = json.load(io.open(os.path.join(r"C:\tfm2mods\MIG\_next\reach", "defidx.json"), encoding="utf-8"))
    gc = build_gcidx()
    T = build_tls()
    tls = T["tls"]; names = T["names"]
    tlsnames = dict(tls)
    namere = re.compile(r"\d+(" + "|".join(sorted(names, key=len, reverse=True)) + r")")
    def locate(sym):
        if sym in ga:
            f, a, b = ga[sym]; return os.path.join(GA, f), a, b
        if sym in gc:
            f, a, b = gc[sym]; return os.path.join(GC, f), a, b
        return None
    roots = [s for s in list(ga) + list(gc) if root_sub in s]
    if len(roots) != 1:
        print("root candidates:", len(roots)); [print("  ", r) for r in roots[:20]]; return
    root = roots[0]
    print("ROOT", root)
    def body(sym):
        loc = locate(sym)
        if not loc: return None
        L = lines_of(loc[0]); return L[loc[1]-1:loc[2]]
    def scan(sym):
        b = body(sym)
        if b is None: return None, None, None
        callees = []; tlsrefs = []; vt = 0
        for i, l in enumerate(b):
            for m in CALLRE.finditer(l):
                c = m.group(1)
                if c.startswith("llvm."): continue
                callees.append((c, i))
            for m in SYMRE.finditer(l):
                g = m.group(1)
                if g in tls:
                    tlsrefs.append((g, i))
                else:
                    mm = namere.search(g)
                    if mm:
                        tlsnames[g] = mm.group(1); tlsrefs.append((g, i))
            if re.search(r"\b(?:call|invoke)\b[^@]*%\d+\(", l) and "@" not in l.split("(")[0]:
                vt += 1
        return callees, tlsrefs, vt
    memo = {}
    def reach(sym, d, path):
        """전이적 TLS 집합 + 첫 경로."""
        if sym in memo: return memo[sym]
        memo[sym] = {}
        callees, tlsrefs, vt = scan(sym)
        out = {}
        if callees is None:
            memo[sym] = out; return out
        for t, i in tlsrefs:
            out.setdefault(tlsnames[t], path + [sym])
        if d < depth:
            seen = set()
            for c, i in callees:
                if c in seen: continue
                seen.add(c)
                sub = reach(c, d + 1, path + [sym])
                for k, v in sub.items():
                    out.setdefault(k, v)
        memo[sym] = out
        return out
    callees, tlsrefs, vt = scan(root)
    print("direct TLS refs in root:", [(tlsnames[t], i) for t, i in tlsrefs])
    print("indirect(vtable/fnptr) call sites in root:", vt)
    print("\n# direct callees in body order (first occurrence) -> transitive TLS")
    seen = set()
    order = 0
    for c, i in callees:
        if c in seen: continue
        seen.add(c); order += 1
        loc = locate(c)
        where = ("%s:%d" % (os.path.basename(loc[0]), loc[1])) if loc else "(no define)"
        r = reach(c, 1, [root])
        short = c
        print("%2d. +L%-5d %s\n      %s" % (order, i, short[:150], where))
        for k, v in sorted(r.items()):
            print("      TLS %-24s via %s" % (k, " > ".join(x[-70:] for x in v[1:])))

if __name__ == "__main__":
    main()
