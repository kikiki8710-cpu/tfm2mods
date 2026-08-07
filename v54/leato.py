# -*- coding: utf-8 -*-
"""특정 .rdata 주소를 가리키는 lea reg,[rip+d32] 사이트 전수(JT 베이스 역추적).
  python leato.py 054 328ff44
"""
import io, re, struct, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
ver = sys.argv[1]
e = load(ver)
_, tva, tvsz, tra, trsz = [s for s in e.sections if s[0] == '.text'][0]
body = e.raw[tra:tra + trsz]
for t in sys.argv[2:]:
    tgt = int(t, 16)
    print('=== lea -> %06x' % tgt)
    n = 0
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body):
        o = m.start()
        d = struct.unpack_from('<i', body, o + 3)[0]
        if tva + o + 7 + d == tgt:
            f = e.func_of(tva + o)
            print('   %06x  (fn %s)  %s' % (tva + o, ('%06x' % f[0]) if f else '?', body[o:o+7].hex()))
            n += 1
    print('   -- %d건' % n)
