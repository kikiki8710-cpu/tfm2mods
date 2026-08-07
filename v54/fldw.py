# -*- coding: utf-8 -*-
"""fldw.py - 특정 구조체 오프셋에 대한 **쓰기/읽기** 사이트를 전역 스캔.
  python fldw.py <ver> <disp_hex> [w|r|rw] [srcfilter] [ctx]
  base 가 rsp/rbp/rip 인 것은 제외(스택/전역 아님 = 힙 객체 필드만).
"""
import io, os, sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
from pe2 import BASE
import capstone
X = capstone.x86

ver = sys.argv[1]
disp = int(sys.argv[2], 16)
mode = sys.argv[3] if len(sys.argv) > 3 else 'rw'
filt = sys.argv[4] if len(sys.argv) > 4 else ''
ctx = int(sys.argv[5]) if len(sys.argv) > 5 else 0
S = Scanner(ver)
n = 0
for f in S.funcs:
    src, lines = src_of(ver, f[0])
    if filt and filt.lower() not in (src or '').lower():
        continue
    try:
        ins = S.disf(f)
    except Exception:
        continue
    hit = []
    for k, i in enumerate(ins):
        for oi, op in enumerate(i.operands):
            if op.type == X.X86_OP_MEM and op.mem.disp == disp and op.mem.base != 0:
                bn = i.reg_name(op.mem.base)
                if bn in ('rsp', 'rbp', 'rip'):
                    continue
                # write = mem is dest operand (index 0) and instr writes
                iswr = (oi == 0 and i.mnemonic not in ('cmp', 'test', 'push'))
                if mode == 'w' and not iswr: continue
                if mode == 'r' and iswr: continue
                hit.append((k, iswr))
    if not hit:
        S._dis.clear(); continue
    print('--- fn %06x-%06x %s [%s]' % (f[0], f[1], (src or '?')[:80], (lines or '')[:40]))
    shown = set()
    for k, iswr in hit:
        for j in range(max(0, k - ctx), min(len(ins), k + ctx + 1)):
            if j in shown: continue
            shown.add(j)
            x = ins[j]
            print('    %s %06x  %-22s %s %s' % ('W' if (j == k and iswr) else ' ', x.address - BASE, x.bytes.hex(), x.mnemonic, x.op_str))
        n += 1
    S._dis.clear()
print('총 %d 사이트' % n)
