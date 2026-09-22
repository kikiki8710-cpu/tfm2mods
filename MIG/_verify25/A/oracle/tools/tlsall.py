# -*- coding: utf-8 -*-
"""루트에서 전이적으로 닿는 모든 LocalKey::with 인스턴스(최단 경로) + 지목된 TLS(HP_VALUE_MEMO/INTER_CTX/CAMP_POS/…) 도달 여부"""
import pickle, re, sys, os
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tlsreach import cg, loc, tlsg, short, is_with, root, GAME_TLS
seen = {root: None}; q = [root]; hits = {}
while q:
    s = q.pop(0)
    if is_with(s):
        hits[s] = seen[s]; continue
    for k in cg.get(s, ()):
        if k not in seen:
            seen[k] = s; q.append(k)
print('reachable defines', len(seen), 'with instances', len(hits))
for h in sorted(hits, key=lambda x: short(x, 3)):
    path = []; x = h
    while x is not None: path.append(short(x, 2)); x = seen[x]
    print('%-60s depth=%d\n    %s' % (short(h, 3), len(path)-1, ' > '.join(reversed(path))))
print('\n--- game TLS 전역 전수(80 중 game):')
for g in sorted(GAME_TLS): print('  ', short(g, 4), tlsg[g])
print('\n--- 미도달 검사(이름 조각):')
for frag in ['champion_hp_value', 'HP_VALUE', 'interaction_score', 'INTER_CTX', 'camp_pos', 'CAMP_POS', 'siege_stance', 'last_stand', 'MAX_RANGE', 'max_range_cached', 'position_eval_at', 'cast_beams', 'check_kill_die_tick']:
    w = [s for s in cg if is_with(s) and frag.lower() in s.lower()]
    r = [s for s in w if s in hits]
    print('  %-22s with 인스턴스 %d개 · 루트에서 도달 %d개' % (frag, len(w), len(r)))
