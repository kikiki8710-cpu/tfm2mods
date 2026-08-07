# -*- coding: utf-8 -*-
import sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
from pe2 import BASE
def calls(ver, fs):
    S = Scanner(ver); f = S.func_of(fs)
    out = collections.Counter()
    for i in S.disf(f):
        if i.mnemonic == 'call' and i.op_str.startswith('0x'):
            t = int(i.op_str,16) - BASE
            out[t]+=1
    res=[]
    for t,c in out.most_common():
        tf = S.func_of(t)
        src = src_of(ver, tf[0])[0] if tf else None
        res.append((t,c,(tf[1]-tf[0]) if tf else 0,src))
    return f,res
if __name__=='__main__':
    f,res = calls(sys.argv[1], int(sys.argv[2],16))
    print('fn %06x-%06x src=%s'%(f[0],f[1],src_of(sys.argv[1],f[0])[0]))
    for t,c,sz,src in res:
        print('  %06x x%-3d %6dB  %s'%(t,c,sz,src))
