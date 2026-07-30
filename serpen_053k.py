# -*- coding: utf-8 -*-
# serpen_053k.py — ClientDatabase(db) 오프셋 계열의 시프트 여부를 계열 단위로 판정.
#   db 체인: scene_tag(0x1338) → payload(0x1340) → view#2(payload+0x13d8=db+0x2718)
#            → played_tick(view+0x258=db+0x2970) / tickrate(db+0x2968) / events(0x1670/78/80)
#   방법: 0.5.2에서 이 오프셋들을 쓰는 매핑된 함수 전체를 모아, 시프트 s별로
#         "대응 0.5.3 함수에서 개수가 정확히 일치하는" 사례 수를 합산.
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
        self._c = {}

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return rraw + (rva - va)

    def read(self, rva, n):
        o = self.roff(rva)
        return None if o is None else self.raw[o:o + n]

    def disps(self, rva):
        if rva in self._c:
            return self._c[rva]
        c = collections.Counter()
        b = self.read(rva, self.fn[rva]["size"])
        if b:
            for i in md.disasm(b, rva):
                for op in i.operands:
                    if op.type != 3:
                        continue
                    m = op.mem
                    if m.base in (0, 41) or m.index != 0 or m.disp <= 0x30:
                        continue
                    if i.reg_name(m.base) in ("rsp", "rbp", "esp", "ebp"):
                        continue
                    c[m.disp] += 1
        self._c[rva] = c
        return c


O = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_052.pkl")
N = Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_053.pkl")
A = pickle.load(open(r"C:\tfm2mods\_anchor_052_053.pkl", "rb"))
print(f"  앵커맵 {len(A)}쌍")

FAM = {
    "db 체인(scene/payload/events/view)": [0x1338, 0x1340, 0x1598, 0x1630, 0x1670, 0x1678,
                                          0x1680, 0x2718, 0x2968, 0x2970, 0xba0, 0x13d8],
    "provider serpen 대역": [0xeab8, 0xeac0, 0xeae8, 0xecd0, 0xecd8, 0xed18, 0xed20,
                             0xed28, 0xed30, 0xed38, 0xed50, 0xed58],
    "엔티티": [0x68, 0xb0, 0x1b8, 0x250, 0x258, 0x5a8, 0x610, 0x658, 0x670],
    "World 슬롯맵": [0x720, 0x728, 0x738, 0x740, 0x820, 0x840, 0x848, 0x8b8, 0x8c0],
    "Game": [0x1660, 0x1dc0],
}
SH = [0, 8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x60, 0x80,
      -8, -0x10, -0x18, -0x20, -0x40]

# 0.5.2 매핑함수 전수 스캔 (한 번만)
WANT = {o for v in FAM.values() for o in v}
users = collections.defaultdict(list)
for f in A:
    if f not in O.fn:
        continue
    c = O.disps(f)
    for off in WANT:
        if c.get(off):
            users[off].append((f, c[off]))

print()
for fam, offs in FAM.items():
    print("=" * 84)
    print(f"[{fam}]")
    agg = collections.Counter(); tot = 0
    per = {}
    for off in offs:
        us = users.get(off, [])
        if not us:
            per[off] = None
            continue
        tot += len(us)
        sc = {}
        for s in SH:
            k = sum(1 for f, cnt in us if N.disps(A[f]).get(off + s, 0) == cnt)
            sc[s] = k
            agg[s] += k
        per[off] = (len(us), sc)
    for off in offs:
        p = per[off]
        if p is None:
            print(f"   {off:#7x}: 사용처 없음")
            continue
        n, sc = p
        top = sorted(sc.items(), key=lambda kv: (-kv[1], abs(kv[0])))[:3]
        print(f"   {off:#7x}: n={n:4d}  " + "  ".join(f"{s:+#x}:{v}"
              for s, v in top) + f"   → {'불변' if top[0][0]==0 else f'{top[0][0]:+#x}'}")
    top = sorted(agg.items(), key=lambda kv: (-kv[1], abs(kv[0])))[:4]
    print(f"   ▶ 계열 합산(n={tot}): " + "  ".join(f"{s:+#x}:{v}" for s, v in top)
          + f"   ⇒ {'불변' if top[0][0]==0 else f'★{top[0][0]:+#x} 이동'}")
