# -*- coding: utf-8 -*-
"""JT 원시 덤프: 지정 .rdata 주소부터 rel32 엔트리를 N개 그대로 찍는다(경계 판정용).
  python jtdump.py 054 328fed8 60 [fnlo fnhi]
"""
import io, struct, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
ver, jt, n = sys.argv[1], int(sys.argv[2], 16), int(sys.argv[3])
lo = int(sys.argv[4], 16) if len(sys.argv) > 5 else 0
hi = int(sys.argv[5], 16) if len(sys.argv) > 5 else 1 << 40
e = load(ver)
for k in range(n):
    v = e.u32(jt + 4 * k)
    d = struct.unpack('<i', struct.pack('<I', v))[0]
    t = jt + d
    print('[%2d] @%06x raw=%08x -> %06x %s' % (k, jt + 4 * k, v, t & 0xffffffff, '' if lo <= t < hi else '★밖'))
