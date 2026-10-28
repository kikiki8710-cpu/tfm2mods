from x070 import *
import pickle
FO=pickle.load(open(ROOT+'/_fnidx_065.pkl','rb')); FN=pickle.load(open(ROOT+'/_fnidx_070.pkl','rb'))
def ripdata(img,f,P,k=16):
    out=[]
    for i in insns(img,f,int(P[f]['size'])):
        m=re.search(r'\[rip ([+-]) (0x[0-9a-f]+)\]',i.op_str)
        if m:
            t=i.address+i.size+(int(m.group(2),16)*(1 if m.group(1)=='+' else -1))-0x140000000
            b=rd(img,t,k)
            out.append((i.mnemonic,t,b.hex() if b else None))
    return out
def ncall(C,f): return len(C['caller'].get(f,[]))
def info(side,f):
    P,C,img=(PO,CO,O) if side=='o' else (PN,CN,N)
    p=P.get(f)
    if not p: return '%s %x: NOT FN'%(side,f)
    return '%s %x size %d ninsn %d callers %d callees %d'%(side,f,p['size'],p['ninsn'],ncall(C,f),len(C['callee'].get(f,[])))
def same_skel_new(f):
    return FN['by_skel'].get(PO[f]['skel'],[])
def newpeer(oc):
    p=PO.get(oc)
    if not p: return None,'nofn'
    s=FN['by_skel'].get(p['skel'],[])
    if len(s)==1: return s[0],'skel'
    h=FN['by_head'].get(p['head'],[])
    if len(h)==1: return h[0],'head'
    # head collision: pick by size if unique closest equal size
    hs=[x for x in h if PN[x]['size']==p['size']]
    if len(hs)==1: return hs[0],'head+size'
    return None,'amb%d/%d'%(len(s),len(h))
def vote(f,cands):
    cands=set(cands); V=collections.Counter(); det=[]
    for oc in CO['caller'].get(f,[]):
        nc,how=newpeer(oc)
        if nc is None: V['?'+how.split('/')[0][:3]]+=1; continue
        hit=[c for c in CN['callee'].get(nc,[]) if c in cands]
        if len(hit)==1: V[hit[0]]+=1
        elif not hit: V['none']+=1
        else: V['multi']+=1
    return V
import difflib
def slotvote(f,cands=None,verbose=False):
    V=collections.Counter(); rows=[]
    for oc in CO['caller'].get(f,[]):
        nc,how=newpeer(oc)
        if nc is None: V['?']+=1; continue
        a=calls_in(O,oc,int(PO[oc]['size'])); b=calls_in(N,nc,int(PN[nc]['size']))
        idx=[i for i,(s,t) in enumerate(a) if t==f]
        if len(a)==len(b):
            for i in idx:
                t=b[i][1]; 
                if cands is None or t in cands: V[t]+=1
                else: V['off']+=1
                rows.append((oc,nc,i,'eq',t))
        else:
            # align by callee size signature
            ka=[str(PO[t]['size']) if t in PO else '?' for s,t in a]
            kb=[str(PN[t]['size']) if t in PN else '?' for s,t in b]
            sm=difflib.SequenceMatcher(a=ka,b=kb,autojunk=False); m={}
            for op,i1,i2,j1,j2 in sm.get_opcodes():
                if op in('equal','replace') and i2-i1==j2-j1:
                    for k in range(i2-i1): m[i1+k]=j1+k
            for i in idx:
                if i in m:
                    t=b[m[i]][1]
                    if cands is None or t in cands: V[t]+=1
                    else: V['off']+=1
                    rows.append((oc,nc,i,'al',t))
                else: V['nomap']+=1
    if verbose:
        for r in rows: print('   %x->%x slot%d %s -> %x'%r)
    return V
