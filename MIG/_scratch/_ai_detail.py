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
    def code(s,r,n):
        o=s.r2o(r); return s.data[o:o+n] if o is not None else b""
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb"))["idx"]
i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))["idx"]
def rows(img,r,n):
    out=[]
    for ins in md.disasm(img.code(r,n),BASE+r):
        s=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: s.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem
                s.append("m[rip]" if m.base==X86_REG_RIP else "m[%d+%d*%d+%#x]"%(m.base,m.index,m.scale,m.disp))
            elif op.type==X86_OP_IMM:
                v=op.imm&0xffffffffffffffff
                s.append("tgt" if (ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP)) else ("iaddr" if BASE<=v<IMGEND else "i%x"%v))
        out.append((",".join(s), "%-8s %s"%(ins.mnemonic,ins.op_str), ins.address-BASE))
    return out
import json
T=json.loads(open(r"C:\tfm2mods\MIG\_pairs.json",encoding="utf-8").read())
for nm,o,n,lim in T:
    o=int(o,16); n=int(n,16)
    A=rows(a,o,i57[o]["size"]); B=rows(b,n,i58[n]["size"])
    ka=[x[0] for x in A]; kb=[x[0] for x in B]
    sm=difflib.SequenceMatcher(None,ka,kb,autojunk=False)
    ops=[x for x in sm.get_opcodes() if x[0]!="equal"]
    print("\n########## %s  0.5.7 %#x(ins %d) -> 0.5.8 %#x(ins %d)  블록 %d  유사도 %.4f"%(nm,o,len(A),n,len(B),len(ops),sm.ratio()))
    for k,(t,i1,i2,j1,j2) in enumerate(ops[:lim]):
        print("  --%s @%d"%(t,i1))
        for x in A[i1:i2][:6]: print("     - %#010x %s"%(x[2],x[1]))
        for x in B[j1:j2][:6]: print("     + %#010x %s"%(x[2],x[1]))
