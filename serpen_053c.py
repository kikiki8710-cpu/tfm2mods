# -*- coding: utf-8 -*-
# serpen_053c.py — BinDiff식 2단계 매칭으로 미해결 4종 재핀.
#   ① 시드: 두 이미지에서 skel 해시가 각각 유일한 함수쌍을 자동 앵커로 채택(수만 개)
#   ② 전파: 대상 함수의 caller(또는 callee)를 앵커로 사상 → 0.5.3 쪽에서 같은 다중도로
#            불리는(부르는) 타깃에 투표 → 최다득표 확정
#   콜사이트 다중도까지 맞추므로 우연 일치가 거의 없다.
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
        """전역 E8 스캔 → caller_fn[target]=Counter{callerfn:n}, callee[fn]=Counter{target:n}"""
        va, vsz, rraw, rsz = self.text()
        blob = self.raw[rraw:rraw + rsz]
        fnset = self.fn
        self.caller_fn = collections.defaultdict(collections.Counter)
        self.callee = collections.defaultdict(collections.Counter)
        self.sites = collections.defaultdict(list)
        i = 0; n = len(blob)
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > n:
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i
            tgt = site + 5 + rel
            if tgt in fnset:
                w = self.owner(site)
                if w is not None:
                    self.caller_fn[tgt][w] += 1
                    self.callee[w][tgt] += 1
                    self.sites[(w, tgt)].append(site)
            i += 1
        print(f"  [{self.tag}] 콜그래프: 타깃 {len(self.caller_fn)} / 호출자 {len(self.callee)}")


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl", "0.5.2")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")
O.build_graph(); N.build_graph()

# ── ① 시드 앵커: skel 유일쌍 ────────────────────────────────
so = collections.defaultdict(list); sn = collections.defaultdict(list)
for r, v in O.fn.items():
    so[v["skel"]].append(r)
for r, v in N.fn.items():
    sn[v["skel"]].append(r)
A = {}
for k, vs in so.items():
    if len(vs) == 1 and len(sn.get(k, ())) == 1:
        A[vs[0]] = sn[k][0]
# 실측 확정쌍 덮어쓰기
A.update({
    0x21f8ca0: 0x1535810, 0x53aae0: 0xabdf60, 0x539f40: 0xabd340,
    0x5ac950: 0x91ab0, 0x24b5a00: 0x1a6530, 0x811500: 0x960df0,
    0x22164a0: 0xfdbbb0, 0x22d2b20: 0x12c3bb0, 0xc2f990: 0x1b0aba0,
    0x1d96870: 0xeb8810, 0x74d510: 0x997740, 0x1554930: 0x229a410,
    0x25c4dd0: 0x28e3b10,
})
print(f"  시드 앵커 = {len(A)}쌍 (skel 유일 + 실측확정)")
RA = {v: k for k, v in A.items()}


def cos(a, b):
    X, Y = O.fn[a]["mnem"], N.fn[b]["mnem"]
    ks = set(X) | set(Y)
    da = math.sqrt(sum(X.get(k, 0) ** 2 for k in ks)); db = math.sqrt(sum(Y.get(k, 0) ** 2 for k in ks))
    return 0.0 if not da or not db else sum(X.get(k, 0) * Y.get(k, 0) for k in ks) / (da * db)


def vote(name, old_rva, note=""):
    print("=" * 78)
    osz = O.fn[old_rva]["size"]
    print(f"[{name}] 0.5.2={old_rva:#x} size={osz} {note}")
    up = collections.Counter(); upn = 0
    for cf, cnt in O.caller_fn.get(old_rva, {}).items():
        if cf in A:
            upn += 1
            for t, k in N.callee.get(A[cf], {}).items():
                if k == cnt:
                    up[t] += 1
    dn = collections.Counter(); dnn = 0
    for ce, cnt in O.callee.get(old_rva, {}).items():
        if ce in A:
            dnn += 1
            for f, k in N.caller_fn.get(A[ce], {}).items():
                if k == cnt:
                    dn[f] += 1
    tot = collections.Counter()
    for t, v in up.items():
        tot[t] += v
    for t, v in dn.items():
        tot[t] += v
    print(f"  사상된 caller {upn}/{len(O.caller_fn.get(old_rva,{}))} · "
          f"callee {dnn}/{len(O.callee.get(old_rva,{}))}")
    res = []
    for t, v in tot.most_common(10):
        if t not in N.fn:
            continue
        c = cos(old_rva, t); r = N.fn[t]["size"] / osz
        res.append((v, c, t, r))
    for v, c, t, r in res[:8]:
        flag = "★" if (0.8 <= r <= 1.35 and c > 0.97) else " "
        print(f"   {flag} {t:#x} 표={v}(↑{up[t]}/↓{dn[t]}) cos={c:.4f} size={N.fn[t]['size']} 비={r:.3f} "
              f"16B={N.read(t,16).hex(' ')}")
    return res


print()
vote("RUNNER_CTOR", 0x1d981e0, "(화면경기 sim Game 생성자)"); print()
vote("MOBATICK", 0x230c290, "(MobaMode::tick, 간접호출)"); print()
vote("ARG_STR", 0xfef190, "(i18n arg(key,&String))"); print()

# ── UIALLOC: 바이트 완전동일 탐색(allocator 쉼은 버전간 동일 코드) ──
print("=" * 78)
print("[UIALLOC] 0.5.2=0x25c4d30 size=93 — 전역 바이트 완전일치 탐색")
pat = O.read(0x25c4d30, 93)
va, vsz, rraw, rsz = N.text()
blob = N.raw[rraw:rraw + rsz]
hits = []
i = 0
while True:
    i = blob.find(pat, i)
    if i < 0:
        break
    hits.append(va + i); i += 1
print(f"  93B 완전일치 = {[hex(h) for h in hits]}")
for h in hits:
    print(f"    {h:#x} 함수시작={h in N.fn} size={N.fn.get(h,{}).get('size')} "
          f"caller수={len(N.caller_fn.get(h,{}))} (0.5.2 caller수={len(O.caller_fn.get(0x25c4d30,{}))})")
