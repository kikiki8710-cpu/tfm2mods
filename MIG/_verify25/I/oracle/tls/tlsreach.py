"""next_plan 에서 도달 가능한 TLS 전역(thread_local) 작성자를 IR 호출 그래프로 전이 탐색한다.
- _gaibc + _gcbc 의 define 을 mmap 으로 색인(바이트 오프셋)
- 각 함수 본문에서 직접 호출 심볼과 thread_local 전역 참조를 수집
- next_plan 직접 콜리별로 도달 TLS 집합 + 경로 1개를 출력
"""
import io, os, re, sys, glob, mmap, json, pickle, time
SC = os.path.dirname(os.path.abspath(__file__))
FILES = sorted(glob.glob(r"C:\tfm2mods\_gaibc\m*.ll")) + sorted(glob.glob(r"C:\tfm2mods\_gcbc\g*.ll"))
DEF = re.compile(rb"\ndefine [^\n]*?@([A-Za-z0-9_$.]+)\(")
TLSG = re.compile(rb"\n(@[A-Za-z0-9_$.]+) = [^\n]*thread_local[^\n]*")
CALL = re.compile(rb"(?:call|invoke)[^\n]*?@([A-Za-z0-9_$.]+)\(")
REF = re.compile(rb"@([A-Za-z0-9_$.]+)")

IDX = os.path.join(SC, "tlsidx.pkl")

def build():
    idx = {}      # sym -> (fileno, start, end)
    tls = {}      # global name -> (file, line text)
    for fi, f in enumerate(FILES):
        t0 = time.time()
        with io.open(f, "rb") as fh:
            mm = mmap.mmap(fh.fileno(), 0, access=mmap.ACCESS_READ)
            for m in TLSG.finditer(mm):
                tls[m.group(1).decode()] = (os.path.basename(f), m.group(0)[1:160].decode(errors="replace"))
            for m in DEF.finditer(mm):
                s = m.start() + 1
                e = mm.find(b"\n}\n", m.end())
                if e == -1: e = len(mm)
                sym = m.group(1).decode()
                if sym not in idx:
                    idx[sym] = (fi, s, e + 2)
            mm.close()
        print("indexed", os.path.basename(f), "%.1fs" % (time.time() - t0), len(idx), file=sys.stderr)
    pickle.dump((idx, tls), open(IDX, "wb"))
    return idx, tls

if os.path.exists(IDX):
    idx, tls = pickle.load(open(IDX, "rb"))
else:
    idx, tls = build()
print("defines", len(idx), "tls globals", len(tls))
json.dump(tls, io.open(os.path.join(SC, "tls_globals.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)

_mm = {}
def body(sym):
    fi, s, e = idx[sym]
    if fi not in _mm:
        fh = io.open(FILES[fi], "rb")
        _mm[fi] = mmap.mmap(fh.fileno(), 0, access=mmap.ACCESS_READ)
    return _mm[fi][s:e]

def interesting(sym):
    return ("7game_ai" in sym) or ("9game_core" in sym)

tlsnames = set(k[1:] for k in tls)  # without '@'
cache = {}
def info(sym):
    if sym in cache: return cache[sym]
    b = body(sym)
    calls = []
    seen = set()
    for m in CALL.finditer(b):
        c = m.group(1).decode()
        if c.startswith("llvm.") or c in seen: continue
        seen.add(c); calls.append(c)
    refs = set(m.group(1).decode() for m in REF.finditer(b)) & tlsnames
    cache[sym] = (calls, refs)
    return cache[sym]

ROOT = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan9next_plan"
sys.setrecursionlimit(10000)
# 전이 도달 TLS: memo with DFS (cycle-safe: in-progress -> empty contribution, then fixpoint via a couple of passes)
reach = {}
onstack = set()
def dfs(sym, depth=0):
    if sym in reach: return reach[sym]
    if sym in onstack or depth > 40: return {}
    if sym not in idx:
        return {}
    onstack.add(sym)
    calls, refs = info(sym)
    out = {r: [sym] for r in refs}
    for c in calls:
        if not interesting(c) and "thread" not in c and "local" not in c: continue
        sub = dfs(c, depth + 1)
        for r, path in sub.items():
            if r not in out:
                out[r] = [sym] + path
    onstack.discard(sym)
    reach[sym] = out
    return out

rcalls, rrefs = info(ROOT)
print("\n== next_plan 직접 참조 TLS:", sorted(rrefs))
print("== next_plan 직접 호출", len(rcalls))
res = {}
for c in rcalls:
    d = dfs(c)
    res[c] = d
    tag = "(정의 없음)" if c not in idx else ""
    print("\n--", c[:150], tag)
    for r, path in sorted(d.items()):
        print("   TLS", r, tls["@" + r][0])
        print("      경로:", " -> ".join(p[:90] for p in path))
json.dump({k: {r: p for r, p in v.items()} for k, v in res.items()}, io.open(os.path.join(SC, "tlsreach.json"), "w", encoding="utf-8"), indent=1)
