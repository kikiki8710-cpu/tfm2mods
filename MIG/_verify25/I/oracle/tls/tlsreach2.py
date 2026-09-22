"""tlsreach2 — tlsidx.pkl(색인) 로 next_plan 직접 콜리별 thread_local 전이 도달을 반복(BFS)으로 계산한다(재귀 X)."""
import io, os, re, sys, glob, mmap, json, pickle, time
SC = os.path.dirname(os.path.abspath(__file__))
FILES = sorted(glob.glob(r"C:\tfm2mods\_gaibc\m*.ll")) + sorted(glob.glob(r"C:\tfm2mods\_gcbc\g*.ll"))
CALL = re.compile(rb"(?:call|invoke)[^\n]*?@([A-Za-z0-9_$.]+)\(")
REF = re.compile(rb"@([A-Za-z0-9_$.]+)")
idx, tls = pickle.load(open(os.path.join(SC, "tlsidx.pkl"), "rb"))
tlsnames = set(k[1:] for k in tls)
anon = pickle.load(open(os.path.join(SC, "tlsanon.pkl"), "rb"))
ANON = re.compile(rb"@(anon\.[0-9a-f]+\.\d+)")
sys.stdout.reconfigure(encoding="utf-8")
print("defines", len(idx), "tls globals", len(tls), flush=True)
_mm = {}
def body(sym):
    fi, s, e = idx[sym]
    if fi not in _mm:
        fh = io.open(FILES[fi], "rb"); _mm[fi] = mmap.mmap(fh.fileno(), 0, access=mmap.ACCESS_READ)
    return _mm[fi][s:e]
def interesting(c):
    return ("7game_ai" in c) or ("9game_core" in c) or ("thread" in c and "local" in c)
cache = {}
def info(sym):
    if sym in cache: return cache[sym]
    b = body(sym)
    seen = set(); calls = []
    for m in CALL.finditer(b):
        c = m.group(1).decode()
        if c.startswith("llvm.") or c in seen: continue
        seen.add(c); calls.append(c)
    refs = set(m.group(1).decode() for m in REF.finditer(b)) & tlsnames
    fi = idx[sym][0]
    for m in ANON.finditer(b):
        f = anon.get((fi, "@" + m.group(1).decode()))
        if f and f not in seen: seen.add(f); calls.append(f)
    cache[sym] = (calls, refs)
    return cache[sym]

ROOT = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan9next_plan"
rcalls, rrefs = info(ROOT)
print("root direct TLS refs:", sorted(rrefs))
print("root direct calls:", len(rcalls), flush=True)
def bfs(start):
    parent = {start: None}; q = [start]; hit = {}
    while q:
        s = q.pop(0)
        if s not in idx: continue
        calls, refs = info(s)
        for r in refs:
            if r not in hit: hit[r] = s
        for c in calls:
            if not interesting(c) or c in parent: continue
            parent[c] = s; q.append(c)
    return parent, hit
def path(parent, s):
    out = []
    while s is not None: out.append(s); s = parent[s]
    return list(reversed(out))
res = {}
t0 = time.time()
for c in rcalls:
    parent, hit = bfs(c)
    res[c] = {r: path(parent, s) for r, s in hit.items()}
    print("\n--", c[:160], "" if c in idx else "(정의 없음: declare/간접)", "노드", len(parent), "%.0fs" % (time.time() - t0), flush=True)
    for r, pth in sorted(res[c].items()):
        print("   TLS", r, "  @", tls["@" + r][0])
        print("      경로:", " -> ".join(p[:100] for p in pth), flush=True)
json.dump(res, io.open(os.path.join(SC, "tlsreach.json"), "w", encoding="utf-8"), indent=1)
