# -*- coding: utf-8 -*-
u"""통합판이 **새로 검사 가능하게 만든** 상수 목록(현행 게이트는 강한 후보가 0 이던 것)."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G
import srclinecheck as S

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def oldstrong(sp, j):
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    own = sp["src"].split(u"\\")[-1]
    src, meta = S.load(f)
    val = sp["consts"][j].get("value")
    pat = S.litpat(val)
    found = set()
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln or not pat.search(ln):
            continue
        st, wk = S.dbg_ids(src, meta, k, a, b, val)
        for did in st:
            for (fn, li) in S.chain(meta, did):
                if fn == own and li:
                    found.add(li)
    return found


n = 0
for i, sp in enumerate(D["specs"]):
    try:
        r = G.collect(sp)
    except Exception:
        continue
    for j, c in enumerate(sp.get("consts") or []):
        if not isinstance(c.get("src_line"), int) or j not in r:
            continue
        ns, nw, _ = r[j]
        if ns and not oldstrong(sp, j):
            n += 1
            cl = c["src_line"]
            v = u"claim∈강" if cl in ns else (u"claim∈약(구제)" if cl in nw else u"★불일치")
            print(u"  specs[%2d] %-38s consts[%-2d] val=%-8s claim=L%-6s 강=%s  %s"
                  % (i, sp["name"][:38], j, c["value"], cl, sorted(ns), v))
print(u"\n새로 검사 가능해진 상수 = %d개" % n)
