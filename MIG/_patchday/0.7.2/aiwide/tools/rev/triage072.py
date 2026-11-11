# -*- coding: utf-8 -*-
"""sweep072.pkl → 재검 후보 분류. 0.7.1 triage 의 잡음 규칙을 0.7.2 공통 원인으로 바꾼 것.

잡음(자동 제거) — 근거·빈도 = aiwide\\공통원인.md §2
  · 효과(엔티티) trait vtable 슬롯 재배열(구 76/77 메서드 → 신 80/81, 메서드 4개 끼어듦):
      구 +0x28~+0x128 → +8 · +0x130~+0x228 → +0x10 · +0x230~+0x268 → +0x18 · +0x270 → +0x288|+0x290 · +0x278 → +0x298
    — 사라진 슬롯 수만큼 사상된 신 슬롯이 생겼을 때만(양쪽 수가 맞을 때만)
  · 구조체 배치 즉치 짝(LAYOUT 표 — 보폭·크기·필드 오프셋) — 사라진 수만큼 사상된 신 값이 생겼을 때만
  · 호출 대상 교체가 크기 ±15% 안에서 개수가 맞음(썽크·자매 함수)
  · 필드 오프셋형 즉치 이동(0x100~0x10000 · 짝 수 일치 · 차 ≤0x800)
  · (semlib) Blackboard 보폭 0xf18→0x1010 · BigPlan 크기 0x4c8→0x4d8 는 구 쪽 즉치를 미리 정규화
약한 차이(따로 표시 · 자동 제거 안 함)
  · cmp 즉치 ±1 짝(cmp x,N;ja ↔ cmp x,N+1;jae 코드젠일 수 있으나 진짜 문턱 변경일 수도 있다)
"""
import collections
import os
import pickle
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from semlib import *

X = pickle.load(open(os.path.join(HERE2, 'sweep072.pkl'), 'rb'))
R, H, G = X['R'], X['H'], X['G']
GI = {v: k for k, v in G.items()}
FO, FN = full()
def slot_new(s):
    """효과(엔티티) trait vtable 구 슬롯 → 신 슬롯 후보. vtable 91개 지문 정렬로 확정(공통원인.md §2-1)."""
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


SLOT = {s: slot_new(s) for s in range(0x28, 0x280, 8)}
# 구조체 배치 즉치 짝(구 → 신 후보). 'imm op' 에만 적용. 공통원인.md §2-3 표와 같다(표본 RVA·명령은 거기).
#   전부 `add r,imm`(필드 기준)·`imul r,r,imm`(보폭)·`mov r8d,imm`(memcpy 크기) 꼴로 확인한 것만 넣었다.
LAYOUT = {
    0x238: (0x210,),     # 필드 기준 +0x238→+0x210 (handler·big_plan 전반 · Δ-0x28)
    0x990: (0xa80,), 0x998: (0xa88,), 0x878: (0x968,),   # Blackboard 안 필드(Δ+0xf0)
    0x1358: (0x1540,),   # handler 계열 필드 기준(Δ+0x1e8)
    0x1b8: (0x390,),     # team_plan cascade·objective_handlers·engage 필드 기준(Δ+0x1d8)
    0x170: (0x140,),     # big_plan defense_nexus 필드 기준(Δ-0x30)
    0x190: (0x1c0,), 0x188: (0x1b8,),   # rl.rs 원소 보폭·끝 태그(Δ+0x30)
    0xc0: (0xf0,), 0xb8: (0xe8,), 0xd0: (0x100,), 0x88: (0xb8,),   # Δ+0x30 계열 크기·보폭(auction·rl·line_wait·small_action)
    0xe8: (0x100,),      # handler 필드 기준(Δ+0x18)
}
# 작은 수라 자동 제거하지 않고 'weak' 로만 표시하는 배치 짝(진짜 상수 40·80 변경과 구별이 안 된다)
LAYOUT_WEAK = {
    0x28: (0x78,), 0x50: (0xf0, 0x78), 0xa0: (0xf0,),   # handler relays·lane_ops 팀별 하위 레코드 0x28→0x78(×2 펼침 0x50→0xf0)
    0x40: (0x68, 0x70), 0x48: (0x70,), 0x80: (0xb0,),    # Δ+0x28·+0x30 필드 오프셋(보조 함수·small_action)
}


def cancel(rm, ad, table):
    """rm·ad = Counter(값→개수). table[구값] = 신값 후보들. 사라진 수 ≤ 후보 쪽 생긴 수 합일 때만 지운다. 지운 것 반환."""
    hit = []
    for s, c in list(rm.items()):
        cands = table.get(s, ())
        if not cands or c <= 0:
            continue
        if sum(ad.get(t, 0) for t in cands) < c:
            continue
        left = c
        for t in cands:
            u = min(left, ad.get(t, 0))
            ad[t] -= u
            left -= u
        del rm[s]
        hit.append(s)
    return hit


def nsz(t):
    k = inn.owner(t)
    return (k[1] - k[0]) if k else -1


def classify(d):
    rest = []
    calls_rm = collections.Counter()
    calls_ad = collections.Counter()
    vrm = collections.Counter()
    vad = collections.Counter()
    for (t, v), a, b, oa, na in d:
        if t == 'vcall':
            s = int(v[1:], 16)
            (vad if b > a else vrm)[s] += abs(b - a)
            continue
        if t in ('call', 'call?'):
            if t == 'call':
                tt = int(v, 16)
                sz = nsz(tt)
            else:
                tt = int(v[1:], 16)
                if v[0] == 'o':
                    k = io_.owner(tt)
                    sz = (k[1] - k[0]) if k else -1
                else:
                    sz = nsz(tt)
            if b > a:
                calls_ad[sz] += b - a
            else:
                calls_rm[sz] += a - b
            continue
        rest.append((t, v, a, b))
    cancel(vrm, vad, SLOT)
    # 구조체 배치 즉치 짝
    irm = collections.Counter()
    iad = collections.Counter()
    for x in rest:
        if x[0] == 'imm' and x[1].startswith('op '):
            vv = int(x[1][3:], 16)
            if x[2] > x[3]:
                irm[vv] += x[2] - x[3]
            else:
                iad[vv] += x[3] - x[2]
    hit = cancel(irm, iad, LAYOUT)
    hitw = cancel(irm, iad, LAYOUT_WEAK)
    if hit or hitw:
        rest = [x for x in rest if not (x[0] == 'imm' and x[1].startswith('op '))]
        rest += [('imm', 'op %#x' % vv, c, 0) for vv, c in sorted(irm.items()) if c]
        rest += [('imm', 'op %#x' % vv, 0, c) for vv, c in sorted(iad.items()) if c]
        if hit:
            rest.append(('layout', '', 0, 0))
        if hitw:
            rest.append(('layout?', ','.join('%#x' % vv for vv in hitw), 0, 0))
    for s, c in vrm.items():
        if c:
            rest.append(('vcall-', '+%x' % s, c, 0))
    for s, c in vad.items():
        if c:
            rest.append(('vcall+', '+%x' % s, 0, c))
    a_ = sorted(calls_rm.elements())
    b_ = sorted(calls_ad.elements())
    if not (len(a_) == len(b_) and all(abs(x - y) <= max(16, 0.15 * max(x, y)) for x, y in zip(a_, b_))):
        rest.append(('calls', str(dict(calls_rm)) + '->' + str(dict(calls_ad)), 0, 0))
    im = [x for x in rest if x[0] == 'imm' and x[1].startswith('op ')]
    rm = [(int(x[1][3:], 16), x[2] - x[3]) for x in im if x[2] > x[3]]
    ad = [(int(x[1][3:], 16), x[3] - x[2]) for x in im if x[3] > x[2]]
    if rm and ad and sum(c for _, c in rm) == sum(c for _, c in ad) and all(0x100 <= v < 0x10000 for v, _ in rm + ad) \
            and all(min(abs(v - w) for w, _ in ad) <= 0x800 for v, _ in rm):
        rest = [x for x in rest if x not in im] + [('offset-shift', '', 0, 0)]
    return rest


NOISE = ('offset-shift', 'layout')


def weak_only(rest):
    """남은 차이가 cmp 즉치 ±1 짝 · 작은 배치 짝(layout?) 뿐인가."""
    core = [x for x in rest if x[0] not in NOISE and x[0] != 'layout?']
    c = [x for x in core if x[0] == 'imm' and x[1].startswith('cmp ')]
    if len(c) != len(core):
        return False
    if not c:
        return any(x[0] == 'layout?' for x in rest)
    rm = sorted(int(x[1][4:], 16) for x in c for _ in range(max(x[2] - x[3], 0)))
    ad = sorted(int(x[1][4:], 16) for x in c for _ in range(max(x[3] - x[2], 0)))
    return len(rm) == len(ad) and all(abs(p - q) == 1 for p, q in zip(rm, ad))


out = collections.defaultdict(list)
for ko, (cat, kn, d, j) in sorted(R.items()):
    rest = classify(d)
    if cat == 'EDITED':
        out['EDITED'].append((ko, kn, cat, rest, j))
    elif all(x[0] in NOISE for x in rest) and j[0] == j[1]:
        out['clean' if not d else 'noise'].append((ko, kn))
    elif weak_only(rest) and j[0] == j[1]:
        out['weak'].append((ko, kn, cat, rest, j))
    else:
        out['review'].append((ko, kn, cat, rest, j))
for ko, (kn, d, j) in sorted(H.items()):
    rest = classify(d)
    if all(x[0] in NOISE for x in rest) and j[0] == j[1]:
        out['H-clean' if not d else 'H-noise'].append((ko, kn))
    else:
        out['H-review'].append((ko, kn, 'H', rest, j))
for k, v in out.items():
    print(k, len(v))
f = open(os.path.join(HERE2, 'review.txt'), 'w', encoding='utf-8')
for key in ('EDITED', 'review', 'weak', 'H-review'):
    print('#####', key, file=f)
    for ko, kn, cat, rest, j in out[key]:
        top = short(FO[ko]['top']) if ko in FO else 'no-loc'
        print('%s %#x->%#x %s %dB->%dB jcc %d->%d | %s' % (cat, ko[0], kn[0], top, ko[1] - ko[0], kn[1] - kn[0], j[0], j[1],
                                                         '; '.join('%s %s %s->%s' % x for x in rest)[:320]), file=f)
pickle.dump(dict(out), open(os.path.join(HERE2, 'triage072.pkl'), 'wb'))
mods = collections.Counter()
for key in ('EDITED', 'review'):
    for ko, kn, cat, rest, j in out[key]:
        mods[short(FO[ko]['top']).split(chr(92))[0] if False else short(FO[ko]['top'])] += 1
for m, c in mods.most_common(60):
    print(c, m)
