# -*- coding: utf-8 -*-
import io,re,sys
from collections import defaultdict,deque
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
P=r'C:\tfm2mods\_gaibc\m13.ll'
txt=io.open(P,encoding='utf-8',errors='replace').read().split('\n')
st=29692-1; en=33371-1
body=txt[st:en+1]; base=st
RE_LBL=re.compile(r'^(\d+):')
blocks={}; cur='entry'; curstart=1
for i,l in enumerate(body):
    m=RE_LBL.match(l)
    if m:
        blocks[cur]=(curstart,i-1); cur=m.group(1); curstart=i+1
blocks[cur]=(curstart,len(body)-1)
RE_BR=re.compile(r'\blabel %(\d+)')
succ=defaultdict(list)
for b,(a,z) in blocks.items():
    j=z
    while j>=a and not body[j].strip(): j-=1
    k=j
    while k>a-1:
        s=body[k].strip()
        if s.startswith(('br ','switch ','ret ','unreachable','invoke ','indirectbr')): break
        k-=1
    for m in RE_BR.finditer('\n'.join(body[k:j+1])): succ[b].append(m.group(1))
sw=[];k=29802-1-base
while True:
    sw.append(body[k])
    if ']' in body[k]: break
    k+=1
cases=re.findall(r'i8 (-?\d+), label %(\d+)','\n'.join(sw))
t2v=defaultdict(list)
for v,t in cases: t2v[t].append(int(v))
owner=defaultdict(set)
for t in sorted(set(t for _,t in cases)):
    seen=set();dq=deque([t])
    while dq:
        b=dq.popleft()
        if b in seen: continue
        seen.add(b)
        for s in succ.get(b,[]):
            if s not in seen: dq.append(s)
    for b in seen: owner[b].add(t)
for L in [int(x) for x in sys.argv[1:]]:
    i=L-1-base
    for b,(a,z) in blocks.items():
        if a<=i<=z:
            o=owner.get(b,set())
            print('ll %d -> block %%%s  owners=%s  vals=%s'%(L,b,sorted(o),sorted(sum((t2v[t] for t in o),[]))))
