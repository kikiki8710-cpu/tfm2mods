# -*- coding: utf-8 -*-
"""nearby.py <ver> <addr> — 함수시작 기준 선형디스어셈에서 addr 를 포함하는 명령 찾기"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
B=0x140000000
E=load(sys.argv[1]); a=int(sys.argv[2],16); f=E.func_of(a)
ins=list(E.dis(f[0],f[1]-f[0]))
for k,i in enumerate(ins):
    r=i.address-B
    if r<=a<r+i.size:
        print('fn %06x-%06x : %06x 은 %06x %s %s 의 +%d 바이트'%(f[0],f[1],a,r,i.mnemonic,i.op_str,a-r))
        print('  bytes=%s size=%d'%(i.bytes.hex(),i.size))
        for q in range(max(0,k-4),min(len(ins),k+5)):
            x=ins[q]; print('   %s %06x %-20s %s %s'%('>' if q==k else ' ',x.address-B,x.bytes.hex()[:20],x.mnemonic,x.op_str))
        break
else: print('못찾음')
