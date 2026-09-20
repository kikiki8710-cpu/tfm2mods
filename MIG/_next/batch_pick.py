# -*- coding: utf-8 -*-
"""repro060_order tier N 후보에 1단계 발화수·함수 크기·sig(tcx ret) 를 붙여 배치 선정용 표 출력"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mig060 as M
tier = sys.argv[1] if len(sys.argv) > 1 else "1"
done = set(x.lower() for x in sys.argv[2:])
V = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v060.json", encoding="utf-8"))["specs"]
hits = {r["rva"]: r["hits"] for r in json.load(io.open(r"C:\tfm2mods\MIG\_next\probe060_result.json", encoding="utf-8"))}
ex = M.Exe(M.NEW, "new")
rows = []
for line in io.open(r"C:\tfm2mods\MIG\_next\repro060_order.md", encoding="utf-8"):
    m = re.match(r"\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|\s*`([0-9a-f]+)`\s*\|\s*`([0-9a-f]+)`\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]*)\|", line)
    if not m or m.group(1) != tier: continue
    t, nc, i, old, new, name, verdict, conf = m.groups()
    i = int(i); sp = V[i]
    if new.lower() in done: continue
    rva = int(new, 16); size = ex.ends.get(rva, rva) - rva
    sig = (sp.get("sig") or {}).get("tcx") or ""
    ret = sig.split("->")[-1].strip() if "->" in sig else "?"
    rows.append((i, new, name.strip(), nc, hits.get(new, "-"), size, ret, sig))
for r in rows:
    print("i=%-3d %s %-45s callee=%s hits=%-10s size=%-5d ret=%-14s %s" % (r[0], r[1], r[2][:45], r[3], r[4], r[5], r[6][:14], r[7][:90]))
