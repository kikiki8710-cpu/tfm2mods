# -*- coding: utf-8 -*-
# msig054.py - masked full-body signature matcher (0.5.3 -> 0.5.4)
import pe054, sys
from capstone import *
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM, X86_GRP_JUMP, X86_GRP_CALL
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True


def masked(pe, start, end, limit=None):
    """returns (pat, mask) for body [start,end)"""
    n = end - start
    if limit: n = min(n, limit)
    code = pe.read(start, n + 16)
    pat = bytearray(); mask = bytearray()
    for ins in md.disasm(code, start):
        if len(pat) >= n: break
        wild = any(op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP for op in ins.operands)
        if (ins.group(X86_GRP_JUMP) or ins.group(X86_GRP_CALL)) and any(op.type == X86_OP_IMM for op in ins.operands):
            wild = True
        for bb in ins.bytes:
            pat.append(bb); mask.append(0 if wild else 1)
    return bytes(pat), bytes(mask)


def fixed_prefix(pat, mask, minlen=8):
    pre = bytearray()
    for b, m in zip(pat, mask):
        if m: pre.append(b)
        else: break
    return bytes(pre)


def scan(pe_new, pat, mask):
    va, blob = pe_new.text()
    pre = fixed_prefix(pat, mask)
    if len(pre) < 4:
        return None  # too weak
    hits = []
    s = 0
    while True:
        i = blob.find(pre, s)
        if i < 0: break
        seg = blob[i:i + len(pat)]
        if len(seg) == len(pat):
            ok = True
            for k in range(len(pat)):
                if mask[k] and seg[k] != pat[k]:
                    ok = False; break
            if ok:
                hits.append(va + i)
        s = i + 1
    return hits


def score_partial(pe_new, pat, mask, cand):
    """fraction of fixed bytes matching"""
    seg = pe_new.read(cand, len(pat))
    if len(seg) < len(pat): return 0.0
    tot = sum(1 for m in mask if m)
    ok = sum(1 for k in range(len(pat)) if mask[k] and seg[k] == pat[k])
    return ok / max(tot, 1)


def try_match(a, b, rva, name, limit=None):
    f = a.func_of(rva)
    if not f:
        print('%-10s 0.5.3 %08x : NOT a .pdata function body!' % (name, rva)); return None
    s, e = f
    pat, mask = masked(a, s, e, limit)
    hits = scan(b, pat, mask)
    print('%-10s 0.5.3 fn=%08x-%08x (%d B, is_start=%s) sig=%dB fixed=%d -> 0.5.4 exact hits=%s'
          % (name, s, e, e - s, rva == s, len(pat), sum(mask), [('%08x' % h) for h in (hits or [])[:6]]))
    return hits
