# -*- coding: utf-8 -*-
# serpen_053e.py — UIALLOC(게임 힙 alloc) 확정 + 구조체 오프셋군 안정성 교차검증.
import sys, io, struct, pickle, collections, bisect
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64)

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"


class Img:
    def __init__(self, path, pkl, tag):
        self.tag = tag
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
        self.starts = sorted(self.fn)

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return rraw + (rva - va)

    def read(self, rva, n):
        o = self.roff(rva)
        return None if o is None else self.raw[o:o + n]

    def sec(self, nm_want):
        for nm, va, vsz, rraw, rsz in self.secs:
            if nm == nm_want:
                return va, vsz, rraw, rsz


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl", "0.5.2")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")

ALLOC_O = 0x25c4d30
print("=" * 78)
print("[UIALLOC] 0.5.2 원본")
b = O.read(ALLOC_O, O.fn[ALLOC_O]["size"])
for i in md.disasm(b, ALLOC_O):
    print(f"   {i.address:#x}  {i.bytes.hex(' '):<24} {i.mnemonic} {i.op_str}")

# ── ① 진입 16B 완전일치 후보 (전 .text) ────────────────────────
pre = O.read(ALLOC_O, 16)
va, vsz, rraw, rsz = N.sec(".text")
blob = N.raw[rraw:rraw + rsz]
hits = []
i = 0
while True:
    i = blob.find(pre, i)
    if i < 0:
        break
    r = va + i
    if r in N.fn:
        hits.append(r)
    i += 1
print(f"\n  진입16B 동일 & 함수시작 = {len(hits)}개")
for h in hits:
    print(f"    {h:#x} size={N.fn[h]['size']} (0.5.2 size={O.fn[ALLOC_O]['size']}) "
          f"mnem동일={N.fn[h]['mnem']==O.fn[ALLOC_O]['mnem']}")

# ── ② 마스킹 바이트 비교(rel32/disp32 무시) ─────────────────────
def masked(img, rva):
    n = img.fn[rva]["size"]
    out = bytearray()
    for ins in md.disasm(img.read(rva, n), rva):
        bb = bytearray(ins.bytes)
        if ins.mnemonic in ("call", "jmp") and bb[0] in (0xE8, 0xE9):
            bb[1:5] = b"\0\0\0\0"
        out += bb
    return bytes(out)


mo = masked(O, ALLOC_O)
exact = [h for h in hits if masked(N, h) == mo]
print(f"\n  ▶ 마스킹(call rel32 제거) 완전일치 = {[hex(h) for h in exact]}")

# ── ③ 0.5.2 dealloc/realloc 도 같은 방식으로 찾아 클러스터 확인 ──
print("\n  참고 클러스터 검증:")
for nm, orv, known in (("dealloc", 0x25c4d90, None), ("realloc", 0x25c4dd0, 0x28e3b10)):
    p = O.read(orv, 16); hs = []
    j = 0
    while True:
        j = blob.find(p, j)
        if j < 0:
            break
        if va + j in N.fn:
            hs.append(va + j)
        j += 1
    mm = masked(O, orv)
    ex = [h for h in hs if masked(N, h) == mm]
    tag = "" if known is None else f" (기확정 {known:#x} 포함={known in ex})"
    print(f"    {nm} 0.5.2={orv:#x}: 진입16B후보 {len(hs)} → 마스킹일치 {[hex(x) for x in ex]}{tag}")

# ── ④ 구조체 오프셋 안정성: disp32 리터럴 전역 등장수 대조 ────────
print("\n" + "=" * 78)
print("구조체 오프셋 안정성 (disp32 리터럴 raw 등장수, .text)")
vo, so_, ro, rso = O.sec(".text")
bo = O.raw[ro:ro + rso]
for off in (0xeab8, 0xeac0, 0xecd0, 0xecd8, 0xed18, 0xed20, 0xed28, 0xed30, 0xed38, 0xed50, 0xed58, 0x1dc0, 0x1660):
    pat = struct.pack("<I", off)
    print(f"  {off:#7x}: 0.5.2 {bo.count(pat):5d}  →  0.5.3 {blob.count(pat):5d}")
