# -*- coding: utf-8 -*-
"""callseq.py <fn3> <fn4> — 두 함수의 직접 call 대상을 순서대로 나란히"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
def seq(E,a):
    f=E.func_of(int(a,16)); out=[]
    for i in R.insns(E,f[0],f[1]):
        if i.mnemonic=='call' and i.op_str.startswith('0x'):
            out.append((i.address-B,int(i.op_str,16)-B))
    return out
a=seq(R.E3,sys.argv[1]); b=seq(R.E4,sys.argv[2])
for k in range(max(len(a),len(b))):
    x=('%06x -> %06x'%a[k]) if k<len(a) else ''
    y=('%06x -> %06x'%b[k]) if k<len(b) else ''
    print('%2d  %-22s | %s'%(k,x,y))
