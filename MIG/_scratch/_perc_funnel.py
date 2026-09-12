#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""_perc_funnel.py - 전 씨앗 1회 확산 후 **깔때기**를 센다.
어디서 후보가 사라지는지(증인 부족 / 동점 / 1:1 충돌)를 수치로 본다. 읽기 전용."""
import io
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
HERE = os.path.dirname(os.path.abspath(__file__))
CAP = int(sys.argv[1]) if len(sys.argv) > 1 else 40

c = json.load(io.open(os.path.join(HERE, '_perc_cache.json'), encoding='utf-8'))
succ_e = {int(k): v for k, v in c['cg_e'].items()}
succ_d = {int(k): v for k, v in c['cg_d'].items()}


def rev(g):
    o = {}
    for a, bs in g.items():
        for b in bs:
            o.setdefault(b, []).append(a)
    return o


pred_e, pred_d = rev(succ_e), rev(succ_d)
rva_of = {k: v[0] for k, v in c['dsym'].items()}
dcand = set(rva_of.values())
seeds = {int(k): v for k, v in c['seeds'].items()}
pairs = {e: rva_of[v['m']] for e, v in seeds.items()}
fp_e = {int(k): set(v) for k, v in c['fp_e'].items()}
fp_d = {k: set(v) for k, v in c['fp_d'].items()}
fp_dr = {}
for n, r in rva_of.items():
    if n in fp_d and r not in fp_dr:
        fp_dr[r] = fp_d[n]

marks = {}
for E1, D1 in pairs.items():
    A, B = pred_e.get(E1, ()), pred_d.get(D1, ())
    if len(A) <= CAP and len(B) <= CAP:
        Bc = [x for x in B if x in dcand]
        for E in A:
            for D in Bc:
                marks[(E, D)] = marks.get((E, D), 0) + 1
    A, B = succ_e.get(E1, ()), succ_d.get(D1, ())
    if len(A) <= CAP and len(B) <= CAP:
        Bc = [x for x in B if x in dcand]
        for E in A:
            for D in Bc:
                marks[(E, D)] = marks.get((E, D), 0) + 1

byE = {}
for (E, D), w in marks.items():
    if E in pairs:
        continue
    byE.setdefault(E, []).append((w, D))
print('=== 1회 확산 깔때기 (씨앗 %d개 전량 사용, cap=%d) ===' % (len(pairs), CAP))
print('후보쌍 %d개 · 후보를 하나라도 받은 미확정 exe 함수 %d개' % (len(marks), len(byE)))
for r in (1, 2, 3, 4, 5):
    ok = [E for E, v in byE.items() if max(w for w, _ in v) >= r]
    uniq = 0
    for E in ok:
        v = sorted(byE[E], reverse=True)
        if len(v) == 1 or v[0][0] > v[1][0]:
            uniq += 1
    # 지문으로 동점을 가르면?
    uniq_fp = 0
    for E in ok:
        v = sorted(byE[E], reverse=True)
        top = v[0][0]
        tied = [D for w, D in v if w == top]
        if len(tied) == 1:
            uniq_fp += 1
            continue
        fe = fp_e.get(E)
        if not fe or len(fe) < 5:
            continue
        sc = []
        for D in tied:
            fd = fp_dr.get(D)
            if not fd or len(fd) < 5:
                sc.append((-1, D))
                continue
            i = len(fe & fd)
            sc.append((i / float(len(fe) + len(fd) - i), D))
        sc.sort(reverse=True)
        if sc[0][0] > 0 and (len(sc) == 1 or sc[0][0] > sc[1][0]):
            uniq_fp += 1
    print('  증인 >=%d : 함수 %5d개 · 그중 1위가 유일 %5d개 · 지문으로 동점까지 가르면 %5d개'
          % (r, len(ok), uniq, uniq_fp))
print()
print('※ "증인 >=r 인 함수" 는 **상한**이다. 여기서 1:1 충돌(같은 D 를 여러 E 가 노림)과')
print('  전파 라운드의 순차 확정이 더 깎는다. 또 이 수치엔 **DLL 에 짝이 없는 exe 함수**가')
print('  섞여 있어(exe 의 96.8% 가 그렇다) 확정될수록 오답일 가능성이 함께 오른다.')

# 1:1 충돌 규모
top = {}
for E, v in byE.items():
    w, D = max(v)
    if w >= 2:
        top.setdefault(D, []).append(E)
mult = {D: v for D, v in top.items() if len(v) > 1}
print()
print('증인>=2 에서 1위 D 가 겹치는 경우: D %d개를 exe 함수 %d개가 동시에 지목'
      % (len(mult), sum(len(v) for v in mult.values())))
