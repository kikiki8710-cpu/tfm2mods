# -*- coding: utf-8 -*-
u"""임무① — **전역**(함수 경계 무시) 철자 분열 + 같은 (머리, 오프셋) 이 같은 필드를 가리키는지 대조.

좌표계가 같다는 증거 = **같은 오프셋에 같은 필드명**이 나오는 것. 이게 성립해야 철자를 합칠 수 있다.
"""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def head(b):
    s = re.sub(r"\s*\(.*?\)\s*", u" ", (b or u"").strip())
    s = re.sub(r"^dyn\s+", u"", s)
    s = re.sub(r"^bumpalo\s+", u"", s)
    s = re.sub(r"\s*::\s*vtable$", u" vtable", s)
    return re.sub(r"\s+", u" ", s).strip()


g = collections.defaultdict(lambda: collections.defaultdict(list))
for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        g[head(m.get("base"))][m.get("base")].append((i, j, m.get("offset"), m.get("name")))

for h in sorted(g):
    if len(g[h]) < 2:
        continue
    print(u"\n##### 머리 %r — 철자 %d종" % (h, len(g[h])))
    off = collections.defaultdict(set)
    for b, rows in sorted(g[h].items()):
        print(u"   %-38r x%d  specs=%s" % (b, len(rows), sorted({r[0] for r in rows})))
        for (i, j, o, n) in rows:
            print(u"        %02d mem[%-2d] %-8s %s" % (i, j, o, n))
            off[o].add((b, (n or u"").split(u"(")[0].strip()))
    for o in sorted(off):
        if len({x[0] for x in off[o]}) > 1:
            nm = {x[1] for x in off[o]}
            print(u"     ⟹ 같은 오프셋 %s 에 철자 %d종, 필드명 %s %s"
                  % (o, len({x[0] for x in off[o]}), u"일치" if len(nm) == 1 else u"불일치", sorted(nm)))
