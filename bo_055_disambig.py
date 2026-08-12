# -*- coding: utf-8 -*-
"""컨테이너 후보들을 difflib 정렬 비율로 판별(0.5.4 컨테이너 vs 각 0.5.5 후보)."""
import re, difflib
from bo_055 import O, N, BASE
NUM = re.compile(r'0x[0-9a-f]+|\b\d+\b')
def dis_all(E, start, end):
    b=E.read(start,end-start); out=[]
    for ins in E.md.disasm(b, BASE+start):
        out.append((ins.address-BASE, ins.mnemonic, ins.op_str, bytes(ins.bytes)))
    return out
def norm(mn,ops): return mn+' '+NUM.sub('#',ops)
def ratio(ostart,oend,nstart):
    f=N.func_of(nstart); nend = f[1] if f else nstart+(oend-ostart)
    oi=dis_all(O,ostart,oend); ni=dis_all(N,nstart,nend)
    a=[norm(m,o) for _,m,o,_ in oi]; b=[norm(m,o) for _,m,o,_ in ni]
    sm=difflib.SequenceMatcher(None,a,b,autojunk=False)
    eq=sum((i2-i1) for tag,i1,i2,j1,j2 in sm.get_opcodes() if tag=='equal')
    return eq/max(1,len(oi)), len(oi), len(ni), nend

# (name, 0.5.4 start, 0.5.4 end, [candidates])
C4 = ['0x12acaf0','0x12b0480','0x16ad220','0x16ae790','0x1bedc80','0x1bef620','0x1c532b0','0x1e4f000','0x21b6170','0x12156b0','0x1cb9dd0','0x1cbbdc0']
C8 = ['0x12adc00','0x12b0c90','0x194be40','0x1cb8640','0x1cbb1a0']
TASK = [
 ('AI1',    0x149e380,0x149ef6c, C4),
 ('AI2',    0x14a1e60,0x14a2585, C4),
 ('AITURN', 0x211dd40,0x211f2b3, C4),
 ('MATCHUI',0x237c030,0x238761b, C4),
 ('COMMIT', 0x11c04a0,0x11c0bd0, C4),
 ('AI6a',   0x215e050,0x215f377, C8),
 ('AI6b',   0x215f680,0x216042f, C4),
 ('AI6c',   0x2160680,0x2161077, C8),
 ('AI6d',   0x2161200,0x2161b5e, C4),
]
if __name__=='__main__':
    for nm,os_,oe,cands in TASK:
        scored=[]
        for c in cands:
            cv=int(c,16)
            try:
                r,no,nn,nend=ratio(os_,oe,cv)
            except Exception as ex:
                r=-1; no=nn=0
            scored.append((r,c,no,nn))
        scored.sort(reverse=True)
        best=scored[0]; second=scored[1] if len(scored)>1 else (0,'-',0,0)
        print(f"{nm:8s} 0.5.4 {os_:#x}  BEST {best[1]} r={best[0]:.3f} (oi={best[2]} ni={best[3]})  2nd {second[1]} r={second[0]:.3f}")
