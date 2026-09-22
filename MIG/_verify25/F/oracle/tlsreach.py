"""TLS 도달 그래프 v2: game_ai(_gaibc m*.ll) + game_core(_gcbc g*.ll).
TLS 접점 판정 = 본문이 참조하는 @심볼(직접 · `@anon.X` 상수가 가리키는 심볼까지 해석) 이름에
thread_local 전역의 키 이름(예 14DIE_TICK_CACHE)이 들어 있으면 그 키에 닿은 것으로 본다
(Rust LocalKey::with 는 `@anon.X = constant ptr @<KEY…call_once>` 를 거쳐 간접 호출하므로 threadlocal.address 가 본문에 없다).
python tlsreach.py [심볼]   # 캐시(tlsgraph2.pkl) 생성 후 루트의 직접 콜리별 BFS 보고
"""
import re, io, glob, os, pickle, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, "tlsgraph2.pkl")
SYM_RX = re.compile(r"@([A-Za-z0-9_.$]+)")
CALL_RX = re.compile(r"(?:call|invoke)[^@\n]*@([A-Za-z0-9_.$]+)\(")
DEF_RX = re.compile(r"^define [^\n]*@([A-Za-z0-9_.$]+)\(", re.M)
KEY_RX = re.compile(r"(\d+)([A-Z][A-Z0-9_]{3,})")

def build():
    files = sorted(glob.glob(r"C:\tfm2mods\_gaibc\m*.ll")) + sorted(glob.glob(r"C:\tfm2mods\_gcbc\g*.ll"))
    texts = {}
    tlskeys = set()
    anon = {}   # anon 상수 -> 가리키는 심볼들
    for f in files:
        t = io.open(f, encoding="utf-8", errors="ignore").read()
        texts[f] = t
        for m in re.finditer(r"^@([A-Za-z0-9_.$]+) = [^\n]*thread_local[^\n]*", t, re.M):
            for n, k in KEY_RX.findall(m.group(1)):
                n = int(n)
                if 4 <= n <= len(k):
                    tlskeys.add(k[:n])
        for m in re.finditer(r"^@(anon\.[A-Za-z0-9_.$]+) = [^\n]*constant[^\n]*", t, re.M):
            syms = set(SYM_RX.findall(m.group(0))) - {m.group(1)}
            if syms:
                anon[m.group(1)] = syms
    key_rx = re.compile(r"(\d+)(" + "|".join(sorted(tlskeys, key=len, reverse=True)) + r")")
    def keys_of(sym):
        out = set()
        for m in key_rx.finditer(sym):
            if int(m.group(1)) == len(m.group(2)):
                out.add(m.group(2))
        return out
    calls = {}; tls = {}
    for f in files:
        t = texts[f]
        for m in DEF_RX.finditer(t):
            sym = m.group(1); i = m.start()
            j = t.find("\n}\n", i)
            if j == -1: continue
            body = t[i:j]
            cs = set(CALL_RX.findall(body)); cs.discard(sym)
            ts = set()
            for s in set(SYM_RX.findall(body)):
                if s == sym: continue
                ts |= keys_of(s)
                if s.startswith("anon."):
                    for s2 in anon.get(s, ()):
                        ts |= keys_of(s2)
            calls.setdefault(sym, set()).update(cs)
            tls.setdefault(sym, set()).update(ts)
        print("scanned", os.path.basename(f), len(calls), file=sys.stderr)
    pickle.dump((calls, tls, sorted(tlskeys)), open(CACHE, "wb"))
    return calls, tls, sorted(tlskeys)

if os.path.exists(CACHE):
    calls, tls, tlskeys = pickle.load(open(CACHE, "rb"))
else:
    calls, tls, tlskeys = build()

ROOT = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan17action_candidates"
SKIP = ("llvm.", "panic", "drop_glue", "unwrap_failed", "4drop4Drop", "prof")
def reach(start, maxd=10):
    seen = {start: 0}; q = collections.deque([start]); hits = {}
    while q:
        s = q.popleft(); d = seen[s]
        for g in tls.get(s, ()):
            hits.setdefault(g, (d, s))
        if d >= maxd: continue
        for c in calls.get(s, ()):
            if c not in seen:
                seen[c] = d + 1; q.append(c)
    return hits, seen

def short(s):
    return re.sub(r"_R[A-Za-z0-9]*?(\d+)([a-z_]+)", "", s)[:0] or s[-90:]

if __name__ == "__main__":
    tgt = sys.argv[1] if len(sys.argv) > 1 else ROOT
    print("TLS keys:", len(tlskeys))
    print("root:", tgt[-80:], "direct callees:", len(calls.get(tgt, ())), "own tls:", tls.get(tgt))
    for c in sorted(calls.get(tgt, ())):
        if any(k in c for k in SKIP): continue
        hits, seen = reach(c)
        tag = "TLS" if hits else "-  "
        print(f"[{tag}] {c[-110:]}")
        for g, (d, via) in sorted(hits.items(), key=lambda kv: (kv[1][0], kv[0])):
            print(f"       d={d:<2} {g:<28} via {via[-90:]}")
