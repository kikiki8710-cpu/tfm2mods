# -*- coding: utf-8 -*-
u"""role 원문 덤프 — 백틱 조각·인용 추출까지."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
CITE = re.compile(r"(m\d+\.ll):(\d+)((?:\s*[/~\u00b7,]\s*\d+)*)")
for arg in sys.argv[1:]:
    i, j = [int(x) for x in arg.split(":")]
    sp = D["specs"][i]
    p = sp["sig"]["params"][j]
    print(u"=== specs[%d] %s p[%d] name=%r i=%r" % (i, sp["name"], j, p.get("name"), p.get("i")))
    print(p.get("role"))
    print(u"--- 백틱 조각:")
    for q in re.findall(r"`([^`]{1,200})`", p.get("role") or u""):
        print(u"    [%d] %r" % (len(q), q))
    print(u"--- 인용:")
    for m in CITE.finditer(p.get("role") or u""):
        print(u"    %r at %d" % (m.group(0), m.start()))
    print()
