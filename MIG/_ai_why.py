# -*- coding: utf-8 -*-
import struct,re,io,sys,difflib
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md=Cs(CS_ARCH_X86,CS_MODE_64)
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
def toks(d,secs,rva,size):
    o=ro(secs,rva); return [i.mnemonic+"|"+RE_NUM.sub("I",i.op_str) for i in md.disasm(d[o:o+size],0x140000000+rva)]
d57,s57=load(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
d58,s58=load(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
for (ra,sa),(rb,sb),tag in (((0xe8b800,0x4c24),(0xe65b10,0x4c24),"RVA_AUCTION"),
                            ((0xd1b0e0,0xd8f),(0xdf0e90,0xd8f),"detour:1737"),
                            ((0xe14e50,0x838),(0xd2da10,0x838),"detour:818")):
    A=toks(d57,s57,ra,sa); B=toks(d58,s58,rb,sb)
    sm=difflib.SequenceMatcher(None,A,B,autojunk=False)
    ops=[o for o in sm.get_opcodes() if o[0]!="equal"]
    print("==",tag,len(A),len(B),"블록",len(ops))
    for o in ops[:8]:
        print("  ",o[0],A[o[1]:o[2]][:3],"->",B[o[3]:o[4]][:3])
