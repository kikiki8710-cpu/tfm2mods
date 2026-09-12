# -*- coding: utf-8 -*-
u"""probe3 — history.now 의 '정정 선언' 분포와 앵커(오프셋/심볼/rs줄) 실태."""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

CORR = re.compile(u"(정정|틀렸다|오답|아니다|실제는|잘못|뒤집|반대다|다르다|삭제|"
                  u"존재하지 않는|없다|폐기|아니라)")
OFF = re.compile(r"\+?0x[0-9a-fA-F]{1,4}\b")
SYM = re.compile(r"\b([a-z_][a-z0-9_]{4,45})\b")
RS = re.compile(r"([a-z_][a-z0-9_]*\.rs):(\d{2,5})")

c = Counter()
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        now = json.dumps({k: v for k, v in h.items() if k != "was"}, ensure_ascii=False)
        if CORR.search(now):
            c["corr"] += 1
        c["all"] += 1
print(c)

# 앵커: history 전체가 말하는 오프셋 집합 vs 표가 가진 오프셋 집합
for i, sp in enumerate(S):
    hoff = set()
    for h in sp.get("history") or []:
        hoff |= set(x.lstrip("+").lower() for x in OFF.findall(json.dumps(h, ensure_ascii=False)))
    toff = set()
    for f in ("mem", "consts", "knobs"):
        for x in sp.get(f) or []:
            o = (x.get("offset") or u"") if isinstance(x, dict) else u""
            if o:
                toff.add(o.lstrip("+").lower())
    print(u"[%02d] history오프셋 %d · mem오프셋 %d · history에만 %d"
          % (i, len(hoff), len(toff), len(hoff - toff)))
