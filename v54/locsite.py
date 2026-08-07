# -*- coding: utf-8 -*-
"""함수 구간 안에서 Rust Location(패닉 위치) 를 집는 lea 사이트를 **주소별로** 나열.
srcmap 은 함수 단위 집계라 '어느 arm 이 어느 소스인지' 를 못 준다 — 이건 준다.
  python locsite.py 054 e145b0 e14979
"""
import io, re, sys, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load

SRC_PAT = re.compile(rb'[A-Za-z0-9_\-]+[\\/](?:[A-Za-z0-9_\-]+[\\/])*[A-Za-z0-9_\-]+\.rs')
_C = {}


def locmap(e):
    if e.ver in _C:
        return _C[e.ver]
    _, rd_va, _, rd_ra, rd_rsz = [s for s in e.sections if s[0] == '.rdata'][0]
    blob = e.raw[rd_ra:rd_ra + rd_rsz]
    str_at = {rd_va + m.start(): m.group().decode('latin1') for m in SRC_PAT.finditer(blob)}
    d = {}
    for i in range(0, len(blob) - 24, 8):
        ptr = int.from_bytes(blob[i:i + 8], 'little')
        if ptr < e.imagebase or ptr > e.imagebase + 0x5000000:
            continue
        s = str_at.get(ptr - e.imagebase)
        if not s or int.from_bytes(blob[i + 8:i + 16], 'little') != len(s):
            continue
        line = int.from_bytes(blob[i + 16:i + 20], 'little')
        if 0 < line < 100000:
            d[rd_va + i] = (s, line)
    _C[e.ver] = d
    return d


def sites(e, lo, hi):
    d = locmap(e)
    _, t_va, _, t_ra, t_rsz = [s for s in e.sections if s[0] == '.text'][0]
    body = e.raw[t_ra:t_ra + t_rsz]
    out = []
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body):
        o = m.start()
        site = t_va + o
        if not (lo <= site < hi):
            continue
        disp = int.from_bytes(body[o + 3:o + 7], 'little', signed=True)
        t = t_va + o + 7 + disp
        if t in d:
            out.append((site, d[t][0], d[t][1]))
    return out


if __name__ == '__main__':
    e = load(sys.argv[1])
    for a, s, l in sites(e, int(sys.argv[2], 16), int(sys.argv[3], 16)):
        print('%06x  %s:%d' % (a, s, l))
