# -*- coding: utf-8 -*-
# 콜러 집합 지문 대조 (컨테이너 크기+함수내 오프셋) + vtable 데이터참조
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
o=O(); n=Nw(); o.sites(); n.sites()
def fp(img, tgt):
    out=[]
    for s in img.by_tgt.get(tgt, []):
        w=img.owner(s)
        out.append((img.fn[w]["size"] if w else -1, s-(w or 0)))
    return sorted(out)
for nm, a, b in json.loads(sys.argv[1]):
    a=int(a,16); b=int(b,16)
    fa=fp(o,a); fb=fp(n,b)
    ok = sum(1 for x in fa if x in fb)
    print("="*80)
    print(f"[{nm}] 053 {a:#x}({len(fa)} 콜러) → 054 {b:#x}({len(fb)} 콜러)  완전일치 {ok}/{len(fa)}")
    print(f"   053: {[(s,hex(o_)) for s,o_ in fa[:10]]}")
    print(f"   054: {[(s,hex(o_)) for s,o_ in fb[:10]]}")
    da=o.data_refs(a); db=n.data_refs(b)
    print(f"   데이터참조(vtable) 053={len(da)} 054={len(db)}  {[(x,hex(y)) for x,y in da[:4]]} → {[(x,hex(y)) for x,y in db[:4]]}")
