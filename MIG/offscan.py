#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""offscan.py — .pdata 함수 전수에서 「메모리 오퍼랜드 변위(disp)·즉치 집합」을 모두 가진 함수를 찾는다. (2026-09-13 신설)

왜: 패닉 Location 이 없는 작은 함수(예: `BattlePlan::with_runaway` — IR alloc refs 0)는 fnprobe/locfind/aimap 으로
   exe 짝을 못 찾는다. 그런 함수는 IR 본문의 gep 변위(self+64/+192/+255/+246, context→+4856)가 지문이다.
   지문 = IR gep 상수(10진) 를 그대로 넣는다. 단독 판정 아님(추정) — fnprobe 로 인자·호출자를 확인해 확정.

사용: python -X utf8 MIG\\offscan.py 64 192 255 246 4856 [--max 2000] [--imm 12345 ...]
"""
import sys, struct, pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"

def main():
    a = sys.argv[1:]
    mx = 4000
    if "--max" in a:
        k = a.index("--max"); mx = int(a[k + 1]); del a[k:k + 2]  # ⚠초판은 --max 값이 disps 에 섞여 전부 0건(09-13)
    imms = set()
    if "--imm" in a:
        imms = set(int(x) for x in a[a.index("--imm") + 1:] if x.isdigit())
        a = a[:a.index("--imm")]
    disps = set(int(x) for x in a if x.isdigit())
    pe = pefile.PE(EXE, fast_load=True); img = pe.get_memory_mapped_image()
    pd = [s for s in pe.sections if s.Name.startswith(b'.pdata')][0]
    pdata = img[pd.VirtualAddress: pd.VirtualAddress + pd.Misc_VirtualSize]
    md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
    hits = []
    for i in range(0, len(pdata) - 11, 12):
        b, e, u = struct.unpack_from('<III', pdata, i)
        if b == 0: break
        if e - b > mx: continue
        seen_d, seen_i = set(), set()
        for ins in md.disasm(img[b:e], b):
            for op in ins.operands:
                if op.type == X86_OP_MEM and op.mem.disp in disps: seen_d.add(op.mem.disp)
                if op.type == X86_OP_IMM and op.imm in imms: seen_i.add(op.imm)
        if seen_d == disps and seen_i == imms:
            hits.append((b, e - b))
    for b, n in hits: print("0x%x  %dB" % (b, n))
    print("hits", len(hits))

if __name__ == "__main__":
    main()
