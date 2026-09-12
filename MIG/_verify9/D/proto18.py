# -*- coding: utf-8 -*-
u"""G18 프로토타입 — `logic` 이 인용한 오프셋이 표에 있나. **먼저 규모를 잰다.**"""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
STRIKE = re.compile(r"~~(?!~).+?~~")

# 주소 문맥의 오프셋 인용만: `X+0x..` · `+0x..` · `@+0x..` · `@0x..`
CITE = re.compile(r"(?:@\s*)?\+\s*(0x[0-9a-fA-F]+)|@\s*(0x[0-9a-fA-F]+)")

tot = miss = 0
per = collections.Counter()
for i, sp in enumerate(D["specs"]):
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        continue
    offs = {(a or b).lower() for a, b in CITE.findall(lg)}
    tbl = set()
    for f in ("mem", "consts", "knobs"):
        for x in sp.get(f) or []:
            o = str(x.get("offset") or u"").lower()
            if o:
                tbl.add(o)
    blob = json.dumps({k: sp.get(k) for k in ("mem", "consts", "knobs", "sig", "notes", "open",
                                              "history", "closed", "callees", "resolved")},
                      ensure_ascii=False).lower()
    for o in sorted(offs, key=lambda s: int(s, 16)):
        tot += 1
        if o in tbl:
            continue
        if o in blob:                 # 표 어딘가(note/name/value)에 문면으로 있다
            per[u"표 문면에만 있음"] += 1
            continue
        miss += 1
        per[u"**표에 없음**"] += 1
        print(u"  specs[%-2d] %-28s %-8s" % (i, sp["name"], o))
print(u"\n오프셋 인용 %d · 표 offset 칸 일치 %d · 문면만 %d · **없음 %d**"
      % (tot, tot - per[u"표 문면에만 있음"] - miss, per[u"표 문면에만 있음"], miss))
