# -*- coding: utf-8 -*-
import struct,io,sys
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
BASE=0x140000000; md=Cs(CS_ARCH_X86,CS_MODE_64)
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
for nm,o57,o58,ln in (("CONDGATE",0xdb1e20,0xcaf0d0,15),("MOVEPRI",0xdb2760,0xcaf9f0,14),("FC59A0",0xe61600,0xd40f10,12)):
    for tag,img,r in (("0.5.7",a,o57),("0.5.8",b,o58)):
        bnds=[];rip=False;off=0
        for ins in md.disasm(img.code(r,40),BASE+r):
            bnds.append(off); off+=ins.size
            if off<=ln and "rip" in ins.op_str: rip=True
            if off>ln+8: break
        bnds.append(off)
        raw=" ".join("%02x"%x for x in img.code(r,ln))
        print("%-9s %s %#x  경계=%s  orig_len=%d %s  rip-rel(내부)=%s  bytes=%s"%(
            nm,tag,r,bnds[:9],ln,"OK" if ln in bnds else "★경계아님",rip,raw))
    print()
