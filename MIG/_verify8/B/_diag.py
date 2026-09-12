# -*- coding: utf-8 -*-
u"""후보 3벌의 판정을 행 단위로 교차 비교한다(진단 전용, 게이트 아님)."""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.path.insert(0, r"C:\tfm2mods\MIG\_gates\consts_kind")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")

import B_kindcheck as BB
import C_kindchk as CC
import D_constkind as DD

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))

SENT = DD.SENT
rows = []
for i in range(20):
    sp = D["specs"][i]
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    cres = {r[0]: r for r in CC.check_spec(sp)}
    for j, c in enumerate(sp.get("consts") or []):
        val, kind = c.get("value"), (c.get("kind") or u"")
        obs = BB.observed(sp, val)
        bok = (kind in obs) or (kind == u"센티널" and obs)
        cr = cres.get(j)
        cv = cr[4] if cr else u"OK"
        u_ = DD.usage(f, a, b, val) if f else set()
        dbad = []
        if str(val) in SENT and kind == u"태그":
            dbad.append("R1")
        if "SWITCH" in u_ and "REL" not in u_ and kind == u"임계":
            dbad.append("R2")
        if kind == u"임계" and u_ and not (u_ & {"REL", "EQ", "SWITCH"}):
            dbad.append("R3")
        rows.append(dict(i=i, j=j, name=sp["name"], val=val, kind=kind,
                         B=("OK" if bok else "X"), Bobs=obs,
                         C=cv, Dusage=sorted(u_), D=(",".join(dbad) or "OK"),
                         meaning=(c.get("meaning") or u"")))

json.dump(rows, io.open(r"C:\tfm2mods\MIG\_verify8\B\_diag.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)

from collections import Counter
print(u"총 %d행" % len(rows))
print(u"kind 분포:", Counter(r["kind"] for r in rows))
print(u"B 불일치 %d / C 불일치 %d / C 보류 %d / D 적발 %d"
      % (sum(1 for r in rows if r["B"] == "X"),
         sum(1 for r in rows if r["C"].startswith("**")),
         sum(1 for r in rows if r["C"].startswith(u"판정보류")),
         sum(1 for r in rows if r["D"] != "OK")))
print()
print(u"── 교차표 (B,Cbad,Dbad) ──")
print(Counter((r["B"] == "X", r["C"].startswith("**"), r["D"] != "OK") for r in rows))
print()
print(u"── B만 적발(C·D 무사)한 행의 관측 분포 ──")
print(Counter(tuple(sorted(r["Bobs"])) for r in rows
              if r["B"] == "X" and not r["C"].startswith("**") and r["D"] == "OK"))
print()
print(u"── C만 적발한 행의 kind·문맥 ──")
for r in rows:
    if r["C"].startswith("**") and r["B"] == "OK" and r["D"] == "OK":
        print(u"  [%02d]c%-2d %-14s kind=%-4s Bobs=%-30s Dusage=%s"
              % (r["i"], r["j"], str(r["val"])[:14], r["kind"],
                 str(sorted(r["Bobs"])), r["Dusage"]))
