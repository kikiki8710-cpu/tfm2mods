import sys,os,pickle,collections,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from semlib import *
X=pickle.load(open(os.path.join(HERE2,'sweep2.pkl'),'rb'))
H=X['H']
import cov as _c
FO,FN=full()
def crate(F,k):
    v=F.get(k)
    return short(v['top']) if v else '(no-loc)'
out=open(os.path.join(HERE2,'helpers_sem.txt'),'w',encoding='utf-8')
n=0
for ko,(kn,d,j) in sorted(H.items()):
    sem=[x for x in d if x[0][0] in ('call','vcall','imm','f','d','x','call?')]
    if not sem and j[0]==j[1]: continue
    n+=1
    m=_c.mentions(ko[0])+_c.mentions(kn[0])
    print('## %#x->%#x %s | %s %dB->%dB jcc %d->%d docs:%s'%(ko[0],kn[0],crate(FO,ko),crate(FN,kn),ko[1]-ko[0],kn[1]-kn[0],j[0],j[1],sorted(set(m))),file=out)
    for kk,a,b,oa,na in sem[:14]:
        print('   %-6s %-24s %d->%d  o:%s n:%s'%(kk[0],kk[1],a,b,','.join('%x'%x for x in oa),','.join('%x'%x for x in na)),file=out)
print(n)
