# -*- coding: utf-8 -*-
u"""`mkspec3._rank_callees` 의 `_anchor` 판정을 **길이접두 포함 매칭**으로 다시 계산해
현행 v3 의 `ev3`(=IR 호출 심볼 일치) 도장이 몇 개나 거짓인지 센다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkspec3 as M

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
S = D["specs"] if isinstance(D, dict) else D
DI = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))
S2 = DI["specs"] if isinstance(DI, dict) else DI

tot = anch = bad = 0
for i, sp in enumerate(S):
    irsy = M._ir_callsyms(S2[i])
    for j, c in enumerate(sp.get("callees") or []):
        tot += 1
        if c.get("ev") != 3:
            continue
        anch += 1
        segs = [s for s in (c.get("path") or u"").split(u"::") if s]
        # 길이접두를 붙여서 재판정
        ok = any(all((u"%d%s" % (len(s), s)) in sym for s in segs) for sym in irsy)
        if not ok:
            bad += 1
            print(u"[%02d] callees[%d] %-40s  ev3 인데 길이접두 매칭 실패" % (i, j, c.get("path")[:60]))
            for s in segs:
                hit = [sym[:60] for sym in irsy if (u"%d%s" % (len(s), s)) in sym]
                print(u"      seg %-24s lenpfx=%s  hit=%d" % (s, u"%d%s" % (len(s), s), len(hit)))
print(u"\n총 callees %d행 / ev3 %d행 / 길이접두로 무너지는 것 %d행" % (tot, anch, bad))
