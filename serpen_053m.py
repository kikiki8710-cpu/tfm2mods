# -*- coding: utf-8 -*-
# serpen_053m.py — UIALLOC(게임 힙 alloc(size,align)) 을 "형태"로 찾는다.
#   0.5.2 shim 특징: 소형(93B) / cmp rdx,0x11 분기 / align<=16 이면 실할당자로 tail-jmp(E9)
#                    / align>16 이면 같은 실할당자를 call(E8) 후 정렬 보정 → 즉 E9 타깃 == E8 타깃.
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


def shape(img, rva):
    """(mnem시퀀스, E8타깃집합, E9타깃집합, imm중 0x11 유무)"""
    b = img.read(rva, img.fn[rva]["size"])
    ms = []; e8 = set(); e9 = set(); has11 = False
    for i in md.disasm(b, rva):
        ms.append(i.mnemonic)
        if i.bytes[0] == 0xE8:
            e8.add(i.address + 5 + struct.unpack_from("<i", i.bytes, 1)[0])
        if i.bytes[0] == 0xE9:
            e9.add(i.address + 5 + struct.unpack_from("<i", i.bytes, 1)[0])
        for op in i.operands:
            if op.type == 2 and op.imm == 0x11:
                has11 = True
    return ms, e8, e9, has11


ms_o, e8_o, e9_o, _ = shape(O, 0x25c4d30)
print(f"[기준] 0.5.2 alloc 0x25c4d30: mnem={len(ms_o)}개 E8={[hex(x) for x in e8_o]} "
      f"E9={[hex(x) for x in e9_o]} 동일타깃={e8_o == e9_o}")
print(f"        mnem열 = {' '.join(ms_o)}")

cands = []
for f, v in N.fn.items():
    if not (70 <= v["size"] <= 130):
        continue
    m = v["mnem"]
    if m.get("call", 0) != 1 or m.get("jmp", 0) != 1 or m.get("cmp", 0) != 1:
        continue
    ms, e8, e9, has11 = shape(N, f)
    if not has11 or not e8 or not e9 or e8 != e9:
        continue
    cands.append((f, ms, e8))

print(f"\n  ▶ 형태일치 후보 {len(cands)}개 (소형 + call1/jmp1/cmp1 + imm 0x11 + E8타깃==E9타깃)")
for f, ms, e8 in cands:
    same = ms == ms_o
    print(f"    {f:#x} size={N.fn[f]['size']} mnem열동일={same} 타깃={[hex(x) for x in e8]}")
    print(f"        16B={N.read(f,16).hex(' ')}")
    if not same:
        print(f"        mnem열 = {' '.join(ms)}")

# 실할당자(0.5.2 0x25d9640) 대응 확인
A = pickle.load(open(r"C:\tfm2mods\_anchor_052_053.pkl", "rb"))
tgt_o = list(e8_o)[0]
print(f"\n  0.5.2 실할당자 {tgt_o:#x} → 앵커맵: {hex(A[tgt_o]) if tgt_o in A else '미매핑'}")
if tgt_o in A:
    t = A[tgt_o]
    print(f"    0.5.3 {t:#x} size={N.fn.get(t,{}).get('size')} 16B={N.read(t,16).hex(' ')}")
    hit = [f for f, ms, e8 in cands if t in e8]
    print(f"    ▶ 그 실할당자를 쓰는 후보 = {[hex(x) for x in hit]}")
