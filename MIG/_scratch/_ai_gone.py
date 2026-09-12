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
    def u64(s,r): return struct.unpack_from("<Q",s.data,s.r2o(r))[0]
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb"))["idx"]
i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))["idx"]
def tk(img,r,n):
    o=[]
    for ins in md.disasm(img.code(r,n),BASE+r):
        s=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: s.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem; s.append("m[rip]" if m.base==X86_REG_RIP else "m[%d+%d*%d]"%(m.base,m.index,m.scale))
            elif op.type==X86_OP_IMM:
                v=op.imm&0xffffffffffffffff
                s.append("tgt" if (ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP)) else ("iaddr" if BASE<=v<IMGEND else "i"))
        o.append(",".join(s))
    return o
for nm,r in (("0.5.7 sub_plan3",0xde48f0),("0.5.7 sub_plan5",0xde2470)):
    sz=i57[r]["size"]; L=tk(a,r,sz); best=[]
    for r2,f in i58.items():
        if abs(f["ninsn"]-len(L))>len(L)*0.35: continue
        L2=tk(b,r2,f["size"])
        q=difflib.SequenceMatcher(None,L,L2,autojunk=False).quick_ratio()
        if q>0.80:
            best.append((difflib.SequenceMatcher(None,L,L2,autojunk=False).ratio(),r2,f["size"]))
    best.sort(reverse=True)
    print("%s %#x sz=%#x ins=%d -> 0.5.8 전역 최고 유사도:"%(nm,r,sz,len(L)),
          [("%#x"%x[1],round(x[0],3)) for x in best[:4]] or "없음(<0.80)")
print("\nRVA_TABLE_A 0.5.7 @0x33817c8:",[hex(a.u64(0x33817c8+8*k)) for k in range(6)])
print("RVA_TABLE_A 0.5.8 @0x33e1808:",[hex(b.u64(0x33e1808+8*k)) for k in range(6)])
