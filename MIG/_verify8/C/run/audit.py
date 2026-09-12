# -*- coding: utf-8 -*-
u"""124행 전수 실측표 — role 의 「미사용」 주장 ↔ IR 실사용. P5 강등이 진짜 결함을 덮는지 확인용."""
import io, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG\_verify8\C")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import gate as G

D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
print(u"| # | j | name | reg | claim | use | outcall | spill | off0 | dyn |")
print(u"|---|---|---|---|---|---|---|---|---|---|")
bad = 0
for i in range(20):
    sp = D["specs"][i]
    head, body, lno = G.defhead(sp)
    m, args, fin = G.align(sp)
    if m is None:
        print(u"| %02d | — | **정렬불능** | | | | | | | |" % i); continue
    use, outcall, spill, off0, off1, dyn = G.analyze(body, ["%%%d" % k for k in range(len(args))])
    ps = (sp.get("sig") or {}).get("params") or []
    for j, p in enumerate(ps):
        role = p.get("role") or u""
        rl = m.get(j) or []
        cl = u"NEG" if G.FULL_NEG.search(role) else u"-"
        pos = u"+" if G.POS_USE.search(role) else u""
        u_ = sum(use[r] for r in rl); o_ = sum(outcall[r] for r in rl)
        s_ = sum(spill[r] for r in rl)
        of = sorted({x for r in rl for x in off0[r]})
        dy = any(dyn[r] for r in rl)
        mark = u""
        if cl == u"NEG" and (o_ - s_) > 0:
            mark = u" ★"; bad += 1
        print(u"| %02d | %d | %s | %s | %s%s%s | %d | %d | %d | %s | %s |"
              % (i, j, (p.get("name") or u"")[:20], ",".join(rl) or u"(없음)", cl, pos, mark,
                 u_, o_, s_, ",".join(hex(x) for x in of)[:34], u"Y" if dy else u""))
print(u"\n부정주장 ∧ call밖 실사용>스필 = %d행" % bad)
