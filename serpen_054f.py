# -*- coding: utf-8 -*-
import sys, io, collections, json
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
o=O(); n=Nw()
def imms(img, rva, lo, hi):
    b=img.body(rva); c=collections.Counter()
    for ins in md.disasm(b, rva):
        for op in ins.operands:
            if op.type==2 and lo<=op.imm<hi: c[op.imm]+=1
    return c
PAIRS=json.loads(sys.argv[1]); LO=int(sys.argv[2],0); HI=int(sys.argv[3],0)
for nm,a,b in PAIRS:
    a=int(a,16); b=int(b,16)
    ca=imms(o,a,LO,HI); cb=imms(n,b,LO,HI)
    print("="*80); print(f"[{nm}] imm 대역 [{LO:#x},{HI:#x})")
    print("  053:", [(hex(k),v) for k,v in sorted(ca.items())])
    print("  054:", [(hex(k),v) for k,v in sorted(cb.items())])
