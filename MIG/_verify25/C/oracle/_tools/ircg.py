"""ircg.py — LLVM IR(.ll) 스트리밍 콜그래프 빌더. define 별 (콜리 심볼 집합, TLS 전역 참조 집합) 을 pickle 로 저장.
usage: python ircg.py build <out.pkl> <dir1> [dir2...]
       python ircg.py reach <pkl> <root_symbol_regex> [depth]    # 루트에서 TLS 전역까지 경로 BFS
"""
import sys, re, io, os, pickle, glob, collections

CALL = re.compile(r'(?:call|invoke)\b[^@]*?@([A-Za-z0-9_$.]+)\(')
TLS = re.compile(r'@([A-Za-z0-9_$.]*(?:RUST_STD_INTERNAL_VAL|MEMO|CACHE|SCRATCH|CTX|BEAMS|VISITED|GRID|RECENCY|BBOX|FLAG|SPAWNS|PROFILE|BRIEFS|ACC|SEED|STATE|COLOR)[A-Za-z0-9_$.]*)')
DEF = re.compile(r'^define [^@]*@([A-Za-z0-9_$.]+)\(')
TLSDECL = re.compile(r'^@([A-Za-z0-9_$.]+) = [^\n]*thread_local')

def build(out, dirs):
    G = {}
    tlsnames = set()
    for d in dirs:
        for fn in sorted(glob.glob(os.path.join(d, '*.ll'))):
            cur = None; calls = set(); tls = set()
            with io.open(fn, encoding='utf-8', errors='replace') as f:
                for ln in f:
                    if cur is None:
                        m = DEF.match(ln)
                        if m:
                            cur = m.group(1); calls = set(); tls = set()
                        else:
                            m = TLSDECL.match(ln)
                            if m: tlsnames.add(m.group(1))
                        continue
                    if ln.startswith('}'):
                        G[cur] = (frozenset(calls), frozenset(tls), os.path.basename(fn))
                        cur = None; continue
                    for m in CALL.finditer(ln):
                        s = m.group(1)
                        if not s.startswith('llvm.'): calls.add(s)
                    if 'thread_local' in ln or 'RUST_STD_INTERNAL_VAL' in ln or '@_RNvNCNKNv' in ln:
                        for m in re.finditer(r'@(_RNvNCNKNv[A-Za-z0-9_$.]+|[A-Za-z0-9_$.]*RUST_STD_INTERNAL_VAL[A-Za-z0-9_$.]*)', ln):
                            tls.add(m.group(1))
            print(fn, len(G), file=sys.stderr)
    with open(out, 'wb') as f:
        pickle.dump((G, tlsnames), f)
    print('defines', len(G), 'tls decls', len(tlsnames))

def reach(pkl, rootrx, depth):
    with open(pkl, 'rb') as f:
        G, tlsnames = pickle.load(f)
    rx = re.compile(rootrx)
    roots = [s for s in G if rx.search(s)]
    print('roots', len(roots))
    for r in roots:
        print('ROOT', r[:140])
        # BFS
        seen = {r: None}
        q = collections.deque([(r, 0)])
        hits = []
        while q:
            s, d = q.popleft()
            calls, tls, fn = G.get(s, (frozenset(), frozenset(), ''))
            tls2 = {t for t in tls if t in tlsnames or 'RUST_STD_INTERNAL_VAL' in t}
            if '8LocalKey' in s and '4with' in s:
                m = re.search(r'8LocalKey(.*?)4with', s)
                tls2.add('LocalKey::with<' + (m.group(1) if m else '?') + '>')
            if tls2:
                hits.append((s, d, tls2))
            if d >= depth: continue
            for c in calls:
                if c not in seen and c in G:
                    seen[c] = s; q.append((c, d + 1))
        for s, d, t in sorted(hits, key=lambda x: x[1]):
            path = []; x = s
            while x is not None:
                path.append(x); x = seen[x]
            path.reverse()
            print('  depth', d, 'TLS', sorted(short(u) for u in t))
            print('     via', ' -> '.join(short(p) for p in path[1:]))

def short(s):
    s = re.sub(r'^_R[A-Za-z0-9]*?(?=\d+[a-z_]+)', '', s)
    m = re.findall(r'\d+([A-Za-z_][A-Za-z0-9_]*)', s)
    return '::'.join(m[-4:]) if m else s[:60]

if __name__ == '__main__':
    if sys.argv[1] == 'build':
        build(sys.argv[2], sys.argv[3:])
    else:
        reach(sys.argv[2], sys.argv[3], int(sys.argv[4]) if len(sys.argv) > 4 else 6)
