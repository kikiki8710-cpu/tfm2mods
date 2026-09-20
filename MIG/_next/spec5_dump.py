# -*- coding: utf-8 -*-
"""2단계 sweep 첫 5개(tier1 잎) 명세 덤프 — signature/logic/reads/writes/constants/ir"""
import io, json, sys
sys.stdout.reconfigure(encoding="utf-8")
V = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v060.json", encoding="utf-8"))
want = [int(x) for x in sys.argv[1:]] or [117, 120, 9, 58, 229]
byi = {}
for k, sp in enumerate(V["specs"]):
    byi[k] = sp
for i in want:
    sp = byi[i]
    print("=" * 100)
    print("i=%d name=%s sym=%s" % (i, sp.get("name"), (sp.get("sym") or "")[:80]))
    print("src", sp.get("src"), sp.get("src_line"), "layer", sp.get("layer"), "ir", sp.get("ir"))
    v = sp.get("v060", {})
    print("v060 addr", v.get("addr"), "addr_058", v.get("addr_058"), "verdict", v.get("verdict"), "conf", v.get("confidence_060"), "callee_changed", v.get("callee_changed"))
    print("exe", sp.get("exe"))
    print("--- signature:", json.dumps(sp.get("signature"), ensure_ascii=False))
    print("--- one_line:", sp.get("one_line"))
    print("--- logic:")
    lg = sp.get("logic")
    print(lg if isinstance(lg, str) else json.dumps(lg, ensure_ascii=False, indent=1))
    if v.get("logic_060"):
        print("--- logic_060:"); print(v["logic_060"])
    for key in ("reads", "writes", "constants", "calls", "aux", "notes", "caveat", "abi", "args"):
        if sp.get(key):
            print("--- %s:" % key, json.dumps(sp[key], ensure_ascii=False)[:3000])
    print("--- other keys:", [k for k in sp if k not in ("signature", "logic", "reads", "writes", "constants", "calls", "v060", "name", "sym", "src", "src_line", "layer", "ir", "one_line", "exe", "aux", "notes", "caveat", "abi", "args")])
