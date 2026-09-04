# -*- coding: utf-8 -*-
import struct,io,sys,re,collections,os,pickle,bisect
from capstone import *
from capstone.x86 import *
BASE=0x140000000
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BS=chr(92)

class Img:
    def __init__(s,p):
        d=s.data=open(p,'rb').read(); e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]; ss=e+24+struct.unpack_from('<H',d,e+20)[0]; s.secs=[]
        for i in range(n):
            o=ss+i*40
            nm=d[o:o+8].rstrip(b'\0').decode('latin1')
            va=struct.unpack_from('<I',d,o+12)[0]; vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]; pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((nm,va,max(vsz,rsz),pr))
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
    def rawfuncs(s):
        o=s.r2o(s.pdata_rva); out=[]
        for k in range(s.pdata_sz//12):
            b,e,u=struct.unpack_from('<III',s.data,o+12*k)
            out.append((b,e,u))
        return out
    def prep(s):
        """merge CHAININFO entries into their primary function"""
        raw=s.rawfuncs()
        # resolve chain: unwind rva -> parent
        d=s.data
        prim={}   # begin -> (begin,end) primary
        segs=[]   # (begin,end,primary_begin)
        def resolve(b,e,u,depth=0):
            if depth>8: return (b,e)
            o=s.r2o(u)
            if o is None: return (b,e)
            vf=d[o]; flags=vf>>3; cnt=d[o+2]
            if flags & 0x4:
                off=o+4+2*((cnt+1)&~1)
                cb,ce,cu=struct.unpack_from('<III',d,off)
                return resolve(cb,ce,cu,depth+1)
            return (b,e)
        parts=collections.defaultdict(list)
        for b,e,u in raw:
            pb,pe=resolve(b,e,u)
            parts[pb].append((b,e))
        s.parts=parts
        s._fs=[]
        for pb,ps in parts.items():
            lo=min(x[0] for x in ps); hi=max(x[1] for x in ps)
            s._fs.append((lo,hi))
        s._fs=sorted(set(s._fs))
        s._starts=[x[0] for x in s._fs]
        # also flat segment index for containment (segments may be non contiguous)
        s._segs=sorted([(b,e,pb) for pb,ps in parts.items() for (b,e) in ps])
        s._segstarts=[x[0] for x in s._segs]
    def frange(s,rva):
        i=bisect.bisect_right(s._segstarts,rva)-1
        if i>=0 and s._segs[i][0]<=rva<s._segs[i][1]:
            pb=s._segs[i][2]
            ps=s.parts[pb]
            return (min(x[0] for x in ps), max(x[1] for x in ps))
        return None
    def fparts(s,rva):
        i=bisect.bisect_right(s._segstarts,rva)-1
        if i>=0 and s._segs[i][0]<=rva<s._segs[i][1]:
            return s._segs[i][2], sorted(s.parts[s._segs[i][2]])
        return None,None

PAT=re.compile(b'[a-z][a-z0-9_-]{2,20}'+bytes([92,92])+b'src'+bytes([92,92])+b'[A-Za-z0-9_.'+bytes([92,92])+b']{2,90}[.]rs')
def build_strs(img,cache):
    if os.path.exists(cache): return pickle.load(open(cache,'rb'))
    d=img.data; strs={}
    for m in PAT.finditer(d):
        off=m.start()
        for nm,va,sz,pr in img.secs:
            if pr<=off<pr+sz: strs[va+(off-pr)]=m.group(0).decode('latin1'); break
    tgt={BASE+r:t for r,t in strs.items()}; ptr2str={}
    for nm,va,sz,pr in img.secs:
        if nm not in ('.rdata','.data'): continue
        blob=d[pr:pr+sz]
        for i in range(0,len(blob)-8,8):
            v=struct.unpack_from('<Q',blob,i)[0]
            if v in tgt: ptr2str[va+i]=tgt[v]
    pickle.dump((strs,ptr2str),open(cache,'wb')); return strs,ptr2str

def fn_insns(img,rva_start):
    """disasm all parts of the function containing rva_start; returns list of (addr,ins)"""
    pb,ps=img.fparts(rva_start)
    if pb is None: return None,None
    out=[]
    for b,e in ps:
        for ins in md.disasm(img.code(b,e-b),BASE+b):
            out.append(ins)
    return pb,out

def modhits(img,strs,p2s,rva):
    pb,ins=fn_insns(img,rva)
    if pb is None: return None,None
    hits=collections.Counter()
    for i in ins:
        for op in i.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=i.address+i.size+op.mem.disp-BASE
                if t in strs: hits[strs[t]]+=1
                elif t in p2s: hits[p2s[t]]+=1
    return pb,hits

NUMRE=re.compile(r'\b0x[0-9a-f]+\b')
RIPRE=re.compile(r'rip [-+] 0x[0-9a-f]+')
def norm(ins):
    s=RIPRE.sub('rip+X',ins.op_str)
    s=NUMRE.sub('I',s)
    return ins.mnemonic+" "+s

def load(which):
    p=E57 if which=='57' else E58
    img=Img(p); img.prep()
    strs,p2s=build_strs(img,'/c/tfm2mods/MIG/knob/_s%sk.pkl'%which)
    return img,strs,p2s
