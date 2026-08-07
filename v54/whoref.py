# -*- coding: utf-8 -*-
"""whoref.py <ver> <ascii> — .rdata 에서 문자열 찾고, 그 주소를 rip-rel 로 참조하는 함수 열거"""
import io,sys,re,struct,bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding='utf-8')
sys.path.insert(0,r'C:\tfm2mods\v54')
from pe2 import load
E=load(sys.argv[1]); pat=sys.argv[2].encode()
fs=E.funcs(); st=[x[0] for x in fs]
def fo(r):
    k=bisect.bisect_right(st,r)-1
    return fs[k] if k>=0 and fs[k][0]<=r<fs[k][1] else None
tgts=[]
for nm,va,vsz,ra,rsz in E.sections:
    if nm not in ('.rdata','.data'): continue
    blob=E.raw[ra:ra+rsz]; o=blob.find(pat)
    while o>=0:
        tgts.append(va+o); o=blob.find(pat,o+1)
print('문자열 위치:',['%06x'%t for t in tgts])
LEA=re.compile(rb'(?:\x48\x8d|\x4c\x8d)[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]',re.S)
for nm,va,vsz,ra,rsz in E.sections:
    if nm!='.text': continue
    text=E.raw[ra:ra+rsz]
    for m in LEA.finditer(text):
        p=m.start()
        disp=struct.unpack_from('<i',text,p+3)[0]
        site=va+p; t=site+7+disp
        if t in tgts:
            f=fo(site)
            print('  ref %06x  fn %s  -> %06x'%(site,('%06x-%06x'%f) if f else '?',t))
