# -*- coding: utf-8 -*-
"""strict 재판정 v2: 이미지 절대주소 imm/disp 정규화(재링크 노이즈 제거) + 최적 후보 선택."""
import json, pickle, struct, hashlib, io, sys, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE=0x140000000; IMGEND=BASE+0x5000000
OLD=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
NEW=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read()
        e=struct.unpack_from('<I',d,0x3c)[0]; n=struct.unpack_from('<H',d,e+6)[0]
        os_=e+24; ss=os_+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((va,max(vsz,rsz),pr))
    def r2o(s,r):
        for va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def code(s,r,n):
        o=s.r2o(r); return s.data[o:o+n] if o is not None else b""
a=Img(OLD); b=Img(NEW)
def isaddr(v): return BASE<=v<IMGEND
def strict(code,addr,cap=30000):
    out=[]
    for ins in md.disasm(code,addr):
        seg=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: seg.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem
                if m.base==X86_REG_RIP: seg.append("m[rip]")
                elif m.base==0 and m.index==0 and isaddr(m.disp&0xffffffffffffffff): seg.append("m[abs]")
                else: seg.append("m[%d+%d*%d+%#x]"%(m.base,m.index,m.scale,m.disp))
            elif op.type==X86_OP_IMM:
                v=op.imm&0xffffffffffffffff
                if ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP): seg.append("tgt")
                elif isaddr(v): seg.append("iaddr")
                else: seg.append("i%x"%v)
        out.append(",".join(seg))
        if len(out)>cap: break
    return out
def sh(l): return hashlib.md5("|".join(l).encode()).hexdigest()
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57,idx58=i57["idx"],i58["idx"]; bs58=i58["by_skel"]
owners=json.load(open(r"C:\tfm2mods\MIG\_ai_owners2.json",encoding="utf-8"))
res=[]
for i,o in enumerate(owners):
    ro=o["owner"]; so=strict(a.code(ro,o["size"]),BASE+ro); ho=sh(so)
    cands=bs58.get(idx57[ro]["skel"],[])
    rec=dict(o); rec["nins_old"]=len(so)
    if not cands:
        rec["verdict"]="LOGIC_CHANGED"; rec["new"]=None; res.append(rec); continue
    best=None
    for c in cands[:300]:
        sn=strict(b.code(c,idx58[c]["size"]),BASE+c)
        if sh(sn)==ho: best=(0,c,[]); break
        dl=[(k,so[k],sn[k]) for k in range(min(len(so),len(sn))) if so[k]!=sn[k]]
        if best is None or len(dl)<best[0]: best=(len(dl),c,dl)
    nd,c,dl=best
    rec["new"]=c; rec["ndiff"]=nd
    rec["verdict"]="IDENTICAL" if nd==0 else "CONST_OR_OFFSET_CHANGED"
    rec["diff_sample"]=dl[:10]
    res.append(rec)
    if i%50==0: print("...",i,flush=True)
print(collections.Counter(r["verdict"] for r in res))
json.dump(res,open(r"C:\tfm2mods\MIG\_ai_verdict2.json","w",encoding="utf-8"),ensure_ascii=False)
for r in res:
    if r["verdict"]=="CONST_OR_OFFSET_CHANGED":
        print("%#010x -> %#010x  ndiff=%d ins=%d  clones=%d  mods=%s" % (r["owner"],r["new"],r["ndiff"],r["nins_old"],r["ncand"],",".join(m.split("/")[-1] for m in r["mods"])))
        for k,x,y in r["diff_sample"][:4]: print("      [%d] %s   ->   %s"%(k,x,y))
