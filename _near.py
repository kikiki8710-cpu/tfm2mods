# -*- coding: utf-8 -*-
# _near.py <exe> <fnstart> <size> <center> [win] : center 주변 window 안의 문자열 LEA/호출 나열
import sys, io, struct
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
import capstone
EXE=sys.argv[1]; raw=open(EXE,'rb').read()
pe=struct.unpack_from("<I",raw,0x3c)[0]; nsec=struct.unpack_from("<H",raw,pe+6)[0]; opt=pe+24
IB=struct.unpack_from("<Q",raw,opt+24)[0]; sectab=opt+struct.unpack_from("<H",raw,pe+20)[0]
secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rr=struct.unpack_from("<IIII",raw,o+8); secs.append((nm,va,max(vsz,rsz),rr))
def roff(rva):
    for nm,va,sz,rr in secs:
        if va<=rva<va+sz: return rr+(rva-va)
def rs(va,ml=140):
    o=roff(va-IB)
    if o is None: return None
    b=raw[o:o+ml]; e=b.find(b'\x00'); e=ml if e<0 else e
    try: s=b[:e].decode('utf-8')
    except Exception: return None
    return s if s and all(9<=ord(c)<127 for c in s) else None
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
st=int(sys.argv[2],16); n=int(sys.argv[3],16); c=int(sys.argv[4],16); w=int(sys.argv[5],16) if len(sys.argv)>5 else 0x200
o=roff(st)
for ins in md.disasm(raw[o:o+n], IB+st):
    a=ins.address-IB
    if not (c-w<=a<=c+w): continue
    ex=""
    for op in ins.operands:
        if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
            t=ins.address+ins.size+op.mem.disp
            d=rs(t)
            if d: ex="  ; %r"%d[:70]
            else: ex="  ; ->%#x"%(t-IB)
    if ins.mnemonic=='call' and ins.operands[0].type==capstone.x86.X86_OP_IMM:
        ex="  ; call %#x"%(ins.operands[0].imm-IB)
    print("  %#x  %-8s %-46s%s"%(a,ins.mnemonic,ins.op_str,ex))
