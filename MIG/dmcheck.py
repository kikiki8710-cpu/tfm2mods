#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""dmcheck.py - `dllmatch.json`(2안) 의 이름을 **완전히 다른 신호**로 검산한다.

왜 필요한가:
  2안은 기계어 지문 + 호출그래프로 이름을 붙인다. 둘 다 **바이너리 쪽 신호**라
  같이 틀릴 수 있다. 독립 검산에는 **DWARF 의 소스 줄**을 쓴다(1안이 쓰던 신호).
  판정: exe 함수의 **패닉 줄이 그 rlib 함수의 줄 집합 안에 들어가는가**.
    - 들어가면 `일치`     : 두 독립 신호가 같은 답을 낸다.
    - 안 들어가면 `모순`  : 둘 중 하나가 틀렸다 - 사람이 봐야 한다.
    - 줄 정보가 없으면 `판정불가`(패닉 없는 함수 - 1안이 원래 못 잡던 것).
  ⚠`모순` 이 0 이어야 한다는 뜻은 아니다. rlib 과 exe 가 **실제로 다른 소스**인 곳이
    있기 때문이다(실측: nexus_final_stand 는 술어 자체가 다르다).
"""
import io
import json
import os
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

# ★줄 집합 추출은 `dllmatch.ir_index()` 가 정본이다.
#   원래 `ghidra_syms.build()` 를 썼는데 **그쪽은 인라인된 남의 파일 줄까지 섞는 버그**가 있다.
#   검산기가 오염된 신호를 쓰면 검산 자체가 무의미해진다(실측: 확정된 짝을 "모순" 으로 찍었다).
_gs = io.open(os.path.join(HERE, 'dllmatch.py'), encoding='utf-8').read().replace('\nmain()\n', '\n')
G = {'__name__': 'dm', '__file__': os.path.join(HERE, 'dllmatch.py')}
exec(compile(_gs, 'dllmatch.py', 'exec'), G)

def leaf_of(mangled):
    """v0 망글링은 `<길이><이름>` 길이접두라 정규식으로 못 자른다."""
    out, i, n = [], 0, len(mangled)
    while i < n:
        if mangled[i].isdigit():
            j = i
            while j < n and mangled[j].isdigit():
                j += 1
            ln = int(mangled[i:j])
            if j + ln <= n and ln > 0:
                out.append(mangled[j:j + ln].lstrip('_'))
                i = j + ln
                continue
        i += 1
    return out[-1] if out else mangled


_src, _lines = G['ir_index']()
# 아래 코드가 기대하는 모양으로 맞춘다: sym -> (_, 소스파일, _, _, 줄집합)
top = {s: (None, _src[s] + '.rs', 0, set(), _lines.get(s, set())) for s in _src}
info = json.load(open(os.path.join(HERE, 'aimap.json')))['info']
rows = json.load(open(os.path.join(HERE, 'dllmatch.json'), encoding='utf-8'))

# ⚠판정은 **부분집합이 아니라 순위**로 한다.
#   처음엔 `exe줄 ⊆ IR줄` 로 봤다가 `position_eval_at_uncached`(세 근거로 확정한 건)까지
#   "모순" 으로 찍혔다. IR 의 `!dbg` 수집이 완전하지 않아 몇 줄만 빠져도 부분집합이 깨진다.
#   의미 있는 질문은 "**같은 모듈의 모든 후보 중 이 심볼이 가장 잘 맞는가**" 다.
by_src = {}
for sym, t in top.items():
    base = t[1].replace(chr(92), '/').split('/')[-1].replace('.rs', '')
    by_src.setdefault(base, []).append(sym)

rank1 = ranked = worse = na = nosym = 0
bad_rows, cg_r1, cg_n = [], 0, 0
for r in rows:
    e = info.get(r['addr']) or info.get('0x%x' % r['rva'])
    if not isinstance(e, dict):
        continue                      # aimap 밖(그래프 전파 덤) - 줄 정보 자체가 없다
    ls = {x for x in e.get('lines', []) if isinstance(x, int)}
    t = top.get(r['mangled'])
    if t is None:
        nosym += 1
        continue
    if not ls:
        na += 1
        continue
    mine = len(ls & t[4]) / len(ls)
    cand = []
    for sym in by_src.get(str(e.get('mod', '')).replace(chr(92), '/').split('/')[-1], ()):
        cand.append((len(ls & top[sym][4]) / len(ls), sym))
    cand.sort(reverse=True)
    best = cand[0][0] if cand else 0.0
    cg = r.get('via') == 'callgraph'
    if cg:
        cg_n += 1
    if mine >= best - 1e-9:
        rank1 += 1
        if cg:
            cg_r1 += 1
    elif mine >= 0.5:
        ranked += 1
    else:
        worse += 1
        bad_rows.append((r.get('via', '지문'), r['addr'], r['name'], mine, best,
                         cand[0][1] if cand else '-'))

tot = rank1 + ranked + worse
print('=== 독립 검산(DWARF 패닉 줄 겹침 순위) ===')
print('  판정 가능 %d건' % tot)
print('    1위(같은 모듈 후보 중 최고)  %d (%.1f%%)' % (rank1, 100.0 * rank1 / max(tot, 1)))
print('    1위는 아니나 겹침>=0.5       %d' % ranked)
print('    ★겹침<0.5 이고 더 나은 후보 있음 %d  <- 의심' % worse)
print('  판정불가 %d(패닉 줄 없음) · IR 색인에 없는 심볼 %d' % (na, nosym))
print('  그래프 전파분 %d건 중 1위 %d (%.1f%%)' % (cg_n, cg_r1, 100.0 * cg_r1 / max(cg_n, 1)))
print()
print('  의심 사례(최대 20) — 내 답 vs 줄 겹침이 더 높은 후보:')
for via, addr, nm, mine, best, alt in bad_rows[:20]:
    print('    %-9s %-12s %-34s 내답겹침%.2f  최고%.2f  더나은후보=%s'
          % (via, addr, nm[:34], mine, best, leaf_of(alt)))
