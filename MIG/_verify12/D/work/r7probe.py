# -*- coding: utf-8 -*-
u"""R7 후보 — **판정 전파 누락**: 같은 (base,offset) 을 적은 행들 중 한쪽만
「죽은 슬롯 / 소비처 0건 / 노브 아님」 류 확정을 달고 있으면 나머지는 안 따라간 것이다."""
import io, json, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import sharedchk as S
D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]
DEAD = re.compile(u"\uC18C\uBE44\uCC98 0\uAC74|\uC8FD\uC740 \uC2AC\uB86F|\uC544\uBB34\uB3C4 \uC77D\uC9C0 \uC54A|"
                  u"\uB178\uBE0C\uAC00 \uC544\uB2C8|\uC77D\uB294 \uCF54\uB4DC\uAC00 .{0,6}0\uAC74|\uC790\uB9AC\uCC44\uC6C0")
g = collections.defaultdict(list)
for i, sp in enumerate(D):
    for j, r in enumerate(sp.get("mem") or []):
        b, o = S.norm_base(r.get("base")), S.norm_off(r.get("offset"))
        if b and o:
            g[(b, o)].append((i, j, r))
n = 0
for k, v in sorted(g.items()):
    if len(set(x[0] for x in v)) < 2:
        continue
    flags = {}
    for (i, j, r) in v:
        t = u" ".join(str(r.get(x) or u"") for x in ("name", "note", "value"))
        flags.setdefault(i, False)
        flags[i] = flags[i] or bool(DEAD.search(t))
    if any(flags.values()) and not all(flags.values()):
        n += 1
        print(u"  %s %s" % k)
        for i, f in sorted(flags.items()):
            print(u"     [%02d] %s  %s" % (i, u"확정있음" if f else u"없음 ←전파누락 후보", D[i]["name"][:34]))
print(u"R7 후보 %d건" % n)
