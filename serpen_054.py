# -*- coding: utf-8 -*-
# serpen_054.py — 0.5.3→0.5.4 앵커 전파 매칭 (serpen_053d.py 기반, 대상=serpen 13종)
import sys, io, struct, pickle, collections, bisect, math
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64)

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"

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
            if va <= rva < va + vsz: return rraw + (rva - va)
    def read(self, rva, n):
        o = self.roff(rva); return None if o is None else self.raw[o:o + n]
    def text(self):
        for nm, va, vsz, rraw, rsz in self.secs:
            if nm == ".text": return va, vsz, rraw, rsz
    def owner(self, rva):
        i = bisect.bisect_right(self.starts, rva) - 1
        if i < 0: return None
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
            if i < 0 or i + 5 > n: break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i; tgt = site + 5 + rel
            if tgt in fnset:
                w = self.owner(site)
                if w is not None:
                    self.caller_fn[tgt][w] += 1; self.callee[w][tgt] += 1
            i += 1
        print(f"  [{self.tag}] 타깃 {len(self.caller_fn)} / 호출자 {len(self.callee)}")

O = Img(OLD, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")
N = Img(NEW, r"C:\tfm2mods\_fnidx_054.pkl", "0.5.4")
O.build_graph(); N.build_graph()

so = collections.defaultdict(list); sn = collections.defaultdict(list)
for r, v in O.fn.items(): so[v["skel"]].append(r)
for r, v in N.fn.items(): sn[v["skel"]].append(r)
A = {vs[0]: sn[k][0] for k, vs in so.items() if len(vs) == 1 and len(sn.get(k, ())) == 1}
used = set(A.values())
print(f"  시드 앵커 {len(A)}쌍")

def cosf(a, b):
    X, Y = O.fn[a]["mnem"], N.fn[b]["mnem"]
    ks = set(X) | set(Y)
    da = math.sqrt(sum(X.get(k,0)**2 for k in ks)); db = math.sqrt(sum(Y.get(k,0)**2 for k in ks))
    return 0.0 if not da or not db else sum(X.get(k,0)*Y.get(k,0) for k in ks)/(da*db)

def tally(old_rva):
    up = collections.Counter(); dn = collections.Counter(); nup = ndn = 0
    for cf, cnt in O.caller_fn.get(old_rva, {}).items():
        if cf in A:
            nup += 1
            for t, k in N.callee.get(A[cf], {}).items():
                if k == cnt: up[t] += 1
    for ce, cnt in O.callee.get(old_rva, {}).items():
        if ce in A:
            ndn += 1
            for f, k in N.caller_fn.get(A[ce], {}).items():
                if k == cnt: dn[f] += 1
    return up + dn, up, dn, nup, ndn

TARGETS = {
 "SERPEN":0x1535810, "MOBATICK":0xeeeac0, "SPAWN0":0xabdf60, "SPAWN1":0xabd340,
 "LAUNCHER":0xeb8810, "UILOADER":0x2e1550, "UIPARSER":0x1a6530, "UIALLOC":0x28f7df0,
 "RENDER_STEP":0x960df0, "RUNNER_CTOR":0xeba490, "DMGA":0xfdbbb0, "DMGB":0x12c3bb0,
 "KEYRES":0x1b0aba0, "ARG_STR":0x1228a90,
 "RETA_CONT":0x997740, "RETC_CONT":0x229a410,
}
for rnd in range(int(__import__("os").environ.get("ROUNDS","0"))):
    todo = [f for f in O.fn if f not in A and (O.caller_fn.get(f) or O.callee.get(f))]
    added = 0
    for f in todo:
        tot, up, dn, nup, ndn = tally(f)
        if not tot or (nup + ndn) < 2: continue
        best = tot.most_common(2); t, v = best[0]
        if v < 2 or (len(best) > 1 and best[1][1] >= v): continue
        if t in used or t not in N.fn: continue
        r = N.fn[t]["size"] / max(1, O.fn[f]["size"])
        if not (0.6 <= r <= 1.8) or cosf(f, t) < 0.93: continue
        A[f] = t; used.add(t); added += 1
    print(f"  전파 라운드 {rnd+1}: +{added} → 총 {len(A)}")
    if added == 0: break

print()
for nm, o in TARGETS.items():
    print("="*78)
    tot, up, dn, nup, ndn = tally(o)
    osz = O.fn[o]["size"]; fixed = A.get(o)
    print(f"[{nm}] 0.5.3={o:#x} size={osz} callers {len(O.caller_fn.get(o,{}))}(사상{nup}) "
          f"callees {len(O.callee.get(o,{}))}(사상{ndn}) 전파확정={fixed and hex(fixed)}")
    for t, v in tot.most_common(5):
        if t not in N.fn: continue
        print(f"    {t:#x} 표={v}(↑{up[t]}/↓{dn[t]}) cos={cosf(o,t):.4f} size={N.fn[t]['size']} "
              f"비={N.fn[t]['size']/osz:.3f} callers={len(N.caller_fn.get(t,{}))} 16B={N.read(t,16).hex(' ')}")
pickle.dump({hex(k):hex(v) for k,v in A.items()}, open(r"C:\tfm2mods\_anchor_053_054.pkl","wb"))
print("[saved] _anchor_053_054.pkl", len(A))
