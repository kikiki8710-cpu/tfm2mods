# -*- coding: utf-8 -*-
"""fstr.py <ver> <fn> — 함수가 rip-rel 로 가리키는 ASCII 문자열 열거"""
import io,sys,re
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
E=R.E3 if sys.argv[1]=='053' else R.E4
f=E.func_of(int(sys.argv[2],16))
print('fn %06x-%06x'%f)
seen=set()
for i in R.insns(E,f[0],f[1]):
    if 'rip' not in i.op_str: continue
    try:
        d=i.op_str.split('rip ')[1].split(']')[0]
        t=i.address-B+i.size+(1 if d[0]=='+' else -1)*int(d[1:].strip(),16)
    except: continue
    if t in seen: continue
    seen.add(t)
    b=E.rd(t,120)
    m=re.match(rb'[\x20-\x7e]{6,}',b)
    if m: print('  %06x  %s  <- %06x %s %s'%(t,m.group().decode('latin1')[:100],i.address-B,i.mnemonic,i.op_str))
