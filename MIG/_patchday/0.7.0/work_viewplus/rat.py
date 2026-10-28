import sys,re,difflib
from adiff import *
def ratio(o,n):
    A=insns(O,o,int(PO[o]['size'])); B=insns(N,n,int(PN[n]['size']))
    a=[norm(i) for i in A]; b=[norm(i) for i in B]
    sm=difflib.SequenceMatcher(a=a,b=b,autojunk=False)
    nd=sum(max(i2-i1,j2-j1) for op,i1,i2,j1,j2 in sm.get_opcodes() if op!='equal')
    return sm.ratio(),nd,rd(O,o,16)==rd(N,n,16)
