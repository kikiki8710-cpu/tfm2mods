# -*- coding: utf-8 -*-
import struct,io,sys,bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
BASE=0x140000000; md=Cs(CS_ARCH_X86,CS_MODE_64)
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((va,max(vsz,rsz),pr))
    def r2o(s,r):
        for va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def i32(s,r): o=s.r2o(r); return struct.unpack_from("<i",s.data,o)[0]
    def code(s,r,n): o=s.r2o(r); return s.data[o:o+n]
def analyze(img,fn,size,jtb,narm,tag,names):
    arms=[]
    for k in range(narm):
        arms.append(((jtb+img.i32(jtb+4*k))&0xffffffff,k))
    starts=sorted(a[0] for a in arms)
    idxof={a:k for a,k in arms}
    calls={}
    for ins in md.disasm(img.code(fn,size),BASE+fn):
        if ins.mnemonic=="call" and ins.op_str.startswith("0x"):
            t=int(ins.op_str,16)-BASE
            site=ins.address-BASE
            i=bisect.bisect_right(starts,site)-1
            arm=idxof[starts[i]] if i>=0 else None
            calls.setdefault(t,[]).append((arm,site))
    print("== %s  fn=%#x arm수=%d"%(tag,fn,narm))
    for t,nm in names.items():
        if t in calls:
            for arm,site in calls[t]:
                print("   %-22s call@%#x  arm=%s  ⟹ disc=%s"%(nm,site,arm,(arm+2) if arm is not None else "?"))
        else: print("   %-22s (직접 call 없음)"%nm)
    # arm 별 첫 call 요약
    print("   arm별 대표 호출:")
    per={}
    for t,lst in calls.items():
        for arm,site in lst: per.setdefault(arm,[]).append((site,t))
    for arm in sorted(x for x in per if x is not None):
        s=sorted(per[arm])[:3]
        print("     arm%2d(disc%2d): %s"%(arm,arm+2," ".join("%#x"%t for _,t in s)))
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
analyze(a,0xcbf340,0x4fe,0x336BE88,19,"0.5.7 SubPlan 디스패처",{0xe9fd70:"DISC18_HANDLER",0xeae620:"DISC19_HANDLER"})
analyze(b,0xe35bd0,0x4ce,0x33E8910,17,"0.5.8 SubPlan 디스패처",{0xe81680:"DISC18_HANDLER",0xe928f0:"DISC19_HANDLER"})
