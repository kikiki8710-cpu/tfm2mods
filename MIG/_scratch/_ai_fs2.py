# -*- coding: utf-8 -*-
import struct,io,sys
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((va,max(vsz,rsz),pr))
    def code(s,r,n):
        for va,sz,pr in s.secs:
            if va<=r<va+sz: return s.data[pr+(r-va):pr+(r-va)+n]
        return b""
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
T=[("FS2_A",0xd81ea2,0xd199f2,bytes([0x4b,0x8b,0x84,0xec,0xb8,0x01,0x00,0x00,0x48,0xd1,0xe8,0x4b,0x03,0x44,0xec,0x28])),
   ("FS2_B",0xd82111,0xd19c61,bytes([0x4b,0x8b,0x84,0xfa,0xb8,0x01,0x00,0x00,0x48,0xd1,0xe8,0x4b,0x03,0x44,0xfa,0x28])),
   ("FS2_N1",0xd82357,0xd19ea7,bytes([0x4c,0x89,0x50,0x10])),
   ("FS2_N2",0xd8238f,0xd19edf,bytes([0x4c,0x89,0x50,0x10]))]
for nm,new,old,orig in T:
    gn=b.code(new,len(orig)); go=a.code(old,len(orig))
    print("%-7s 0.5.8@%#x  실제=%s  기대=%s  %s   | 0.5.7@%#x 실제=%s %s"%(
        nm,new,gn.hex(),orig.hex(),"일치" if gn==orig else "★불일치",
        old,go.hex(),"일치" if go==orig else "★불일치"))
