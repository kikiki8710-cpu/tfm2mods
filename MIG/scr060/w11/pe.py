import struct,sys
from capstone import *
P={'058':r'C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.8\TeamfightManager2.exe','060':r'C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.0\TeamfightManager2.exe'}
base=0x140000000
class PE:
    def __init__(s,ver):
        s.d=open(P[ver],'rb').read(); d=s.d
        pe=struct.unpack_from('<I',d,0x3c)[0]
        nsec=struct.unpack_from('<H',d,pe+6)[0]
        opt=struct.unpack_from('<H',d,pe+20)[0]
        s.secs=[]; off=pe+24+opt
        for i in range(nsec):
            name=d[off:off+8].rstrip(b'\0').decode()
            vs,va,rs,ro=struct.unpack_from('<IIII',d,off+8)
            s.secs.append((name,va,vs,ro,rs)); off+=40
    def rva2off(s,r):
        for n,va,vs,ro,rs in s.secs:
            if va<=r<va+max(vs,rs): return ro+(r-va)
    def q(s,a): return struct.unpack_from('<Q',s.d,s.rva2off(a-base))[0]
    def bytes(s,a,n): o=s.rva2off(a-base); return s.d[o:o+n]
    def str_at(s,a,n=80):
        o=s.rva2off(a-base)
        if o is None: return ''
        b=s.d[o:o+n]
        if b and all(32<=c<127 for c in b[:6]): return b.split(b'\0')[0][:n].decode('latin1')
        return ''
    def loc(s,a):
        o=s.rva2off(a-base)
        p_,l,line,col=struct.unpack_from('<QQII',s.d,o)
        so=s.rva2off(p_-base)
        return s.d[so:so+l].decode('latin1'),line,col
    def disasm(s,start,end):
        md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
        code=s.bytes(start,end-start)
        return list(md.disasm(code,start))
