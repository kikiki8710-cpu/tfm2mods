#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""_perc_diag.py - 전파가 왜 되는지/안 되는지의 **원인 지표**를 잰다(읽기 전용).

퍼콜레이션이 성공하려면 필요한 것은 결국 두 가지다:
  (1) **엣지 보존율 s** - 씨앗끼리 exe 에 있는 호출 엣지가 DLL 에도 그대로 있는 비율.
      증인 하나가 서려면 엣지가 **양쪽 다** 살아 있어야 하므로 실효 확률은 s 다.
  (2) **씨앗 밀도** - 후보의 이웃 중 이미 맞춰진 것의 비율.
  기대 증인수 ~= (이웃 수) x (이웃이 씨앗일 확률) x s. 이게 r 을 못 넘으면 안 퍼진다.
"""
import io
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
HERE = os.path.dirname(os.path.abspath(__file__))

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
seeds = {int(k): v for k, v in c['seeds'].items()}
pairs = {e: rva_of[v['m']] for e, v in seeds.items()}
sset_d = set(succ_d) | set(pred_d)

print('=== 1. 엣지 보존율 s (씨앗↔씨앗 엣지가 양쪽에 다 있나) ===')
tot = keep = 0
lost_hub = 0
for E, D in pairs.items():
    for x in succ_e.get(E, ()):
        if x not in pairs:
            continue
        tot += 1
        if pairs[x] in succ_d.get(D, ()):
            keep += 1
print('  exe 에 있는 씨앗↔씨앗 호출엣지 %d개 중 DLL 에도 있는 것 %d개  ->  s = %.3f'
      % (tot, keep, keep / max(tot, 1.0)))
# 반대 방향(DLL 기준)도 본다 - 어느 쪽이 엣지를 더 잃었는지
rpairs = {}
for e, dr in pairs.items():
    rpairs.setdefault(dr, []).append(e)
tot2 = keep2 = 0
for D, Es in rpairs.items():
    for x in succ_d.get(D, ()):
        if x not in rpairs:
            continue
        tot2 += 1
        if any(pairs.get(e2) == x for E in Es for e2 in succ_e.get(E, ())):
            keep2 += 1
print('  DLL 에 있는 씨앗↔씨앗 호출엣지 %d개 중 exe 에도 있는 것 %d개  ->  s\' = %.3f'
      % (tot2, keep2, keep2 / max(tot2, 1.0)))

print()
print('=== 2. 차수 대응 (인라인 차이로 그래프 모양이 얼마나 어긋났나) ===')
import statistics as st
ro, ri = [], []
for E, D in pairs.items():
    oe, od = len(succ_e.get(E, ())), len(succ_d.get(D, ()))
    ie, idd = len(pred_e.get(E, ())), len(pred_d.get(D, ()))
    if oe or od:
        ro.append((oe, od))
    if ie or idd:
        ri.append((ie, idd))
same_o = sum(1 for a, b in ro if a == b)
print('  진출차수: exe 중앙 %d / dll 중앙 %d · 완전일치 %d/%d (%.0f%%)'
      % (st.median([a for a, b in ro]), st.median([b for a, b in ro]),
         same_o, len(ro), 100.0 * same_o / max(len(ro), 1)))
same_i = sum(1 for a, b in ri if a == b)
print('  진입차수: exe 중앙 %d / dll 중앙 %d · 완전일치 %d/%d (%.0f%%)'
      % (st.median([a for a, b in ri]), st.median([b for a, b in ri]),
         same_i, len(ri), 100.0 * same_i / max(len(ri), 1)))

print()
print('=== 3. 씨앗 이웃 밀도 (전파 여지) ===')
nb = set()
for E in pairs:
    nb.update(succ_e.get(E, ()))
    nb.update(pred_e.get(E, ()))
nb -= set(pairs)
print('  씨앗 1홉 이웃 중 미확정 exe 함수 %d개' % len(nb))
h = {}
for E in nb:
    k = 0
    for x in succ_e.get(E, ()):
        if x in pairs:
            k += 1
    for x in pred_e.get(E, ()):
        if x in pairs:
            k += 1
    h[min(k, 5)] = h.get(min(k, 5), 0) + 1
print('  그중 씨앗 이웃을 1개만 가진 것 %d(%.0f%%) · 2개 %d · 3개 %d · 4개 %d · 5+ %d'
      % (h.get(1, 0), 100.0 * h.get(1, 0) / max(len(nb), 1), h.get(2, 0), h.get(3, 0),
         h.get(4, 0), h.get(5, 0)))
print('  ★증인은 **씨앗 이웃 수의 상한을 못 넘는다**. 씨앗 이웃이 1개인 함수는')
print('    엣지가 100%% 보존돼도 증인이 최대 1개라 r>=2 에서 원리적으로 확정 불가.')
ge2 = sum(v for k, v in h.items() if k >= 2)
print('  r>=2 가 원리적으로 가능한 미확정 함수 = %d개 (전체 미확정 이웃의 %.0f%%)'
      % (ge2, 100.0 * ge2 / max(len(nb), 1)))
print('  s=%.2f 를 곱한 기대 확정 가능 수 ~= %d개' % (keep / max(tot, 1.0),
                                          int(ge2 * (keep / max(tot, 1.0)) ** 2)))

print()
print('=== 4. 후보 풀 비대칭 (부분그래프 매칭 문제의 크기) ===')
print('  exe 노드 %d · DLL 심볼 후보 %d  ->  exe 노드 %d개당 후보 1개'
      % (len(set(succ_e) | set(pred_e)), len(set(rva_of.values())),
         (len(set(succ_e) | set(pred_e))) // max(len(set(rva_of.values())), 1)))
print('  = exe 함수의 %.1f%% 만이 DLL 에 짝이 있을 수 있다. 나머지는 "매칭 없음"이 정답.'
      % (100.0 * len(set(rva_of.values())) / max(len(set(succ_e) | set(pred_e)), 1)))
