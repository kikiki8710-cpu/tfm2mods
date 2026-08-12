# -*- coding: utf-8 -*-
# ct_055_repin.py — 0.5.4 -> 0.5.5 마스크시그(method A) 전역유일 재핀 + pdata owner 보고.
#   ct_054.py 구조를 0.5.4(old)->0.5.5(new)로 개조. rip-rel disp / 분기 rel imm 만 와일드카드.
import struct, sys, io
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_REG_RIP, X86_OP_MEM
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

P4 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"
P5 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe"

def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
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
    return d, secs, ex, ez

def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None

def pdata(d, secs, ex, ez):
    po = roff(secs, ex)
    out = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(ez // 12)]
    out.sort(); return out

def owner(fns, rva):
    lo, hi = 0, len(fns) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if fns[m][0] <= rva < fns[m][1]: return fns[m]
        if rva < fns[m][0]: hi = m - 1
        else: lo = m + 1
    return None

def sec(secs, nm=".text"):
    for n, va, vsz, rr, rs in secs:
        if n == nm: return va, vsz, rr, rs

D4, S4, E4, Z4 = load(P4)
D5, S5, E5, Z5 = load(P5)
F4, F5 = pdata(D4, S4, E4, Z4), pdata(D5, S5, E5, Z5)
T4 = sec(S4); T5 = sec(S5)
BLOB4 = D4[T4[2]:T4[2] + T4[3]]
BLOB5 = D5[T5[2]:T5[2] + T5[3]]

def disfn(d, secs, fns, rva):
    f = owner(fns, rva)
    if not f: return None, None
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f

def ibytes(d, secs, i):
    o = roff(secs, i.address); return d[o:o + i.size]

def mask_insn(d, secs, i):
    b = bytearray(ibytes(d, secs, i)); m = [True] * len(b); enc = i.encoding
    riprel = any(op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP for op in i.operands)
    if riprel and enc.disp_size:
        for k in range(enc.disp_offset, enc.disp_offset + enc.disp_size):
            if k < len(m): m[k] = False
    if (i.mnemonic.startswith("j") or i.mnemonic == "call" or i.mnemonic.startswith("loop")) and enc.imm_size:
        for k in range(enc.imm_offset, enc.imm_offset + enc.imm_size):
            if k < len(m): m[k] = False
    return bytes(b), m

def build_sig(d, secs, ins, idx, kb, ka):
    lo = max(0, idx - kb); hi = min(len(ins), idx + ka + 1)
    B, M = bytearray(), []; site_off = None
    for j in range(lo, hi):
        if j == idx: site_off = len(B)
        b, m = mask_insn(d, secs, ins[j]); B += b; M += m
    return bytes(B), M, site_off

def scan(blob, base, sig, mask):
    best = (0, 0); cur = 0
    for i, mm in enumerate(list(mask) + [False]):
        if mm: cur += 1
        else:
            if cur > best[0]: best = (cur, i - cur)
            cur = 0
    alen, aoff = best
    if alen < 3: return []
    anchor = sig[aoff:aoff + alen]; hits = []; i = 0
    while True:
        i = blob.find(anchor, i)
        if i < 0: break
        st = i - aoff
        if st >= 0 and st + len(sig) <= len(blob):
            if all((not mask[j]) or blob[st + j] == sig[j] for j in range(len(sig))):
                hits.append(base + st)
        i += 1
    return hits

def repin(rva, orig, name="", kmin=1, kmax=12):
    """rva=0.5.4 site, orig=bytes. 0.5.5 전역유일 히트 찾기(0.5.4 자기검증 1건도 요구)."""
    ins, f = disfn(D4, S4, F4, rva)
    if ins is None: return dict(name=name, status="NOPDATA")
    idx = next((k for k, i in enumerate(ins) if i.address <= rva < i.address + i.size), None)
    if idx is None: return dict(name=name, status="BADIDX", cont=f[0])
    off_in_ins = rva - ins[idx].address
    trace = []
    for k in range(kmin, kmax + 1):
        sig, mask, so = build_sig(D4, S4, ins, idx, k, k)
        h5 = scan(BLOB5, T5[0], sig, mask)
        h4 = scan(BLOB4, T4[0], sig, mask)
        trace.append((k, len(h5), len(h4)))
        if len(h5) == 1 and len(h4) == 1:
            addr = h5[0] + so + off_in_ins
            no = roff(S5, addr); act = D5[no:no + len(orig)]
            ow = owner(F5, addr)
            return dict(name=name, status="OK" if act == orig else "OK_BYTEDIFF",
                        new=addr, orig_actual=act.hex(), want=orig.hex(), k=k,
                        cont4=f[0], cont5=ow[0] if ow else None,
                        site_insn=f"{ins[idx].mnemonic} {ins[idx].op_str}")
    sig, mask, so = build_sig(D4, S4, ins, idx, kmax, kmax)
    h5 = scan(BLOB5, T5[0], sig, mask)
    return dict(name=name, status="AMBIG" if h5 else "MISS",
                hits=[hex(h + so + off_in_ins) for h in h5[:8]], cont4=f[0], trace=trace[-3:],
                site_insn=f"{ins[idx].mnemonic} {ins[idx].op_str}")

SITES = [
    ("no_stamina_cost",     0x20ecf0c, bytes.fromhex("05")),
    ("dr_inline_a",         0x2306164, bytes.fromhex("4c0f44e2")),
    ("dr_inline_b",         0x2310c86, bytes.fromhex("4120c5")),
    ("dr_inline_d",         0x23ce6bc, bytes.fromhex("4c0f44f8")),
    ("panel_btn_daily_gate",0x23ceae2, bytes.fromhex("20c1")),
    ("daily_inc_gate",      0x20e8246, bytes.fromhex("04")),
    ("server_pregate",      0x20e5471, bytes.fromhex("04")),
    ("server_dedup_real",   0x2126f73, bytes.fromhex("0f85d3000000")),
    ("allow_dup_players",   0x2311131, bytes.fromhex("7547")),
    ("server_dedup",        0x20e42d1, bytes.fromhex("7510")),
    ("btn5v5_roster_min_a", 0x23ceae4, bytes.fromhex("4883fb0a0fb6f9b8")),
    ("btn5v5_warn_text",    0x23ce6fc, bytes.fromhex("4883ff0ab8380000")),
    ("server_roster_min",   0x2126ed0, bytes.fromhex("4801db")),
    ("roster_count_gate",   0x231e155, bytes.fromhex("0f834d010000")),
    ("collected_gate",      0x231e142, bytes.fromhex("7517")),
    ("collect_err_gate",    0x231e127, bytes.fromhex("7457")),
    ("run_push_gate",       0x231e838, bytes.fromhex("0f8412faffff")),
    ("TAKE_RVA",            0x2311d43, bytes.fromhex("14000000")),
    ("PAGE_IMM_RVA",        0x230d0ec, bytes.fromhex("05")),
]

if __name__ == "__main__":
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
    print(f"0.5.4 .text va=0x{T4[0]:x} rawsz=0x{T4[3]:x} pdata {len(F4)}")
    print(f"0.5.5 .text va=0x{T5[0]:x} rawsz=0x{T5[3]:x} pdata {len(F5)}")
    print("="*80)
    for name, rva, orig in SITES:
        r = repin(rva, orig, name)
        st = r["status"]
        if st in ("OK", "OK_BYTEDIFF"):
            c5 = f"0x{r['cont5']:x}" if r.get("cont5") else "?"
            print(f"{name:22s} 0x{rva:x} -> 0x{r['new']:x}  [{st}] k={r['k']} "
                  f"orig_new={r['orig_actual']} want={r['want']} cont5={c5}  ({r['site_insn']})")
        else:
            print(f"{name:22s} 0x{rva:x} -> [{st}] {r.get('hits','')} {r.get('trace','')} ({r.get('site_insn','')})")
