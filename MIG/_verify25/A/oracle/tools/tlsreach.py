# -*- coding: utf-8 -*-
"""④ action_candidates 의 직접 콜리마다 전이적으로 닿는 LocalKey::with 인스턴스(=TLS 작성자) 를 BFS 로 뽑는다.
   사용: python tlsreach.py [루트 심볼 꼬리]   (기본 = action_candidates)"""
import pickle, re, sys, os, io
sys.stdout.reconfigure(encoding='utf-8')
d = pickle.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cg.pkl'), 'rb'))
cg, loc, tlsg = d['cg'], d['loc'], d['tlsg']
tail = sys.argv[1] if len(sys.argv) > 1 else '13BattleSubPlan17action_candidates'
root = [s for s in cg if s.endswith(tail)][0]
print('root', root, loc[root])
SKIP = set('game_ai game_core core std bumpalo collections vec Vec plan_legacy sub_plan simulation iter adapters traits iterator Iterator ops function FnOnce call_once FnMut call_mut option Option slice entity Entity ptr drop_glue thread local LocalKey cell RefCell'.split())
def short(sym, n=6):
    s = re.sub(r'Cs[A-Za-z0-9]+_', '', sym)
    s = re.sub(r'B[A-Za-z0-9]*_', '', s)
    out = []; i = 0
    while i < len(s):
        m = re.match(r's[A-Za-z0-9]*_', s[i:])
        if m and (i == 0 or not s[i-1].isdigit()):
            i += m.end(); continue
        m = re.match(r'(\d+)', s[i:])
        if m:
            k = int(m.group(1)); j = i + m.end()
            name = s[j:j+k]
            if name and re.match(r'[A-Za-z_][A-Za-z0-9_]*$', name) and k >= 2:
                out.append(name); i = j + k; continue
        i += 1
    keep = [w for w in out if w not in SKIP]
    return '::'.join(keep[-n:]) or sym[:40]
def is_with(s): return '5local' in s and '8LocalKey' in s and 'with' in s
GAME_TLS = {g for g in tlsg if 'game_ai' in g or 'game_core' in g}
def tls_of_with(w, depth=4):
    seen = set(); q = [(w, 0)]; found = set()
    while q:
        s, dd = q.pop()
        if s in seen or dd > depth: continue
        seen.add(s)
        for g in d['tlsref'].get(s, ()):
            if g in GAME_TLS: found.add(g)
        for c in cg.get(s, ()): q.append((c, dd+1))
    return found
direct = sorted(cg[root])
print('direct callees', len(direct))
rows = []
for c in direct:
    seen = {c: None}; q = [c]; hits = []
    while q:
        s = q.pop(0)
        if is_with(s):
            hits.append(s); continue
        for k in cg.get(s, ()):
            if k not in seen:
                seen[k] = s; q.append(k)
    for h in hits:
        path = []; x = h
        while x is not None: path.append(short(x, 3)); x = seen[x]
        rows.append((short(c, 4), short(h, 3), ' > '.join(reversed(path)), sorted(short(g, 3) for g in tls_of_with(h))))
seen_c = set()
for r in rows:
    if r[0] not in seen_c:
        print('\n### %s' % r[0]); seen_c.add(r[0])
    print('  with=%s\n     path: %s\n     tls : %s' % (r[1], r[2], r[3]))
print('\n--- 직접 콜리 중 TLS 미도달:')
withc = set(r[0] for r in rows)
for c in direct:
    if short(c, 4) not in withc: print('   ', short(c, 4))
