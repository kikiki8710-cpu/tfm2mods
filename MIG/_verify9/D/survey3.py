# -*- coding: utf-8 -*-
u"""임무① — `base` 칸 표기 형태 전수. offset 이 **무엇 기준인지**를 정하는 것은 base 다."""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

bases = collections.Counter()
where = collections.defaultdict(list)
for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        b = str(m.get("base") or u"").strip()
        bases[b] += 1
        where[b].append((i, j, m.get("offset"), m.get("name")))

print(u"distinct base = %d" % len(bases))
PAT = [
    (u"순수타입", re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")),
    (u"괄호주석", re.compile(r"^[A-Za-z_][A-Za-z0-9_]*\s*\(.+\)$")),
    (u"경로", re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z0-9_]+)+.*$")),
]
grp = collections.defaultdict(list)
for b in bases:
    hit = u"기타"
    for nm, rx in PAT:
        if rx.match(b):
            hit = nm
            break
    grp[hit].append(b)
for k in sorted(grp):
    print(u"\n===== %s : %d종 / %d행" % (k, len(grp[k]), sum(bases[b] for b in grp[k])))
    for b in sorted(grp[k]):
        print(u"   %-46s x%-3d  ex: %s" % (b, bases[b], where[b][0][2:]))
