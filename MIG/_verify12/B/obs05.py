# -*- coding: utf-8 -*-
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import kindchk as KC

SP = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
specs = SP["specs"] if isinstance(SP, dict) else SP
idx = int(sys.argv[1]) if len(sys.argv) > 1 else 5
sp = specs[idx]
ir = sp.get("ir") or {}
print("spec", idx, sp.get("name"), "ir", ir)
for j, c in enumerate(sp.get("consts") or []):
    obs = KC.observe(ir.get("file"), ir.get("frm"), ir.get("to"), c.get("value")) or {}
    strong = dict((k, v) for k, v in obs.items() if k not in KC.WEAK)
    weak = dict((k, v) for k, v in obs.items() if k in KC.WEAK)
    print(u"c%-2d val=%-8s kind=%-6s strong=%s weak=%s" % (
        j, c.get("value"), c.get("kind"),
        u",".join(u"%s@%s" % (k, v[:3]) for k, v in sorted(strong.items())),
        u",".join(u"%s@%s" % (k, v[:3]) for k, v in sorted(weak.items()))))
