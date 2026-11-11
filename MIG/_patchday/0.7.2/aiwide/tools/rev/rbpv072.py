# -*- coding: utf-8 -*-
"""자동 제거한 쌍의 사각지대 검사(0.7.1 재검0 §4 의 0.7.2 판).

의미 토큰은 ①`call [rbp+..]` vcall ②`cmp`·`test` 이외 일부 명령의 |v|<3 즉치 ③비교 방향(jcc 종류)을 보지 않는다.
- ① rbp/rsp 기반 간접 호출 슬롯 다중집합 비교
- ② 즉치 0·1·2 포함 cmp/test 다중집합 비교(작은 문턱 변경을 놓치지 않기 위해)
- ③ 조건 분기 니모닉 다중집합 비교(ja↔jb 처럼 방향이 바뀐 것)
셋 다 코드젠으로도 달라질 수 있어 「차이 있음」 = 재검 후보일 뿐이다.
"""
import collections
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from semlib import *

T = pickle.load(open(os.path.join(HERE2, 'triage072.pkl'), 'rb'))
FO, FN = full()


def slot_new(s):
    """효과(엔티티) trait vtable 구 슬롯 → 신 슬롯 후보(triage072.slot_new 와 같은 표 · 공통원인.md §2-1)."""
    if 0x28 <= s <= 0x128:
        return (s + 0x8,)
    if 0x130 <= s <= 0x228:
        return (s + 0x10,)
    if 0x230 <= s <= 0x268:
        return (s + 0x18,)
    if s == 0x270:
        return (0x288, 0x290)
    if s == 0x278:
        return (0x298,)
    return ()


def feats(img, k):
    rv = collections.Counter()
    small = collections.Counter()
    jc = collections.Counter()
    for i in dis(img, k):
        mn = i.mnemonic
        if mn in ('call', 'jmp') and i.operands and i.operands[0].type == X86_OP_MEM:
            m = i.operands[0].mem
            if m.base in (X86_REG_RBP, X86_REG_RSP):
                rv[m.disp] += 1
        if mn in ('cmp', 'test') and len(i.operands) == 2 and i.operands[1].type == X86_OP_IMM:
            v = i.operands[1].imm
            if 0 <= v < 3:
                small[(mn, v)] += 1
        if mn.startswith('j') and mn != 'jmp':
            jc[mn] += 1
    return rv, small, jc


sets = [('R', a, b) for a, b in T.get('noise', []) + T.get('clean', [])] + [('H', a, b) for a, b in T.get('H-noise', []) + T.get('H-clean', [])]
print('대상', len(sets), flush=True)
hit = collections.Counter()
out = open(os.path.join(HERE2, 'blind.txt'), 'w', encoding='utf-8')
for tag, ko, kn in sets:
    a = feats(io_, ko)
    b = feats(inn, kn)
    d = []
    if a[0] != b[0]:
        rm = a[0] - b[0]
        ad = b[0] - a[0]
        for s in list(rm):
            cands = slot_new(s)
            if cands and sum(ad.get(t, 0) for t in cands) >= rm[s]:
                left = rm[s]
                for t in cands:
                    u = min(left, ad.get(t, 0))
                    ad[t] -= u
                    left -= u
                rm[s] = 0
        rm = +rm
        ad = +ad
        if rm or ad:
            d.append('rbp-vcall %s→%s' % ({hex(k): v for k, v in rm.items()}, {hex(k): v for k, v in ad.items()}))
            hit['rbp'] += 1
    if a[1] != b[1]:
        d.append('작은즉치 -%s +%s' % (dict(a[1] - b[1]), dict(b[1] - a[1])))
        hit['small'] += 1
    if a[2] != b[2]:
        d.append('분기 -%s +%s' % (dict(a[2] - b[2]), dict(b[2] - a[2])))
        hit['jcc'] += 1
    if d:
        top = short(FO[ko]['top']) if ko in FO else 'no-loc'
        print('%s %#x->%#x %s %dB->%dB | %s' % (tag, ko[0], kn[0], top, ko[1] - ko[0], kn[1] - kn[0], ' ; '.join(d)), file=out)
print(hit)
