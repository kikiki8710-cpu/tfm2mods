# -*- coding: utf-8 -*-
"""rd.py - RVA 위치의 원시 바이트를 여러 형식으로 덤프 (.rdata 상수표 판독용, 2026-08-05)
  python rd.py <ver> <rva> <len> [fmt]   fmt = hex|i32|i64|f32|f64|u16
"""
import io, struct, sys
try:
    sys.stdout.reconfigure(encoding='utf-8')
except Exception:
    pass
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load

ver, rva, n = sys.argv[1], int(sys.argv[2], 16), int(sys.argv[3], 0)
fmt = sys.argv[4] if len(sys.argv) > 4 else 'hex'
e = load(ver)
b = e.rd(rva, n)
if fmt == 'hex':
    for o in range(0, n, 16):
        print('%08x  %s' % (rva + o, b[o:o+16].hex(' ')))
elif fmt == 'i32':
    for o in range(0, n, 4):
        v = struct.unpack_from('<i', b, o)[0]
        print('%08x  %08x  %d' % (rva + o, v & 0xffffffff, v))
elif fmt == 'i64':
    for o in range(0, n, 8):
        v = struct.unpack_from('<q', b, o)[0]
        print('%08x  %016x  %d' % (rva + o, v & (2**64-1), v))
elif fmt == 'u16':
    for o in range(0, n, 2):
        v = struct.unpack_from('<H', b, o)[0]
        print('%08x  %04x  %d' % (rva + o, v, v))
elif fmt == 'f32':
    for o in range(0, n, 4):
        v = struct.unpack_from('<f', b, o)[0]
        print('%08x  %08x  %r' % (rva + o, struct.unpack_from('<I', b, o)[0], v))
elif fmt == 'f64':
    for o in range(0, n, 8):
        v = struct.unpack_from('<d', b, o)[0]
        print('%08x  %016x  %r' % (rva + o, struct.unpack_from('<Q', b, o)[0], v))

