# -*- coding: utf-8 -*-
# ct_054.py — tfm2_comptest_unlock 바이트패치 사이트를 0.5.3 -> 0.5.4 로 재핀.
import struct, sys, io, re, bisect, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_REG_RIP, X86_OP_IMM, X86_OP_MEM
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

P053 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
P054 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"

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
    out = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(ez // 12)]
    out.sort()
    return out

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

def sec(secs, nm=".text"):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs

D3, IB3, S3, E3, Z3 = load(P053)
D4, IB4, S4, E4, Z4 = load(P054)
F3, F4 = pdata(D3, S3, E3, Z3), pdata(D4, S4, E4, Z4)
T3 = sec(S3); T4 = sec(S4)
BLOB4 = D4[T4[2]:T4[2] + T4[3]]
BLOB3 = D3[T3[2]:T3[2] + T3[3]]

def disfn(d, secs, fns, rva):
    f = owner(fns, rva)
    if not f:
        return None, None
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f

def ibytes(d, secs, i):
    o = roff(secs, i.address)
    return d[o:o + i.size]

def mask_insn(d, secs, i):
    b = bytearray(ibytes(d, secs, i))
    m = [True] * len(b)
    enc = i.encoding
    riprel = False
    for op in i.operands:
        if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
            riprel = True
    if riprel and enc.disp_size:
        for k in range(enc.disp_offset, enc.disp_offset + enc.disp_size):
            if k < len(m): m[k] = False
    if (i.mnemonic.startswith("j") or i.mnemonic == "call" or i.mnemonic.startswith("loop")) and enc.imm_size:
        for k in range(enc.imm_offset, enc.imm_offset + enc.imm_size):
            if k < len(m): m[k] = False
    return bytes(b), m

def build_sig(d, secs, ins, idx, kb, ka):
    lo = max(0, idx - kb); hi = min(len(ins), idx + ka + 1)
    B, M = bytearray(), []
    site_off = None
    for j in range(lo, hi):
        if j == idx: site_off = len(B)
        b, m = mask_insn(d, secs, ins[j])
        B += b; M += m
    return bytes(B), M, site_off

def scan(blob, base, sig, mask):
    best = (0, 0); cur = 0
    for i, mm in enumerate(list(mask) + [False]):
        if mm: cur += 1
        else:
            if cur > best[0]: best = (cur, i - cur)
            cur = 0
    alen, aoff = best
    if alen < 3:
        return []
    anchor = sig[aoff:aoff + alen]
    hits = []; i = 0
    while True:
        i = blob.find(anchor, i)
        if i < 0: break
        st = i - aoff
        if st >= 0 and st + len(sig) <= len(blob):
            if all((not mask[j]) or blob[st + j] == sig[j] for j in range(len(sig))):
                hits.append(base + st)
        i += 1
    return hits

def repin(rva, orig, name="", kmin=1, kmax=10):
    ins, f = disfn(D3, S3, F3, rva)
    if ins is None:
        return dict(name=name, rva=rva, status="NOPDATA", hits=[], cont=None)
    idx = next((k for k, i in enumerate(ins) if i.address <= rva < i.address + i.size), None)
    if idx is None:
        return dict(name=name, rva=rva, status="BADIDX", hits=[], cont=f[0])
    off_in_ins = rva - ins[idx].address
    trace = []
    for k in range(kmin, kmax + 1):
        sig, mask, so = build_sig(D3, S3, ins, idx, k, k)
        hits = scan(BLOB4, T4[0], sig, mask)
        h3 = scan(BLOB3, T3[0], sig, mask)
        trace.append((k, len(hits), len(h3)))
        if len(hits) == 1 and len(h3) == 1:
            addr = hits[0] + so + off_in_ins
            no = roff(S4, addr)
            act = D4[no:no + len(orig)]
            return dict(name=name, rva=rva, status="OK" if act == orig else "OK_BYTEDIFF",
                        new=addr, orig_actual=act, cont=f[0], k=k, trace=trace,
                        site_insn=f"{ins[idx].mnemonic} {ins[idx].op_str}",
                        contsize=f[1]-f[0])
    sig, mask, so = build_sig(D3, S3, ins, idx, kmax, kmax)
    hits = scan(BLOB4, T4[0], sig, mask)
    return dict(name=name, rva=rva, status="AMBIG" if hits else "MISS",
                hits=[h + so + off_in_ins for h in hits[:8]], cont=f[0], trace=trace,
                site_insn=f"{ins[idx].mnemonic} {ins[idx].op_str}", contsize=f[1]-f[0])

if __name__ == "__main__":
    print(f"0.5.3 .text va=0x{T3[0]:x} rawsize=0x{T3[3]:x}  pdata {len(F3)}")
    print(f"0.5.4 .text va=0x{T4[0]:x} rawsize=0x{T4[3]:x}  pdata {len(F4)}")
