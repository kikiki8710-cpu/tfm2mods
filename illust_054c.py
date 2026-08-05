# -*- coding: utf-8 -*-
# illust_054c.py — 문자열/패닉로케이션 xref 로 0.5.4 후보 함수 찾기
import sys, re, struct, collections, bisect
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
roff, owner, sec = B.roff, B.owner, B.sec

def find_str(d, secs, s):
    """정확 문자열 바이트 위치들(rva)"""
    pat = s.encode()
    out = []
    for w in (".rdata", ".data", ".text"):
        S = sec(secs, w)
        if not S: continue
        va, vsz, rr, rs = S
        for m in re.finditer(re.escape(pat), d[rr:rr+rs]):
            out.append(va + m.start())
    return out

def find_slice_consts(d, secs, str_rvas, slen):
    """(ptr,len) 쌍 상수 위치"""
    out = []
    IB = 0x140000000
    S = sec(secs, ".rdata")
    va, vsz, rr, rs = S
    blob = d[rr:rr+rs]
    for sr in str_rvas:
        pat = struct.pack("<QQ", IB + sr, slen)
        for m in re.finditer(re.escape(pat), blob):
            out.append(va + m.start())
        pat2 = struct.pack("<Q", IB + sr)
        for m in re.finditer(re.escape(pat2), blob):
            out.append(("ptr", va + m.start()))
    return out

def lea_xrefs(d, secs, fns, targets):
    """.text 전체 선형스캔: lea reg,[rip+X] 가 targets 중 하나를 가리키는 사이트 → (site, target, owner_fn)"""
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr+rs]
    tg = set(targets)
    starts = [f[0] for f in fns]
    hits = []
    # lea 는 48 8d /r ... 여러 prefix. 간단히 전체를 스캔하며 rip-rel disp4 후보를 계산
    for i in range(len(blob)-7):
        b0 = blob[i]
        if b0 not in (0x48,0x4c,0x4d,0x49): continue
        if blob[i+1] != 0x8d: continue
        modrm = blob[i+2]
        if (modrm & 0xc7) != 0x05: continue
        disp = struct.unpack_from("<i", blob, i+3)[0]
        site = va + i
        t = site + 7 + disp
        if t in tg:
            j = bisect.bisect_right(starts, site) - 1
            fn = starts[j] if j >= 0 else None
            hits.append((site, t, fn))
    return hits

if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "new"
    D, S, F = (B.DN, B.SN, B.FN) if which == "new" else (B.DO, B.SO, B.FO)
    tag = "0.5.4" if which == "new" else "0.5.3"
    for s in sys.argv[2:]:
        rvas = find_str(D, S, s)
        print(f"[{tag}] '{s}' 문자열 위치 {len(rvas)}: " + ", ".join(hex(x) for x in rvas[:6]))
        cons = find_slice_consts(D, S, rvas, len(s))
        pairs = [c for c in cons if not isinstance(c, tuple)]
        ptrs = [c[1] for c in cons if isinstance(c, tuple)]
        print(f"    (ptr,len) 상수 {len(pairs)}: " + ", ".join(hex(x) for x in pairs[:6]))
        tgts = set(rvas) | set(pairs) | set(ptrs)
        hits = lea_xrefs(D, S, F, tgts)
        byfn = collections.Counter(h[2] for h in hits)
        print(f"    lea xref {len(hits)}건 / 함수 {len(byfn)}개:")
        for fn, c in byfn.most_common(12):
            f = owner(F, fn) if fn else None
            print(f"       fn 0x{fn:x} (size {f[1]-f[0] if f else '?'}) x{c}")
        print()
