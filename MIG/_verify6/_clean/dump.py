# -*- coding: utf-8 -*-
u"""취소선 37곳의 전문 + v3 경로를 덤프한다(읽기 전용)."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC = os.path.join(HERE, "_spec", "specs20.json")
STRIKE = re.compile(u"~~[^~]{1,200}?~~")
TARGET_KEYS = ("logic", "one_line")
ROW_FIELDS = ("reads", "writes", "constants", "knobs", "new_knobs")
ROW_KEYS = ("note", "meaning", "effect", "what", "where", "value", "name")

D = json.load(io.open(SRC, encoding="utf-8"))
n = 0
only = sys.argv[1:] if len(sys.argv) > 1 else None
for i, sp in enumerate(D["specs"]):
    nr = len(sp.get("reads") or [])
    nk = len(sp.get("knobs") or [])
    for k in TARGET_KEYS:
        v = sp.get(k)
        if isinstance(v, str) and STRIKE.search(v):
            n += 1
            if only and str(i) not in only:
                continue
            print(u"\n" + u"#" * 100)
            print(u"[%02d] v2=specs[%d].%s   v3path=/specs[%d]/%s" % (n, i, k, i, k))
            print(u"#" * 100)
            print(v)
    for f in ROW_FIELDS:
        for j, x in enumerate(sp.get(f) or []):
            if not isinstance(x, dict):
                continue
            for k in ROW_KEYS:
                v = x.get(k)
                if isinstance(v, str) and STRIKE.search(v):
                    n += 1
                    if only and str(i) not in only:
                        continue
                    if f == "reads":
                        v3 = u"/specs[%d]/mem[%d]/%s" % (i, j, k)
                    elif f == "writes":
                        v3 = u"/specs[%d]/mem[%d]/%s" % (i, nr + j, k)
                    elif f == "constants":
                        v3 = u"/specs[%d]/consts[%d]/%s" % (i, j, k)
                    elif f == "knobs":
                        v3 = u"/specs[%d]/knobs[%d]/%s" % (i, j, k)
                    else:
                        v3 = u"/specs[%d]/knobs[%d]/%s" % (i, nk + j, k)
                    print(u"\n" + u"#" * 100)
                    print(u"[%02d] v2=specs[%d].%s[%d].%s   v3path=%s" % (n, i, f, j, k, v3))
                    print(u"#" * 100)
                    print(v)
print(u"\n총 %d곳" % n)
