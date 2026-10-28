# -*- coding: utf-8 -*-
"""0.7.0 번들의 같은 경로 중복판(_c/_d)과 0.6.5 원본·view_plus 오버라이드의 노드 차이."""
import sys, os, re
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
OLD = os.path.join(HERE, "bundle_ui_0.6.5", "ui", "layout")
NEW = os.path.join(HERE, "bundle_ui_0.7.0", "ui", "layout")
OVR = r"F:\claude\tfm2mods\banpick_view_plus\ui\layout\banpick"
ID = re.compile(r"#([A-Za-z_][A-Za-z0-9_]*)")
def ids(p):
    return set(ID.findall(open(p, encoding="utf-8", errors="replace").read())) if os.path.exists(p) else set()
for base in ["layout", "blue_pick_slot", "red_pick_slot"]:
    old = ids(os.path.join(OLD, "banpick", base + ".ui"))
    ovr = ids(os.path.join(OVR, base + ".ui"))
    for suf in ["_c", "_d"]:
        p = os.path.join(NEW, "banpick", base + suf + ".ui")
        if not os.path.exists(p):
            continue
        new = ids(p)
        print("==", base + suf)
        print("  0.6.5 대비 신규 id:", sorted(new - old))
        print("  0.6.5 대비 삭제 id:", sorted(old - new))
        print("  오버라이드에 없는 id:", sorted(new - ovr))
for f in ["captain_badge.ui", "promise_badge.ui"]:
    print("==", f)
    print(open(os.path.join(NEW, f), encoding="utf-8", errors="replace").read()[:1500])
