# -*- coding: utf-8 -*-
# _callers.py <exe> <target_rva> : 직접 callers(e8/ff15 아님, e8 rel32 + jmp e9) + 컨테이너(.pdata) 열거
import sys, io, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
EXE = sys.argv[1]; TGT = int(sys.argv[2], 16)
raw = open(EXE, 'rb').read()
pe = struct.unpack_from("<I", raw, 0x3c)[0]
nsec = struct.unpack_from("<H", raw, pe + 6)[0]; opt = pe + 24
IB = struct.unpack_from("<Q", raw, opt + 24)[0]
sectab = opt + struct.unpack_from("<H", raw, pe + 20)[0]
secs = []
for i in range(nsec):
    o = sectab + i * 40
    nm = raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz, va, rsz, rr = struct.unpack_from("<IIII", raw, o + 8)
    secs.append((nm, va, max(vsz, rsz), rr, rsz))
magic = struct.unpack_from("<H", raw, opt)[0]
ddir = opt + (112 if magic == 0x20b else 96)
ex_rva, ex_sz = struct.unpack_from("<II", raw, ddir + 3*8)
def roff(rva):
    for nm, va, sz, rr, rsz in secs:
        if va <= rva < va + sz: return rr + (rva - va)
    return None
# .pdata ranges
po = roff(ex_rva); ranges = {}
for i in range(ex_sz // 12):
    b, e, u = struct.unpack_from("<III", raw, po + i*12)
    if e <= b or e - b > (1 << 20): continue
    if b not in ranges or e > ranges[b]: ranges[b] = e
starts = sorted(ranges)
import bisect
def container(rva):
    i = bisect.bisect_right(starts, rva) - 1
    if i >= 0 and rva < ranges[starts[i]]: return starts[i], ranges[starts[i]] - starts[i]
    return None, 0
# scan .text
txt = [s for s in secs if s[0] == '.text'][0]
nm, va, sz, rr, rsz = txt
code = raw[rr:rr+rsz]
res = []
for i in range(len(code) - 4):
    b = code[i]
    if b == 0xe8 or b == 0xe9:
        rel = int.from_bytes(code[i+1:i+5], 'little', signed=True)
        site = va + i
        tgt = site + 5 + rel
        if tgt == TGT:
            c, csz = container(site)
            res.append((site, b, c, csz))
print("target %#x : %d callers" % (TGT, len(res)))
for site, b, c, csz in res:
    print("  site %#x (%s) ret %#x | container %s size %#x" % (site, 'call' if b==0xe8 else 'jmp', site+5, ("%#x" % c) if c else "?", csz))
