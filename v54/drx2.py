# -*- coding: utf-8 -*-
"""drx2.py - 데이터 RVA 로의 rip-relative 참조를 numpy 로 전수 탐색(빠름).
   .text 전 바이트오프셋에서 disp32 == target-(next_addr) 인 자리를 찾고,
   그 자리를 포함하는 함수를 .pdata 로 역인용 → 함수 시작에서 재디스어셈해 명령 확정.
  python drx2.py <ver> <data_rva> [<data_rva>...]
"""
import io, os, sys, bisect
import numpy as np
try:
    sys.stdout.reconfigure(encoding='utf-8')
except Exception:
    pass
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
import ls2
from pe2 import BASE

ver = sys.argv[1]
tgts = [int(x, 16) for x in sys.argv[2:]]
S = Scanner(ver)
b = np.frombuffer(S.body, dtype=np.uint8)
n = len(b) - 4
v = (b[0:n].astype(np.uint32) | (b[1:n+1].astype(np.uint32) << 8)
     | (b[2:n+2].astype(np.uint32) << 16) | (b[3:n+3].astype(np.uint32) << 24))
idx = np.arange(n, dtype=np.uint32)
sm = {}
for s, e, src, l in ls2.rows(ver):
    sm[s] = (ls2.short_of(src), l)
for t in tgts:
    req = (np.uint32(t - S.tva - 4) - idx).astype(np.uint32)
    cand = np.nonzero(v == req)[0]
    print('=== data %06x : raw cand %d' % (t, len(cand)))
    seen = set()
    for o in cand:
        rva = S.tva + int(o)
        f = S.func_of(rva)
        if not f:
            continue
        for i in S.disf(f):
            a = i.address - BASE
            if a <= rva < a + i.size:
                if a + i.size == rva + 4 and a not in seen:
                    seen.add(a)
                    src, l = sm.get(f[0], ('?', '?'))
                    print('   %06x  fn %06x  %-6s %-34s | %s [%s]'
                          % (a, f[0], i.mnemonic, i.op_str, src[:58], l[:40]))
                break
    print('   -- 확정 %d건' % len(seen))

