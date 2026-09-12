# -*- coding: utf-8 -*-
import json, pickle, collections, io, sys, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
rep = json.load(open(r"C:\tfm2mods\MIG\repin_058.json", encoding="utf-8"))["tfm2_ai_adjust"]
man = json.load(open(r"C:\tfm2mods\MIG\manifest\tfm2_ai_adjust.json", encoding="utf-8"))["entries"]
i57 = pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58 = pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57, idx58 = i57["idx"], i58["idx"]; bs58 = i58["by_skel"]; bs57 = i57["by_skel"]
st57 = sorted(idx57); st58 = sorted(idx58)
def own(rva, st, idx):
    i = bisect.bisect_right(st, rva)-1
    if i < 0: return None
    b = st[i]
    return b if rva < b+idx[b]["size"] else None

# manifest 로 이름/kind 보강: 0.5.7 값 그대로인 엔트리는 value 가 곧 OLD
byname = collections.defaultdict(list)
old_named = {}   # oldrva -> set(names)
for x in man:
    v = x.get("value","")
    if isinstance(v,str) and v.startswith("0x") and x.get("ver","").startswith("0.5.7"):
        old_named.setdefault(int(v,16), set()).add(x["name"])

sites = []
for k, v in rep.items():
    o = int(k, 16)
    sites.append(dict(old=o, name=v["name"], kind=v["kind"], note=v.get("note",""),
                      new=int(v["new"],16) if v.get("new") else None,
                      locs=v.get("locs",[]), sect=v.get("sect_old","")))
# repin 에 없는 0.5.7 잔여
for o, ns in old_named.items():
    if ("0x%x"%o) not in rep:
        sites.append(dict(old=o, name=sorted(ns)[0], kind="MANIFEST_ONLY", note="", new=None, locs=[], sect=".text"))
print("sites(OLD 0.5.7):", len(sites))

# 이름 보강
for s in sites:
    if s["old"] in old_named: s["names"] = sorted(old_named[s["old"]])
    else: s["names"] = [s["name"]]

owners = collections.defaultdict(list)
noown = []
for s in sites:
    o = own(s["old"], st57, idx57)
    if o is None: noown.append(s); continue
    s["owner"] = o; s["off"] = s["old"]-o
    owners[o].append(s)
print("unique 0.5.7 owner fns:", len(owners), " no-owner:", len(noown))
stat = collections.Counter()
out=[]
for o, ss in sorted(owners.items()):
    sk = idx57[o]["skel"]; c = bs58.get(sk, [])
    cls = "SKEL_NOMATCH" if not c else ("UNIQUE" if len(c)==1 else "MULTI")
    stat[cls]+=1
    mods = sorted({l.split(":")[0] for s in ss for l in s["locs"]})
    nm = sorted({n for s in ss for n in s["names"] if not n.startswith("INLINE@")})
    out.append(dict(owner=o, size=idx57[o]["size"], ninsn=idx57[o]["ninsn"], cls=cls,
                    ncand=len(c), cands=c[:8], nclone57=len(bs57.get(sk,[])),
                    nsites=len(ss), fail=sum(1 for s in ss if s["kind"]=="FAIL"),
                    mods=mods, named=nm, sites=[dict(old=s["old"],off=s["off"],kind=s["kind"],name=s["name"],names=s["names"],note=s["note"][:160],locs=s["locs"][:4]) for s in ss]))
print(stat)
json.dump(out, open(r"C:\tfm2mods\MIG\_ai_owners2.json","w",encoding="utf-8"), ensure_ascii=False)
json.dump([dict(old=s["old"],name=s["name"],kind=s["kind"],names=s["names"],locs=s["locs"][:4]) for s in noown],
          open(r"C:\tfm2mods\MIG\_ai_noowner2.json","w",encoding="utf-8"), ensure_ascii=False)
print("noown:", [("%#x"%s["old"], s["name"]) for s in noown][:20])
