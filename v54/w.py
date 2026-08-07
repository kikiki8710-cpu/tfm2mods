# -*- coding: utf-8 -*-
"""window disasm: decode from function start (correct alignment), print [lo,hi)"""
import io, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import io,sys
from scan import Scanner, src_of
from pe2 import BASE
ver, lo, hi = sys.argv[1], int(sys.argv[2],16), int(sys.argv[3],16)
S = Scanner(ver)
f = S.func_of(lo)
print('fn %06x-%06x  src=%s' % (f[0], f[1], src_of(ver, f[0])[0]))
for i in S.disf(f):
    a = i.address - BASE
    if lo <= a < hi:
        print('%06x  %-22s %s %s' % (a, i.bytes.hex(), i.mnemonic, i.op_str))
