# -*- coding: utf-8 -*-
"""callat.py <ver> <fn> <site1> ... — 함수 내 call 순번 표기 + 지정 사이트의 순번"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
E=R.E3 if sys.argv[1]=='053' else R.E4
f=E.func_of(int(sys.argv[2],16))
sites=[int(x,16) for x in sys.argv[3:]]
n=0
for i in R.insns(E,f[0],f[1]):
    if i.mnemonic=='call':
        n+=1
        if (i.address-B) in sites:
            print('  #%d  %06x  %s %s'%(n,i.address-B,i.mnemonic,i.op_str))
print('총 call %d'%n)
