# -*- coding: utf-8 -*-
# mig060 미발견 함수 진단: 구 Location 목록 · 0.6.0 (파일,열) 부분 일치 후보 상위 5(완화 J≥0.25) · 크기/니모닉
import io, json, os, sys, pickle, collections, math
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mig060 as M
O = M.Exe(M.OLD, "old"); N = M.Exe(M.NEW, "new"); O.build_locidx(); N.build_locidx()
FO = {(int(k, 16) if isinstance(k, str) else k): v for k, v in pickle.load(open(M.PKL["old"], "rb"))["idx"].items()}
FN = {(int(k, 16) if isinstance(k, str) else k): v for k, v in pickle.load(open(M.PKL["new"], "rb"))["idx"].items()}
fc_item_new = collections.defaultdict(list)
for f, L in N.fnloc.items():
    for key in set((ff, c) for ff, l, c in L): fc_item_new[key].append(f)
file_new = collections.defaultdict(list)
for f, L in N.fnloc.items():
    for ff, l, c in L: file_new[ff].append(f)
R = json.load(io.open(os.path.join(M.HERE, "_next", "mig060_judge.json"), encoding="utf-8"))
def cosf(a, b):
    mo = FO.get(a, {}).get("mnem", {}); mn = FN.get(b, {}).get("mnem", {}); keys = set(mo) | set(mn)
    dot = sum(mo.get(k, 0) * mn.get(k, 0) for k in keys)
    return dot / (math.sqrt(sum(v * v for v in mo.values())) * math.sqrt(sum(v * v for v in mn.values())) + 1e-9) if mo and mn else 0
only = [a.lower() for a in sys.argv[1:]]
for r in R:
    if "new" in r or r["src"] != "spec": continue
    if only and r["old"] not in only: continue
    a = int(r["old"], 16); L = O.fnloc.get(a, [])
    osz = O.ends[a] - a
    print(u"\n■ %s %s  %dB · Location %d: %s" % (r["old"], r["name"], osz, len(L), u" · ".join(u"%s:%d:%d" % (os.path.basename(f), l, c) for f, l, c in sorted(set(L))[:8])))
    fcL = collections.Counter((f, c) for f, l, c in L)
    pool = collections.Counter()
    for key in fcL:
        for f2 in fc_item_new.get(key, []): pool[f2] += 1
    if not pool and L:
        for f, l, c in set(L):
            for f2 in file_new.get(f, []): pool[f2] += 0.1
    sc = []
    for f2, hit in pool.most_common(400):
        L2 = N.fnloc.get(f2, []); fc2 = collections.Counter((f, c) for f, l, c in L2)
        j = sum((fcL & fc2).values()) / max(1, sum((fcL | fc2).values()))
        nsz = N.ends.get(f2, f2) - f2
        sc.append((j * 0.6 + cosf(a, f2) * 0.4, j, cosf(a, f2), f2, nsz, len(L2)))
    sc.sort(reverse=True)
    for s, j, cs, f2, nsz, nl in sc[:5]:
        L2 = N.fnloc.get(f2, [])
        print(u"    후보 %x  %5dB(비 %.2f) · J %.2f · cos %.3f · Loc %d: %s" % (f2, nsz, min(osz, nsz) / max(osz, nsz), j, cs, nl, u" · ".join(u"%s:%d:%d" % (os.path.basename(f), l, c) for f, l, c in sorted(set(L2))[:6])))
    if not sc: print(u"    (Location 기반 후보 없음 — 콜리 수 %d)" % len(M.pickle.load(open(M.CG["old"], "rb"))["callee"].get(a, [])) if False else u"    (Location 기반 후보 없음)")
