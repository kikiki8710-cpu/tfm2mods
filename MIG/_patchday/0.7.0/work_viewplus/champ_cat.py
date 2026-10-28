# -*- coding: utf-8 -*-
"""번들에서 챔피언 역할군(category/class) 표기를 찾아 암살자 목록을 본다 (training_view_plus 일괄 적용 점검)."""
import sys, re
sys.stdout.reconfigure(encoding="utf-8")
B = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle.game_data"
data = open(B, "rb").read()
for kw in [b"Assassin", b"assassin"]:
    hits = [m.start() for m in re.finditer(kw, data)]
    print(kw, len(hits))
    for h in hits[:40]:
        s = data[max(0, h - 160):h + 40]
        print("  ", h, s.decode("utf-8", "replace").replace("\n", " ")[-200:])
