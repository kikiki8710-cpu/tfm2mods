# -*- coding: utf-8 -*-
u"""probe2 — 표(mem/consts/knobs) 안의 placeholder / 미확정 문면 실태조사."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

PH = re.compile(u"(미상|불명|미확정|알 수 없|모름|\\?\\?\\?|이름 미|미규명|미해결|확정 불가|불가능)")
for i, sp in enumerate(S):
    for f in ("mem", "consts", "knobs"):
        for j, x in enumerate(sp.get(f) or []):
            blob = STRIKE.sub(u" ", json.dumps(x, ensure_ascii=False))
            m = PH.search(blob)
            if m:
                nm = x.get("name") or x.get("what") or u""
                print(u"[%02d] %s[%d] %-28s  <%s>  %s" % (i, f, j, nm[:28], m.group(1), blob[:170]))
