# -*- coding: utf-8 -*-
import io, json, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
P = sys.argv[1] if len(sys.argv) > 1 else r"C:\tfm2mods\MIG\_spec\specs20.json"
D = json.load(io.open(P, encoding="utf-8"))
specs = D["specs"] if isinstance(D, dict) else D
i = int(sys.argv[2])
sp = specs[i]
keys = sys.argv[3].split(",") if len(sys.argv) > 3 else list(sp.keys())
for k in keys:
    v = sp.get(k)
    print("=== %s ===" % k)
    if isinstance(v, list):
        for n, e in enumerate(v):
            print("[%d] %s" % (n, json.dumps(e, ensure_ascii=False)))
    else:
        print(json.dumps(v, ensure_ascii=False))
