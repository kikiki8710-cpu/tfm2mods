# -*- coding: utf-8 -*-
u"""임무① 검증 — **base 철자 통일이 G14 의 보류를 실제로 줄이는가**(규약의 값어치 측정).

정본은 건드리지 않는다. 메모리에서만 철자를 바꿔 `memdir.check_spec` / 보류 집계를 다시 돌린다.
"""
import io, json, os, sys, copy

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
if MIG not in sys.path:
    sys.path.insert(0, MIG)
import memdir as MD

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

# (specidx, mem idx) -> 통일할 철자
FIX = {
    (11, 14): u"BigPlan(지역변수 plan)",
    (12, 22): u"PendingTraceEvent(힙 원소)",
    (15, 12): u"SinglePlanBattle",
}


def tally(specs, tag):
    tot = bad = hold = skip = 0
    for i, sp in enumerate(specs):
        ir = sp.get("ir") or {}
        if not ir.get("file"):
            continue
        reads, writes, hr, hw, touch = MD.scan(ir["file"], ir["frm"], ir["to"])
        bad += len(MD.check_spec(sp))
        for j, m in enumerate(sp.get("mem") or []):
            d = (m.get("dir") or "").strip()
            o = MD.off_of(m)
            if d not in ("r", "w") or o is None:
                skip += 1
                continue
            tot += 1
            anc = MD.base_anchors_for(sp, j, m.get("base"), reads, writes, hr, hw)
            if not anc:
                hold += 1
            elif not any(MD.covers(reads if d == "r" else writes, r, o + sh) for (r, sh) in anc):
                hold += 1
    print(u"%-12s 검사 %d · 불일치 %d · 보류 %d · 대상외 %d" % (tag, tot, bad, hold, skip))
    return tot, bad, hold, skip


tally(D["specs"], u"원본")
S2 = copy.deepcopy(D["specs"])
for (i, j), v in FIX.items():
    S2[i]["mem"][j]["base"] = v
tally(S2, u"철자통일")

print(u"\n---- 대상외 2행 색출")
for i, sp in enumerate(D["specs"]):
    for j, m in enumerate(sp.get("mem") or []):
        d = (m.get("dir") or "").strip()
        if d not in ("r", "w") or MD.off_of(m) is None:
            print(u"  specs[%d] mem[%d] dir=%r off=%r base=%r name=%r"
                  % (i, j, m.get("dir"), m.get("offset"), m.get("base"), m.get("name")))
