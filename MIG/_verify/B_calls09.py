# -*- coding: utf-8 -*-
u"""B_calls09 — check_favorable_engage_formation 호출부 전수(인자 version/engage_range + 감싼 함수)."""
import io, os, re, sys
try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

TGT = "32check_favorable_engage_formation"
PAT_VER = re.compile(re.escape(TGT) + r"\(i64 noundef ([^,]+),")
PAT_RNG = re.compile(r"i64 noundef ([%\w]+)\)(?:$|\s*,\s*!)")

for f in ("m13", "m15"):
    p = r"C:\tfm2mods\_gaibc\%s.ll" % f
    cur = "?"
    for i, l in enumerate(io.open(p, encoding="utf-8", errors="ignore"), 1):
        if l.startswith("define"):
            m = re.search(r"@([A-Za-z0-9_.$]+)\(", l)
            cur = m.group(1) if m else "?"
        if TGT in l and ("call " in l or "invoke " in l):
            mv = PAT_VER.search(l)
            mr = PAT_RNG.search(l)
            print(u"%s:%-7d ver=%-8s range=%-10s  in %s" % (
                f, i, mv.group(1) if mv else "?", mr.group(1) if mr else "?", cur[:120]))
