# -*- coding: utf-8 -*-
# 콜그래프 앵커 전파 (0.5.4 -> 0.5.5) : skel 유일쌍 시드앵커 + 콜러/콜리 투표
import struct, collections, bisect
import _mig055 as M
def text_sec(img):
    for nm,va,vsz,rraw,rsz in img.secs:
        if nm==".text": return va,rraw,rsz
def build_cg(img):
    va,raw,rsz=text_sec(img); text=img.raw[raw:raw+rsz]
    starts=img.starts; fn=img.fn
    callee=collections.defaultdict(set); caller=collections.defaultdict(set)
    i=0; n=len(text)
    while True:
        i=text.find(b"\xe8",i)
        if i<0 or i+5>n: break
        rel=struct.unpack_from("<i",text,i+1)[0]; site=va+i; tgt=site+5+rel
        if tgt in fn:
            j=bisect.bisect_right(starts,site)-1
            if j>=0:
                own=starts[j]
                if site<own+fn[own]["size"]:
                    callee[own].add(tgt); caller[tgt].add(own)
        i+=1
    return callee,caller
print("building callgraphs...")
CO,RO=build_cg(M.O); CN,RN=build_cg(M.N)
# 시드앵커: skel 양쪽 유일
anchor={}
for s,olds in M.O.by_skel.items():
    if len(olds)==1:
        news=M.N.by_skel.get(s,[])
        if len(news)==1: anchor[olds[0]]=news[0]
print(f"seed anchors: {len(anchor)}")
def vote(f_old):
    # 콜리 투표
    ce=[anchor[c] for c in CO.get(f_old,()) if c in anchor]
    cr=[anchor[c] for c in RO.get(f_old,()) if c in anchor]
    # 0.5.5에서 이 콜리들을 부르는 함수 = 후보 (callee 기준)
    box=collections.Counter()
    for cnew in ce:
        for f in RN.get(cnew,()): box[f]+= 1
    for cnew in cr:  # 콜러 기준: cnew가 부르는 함수
        for f in CN.get(cnew,()): box[f]+= 1
    return box, len(ce), len(cr)
if __name__=="__main__":
    cases=[("it:TIP_SHOW",0x236dc00,10010),("launcher(it/serpen)",0x13b53d0,4432),
      ("ct:RUN_RVA",0x231de30,3750),("serpen:SERPEN",0x1328950,3959),
      ("serpen:MOBATICK",0x13ee0a0,41740),("serpen:RUNNER_CTOR",0x13b7050,3640),
      ("serpen:DMGA",0x10670a0,811),("serpen:DMGB",0x14eaef0,3589),("it:SPAWN",0x13bca10,1891)]
    for name,rva,osz in cases:
        box,ne,nr=vote(rva)
        top=box.most_common(5)
        # size 근접도 표시
        show=[(hex(f),v,M.N.fn[f]["size"]) for f,v in top]
        print(f"\n{name} {rva:#x} osz={osz} (콜리앵커 {ne}, 콜러앵커 {nr})")
        for f,v,sz in show:
            print(f"    {f}  votes={v}  size={sz}{'  <-size일치' if sz==osz else ''}")
