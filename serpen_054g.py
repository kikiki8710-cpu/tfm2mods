# -*- coding: utf-8 -*-
# 스택(rsp/rbp) 베이스 제외한 disp 히스토그램 대조 = 구조체 오프셋 전용
import sys, io, collections, json
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64, x86
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
o=O(); n=Nw()
SP={x86.X86_REG_RSP, x86.X86_REG_RBP, x86.X86_REG_RIP}
def disps(img, rva, lo, hi):
    b=img.body(rva); c=collections.Counter()
    for ins in md.disasm(b, rva):
        for op in ins.operands:
            if op.type==3 and op.mem.base and op.mem.base not in SP:
                d=op.mem.disp
                if lo<=d<hi: c[d]+=1
    return c
PAIRS=json.loads(sys.argv[1]); LO=int(sys.argv[2],0); HI=int(sys.argv[3],0)
for nm,a,b in PAIRS:
    a=int(a,16); b=int(b,16)
    ca=disps(o,a,LO,HI); cb=disps(n,b,LO,HI)
    print("="*80); print(f"[{nm}] 비스택 disp [{LO:#x},{HI:#x})")
    print("  053:", [(hex(k),v) for k,v in sorted(ca.items())])
    print("  054:", [(hex(k),v) for k,v in sorted(cb.items())])
    same=sum(v for k,v in ca.items() if cb.get(k)==v)
    print(f"  동일 오프셋·동일횟수 비율 {same}/{sum(ca.values())}")
