# -*- coding: utf-8 -*-
"""stk.py — 함수 내에서 특정 [reg+disp] 스택 슬롯을 참조하는 명령 전수.
사용: python stk.py <ver> <함수내RVA> <disp(hex)> [reg]"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from pe2 import BASE
import capstone
ver, rva, disp = sys.argv[1], int(sys.argv[2],16), int(sys.argv[3],16)
reg = sys.argv[4] if len(sys.argv)>4 else 'rbp'
S = Scanner(ver); f = S.func_of(rva)
print('fn %06x-%06x'%f)
for i in S.disf(f):
    for op in i.operands:
        if op.type==capstone.x86.X86_OP_MEM and op.mem.disp==disp and i.reg_name(op.mem.base)==reg:
            print('%06x  %-24s %s %s'%(i.address-BASE, i.bytes.hex(), i.mnemonic, i.op_str))
            break
