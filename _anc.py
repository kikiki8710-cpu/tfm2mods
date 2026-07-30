# -*- coding: utf-8 -*-
# _anc.py <exe> <rva> <depth> : 역방향 호출 조상 트리 + 각 조상의 panic Location 파일
import sys, io, struct, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
import capstone
EXE = sys.argv[1]; ROOT = int(sys.argv[2], 16); DEPTH = int(sys.argv[3])
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
po = roff(ex_rva); ranges = {}
for i in range(ex_sz // 12):
    b, e, u = struct.unpack_from("<III", raw, po + i*12)
    if e <= b or e - b > (1 << 20): continue
    if b not in ranges or e > ranges[b]: ranges[b] = e
starts = sorted(ranges)
def container(rva):
    i = bisect.bisect_right(starts, rva) - 1
    if i >= 0 and rva < ranges[starts[i]]: return starts[i]
    return None
txt = [s for s in secs if s[0] == '.text'][0]
_, tva, tsz, trr, trsz = txt
code = raw[trr:trr+trsz]
# 전체 콜그래프 1패스
print("[scan] building call graph...", file=sys.stderr)
rev = {}
i = 0
L = len(code) - 5
while i < L:
    if code[i] == 0xe8 or code[i] == 0xe9:
        rel = int.from_bytes(code[i+1:i+5], 'little', signed=True)
        site = tva + i; tgt = site + 5 + rel
        if 0 < tgt < tva + tsz:
            rev.setdefault(tgt, []).append(site)
    i += 1
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True
def u64(va):
    o = roff(va - IB); return int.from_bytes(raw[o:o+8], 'little') if o is not None else 0
def rsl(va, ln):
    o = roff(va - IB)
    if o is None: return None
    try: s = raw[o:o+ln].decode('utf-8')
    except Exception: return None
    return s if all(9 <= ord(c) < 127 for c in s) else None
def rs(va, ml=120):
    o = roff(va - IB)
    if o is None: return None
    b = raw[o:o+ml]; e = b.find(b'\x00'); e = ml if e < 0 else e
    try: s = b[:e].decode('utf-8')
    except Exception: return None
    return s if s and all(9 <= ord(c) < 127 for c in s) else None
def info(f):
    if f is None: return "?"
    e = ranges[f]; o = roff(f)
    files = set(); strs = set()
    for ins in md.disasm(raw[o:o+(e-f)], IB + f):
        if ins.mnemonic == 'lea':
            for op in ins.operands:
                if op.type == capstone.x86.X86_OP_MEM and op.mem.base == capstone.x86.X86_REG_RIP:
                    t = ins.address + ins.size + op.mem.disp
                    p, ln = u64(t), u64(t+8)
                    s = rsl(p, ln) if 4 < ln < 200 else None
                    if s and s.endswith('.rs'):
                        oo = roff(t + 16 - IB)
                        line = int.from_bytes(raw[oo:oo+4], 'little') if oo else 0
                        files.add("%s:%d" % (s.split(chr(92))[-1], line))
                    else:
                        d = rs(t)
                        if d and 3 < len(d) < 60 and not d.startswith('Hash table'): strs.add(d)
    return "size %#x | %s | str:%s" % (e - f, sorted(files)[:6], sorted(strs)[:8])
seen = set()
def walk(f, d, pref):
    if f in seen or d > DEPTH: return
    seen.add(f)
    print("%s%#x  %s" % (pref, f, info(f)))
    cs = rev.get(f, [])
    cf = []
    for s in cs:
        c = container(s)
        cf.append((c if c is not None else s, s))
    for c, s in sorted(set(cf)):
        print("%s  ← site %#x" % (pref, s))
        walk(c, d + 1, pref + "    ")
walk(ROOT, 0, "")
