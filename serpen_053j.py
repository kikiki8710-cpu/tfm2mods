# -*- coding: utf-8 -*-
# serpen_053j.py — 애매 오프셋을 "모드가 실제로 훅하는 함수" 안에서만 직접 대조.
import sys, io, struct, pickle, collections, bisect
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

PAIRS = [("SERPEN", 0x21f8ca0, 0x1535810), ("MOBATICK", 0x230c290, 0xeeeac0),
         ("DMGB", 0x22d2b20, 0x12c3bb0), ("DMGA", 0x22164a0, 0xfdbbb0),
         ("RENDER_STEP", 0x811500, 0x960df0), ("LAUNCHER", 0x1d96870, 0xeb8810),
         ("RUNNER_CTOR", 0x1d981e0, 0xeba490), ("ARG_STR", 0xfef190, 0x1228a90),
         ("SPAWN0", 0x53aae0, 0xabdf60), ("SPAWN1", 0x539f40, 0xabd340),
         ("KEYRES", 0xc2f990, 0x1b0aba0), ("씬빌더", 0x74d510, 0x997740),
         ("리플레이핸들러", 0x1554930, 0x229a410)]

TARG = [(0x1b8, "O_ENTITY_ACCESSOR"), (0x1338, "SCENE_TAG_OFF"), (0x1598, "LIVE_PLAYED_OFF"),
        (0x1630, "VIEW2_TICK_OFF"), (0x1670, "EV_CAP"), (0x1678, "EV_PTR_OFF"),
        (0x1680, "EV_LEN_OFF"), (0x1660, "Game+0x1660"), (0x1dc0, "GAME_PROVIDER_OFF"),
        (0x258, "VIEW_TICK_REL"), (0x6a8, "CHAMP_STRIDE"), (0x8d0, "PLAYER_STRIDE")]

for off, name in TARG:
    rows = []
    for nm, o, n in PAIRS:
        co = O.disps(o)
        if not co.get(off):
            continue
        cn = N.disps(n)
        rows.append((nm, co[off], [cn.get(off + s, 0) for s in (0, 8, 0x10, 0x18, 0x20, 0x40, -0x10)]))
    print(f"== {name} {off:#x}")
    if not rows:
        print("   (훅 대상 함수에 미등장)")
        continue
    print(f"   {'함수':12s} {'old':>4s} | {'+0':>4s} {'+8':>4s} {'+10':>4s} {'+18':>4s} {'+20':>4s} {'+40':>4s} {'-10':>4s}")
    for nm, c, v in rows:
        print(f"   {nm:12s} {c:>4d} | " + " ".join(f"{x:>4d}" for x in v))
print()
print("== 스트라이드(배열 간격) 확인: imul/lea 상수는 disp가 아니라 imm ⇒ 별도 스캔")
for nm, o, n in (("MOBATICK", 0x230c290, 0xeeeac0), ("씬빌더", 0x74d510, 0x997740)):
    for img, rva, tag in ((O, o, "0.5.2"), (N, n, "0.5.3")):
        b = img.read(rva, img.fn[rva]["size"])
        cnt = collections.Counter()
        for i in md.disasm(b, rva):
            if i.mnemonic in ("imul", "add", "lea", "mov", "cmp", "shl"):
                for op in i.operands:
                    if op.type == 2 and 0x100 <= op.imm <= 0x2000:  # IMM
                        cnt[op.imm] += 1
        keep = {k: v for k, v in cnt.items() if k in (0x6a8, 0x6b0, 0x6a0, 0x8d0, 0x8d8, 0x8c8, 0x6c0, 0x8e0)}
        print(f"   {nm} {tag}: " + (str({hex(k): v for k, v in sorted(keep.items())}) if keep else "없음"))
