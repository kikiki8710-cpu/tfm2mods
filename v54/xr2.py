# -*- coding: utf-8 -*-
"""xr2.py <ver> <target_rva> — 전 .text 에서 rel32 call/jmp 참조 + 절대주소(8B) 참조 스캔"""
import io,sys,struct,bisect
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
B=0x140000000
E=load(sys.argv[1]); t=int(sys.argv[2],16)
fs=E.funcs(); starts=[x[0] for x in fs]
def fo(r):
    k=bisect.bisect_right(starts,r)-1
    if k>=0 and fs[k][0]<=r<fs[k][1]: return fs[k]
    return None
res=[]
for nm,va,vsz,ra,rsz in E.sections:
    if nm not in ('.text',): continue
    blob=E.raw[ra:ra+rsz]
    for i in range(len(blob)-5):
        b=blob[i]
        if b in (0xE8,0xE9):
            rel=struct.unpack_from('<i',blob,i+1)[0]
            src=va+i
            if src+5+rel==t: res.append(('rel32',src,'call' if b==0xE8 else 'jmp'))
print('직접 rel32 참조 %d'%len(res))
for k,src,kind in res[:40]:
    f=fo(src)
    print('  %s %06x  (fn %s)'%(kind,src,('%06x-%06x'%f) if f else '?'))
# 절대 8바이트(vtable 등)
abs_=[]
for nm,va,vsz,ra,rsz in E.sections:
    blob=E.raw[ra:ra+rsz]; tgt=struct.pack('<Q',B+t)
    o=blob.find(tgt)
    while o>=0:
        abs_.append((nm,va+o)); o=blob.find(tgt,o+1)
        if len(abs_)>40: break
print('절대주소 참조 %d: %s'%(len(abs_),[('%s:%06x'%x) for x in abs_[:20]]))
