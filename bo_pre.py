# -*- coding: utf-8 -*-
from bo_054 import O,N,BASE
def dis_list(E,s,e):
    b=E.read(s,e-s); return [(i.address-BASE,i.mnemonic,i.op_str,bytes(i.bytes).hex()) for i in E.md.disasm(b,BASE+s)]
def pre(E,cstart,site,k=14):
    L=dis_list(E,cstart,site+16)
    idx=[i for i,x in enumerate(L) if x[0]==site]
    if not idx: return None
    i=idx[0]
    return L[max(0,i-k):i+2]
def show(oc,os_,nc,ns_,tag,k=14):
    a=pre(O,oc,os_,k); b=pre(N,nc,ns_,k)
    print(f"--- {tag}")
    for i in range(max(len(a),len(b))):
        x=a[i] if i<len(a) else None; y=b[i] if i<len(b) else None
        xs=f"{x[0]:#x} {x[1]} {x[2]}" if x else '-'
        ys=f"{y[0]:#x} {y[1]} {y[2]}" if y else '-'
        mk='' if (x and y and x[3]==y[3]) else '  <<<'
        print(f"  {xs:<46} | {ys:<46}{mk}")
