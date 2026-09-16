# -*- coding: utf-8 -*-
# 2단 노드(ext=2)에서 정적 xref 로 계속 올라가면 뿌리가 있나 — .text call/jmp 전수 + .pdata 로 최대 8단
import io, re, json, sys
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import retedges, fnmap_ext
W = r"C:\Users\jungs\Desktop\claude\tfm2\.claude\worktrees\swap-order-button-style-fe9e04\mods_report\tfm2_ai_adjust\AI함수지도.html"
h = io.open(W, encoding="utf-8").read()
d = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
BY = {n["a"]: n for n in d}
starts, ends = retedges.pdata_starts()
L2 = [n["a"] for n in d if n.get("ext") == 2]
print("2단 노드", len(L2))
frontier = {int(a, 16) for a in L2}
seen = set(frontier); parent = {}
levels = []
for lv in range(8):
    if not frontier: break
    calls = fnmap_ext.text_calls(frontier)
    nxt = set(); dead_end = []
    for tgt in frontier:
        owners = set()
        for site in calls.get(tgt, []):
            o = retedges.owner(starts, ends, site)
            if o is not None and o != tgt: owners.add(o)
        if not owners: dead_end.append(tgt)
        for o in owners:
            parent.setdefault(o, tgt)
            if o not in seen: seen.add(o); nxt.add(o)
    levels.append((lv, len(frontier), len(dead_end), len(nxt)))
    print("단 %d: 노드 %d · 정적 호출자 없음(뿌리 후보/vtable 경유) %d · 새로 올라간 %d" % (lv + 2, len(frontier), len(dead_end), len(nxt)))
    for tgt in dead_end:
        a = "%x" % tgt
        print("    뿌리 후보 %s %s" % (a, (BY.get(a) or {}).get("n") or ""))
    frontier = nxt
print("도달한 게임 함수 총", len(seen), "· 틱 루프 0x1879080 포함:", 0x1879080 in seen)
