# -*- coding: utf-8 -*-
"""dbgfind.py - derived-Debug fmt 함수 후보 찾기.
  DebugStruct::field 헬퍼(인자로 준 RVA)를 N회 이상 호출하고,
  본문에 특정 disp(구조체 오프셋)를 쓰는 함수를 찾는다.
  python dbgfind.py <ver> <helper_rva_hex> <min_calls> [disp_hex,disp_hex...]
"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
from pe2 import BASE
import capstone
X = capstone.x86

ver = sys.argv[1]
helper = int(sys.argv[2], 16)
minc = int(sys.argv[3])
disps = set(int(x, 16) for x in sys.argv[4].split(',')) if len(sys.argv) > 4 else set()
S = Scanner(ver)
for f in S.funcs:
    try:
        ins = S.disf(f)
    except Exception:
        continue
    nc = 0
    ds = set()
    for i in ins:
        if i.mnemonic == 'call' and i.operands and i.operands[0].type == X.X86_OP_IMM:
            if i.operands[0].imm - BASE == helper:
                nc += 1
        for op in i.operands:
            if op.type == X.X86_OP_MEM and op.mem.base and i.reg_name(op.mem.base) not in ('rip',):
                ds.add(op.mem.disp)
    if nc >= minc and (not disps or disps <= ds):
        src, _ = src_of(ver, f[0])
        print('%06x-%06x  calls=%d  %s' % (f[0], f[1], nc, (src or '?')[:70]))
    S._dis.clear()
