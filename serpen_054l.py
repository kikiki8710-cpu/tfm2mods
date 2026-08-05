# -*- coding: utf-8 -*-
# ClientDatabase(db) 오프셋 불변 검증: 0x1338+0x1340 동시참조 함수를 앵커로 사상 → 신 exe disp 확인
import sys, io, collections, pickle
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64, x86
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail=True
o=O(); n=Nw()
A = {int(k,16):int(v,16) for k,v in pickle.load(open(r"C:\tfm2mods\_anchor_053_054.pkl","rb")).items()}
SP={x86.X86_REG_RSP, x86.X86_REG_RBP, x86.X86_REG_RIP}
def ds(img, rva):
    b=img.body(rva); c=collections.Counter()
    if not b: return c
    for ins in md.disasm(b, rva):
        for op in ins.operands:
            if op.type==3 and op.mem.base and op.mem.base not in SP: c[op.mem.disp]+=1
    return c
WANT=[0x1338,0x1340,0x1598,0x1630,0x1670,0x1678,0x1680,0x2718,0x2968,0x2970,0xba0,0x13d8]
cand=[]
for r in o.fn:
    if r not in A: continue
    c=ds(o,r)
    if c.get(0x1338) and c.get(0x1340): cand.append((r,c))
print(f"0.5.3서 0x1338&0x1340 동시참조 + 앵커사상 함수 {len(cand)}개")
tot=collections.Counter(); shifted=collections.Counter()
for r,c in cand[:400]:
    cn=ds(n,A[r])
    for w in WANT:
        if c.get(w):
            tot[w]+=c[w]
            if cn.get(w)==c[w]: shifted[(w,0)]+=c[w]
            else:
                for sh in (0x8,0x10,0x20,0x30,0x40,-0x8,-0x10,-0x20):
                    if cn.get(w+sh)==c[w]: shifted[(w,sh)]+=c[w]; break
                else: shifted[(w,None)]+=c[w]
for w in WANT:
    if not tot[w]: continue
    row={sh:v for (ww,sh),v in shifted.items() if ww==w}
    print(f"  +{w:#x}: 총{tot[w]}  " + " ".join(f"시프트{('+'+hex(k)) if isinstance(k,int) and k>0 else (hex(k) if isinstance(k,int) else '불명')}={v}" for k,v in sorted(row.items(), key=lambda x:-x[1])))
