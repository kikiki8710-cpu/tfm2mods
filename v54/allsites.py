# -*- coding: utf-8 -*-
"""detour.rs 의 모든 byte-patch 사이트(base + 0xRVA)를 뽑아 053 함수/소스에 귀속시킨다."""
import io, os, re, sys, bisect, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0,'C:/tfm2mods/v54')
from pe2 import load, BASE
D='C:/tfm2mods/v54'
SEP=chr(92)
src={}
for ln in io.open(os.path.join(D,'053_srcmap.tsv'),encoding='utf-8'):
    s,e,sr,li=ln.rstrip('\n').split('\t'); src[int(s,16)]=(sr,li)
e=load('053'); fs=e.funcs(); starts=[f[0] for f in fs]
def fo(r):
    i=bisect.bisect_right(starts,r)-1
    if i>=0 and fs[i][0]<=r<fs[i][1]: return fs[i]
    return None
txt=io.open('C:/tfm2mods/tfm2_ai_adjust/src/detour.rs',encoding='utf-8').read()
lines=txt.split('\n')
# 현재 unsafe fn 이름 추적
cur='?'
sites=collections.defaultdict(list)   # fnstart -> [(rva, fnname, line, snippet)]
fnpat=re.compile(r'^\s*(?:pub\s+)?unsafe\s+fn\s+(\w+)')
rvapat=re.compile(r'base\s*\+\s*(0x[0-9a-fA-F]+)')
arrpat=re.compile(r'0x([0-9a-fA-F]{5,7})usize|0x([0-9a-fA-F]{5,7})\b')
for idx,l in enumerate(lines):
    m=fnpat.match(l)
    if m: cur=m.group(1)
    for mm in rvapat.finditer(l):
        r=int(mm.group(1),16)
        f=fo(r)
        if f: sites[f[0]].append((r,cur,idx+1,l.strip()[:100]))
    # for a in [0x...usize, 0x...] 형태
    if re.search(r'for\s+\w+\s+in\s+\[', l) or re.match(r'^\s*0x[0-9a-f]+', l.strip()):
        for mm in re.finditer(r'0x([0-9a-fA-F]{5,7})(?:usize)?', l):
            r=int(mm.group(1),16)
            if 0xc00000 <= r <= 0x1200000:
                f=fo(r)
                if f and (r,cur,idx+1) not in [(x[0],x[1],x[2]) for x in sites[f[0]]]:
                    sites[f[0]].append((r,cur,idx+1,l.strip()[:100]))
# 소스별 집계
bysrc=collections.defaultdict(list)
for fst,lst in sites.items():
    sr=src.get(fst,('(nosrc)',''))[0]
    bysrc[sr].append((fst,sorted(set(x[0] for x in lst)),sorted(set(x[1] for x in lst))))
TARGET=sys.argv[1] if len(sys.argv)>1 else None
for sr in sorted(bysrc,key=lambda k:-sum(len(x[1]) for x in bysrc[k])):
    if TARGET and TARGET.lower() not in sr.lower(): continue
    sh=' | '.join(p.split(SEP)[-1] for p in sr.split(' | '))
    n=sum(len(x[1]) for x in bysrc[sr])
    print('=== %-70s 사이트 %d ==='%(sh[:70],n))
    for fst,rvas,fns in sorted(bysrc[sr]):
        print('   fn %06x  (%d사이트)  노브함수=%s'%(fst,len(rvas),','.join(fns)))
        print('      ',' '.join('%06x'%x for x in rvas))
