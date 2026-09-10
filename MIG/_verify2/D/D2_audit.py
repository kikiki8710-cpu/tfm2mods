# -*- coding: utf-8 -*-
u"""D2_audit — 2차. `_spec\\specs20.json`(1차 정정 반영본)의 15~19 를 tcx 정본으로 전수 재확인.
1차의 `D_audit` 은 `_verify\\1x_*.json`(정정 전 스냅숏)을 봤다 — 여기서는 **본 표**를 본다.
"""
import json, io, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, MIG)
import tcxaudit as TA

try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

spec = json.load(io.open(os.path.join(MIG, "_spec", "specs20.json"), encoding="utf-8"))
A = TA.Audit()
for i in range(15, 20):
    sp = spec["specs"][i]
    tag = "%02d_%s" % (i, sp["name"])
    for k in ("reads", "writes"):
        for e in sp.get(k, []):
            A.check(u"%s/%s" % (tag, k), e.get("base", "?"), e.get("offset", "?"), e.get("name"))
    TA.prose_scan(A, json.dumps(sp, ensure_ascii=False), tag + "(prose)")
TA.report(A)
