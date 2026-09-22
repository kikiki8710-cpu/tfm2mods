# -*- coding: utf-8 -*-
"""생성자 define 의 sret 속성(initializes) + 본문 store 오프셋 전수(sret 기준)."""
import io, os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tlsreach as T
ga = json.load(io.open(r"C:\tfm2mods\MIG\_next\reach\defidx.json", encoding="utf-8"))
targets = [
 "12move_actionsNtB2_18SmallActionRunAway14new_with_skill",
 "12move_actionsNtB2_18SmallActionRunAway3new",
 "6aroundNtB2_17SmallActionAround3new",
 "4castNtB2_17SmallActionAttack3new",
 "4castNtB5_16SmallActionSkill3new",
 "4castNtB5_17SmallActionSkill23new",
 "6aroundNtB5_25SmallActionAroundPosition3new",
]
for t in targets:
    syms = [s for s in ga if s.endswith(t)]
    for s in syms:
        f, a, b = ga[s]
        L = T.lines_of(os.path.join(T.GA, f))[a-1:b]
        head = L[0]
        m = re.search(r"sret\(\[(\d+) x i8\]\)[^%]*%0", head)
        init = re.search(r"initializes\(([^)]*)\)", head)
        print("==", f, a, "-", b, s[-70:])
        print("   sret", m.group(1) if m else "?", "initializes", init.group(1) if init else "(없음)")
        # sret 기준 store 오프셋: %0 직접 store 또는 gep %0, N 후 store
        geps = {}
        stores = []
        for i, l in enumerate(L):
            mm = re.match(r"\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (\d+)", l)
            if mm and (mm.group(2) == "%0" or mm.group(2) in geps):
                geps[mm.group(1)] = int(mm.group(3)) + geps.get(mm.group(2), 0)
            ms = re.match(r"\s*store (\S+) ([^,]+), ptr (%\d+)", l)
            if ms and (ms.group(3) == "%0" or ms.group(3) in geps):
                off = geps.get(ms.group(3), 0)
                stores.append((off, ms.group(1), ms.group(2)[:40], a + i))
            mc = re.search(r"llvm\.mem(cpy|set)\.p0\S*\(ptr (%\d+), (?:ptr )?([^,]+), i64 (\d+)", l)
            if mc and (mc.group(2) == "%0" or mc.group(2) in geps):
                stores.append((geps.get(mc.group(2), 0), "mem" + mc.group(1), mc.group(3)[:30], a + i, int(mc.group(4))))
        for st in sorted(stores):
            print("   ", st)
