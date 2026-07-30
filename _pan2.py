# -*- coding: utf-8 -*-
import sys, io, struct
import capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

EXE = sys.argv[1]
raw = open(EXE, 'rb').read()
pe = struct.unpack_from("<I", raw, 0x3c)[0]
nsec = struct.unpack_from("<H", raw, pe + 6)[0]
opt = pe + 24
IB = struct.unpack_from("<Q", raw, opt + 24)[0]
sectab = opt + struct.unpack_from("<H", raw, pe + 20)[0]
secs = []
for i in range(nsec):
    o = sectab + i * 40
    nm = raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz, va, rsz, rr = struct.unpack_from("<IIII", raw, o + 8)
    secs.append((nm, va, max(vsz, rsz), rr))

def r2o(rva):
    for nm, va, sz, rr in secs:
        if va <= rva < va + sz:
            return rr + (rva - va)
    return None

def u64(va):
    o = r2o(va - IB)
    return int.from_bytes(raw[o:o+8], 'little') if o is not None else 0

def rs(va, ml=160):
    o = r2o(va - IB)
    if o is None: return None
    b = raw[o:o+ml]
    e = b.find(b'\x00'); e = ml if e < 0 else e
    try:
        s = b[:e].decode('utf-8')
    except Exception:
        return None
    return s if s and all(9 <= ord(c) < 127 for c in s) else None

def rsl(va, ln):
    o = r2o(va - IB)
    if o is None: return None
    try:
        s = raw[o:o+ln].decode('utf-8')
    except Exception:
        return None
    return s if all(9 <= ord(c) < 127 for c in s) else None

md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True
start = int(sys.argv[2], 16)
if start < IB: start += IB
n = int(sys.argv[3], 16)
o = r2o(start - IB)
code = raw[o:o+n]

locs = []; strs = []; calls = []
for ins in md.disasm(code, start):
    if ins.mnemonic == 'lea':
        for op in ins.operands:
            if op.type == capstone.x86.X86_OP_MEM and op.mem.base == capstone.x86.X86_REG_RIP:
                t = ins.address + ins.size + op.mem.disp
                p, ln = u64(t), u64(t + 8)
                s = rsl(p, ln) if 4 < ln < 200 else None
                if s and s.endswith('.rs'):
                    oo = r2o(t + 16 - IB)
                    line = int.from_bytes(raw[oo:oo+4], 'little') if oo else 0
                    locs.append((ins.address, s, line))
                else:
                    d = rs(t)
                    if d and len(d) >= 4:
                        strs.append((ins.address, d[:90]))
    elif ins.mnemonic == 'call' and len(ins.operands) == 1 and ins.operands[0].type == capstone.x86.X86_OP_IMM:
        calls.append((ins.address, ins.operands[0].imm))

print("== %#x size %#x  [%s]" % (start - IB, n, EXE))
print("-- panic Location files:")
agg = {}
for a, f, l in locs:
    agg.setdefault(f, []).append(l)
for f, ls in sorted(agg.items(), key=lambda kv: -len(kv[1])):
    print("   %s   lines=%s (n=%d)" % (f, sorted(set(ls))[:16], len(ls)))
if '--str' in sys.argv:
    print("-- string LEAs:")
    for a, d in strs[:80]:
        print("   %#x: %r" % (a - IB, d))
if '--calls' in sys.argv:
    from collections import Counter
    print("-- direct calls:")
    c = Counter(t for _, t in calls)
    for t, k in c.most_common():
        print("   %#x x%d" % (t - IB, k))
