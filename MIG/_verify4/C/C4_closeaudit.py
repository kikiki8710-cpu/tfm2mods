# -*- coding: utf-8 -*-
u"""closelist 의 needle 이 실제로 무언가를 닫았는지 감사한다.
CLOSE 는 (index, needle) 로 부분문자열 매칭하고, **매칭이 0건이면 조용히 no-op** 이다.
= siblings 망글링 파싱 실패·G3 plan==null 통과와 같은 부류의 '침묵 실패'."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, os.path.join(MIG, "_spec"))
import closelist as CL

V2 = json.load(io.open(os.path.join(MIG, "_spec", "specs20.json"), encoding="utf-8"))["specs"]
V3 = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))["specs"]


def opens_v2(i):
    sp = V2[i]
    return list(sp.get("unknown") or []) + list(sp.get("still_unknown") or [])


dead, mis = [], []
for j, needle, why in CL.CLOSE:
    hit = [t for t in opens_v2(j) if needle in t]
    if hit:
        continue
    other = [k for k in range(len(V2)) if k != j and any(needle in t for t in opens_v2(k))]
    (mis if other else dead).append((j, needle, other))

print(u"## CLOSE 총 %d개" % len(CL.CLOSE))
print(u"## 아무것도 매칭하지 않는 needle: %d개" % (len(dead) + len(mis)))
print(u"\n### (A) 다른 index 에서 매칭된다 = **index 오기** %d건" % len(mis))
for j, n, other in mis:
    print(u"  CLOSE(i=%d, '%s')  -> 실제 매칭 index %s  [%s]"
          % (j, n, other, u" / ".join(V2[k]["name"] for k in other)))
print(u"\n### (B) 어느 index 에서도 매칭 0 = 오타/문면변경 %d건" % len(dead))
for j, n, _ in dead:
    print(u"  CLOSE(i=%d, '%s')" % (j, n))

# 그 결과 v3 에 남아 있는 open 을 짚는다
print(u"\n### (C) index 오기 때문에 v3 에 아직 살아 있는 open")
for j, n, other in mis:
    for k in other:
        for q, o in enumerate(V3[k].get("open") or []):
            if n in o["q"]:
                print(u"  /specs[%d]/open[%d]  (needle '%s' 가 i=%d 로 잘못 등록)" % (k, q, n, j))
                print(u"      %s" % o["q"][:150])
