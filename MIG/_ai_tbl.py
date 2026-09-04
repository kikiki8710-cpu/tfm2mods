# -*- coding: utf-8 -*-
import struct,io,sys
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
BASE=0x140000000
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40; nm=d[o:o+8].rstrip(b"\0").decode('latin1'); va=struct.unpack_from('<I',d,o+12)[0]
            vsz=struct.unpack_from('<I',d,o+8)[0]; rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((nm,va,max(vsz,rsz),pr))
    def r2o(s,r):
        for nm,va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def u64(s,r):
        o=s.r2o(r); return struct.unpack_from("<Q",s.data,o)[0] if o is not None else None
    def i32(s,r):
        o=s.r2o(r); return struct.unpack_from("<i",s.data,o)[0] if o is not None else None
    def cstr(s,r,n=48):
        o=s.r2o(r)
        if o is None: return None
        return s.data[o:o+n]
a=Img(r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe")
b=Img(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe")
def dumptbl(img,base,cnt,tag):
    print("== %s  표 @%#x  %d엔트리"%(tag,base,cnt))
    for k in range(cnt+2):
        v=img.u64(base+8*k)
        if v is None: break
        rva=v-BASE if BASE<=v<BASE+0x5000000 else None
        # (ptr,len) 쌍 가능성: 다음 워드
        nxt=img.u64(base+8*k+8)
        txt=""
        if rva is not None:
            raw=img.cstr(rva,40)
            if raw: txt=raw[:32].decode('latin1').replace("\x00",".")
        print("  [%2d] %016x %s %s"%(k,v,("rva=%#x"%rva) if rva else "", repr(txt) if txt else ""))
dumptbl(a,0x336BB30,16,"0.5.7 SubPlan 이름표")
dumptbl(b,0x33E86C0,14,"0.5.8 SubPlan 이름표")

def jt32(img,base,fn_lo,fn_hi,tag,maxn=24):
    print("== %s JT @%#x (fn %#x..%#x)"%(tag,base,fn_lo,fn_hi))
    n=0
    for k in range(maxn):
        v=img.i32(base+4*k)
        if v is None: break
        t=(base+v)&0xffffffff
        ok = fn_lo<=t<fn_hi
        print("  [%2d] %+#010x -> %#010x %s"%(k,v,t,"OK" if ok else "×(표 끝)"))
        if not ok: break
        n+=1
    print("  유효 arm 수 =",n)
jt32(a,0x3378428,0xdb2760,0xdb2760+0x426,"0.5.7 Plan(MOVEPRI)")
jt32(b,0x33d766c,0xcaf9f0,0xcaf9f0+0x419,"0.5.8 Plan(MOVEPRI)")
jt32(a,0x336BE88,0xcbf340,0xcbf340+0x4fe,"0.5.7 SubPlan(DISPATCH)")
jt32(b,0x33E8910,0xe35bd0,0xe35bd0+0x4ce,"0.5.8 SubPlan(DISPATCH)")
