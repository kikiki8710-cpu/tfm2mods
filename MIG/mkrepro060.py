# -*- coding: utf-8 -*-
"""
mkrepro060.py — 0.6.0 재현체(Rust 직접 구현) 작업 순서표. specs20_v060 268 을 「잎부터」 정렬한다.
tier 1 = 동치 + 콜리 전부 동치(A 등급) · 콜리 수 적은 순  → 오프셋만 갱신해 그대로 재현 → sweep DIFF=0 빠르게 쌓는 구간
tier 2 = 동치 + 콜리 변경(B/C)                           → 콜리를 먼저(tier 3) 끝내야 값이 같음
tier 3 = 변경(logic_060 · confidence A)                   → logic_060 대로
tier 4 = 변경 B/C · ★미독 포함                            → 추가 RE 뒤
출력: _next/repro060_order.md
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
V = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v060.json"), encoding="utf-8"))
rows = []
for sp in V["specs"]:
    b = sp["v060"]; vd = b["verdict"]; o = b.get("addr_058"); n = b.get("addr")
    if not o or vd.startswith((u"소멸", u"exe")): continue
    eq = vd.startswith((u"동치", u"✅동치"))
    cc = b.get("callee_changed") or []
    ncal = len(sp.get("callees") or [])
    conf = (b.get("confidence_060") or u"").strip("* -:")[:1] or (u"A" if eq else u"?")
    miss = (b.get("logic_060") or u"").count(u"★미독")
    if eq and not cc: tier = 1
    elif eq: tier = 2
    elif conf == "A" and miss == 0: tier = 3
    else: tier = 4
    rows.append((tier, ncal, sp.get("i"), o, n, sp["name"], vd.split("(")[0], conf, len(cc), miss))
rows.sort()
L = [u"# repro060 작업 순서 — 재현체(Rust 직접 구현) 잎부터 · %d 함수(09-17)" % len(rows), u"",
     u"| tier | 콜리 수 | i | 구 | 신 | 함수 | 판정 | conf | 콜리 변경 | ★미독 |", u"|---|---|---|---|---|---|---|---|---|---|"]
import collections; C = collections.Counter(r[0] for r in rows)
L.insert(2, u"tier 1 잎·동치 %d · tier 2 동치(콜리 변경) %d · tier 3 변경 A %d · tier 4 변경 B/C·미독 %d" % (C[1], C[2], C[3], C[4]))
L.insert(3, u"")
for t, nc, i, o, n, name, vd, conf, ncc, miss in rows:
    L.append(u"| %d | %d | %s | `%s` | `%s` | %s | %s | %s | %d | %d |" % (t, nc, i, o, n, name[:44], vd[:14], conf, ncc, miss))
io.open(os.path.join(HERE, "_next", "repro060_order.md"), "w", encoding="utf-8").write(u"\n".join(L))
print(u"tier1 %d · tier2 %d · tier3 %d · tier4 %d" % (C[1], C[2], C[3], C[4]))
