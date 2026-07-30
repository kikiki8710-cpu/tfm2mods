# -*- coding: utf-8 -*-
# serpen_053f.py — ① provider 구조체 오프셋군 실측 대조(확정 함수 본문 disp 히스토그램)
#                  ② UIALLOC 시그니처 탐색(cmp rdx,0x11 + 같은 타깃으로 jmp/call 하는 소형 shim)
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

    def sec(self, w):
        for nm, va, vsz, rraw, rsz in self.secs:
            if nm == w:
                return va, vsz, rraw, rsz


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl")


def disps(img, rva, lo, hi):
    """함수 본문의 메모리 오퍼랜드 disp 중 [lo,hi) 히스토그램 (base=rip 제외)"""
    c = collections.Counter()
    b = img.read(rva, img.fn[rva]["size"])
    for i in md.disasm(b, rva):
        for op in i.operands:
            if op.type == 3:  # X86_OP_MEM
                if op.mem.base in (0, 41):  # none / rip
                    continue
                d = op.mem.disp
                if lo <= d < hi:
                    c[d] += 1
    return c


print("=" * 78)
print("① provider 구조체 오프셋 실측 (확정 함수 본문의 disp 히스토그램, 0xe000~0xf000)")
PAIRS = [("MOBATICK", 0x230c290, 0xeeeac0), ("SERPEN", 0x21f8ca0, 0x1535810)]
for nm, o, n in PAIRS:
    co = disps(O, o, 0xe000, 0xf000)
    cn = disps(N, n, 0xe000, 0xf000)
    print(f"\n  [{nm}] 0.5.2 {o:#x} → 0.5.3 {n:#x}")
    keys = sorted(set(co) | set(cn))
    same = diff = 0
    for k in keys:
        a, b = co.get(k, 0), cn.get(k, 0)
        mark = "  " if a and b else ("← 0.5.2만" if a else "→ 0.5.3만")
        if a and b:
            same += 1
        else:
            diff += 1
        print(f"     {k:#7x}  0.5.2={a:3d}  0.5.3={b:3d}  {mark}")
    print(f"     ⇒ 양쪽공통 {same} / 한쪽만 {diff}")

print("\n" + "=" * 78)
print("② UIALLOC — 시그니처 탐색")
va, vsz, rraw, rsz = N.sec(".text")
blob = N.raw[rraw:rraw + rsz]
# cmp rdx,0x11 ; jae  = 48 83 fa 11 73
for pat, desc in ((b"\x48\x83\xfa\x11\x73", "cmp rdx,0x11; jae rel8"),
                  (b"\x48\x83\xfa\x11\x0f\x83", "cmp rdx,0x11; jae rel32"),
                  (b"\x48\x83\xfa\x11", "cmp rdx,0x11")):
    hs = []
    i = 0
    while True:
        i = blob.find(pat, i)
        if i < 0:
            break
        hs.append(va + i); i += 1
    print(f"  '{desc}' = {len(hs)}건")
    if len(hs) <= 40:
        starts = sorted(N.fn)
        import bisect
        for h in hs:
            j = bisect.bisect_right(starts, h) - 1
            f = starts[j] if j >= 0 else None
            inside = f is not None and h < f + N.fn[f]["size"]
            if inside:
                sz = N.fn[f]["size"]
                print(f"    {h:#x} in fn {f:#x} size={sz} off=+{h-f}"
                      + ("   ★소형shim" if sz <= 140 else ""))
print()
print("  0.5.2 alloc 의 최종 타깃 0x25d9640 (실할당자) 확인:")
print(f"    0.5.2 0x25d9640 size={O.fn.get(0x25d9640,{}).get('size')} "
      f"16B={O.read(0x25d9640,16).hex(' ')}")
