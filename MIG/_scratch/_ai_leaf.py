# -*- coding: utf-8 -*-
"""HR_AE_FN / HR_AP_FN (62B 예측 leaf) 의 0.5.8 대응 탐색 + 본문 대조."""
import pickle, struct, io, sys, difflib, collections
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE=0x140000000; IMGEND=BASE+0x5000000
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read()
        e=struct.unpack_from('<I',d,0x3c)[0]; n=struct.unpack_from('<H',d,e+6)[0]
        ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((va,max(vsz,rsz),pr))
    def r2o(s,r):
        for va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def code(s,r,n):
        o=s.r2o(r); return s.data[o:o+n] if o is not None else b""
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
def txt(img,r,n):
    return [(i.address-BASE,i.mnemonic+" "+i.op_str) for i in md.disasm(img.code(r,n),BASE+r)]
def tk(img,r,n,loose=False):
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
i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))["idx"]
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb"))["idx"]
for name,rva in (("HR_AE_FN",0x15a1330),("HR_AP_FN",0x15b50a0)):
    sz=i57[rva]["size"]
    print("=== %s 0.5.7 %#x sz=%#x"%(name,rva,sz))
    for off,t in txt(a,rva,sz): print("   %#010x  %s"%(off,t))
    T=tk(a,rva,sz); L=tk(a,rva,sz,loose=True)
    best=[]
    for r2,f in i58.items():
        if abs(f["ninsn"]-len(T))>4 or abs(f["size"]-sz)>24: continue
        L2=tk(b,r2,f["size"],loose=True)
        ratio=difflib.SequenceMatcher(None,L,L2,autojunk=False).ratio()
        if ratio>0.85: best.append((ratio,r2,f["size"]))
    best.sort(reverse=True)
    print("  0.5.8 후보(loose>0.85):",len(best))
    for ratio,r2,s2 in best[:5]:
        T2=tk(b,r2,s2)
        ex = (T==T2)
        print("   %#010x sz=%#x loose=%.3f  strict동일=%s"%(r2,s2,ratio,ex))
        if not ex:
            for o in difflib.SequenceMatcher(None,T,T2,autojunk=False).get_opcodes():
                if o[0]!="equal": print("       ",o[0],T[o[1]:o[2]],"->",T2[o[3]:o[4]])
