# -*- coding: utf-8 -*-
"""미확정 7건: 콜그래프(콜러 사영 + callee 집합) 로 0.5.8 후보 탐색."""
import pickle, io, sys, collections, json, struct
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57,idx58=i57["idx"],i58["idx"]; bs57,bs58=i57["by_skel"],i58["by_skel"]
gmap={}
for s,a in bs57.items():
    b=bs58.get(s)
    if b and len(a)==1 and len(b)==1: gmap[a[0]]=b[0]
cg57=pickle.load(open(r"C:\tfm2mods\_cg_057.pkl","rb")); cg58=pickle.load(open(r"C:\tfm2mods\_cg_058.pkl","rb"))
ce57,ce58,cr57,cr58=cg57["callee"],cg58["callee"],cg57["caller"],cg58["caller"]
# 역인덱스: 새 exe 에서 callee 집합 -> 함수
TARGETS=[0xda9b30,0xda9ee0,0xdb7260,0xde2470,0xe49a70,0x15a1330,0x15b50a0,0xd50180,0xd53d60,0xde6880]
for t in TARGETS:
    f=idx57.get(t)
    cal=[gmap[x] for x in ce57.get(t,[]) if x in gmap]
    calls_all=ce57.get(t,[])
    # 콜러 사영: 0.5.7 콜러들 -> 0.5.8 콜러 -> 그들이 부르는 함수들 = 후보풀
    pool=collections.Counter()
    for c in cr57.get(t,[]):
        cn=gmap.get(c)
        if cn: 
            for x in ce58.get(cn,[]): pool[x]+=1
    # callee 사영 채점
    sc=collections.Counter()
    if cal:
        cand=set()
        for x in cal:
            for c in cr58.get(x,[]): cand.add(c)
        for c in cand:
            s=len(set(cal)&set(ce58.get(c,[])))
            if s: sc[c]=s
    best=sc.most_common(5)
    print("== %#x sz=%#x ins=%d  callee=%d(사영%d) caller=%d" % (t,f["size"],f["ninsn"],len(calls_all),len(cal),len(cr57.get(t,[]))))
    if best:
        print("   callee사영 상위:", ["%#x:%d/%d sz=%#x"%(c,s,len(set(cal)),idx58[c]["size"]) for c,s in best])
    if pool:
        print("   콜러사영 풀 상위:", ["%#x:%d sz=%#x"%(c,n,idx58[c]["size"]) for c,n in pool.most_common(6)])
    if t in bs57 or True:
        sk=idx57[t]["skel"]; hd=idx57[t]["head"]
        print("   skel후보58=%d  head후보58=%d  clone57=%d"%(len(bs58.get(sk,[])),len(i58["by_head"].get(hd,[])),len(bs57.get(sk,[]))))
