# -*- coding: utf-8 -*-
u"""담당 5함수의 IR 범위에서 실제 호출 심볼을 뽑아 leaf 이름까지 디망글한다."""
import io, os, re, sys, json
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
RNG = {15: ("m13.ll", 33374, 33739), 16: ("m10.ll", 51913, 52374),
       17: ("m05.ll", 17695, 17825), 18: ("m09.ll", 6879, 7264),
       19: ("m04.ll", 62570, 62978)}
CS = re.compile(r"@([\w.$]+)")
CALL = re.compile(r"\b(?:tail |musttail |notail )?(?:call|invoke)\b")
for k in sorted(RNG):
    f, a, b = RNG[k]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    syms, indirect = {}, []
    for n, ln in enumerate(src[a - 1:b], a):
        if not CALL.search(ln):
            continue
        ms = CS.findall(ln)
        if not ms:
            m2 = re.search(r"(?:call|invoke)[^@]*?\s(%\d+)\(", ln)
            if m2:
                indirect.append((n, m2.group(1), ln.strip()[:110]))
            continue
        for s in ms:
            if s.startswith("llvm.") or s.startswith("__"):
                continue
            syms.setdefault(s, []).append(n)
    print("=" * 100)
    print("### %02d  %s %d~%d   호출 심볼 %d종 · 간접호출 %d" % (k, f, a, b, len(syms), len(indirect)))
    for s in sorted(syms):
        print("  %-6s %s" % (str(syms[s][0]), s[:220]))
    for n, r, ln in indirect:
        print("  IND %-6s %s" % (n, ln))
