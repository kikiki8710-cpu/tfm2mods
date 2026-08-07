# -*- coding: utf-8 -*-
"""findc.py <ver> <fnstart> <target> — 함수 내 call/jmp target 사이트 열거"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
E=load(sys.argv[1]); f=E.func_of(int(sys.argv[2],16)); t=int(sys.argv[3],16)+0x140000000
print('fn %06x-%06x'%(f[0],f[1]))
ins=list(E.dis(f[0],f[1]-f[0]))
for k,i in enumerate(ins):
    if i.mnemonic in ('call','jmp') and i.op_str.startswith('0x') and int(i.op_str,16)==t:
        print('  %06x %s %s'%(i.address-0x140000000,i.mnemonic,i.op_str))
        for q in range(max(0,k-3),min(len(ins),k+5)):
            x=ins[q]; print('     %s %06x %-18s %s %s'%('>' if q==k else ' ',x.address-0x140000000,x.bytes.hex()[:18],x.mnemonic,x.op_str))
