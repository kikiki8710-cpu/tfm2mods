import sys,re,difflib,collections
from h import *
RX=re.compile(r'\[(r\w+)(?: \+ r\w+\*\d)? \+ (0x[0-9a-f]+)\]')
def key(i): return i.mnemonic+'|'+re.sub(r'0x[0-9a-f]+','N',i.op_str)
def pair(o,n,want=()):
    A=insns(O,o,int(PO[o]['size'])); B=insns(N,n,int(PN[n]['size']))
    sm=difflib.SequenceMatcher(a=[key(i) for i in A],b=[key(i) for i in B],autojunk=False)
    P=collections.Counter()
    for a0,b0,sz in sm.get_matching_blocks():
        for k in range(sz):
            for (ra,da),(rb,db) in zip(RX.findall(A[a0+k].op_str),RX.findall(B[b0+k].op_str)):
                if ra not in ('rsp','rbp','rip'): P[(int(da,16),int(db,16))]+=1
    diff=[(hex(a),hex(b),c) for (a,b),c in sorted(P.items()) if a!=b]
    print('ratio %.3f  pairs %d  changed %d'%(sm.ratio(),len(P),len(diff))); print(' changed:',diff[:80])
    if want: print(' want:',[(hex(a),hex(b),c) for (a,b),c in sorted(P.items()) if a in want])
    return sm.ratio()
if __name__=='__main__':
    pair(int(sys.argv[1],16),int(sys.argv[2],16),set(int(x,16) for x in sys.argv[3:]))
