# -*- coding: utf-8 -*-
import io, json, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]
pat = re.compile(u"\u3010?\u26a0?\uBBF8\uD655\uC815 .* \uD6C4\uBCF4 (\\d+)\uAC1C \uC911 \uC0C1\uC704 (\\d+)\uAC1C")
tot = collections.Counter()
for i, s in enumerate(D):
    for j, c in enumerate(s.get("callees") or []):
        p = c.get("pick") or u""
        if u"IR \uD638\uCD9C \uC2EC\uBCFC \uC77C\uCE58" in p:
            tot["ev3"] += 1
            continue
        m = re.search(u"\uD6C4\uBCF4 (\\d+)\uAC1C \uC911 \uC0C1\uC704 (\\d+)\uAC1C", p)
        if m:
            n = int(m.group(1))
            tot["ev4"] += 1
            tot["ev4_cand%d" % min(n, 9)] += 1
            if n == 1:
                tot["ev4_unique"] += 1
print(dict(tot))
print()
for i in (15, 16, 17, 18, 19):
    s = D[i]
    print("=== %02d %s  callees %d" % (i, s["name"], len(s.get("callees") or [])))
    for j, c in enumerate(s.get("callees") or []):
        p = c.get("pick") or u""
        m = re.search(u"\uD6C4\uBCF4 (\\d+)\uAC1C", p)
        tagv = "ev3" if c.get("ev") == 3 else ("uniq1" if (m and m.group(1) == "1") else "AMBIG%s" % (m.group(1) if m else "?"))
        print("  [%2d] %-34s %-7s %s" % (j, c["name"], tagv, c["path"]))
