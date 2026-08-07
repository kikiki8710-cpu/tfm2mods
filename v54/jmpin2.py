# -*- coding: utf-8 -*-
"""후킹 대상 선두 12B 안으로 **점프해 들어오는 분기**가 있는지 exe 전역 스캔.
07-31 SubPlan 디스패처 wrap 크래시의 원인 후보 ① 을 정적으로 검사한다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
import capstone

p = load('054')
TARGETS = {'경매 eacf10': 0xeacf10, 'disc18 da1850(안전 실증)': 0xda1850,
           'disc19 dac090(안전 실증)': 0xdac090, 'SubPlan disp e52990(구 크래시)': 0xe52990}

md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
BR = capstone.CS_GRP_JUMP, capstone.CS_GRP_CALL

# .text 전역 선형 디스어셈 (근사 — 시작 정렬 오차는 무시할 만함)
sec = [s for s in p.sections if s[0]=='.text'][0]
start, size = sec[1], sec[2]
data = p.rd(start, size)

hits = {k: [] for k in TARGETS}
for ins in md.disasm(data, start):
    if not (set(ins.groups) & {capstone.CS_GRP_JUMP, capstone.CS_GRP_CALL}):
        continue
    if len(ins.operands) != 1 or ins.operands[0].type != capstone.CS_OP_IMM:
        continue
    t = ins.operands[0].imm
    for k, base in TARGETS.items():
        if t == base:
            hits[k].append(ins.mnemonic)

from collections import Counter
for k, base in TARGETS.items():
    c = Counter(hits[k])
    tail = sum(v for m, v in c.items() if m != 'call')
    print('%-28s call=%-4d 테일콜(jmp/jcc)=%d  %s' % (k, c.get('call', 0), tail,
          '⚠테일콜 진입 있음' if tail else '안전'))
