# -*- coding: utf-8 -*-
# serpen_053i.py — 모드가 쓰는 raw 구조체 오프셋을 상수별로 확증.
#   방법: 0.5.2에서 그 오프셋을 실제로 쓰는 함수들을 모두 찾고 → 전파 매칭맵으로 0.5.3 대응함수를 구한 뒤
#         대응함수에서 off / off+shift 중 어느 쪽이 같은 횟수로 나타나는지 집계(다수결).
#   앵커맵은 pickle 캐시.
import sys, io, struct, pickle, collections, bisect, math, os
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
CACHE = r"C:\tfm2mods\_anchor_052_053.pkl"


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
        self._dc = {}

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

    def owner(self, rva):
        i = bisect.bisect_right(self.starts, rva) - 1
        if i < 0:
            return None
        s = self.starts[i]
        return s if rva < s + self.fn[s]["size"] else None

    def disps(self, rva):
        if rva in self._dc:
            return self._dc[rva]
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
        self._dc[rva] = c
        return c

    def build_graph(self):
        va, vsz, rraw, rsz = self.sec(".text")
        blob = self.raw[rraw:rraw + rsz]
        self.caller_fn = collections.defaultdict(collections.Counter)
        self.callee = collections.defaultdict(collections.Counter)
        i = 0; n = len(blob)
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > n:
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i; tgt = site + 5 + rel
            if tgt in self.fn:
                w = self.owner(site)
                if w is not None:
                    self.caller_fn[tgt][w] += 1
                    self.callee[w][tgt] += 1
            i += 1


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl", "0.5.2")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")

if os.path.exists(CACHE):
    A = pickle.load(open(CACHE, "rb"))
    print(f"  앵커맵 캐시 로드 {len(A)}쌍")
else:
    O.build_graph(); N.build_graph()
    so = collections.defaultdict(list); sn = collections.defaultdict(list)
    for r, v in O.fn.items():
        so[v["skel"]].append(r)
    for r, v in N.fn.items():
        sn[v["skel"]].append(r)
    A = {vs[0]: sn[k][0] for k, vs in so.items() if len(vs) == 1 and len(sn.get(k, ())) == 1}
    A.update({0x21f8ca0: 0x1535810, 0x53aae0: 0xabdf60, 0x539f40: 0xabd340, 0x5ac950: 0x91ab0,
              0x24b5a00: 0x1a6530, 0x811500: 0x960df0, 0x22164a0: 0xfdbbb0, 0x22d2b20: 0x12c3bb0,
              0xc2f990: 0x1b0aba0, 0x1d96870: 0xeb8810, 0x74d510: 0x997740, 0x1554930: 0x229a410,
              0x25c4dd0: 0x28e3b10, 0x230c290: 0xeeeac0, 0xfef190: 0x1228a90, 0x1d981e0: 0xeba490})
    used = set(A.values())

    def cosf(a, b):
        X, Y = O.fn[a]["mnem"], N.fn[b]["mnem"]
        ks = set(X) | set(Y)
        da = math.sqrt(sum(X.get(k, 0) ** 2 for k in ks)); db = math.sqrt(sum(Y.get(k, 0) ** 2 for k in ks))
        return 0.0 if not da or not db else sum(X.get(k, 0) * Y.get(k, 0) for k in ks) / (da * db)

    for rnd in range(3):
        added = 0
        for f in [x for x in O.fn if x not in A and (O.caller_fn.get(x) or O.callee.get(x))]:
            tot = collections.Counter(); nb = 0
            for cf, cnt in O.caller_fn.get(f, {}).items():
                if cf in A:
                    nb += 1
                    for t, k in N.callee.get(A[cf], {}).items():
                        if k == cnt:
                            tot[t] += 1
            for ce, cnt in O.callee.get(f, {}).items():
                if ce in A:
                    nb += 1
                    for t, k in N.caller_fn.get(A[ce], {}).items():
                        if k == cnt:
                            tot[t] += 1
            if nb < 2 or not tot:
                continue
            b = tot.most_common(2); t, v = b[0]
            if v < 2 or (len(b) > 1 and b[1][1] >= v) or t in used or t not in N.fn:
                continue
            r = N.fn[t]["size"] / max(1, O.fn[f]["size"])
            if not (0.6 <= r <= 1.8) or cosf(f, t) < 0.93:
                continue
            A[f] = t; used.add(t); added += 1
        print(f"  전파 라운드 {rnd+1}: +{added} → {len(A)}")
        if not added:
            break
    pickle.dump(A, open(CACHE, "wb"))
    print(f"  앵커맵 저장 {len(A)}쌍")

# 0.5.2 전 함수의 disp 인덱스 (필요한 오프셋만)
CONST = [
    ("ENTITY_KIND_OFF", 0x68), ("O_SERPEN_TEMPLATE", 0xb0), ("O_ENTITY_ACCESSOR", 0x1b8),
    ("O_SPRITE_NAME_PTR", 0x250), ("O_SPRITE_NAME_LEN/VIEW_TICK_REL", 0x258),
    ("O_ENTITY_ID", 0x5a8), ("O_EXEC_MAXHP", 0x610), ("O_CUR_HP", 0x658), ("O_DMG_WINDOW", 0x670),
    ("W_CHAMP_DENSE", 0x720), ("W_CHAMP_SLOTS", 0x738), ("P_TEAM", 0x820), ("W_PLAYER_DENSE", 0x840),
    ("P_CHAMP_TAG", 0x8b8), ("P_CHAMP_KEY", 0x8c0),
    ("SCENE_TAG_OFF", 0x1338), ("LIVE_PLAYED_OFF", 0x1598), ("VIEW2_TICK_OFF", 0x1630),
    ("EV_CAP", 0x1670), ("EV_PTR_OFF", 0x1678), ("EV_LEN_OFF", 0x1680),
    ("Game+0x1660", 0x1660), ("GAME_PROVIDER_OFF", 0x1dc0),
    ("SEED_OFF", 0xeab8), ("SIM_TICK_OFF", 0xeac0), ("W+0xeae8(세트플래그)", 0xeae8),
    ("CAMP_SPAWN_TICK", 0xecd0), ("CAMP_WAVE_IDX", 0xecd8),
    ("KILLS_CAP", 0xed18), ("KILLS_PTR_OFF", 0xed20), ("KILLS_LEN_OFF", 0xed28),
    ("BUFF_B", 0xed30), ("BUFF_R", 0xed38), ("KILLS_BLUE_OFF", 0xed50), ("KILLS_RED_OFF", 0xed58),
]
WANT = {off for _, off in CONST}
print("\n  0.5.2 함수별 disp 스캔 중...")
users = collections.defaultdict(list)   # off -> [(fn, cnt)]
for f in O.fn:
    if f not in A:
        continue
    c = O.disps(f)
    for off in WANT:
        if c.get(off):
            users[off].append((f, c[off]))
print("  완료\n")

SH = (0, 0x40, 0x20, 0x10, 8, -8, -0x10, -0x20, -0x40, 0x80)
print("=" * 96)
print(f"{'상수':32s} {'0.5.2':>7s} {'쓰는함수':>6s}  시프트별 '대응함수에서 동일횟수' 함수 수 → 판정")
print("=" * 96)
for cname, off in CONST:
    us = users.get(off, [])
    if not us:
        print(f"  {cname:30s} {off:#7x}  — 매핑된 사용처 없음(판정불가)")
        continue
    score = {}
    for s in SH:
        k = 0
        for f, cnt in us:
            if N.disps(A[f]).get(off + s, 0) == cnt:
                k += 1
        score[s] = k
    best = sorted(score.items(), key=lambda kv: (-kv[1], abs(kv[0])))
    bs, bv = best[0]
    n = len(us)
    tail = " ".join(f"{s:+#x}:{v}" for s, v in best[:4])
    verdict = f"불변" if bs == 0 else f"★{bs:+#x} ⇒ {off+bs:#x}"
    conf = "확실" if bv >= max(3, n * 0.6) and (len(best) < 2 or best[1][1] < bv) else "약함"
    print(f"  {cname:30s} {off:#7x} {n:>6d}  {tail:34s} ⇒ {verdict} ({conf})")
