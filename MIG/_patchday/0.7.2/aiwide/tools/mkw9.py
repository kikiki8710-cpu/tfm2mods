# -*- coding: utf-8 -*-
"""game-core 크레이트(시뮬레이션·이적 AI 등) 변경 목록 → lists\\W9.txt(120행 넘으면 W9a·W9b). game-ai 밖이라 aidiff 표에 없는 부분."""
import collections
import os
import pickle
import sys

sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A

HERE = os.path.dirname(os.path.abspath(__file__))
a = pickle.load(open(os.path.join(HERE, 'full071.pkl'), 'rb'))
b = pickle.load(open(os.path.join(HERE, 'full072.pkl'), 'rb'))
pf, gf, af = A.pair(a, b)
B = chr(92)
rows = []
for ko, (kn, d, n) in sorted(pf.items()):
    top = a[ko]['top']
    if 'game-core' not in top:
        continue
    v, extra = A.verdict(a[ko], b[kn])
    if v.startswith('EDITED'):
        rows.append('EDITED %#x->%#x %s %dB->%dB' % (ko[0], kn[0], top.split('game-core' + B + 'src' + B)[-1], ko[1] - ko[0], kn[1] - kn[0]))
for k in sorted(gf):
    if 'game-core' in a[k]['top']:
        rows.append('GONE %#x %s %dB' % (k[0], a[k]['top'].split('game-core' + B + 'src' + B)[-1], k[1] - k[0]))
for k in sorted(af):
    if 'game-core' in b[k]['top']:
        rows.append('NEW %#x %s %dB' % (k[0], b[k]['top'].split('game-core' + B + 'src' + B)[-1], k[1] - k[0]))
OUT = os.path.join(HERE, '..', 'lists')
for f in os.listdir(OUT):
    if f.startswith('W9') and f.endswith('.txt'):
        os.remove(os.path.join(OUT, f))   # 이전 실행 산출물(W9 · W9a · W9b)
SPLIT = 120   # bucket072 와 같은 규칙: 넘으면 모듈 순으로 정렬해 모듈 경계에서 반으로 W9a·W9b
if len(rows) > SPLIT:
    rows = sorted(rows, key=lambda x: x.split()[2])
    mods = [x.split()[2] for x in rows]
    best = None
    for i in range(1, len(rows)):
        if mods[i] != mods[i - 1] and (best is None or abs(i - len(rows) / 2) < abs(best - len(rows) / 2)):
            best = i
    parts = {'W9a': rows[:best], 'W9b': rows[best:]}
else:
    parts = {'W9': rows}
for w, rr in parts.items():
    open(os.path.join(OUT, w + '.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(rr) + '\n')
    c = collections.Counter(x.split()[0] for x in rr)
    print(w, len(rr), dict(c), rr[0].split()[2], '...', rr[-1].split()[2])
print(len(rows))
