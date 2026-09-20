# -*- coding: utf-8 -*-
"""함수들의 직접 call 타깃(패닉/alloc 류 제외 표시) 을 크기·spec 이름과 함께 나열 — 재현체 배치 선정용"""
import io, json, sys, os
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mig060 as M
V = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v060.json", encoding="utf-8"))["specs"]
name060 = {}
for sp in V:
    a = (sp.get("v060") or {}).get("addr")
    if a: name060[int(a, 16)] = sp["name"]
ex = M.Exe(M.NEW, "new")
PANIC = {0x3821813: "panic_bounds", 0x3821770: "unwrap_none", 0x3821b30: "panic_const", 0x3821af0: "panic_div0"}
def survey(rva, depth=0, seen=None):
    seen = seen if seen is not None else set()
    start = ex.owner(rva) or rva; end = ex.ends.get(start, start + 16)
    calls = []
    for ins in ex.md.disasm(ex.img[start:end], start):
        if ins.mnemonic == "call":
            if ins.op_str.startswith("0x"):
                t = int(ins.op_str, 16); calls.append((ins.address, t, ex.ends.get(t, t) - t))
            else:
                calls.append((ins.address, ins.op_str, 0))
    pad = "  " * depth
    print("%s%x %s (%dB) calls=%d" % (pad, start, name060.get(start, ""), end - start, len(calls)))
    for at, t, sz in calls:
        if isinstance(t, int):
            tag = PANIC.get(t) or name060.get(t, "")
            print("%s  @%x -> %x %s (%dB)" % (pad, at, t, tag, sz))
            if depth < 1 and t not in PANIC and t not in seen and sz < 3000:
                seen.add(t); survey(t, depth + 1, seen)
        else:
            print("%s  @%x -> %s (간접)" % (pad, at, t))
for a in sys.argv[1:]:
    survey(int(a, 16)); print()
