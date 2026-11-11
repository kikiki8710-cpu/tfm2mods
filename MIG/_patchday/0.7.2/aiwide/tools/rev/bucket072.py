# -*- coding: utf-8 -*-
"""재검 후보를 묶음 W1~W8 로 나눠 aiwide\\lists\\W*.txt 를 만든다(EDITED·review·weak·H-review·사라짐·새로 생김 전부).

분류 원칙 = 소스 모듈. 0.7.1 회차 W1~W6 과 같은 축 + path/rl 묶음 W7 + 위치정보 없는 보조 함수 짝(H) W8.
(0.7.1 회차는 H 를 W7 에 넣은 뒤 손으로 W8 로 옮겼다 — 여기선 처음부터 W8.)
한 묶음이 SPLIT 행을 넘으면 같은 모듈이 갈라지지 않게 모듈 순으로 반씩 W?a·W?b 로 나눈다(W8 은 모듈이 no-loc 라 RVA 순).
"""
import collections
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from semlib import *

T = pickle.load(open(os.path.join(HERE2, 'triage072.pkl'), 'rb'))
FO, FN = full()
OUT = os.path.join(HERE2, '..', '..', 'lists')
os.makedirs(OUT, exist_ok=True)
B = chr(92)


def bucket(m):
    if m.startswith('plan' + B + 'handler'):
        return 'W1'
    if any(m.startswith('plan' + B + 'big_plan' + B + x) for x in ('battle', 'fight_model', 'defense_nexus', 'single_battle', 'single_line', 'death_battle')) \
            or m.startswith('fight_check') or m.startswith('finish_race') or m.startswith('game_finish'):
        return 'W2'
    if m.startswith('plan' + B + 'big_plan') or m.startswith('plan' + B + 'lane') or m.startswith('lane_'):
        return 'W3'
    if m.startswith('plan' + B + 'sub_plan') or m.startswith('small_action'):
        return 'W4'
    if m.startswith('plan' + B + 'team_plan'):
        return 'W5'
    if m.startswith(('path_field', 'path_finder', 'free_dist', 'rl.rs')):
        return 'W7'
    return 'W6'


L = collections.defaultdict(list)
for key in ('EDITED', 'review', 'weak'):
    for ko, kn, cat, rest, j in T.get(key, []):
        m = short(FO[ko]['top'])
        L[bucket(m)].append('%s %#x->%#x %s %dB->%dB jcc %d->%d | %s%s' % (
            cat, ko[0], kn[0], m, ko[1] - ko[0], kn[1] - kn[0], j[0], j[1], '[약함] ' if key == 'weak' else '',
            '; '.join('%s %s %s->%s' % x for x in rest)[:300]))
for k in gone:
    m = short(fo[k]['top'])
    L[bucket(m)].append('GONE %#x %s %dB' % (k[0], m, k[1] - k[0]))
for k in added:
    m = short(fn[k]['top'])
    L[bucket(m)].append('NEW %#x %s %dB' % (k[0], m, k[1] - k[0]))
for ko, kn, cat, rest, j in T.get('H-review', []):
    a = short(FO[ko]['top']) if ko in FO else 'no-loc'
    L['W8'].append('H %#x->%#x %s %dB->%dB jcc %d->%d | %s' % (ko[0], kn[0], a, ko[1] - ko[0], kn[1] - kn[0], j[0], j[1], '; '.join('%s %s %s->%s' % x for x in rest)[:300]))
# 사각지대(rbpv072 → blind.txt): 자동 제거한 짝 중 rbp 간접호출·작은 즉치·분기 방향이 다른 것. R 은 모듈 묶음, H 는 W8.
pb = os.path.join(HERE2, 'blind.txt')
if os.path.exists(pb):
    for line in open(pb, encoding='utf-8'):
        f = line.split()
        if not f:
            continue
        L['W8' if f[0] == 'H' else bucket(f[2])].append('BLIND ' + line.rstrip()[2:])
# 짝 없는 피호출(orph.py → lists\orphans.txt) → W8 의 둘째 몫. 있으면 W8 = W8a(H·BLIND-H) · W8b(ORPH).
po = os.path.join(OUT, 'orphans.txt')
if os.path.exists(po):
    orph = ['ORPH ' + x.strip() for x in open(po, encoding='utf-8') if x.startswith('  ')]
    if orph:
        L['W8a'] = L.pop('W8', [])
        L['W8b'] = orph
SPLIT = 120


def modkey(line):
    return line.split()[2]   # 'EDITED 0x..->0x.. 모듈 …' · 'GONE 0x.. 모듈 …' 둘 다 세 번째 칸


for w in list(L):
    rows = L[w]
    if len(rows) <= SPLIT or w[-1] in 'ab':
        continue
    if w == 'W8':
        rows = sorted(rows, key=lambda x: int(x.split()[1].split('->')[0], 16))
        cut = len(rows) // 2
    else:
        rows = sorted(rows, key=modkey)
        mods = [modkey(x) for x in rows]
        best = None
        for i in range(1, len(rows)):
            if mods[i] != mods[i - 1] and (best is None or abs(i - len(rows) / 2) < abs(best - len(rows) / 2)):
                best = i
        cut = best or len(rows) // 2
    del L[w]
    L[w + 'a'] = rows[:cut]
    L[w + 'b'] = rows[cut:]
for old in os.listdir(OUT):
    if old.startswith('W') and old.endswith('.txt') and old[:-4] not in L and not old.startswith('W9'):
        os.remove(os.path.join(OUT, old))   # 이전 실행에서 나눴던 낡은 묶음 파일
for w in sorted(L):
    p = os.path.join(OUT, w + '.txt')
    open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(L[w]) + '\n')
    c = collections.Counter(x.split()[0] for x in L[w])
    print(w, len(L[w]), dict(c))
print('합계', sum(len(v) for v in L.values()))
