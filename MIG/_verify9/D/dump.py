# -*- coding: utf-8 -*-
u"""행 덤프 — `python dump.py <spec> <memidx> [<memidx>...]`  / `--field consts` 도 가능."""
import io, json, os, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
a = sys.argv[1:]
fld = "mem"
if "--field" in a:
    k = a.index("--field")
    fld = a[k + 1]
    a = a[:k] + a[k + 2:]
i = int(a[0])
sp = D["specs"][i]
print(u"## specs[%d] %s   ir=%s" % (i, sp["name"], json.dumps(sp.get("ir"), ensure_ascii=False)))
rows = sp.get(fld) or []
idxs = [int(x) for x in a[1:]] or range(len(rows))
for j in idxs:
    print(u"\n[%s][%d] %s" % (fld, j, json.dumps(rows[j], ensure_ascii=False)))
