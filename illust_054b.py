# -*- coding: utf-8 -*-
# illust_054b.py — 대상 함수의 지문(크기/명령수/참조문자열/고유 imm) 덤프 (OLD/NEW 공용)
import sys, re, struct, collections
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
roff, owner, sec = B.roff, B.owner, B.sec

def rd(d, secs, rva, n):
    o = roff(secs, rva)
    return d[o:o+n] if o is not None else b""

def cstr(d, secs, rva, ml=160):
    b = rd(d, secs, rva, ml)
    if not b: return None
    e = b.find(b"\0"); e = ml if e < 0 else e
    s = b[:e]
    if len(s) < 3: return None
    try: t = s.decode("utf-8")
    except Exception: return None
    return t if all(0x20 <= ord(c) < 0x7f or ord(c) > 0x7f for c in t) else None

def rustslice(d, secs, rva):
    """(ptr,len) 쌍 형태 상수 → 문자열"""
    b = rd(d, secs, rva, 16)
    if len(b) < 16: return None
    p, L = struct.unpack("<QQ", b)
    if not (4 <= L < 200): return None
    s = rd(d, secs, p - 0x140000000, L)
    if len(s) != L: return None
    try: t = s.decode("utf-8")
    except Exception: return None
    return t if all(c.isprintable() for c in t) else None

def fingerprint(d, secs, fns, rva, tag=""):
    f = owner(fns, rva)
    if not f: return None
    o = roff(secs, f[0]); size = f[1]-f[0]
    code = d[o:o+size]
    strs, imms, calls = [], collections.Counter(), 0
    ninsn = 0
    mn = collections.Counter()
    for i in md.disasm(code, f[0]):
        ninsn += 1; mn[i.mnemonic] += 1
        if i.mnemonic == "call": calls += 1
        for op in i.operands:
            if op.type == 3 and op.mem.base == 41:  # rip
                t = i.address + i.size + op.mem.disp
                s = rustslice(d, secs, t) or cstr(d, secs, t)
                if s and 3 < len(s) < 120: strs.append((hex(t), s))
            if op.type == 2:  # imm
                v = op.imm
                if 0x1000 < abs(v) < (1<<63): imms[v] += 1
    return dict(start=f[0], size=size, ninsn=ninsn, calls=calls, strs=strs,
                imms=imms, mn=mn, head=code[:32])

if __name__ == "__main__":
    T = [("RVA_FX_SET",0x1bd8e50),("RVA_CARD_DRAW",0x1bee8e0),("RVA_ILLUST_GET",0x1e91400),
         ("RVA_SUBMIT",0x1859f0),("RVA_SUBMIT_TEXT",0x185c70),("RVA_IMG_BUILD",0x187110),
         ("RVA_IMG_UV",0x186f70),("RVA_IMG_FLAG",0x187420),("RVA_IMG_COLOR",0x23b8150),
         ("RVA_IMG_SHADER",0x188a20),("RVA_TEXT_BUILD",0x186600),("RVA_NAME_GET",0x1c19520),
         ("RVA_ASSET_GET",0x143d50),("RVA_ANIM_GET",0x888fd0),("RVA_SPRITE_CALC",0x1c1e4e0),
         ("RVA_GAME_ALLOC",0x28f7df0)]
    for nm, r in T:
        fp = fingerprint(B.DO, B.SO, B.FO, r)
        if not fp:
            print(f"{nm:16s} 0x{r:x}  ▶ .pdata 없음"); continue
        mark = "" if fp["start"]==r else f" (컨테이너 시작 0x{fp['start']:x} ≠ 지정)"
        print(f"{nm:16s} 0x{r:x}{mark}  size={fp['size']} insn={fp['ninsn']} call={fp['calls']}")
        print(f"    head24={fp['head'][:24].hex(' ')}")
        if fp["strs"]:
            print(f"    strs({len(fp['strs'])}): " + " | ".join(f"{s}" for _,s in fp["strs"][:8]))
        top = [f"{v:#x}x{c}" for v,c in fp["imms"].most_common(6)]
        print(f"    imm: " + ", ".join(top))
        print()
