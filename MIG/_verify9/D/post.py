# -*- coding: utf-8 -*-
u"""patch 적용 **후**(메모리 사본) 의 G14/G18/문법 상태 — 정본은 건드리지 않는다."""
import io, json, os, re, sys, copy

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G18
sys.path.insert(0, G18.MIG)
import memdir as MD

D = json.load(io.open(os.path.join(G18.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
P = json.load(io.open(os.path.join(HERE, "patch.json"), encoding="utf-8"))
PTH = re.compile(r"^/specs\[(\d+)\]/(\w+)(?:\[(\d+)\])?(?:/(\w+))?$")


def apply(D):
    D = copy.deepcopy(D)
    for e in P["errors"]:
        if e.get("op") == "insert":
            i = int(PTH.match(e["path"]).group(1))
            D["specs"][i]["mem"].insert(e["at"], e["new"])
            continue
        m = PTH.match(e["path"])
        i, fld, idx, key = int(m.group(1)), m.group(2), m.group(3), m.group(4)
        row = D["specs"][i][fld][int(idx)] if idx is not None else D["specs"][i][fld]
        assert row[key] == e["old"], (e["path"], row[key])
        row[key] = e["new"]
    return D


def tally(specs):
    tot = bad = hold = 0
    for sp in specs:
        ir = sp.get("ir") or {}
        if not ir.get("file"):
            continue
        r, w, hr, hw, _t = MD.scan(ir["file"], ir["frm"], ir["to"])
        bad += len(MD.check_spec(sp))
        for j, m in enumerate(sp.get("mem") or []):
            d = (m.get("dir") or "").strip()
            o = MD.off_of(m)
            if d not in ("r", "w") or o is None:
                continue
            tot += 1
            anc = MD.base_anchors_for(sp, j, m.get("base"), r, w, hr, hw)
            if not anc or not any(MD.covers(r if d == "r" else w, rr, o + sh) for (rr, sh) in anc):
                hold += 1
    return tot, bad, hold


TY = r"[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z0-9_]+)*(?:<[^<>]*(?:<[^<>]*>)?[^<>]*>)?"
BASE = re.compile(r"^(?P<ty>" + TY + r")(?P<path>(?:\.[A-Za-z0-9_]+(?:\[[^\]]*\])?)*)"
                  r"(?:\s*\((?P<q>.+)\))?$")


def gram(specs):
    n = 0
    split = 0
    for sp in specs:
        g = {}
        for m in sp.get("mem") or []:
            b = (m.get("base") or u"").strip()
            mm = BASE.match(b)
            if not mm:
                n += 1
            k = (mm.group("ty") + (mm.group("path") or u"")) if mm else b
            g.setdefault(k, set()).add(b)
        split += sum(1 for k, s in g.items() if len(s) > 1)
    return n, split


for tag, S in ((u"전", D["specs"]), (u"후", apply(D)["specs"])):
    t, b, h = tally(S)
    n, sp = gram(S)
    g18 = sum(len(G18.check_spec(x)) for x in S)
    print(u"%s : G14 검사 %d·불일치 %d·보류 %d | base 문법위반 %d행·함수내 철자분열 %d그룹 | G18 %d"
          % (tag, t, b, h, n, sp, g18))
