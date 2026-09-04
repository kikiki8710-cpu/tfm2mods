# -*- coding: utf-8 -*-
"""strict 비교: loose skel 이 같아도 imm/disp 가 바뀌었는지 본다."""
import json, pickle, struct, hashlib, io, sys, re, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE = 0x140000000
OLD = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
NEW = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

class Img:
    def __init__(self, path):
        d = self.data = open(path,'rb').read()
        e = struct.unpack_from('<I',d,0x3c)[0]; nsec=struct.unpack_from('<H',d,e+6)[0]
        optsz=struct.unpack_from('<H',d,e+20)[0]; optStart=e+24; ss=optStart+optsz
        self.secs=[]
        for i in range(nsec):
            o=ss+i*40; va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; praw=struct.unpack_from('<I',d,o+20)[0]
            self.secs.append((va,max(vsz,rsz),praw))
    def r2o(self,rva):
        for va,sz,praw in self.secs:
            if va<=rva<va+sz: return praw+(rva-va)
        return None
    def code(self,rva,n):
        o=self.r2o(rva); return self.data[o:o+n] if o is not None else b""

a=Img(OLD); b=Img(NEW)

def strict(code, addr, cap=20000):
    """명령 시퀀스. 분기/호출 타깃과 rip-rel 만 정규화, imm/mem-disp 는 보존."""
    out=[]
    for ins in md.disasm(code, addr):
        seg=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: seg.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem
                if m.base==X86_REG_RIP: seg.append("m[rip]")
                else: seg.append("m[%d+%d*%d+%#x]"%(m.base,m.index,m.scale,m.disp))
            elif op.type==X86_OP_IMM:
                if ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP):
                    seg.append("tgt")
                else: seg.append("i%x"%(op.imm & 0xffffffffffffffff))
        out.append(",".join(seg))
        if len(out)>cap: break
    return out

def sh(lst): return hashlib.md5("|".join(lst).encode()).hexdigest()

i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb"))
i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57,idx58 = i57["idx"], i58["idx"]; bs58=i58["by_skel"]
owners=json.load(open(r"C:\tfm2mods\MIG\_ai_owners.json",encoding="utf-8"))

res=[]
for i,o in enumerate(owners):
    ro=o["owner"]; sz=o["size"]
    so=strict(a.code(ro,sz), BASE+ro)
    ho=sh(so)
    cands=bs58.get(idx57[ro]["skel"],[])
    rec=dict(o); rec["strict_old"]=ho; rec["nins_old"]=len(so)
    if not cands:
        rec["verdict"]="LOGIC_CHANGED"; rec["new"]=None
    else:
        hit=None
        for c in cands[:200]:
            sn=strict(b.code(c,idx58[c]["size"]), BASE+c)
            if sh(sn)==ho: hit=c; break
        if hit is not None:
            rec["verdict"]="IDENTICAL"; rec["new"]=hit
        else:
            # 구조는 같은데 imm/disp 가 다름 -> 첫 후보와 상세 diff
            c=cands[0]
            sn=strict(b.code(c,idx58[c]["size"]), BASE+c)
            diffs=[(k,so[k],sn[k]) for k in range(min(len(so),len(sn))) if so[k]!=sn[k]]
            rec["verdict"]="CONST_OR_OFFSET_CHANGED"; rec["new"]=c
            rec["ndiff"]=len(diffs); rec["diff_sample"]=diffs[:8]
    res.append(rec)
    if i%50==0: print("...",i,flush=True)

cnt=collections.Counter(r["verdict"] for r in res)
print(cnt)
json.dump(res, open(r"C:\tfm2mods\MIG\_ai_verdict.json","w",encoding="utf-8"), ensure_ascii=False)
