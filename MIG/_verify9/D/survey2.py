# -*- coding: utf-8 -*-
u"""임무① 예비2 — v2/v3 양쪽의 offset 표기 + 8차가 고쳤다는 3행의 현재 상태."""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"

for tag, fn in ((u"v3", "specs20_v3.json"), (u"v2", "specs20.json")):
    D = json.load(io.open(os.path.join(MIG, "_spec", fn), encoding="utf-8"))
    cnt = collections.Counter()
    odd = []
    tot = 0
    for i, sp in enumerate(D["specs"]):
        for j, m in enumerate(sp.get("mem") or []):
            tot += 1
            o = str(m.get("offset") if m.get("offset") is not None else u"").strip()
            if re.match(r"^0x[0-9a-fA-F]+$", o):
                cnt[u"순수16진"] += 1
            elif re.match(r"^\d+$", o):
                cnt[u"십진"] += 1
                odd.append((i, j, o, m.get("base"), m.get("name")))
            else:
                cnt[u"기타"] += 1
                odd.append((i, j, o, m.get("base"), m.get("name")))
    print(u"\n##### %s (%s) 총 %d행  %s" % (tag, fn, tot, dict(cnt)))
    for r in odd:
        print(u"   %02d mem[%-2d] %-22r base=%-30s name=%s" % r)

# 8차가 고쳤다는 3행
print(u"\n##### 8차 정정 대상 3행 현재 상태 (v3)")
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
for (i, j) in ((5, 27), (12, 13), (18, 15)):
    m = (D["specs"][i].get("mem") or [])[j]
    print(u"  specs[%d] mem[%d] = %s" % (i, j, json.dumps(m, ensure_ascii=False)))
