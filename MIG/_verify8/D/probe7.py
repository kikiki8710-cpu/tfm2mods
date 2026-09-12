# -*- coding: utf-8 -*-
u"""probe7 — history 가 **표 칸을 이름으로 지목**하는 경우를 전수 수집한다.

근거: 이 축의 정정은 대개 '어느 칸이 틀렸다'를 명시한다(`constants[21]`, `knobs[4]`, `mem 표`, `logic`).
      지목이 있으면 앵커가 확정되므로 기계 대조가 가능하다.
"""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]

SLOT = re.compile(r"\b(constants|consts|knobs|new_knobs|mem|reads|writes|logic|sig|notes|unknown|still_unknown|resolved)\s*(?:\[\s*(\d+)\s*\])?")
c = Counter()
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        blob = json.dumps(h, ensure_ascii=False)
        hits = set(SLOT.findall(blob))
        if not hits:
            continue
        named = sorted(x for x in hits if x[1])
        c["ent"] += 1
        if named:
            c["indexed"] += 1
            print(u"[%02d] h[%d]  %s   | %s"
                  % (i, hi, ", ".join(u"%s[%s]" % x for x in named),
                     re.sub(r"\s+", " ", (h.get("was") or u""))[:90]))
print(c)
