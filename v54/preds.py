# -*- coding: utf-8 -*-
"""preds.py — 함수 내부 CFG 역추적: 주어진 RVA 로 오는 분기/폴스루 선행자 나열.
사용: python preds.py <ver> <fn내RVA> <target(hex)> [depth]"""
import sys
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from pe2 import BASE
import capstone
ver, anchor, tgt = sys.argv[1], int(sys.argv[2],16), int(sys.argv[3],16)
depth = int(sys.argv[4]) if len(sys.argv)>4 else 1
S=Scanner(ver); f=S.func_of(anchor); ins=S.disf(f)
addr=[i.address-BASE for i in ins]
JCC=set('jo jno jb jae je jne jbe ja js jns jp jnp jl jge jle jg'.split())
def preds(t):
    out=[]
    for k,i in enumerate(ins):
        a=i.address-BASE
        if i.mnemonic in JCC or i.mnemonic=='jmp':
            for op in i.operands:
                if op.type==capstone.x86.X86_OP_IMM and op.imm-BASE==t:
                    out.append(('br',a,i))
        if k+1<len(ins) and addr[k+1]==t and i.mnemonic not in ('jmp','ret','ud2'):
            out.append(('fall',a,i))
    return out
seen=set(); cur=[tgt]
for d in range(depth):
    nxt=[]
    for t in cur:
        print('== target %06x'%t)
        for kind,a,i in preds(t):
            print('   %-5s %06x  %-20s %s %s'%(kind,a,i.bytes.hex(),i.mnemonic,i.op_str))
            if a not in seen: seen.add(a); nxt.append(a)
    cur=nxt
