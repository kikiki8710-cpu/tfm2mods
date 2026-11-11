# -*- coding: utf-8 -*-
"""exe .text 에서 대상 RVA 로 가는 rel32 call(E8)/jmp(E9) 를 전수 수집. 사용: callers_of.py <exe> <rva> [<rva>...]"""
import sys, struct
import numpy as np
import pefile
pe = pefile.PE(sys.argv[1], fast_load=True)
t = next(s for s in pe.sections if s.Name.rstrip(b"\0") == b".text")
data = t.get_data(); base = t.VirtualAddress
arr = np.frombuffer(data, dtype=np.uint8)
idx = np.nonzero((arr == 0xE8) | (arr == 0xE9))[0]
idx = idx[idx + 5 <= len(data)]
rel = np.frombuffer(b"".join([b""]), dtype=np.int32)
d = arr.astype(np.int64)
rel = d[idx + 1] | (d[idx + 2] << 8) | (d[idx + 3] << 16) | (d[idx + 4] << 24)
rel = np.where(rel >= 2**31, rel - 2**32, rel)
tgt = base + idx + 5 + rel
for r in sys.argv[2:]:
    r = int(r, 16)
    hits = idx[tgt == r]
    print(hex(r), len(hits), "건")
    for h in hits:
        kind = "call" if arr[h] == 0xE8 else "jmp"
        print("  %s at 0x%x  ret=0x%x" % (kind, base + h, base + h + 5))
