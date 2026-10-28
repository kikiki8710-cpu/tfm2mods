import sys,re,collections
from h import *
RX=re.compile(r'\[(r\w+)(?: \+ r\w+\*\d)? \+ (0x[0-9a-f]+)\]')
def nz(i):
    s=re.sub(r'0x[0-9a-f]+','N',i.op_str)
    if i.mnemonic=='call' or i.mnemonic.startswith('j'): s='T'
    return i.mnemonic+' '+s
def ctxmatch(o,n,want,W=3):
    A=insns(O,o,int(PO[o]['size'])); B=insns(N,n,int(PN[n]['size']))
    an=[nz(i) for i in A]; bn=[nz(i) for i in B]
    idx=collections.defaultdict(list)
    for k in range(W,len(B)-W): idx[tuple(bn[k-W:k+W+1])].append(k)
    R=collections.defaultdict(collections.Counter)
    for k in range(W,len(A)-W):
        m=RX.findall(A[k].op_str)
        for reg,d in m:
            dv=int(d,16)
            if reg in('rsp','rbp','rip') or dv not in want: continue
            hits=idx.get(tuple(an[k-W:k+W+1]),[])
            if not hits: R[dv]['nomatch']+=1; continue
            for h in hits:
                mb=RX.findall(B[h].op_str)
                for rb,db in mb:
                    if rb not in('rsp','rbp','rip'): R[dv][hex(int(db,16))+('' if len(hits)==1 else '?')]+=1
    for dv in sorted(want): print('  %s -> %s'%(hex(dv),dict(R.get(dv,{}))))
if __name__=='__main__':
    ctxmatch(int(sys.argv[1],16),int(sys.argv[2],16),set(int(x,16) for x in sys.argv[4:]),int(sys.argv[3]))
