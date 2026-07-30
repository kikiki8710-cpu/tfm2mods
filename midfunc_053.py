# -*- coding: utf-8 -*-
# midfunc_053.py — mid-func 사이트(byte-patch imm·콜사이트)를 "컨테이너 안에서 원본 명령 패턴 재탐색"으로 재도출.
#   0.5.3 함수는 2~10% 커져 함수내 오프셋이 보존되지 않는다(_MIGRATE_053.md §2) ⟹ 오프셋 이전 금지.
#   방법: 0.5.2 사이트 주변 K바이트를 원본 시그로 삼아 NEW 컨테이너 전체를 스캔 → 유일하면 확정.
import struct, sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)


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
    ex_rva, ex_sz = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex_rva, ex_sz


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            off = rva - va
            return rraw + off if off < rsz else None
    return None


def pdata(d, secs, ex_rva, ex_sz):
    po = roff(secs, ex_rva)
    out = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(ex_sz // 12)]
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


DO, IBO, SO, EO, ZO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe")
DN, IBN, SN, EN, ZN = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe")
FO = pdata(DO, SO, EO, ZO)
FN = pdata(DN, SN, EN, ZN)


def ctx(d, secs, rva, back, fwd):
    o = roff(secs, rva)
    return d[o - back: o + fwd]


def find_site(name, old_site, new_fn, back=12, fwd=20, expand=(0, 4, 8)):
    """old_site 주변 (back,fwd) 바이트를 시그로 NEW 컨테이너 안을 스캔."""
    fo = owner(FO, old_site)
    fnn = owner(FN, new_fn)
    if fnn is None:
        print(f"[{name}] ✗ NEW 컨테이너 0x{new_fn:x} .pdata 없음")
        return None
    ns, ne = fnn
    no = roff(SN, ns)
    blob = DN[no: no + (ne - ns)]
    for extra in expand:
        sig = ctx(DO, SO, old_site, back + extra, fwd + extra)
        hits = [m.start() for m in re.finditer(re.escape(sig), blob)]
        if len(hits) == 1:
            site = ns + hits[0] + back + extra
            print(f"[{name}] ✓ 0x{old_site:x} → **0x{site:x}**  (컨테이너 0x{ns:x}+{site-ns:#x}, 시그 {len(sig)}B 유일)")
            return site
        if len(hits) > 1:
            continue
        if len(hits) == 0:
            break
    # 축소 재시도
    for b, f in ((8, 12), (6, 10), (5, 8)):
        sig = ctx(DO, SO, old_site, b, f)
        hits = [m.start() for m in re.finditer(re.escape(sig), blob)]
        if len(hits) == 1:
            site = ns + hits[0] + b
            print(f"[{name}] ✓ 0x{old_site:x} → **0x{site:x}**  (컨테이너 0x{ns:x}+{site-ns:#x}, 축소시그 {len(sig)}B 유일)")
            return site
        if len(hits) > 1:
            print(f"[{name}] ~ 시그{len(sig)}B 다중({len(hits)}) — 보류")
            return None
    print(f"[{name}] ✗ 미발견 (컨테이너 0x{ns:x} size={ne-ns})")
    return None


if __name__ == "__main__":
    print("== SIMUNCHUNK (rayon bridge, 원본 12B `74 a0 48 d1 eb 48 89 5d c0 48 89 f0`) ==")
    find_site("SIMUNCHUNK_RVA", 0x19b40c3, 0x25b12e0)
    o = roff(SO, 0x19b40c3)
    print(f"   OLD 사이트 12B = {DO[o:o+12].hex(' ')}")
