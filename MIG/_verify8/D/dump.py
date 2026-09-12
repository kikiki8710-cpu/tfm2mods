# -*- coding: utf-8 -*-
u"""dump — 한 함수의 표를 사람이 읽을 수 있게 편다.  사용: dump.py <i> [field] [grep]"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
i = int(sys.argv[1])
fields = [sys.argv[2]] if len(sys.argv) > 2 and sys.argv[2] != "-" else ["mem", "consts", "knobs", "logic"]
pat = sys.argv[3] if len(sys.argv) > 3 else None
sp = S[i]
print(u"### [%02d] %s" % (i, sp["name"]))
for f in fields:
    v = sp.get(f)
    if isinstance(v, list):
        for j, x in enumerate(v):
            b = json.dumps(x, ensure_ascii=False)
            if pat and not re.search(pat, b):
                continue
            print(u"%s[%d] %s" % (f, j, b))
    elif v:
        b = v if isinstance(v, str) else json.dumps(v, ensure_ascii=False)
        if pat:
            for ln in b.split("\n"):
                if re.search(pat, ln):
                    print(u"%s| %s" % (f, ln))
        else:
            print(u"%s = %s" % (f, b))
