import sys,os,pickle,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from semlib import *
X=pickle.load(open(os.path.join(HERE2,'sweep2.pkl'),'rb'))
cov=pickle.load(open(os.path.join(HERE2,'cov.pkl'),'rb'))
R=X['R']
out=open(os.path.join(HERE2,'notW_sem.txt'),'w',encoding='utf-8')
n=0
for ko,(cat,kn,d,j) in sorted(R.items()):
    hasW=any(x.startswith('W') for x in cov[ko][2]+cov[ko][3])
    if hasW: continue
    sem=[x for x in d if x[0][0] in ('call','vcall','imm','f','d','x','call?')]
    if not sem and j[0]==j[1]: continue
    n+=1
    print('## %s %#x->%#x %s %dB->%dB jcc %d->%d  other-doc:%s'%(cat,ko[0],kn[0],short(fo[ko]['top']),ko[1]-ko[0],kn[1]-kn[0],j[0],j[1],cov[ko][2]+cov[ko][3]),file=out)
    for kk,a,b,oa,na in sem:
        print('   %-6s %-24s %d->%d  o:%s n:%s'%(kk[0],kk[1],a,b,','.join('%x'%x for x in oa),','.join('%x'%x for x in na)),file=out)
print(n)
