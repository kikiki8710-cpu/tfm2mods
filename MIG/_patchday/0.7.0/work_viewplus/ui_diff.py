# -*- coding: utf-8 -*-
"""0.6.5 / 0.7.0 번들에서 밴픽 레이아웃을 뽑아 노드 id 차이를 본다 (view_plus 오버라이드 누락 점검)."""
import sys, os, re, importlib.util
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("ebu", r"F:\claude\tfm2mods\tools\bundle\extract_bundle_ui.py")
ebu = importlib.util.module_from_spec(spec); spec.loader.exec_module(ebu)

VERS = {"0.6.5": r"F:\claude\tfm2\tfm2_0.6.5\bundle.game_data",
        "0.7.0": r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"}
if "--extract" in sys.argv:
    for v, b in VERS.items():
        ebu.extract(b, os.path.join(HERE, "bundle_ui_" + v))

FILES = ["layout", "blue_pick_slot", "red_pick_slot"]
OVR = r"F:\claude\tfm2mods\banpick_view_plus\ui\layout\banpick"
ID = re.compile(r"#([A-Za-z0-9_]+)")
def ids(p):
    return set(ID.findall(open(p, encoding="utf-8", errors="replace").read())) if os.path.exists(p) else None
for f in FILES:
    a = ids(os.path.join(HERE, "bundle_ui_0.6.5", "ui", "layout", "banpick", f + ".ui"))
    b = ids(os.path.join(HERE, "bundle_ui_0.7.0", "ui", "layout", "banpick", f + ".ui"))
    o = ids(os.path.join(OVR, f + ".ui"))
    print("==", f, "065:", a and len(a), "070:", b and len(b), "ovr:", o and len(o))
    if a is None or b is None:
        continue
    print("  0.7.0 신규:", sorted(b - a))
    print("  0.7.0 삭제:", sorted(a - b))
    print("  0.7.0 에 있고 오버라이드에 없음:", sorted(b - o))
