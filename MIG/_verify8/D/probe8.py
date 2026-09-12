# -*- coding: utf-8 -*-
u"""probe8 — history 가 선언한 `ev N→M` 이 표의 ev 에 반영됐는가.

근거: `ev` 는 표의 **정수 필드**라 자유 서술과 달리 완전 기계 대조가 된다.
      history 는 증거등급을 올릴 때 거의 항상 `ev 4→2` 꼴로 적는다(문면 규약).
"""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]

EV = re.compile(u"ev\\s*(\\d)\\s*(?:->|→|=>)\\s*\\*{0,2}(?:ev)?\\s*(\\d)")
FIELD = {u"constants": "consts", u"consts": "consts", u"knobs": "knobs",
         u"new_knobs": "knobs", u"mem": "mem", u"reads": "mem", u"writes": "mem"}
SLOT = re.compile(u"(constants|consts|knobs|new_knobs|mem|reads|writes)\\s*\\[\\s*(\\d+)\\s*\\]")
c = Counter()
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        blob = json.dumps(h, ensure_ascii=False)
        evs = EV.findall(blob)
        if not evs:
            continue
        c["decl"] += 1
        slots = SLOT.findall(blob)
        tgt = sorted(set((_from, _to) for _from, _to in evs))
        print(u"[%02d] h[%d] ev선언 %s  슬롯지목 %s" % (i, hi, tgt, slots))
        for fname, idx in slots:
            f = FIELD[fname]
            rows = sp.get(f) or []
            j = int(idx)
            if j < len(rows):
                print(u"      → 현재 %s[%d].ev = %s" % (f, j, rows[j].get("ev")))
print(c)
