# -*- coding: utf-8 -*-
import pickle, struct, hashlib, re, io, sys, collections
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RE_NUM=re.compile(r'0x[0-9a-f]+')
def load(p):
    d=open(p,'rb').read(); pe=struct.unpack_from("<I",d,0x3c)[0]
    n=struct.unpack_from("<H",d,pe+6)[0]; opt=pe+24
    st=opt+struct.unpack_from("<H",d,pe+20)[0]; secs=[]
    for i in range(n):
        o=st+i*40; vsz,va,rsz,rr=struct.unpack_from("<IIII",d,o+8); secs.append((va,max(vsz,rsz),rr))
    return d,secs
def ro(secs,r):
    for va,sz,rr in secs:
        if va<=r<va+sz: return rr+(r-va)
def skel(d,secs,rva,size):
    o=ro(secs,rva); code=d[o:o+size]; t=[]
    for ins in md.disasm(code,0x140000000+rva):
        t.append(ins.mnemonic+"|"+RE_NUM.sub("I",ins.op_str))
    return hashlib.md5("\n".join(t).encode()).hexdigest(), len(t)
P57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
P58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb")); i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))
print("pkl57 path:",i57["path"]); print("pkl58 path:",i58["path"])
d57,s57=load(P57); d58,s58=load(P58)
import os
print("backup 0.5.7 size",os.path.getsize(P57)," live",os.path.getsize(P58))
bad=0; n=0
for rva,f in list(i57["idx"].items())[:4000]:
    h,ni=skel(d57,s57,rva,f["size"]); n+=1
    if h!=f["skel"]: bad+=1
print("57 pkl vs backup exe: 표본 %d 중 불일치 %d"%(n,bad))
bad=0;n=0
for rva,f in list(i58["idx"].items())[:4000]:
    h,ni=skel(d58,s58,rva,f["size"]); n+=1
    if h!=f["skel"]: bad+=1
print("58 pkl vs live exe: 표본 %d 중 불일치 %d"%(n,bad))
# 문제 함수 개별
for rva,exe,secs,idx,tag in ((0xe8b800,d57,s57,i57,"57@0xe8b800"),(0xe65b10,d58,s58,i58,"58@0xe65b10"),
                             (0xd1b0e0,d57,s57,i57,"57@0xd1b0e0"),(0xdf0e90,d58,s58,i58,"58@0xdf0e90")):
    f=idx["idx"].get(rva)
    print(tag,"pdata size",hex(f["size"]) if f else None,"ninsn",f["ninsn"] if f else None,
          "skel",f["skel"][:12] if f else None, "recomputed", skel(exe,secs,rva,f["size"])[0][:12] if f else None)
