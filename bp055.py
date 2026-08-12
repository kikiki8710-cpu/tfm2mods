# -*- coding: utf-8 -*-
# bp055.py — 0.5.4(OLD) / 0.5.5(NEW) PE 로더 공통 모듈 (bp054.py 의 055 판)
import struct, sys, io, re
try:
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
except Exception:
    pass
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

OLDP = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"
NEWP = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe"

def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex, ez = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex, ez

def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None

def pdata(d, secs, ex, ez):
    po = roff(secs, ex)
    rng = {}
    for i in range(ez // 12):
        b, e, u = struct.unpack_from("<III", d, po + i * 12)
        if e <= b or e - b > (1 << 20):
            continue
        if b not in rng or e > rng[b]:
            rng[b] = e
    return sorted(rng.items())

def owner(fns, rva):
    lo, hi = 0, len(fns) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if fns[m][0] <= rva < fns[m][1]:
            return fns[m]
        if rva < fns[m][0]:
            hi = m - 1
        else:
            lo = m + 1
    return None

def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs

DO, IBO, SO, EO, ZO = load(OLDP)
DN, IBN, SN, EN, ZN = load(NEWP)
FO, FN = pdata(DO, SO, EO, ZO), pdata(DN, SN, EN, ZN)
