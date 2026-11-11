import sys,os,pickle,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import semlib
from semlib import *
X=pickle.load(open(os.path.join(HERE2,'sweep072.pkl'),'rb'))
G=X['G']; GI={v:k for k,v in G.items()}
FO,FN=full()
# all callees of game-ai functions (both sides) + of helper pairs, unmapped
def callees(img,k):
    out=set()
    for i in dis(img,k):
        if i.mnemonic in('call','jmp') and i.operands and i.operands[0].type==X86_OP_IMM:
            t=i.operands[0].imm-BASE
            if not(k[0]<=t<k[1]): out.add(t)
    return out
ro=collections.defaultdict(set); rn=collections.defaultdict(set)
for k in fo:
    for t in callees(io_,k):
        if t not in G: ro[t].add(k[0])
for k in fn:
    for t in callees(inn,k):
        if t not in GI: rn[t].add(k[0])
def info(img,F,t):
    k=img.owner(t)
    if not k: return '?',0
    return (short(F[k]['top']) if k in F else 'no-loc'), k[1]-k[0], k[0]==t
print('OLD unmapped callees of game-ai fns:',len(ro))
for t,cs in sorted(ro.items(), key=lambda x:-len(x[1])):
    i=info(io_,FO,t)
    if i[0]!='no-loc' or not i[2] or i[1]<60: continue
    print('  o %#x %s %dB callers %d %s docs:%s'%(t,i[0],i[1],len(cs),['%#x'%c for c in sorted(cs)[:3]],''))
print('NEW unmapped callees of game-ai fns:',len(rn))
for t,cs in sorted(rn.items(), key=lambda x:-len(x[1])):
    i=info(inn,FN,t)
    if i[0]!='no-loc' or not i[2] or i[1]<60: continue
    print('  n %#x %s %dB callers %d %s docs:%s'%(t,i[0],i[1],len(cs),['%#x'%c for c in sorted(cs)[:3]],''))
