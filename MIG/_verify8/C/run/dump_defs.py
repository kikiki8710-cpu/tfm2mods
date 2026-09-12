# -*- coding: utf-8 -*-
u"""각 spec 의 IR define 헤더 원문 + 인자 분해를 그대로 찍는다(반증용 원문)."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def split_args(ln):
    at = ln.find("@")
    i = ln.find("(", at)
    depth, j = 0, i
    while j < len(ln):
        if ln[j] == "(":
            depth += 1
        elif ln[j] == ")":
            depth -= 1
            if depth == 0:
                break
        j += 1
    inner = ln[i + 1:j]
    parts, d, cur = [], 0, ""
    for ch in inner:
        if ch in "([{":
            d += 1
        elif ch in ")]}":
            d -= 1
        if ch == "," and d == 0:
            parts.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


idxs = [int(x) for x in sys.argv[1:]] or list(range(20))
for i in idxs:
    sp = D["specs"][i]
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    ln = None
    for k in range(a - 1, min(a + 6, len(src))):
        if src[k].lstrip().startswith("define"):
            ln = src[k]; lno = k + 1
            break
    ps = (sp.get("sig") or {}).get("params") or []
    args = split_args(ln)
    print(u"\n##### specs[%d] %s  %s:%d  IR인자=%d spec params=%d" % (i, sp["name"], f, lno, len(args), len(ps)))
    print(u"  define: %s" % ln.strip()[:400])
    for k, aa in enumerate(args):
        print(u"    IR %%%d : %s" % (k, aa[:150]))
    for k, p in enumerate(ps):
        print(u"    SP p[%d] i=%s name=%s" % (k, p.get("i"), p.get("name")))
    print(u"    sig.tcx = %s" % ((sp.get("sig") or {}).get("tcx") or u"(없음)")[:300])
