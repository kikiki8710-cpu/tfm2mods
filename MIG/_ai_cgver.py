# -*- coding: utf-8 -*-
"""콜그래프 사영으로 old->new 짝짓기 신뢰도 채점."""
import json, pickle, io, sys, collections
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
bs57,bs58=i57["by_skel"],i58["by_skel"]
gmap={}
for s,a in bs57.items():
    b=bs58.get(s)
    if b and len(a)==1 and len(b)==1: gmap[a[0]]=b[0]
rmap={v:k for k,v in gmap.items()}
print("UNIQUE 기준맵:",len(gmap))
cg57=pickle.load(open(r"C:\tfm2mods\_cg_057.pkl","rb")); cg58=pickle.load(open(r"C:\tfm2mods\_cg_058.pkl","rb"))
ce57,ce58=cg57["callee"],cg58["callee"]; cr57,cr58=cg57["caller"],cg58["caller"]
def score(o,n):
    a=[gmap[x] for x in ce57.get(o,[]) if x in gmap]; b=set(ce58.get(n,[]))
    ca=[gmap[x] for x in cr57.get(o,[]) if x in gmap]; cb=set(cr58.get(n,[]))
    return (len(set(a)&b), len(set(a)), len(set(ca)&cb), len(set(ca)))
PAIRS=json.load(open(r"C:\tfm2mods\MIG\_ai_changed.json",encoding="utf-8"))
print("%-11s %-11s %-24s %-7s %-12s %-12s"%("OLD57","NEW58","named","유사도","callee일치","caller일치"))
for r in PAIRS:
    if "new" not in r: 
        print("%#-11x %-11s %-24s  ---  대응 미확정"%(r["owner"],"?",",".join(r["named"]) or "-")); continue
    ci,ct,ri,rt=score(r["owner"],r["new"])
    print("%#-11x %#-11x %-24s %.3f   %2d/%-2d        %2d/%-2d"%(
        r["owner"],r["new"],",".join(r["named"]) or "-",r["ratio"],ci,ct,ri,rt))
