# -*- coding: utf-8 -*-
u"""D_audit — 담당 5건(15~19) 명세의 reads/writes 오프셋 + 산문 오프셋을 tcx 정본으로 전수 재확인."""
import json, io, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(HERE)
sys.path.insert(0, MIG)
import tcxaudit as TA

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

FILES = ["15_single_try_engage.json", "16_max_range_nearly_can_use.json",
         "17_new.json", "18_v3_epicops_buff_window.json", "19_best_jungle_goal.json"]

A = TA.Audit()
for fn in FILES:
    p = os.path.join(HERE, fn)
    sp = json.load(io.open(p, encoding="utf-8"))
    tag = fn[:2] + "_" + sp["name"]
    for k in ("reads", "writes"):
        for e in sp.get(k, []):
            A.check(u"%s/%s" % (tag, k), e.get("base", "?"), e.get("offset", "?"), e.get("name"))
    with io.open(p, encoding="utf-8") as f:
        TA.prose_scan(A, f.read(), tag + "(prose)")
TA.report(A)
