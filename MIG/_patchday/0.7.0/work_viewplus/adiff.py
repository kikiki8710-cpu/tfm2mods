import sys,re,difflib
from h import *
def norm(i):
    s=i.op_str
    s=re.sub(r'\[rip [+-] 0x[0-9a-f]+\]','[rip]',s)
    if i.mnemonic in('call','jmp') or i.mnemonic.startswith('j'): s='T'
    return i.mnemonic+' '+s
def adiff(o,n,lo=None,hi=None):
    A=insns(O,o,int(PO[o]['size'])); B=insns(N,n,int(PN[n]['size']))
    a=[norm(i) for i in A]; b=[norm(i) for i in B]
    sm=difflib.SequenceMatcher(a=a,b=b,autojunk=False); print('ratio %.4f'%sm.ratio())
    for op,i1,i2,j1,j2 in sm.get_opcodes():
        if op=='equal': continue
        print('== %s'%op)
        for k in range(i1,i2): print('  - %x(+%x) %s'%(A[k].address-0x140000000,A[k].address-0x140000000-o,a[k]))
        for k in range(j1,j2): print('  + %x(+%x) %s'%(B[k].address-0x140000000,B[k].address-0x140000000-n,b[k]))
if __name__=='__main__': adiff(int(sys.argv[1],16),int(sys.argv[2],16))
