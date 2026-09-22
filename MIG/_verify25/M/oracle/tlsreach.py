"""tlsreach.py <root-symbol-substring> [--depth N] — _gaibc+_gcbc IR 정의를 인덱싱하고, 루트에서 call/invoke 를 BFS 로 따라가며
각 함수가 참조하는 TLS 전역(@llvm.threadlocal.address / thread_local 전역 참조)을 모은다. 루트의 직접 콜리별로 도달 TLS 집합을 낸다."""
import sys, os, re, io, json, collections, pickle

SCR = os.path.dirname(os.path.abspath(__file__))
DIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc']
IDX = os.path.join(SCR, 'defidx2.pkl')
DEFRE = re.compile(rb'^define[^@]*@([A-Za-z0-9_$.]+)\(')
CALLRE = re.compile(r'(?:call|invoke)[^@\n]*@([A-Za-z0-9_$.]+)\(')
TLSRE = re.compile(r'@llvm\.threadlocal\.address(?:\.p0)?\(ptr @([A-Za-z0-9_$.]+)\)')

def build_idx():
    if os.path.exists(IDX):
        return pickle.load(open(IDX, 'rb'))
    idx = {}
    tlsg = set(); anon = {}
    for d in DIRS:
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.ll') or fn.startswith('_'): continue
            p = os.path.join(d, fn)
            with open(p, 'rb') as f:
                cur = None; off = 0
                for raw in f:
                    if raw.startswith(b'define'):
                        m = DEFRE.match(raw)
                        cur = m.group(1).decode() if m else None
                        if cur: idx[cur] = [p, off, None]
                    elif cur and raw.startswith(b'}'):
                        idx[cur][2] = off + len(raw); cur = None
                    elif raw.startswith(b'@') and b'thread_local' in raw:
                        tlsg.add(raw.split(b' ')[0][1:].decode())
                    elif raw.startswith(b'@anon.') and b'constant ptr @' in raw:
                        m2 = re.match(rb'^@(anon\.[0-9a-f]+\.\d+) = [^@]*constant ptr @([A-Za-z0-9_$.]+)', raw)
                        if m2: anon[(p, m2.group(1).decode())] = m2.group(2).decode()
                    off += len(raw)
    pickle.dump((idx, tlsg, anon), open(IDX, 'wb'))
    return idx, tlsg, anon

def body(idx, sym):
    p, s, e = idx[sym]
    with open(p, 'rb') as f:
        f.seek(s); return f.read(e - s).decode('utf-8', 'replace')

SKIP = ('core::', 'panicking', 'drop_glue', 'drop_in_place', 'unwrap_failed', 'panic_bounds', '_RNvNtCsjihNppCmMEE_4core', '_RNvNtCs9ec1k27omRZ_3std', 'alloc')

def info(idx, sym, cache):
    if sym in cache: return cache[sym]
    b = body(idx, sym)
    calls = []
    p = idx[sym][0]
    for a in set(re.findall(r'@(anon\.[0-9a-f]+\.\d+)', b)):
        t = ANON.get((p, a))
        if t and t in idx and t not in calls: calls.append(t)
    for m in CALLRE.finditer(b):
        c = m.group(1)
        if c.startswith('llvm.'): continue
        if c not in calls: calls.append(c)
    tls = sorted(set(TLSRE.findall(b)))
    cache[sym] = (calls, tls)
    return cache[sym]

def main():
    root_sub = sys.argv[1]
    depth = 64
    if '--depth' in sys.argv: depth = int(sys.argv[sys.argv.index('--depth')+1])
    global ANON
    idx, tlsg, ANON = build_idx()
    roots = [s for s in idx if root_sub in s and 'retain' not in s and not s.startswith('_RINvMs2_NtNtCs9ec1k27omRZ_3std')]
    print('roots', roots)
    root = roots[0]
    cache = {}
    calls, tls = info(idx, root, cache)
    print('ROOT TLS direct:', tls)
    print('ROOT direct callees (', len(calls), '):')
    # per direct callee BFS
    result = {}
    for c in calls:
        if c not in idx:
            print('  [no define]', c[:120]); continue
        seen = {c: 0}; q = collections.deque([(c, 0)]); reach = {}; path = {c: [c]}
        while q:
            s, d = q.popleft()
            cs, ts = info(idx, s, cache)
            for t in ts:
                if t not in reach: reach[t] = (d, path[s])
            if d >= depth: continue
            for n in cs:
                if n in seen or n not in idx: continue
                if any(k in n for k in ('drop_glue', 'drop_in_place', 'panicking', 'unwrap_failed', 'panic_bounds')): continue
                seen[n] = d+1; path[n] = path[s] + [n]; q.append((n, d+1))
        result[c] = {"reach": reach, "seen": sorted(seen)}
        print('  ', c[:110], '| fns', len(seen), '| TLS', len(reach))
        for t, (d, pth) in sorted(reach.items(), key=lambda x: x[1][0]):
            print('      ', t[-90:], 'depth', d, 'via', ' > '.join(p[-60:] for p in pth[-3:]))
    json.dump(result, open(os.path.join(SCR, 'tlsreach_out.json'), 'w', encoding='utf-8'), indent=1)

main()
