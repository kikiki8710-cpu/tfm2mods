# -*- coding: utf-8 -*-
import pickle,struct,io,sys,difflib
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE=0x140000000; IMGEND=BASE+0x5000000
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
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
    def code(s,r,n): o=s.r2o(r); return s.data[o:o+n]
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57,idx58=i57["idx"],i58["idx"]; bs57,bs58=i57["by_skel"],i58["by_skel"]
gmap={}
for s,x in bs57.items():
    y=bs58.get(s)
    if y and len(x)==1 and len(y)==1: gmap[x[0]]=y[0]
cg57=pickle.load(open(r"C:\tfm2mods\_cg_057.pkl","rb")); cg58=pickle.load(open(r"C:\tfm2mods\_cg_058.pkl","rb"))
ce57,ce58=cg57["callee"],cg58["callee"]
def tk(img,r,n,loose):
    o=[]
    for ins in md.disasm(img.code(r,n),BASE+r):
        s=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: s.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem
                s.append("m[rip]" if m.base==X86_REG_RIP else ("m[%d+%d*%d]"%(m.base,m.index,m.scale) if loose else "m[%d+%d*%d+%#x]"%(m.base,m.index,m.scale,m.disp)))
            elif op.type==X86_OP_IMM:
                v=op.imm&0xffffffffffffffff
                s.append("tgt" if (ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP)) else ("iaddr" if BASE<=v<IMGEND else ("i" if loose else "i%x"%v)))
        o.append(",".join(s))
    return o
OLD=[(2,0xd476b0),(3,0xde48f0),(4,0xeab850),(5,0xde2470),(6,0xcbd450),(7,0xcbcfb0),(8,0xd52470),(9,0xea3270),
     (10,0xcb4b40),(11,0xcb9fc0),(12,0xd41230),(13,0xd53d60),(14,0xdebe50),(15,0xd50b60),(16,0xde6880),
     (17,0xd42b20),(18,0xe9fd70),(19,0xeae620),(20,0xea1ab0)]
NEW=[(2,0xe83390),(3,0xccacf0),(4,0xeb43a0),(5,0xcc5ca0),(6,0xcc4260),(7,0xcbbdb0),(8,0xea9a20),(9,0xcb7540),
     (10,0xcb03b0),(11,0xeaeda0),(12,0xcc6170),(13,0xe8b5e0),(14,0xcb1ce0),(15,0xe7caf0),(16,0xe81680),
     (17,0xe928f0),(18,0xcba660)]
print("%-24s | %-24s | 판정"%("0.5.7 SubPlan(disc)","0.5.8 대응"))
for d,o in OLD:
    projected=set(gmap[x] for x in ce57.get(o,[]) if x in gmap)
    best=[]
    for d2,n in NEW:
        cs=set(ce58.get(n,[]))
        cov=len(projected&cs)/max(1,len(projected))
        L=tk(a,o,idx57[o]["size"],True); L2=tk(b,n,idx58[n]["size"],True)
        r=difflib.SequenceMatcher(None,L,L2,autojunk=False).quick_ratio()
        best.append((cov*2+r,cov,r,d2,n))
    best.sort(reverse=True)
    sc,cov,r,d2,n=best[0]
    sc2=best[1][0]
    tag="확정" if (sc-sc2)>0.25 and cov>0.5 else ("추정" if cov>0.3 or r>0.7 else "★대응없음(삭제 후보)")
    print("disc%-2d %#010x sz=%#7x | disc%-2d %#010x cov=%.2f 유사=%.2f | %s (Δ%+d)"%(
        d,o,idx57[o]["size"],d2,n,cov,r,tag,d2-d))
