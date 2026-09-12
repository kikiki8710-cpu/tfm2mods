# -*- coding: utf-8 -*-
u"""probe10 — (E) base 를 필수로 건 오프셋↔이름 결속 대조 / (F) '거짓 선언된 인용문' 잔존."""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")
BIND_A = re.compile(r"\+(0x[0-9a-fA-F]{1,5})\s*=\s*(?:[A-Za-z_][A-Za-z0-9_]*::)?([a-z_][a-z0-9_]{3,45})")
BIND_B = re.compile(r"([a-z_][a-z0-9_]{3,45})\s*\(\s*\+(0x[0-9a-fA-F]{1,5})")
BASE = re.compile(r"\b([A-Z][A-Za-z0-9_]{4,40})\s*\+\s*(0x[0-9a-fA-F]{1,5})")
c = Counter()
print(u"── E: base+offset 동시 일치 결속 대조 ──")
for i, sp in enumerate(S):
    byk = {}
    for j, x in enumerate(sp.get("mem") or []):
        b = (x.get("base") or u"").split("(")[0].strip()
        o = (x.get("offset") or u"").lower().lstrip("+")
        if b and o:
            byk.setdefault((b, o), []).append((j, x))
    for hi, h in enumerate(sp.get("history") or []):
        blob = STRIKE.sub(u" ", json.dumps(h, ensure_ascii=False).replace("\\n", " "))
        for bs, off in set(BASE.findall(blob)):
            rows = byk.get((bs, off.lower()))
            if not rows:
                continue
            # 그 base+offset 문맥에서 history 가 말한 이름 후보
            names = set()
            for o2, n2 in BIND_A.findall(blob):
                if o2.lower() == off.lower():
                    names.add(n2.lower())
            for n2, o2 in BIND_B.findall(blob):
                if o2.lower() == off.lower():
                    names.add(n2.lower())
            if not names:
                continue
            for j, x in rows:
                rb = STRIKE.sub(u" ", json.dumps(x, ensure_ascii=False)).lower()
                if not any(n in rb for n in names):
                    c["E"] += 1
                    print(u"[%02d] h[%d] %s+%s = %s  ↔ mem[%d].name=%s"
                          % (i, hi, bs, off, sorted(names), j, x.get("name")))

print(u"\n── F: '거짓/오답' 으로 선언된 인용문이 표에 살아 있는가 ──")
FALSE = re.compile(u"[「『\"']([^」』\"']{6,70})[」』\"']\\s*(?:은|는|이|가)?\\s*"
                   u"\\*{0,2}(거짓|오답|틀렸|잘못|과했|아니)")
for i, sp in enumerate(S):
    for hi, h in enumerate(sp.get("history") or []):
        blob = STRIKE.sub(u" ", json.dumps(h, ensure_ascii=False))
        for q, _v in FALSE.findall(blob):
            for f in ("mem", "consts", "knobs", "logic"):
                v = sp.get(f)
                rows = v if isinstance(v, list) else ([v] if v else [])
                for j, x in enumerate(rows):
                    b = STRIKE.sub(u" ", x if isinstance(x, str) else json.dumps(x, ensure_ascii=False))
                    if q in b:
                        c["F"] += 1
                        print(u"[%02d] h[%d] %s[%d] 에 거짓선언된 인용문 잔존: %s" % (i, hi, f, j, q[:60]))
print(c)
