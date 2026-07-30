# -*- coding: utf-8 -*-
# serpen_053p.py — ① asset-get 모노모픽 copy 군집 규모 확인(0x91ab0 vs 0x2e1550)
#                  ② UIPARSER(0x24b5a00) 를 콜러 사상으로 독립 검증
import sys, io, struct, pickle, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)


def load(p, pk):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    ns = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    st = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(ns):
        o = st + i * 40
        nm = d[o:o + 8].rstrip(b"\x00").decode("utf-8", "replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, max(vsz, rsz), rraw, rsz))
    P = pickle.load(open(pk, "rb"))["idx"]
    fn = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.items()}
    def rd(rva, n):
        for nm2, va2, vs, rr, rs in secs:
            if va2 <= rva < va2 + vs:
                return d[rr + (rva - va2): rr + (rva - va2) + n]
    return d, secs, fn, rd


DO, SO, FNO, RDO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe",
                        r"C:\tfm2mods\_fnidx_052.pkl")
D, S, FN, RD = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe",
                    r"C:\tfm2mods\_fnidx_053.pkl")

print("① asset-get 모노모픽 copy 군집")
for tag, fn, a in (("0.5.2 0x5ac950", FNO, 0x5ac950), ("0.5.3 0x91ab0", FN, 0x91ab0),
                   ("0.5.3 0x2e1550", FN, 0x2e1550)):
    sk = fn[a]["skel"]
    clones = [r for r, v in fn.items() if v["skel"] == sk]
    print(f"   {tag}: size={fn[a]['size']} skel동일 copy = {len(clones)}개")
    if len(clones) <= 30:
        print(f"      {[hex(c) for c in sorted(clones)][:30]}")

print("\n② UIPARSER 0.5.2 0x24b5a00 콜러 사상 검증")
A = pickle.load(open(r"C:\tfm2mods\_anchor_052_053.pkl", "rb"))


def graph(d, secs, fn):
    va, vsz, rraw, rsz = [s[1:] for s in secs if s[0] == ".text"][0]
    blob = d[rraw:rraw + rsz]
    starts = sorted(fn)
    caller = collections.defaultdict(collections.Counter)
    callee = collections.defaultdict(collections.Counter)
    i = 0
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > len(blob):
            break
        rel = struct.unpack_from("<i", blob, i + 1)[0]
        site = va + i
        t = site + 5 + rel
        if t in fn:
            j = bisect.bisect_right(starts, site) - 1
            f = starts[j] if j >= 0 else None
            if f is not None and site < f + fn[f]["size"]:
                caller[t][f] += 1
                callee[f][t] += 1
        i += 1
    return caller, callee


CO, CEO = graph(DO, SO, FNO)
CN, CEN = graph(D, S, FN)
for name, old in (("UIPARSER", 0x24b5a00), ("UILOADER", 0x5ac950)):
    votes = collections.Counter(); n = 0
    for cf, cnt in CO.get(old, {}).items():
        if cf in A:
            n += 1
            for t, k in CEN.get(A[cf], {}).items():
                if k == cnt:
                    votes[t] += 1
    print(f"   [{name}] 0.5.2 caller {len(CO.get(old,{}))}개 중 사상 {n}개")
    for t, v in votes.most_common(5):
        print(f"      {t:#x} 표={v} size={FN.get(t,{}).get('size')} "
              f"(0.5.2 size={FNO[old]['size']})")
