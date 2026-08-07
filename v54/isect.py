# -*- coding: utf-8 -*-
"""isect.py <ver> <fn1> <fn2> ... — 여러 함수의 직접 call 대상 교집합"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
E=R.E3 if sys.argv[1]=='053' else R.E4
sets=[]
for a in sys.argv[2:]:
    f=E.func_of(int(a,16))
    s=set()
    for i in R.insns(E,f[0],f[1]):
        if i.mnemonic=='call' and i.op_str.startswith('0x'):
            t=int(i.op_str,16)-B
            if not (f[0]<=t<f[1]): s.add(t)
    sets.append(s); print('%s callees %d'%(a,len(s)))
c=set.intersection(*sets)
print('교집합 %d: %s'%(len(c),['%06x'%x for x in sorted(c)]))
