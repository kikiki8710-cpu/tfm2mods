# -*- coding: utf-8 -*-
import io, json, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
V2 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))["specs"]
V3 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]
for i in (17, 18, 19):
    s2, s3 = V2[i], V3[i]
    print("=== %02d %s  reads=%d writes=%d  knobs=%d new_knobs=%d" %
          (i, s3["name"], len(s2.get("reads") or []), len(s2.get("writes") or []),
           len(s2.get("knobs") or []), len(s2.get("new_knobs") or [])))
print()
print("--- 18 reads[3],[4] ---")
for j in (3, 4):
    print(json.dumps(V2[18]["reads"][j], ensure_ascii=False))
print("--- 18 mem[16] -> writes[?] ---")
r = len(V2[18]["reads"])
print("mem16 -> writes[%d]" % (16 - r))
print(json.dumps(V2[18]["writes"][16 - r], ensure_ascii=False)[:900])
print("--- 18 knobs[13] ---")
print(json.dumps(V2[18]["knobs"][13] if len(V2[18]["knobs"]) > 13 else V2[18]["new_knobs"][13 - len(V2[18]["knobs"])], ensure_ascii=False)[:600])
print("--- 19 knobs[6] ---")
k = V2[19]["knobs"]
print("len knobs=%d" % len(k))
print(json.dumps((k[6] if len(k) > 6 else V2[19]["new_knobs"][6 - len(k)]), ensure_ascii=False)[:700])
print("--- 19 one_line ---")
print(repr(V2[19].get("one_line")))
print("--- 17 writes[16] 부근 (mem[23]) ---")
r17 = len(V2[17]["reads"])
print("17 reads=%d ; mem[23] -> writes[%d]" % (r17, 23 - r17))
for j in range(max(0, 23 - r17 - 2), min(len(V2[17]["writes"]), 23 - r17 + 2)):
    print("  writes[%d] %s" % (j, json.dumps(V2[17]["writes"][j], ensure_ascii=False)[:260]))
