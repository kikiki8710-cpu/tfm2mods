# -*- coding: utf-8 -*-
"""LOGIC_CHANGED owner 의 0.5.8 대응 함수를 '이미 해결된 형제 사이트'로 역산 + 상세 diff."""
import json, pickle, bisect, io, sys, collections, difflib, struct, hashlib
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE=0x140000000; IMGEND=BASE+0x5000000
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
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
def isaddr(v): return BASE<=v<IMGEND
def toks(code,addr,loose=False):
    out=[]
    for ins in md.disasm(code,addr):
        seg=[ins.mnemonic]
        for op in ins.operands:
            if op.type==X86_OP_REG: seg.append("r%d"%op.reg)
            elif op.type==X86_OP_MEM:
                m=op.mem
                if m.base==X86_REG_RIP: seg.append("m[rip]")
                elif m.base==0 and m.index==0 and isaddr(m.disp&0xffffffffffffffff): seg.append("m[abs]")
                elif loose: seg.append("m[%d+%d*%d]"%(m.base,m.index,m.scale))
                else: seg.append("m[%d+%d*%d+%#x]"%(m.base,m.index,m.scale,m.disp))
            elif op.type==X86_OP_IMM:
                v=op.imm&0xffffffffffffffff
                if ins.id in (X86_INS_CALL,X86_INS_JMP) or ins.group(CS_GRP_JUMP): seg.append("tgt")
                elif isaddr(v): seg.append("iaddr")
                elif loose: seg.append("i")
                else: seg.append("i%x"%v)
        out.append(",".join(seg))
    return out

i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
idx57,idx58=i57["idx"],i58["idx"]; st57=sorted(idx57); st58=sorted(idx58)
def own(r,st,idx):
    i=bisect.bisect_right(st,r)-1
    if i<0: return None
    x=st[i]; return x if r<x+idx[x]["size"] else None
rep=json.load(open(r"C:\tfm2mods\MIG\repin_058.json",encoding="utf-8"))["tfm2_ai_adjust"]
man=json.load(open(r"C:\tfm2mods\MIG\manifest\tfm2_ai_adjust.json",encoding="utf-8"))["entries"]
# 이름 -> 0.5.8 값 (마이그 후 매니페스트)
new_named=collections.defaultdict(set)
for x in man:
    v=x.get("value","")
    if isinstance(v,str) and v.startswith("0x") and x.get("ver")=="0.5.8":
        new_named[x["name"]].add(int(v,16))
V=json.load(open(r"C:\tfm2mods\MIG\_ai_verdict2.json",encoding="utf-8"))
out=[]
for r in V:
    if r["verdict"]!="LOGIC_CHANGED": continue
    votes=collections.Counter()
    for s in r["sites"]:
        k="0x%x"%s["old"]
        nv=rep.get(k,{}).get("new")
        if nv:
            o=own(int(nv,16),st58,idx58)
            if o is not None: votes[o]+=1
    # 이름 기반 보강
    for nm in r["named"]:
        for nv in new_named.get(nm,()):
            o=own(nv,st58,idx58)
            if o is not None: votes[o]+=3
    r2=dict(owner=r["owner"],size=r["size"],ninsn=r["ninsn"],named=r["named"],mods=r["mods"],
            nsites=r["nsites"],fail=r["fail"],votes=[( "%#x"%k,v) for k,v in votes.most_common(4)])
    if votes:
        nw=votes.most_common(1)[0][0]
        to=toks(a.code(r["owner"],r["size"]),BASE+r["owner"])
        tn=toks(b.code(nw,idx58[nw]["size"]),BASE+nw)
        lo=toks(a.code(r["owner"],r["size"]),BASE+r["owner"],loose=True)
        ln=toks(b.code(nw,idx58[nw]["size"]),BASE+nw,loose=True)
        sm=difflib.SequenceMatcher(None,lo,ln,autojunk=False)
        ops=[o for o in sm.get_opcodes() if o[0]!="equal"]
        r2.update(new=nw,new_size=idx58[nw]["size"],new_ninsn=len(tn),
                  ratio=round(sm.ratio(),4),nblocks=len(ops),
                  ins_add=sum(o[4]-o[3] for o in ops),ins_del=sum(o[2]-o[1] for o in ops),
                  ops=[(o[0],o[1],o[2],o[3],o[4],lo[o[1]:o[1]+6],ln[o[3]:o[3]+6]) for o in ops[:12]])
    out.append(r2)
json.dump(out,open(r"C:\tfm2mods\MIG\_ai_changed.json","w",encoding="utf-8"),ensure_ascii=False)
for r in sorted(out,key=lambda x:-x.get("nsites",0)):
    nm=",".join(r["named"]) or "-"
    if "new" in r:
        print("%#010x -> %#010x  %-22s ins %d->%d  sz %#x->%#x  유사도=%.3f  변경블록=%d (+%d/-%d)"%(
            r["owner"],r["new"],nm,r["ninsn"],r["new_ninsn"],r["size"],r["new_size"],r["ratio"],r["nblocks"],r["ins_add"],r["ins_del"]))
    else:
        print("%#010x -> ???        %-22s ins=%d  (0.5.8 대응 미확정)"%(r["owner"],nm,r["ninsn"]))
