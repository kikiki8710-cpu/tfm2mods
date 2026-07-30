# -*- coding: utf-8 -*-
# serpen_053h.py — World/provider 구조체의 "시프트 시작 경계"를 찾는다.
#   MOBATICK(rcx=World==provider) 확정쌍의 disp 히스토그램을 오프셋 오름차순으로 나란히 출력하고,
#   각 오프셋이 +0 / +0x40 중 어느 쪽에서 개수가 맞는지 표시.
import sys, io, struct, pickle, collections
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"


class Img:
    def __init__(self, path, pkl):
        d = open(path, "rb").read(); self.raw = d
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = pe + 24
        sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
            self.secs.append((nm, va, max(vsz, rsz), rraw, rsz))
        P = pickle.load(open(pkl, "rb"))["idx"]
        self.fn = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.items()}

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return rraw + (rva - va)

    def read(self, rva, n):
        o = self.roff(rva)
        return None if o is None else self.raw[o:o + n]


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl")

# 스택 프레임 오탐 제외: base 가 rsp/rbp 인 접근은 버린다
RSP, RBP = 44, 43  # capstone x86 reg ids (X86_REG_RSP / X86_REG_RBP)


def disps(img, rva):
    c = collections.Counter()
    b = img.read(rva, img.fn[rva]["size"])
    for i in md.disasm(b, rva):
        for op in i.operands:
            if op.type != 3:
                continue
            m = op.mem
            if m.base in (0, 41) or m.index != 0:
                continue
            try:
                rn = i.reg_name(m.base)
            except Exception:
                rn = ""
            if rn in ("rsp", "rbp", "esp", "ebp"):
                continue
            if m.disp > 0x30:
                c[m.disp] += 1
    return c


for nm, o, n in (("MOBATICK", 0x230c290, 0xeeeac0),):
    co, cn = disps(O, o), disps(N, n)
    print(f"[{nm}] 0.5.2 {o:#x} → 0.5.3 {n:#x}  (스택기반 접근 제외)")
    print(f"{'off':>8} {'0.5.2':>6} | {'+0':>10} | {'+0x40':>10}   판정")
    print("-" * 62)
    run0 = run40 = 0
    for d in sorted(co):
        a = co[d]
        b0 = cn.get(d, 0); b4 = cn.get(d + 0x40, 0)
        if b0 == a and b4 != a:
            v = "불변"; run0 += a
        elif b4 == a and b0 != a:
            v = "★+0x40"; run40 += a
        elif b0 == a and b4 == a:
            v = "양쪽동수(무판정)"
        else:
            v = f"불일치(+0={b0} +40={b4})"
        print(f"{d:>8x} {a:>6} | {b0:>10} | {b4:>10}   {v}")
    print(f"\n  집계: 불변확정 {run0}회 / +0x40확정 {run40}회")

# 구간별 요약
print("\n" + "=" * 62)
print("구간별 +0 vs +0x40 우세")
co, cn = disps(O, 0x230c290), disps(N, 0xeeeac0)
BUCKETS = [(0x40, 0x400), (0x400, 0x700), (0x700, 0x900), (0x900, 0x1000),
           (0x1000, 0x2000), (0x2000, 0x8000), (0x8000, 0xe000), (0xe000, 0xea00),
           (0xea00, 0xec00), (0xec00, 0xf000), (0xf000, 0x20000)]
for lo, hi in BUCKETS:
    sub = {d: c for d, c in co.items() if lo <= d < hi}
    if not sub:
        continue
    s0 = sum(min(c, cn.get(d, 0)) for d, c in sub.items())
    s4 = sum(min(c, cn.get(d + 0x40, 0)) for d, c in sub.items())
    tot = sum(sub.values())
    win = "불변" if s0 > s4 else ("★+0x40" if s4 > s0 else "동률")
    print(f"  {lo:#7x}~{hi:#7x}: old {tot:4d}  +0={s0:4d}  +0x40={s4:4d}   ⇒ {win}")
