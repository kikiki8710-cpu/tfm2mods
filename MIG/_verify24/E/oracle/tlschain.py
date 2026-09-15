# -*- coding: utf-8 -*-
"""tlschain.py <tlstree json> — TLS 접점 함수까지의 루트→콜리 사슬을 찍는다."""
import json, io, collections, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
d = json.load(io.open(sys.argv[1], encoding='utf-8'))
g = d['graph']; root = [k for k, v in d['seen'].items() if v == 0][0]
par = {root: None}; q = collections.deque([root])
while q:
    s = q.popleft()
    for c in g.get(s) or []:
        if c not in par:
            par[c] = s; q.append(c)
def short(s):
    m = re.findall(r'[0-9]+([A-Za-z_]+)', s)
    return '::'.join(x for x in m if x not in ('game_ai',))[:120]
for f in d['tls']:
    chain = []; x = f
    while x:
        chain.append(x); x = par.get(x)
    print(' <- '.join(short(c) for c in chain))
    for t in d['tls'][f]:
        m = re.search(r'7RefCell.*?([0-9]+)([A-Za-z]+(?:Cache|Ctx|Masks|Scratch|Memo|MEMO))', t)
        print('     TLS:', (m.group(2) if m else t[:80]), '·', 'Cell' if '4cell4CellI' in t else 'RefCell')
    print()
