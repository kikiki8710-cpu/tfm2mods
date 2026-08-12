# -*- coding: utf-8 -*-
"""컨테이너 명령단위 정렬(0.5.4->0.5.5)."""
import re, difflib
from bo_055 import O, N, BASE
NUM = re.compile(r'0x[0-9a-f]+|\b\d+\b')
def dis_all(E, start, end):
    b=E.read(start,end-start); out=[]
    for ins in E.md.disasm(b, BASE+start):
        out.append((ins.address-BASE, ins.mnemonic, ins.op_str, bytes(ins.bytes)))
    return out
def norm(mn,ops): return mn+' '+NUM.sub('#',ops)
def align(ostart,oend,nstart,nend,verbose=False):
    oi=dis_all(O,ostart,oend); ni=dis_all(N,nstart,nend)
    a=[norm(m,o) for _,m,o,_ in oi]; b=[norm(m,o) for _,m,o,_ in ni]
    sm=difflib.SequenceMatcher(None,a,b,autojunk=False); m={}; eq=0
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag=='equal':
            eq+=(i2-i1)
            for k in range(i2-i1): m[oi[i1+k][0]]=ni[j1+k][0]
    if verbose: print(f"  align matched {eq}/{len(oi)} ({eq/max(1,len(oi)):.3f})")
    return m,oi,ni
def ctx(E,rva,n=8):
    b=E.read(rva,16*n); out=[]
    for ins in E.md.disasm(b,BASE+rva):
        out.append(f"{ins.address-BASE:#x} {bytes(ins.bytes).hex():<20} {ins.mnemonic} {ins.op_str}")
        if len(out)>=n: break
    return out
