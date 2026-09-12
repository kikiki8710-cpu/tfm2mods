# -*- coding: utf-8 -*-
u"""배치A 전수감사 보조: IR 범위에서 gep 오프셋·비교상수·store 를 뽑아
명세의 `mem`/`consts` 와 양방향 대조한다(명세→IR 과 **IR→명세** 둘 다)."""
import json, io, os, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
IRDIR = r"C:\tfm2mods\_gaibc"
V3 = json.load(io.open(os.path.join(ROOT, "_spec", "specs20_v3.json"), encoding="utf-8"))

GEP = re.compile(r"getelementptr[^,]*,\s*ptr\s+(%?[\w.@]+),\s*i64\s+(-?\d+)")
ICMP = re.compile(r"icmp\s+\w+\s+i\d+\s+(%?[\w.-]+),\s*(-?\d+)")
ARITH = re.compile(r"=\s*(add|sub|mul|shl|lshr|ashr|and|or|xor|udiv|sdiv|urem|srem|select)\b[^\n]*?(-?\d+)\s*$")
STORE = re.compile(r"store\s+(?:volatile\s+)?([\w<>\[\] x*]+?)\s+(\S+),\s*ptr\s+(\S+)")
LOAD = re.compile(r"=\s*load\s+([\w<>\[\] x*]+),\s*ptr\s+(\S+)")
DBG = re.compile(r"!dbg !(\d+)")


def lines(sp):
    ir = sp["ir"]
    src = io.open(os.path.join(IRDIR, ir["file"]), encoding="utf-8", errors="replace").read().split("\n")
    return src, ir["frm"], ir["to"], ir["file"]


def main(i):
    sp = V3["specs"][i]
    src, a, b, f = lines(sp)
    body = src[a - 1:b]
    print(u"=== [%02d] %s  %s:%d~%d  (%d줄)" % (i, sp["name"], f, a, b, len(body)))
    # gep 오프셋 히스토그램
    offs = collections.Counter()
    for k, ln in enumerate(body):
        for m in GEP.finditer(ln):
            offs[int(m.group(2))] += 1
    memoffs = set()
    for r in sp.get("mem") or []:
        o = (r.get("offset") or "").strip()
        try:
            memoffs.add(int(o, 16) if o.lower().startswith("0x") else int(o))
        except Exception:
            pass
    print(u"-- gep 오프셋(IR) %d종; 명세 mem 오프셋 %d종" % (len(offs), len(memoffs)))
    print(u"   IR에만: " + u", ".join("0x%x(x%d)" % (o, offs[o]) for o in sorted(offs) if o not in memoffs))
    print(u"   명세에만: " + u", ".join("0x%x" % o for o in sorted(memoffs) if o not in offs))
    # 상수
    lits = collections.Counter()
    for ln in body:
        for m in ICMP.finditer(ln):
            lits[int(m.group(2))] += 1
        m = ARITH.search(ln.rstrip().split(", !dbg")[0])
        if m:
            lits[int(m.group(2))] += 1
    cv = set()
    for c in sp.get("consts") or []:
        try:
            cv.add(int(c.get("value")))
        except Exception:
            pass
    print(u"-- 비교/산술 리터럴(IR) %s" % (u", ".join("%d(x%d)" % (v, lits[v]) for v in sorted(lits))))
    print(u"   명세 consts 값: %s" % (u", ".join(str(v) for v in sorted(cv))))
    print(u"   IR에만: %s" % (u", ".join(str(v) for v in sorted(lits) if v not in cv)))
    print(u"   명세에만: %s" % (u", ".join(str(v) for v in sorted(cv) if v not in lits)))
    # store 유무 (dir=w 검산)
    st = [(a + k, ln.strip()[:150]) for k, ln in enumerate(body) if re.search(r"^\s*store ", ln)]
    print(u"-- store %d건" % len(st))
    for n, s in st[:25]:
        print(u"   %d: %s" % (n, s))


for arg in sys.argv[1:]:
    main(int(arg))
    print()
