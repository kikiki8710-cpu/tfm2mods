import sys, math; sys.path.insert(0,'.')
from common import *
C=pickle.load(open('cen2.pkl','rb'))
def cls(v):
    if v<=0: 
        if v<0 and v> -10**7 and (-v)%1000==0: return 'neg%d'%v
        return None
    for d in (0,1):
        r=math.isqrt(v-d)
        if r*r==v-d and r>=10000 and r%500==0: return 'R2(%d)%s'%(r,'+1' if d else '')
    if 1000<=v<=2000000 and v%500==0 and v not in (0x1000,0x10000,0x8000,0x4000,0x2000,0x3000,0x6000,0x7000): return 'N%d'%v
    if v in (64001,160001,0xfa00,0x27100): return None
    return None
out=[]
for v,ko,kn,e in rows:
    if v!='EDITED': continue
    co,cn=C[(ko,kn)]
    a=collections.Counter(); b=collections.Counter()
    for k,c in co['imms'].items():
        t=cls(k); 
        if t: a[t]+=c
    for k,c in cn['imms'].items():
        t=cls(k)
        if t: b[t]+=c
    d=dc(a,b)
    if d: out.append((short(fo[ko]['top']),ko[0],kn[0],d))
for m,o,n,d in sorted(out):
    print('%-42s %#x->%#x: %s'%(m[-42:],o,n,' '.join('%s:%d>%d'%x for x in d)))
