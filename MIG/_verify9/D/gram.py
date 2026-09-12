# -*- coding: utf-8 -*-
u"""임무① — 규약 문법 위반 전수.

**규약 D9-OFF**
  `offset` = `base` 가 이름 붙인 **좌표 원점**에서의 바이트 변위 · `^0x[0-9a-f]+$` 하나.
  `base`   = `<타입>[.<필드경로>][(<한정어>)]`  — 타입이 **맨 앞**, 한정어는 **괄호 안**.
             한정어는 *같은 함수 안에서 인스턴스를 갈라야 할 때만*. 크기·분기·SSA 이름은 note 로.
"""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

TY = r"[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z0-9_]+)*(?:<[^<>]*(?:<[^<>]*>)?[^<>]*>)?"
BASE = re.compile(r"^(?P<ty>" + TY + r")"
                  r"(?P<path>(?:\.[A-Za-z0-9_]+(?:\[[^\]]*\])?)*)"
                  r"(?:\s*\((?P<q>.+)\))?$")
OFF = re.compile(r"^0x[0-9a-f]+$")

bad_off, bad_base = [], []
for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        o = str(m.get("offset") if m.get("offset") is not None else u"")
        if not OFF.match(o):
            bad_off.append((i, j, o, m.get("base")))
        b = (m.get("base") or u"").strip()
        if not BASE.match(b):
            bad_base.append((i, j, b, m.get("offset"), m.get("name")))

print(u"== offset 문법 위반 %d행" % len(bad_off))
for r in bad_off:
    print(u"   %02d mem[%-2d] %-14r base=%r" % r)
print(u"\n== base 문법 위반 %d행" % len(bad_base))
seen = collections.Counter()
for (i, j, b, o, n) in bad_base:
    seen[b] += 1
    print(u"   %02d mem[%-2d] %-42r off=%-8s %s" % (i, j, b, o, n))
print(u"\n   철자별: %s" % dict(seen))

# 함수 안 철자 분열(한정어 무시한 머리 기준)
print(u"\n== 함수 안 좌표계 철자 분열")
n = 0
for i, sp in enumerate(D["specs"]):
    g = collections.defaultdict(set)
    for j, m in enumerate(sp.get("mem") or []):
        b = (m.get("base") or u"").strip()
        mm = BASE.match(b)
        k = (mm.group("ty") + (mm.group("path") or u"")) if mm else b
        g[k].add(b)
    for k, s in sorted(g.items()):
        if len(s) > 1:
            n += 1
            print(u"   specs[%-2d] %-24s → %s" % (i, k, sorted(s)))
print(u"   총 %d 그룹" % n)
