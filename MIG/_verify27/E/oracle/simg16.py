# -*- coding: utf-8 -*-
"""simg16.py — 내 params 정정안을 v3 사본에 적용해 paramrole.check_spec(G16)·G5 개수 대조를 미리 돌려본다."""
import sys, io, json, copy
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import paramrole as PR
import specgate as SG

with io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8") as fh:
    D = json.load(fh)
specs = D["specs"] if isinstance(D, dict) else D
P = json.load(io.open(sys.argv[1], encoding="utf-8"))

def apply_params(sp, edits):
    ps = sp["sig"]["params"]
    for e in edits:
        if e.get("op") == "delete":
            j = int(e["path"].split("params[")[1].split("]")[0])
            assert e["guard"] in json.dumps(ps[j], ensure_ascii=False), ("guard", e["path"])
            ps.pop(j); continue
        if "params[" not in e["path"]:
            continue
        j = int(e["path"].split("params[")[1].split("]")[0]); key = e["path"].rsplit("/", 1)[1]
        cur = ps[j][key]
        assert e["old"] == cur or e["old"] in cur, ("old mismatch", e["path"], cur[:80])
        ps[j][key] = e["new"] if e["old"] == cur else cur.replace(e["old"], e["new"])

for idx in (258, 259, 260, 261):
    sp = copy.deepcopy(specs[idx])
    edits = [e for e in P["errors"] if e["path"].startswith("/specs[%d]/sig/params" % idx)]
    apply_params(sp, edits)
    rows = PR.check_spec(sp)
    print("== [%d] %s  G16 rows=%d (edits %d)" % (idx, sp["name"], len(rows), len(edits)))
    for r in rows:
        print("   ", r[0], r[1], str(r[2])[:300])
    # G5 개수
    SG.FLAGS = [] if hasattr(SG, "FLAGS") else None
    before = len(getattr(SG, "FLAGS", []) or [])
    try:
        SG.gate5(idx, sp)
    except Exception as ex:
        print("   gate5 err", ex)
    fl = getattr(SG, "FLAGS", None)
    if fl is not None:
        for f in fl[before:]:
            print("   G5:", f)
    m, args, fin = PR.align(sp)
    print("   align:", None if m is None else {k: v for k, v in m.items()})
