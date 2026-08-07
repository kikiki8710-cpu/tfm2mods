# -*- coding: utf-8 -*-
"""drx.py - 데이터 RVA 로의 rip-relative 참조(lea/mov)를 .text 전역에서 찾는다.
   (capstone 선형 디스어셈이 아니라 .pdata 함수 단위 디스어셈 → 정렬 정확)
  python drx.py <ver> <data_rva> [<data_rva>...]
"""
import io, os, sys, bisect
try:
    sys.stdout.reconfigure(encoding='utf-8')
except Exception:
    pass
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner
from ls2 import rows, short_of
from pe2 import BASE
import capstone

ver = sys.argv[1]
tgts = set(int(x, 16) for x in sys.argv[2:])
S = Scanner(ver)
sm = {}
for s, e, src, l in rows(ver):
    sm[s] = (short_of(src), l)
hits = {t: [] for t in tgts}
for f in S.funcs:
    try:
        ins = S.disf(f)
    except Exception:
        continue
    for i in ins:
        for op in i.operands:
            if op.type == capstone.x86.X86_OP_MEM and op.mem.base == capstone.x86.X86_REG_RIP:
                t = (i.address - BASE) + i.size + op.mem.disp
                if t in tgts:
                    hits[t].append((i.address - BASE, f[0], i.mnemonic, i.op_str))
    S._dis.clear()
for t in sorted(tgts):
    print('=== data %06x : %d refs' % (t, len(hits[t])))
    for a, fs, m, o in hits[t]:
        src, l = sm.get(fs, ('?', '?'))
        print('   %06x  fn %06x  %s %s   | %s [%s]' % (a, fs, m, o, src[:60], l[:40]))

