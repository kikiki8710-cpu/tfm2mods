# -*- coding: utf-8 -*-
u"""patch.json 이 **정말 그 행**을 건드리는지 사본에서 확인한다(정본은 건드리지 않는다)."""
import io, json, os, sys, copy
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import applypatch as AP

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))
D = copy.deepcopy(D)
pj = json.load(io.open(r"C:\tfm2mods\MIG\_verify10\C\patch.json", encoding="utf-8"))
log, skipped = [], []
for e in pj["errors"]:
    r = AP.apply_error(D, e, log, skipped)
    print("ERR", r, e["path"])
for u in pj["ev_up"]:
    r = AP.apply_evup(D, u, 10, "C", log)
    print("EVUP", r, u["path"])

print("--- 결과 확인")
print("10 consts[5].src_line =", D["specs"][10]["constants"][5]["src_line"])
print("10 knobs[5].where     =", D["specs"][10]["knobs"][5]["where"][:90])
print("10 knobs[6].what      =", D["specs"][10]["knobs"][6]["what"])
print("10 mem12 dir          =", D["specs"][10]["reads"][12].get("dir"))
print("10 mem15 dir          =", D["specs"][10]["reads"][15].get("dir"))
print("13 mem8  dir          =", D["specs"][13]["reads"][8].get("dir"))
print("14 mem15 dir          =", D["specs"][14]["reads"][15].get("dir"))
print("12 reads[5]           =", D["specs"][12]["reads"][5]["base"], D["specs"][12]["reads"][5]["offset"])
print("12 reads[6]           =", D["specs"][12]["reads"][6]["base"], D["specs"][12]["reads"][6]["offset"])
print("14 reads[1]           =", D["specs"][14]["reads"][1]["base"], D["specs"][14]["reads"][1]["offset"])
print("14 reads[2]           =", D["specs"][14]["reads"][2]["base"], D["specs"][14]["reads"][2]["offset"])
print("12 knobs[3].what      =", D["specs"][12]["knobs"][3]["what"])
print("11 new_knobs[9] tail  =", D["specs"][11]["new_knobs"][9]["effect"][-120:])
print("12 new_knobs[5] tail  =", D["specs"][12]["new_knobs"][5]["effect"][-120:])
print("11 one_line           =", D["specs"][11]["one_line"])
