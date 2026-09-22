"""tlsrev.py — 역방향: 각 TLS(LocalKey::with 인스턴스 / 직접 threadlocal 참조) 노드에서 역BFS 해
루트(action_candidates)의 직접 콜리 중 어느 것들에서 도달되는지, 최단 경로와 함께 찍는다.
usage: python tlsrev.py <pkl> <root_regex> [depth]
"""
import pickle, re, sys, collections
G, tlsnames = pickle.load(open(sys.argv[1], 'rb'))
rx = re.compile(sys.argv[2]); depth = int(sys.argv[3]) if len(sys.argv) > 3 else 8
root = [s for s in G if rx.search(s)][0]
direct = G[root][0]
# reverse graph
R = collections.defaultdict(set)
for s, (c, t, f) in G.items():
    for x in c:
        R[x].add(s)
def short(s):
    m = re.findall(r'\d+([A-Za-z_][A-Za-z0-9_]*)', s)
    return '::'.join(m[-3:]) if m else s[:50]
def cachename(s):
    m = re.search(r'\d+([A-Z][A-Za-z0-9_]*(?:CACHE|MEMO|CTX|BEAMS|SCRATCH|VISITED|GRID|RECENCY|BBOX|FLAG|SPAWNS|PROFILE|BRIEFS|ACC|SEED|STATE|COLOR|Cache|Memo|Ctx|Beams|Scratch))', s)
    if m: return m.group(1)
    m = re.search(r'8LocalKey(.*?)4with', s)
    return 'with<' + (m.group(1)[-60:] if m else '?') + '>'
# TLS nodes = with-instances + defines that touch RUST_STD_INTERNAL_VAL
tlsnodes = [s for s, (c, t, f) in G.items() if ('8LocalKey' in s and '4with' in s) or any('RUST_STD_INTERNAL_VAL' in x for x in t)]
print('tls nodes', len(tlsnodes), 'direct callees', len(direct))
res = collections.defaultdict(dict)   # cache -> direct callee -> (depth, path)
for tn in tlsnodes:
    seen = {tn: None}; q = collections.deque([(tn, 0)])
    while q:
        s, d = q.popleft()
        if s in direct:
            cn = cachename(tn)
            path = []; x = s
            while x is not None: path.append(x); x = seen[x]
            cur = res[cn].get(s)
            if cur is None or d < cur[0]:
                res[cn][s] = (d, path)
        if d >= depth: continue
        for p in R.get(s, ()):
            if p not in seen:
                seen[p] = s; q.append((p, d + 1))
for cn in sorted(res):
    print('==', cn)
    for s, (d, path) in sorted(res[cn].items(), key=lambda x: x[1][0]):
        print('   from %-45s depth %d : %s' % (short(s), d, ' -> '.join(short(p) for p in path)))
