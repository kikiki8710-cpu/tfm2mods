# -*- coding: utf-8 -*-
"""xrefc.py - 직접 call/jmp 대상 RVA 로의 xref 전수(.pdata 함수단위 디스어셈).
  python xrefc.py <ver> <target_rva_hex>
"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
from pe2 import BASE
import capstone
X = capstone.x86
ver, tgt = sys.argv[1], int(sys.argv[2], 16)
S = Scanner(ver)
for f in S.funcs:
    try: ins = S.disf(f)
    except Exception: continue
    for i in ins:
        if i.mnemonic in ('call', 'jmp') and i.operands and i.operands[0].type == X.X86_OP_IMM and i.operands[0].imm - BASE == tgt:
            src, _ = src_of(ver, f[0])
            print('%06x  in fn %06x  %s  %s' % (i.address - BASE, f[0], i.mnemonic, (src or '?')[:60]))
    S._dis.clear()
