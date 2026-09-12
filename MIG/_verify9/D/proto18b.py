# -*- coding: utf-8 -*-
u"""G18 프로토타입 ② — `logic` 이 인용한 **상수 리터럴**·**필드명**이 표에 있나. 규모 측정."""
import io, json, os, re, sys, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
STRIKE = re.compile(r"~~(?!~).+?~~")

# ── ② 상수: 비교/대입의 우변 리터럴 (주소 문맥 `+0x..` 은 제외)
NUM = re.compile(r"(?<![\w.])(?:[<>]=?|[!=]=|\bmin\(|\bmax\(|=\s)\s*(-?\d{2,9})(?![\w.])")
# ── ③ 필드: `ident.field` 사슬의 마지막 마디
FLD = re.compile(r"\b[a-z_][a-z0-9_]*(?:\.[a-z_][a-z0-9_]*)+\b(?!\s*\()")

tn = tm = fn = fm = 0
for i, sp in enumerate(D["specs"]):
    lg = STRIKE.sub(u" ", sp.get("logic") or u"")
    if not lg:
        continue
    lg2 = re.sub(r"\+\s*0x[0-9a-fA-F]+", u" ", lg)
    cv = set()
    for f in ("consts", "knobs"):
        for x in sp.get(f) or []:
            for k in ("value", "what", "name", "note", "default", "range"):
                v = x.get(k)
                if v is not None:
                    cv |= {n for n in re.findall(r"-?\d+", str(v))}
    for n in sorted({m for m in NUM.findall(lg2)}):
        tn += 1
        if n.lstrip("-") not in cv and n not in cv:
            tm += 1
            print(u"  [상수] specs[%-2d] %-26s %s" % (i, sp["name"], n))
    names = u" ".join([str(x.get("name") or u"") for x in (sp.get("mem") or [])])
    for p in sorted(set(FLD.findall(lg))):
        last = p.split(u".")[-1]
        if len(last) < 4:
            continue
        fn += 1
        if last not in names:
            fm += 1
print(u"\n상수 인용 %d · 표에 없음 %d" % (tn, tm))
print(u"필드 인용 %d · mem.name 에 없음 %d" % (fn, fm))
