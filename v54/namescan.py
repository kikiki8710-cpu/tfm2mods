# -*- coding: utf-8 -*-
"""lea reg,[rip+X] 직후 mov r8d/edx,imm 패턴으로 (문자열, 길이) 쌍 복원 → Debug 이름표 전수"""
import io,sys,re,collections
sys.path.insert(0,r'C:\tfm2mods\v54')
from s import E, sec, whose, fsrc
from pe2 import BASE
import capstone

def scan(ver, lo=None, hi=None):
    e=E(ver)
    _,tva,tvsz,tra,trsz = sec(e,'.text')
    body=e.raw[tra:tra+trsz]
    md=capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail=True
    out=[]
    funcs=e.funcs(); starts=[f[0] for f in funcs]
    import bisect
    # find every lea rdx,[rip+d] then scan forward up to 24 bytes for mov r8d,imm
    for m in re.finditer(rb'\x48\x8d\x15', body):
        o=m.start()
        d=int.from_bytes(body[o+3:o+7],'little',signed=True)
        tgt=tva+o+7+d
        if lo is not None and not (lo<=tgt<hi): continue
        # look for  41 b8 imm32  (mov r8d,imm) within next 40 bytes, allowing a jmp
        seg=body[o+7:o+7+48]
        mm=re.search(rb'\x41\xb8(....)', seg, re.S)
        ln=None
        if mm: ln=int.from_bytes(mm.group(1),'little')
        if ln is None or ln>80: 
            out.append((tva+o,tgt,None,None)); continue
        st=e.rd(tgt,ln)
        out.append((tva+o,tgt,ln,st))
    return out

if __name__=='__main__':
    ver=sys.argv[1]; lo=int(sys.argv[2],16); hi=int(sys.argv[3],16)
    e=E(ver)
    seen=collections.OrderedDict()
    for site,tgt,ln,st in scan(ver,lo,hi):
        f=e.func_of(site)
        key=(f[0] if f else 0, tgt, ln)
        if key in seen: continue
        seen[key]=(site,)
        print('fn %06x  site %06x  str %06x len %s  %r'%(f[0] if f else 0, site, tgt, ln, st))
