# -*- coding: utf-8 -*-
u"""probe4 — 'A 가 아니라 B' / 'A 는 오답' 패턴에서 코드 토큰쌍을 뽑아 본다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

# 코드 토큰 = 백틱 인용 안의 식별자/오프셋/수치, 또는 굵게(**...**) 안의 그것
TOK = r"(?:\*\*|`)+\s*([A-Za-z_][A-Za-z0-9_:.]{2,60}|\+?0x[0-9a-fA-F]{1,5}|-?\d{2,9})\s*(?:\*\*|`)+"
PAT = re.compile(TOK + u"\\s*(?:은|는|이|가)?\\s*아니라\\s*" + TOK)

n = 0
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        blob = json.dumps(h, ensure_ascii=False).replace("\\n", " ")
        for a, b in PAT.findall(blob):
            n += 1
            print(u"[%02d] h[%d]  %-34s → %s" % (i, hi, a, b))
print(u"총 %d쌍" % n)
