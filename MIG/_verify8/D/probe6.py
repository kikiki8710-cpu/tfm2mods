# -*- coding: utf-8 -*-
u"""probe6 — history 가 선언한 (오프셋 → 이름) 결속이 mem 표에 그대로 있는가."""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

#  A) `+0x118 = EffectType::on_caster`      B) `nearest_enemy`(Entity +0x88)
BIND_A = re.compile(r"\+(0x[0-9a-fA-F]{1,5})\s*=\s*(?:[A-Za-z_][A-Za-z0-9_]*::)?([a-z_][a-z0-9_]{3,45})")
BIND_B = re.compile(r"([a-z_][a-z0-9_]{3,45})\s*\(\s*\+(0x[0-9a-fA-F]{1,5})")
c = Counter()
for i, sp in enumerate(S):
    memrows = [(j, x) for j, x in enumerate(sp.get("mem") or [])]
    byoff = {}
    for j, x in memrows:
        o = (x.get("offset") or u"").lower().lstrip("+")
        if o:
            byoff.setdefault(o, []).append((j, x))
    for hi, h in enumerate(sp.get("history") or []):
        blob = STRIKE.sub(u" ", json.dumps(h, ensure_ascii=False).replace("\\n", " "))
        binds = set()
        for o, n in BIND_A.findall(blob):
            binds.add((o.lower(), n.lower()))
        for n, o in BIND_B.findall(blob):
            binds.add((o.lower(), n.lower()))
        for o, n in sorted(binds):
            c["bind"] += 1
            rows = byoff.get(o)
            if not rows:
                c["no_mem_row"] += 1
                continue
            ok = any(n in STRIKE.sub(u" ", json.dumps(x, ensure_ascii=False)).lower()
                     for _j, x in rows)
            if not ok:
                c["MISMATCH"] += 1
                print(u"[%02d] h[%d] history: +%s = %s   ↔ mem[%s] = %s"
                      % (i, hi, o, n, ",".join(str(j) for j, _ in rows),
                         " | ".join((x.get("name") or u"")[:40] for _, x in rows)))
print(c)
