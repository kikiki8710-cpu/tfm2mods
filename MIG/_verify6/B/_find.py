# -*- coding: utf-8 -*-
u"""정정 대상 `old` 문면을 **정본에서 그대로 떠 온다**(손으로 옮겨 적다 틀리는 것을 막는다)."""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
V3 = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
V2 = json.load(io.open(os.path.join(MIG, "_spec", "specs20.json"), encoding="utf-8"))

print("== 09 logic 1302/1307 ==")
lg = V3["specs"][9]["logic"]
k = lg.index(u"else if is_significant_flank")
print(repr(lg[k - 4:k + 240]))

print("\n== 08 logic L183 ==")
lg8 = V3["specs"][8]["logic"]
k = lg8.index(u"None      => return true")
print(repr(lg8[k - 4:k + 120]))
k = lg8.index(u"(c) setup")
print(repr(lg8[k - 6:k + 80]))

print("\n== 06 history(=v2 resolved) 인덱스 대조 ==")
h3 = V3["specs"][6].get("history") or []
r2 = V2["specs"][6].get("resolved") or []
print("v3 history=%d · v2 resolved=%d" % (len(h3), len(r2)))
for j, x in enumerate(h3):
    print("  h[%d] was=%s" % (j, (x.get("was") or "")[:40]))
for j, x in enumerate(r2):
    print("  r[%d] was=%s" % (j, (x.get("was") or "")[:40]))
j = [i for i, x in enumerate(h3) if u"오라클로 실행 검증" in (x.get("was") or "")][0]
print("\n  target v3 index =", j)
print(" ", repr(h3[j]["now"]))

print("\n== 09 open[0] (v2 unknown 에서) ==")
for j, u in enumerate(V2["specs"][9].get("unknown") or []):
    if u"fight_check.rs" in u and u"원본" in u:
        print("  unknown[%d]" % j)
        print(" ", repr(u))

print("\n== 05 logic 끝부분 ==")
lg5 = V3["specs"][5]["logic"]
print(repr(lg5[-260:]))
