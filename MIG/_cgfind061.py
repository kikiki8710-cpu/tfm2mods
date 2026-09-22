import sys; sys.argv=['x']; exec(open('C:/tfm2mods/MIG/_verify061.py').read().split("m=json.load")[0])
import collections,math
skN=collections.defaultdict(list); hdN=collections.defaultdict(list)
for r,f in PN.items(): skN[f['skel']].append(r); hdN[f['head']].append(r)
def gm(o):
    f=PO[o]; c=skN.get(f['skel'],[])
    if len(c)==1: return c[0]
    c=[x for x in c if PN[x]['size']==f['size']]
    if len(c)==1: return c[0]
    h=hdN.get(f['head'],[]); h=[x for x in h if abs(int(PN[x]['size'])-int(f['size']))<0.2*int(f['size'])]
    return h[0] if len(h)==1 else None
def cos(a,b):
    ks=set(a)|set(b); return sum(a.get(k,0)*b.get(k,0) for k in ks)/math.sqrt(sum(v*v for v in a.values())*sum(v*v for v in b.values()))
def find(o):
    f=PO[o]; print(f"== old {o:#x} size {f['size']} callers {len(CO['caller'].get(o,[]))} callees {len(CO['callee'].get(o,[]))}")
    votes=collections.Counter()
    for c in CO['caller'].get(o,[]):
        n=gm(c)
        if n: 
            for x in CN['callee'].get(n,[]): votes[x]+=1
        print(f"   caller {c:#x} -> {hex(n) if n else None}")
    ncallees=[gm(x) for x in CO['callee'].get(o,[])]
    print("   callee projections:",[hex(x) if x else None for x in ncallees])
    cands=[]
    for x,v in votes.most_common(8):
        g=PN.get(x); 
        if not g: continue
        cl=set(CN['callee'].get(x,[])); ov=sum(1 for y in ncallees if y in cl)
        cands.append((v,x,g['size'],cos(f['mnem'],g['mnem']),ov,len(CN['caller'].get(x,[]))))
    for v,x,sz,cs_,ov,nc in cands: print(f"   cand {x:#x} votes {v} size {sz} cos {cs_:.3f} callee-overlap {ov}/{len(ncallees)} callers {nc} head={'S' if PN[x]['head']==f['head'] else '-'}")
for a in ARGS: find(int(a,16))
