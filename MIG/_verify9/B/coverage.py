# -*- coding: utf-8 -*-
u"""게이트 **도달률** — 126행 중 각 검사가 실제로 *볼 거리가 있는* 행이 몇 개인가.

「적발 0」의 세 번째 독법을 막는다: ③검사할 주장 자체가 명세에 없다.
변이 시험(포착률)과 짝이다 — 포착률은 *볼 거리가 있을 때* 잡느냐고, 도달률은 *볼 거리가 있느냐*다.
"""
import io, json, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
c = collections.Counter()
rows = 0
detail = collections.defaultdict(list)
for i, sp in enumerate(D["specs"]):
    ps = (sp.get("sig") or {}).get("params") or []
    head, body, lno = G.defhead(sp)
    if head is None or not ps:
        continue
    args = G.split_args(head)
    mapping, _a, _f = G.align(sp)
    rows += len(ps)
    c["P1 정렬(함수)"] += 0
    if G.is_sret_arg(args[0] if args else u""):
        c["P2 sret 행(함수)"] += 1
    if G.tcx_arity(sp) is not None:
        c["P3 tcx 검산(함수)"] += 1
    for j, p in enumerate(ps):
        role = p.get("role") or u""
        claim = G.strip_ir_quotes(role)
        if any(a in claim for a in G.ATTRS):
            c["P4 속성 주장(행)"] += 1
            detail["P4"].append(u"%s p[%d]" % (sp["name"], j))
        if any(G.FULL_NEG.search(x or u"") for x in G.CLAUSE.split(role)):
            c["P5 전면부정 절(행)"] += 1
        if G.cites(role) and [q for _p, q in G.backticks(role) if G.is_ir_quote(q)]:
            c["P6 IR 인용(행)"] += 1
            detail["P6"].append(u"%s p[%d]" % (sp["name"], j))
        if G.OFFPAT.search(role):
            c["E 오프셋 인용(행·약함)"] += 1

print(u"전체 %d행 / 20함수" % rows)
for k in sorted(c):
    print(u"  %-24s %3d" % (k, c[k]))
print(u"\nP6 도달 행: %s" % u", ".join(detail["P6"]))
