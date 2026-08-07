# -*- coding: utf-8 -*-
"""&str 배열([(ptr,len)] 연속열) 스캔 → derive(Debug) enum 의 variant 이름표 찾기.
  python strarr.py 054 9        # 원소 9개짜리 배열만
"""
import io, re, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
ver = sys.argv[1]
want = int(sys.argv[2]) if len(sys.argv) > 2 else 0
e = load(ver)
IB = e.imagebase
secs = [(nm, va, max(vsz, rsz), ra, rsz) for nm, va, vsz, ra, rsz in e.sections]
def rd(rva, n):
    return e.rd(rva, n)
_, rva0, rvsz, rra, rrsz = [s for s in e.sections if s[0] == '.rdata'][0]
blob = e.raw[rra:rra + rrsz]
N = len(blob)
def getstr(p, l):
    if l < 2 or l > 40: return None
    o = e.off(p)
    if o is None: return None
    s = e.raw[o:o + l]
    try: t = s.decode('ascii')
    except: return None
    return t if re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', t) else None
i = 0
runs = []
cur = []
while i + 16 <= N:
    ptr = int.from_bytes(blob[i:i+8], 'little')
    ln = int.from_bytes(blob[i+8:i+16], 'little')
    t = getstr(ptr - IB, ln) if IB <= ptr < IB + 0x5000000 else None
    if t:
        cur.append((rva0 + i, t))
        i += 16
    else:
        if len(cur) >= 3: runs.append(cur)
        cur = []
        i += 8
if len(cur) >= 3: runs.append(cur)
for r in runs:
    if want and len(r) != want: continue
    print('%08x n=%d : %s' % (r[0][0], len(r), ' '.join(t for _, t in r)))
