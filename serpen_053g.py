# -*- coding: utf-8 -*-
# serpen_053g.py — 확정 함수쌍의 disp 히스토그램을 구간별로 정렬해 "구조체 오프셋 시프트"를 자동 검출.
#   0.5.3은 전면 재컴파일이라 구조체 필드가 삽입/이동될 수 있다(provider 0xeaxx 대역 +0x40 확인됨).
#   모드가 쓰는 raw 오프셋 전부에 대해 시프트를 판정한다.
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


def disps(img, rva):
    c = collections.Counter()
    b = img.read(rva, img.fn[rva]["size"])
    for i in md.disasm(b, rva):
        for op in i.operands:
            if op.type == 3 and op.mem.base not in (0, 41) and op.mem.index == 0:
                d = op.mem.disp
                if d > 0x30:
                    c[d] += 1
    return c


SHIFTS = [0, 8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x60, 0x80, 0x100,
          -8, -0x10, -0x20, -0x40, -0x80]
RANGES = [("엔티티/저역 0x40~0x400", 0x40, 0x400),
          ("World 슬롯맵 0x400~0x1000", 0x400, 0x1000),
          ("db/Game 0x1000~0x8000", 0x1000, 0x8000),
          ("provider 0x8000~0x20000", 0x8000, 0x20000)]

PAIRS = [("SERPEN", 0x21f8ca0, 0x1535810), ("MOBATICK", 0x230c290, 0xeeeac0),
         ("DMGA", 0x22164a0, 0xfdbbb0), ("DMGB", 0x22d2b20, 0x12c3bb0),
         ("RENDER_STEP", 0x811500, 0x960df0), ("KEYRES", 0xc2f990, 0x1b0aba0),
         ("LAUNCHER", 0x1d96870, 0xeb8810), ("SPAWN0", 0x53aae0, 0xabdf60),
         ("씬빌더", 0x74d510, 0x997740)]

print("=" * 90)
print("구간별 최적 시프트 검출 (점수 = Σ min(cnt_old, cnt_new@d+s), 동률이면 s=0 우선)")
print("=" * 90)
AGG = {r[0]: collections.Counter() for r in RANGES}
for nm, o, n in PAIRS:
    co, cn = disps(O, o), disps(N, n)
    line = [f"  [{nm:11s}]"]
    for rn, lo, hi in RANGES:
        sub = {d: c for d, c in co.items() if lo <= d < hi}
        if not sub:
            line.append(f"{rn.split()[0]}: —")
            continue
        best = []
        for s in SHIFTS:
            sc = sum(min(c, cn.get(d + s, 0)) for d, c in sub.items())
            best.append((sc, -abs(s), s))
        best.sort(reverse=True)
        sc, _, s = best[0]
        tot = sum(sub.values())
        for ss in SHIFTS:
            AGG[rn][ss] += sum(min(c, cn.get(d + ss, 0)) for d, c in sub.items())
        line.append(f"{rn.split()[0]}: shift={s:+#x} {sc}/{tot}")
    print("  ".join(line))

print("\n  ▶ 전 함수 합산 (구간별 시프트 점수 상위 3):")
for rn, lo, hi in RANGES:
    top = sorted(AGG[rn].items(), key=lambda kv: (-kv[1], abs(kv[0])))[:3]
    print(f"    {rn:26s} " + "  ".join(f"{s:+#x}:{v}" for s, v in top))

# ── 모드가 쓰는 raw 오프셋 개별 판정 ─────────────────────────────
print("\n" + "=" * 90)
print("모드 상수별 개별 판정 (해당 오프셋을 실제로 쓰는 확정쌍에서 old vs old+s 등장수)")
CONST = [
    ("ENTITY_KIND_OFF", 0x68), ("O_ENTITY_ACCESSOR", 0x1b8), ("O_SERPEN_TEMPLATE", 0xb0),
    ("O_SPRITE_NAME_PTR", 0x250), ("O_SPRITE_NAME_LEN", 0x258), ("O_ENTITY_ID", 0x5a8),
    ("O_EXEC_MAXHP", 0x610), ("O_CUR_HP", 0x658), ("O_DMG_WINDOW", 0x670),
    ("W_CHAMP_DENSE", 0x720), ("W_CHAMP_SLOTS", 0x738), ("P_TEAM", 0x820),
    ("W_PLAYER_DENSE", 0x840), ("P_CHAMP_TAG", 0x8b8), ("P_CHAMP_KEY", 0x8c0),
    ("SCENE_TAG_OFF", 0x1338), ("LIVE_PLAYED_OFF", 0x1598), ("VIEW2_TICK_OFF", 0x1630),
    ("EV_PTR_OFF", 0x1678), ("EV_LEN_OFF", 0x1680), ("GAME_PROVIDER_OFF", 0x1dc0),
    ("Game+0x1660", 0x1660), ("VIEW_TICK_REL", 0x258),
    ("SEED_OFF", 0xeab8), ("SIM_TICK_OFF", 0xeac0), ("CAMP_SPAWN_TICK", 0xecd0),
    ("CAMP_WAVE_IDX", 0xecd8), ("KILLS_CAP", 0xed18), ("KILLS_PTR_OFF", 0xed20),
    ("KILLS_LEN_OFF", 0xed28), ("BUFF_B", 0xed30), ("BUFF_R", 0xed38),
    ("KILLS_BLUE_OFF", 0xed50), ("KILLS_RED_OFF", 0xed58),
]
CO = {nm: disps(O, o) for nm, o, n in PAIRS}
CN = {nm: disps(N, n) for nm, o, n in PAIRS}
for cname, off in CONST:
    users = [nm for nm, o, n in PAIRS if CO[nm].get(off)]
    if not users:
        print(f"  {cname:20s} {off:#7x}: 확정쌍에 미등장 — 판정불가")
        continue
    tot_o = sum(CO[u][off] for u in users)
    row = []
    for s in (0, 0x40, 0x20, 0x10, 8, -0x40):
        v = sum(CN[u].get(off + s, 0) for u in users)
        row.append((v, s))
    row.sort(key=lambda x: (-x[0], abs(x[1])))
    best_v, best_s = row[0]
    verdict = "불변" if best_s == 0 else f"★{best_s:+#x} 이동 → {off+best_s:#x}"
    print(f"  {cname:20s} {off:#7x}: old {tot_o}회({','.join(users)}) | "
          + " ".join(f"{s:+#x}:{v}" for v, s in row) + f"  ⇒ {verdict}")
