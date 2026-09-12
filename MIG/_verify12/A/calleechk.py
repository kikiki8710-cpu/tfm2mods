# -*- coding: utf-8 -*-
u"""배치A: callees 앵커(`ev3`) 재검증 — v0 망글링 길이접두로 엄밀 대조.

mkspec3._rank_callees 는 `all(s in sym for s in segs)` = **맨 부분문자열** 대조라
`Effect::range` 가 `...6Effect12range_adjust` 에 걸려 거짓 `ev3` 도장을 받는다.
여기서는 각 성분을 `<len><name>` 으로 요구해 다시 판정한다.
"""
import json, io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
IRDIR = r"C:\tfm2mods\_gaibc"
V3 = json.load(io.open(os.path.join(ROOT, "_spec", "specs20_v3.json"), encoding="utf-8"))
CALLSYM = re.compile(r"@(_R[\w.$]+)")


def ir_callsyms(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    out = set()
    for ln in src[a - 1:b]:
        if "call" not in ln and "invoke" not in ln:
            continue
        out.update(CALLSYM.findall(ln))
    return sorted(out)


def segs_of(path):
    p = re.sub(r"<[^>]*>", lambda m: m.group(0), path or u"")
    # 트레잇 impl `<A as B>::m` → A, B, m 조각으로 쪼갠다
    p = p.replace("<", " ").replace(">", " ").replace(" as ", "::")
    out = []
    for s in p.split("::"):
        s = s.strip()
        if not s:
            continue
        s = re.sub(r"/#\d+", "", s)
        s = s.strip("'& ")
        if s:
            out.append(s)
    return out


def loose(segs, sym):
    return all(s in sym for s in segs)


def strict(segs, sym):
    return all((str(len(s)) + s) in sym for s in segs)


for i in range(int(sys.argv[1]), int(sys.argv[2]) + 1):
    sp = V3["specs"][i]
    irsy = ir_callsyms(sp)
    print(u"=== [%02d] %s  IR호출심볼 %d개" % (i, sp["name"], len(irsy)))
    for s in irsy:
        print(u"    IR: %s" % s)
    for j, c in enumerate(sp.get("callees") or []):
        sg = segs_of(c.get("path"))
        L = [s for s in irsy if loose(sg, s)]
        S = [s for s in irsy if strict(sg, s)]
        mark = u"OK " if (bool(L) == bool(S)) else u"★差"
        print(u"  %s callees[%d] ev=%s %-60s loose=%d strict=%d %s"
              % (mark, j, c.get("ev"), c.get("path"), len(L), len(S),
                 (S[0] if S else (u"(loose만:" + L[0] + u")" if L else u""))))
    print()
