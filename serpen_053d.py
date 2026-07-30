# -*- coding: utf-8 -*-
# serpen_053d.py — 전역 반복 전파(BinDiff propagation)로 앵커를 불려 미해결분 확정 + 교차검증.
#   ① skel 유일쌍 시드 → ② caller/callee 투표로 라운드마다 앵커 확장(3회) → ③ 대상 4종 판정
#   ④ 교차검증: MOBATICK 은 provider 구조체 오프셋(0xed20/0xed28/0xed50/0xed58) 참조 유무로 확인
import sys, io, struct, pickle, collections, bisect, math
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

    def text(self):
        for nm, va, vsz, rraw, rsz in self.secs:
            if nm == ".text":
                return va, vsz, rraw, rsz

    def owner(self, rva):
        i = bisect.bisect_right(self.starts, rva) - 1
        if i < 0:
            return None
        s = self.starts[i]
        return s if rva < s + self.fn[s]["size"] else None

    def build_graph(self):
        va, vsz, rraw, rsz = self.text()
        blob = self.raw[rraw:rraw + rsz]
        self.caller_fn = collections.defaultdict(collections.Counter)
        self.callee = collections.defaultdict(collections.Counter)
        i = 0; n = len(blob); fnset = self.fn
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > n:
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i; tgt = site + 5 + rel
            if tgt in fnset:
                w = self.owner(site)
                if w is not None:
                    self.caller_fn[tgt][w] += 1
                    self.callee[w][tgt] += 1
            i += 1
        print(f"  [{self.tag}] 콜그래프 타깃 {len(self.caller_fn)} / 호출자 {len(self.callee)}")


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl", "0.5.2")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")
O.build_graph(); N.build_graph()

so = collections.defaultdict(list); sn = collections.defaultdict(list)
for r, v in O.fn.items():
    so[v["skel"]].append(r)
for r, v in N.fn.items():
    sn[v["skel"]].append(r)
A = {vs[0]: sn[k][0] for k, vs in so.items() if len(vs) == 1 and len(sn.get(k, ())) == 1}
FIX = {0x21f8ca0: 0x1535810, 0x53aae0: 0xabdf60, 0x539f40: 0xabd340, 0x5ac950: 0x91ab0,
       0x24b5a00: 0x1a6530, 0x811500: 0x960df0, 0x22164a0: 0xfdbbb0, 0x22d2b20: 0x12c3bb0,
       0xc2f990: 0x1b0aba0, 0x1d96870: 0xeb8810, 0x74d510: 0x997740, 0x1554930: 0x229a410,
       0x25c4dd0: 0x28e3b10}
A.update(FIX)
used = set(A.values())
print(f"  시드 앵커 {len(A)}쌍")


def cosf(a, b):
    X, Y = O.fn[a]["mnem"], N.fn[b]["mnem"]
    ks = set(X) | set(Y)
    da = math.sqrt(sum(X.get(k, 0) ** 2 for k in ks)); db = math.sqrt(sum(Y.get(k, 0) ** 2 for k in ks))
    return 0.0 if not da or not db else sum(X.get(k, 0) * Y.get(k, 0) for k in ks) / (da * db)


def tally(old_rva):
    up = collections.Counter(); dn = collections.Counter()
    nup = ndn = 0
    for cf, cnt in O.caller_fn.get(old_rva, {}).items():
        if cf in A:
            nup += 1
            for t, k in N.callee.get(A[cf], {}).items():
                if k == cnt:
                    up[t] += 1
    for ce, cnt in O.callee.get(old_rva, {}).items():
        if ce in A:
            ndn += 1
            for f, k in N.caller_fn.get(A[ce], {}).items():
                if k == cnt:
                    dn[f] += 1
    tot = up + dn
    return tot, up, dn, nup, ndn


TARGETS = {"RUNNER_CTOR": 0x1d981e0, "MOBATICK": 0x230c290, "ARG_STR": 0xfef190, "UIALLOC": 0x25c4d30}

for rnd in range(3):
    todo = [f for f in O.fn if f not in A and (O.caller_fn.get(f) or O.callee.get(f))]
    added = 0
    for f in todo:
        tot, up, dn, nup, ndn = tally(f)
        if not tot or (nup + ndn) < 2:
            continue
        best = tot.most_common(2)
        t, v = best[0]
        if v < 2 or (len(best) > 1 and best[1][1] >= v):
            continue
        if t in used or t not in N.fn:
            continue
        r = N.fn[t]["size"] / max(1, O.fn[f]["size"])
        if not (0.6 <= r <= 1.8) or cosf(f, t) < 0.93:
            continue
        A[f] = t; used.add(t); added += 1
    print(f"  전파 라운드 {rnd+1}: +{added}쌍 → 총 {len(A)}")
    if added == 0:
        break

print()
for nm, o in TARGETS.items():
    print("=" * 78)
    tot, up, dn, nup, ndn = tally(o)
    osz = O.fn[o]["size"]
    fixed = A.get(o)
    print(f"[{nm}] 0.5.2={o:#x} size={osz} 사상 caller {nup}/{len(O.caller_fn.get(o,{}))} "
          f"callee {ndn}/{len(O.callee.get(o,{}))} 전파확정={fixed and hex(fixed)}")
    for t, v in tot.most_common(6):
        if t not in N.fn:
            continue
        r = N.fn[t]["size"] / osz
        print(f"    {t:#x} 표={v}(↑{up[t]}/↓{dn[t]}) cos={cosf(o,t):.4f} size={N.fn[t]['size']} "
              f"비={r:.3f} 16B={N.read(t,16).hex(' ')}")
print()

# ── 교차검증: 구조체 오프셋 참조 ────────────────────────────────
print("=" * 78)
print("교차검증 — provider 구조체 오프셋(0xed20/0xed28/0xed50/0xed58/0xeab8/0xeac0) 참조 개수")


def disp_hist(img, rva, wanted):
    b = img.read(rva, img.fn[rva]["size"])
    c = collections.Counter()
    for i in md.disasm(b, rva):
        for w in wanted:
            if f"0x{w:x}]" in i.op_str:
                c[w] += 1
    return c


W = [0xed20, 0xed28, 0xed50, 0xed58, 0xed30, 0xed38, 0xeab8, 0xeac0, 0xecd0, 0xecd8]
print(f"  0.5.2 MOBATICK 0x230c290 → { {hex(k):v for k,v in disp_hist(O,0x230c290,W).items()} }")
for c in (0xeeeac0, 0xf09a40, 0xf19670):
    if c in N.fn:
        print(f"  0.5.3 후보 {c:#x}      → { {hex(k):v for k,v in disp_hist(N,c,W).items()} }")
print()
print(f"  0.5.2 SERPEN 0x21f8ca0 → { {hex(k):v for k,v in disp_hist(O,0x21f8ca0,W).items()} }")
print(f"  0.5.3 SERPEN 0x1535810 → { {hex(k):v for k,v in disp_hist(N,0x1535810,W).items()} }")
