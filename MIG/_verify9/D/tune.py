# -*- coding: utf-8 -*-
u"""구제 통로별 **검출력↔적발** 곡선. 어느 통로가 검출력을 죽이는지 실측한다."""
import io, json, os, re, sys, copy, importlib

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
FULL = G.TXTKEYS
NARROW = ("mem", "consts", "knobs")
MID = ("mem", "consts", "knobs", "sig")


def run():
    n = 0
    hits = []
    for i, sp in enumerate(D["specs"]):
        r = G.check_spec(sp)
        n += len(r)
        for x in r:
            hits.append((i,) + x)
    return n, hits


def mut(strict=True):
    caught = tot = 0
    for i, sp0 in enumerate(D["specs"]):
        lg = G.STRIKE.sub(u" ", sp0.get("logic") or u"")
        co = {int((a or b), 16) for a, b in G.CITE.findall(lg)}
        co |= {int(m.group(2), 16) for m in G.TYOFF.finditer(lg)}
        cn = u" ".join(m.group(2).split(u".")[-1] for m in G.FLD.finditer(lg))
        b0 = len(G.check_spec(sp0))
        for j, row in enumerate(sp0.get("mem") or []):
            mm = re.match(r"^\s*0x([0-9a-fA-F]+)\s*$", str(row.get("offset") or u""))
            if not mm:
                continue
            off = int(mm.group(1), 16)
            nm = str(row.get("name") or u"")
            if strict and off not in co and not any(
                    re.search(r"(?<![A-Za-z0-9_])%s(?![A-Za-z0-9_])" % re.escape(t), cn)
                    for t in re.findall(r"[a-z_][a-z0-9_]{3,}", nm)):
                continue
            tot += 1
            sp = copy.deepcopy(sp0)
            del sp["mem"][j]
            if len(G.check_spec(sp)) > b0:
                caught += 1
    return caught, tot


for tag, keys in ((u"FULL(현행)", FULL), (u"MID(+sig)", MID), (u"NARROW(표만)", NARROW)):
    G.TXTKEYS = keys
    n, hits = run()
    c, t = mut()
    print(u"\n### %-14s 적발 %-3d · 검출력 %d/%d = %.1f%%" % (tag, n, c, t, 100.0 * c / t))
    for h in hits:
        print(u"    specs[%-2d] %s %s" % (h[0], h[1], h[2][:88]))
