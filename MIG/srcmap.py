# -*- coding: utf-8 -*-
import struct,io,sys,re,collections
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
from capstone import *
from capstone.x86 import *
BASE=0x140000000
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        s.pe=e
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40
            nm=d[o:o+8].rstrip(b'\0').decode('latin1')
            va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((nm,va,max(vsz,rsz),pr))
        # data directory 3 = exception
        opt=e+24; magic=struct.unpack_from('<H',d,opt)[0]
        dd=opt+112 if magic==0x20b else opt+96
        s.pdata_rva,s.pdata_sz=struct.unpack_from('<II',d,dd+3*8)
    def r2o(s,r):
        for nm,va,sz,pr in s.secs:
            if va<=r<va+sz: return pr+(r-va)
    def sec(s,r):
        for nm,va,sz,pr in s.secs:
            if va<=r<va+sz: return nm
    def code(s,r,n):
        o=s.r2o(r); return s.data[o:o+n] if o is not None else b""
    def funcs(s):
        o=s.r2o(s.pdata_rva); out=[]
        for k in range(s.pdata_sz//12):
            b,e,u=struct.unpack_from('<III',s.data,o+12*k)
            out.append((b,e))
        return out
    def frange(s,rva):
        # binary search
        fs=s._fs
        import bisect
        i=bisect.bisect_right(s._starts,rva)-1
        if i>=0 and fs[i][0]<=rva<fs[i][1]: return fs[i]
        return None
    def prep(s):
        s._fs=sorted(set(s.funcs())); s._starts=[x[0] for x in s._fs]

def build(path):
    img=Img(path); img.prep()
    d=img.data
    # find source path strings
    strs={}   # rva -> text
    PAT = re.compile(b'game-ai' + bytes([92,92]) + b'src' + bytes([92,92]) + b'[A-Za-z0-9_' + bytes([92,92,92]) + b'.]+' + bytes([92]) + b'.rs')
    for m in PAT.finditer(d):
        off=m.start()
        # off -> rva
        for nm,va,sz,pr in img.secs:
            if pr<=off<pr+sz:
                strs[va+(off-pr)]=m.group(0).decode('latin1'); break
    # find qwords in rdata/data pointing at those strings
    ptr2str={}
    tgt={BASE+r:t for r,t in strs.items()}
    for nm,va,sz,pr in img.secs:
        if nm not in ('.rdata','.data'): continue
        blob=d[pr:pr+sz]
        for i in range(0,len(blob)-8,8):
            v=struct.unpack_from('<Q',blob,i)[0]
            if v in tgt: ptr2str[va+i]=tgt[v]
    return img,strs,ptr2str

def analyze(img,strs,ptr2str,fn_rva,limit=None):
    fr=img.frange(fn_rva)
    if not fr: return None,None
    b,e=fr
    hits=collections.Counter()
    code=img.code(b,e-b)
    for ins in md.disasm(code,BASE+b):
        for op in ins.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=ins.address+ins.size+op.mem.disp-BASE
                if t in strs: hits[strs[t]]+=1
                elif t in ptr2str: hits[ptr2str[t]]+=1
    return (b,e),hits
