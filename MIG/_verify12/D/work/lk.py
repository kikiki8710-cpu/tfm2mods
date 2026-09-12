# -*- coding: utf-8 -*-
import sys, io
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import spec3lib as L
for n in ("get_entity_by_id", "next", "tick", "push", "is_empty", "drop", "get_game_mode",
          "as_index", "position", "distance_sq", "is_cleared"):
    rs = L.fnlookup(n)
    print("=== %s : %d" % (n, len(rs)))
    for r in rs:
        print("    %-90s %s" % (r["path"][:90], r.get("crate")))
