# -*- coding: utf-8 -*-
"""현행 소스 상수가 0.5.8 exe 에서 무엇을 가리키는가 + 프롤로그."""
import pickle,struct,io,sys,bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
BASE=0x140000000; md=Cs(CS_ARCH_X86,CS_MODE_64)
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; nm=d[o:o+8].rstrip(b"\0").decode('latin1'); va=struct.unpack_from('<I',d,o+12)[0]
            vsz=struct.unpack_from('<I',d,o+8)[0]; rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((nm,va,max(vsz,rsz),pr))
    def sect(s,r):
        for nm,va,sz,pr in s.secs:
            if va<=r<va+sz: return nm
    def r2o(s,r):
        for nm,va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def code(s,r,n): o=s.r2o(r); return s.data[o:o+n] if o is not None else b""
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
i58=pickle.load(open(r"C:\tfm2mods\_fnidx_058.pkl","rb"))["idx"]; st58=sorted(i58)
i57=pickle.load(open(r"C:\tfm2mods\_fnidx_057.pkl","rb"))["idx"]
def own(r):
    i=bisect.bisect_right(st58,r)-1
    if i<0: return None
    x=st58[i]; return x if r<x+i58[x]["size"] else None
def pro(img,r,n=16):
    return " ".join("%-6s %s"%(i.mnemonic,i.op_str) for i in list(md.disasm(img.code(r,24),BASE+r))[:4])
CUR=[("RVA_GENERIC_BUILD",0xceb5f0),("RVA_RETREAT",0xd2f180),("RVA_CONDGATE",0xcaf0d0),("RVA_MOVEPRI",0xcaf9f0),
     ("RVA_SUBPLAN_DISPATCH",0xe35bd0),("RVA_DISC18_HANDLER",0xe81680),("RVA_DISC19_HANDLER",0xe928f0),
     ("RVA_FC59A0",0xd40f10),("RVA_AUCTION",0xe65b10),("RVA_ITEMNET_SCORER",0x11e1b10),
     ("HR_AE_FN_RVA",0x15a1330),("HR_AP_FN_RVA",0x15b50a0),("RVA_ENGAGE_GATE",0x1c9b33d),
     ("RVA_THREATGATE_FN",0x20a8680),("RVA_GB_FUNNEL",0x22dbc4e),("RVA_C8C_DMG_SHEET",0x337f778),
     ("RVA_DISC7_DMG_SHEET",0x3384c30),("RVA_TABLE_A",0x33e1808)]
print("%-22s %-11s %-8s %-11s %s"%("상수(현행 소스값)","값","섹션","함수시작?","0.5.8 프롤로그/내용"))
for nm,v in CUR:
    s=b.sect(v); o=own(v)
    fs = "시작" if o==v else ("중간(+%#x)"%(v-o) if o is not None else "함수아님")
    print("%-22s %#-11x %-8s %-11s %s"%(nm,v,s,fs,pro(b,v) if s==".text" else " ".join("%02x"%x for x in b.code(v,16))))
print("\n[RETREAT 대조] 0.5.7 0xe4a750 프롤로그:", pro(a,0xe4a750))
print("[RETREAT 대조] 0.5.8 0xd2f180 프롤로그:", pro(b,0xd2f180))
print("[GENERIC_BUILD] 0.5.7 0xceb5f0 프롤로그:", pro(a,0xceb5f0))
print("[GENERIC_BUILD] 0.5.8 후보 0xdf36e0 프롤로그:", pro(b,0xdf36e0))
