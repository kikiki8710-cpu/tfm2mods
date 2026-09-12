# -*- coding: utf-8 -*-
u"""probe14 — 안③(memn 식별자 토큰화)을 적용한 gate9 를 그대로 재현해 실제 건수를 잰다.
   ⚠`specgate.py` 는 고치지 않는다(다른 배치 몫). 여기서 사본으로만 돌린다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, MIG)
import spec3lib as L

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
DS = json.load(io.open(os.path.join(MIG, "distruct.json"), encoding="utf-8"))
FIELDIDX = {}
for s, r in DS.items():
    for f in (r or {}).get("fields") or []:
        FIELDIDX.setdefault(f.get("name"), []).append((s, f.get("type") or u""))
STRIKE = re.compile(r"~~(?!~).+?~~")
FIELDPATH = re.compile(r"\.([a-z_][a-z0-9_]{3,45})\b(?!\s*\()")
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")


def isfn(n):
    try:
        return bool(L.fnlookup(n))
    except Exception:
        return False


for mode in (u"현행(name.split('.'))", u"안③(식별자 토큰화)"):
    tot, names = 0, []
    for i, sp in enumerate(S):
        lg = STRIKE.sub(u" ", sp.get("logic") or u"")
        if not lg:
            continue
        have = set((c.get("name") or u"") for c in (sp.get("callees") or []))
        memn = set()
        for x in sp.get("mem") or []:
            nm = x.get("name") or u""
            if mode.startswith(u"현행"):
                for p in nm.split("."):
                    memn.add(p)
            else:
                for p in IDENT.findall(nm):
                    memn.add(p)
        amb = [m for m in sorted(set(FIELDPATH.findall(lg)))
               if m not in have and m not in memn and isfn(m)]
        if amb:
            tot += 1
            names.append(u"[%02d] %s" % (i, ", ".join(amb)))
    print(u"\n%s → G9 손확인 %d건" % (mode, tot))
    for n in names:
        print(u"   " + n)
