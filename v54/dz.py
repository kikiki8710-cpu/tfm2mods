# -*- coding: utf-8 -*-
"""dz.py <ver> <start_hex> <len> — 선형 디스어셈 덤프"""
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
E=load(sys.argv[1]); s=int(sys.argv[2],16); n=int(sys.argv[3],16)
for i in E.dis(s,n):
    print('%06x %-20s %s %s'%(i.address-0x140000000,i.bytes.hex()[:20],i.mnemonic,i.op_str))
