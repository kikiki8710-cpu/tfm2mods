# -*- coding: utf-8 -*-
u"""patch.json 을 **메모리에서만** 적용해 보고 게이트가 여전히 0 인지 본다(정본 안 건드림)."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
P = json.load(io.open(os.path.join(HERE, "patch.json"), encoding="utf-8"))
import re
for e in P["errors"]:
    m = re.match(r"/specs\[(\d+)\]/sig/params\[(\d+)\]/role$", e["path"])
    i, j = int(m.group(1)), int(m.group(2))
    cur = D["specs"][i]["sig"]["params"][j]["role"]
    assert e["old"] in cur, (i, j, "old 없음")
    D["specs"][i]["sig"]["params"][j]["role"] = cur.replace(e["old"], e["new"])
tot = 0
for i, sp in enumerate(D["specs"]):
    r = G.check_spec(sp)
    tot += len(r)
    for (j, why, det) in r:
        print(u"specs[%d] p[%s] %s\n    %s" % (i, j, why, det))
print(u"패치 적용본 적발 = %d" % tot)
