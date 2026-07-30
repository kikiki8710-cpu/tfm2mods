# -*- coding: utf-8 -*-
# check_prologue.py — detour 훅 함수의 새 프롤로그가 orig_len 경계에 정확히 맞는지 검증.
#   install_detour는 orig_len 바이트 복사후 fn+orig_len 으로 점프 → 경계 어긋나면 크래시.
import struct, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
from capstone import *
md=Cs(CS_ARCH_X86,CS_MODE_64)
NEW=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
d=open(NEW,"rb").read(); pe=struct.unpack_from("<I",d,0x3c)[0]
nsec=struct.unpack_from("<H",d,pe+6)[0]; opt=pe+24
ib=struct.unpack_from("<Q",d,opt+24)[0]; sectab=opt+struct.unpack_from("<H",d,pe+20)[0]
secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=d[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rraw=struct.unpack_from("<IIII",d,o+8); secs.append((nm,va,vsz,rraw,rsz))
def roff(rva):
    for nm,va,vsz,rraw,rsz in secs:
        if va<=rva<va+max(vsz,rsz): return rraw+(rva-va)
# (name, newRVA, orig_len)
HOOKS=[("RETREAT*",0x1d35820,12),("FC59A0*",0x1d42db0,12),("CONDGATE*",0x1b08e10,15),("MOVEPRI*",0x1b09590,14),
       ("DD7700",0x18c6490,12),("FCD980",0x18b1550,13),("FCDAF0",0x18b16c0,13),
       ("CHACHA",0x2245cf0,12),("TRANS",0x1af2920,12),
       ("CAND_FILTER",0x1f9f7c0,12),("F80320",0x1fe4990,12),
       ("DF0C10_FN",0x205c510,12),("DISPATCH",0x1b098e0,17)]
# * = 실제 활성 detour(INSTALL_DIAG_HOOKS=false). 나머지는 true 재빌드 대비 검증.
for nm,rva,olen in HOOKS:
    off=roff(rva); code=d[off:off+olen+16]
    boundaries=set(); asm=[]
    pos=0
    for ins in md.disasm(code, ib+rva):
        boundaries.add(ins.address-(ib+rva))
        asm.append("%s %s"%(ins.mnemonic,ins.op_str))
        if ins.address-(ib+rva) >= olen+8: break
    ok = olen in boundaries
    # rip-relative 여부(트램폴린 복사시 깨짐) 점검
    riprel = any('rip' in a for a in asm[:6])
    print(f"{nm:12s} new=0x{rva:x} olen={olen} 경계일치={'OK' if ok else '★어긋남'} riprel={'YES⚠' if riprel else 'no'}")
    print("   ", " | ".join(asm[:7]))
