# -*- coding: utf-8 -*-
"""callers.py <ver> <t1,t2,...> [minsz maxsz] — 지정 대상들을 모두 call 하는 함수 찾기"""
import io,sys,re,struct,bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding='utf-8')
sys.path.insert(0,r'C:\tfm2mods\v54')
from pe2 import load
E=load(sys.argv[1]); tg=[int(x,16) for x in sys.argv[2].split(',')]
lo=int(sys.argv[3]) if len(sys.argv)>3 else 0
hi=int(sys.argv[4]) if len(sys.argv)>4 else 10**9
fs=E.funcs(); st=[x[0] for x in fs]
def fo(r):
    k=bisect.bisect_right(st,r)-1
    return fs[k] if k>=0 and fs[k][0]<=r<fs[k][1] else None
import collections
hit=collections.defaultdict(set)
for nm,va,vsz,ra,rsz in E.sections:
    if nm!='.text': continue
    blob=E.raw[ra:ra+rsz]
    for i in range(len(blob)-5):
        if blob[i]!=0xE8: continue
        rel=struct.unpack_from('<i',blob,i+1)[0]
        t=va+i+5+rel
        if t in tg:
            f=fo(va+i)
            if f: hit[f].add(t)
for f,s in sorted(hit.items()):
    if len(s)==len(tg) and lo<=f[1]-f[0]<=hi:
        print('  fn %06x-%06x (%dB)  %s'%(f[0],f[1],f[1]-f[0],['%06x'%x for x in sorted(s)]))
