# -*- coding: utf-8 -*-
"""disassemble every game-ai function (srcmap) and index immediates - handles imm8 too"""
import sys, collections, pickle, os
sys.path.insert(0,r'C:\tfm2mods\v54')
from scan import Scanner, srcmap
from pe2 import BASE
import capstone
def build(ver, hint='game-ai'):
    S=Scanner(ver)
    idx=collections.defaultdict(list)   # imm -> [(rva, fstart, mnem, ops)]
    sm=srcmap(ver)
    for fs,(src,lines) in sm.items():
        if hint not in src: continue
        f=S.func_of(fs)
        if not f: continue
        for i in S.disf(f):
            for op in i.operands:
                if op.type==capstone.x86.X86_OP_IMM:
                    idx[op.imm].append((i.address-BASE, fs, i.mnemonic, i.op_str))
    return S, idx
if __name__=='__main__':
    val=int(sys.argv[1],0)
    mnem = sys.argv[2:] or None
    out={}
    for ver in ('053','054'):
        S,idx=build(ver)
        sm=srcmap(ver)
        by=collections.defaultdict(list)
        for a,fs,m,o in idx.get(val,[]):
            if mnem and m not in mnem: continue
            by[sm[fs][0]].append((a,fs,m,o))
        out[ver]=by
    ks=sorted(set(out['053'])|set(out['054']))
    print('total 053=%d 054=%d'%(sum(len(v) for v in out['053'].values()),sum(len(v) for v in out['054'].values())))
    for k in ks:
        a,b=out['053'].get(k,[]),out['054'].get(k,[])
        print('%s %-84s %2d -> %2d'%('  ' if len(a)==len(b) else '*',k[:84],len(a),len(b)))
        if len(a)!=len(b):
            for lbl,l in (('053',a),('054',b)):
                for x in l[:10]: print('     [%s] %06x fn %06x %s %s'%(lbl,x[0],x[1],x[2],x[3]))
