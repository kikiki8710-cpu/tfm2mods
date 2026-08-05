# -*- coding: utf-8 -*-
# illust_054d.py — 두 함수의 call 시퀀스(주소순) 정렬 비교
import struct, sys
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

def calls(d, secs, fns, r):
    f = B.owner(fns, r); o = B.roff(secs, f[0])
    code = d[o:o+(f[1]-f[0])]
    out = []
    for i in md.disasm(code, f[0]):
        if i.mnemonic == "call" and i.op_str.startswith("0x"):
            out.append((i.address, int(i.op_str, 16)))
    return out

a = calls(B.DO, B.SO, B.FO, int(sys.argv[1], 16))
b = calls(B.DN, B.SN, B.FN, int(sys.argv[2], 16))
print(f"OLD {len(a)}건 / NEW {len(b)}건")
for k in range(max(len(a), len(b))):
    x = f"0x{a[k][1]:<9x}@+{a[k][0]-a[0][0]:#06x}" if k < len(a) else "-"
    y = f"0x{b[k][1]:<9x}@+{b[k][0]-b[0][0]:#06x}" if k < len(b) else "-"
    print(f"  {k:3d}  {x:26s} → {y}")
