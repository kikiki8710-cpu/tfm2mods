# -*- coding: utf-8 -*-
import json, pickle, collections, io, sys, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
MAN = r"C:\tfm2mods\MIG\manifest\tfm2_ai_adjust.json"
P57 = r"C:\tfm2mods\_fnidx_057.pkl"
P58 = r"C:\tfm2mods\_fnidx_058.pkl"

d = json.load(open(MAN, encoding="utf-8"))
ents = d["entries"]
i57 = pickle.load(open(P57, "rb"))
i58 = pickle.load(open(P58, "rb"))
idx57, idx58 = i57["idx"], i58["idx"]
bs58 = i58["by_skel"]; bs57 = i57["by_skel"]
starts57 = sorted(idx57.keys())
print("57 funcs", len(idx57), " 58 funcs", len(idx58))

import bisect
def owner(rva, starts, idx):
    i = bisect.bisect_right(starts, rva) - 1
    if i < 0: return None
    b = starts[i]
    if rva < b + idx[b]["size"]: return b
    return None

# 소스 모듈 분류
def mods_of(e):
    return sorted({l.split(":")[0] for l in e.get("locs", [])})

rows = []
noown = []
for e in ents:
    v = e.get("value","")
    if not isinstance(v,str) or not v.startswith("0x"): continue
    if e.get("sect") not in (".text", None): continue
    try: rva = int(v,16)
    except: continue
    if rva < 0x1000 or rva > 0x5000000: continue
    o = owner(rva, starts57, idx57)
    if o is None:
        noown.append((e["name"], v, e.get("kind")))
        continue
    rows.append(dict(name=e["name"], val=rva, kind=e.get("kind"), owner=o,
                     off=rva-o, mods=mods_of(e), ver=e.get("ver",""), note=e.get("note","")[:80]))
print("entries with .text owner:", len(rows), " no-owner:", len(noown))

owners = collections.defaultdict(list)
for r in rows: owners[r["owner"]].append(r)
print("unique owner fns:", len(owners))

# 분류
def classify(o):
    f = idx57[o]
    c = bs58.get(f["skel"], [])
    cs = bs57.get(f["skel"], [])
    if len(c) == 0: return "SKEL_NOMATCH", c, cs
    if len(c) == 1: return "UNIQUE", c, cs
    return "MULTI(%d)" % len(c), c, cs

stat = collections.Counter()
out = []
for o, rs in sorted(owners.items()):
    k, c, cs = classify(o)
    stat[k.split("(")[0]] += 1
    out.append(dict(owner=o, size=idx57[o]["size"], ninsn=idx57[o]["ninsn"], cls=k,
                    ncand=len(c), nclone57=len(cs), cands=c[:6],
                    nsites=len(rs), holds=sum(1 for r in rs if r["kind"]=="BYTE_PATCH_HOLD"),
                    mods=sorted({m for r in rs for m in r["mods"]}),
                    names=sorted({r["name"] for r in rs})[:6]))
print(stat)
json.dump(out, open(r"C:\tfm2mods\MIG\_ai_owners.json","w",encoding="utf-8"), ensure_ascii=False)
json.dump(rows, open(r"C:\tfm2mods\MIG\_ai_rows.json","w",encoding="utf-8"), ensure_ascii=False)
print("no-owner sample:", noown[:10])
