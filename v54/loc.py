# -*- coding: utf-8 -*-
"""per-function exact (source,line) panic-location refs, in address order"""
import io, re, sys, bisect, struct
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
SRC_PAT = re.compile(rb'[A-Za-z0-9_\-]+[\\/](?:[A-Za-z0-9_\-]+[\\/])*[A-Za-z0-9_\-]+\.rs')
_C={}
def locmap(ver):
    if ver in _C: return _C[ver]
    e = load(ver)
    rd = [s for s in e.sections if s[0]=='.rdata'][0]
    _,rd_va,rd_vsz,rd_ra,rd_rsz = rd
    blob = e.raw[rd_ra:rd_ra+rd_rsz]
    str_at={}
    for m in SRC_PAT.finditer(blob):
        str_at[rd_va+m.start()] = m.group().decode('latin1')
    loc={}
    for i in range(0,len(blob)-24,8):
        ptr=int.from_bytes(blob[i:i+8],'little')
        if ptr < e.imagebase or ptr > e.imagebase+0x5000000: continue
        rva=ptr-e.imagebase
        s=str_at.get(rva)
        if not s: continue
        ln=int.from_bytes(blob[i+8:i+16],'little')
        if ln!=len(s): continue
        line=int.from_bytes(blob[i+16:i+20],'little')
        col=int.from_bytes(blob[i+20:i+24],'little')
        if 0<line<100000: loc[rd_va+i]=(s,line,col)
    _C[ver]=(e,loc)
    return _C[ver]
def refs(ver, lo, hi):
    e,loc = locmap(ver)
    t=[s for s in e.sections if s[0]=='.text'][0]
    _,t_va,_,t_ra,t_rsz=t
    body=e.raw[t_ra:t_ra+t_rsz]
    out=[]
    off_lo, off_hi = lo-t_va, hi-t_va
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body[off_lo:off_hi]):
        o=off_lo+m.start()
        disp=int.from_bytes(body[o+3:o+7],'little',signed=True)
        tgt=t_va+o+7+disp
        if tgt in loc:
            out.append((t_va+o,)+loc[tgt])
    return out
if __name__=='__main__':
    ver=sys.argv[1]; lo=int(sys.argv[2],16)
    e,_=locmap(ver)
    f=None
    for s,en in e.funcs():
        if s<=lo<en: f=(s,en); break
    if len(sys.argv)>3: f=(lo,int(sys.argv[3],16))
    print('fn %06x-%06x'%f)
    for a,s,l,c in refs(ver,f[0],f[1]):
        print('  %06x  %s:%d:%d'%(a,s,l,c))
