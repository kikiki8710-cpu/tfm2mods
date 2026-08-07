# -*- coding: utf-8 -*-
"""memouse.py — 0xc797c0(defense_nexus 3술어 메모) 호출 직후 마스크 테스트 전수.
사용: python memouse.py <ver> <memo_rva>"""
import sys, re, struct, bisect
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
from pe2 import BASE
ver, tgt = sys.argv[1], int(sys.argv[2],16)
S = Scanner(ver)
sites=[]
for m in re.finditer(re.escape(b'\xe8'), S.body):
    o=m.start()
    if o+5>len(S.body): continue
    d=struct.unpack_from('<i',S.body,o+1)[0]
    if S.tva+o+5+d==tgt: sites.append(S.tva+o)
for s in sorted(sites):
    f=S.func_of(s)
    src=src_of(ver,f[0])[0] or ''
    print('--- callsite %06x  fn %06x  %s'%(s,f[0],src[:70]))
    ins=S.disf(f); k=[j for j,i in enumerate(ins) if i.address-BASE==s]
    if not k: print('    (align miss)'); continue
    for i in ins[k[0]+1:k[0]+5]:
        print('    %06x  %-20s %s %s'%(i.address-BASE,i.bytes.hex(),i.mnemonic,i.op_str))
