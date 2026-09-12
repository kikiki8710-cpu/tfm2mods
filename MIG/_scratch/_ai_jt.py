# -*- coding: utf-8 -*-
import pickle,struct,io,sys
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
BASE=0x140000000
md=Cs(CS_ARCH_X86,CS_MODE_64)
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
def head(img,r,n=40):
    for i,ins in enumerate(md.disasm(img.code(r,300),BASE+r)):
        print("   %#010x  %-8s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
        if i>=n: break
def jt(img,base,cnt):
    o=img.r2o(base); vals=[]
    for k in range(cnt):
        v=struct.unpack_from("<i",img.data,o+4*k)[0]; vals.append((base+v)&0xffffffff)
    return vals
print("### MOVEPRI(Plan 디스패처) 0.5.7 @0xdb2760"); head(a,0xdb2760,26)
print("### MOVEPRI 0.5.8 @0xcaf9f0"); head(b,0xcaf9f0,26)
print("### SUBPLAN_DISPATCH 0.5.7 @0xcbf340"); head(a,0xcbf340,24)
print("### SUBPLAN_DISPATCH 0.5.8 @0xe35bd0"); head(b,0xe35bd0,24)
