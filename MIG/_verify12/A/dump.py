# -*- coding: utf-8 -*-
u"""배치A 작업용: v3 명세의 특정 spec/필드를 JSON 으로 덤프한다."""
import json, io, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
V3 = json.load(io.open(os.path.join(ROOT, "_spec", "specs20_v3.json"), encoding="utf-8"))
V2 = json.load(io.open(os.path.join(ROOT, "_spec", "specs20.json"), encoding="utf-8"))

i = int(sys.argv[1])
field = sys.argv[2] if len(sys.argv) > 2 else None
which = sys.argv[3] if len(sys.argv) > 3 else "v3"
d = V3 if which == "v3" else V2
sp = d["specs"][i]
if field is None:
    print(json.dumps({k: (len(v) if isinstance(v, list) else v) for k, v in sp.items()}, ensure_ascii=False, indent=1)[:6000])
else:
    v = sp
    for part in field.split("."):
        v = v[part]
    print(json.dumps(v, ensure_ascii=False, indent=1))
