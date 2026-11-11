import sys,os,pickle,collections,difflib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from semlib import *
R=pickle.load(open(os.path.join(HERE2,'sweep072.pkl'),'rb'))['R']
# align call sequences to pair unmapped callees
votes=collections.defaultdict(collections.Counter)
uo=collections.Counter(); un=collections.Counter()
for ko,(cat,kn,d,j) in R.items():
    if not any(x[0][0]=='call?' for x in d): continue
    O=[x for x in toks(io_,ko,True) if x[0] in ('call','call?')]
    N=[x for x in toks(inn,kn,False) if x[0] in ('call','call?')]
    so=[x[1] if x[0]=='call' else '?' for x in O]; sn=[x[1] if x[0]=='call' else '?' for x in N]
    sm=difflib.SequenceMatcher(None,so,sn,autojunk=False)
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag=='equal':
            for x,y in zip(O[i1:i2],N[j1:j2]):
                if x[0]=='call?' and y[0]=='call?': votes[int(x[1][1:],16)][int(y[1][1:],16)]+=1
        elif tag=='replace' and i2-i1==j2-j1:
            for x,y in zip(O[i1:i2],N[j1:j2]):
                if x[0]=='call?' and y[0]=='call?': votes[int(x[1][1:],16)][int(y[1][1:],16)]+=1
    for x in O:
        if x[0]=='call?': uo[int(x[1][1:],16)]+=1
    for x in N:
        if x[0]=='call?': un[int(x[1][1:],16)]+=1
print('distinct unmapped old',len(uo),'new',len(un))
P={}
for o,c in votes.items():
    n,v=c.most_common(1)[0]
    P[o]=(n,v,sum(c.values()))
def sz(img,t):
    k=img.owner(t); return (k[1]-k[0]) if k else -1
def nm(img,t,old):
    F=full()[0 if old else 1]; k=img.owner(t)
    if k and k in F: return short(F[k]['top'])
    return ''
pickle.dump(P,open(os.path.join(HERE2,'unkpairs.pkl'),'wb'))
for o,(n,v,tot) in sorted(P.items(), key=lambda x:-uo[x[0]])[:60]:
    print('%#x(%dB %s) -> %#x(%dB %s) votes %d/%d  refs o%d n%d'%(o,sz(io_,o),nm(io_,o,True),n,sz(inn,n),nm(inn,n,False),v,tot,uo[o],un[n]))
left=[o for o in uo if o not in P]
print('unpaired old unmapped',len(left), [('%#x'%o,sz(io_,o)) for o in left[:30]])
