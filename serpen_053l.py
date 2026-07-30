# -*- coding: utf-8 -*-
# serpen_053l.py — O_ENTITY_ACCESSOR 확정: SERPEN 훅 함수 안의 "간접 call [reg+disp]" 전수 비교.
#   모드는 rdx+O_ENTITY_ACCESSOR 를 읽어 함수포인터로 호출한다 ⇒ 틀리면 즉시 크래시. 정밀 확인 필수.
import sys, io, struct, pickle, collections
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True


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


O = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_052.pkl")
N = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_053.pkl")


def indirect_calls(img, rva):
    out = []
    b = img.read(rva, img.fn[rva]["size"])
    ins = list(md.disasm(b, rva))
    for k, i in enumerate(ins):
        if i.mnemonic == "call" and i.bytes[0] == 0xFF:
            ctx = " ; ".join(f"{x.mnemonic} {x.op_str}" for x in ins[max(0, k - 3):k])
            out.append((i.address - rva, i.op_str, ctx))
    return out


for nm, o, n in (("SERPEN", 0x21f8ca0, 0x1535810), ("DMGB", 0x22d2b20, 0x12c3bb0)):
    print("=" * 88)
    print(f"[{nm}] 간접 call [reg+disp]  — 0.5.2 {o:#x} vs 0.5.3 {n:#x}")
    a, b = indirect_calls(O, o), indirect_calls(N, n)
    print(f"  0.5.2 ({len(a)}건):")
    for off, ops, ctx in a:
        print(f"    +{off:<6d} call {ops:<28s} | {ctx}")
    print(f"  0.5.3 ({len(b)}건):")
    for off, ops, ctx in b:
        print(f"    +{off:<6d} call {ops:<28s} | {ctx}")

print("\n" + "=" * 88)
print("SERPEN 본문에서 0x1b8 / 0x1c8 를 쓰는 모든 명령 (문맥 포함)")
for img, rva, tag in ((O, 0x21f8ca0, "0.5.2"), (N, 0x1535810, "0.5.3")):
    b = img.read(rva, img.fn[rva]["size"])
    ins = list(md.disasm(b, rva))
    print(f"  --- {tag} {rva:#x}")
    for k, i in enumerate(ins):
        if any(f"0x{d:x}]" in i.op_str for d in (0x1b8, 0x1c8, 0x1d0)):
            pre = " ; ".join(f"{x.mnemonic} {x.op_str}" for x in ins[max(0, k - 2):k])
            post = " ; ".join(f"{x.mnemonic} {x.op_str}" for x in ins[k + 1:k + 4])
            print(f"    +{i.address-rva:<6d} {i.mnemonic} {i.op_str}")
            print(f"            앞: {pre}")
            print(f"            뒤: {post}")
