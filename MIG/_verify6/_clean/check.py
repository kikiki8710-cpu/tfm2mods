# -*- coding: utf-8 -*-
u"""check — 정본을 **건드리지 않고** 패치를 메모리에 적용해
① applypatch 의 실제 적용 결과 ② cleanspec 의 ev 분포 ③ 남은 취소선을 본다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, MIG)
SRC = os.path.join(MIG, "_spec", "specs20.json")
import applypatch as AP
import cleanspec as CS

D = json.load(io.open(SRC, encoding="utf-8"))
pj = json.load(io.open(os.path.join(HERE, "patch.json"), encoding="utf-8"))
log, skipped, ok, ng = [], [], 0, 0
for e in pj["errors"]:
    r = AP.apply_error(D, e, log, skipped)
    if r is None:
        continue
    ok += r
    ng += (not r)
print(u"적용 성공 %d / 실패 %d / 이미적용(건너뜀) %d" % (ok, ng, len(skipped)))
for w in log:
    print(u"   ✗ %s | %s | %s" % w)
for s in skipped:
    print(u"   ~ 건너뜀 %s" % s)

tmp = os.path.join(HERE, "_after.json")
io.open(tmp, "w", encoding="utf-8").write(json.dumps(D, ensure_ascii=False, indent=1))
b, a = CS.evdist(SRC), CS.evdist(tmp)
print(u"\nev 분포 (정본 → 패치 적용 후)")
bad = False
for t in (1, 2, 3, 4, 5):
    m = u""
    if t <= 3 and a.get(t, 0) < b.get(t, 0):
        m = u"  ★강등 %d행" % (b.get(t, 0) - a.get(t, 0)); bad = True
    print(u"   ev%d  %4d → %4d%s" % (t, b.get(t, 0), a.get(t, 0), m))

# cleanspec 과 **완전히 같은 기준**으로 남은 취소선을 센다
left = []
for i, sp in enumerate(D["specs"]):
    for k in CS.TARGET_KEYS:
        if isinstance(sp.get(k), str) and CS.STRIKE_LEFT.search(sp[k]):
            left.append(u"specs[%d].%s" % (i, k))
    for f in CS.ROW_FIELDS:
        for j, x in enumerate(sp.get(f) or []):
            if not isinstance(x, dict):
                continue
            for k in CS.ROW_KEYS:
                if isinstance(x.get(k), str) and CS.STRIKE_LEFT.search(x[k]):
                    left.append(u"specs[%d].%s[%d].%s" % (i, f, j, k))
print(u"\n남은 취소선 필드(cleanspec 기준) = %d" % len(left))
for s in left:
    print(u"   %s" % s)
os.remove(tmp)
sys.exit(1 if (ng or bad or left) else 0)
