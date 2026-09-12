# -*- coding: utf-8 -*-
u"""probe1 — history.now 의 취소선(~~X~~)이 표에 전파됐는지 탐색(설계용)."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

n = 0
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        blob = json.dumps(h, ensure_ascii=False)
        for m in re.finditer(r"~~([^~]{2,60})~~", blob):
            o = m.group(1).strip(u"`* ")
            n += 1
            print(u"[%02d] h[%d] ~~%s~~" % (i, hi, o))
print(u"총 취소선 %d개" % n)
