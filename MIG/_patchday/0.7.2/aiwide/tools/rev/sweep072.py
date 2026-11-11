# -*- coding: utf-8 -*-
"""0.7.1→0.7.2 AI 함수 전 짝 의미 토큰 대조. 0.7.1 회차 sweep2+triage 를 처음부터 쓰도록 합친 것.

결과: sweep072.pkl · 표준출력 = 분류 통계 + 토큰 빈도(공통 원인 찾기용).
"""
import collections
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import semlib
from semlib import *

P = {}
pp = os.path.join(HERE2, 'unkpairs.pkl')
if os.path.exists(pp):
    P = pickle.load(open(pp, 'rb'))
add = 0
for o, (n, v, tot) in P.items():
    if v >= max(1, 0.6 * tot) and o not in semlib.G and n not in semlib.GI:
        semlib.G[o] = n
        semlib.GI[n] = o
        add += 1
print('투표 보강 짝', add, flush=True)
R = {}
for v, ko, kn, extra in rows:
    d, j = delta(ko, kn)
    R[ko] = (v.split('(')[0], kn, d, j)
H = {}
for o, (n, v, tot) in P.items():
    if semlib.G.get(o) != n:
        continue
    ko = io_.owner(o)
    kn = inn.owner(n)
    if not ko or not kn or ko[0] != o or kn[0] != n:
        continue
    d, j = delta(ko, kn)
    H[ko] = (kn, d, j)
pickle.dump(dict(R=R, H=H, G=semlib.G), open(os.path.join(HERE2, 'sweep072.pkl'), 'wb'))

cnt = collections.Counter()
freq = collections.Counter()
for ko, (cat, kn, d, j) in R.items():
    sem = [x for x in d if x[0][0] in ('call', 'vcall', 'imm', 'f', 'd', 'x')]
    unk = [x for x in d if x[0][0] == 'call?']
    cnt[(cat, 'sem' if sem else ('unk' if unk else ('jcc' if j[0] != j[1] else 'clean')))] += 1
    for (t, v), a, b, _, _ in d:
        if t in ('vcall', 'imm', 'f', 'd', 'x'):
            freq[(t, v, 'old' if a > b else 'new')] += 1
for k in sorted(cnt):
    print(k, cnt[k])
print('보조 짝', len(H))
print('--- 토큰 차이 빈도 상위')
for k, c in freq.most_common(70):
    print(c, k)
