# -*- coding: utf-8 -*-
# 구조체 오프셋 시프트 측정: 매칭된 함수쌍의 disp 히스토그램을 대역별로 정렬 대조
import sys, io, collections, json
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
o=O(); n=Nw()
def disps(img, rva, lo, hi):
    b=img.body(rva); c=collections.Counter()
    for ins in md.disasm(b, rva):
        for op in ins.operands:
            if op.type==3 and (op.mem.base or op.mem.index):
                d=op.mem.disp
                if lo<=d<hi: c[d]+=1
    return c
PAIRS=json.loads(sys.argv[1]); LO=int(sys.argv[2],0); HI=int(sys.argv[3],0)
for nm, a, b in PAIRS:
    a=int(a,16); b=int(b,16)
    ca=disps(o,a,LO,HI); cb=disps(n,b,LO,HI)
    print("="*80); print(f"[{nm}] 053 {a:#x} → 054 {b:#x}  대역 [{LO:#x},{HI:#x})")
    la=sorted(ca.items()); lb=sorted(cb.items())
    print("  053:", [(hex(k),v) for k,v in la])
    print("  054:", [(hex(k),v) for k,v in lb])
    # 시프트 추정: 카운트 시퀀스 정렬 대조
    for sh in range(-0x80, 0x101, 8):
        ok=sum(1 for k,v in la if cb.get(k+sh)==v)
        if ok and ok>=max(1,len(la)*0.6):
            print(f"   ★시프트 {sh:+#x} 일치 {ok}/{len(la)}")
