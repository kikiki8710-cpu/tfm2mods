# -*- coding: utf-8 -*-
u"""IR 범위의 gep 오프셋 · load/store 를 뽑아 mem 표와 대조하기 위한 원자료."""
import io, os, re, sys, json, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
RNG = {15: ("m13.ll", 33374, 33739), 16: ("m10.ll", 51913, 52374),
       17: ("m05.ll", 17695, 17825), 18: ("m09.ll", 6879, 7264),
       19: ("m04.ll", 62570, 62978)}
GEP = re.compile(r"%(\d+) = getelementptr(?: inbounds)?(?: nuw)? i8, ptr (%\d+), i64 (-?\d+)")
LOAD = re.compile(r"%(\d+) = load ([^,]+), ptr (%\d+)")
STORE = re.compile(r"store ([^,]+), ptr (%\d+)")
D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]
which = [int(x) for x in sys.argv[1:]] or sorted(RNG)
for k in which:
    f, a, b = RNG[k]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    base = {}          # reg -> (root, offset)
    for i in range(8):
        base["%%%d" % i] = ("arg%d" % i, 0)
    rows = []
    for n, ln in enumerate(src[a - 1:b], a):
        m = GEP.search(ln)
        if m:
            r, p, o = "%" + m.group(1), m.group(2), int(m.group(3))
            if p in base:
                base[r] = (base[p][0], base[p][1] + o)
            else:
                base[r] = ("?" + p, o)
            continue
        m = LOAD.search(ln)
        if m:
            r, ty, p = "%" + m.group(1), m.group(2).strip(), m.group(3)
            bb = base.get(p, ("?" + p, 0))
            rows.append((n, "r", bb[0], bb[1], ty))
            base[r] = ("*" + str(bb[0]) + "+" + hex(bb[1]), 0)
            continue
        m = STORE.search(ln)
        if m:
            val, p = m.group(1).strip(), m.group(2)
            bb = base.get(p, ("?" + p, 0))
            rows.append((n, "w", bb[0], bb[1], val))
    print("=" * 96)
    print("### %02d %s %d~%d   load/store %d" % (k, D[k]["name"], a, b, len(rows)))
    agg = collections.Counter()
    for (n, d, root, off, ty) in rows:
        agg[(d, root, off)] += 1
    for (d, root, off), c in sorted(agg.items(), key=lambda x: (str(x[0][1]), x[0][2])):
        print("   %s %-14s %-8s x%d" % (d, str(root)[:14], hex(off), c))
