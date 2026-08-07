# -*- coding: utf-8 -*-
"""bs.py <ver> <hexbytes> — .text 전역 바이트열 검색 + 소속함수"""
import io,sys,bisect
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
E=load(sys.argv[1]); pat=bytes.fromhex(sys.argv[2])
fs=E.funcs(); st=[x[0] for x in fs]
def fo(r):
    k=bisect.bisect_right(st,r)-1
    return fs[k] if k>=0 and fs[k][0]<=r<fs[k][1] else None
n=0
for nm,va,vsz,ra,rsz in E.sections:
    if nm!='.text': continue
    blob=E.raw[ra:ra+rsz]; o=blob.find(pat)
    while o>=0:
        r=va+o; f=fo(r); n+=1
        print('  %06x  fn %s'%(r,('%06x-%06x'%f) if f else '?'))
        o=blob.find(pat,o+1)
print('총 %d'%n)
