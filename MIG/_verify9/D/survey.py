# -*- coding: utf-8 -*-
u"""임무① 예비 — `mem[].offset` 451행 전량의 **표기 형태**를 전수 분류한다."""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

FORMS = [
    (u"A 순수16진", re.compile(r"^0x[0-9a-fA-F]+$")),
    (u"B 십진", re.compile(r"^\d+$")),
    (u"C 16진+인덱스", re.compile(r"^0x[0-9a-fA-F]+\[.+\]$")),
    (u"D 16진+연산", re.compile(r"^0x[0-9a-fA-F]+\s*[+\-*].+$")),
    (u"E 16진+화살표", re.compile(r"^0x[0-9a-fA-F]+\s*(?:->|→|>>).+$")),
    (u"F 다중16진", re.compile(r"^0x[0-9a-fA-F]+(?:\s*[/,]\s*0x[0-9a-fA-F]+)+$")),
    (u"G 비움", re.compile(r"^(?:-|—|)$")),
]

cnt = collections.Counter()
ex = collections.defaultdict(list)
tot = 0
for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        tot += 1
        o = str(m.get("offset") if m.get("offset") is not None else u"").strip()
        hit = None
        for nm, rx in FORMS:
            if rx.match(o):
                hit = nm
                break
        if hit is None:
            hit = u"Z 기타"
        cnt[hit] += 1
        ex[hit].append((i, j, o, m.get("base"), m.get("name")))

print(u"총 %d행" % tot)
for k in sorted(cnt):
    print(u"\n== %s : %d" % (k, cnt[k]))
    for (i, j, o, b, n) in ex[k][:200 if k.startswith((u"C", u"D", u"E", u"F", u"G", u"Z")) else 6]:
        print(u"   %02d mem[%-2d] %-24r base=%-28s name=%s" % (i, j, o, b, n))
