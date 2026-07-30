# -*- coding: utf-8 -*-
# _locmap.py <exe> <start> <size> [markrva...] : 함수 내 panic Location 을 주소순으로 나열(+마커 위치 표시)
import sys, io, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
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
def u64(va):
    o=roff(va-IB); return int.from_bytes(raw[o:o+8],'little') if o is not None else 0
def rsl(va,ln):
    o=roff(va-IB)
    if o is None: return None
    try: s=raw[o:o+ln].decode('utf-8')
    except Exception: return None
    return s if all(9<=ord(c)<127 for c in s) else None
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
start=int(sys.argv[2],16); n=int(sys.argv[3],16)
marks=set(int(x,16) for x in sys.argv[4:])
o=roff(start)
ev=[]
for ins in md.disasm(raw[o:o+n], IB+start):
    a=ins.address-IB
    if ins.mnemonic=='lea':
        for op in ins.operands:
            if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
                t=ins.address+ins.size+op.mem.disp
                p,ln=u64(t),u64(t+8)
                s=rsl(p,ln) if 4<ln<200 else None
                if s and s.endswith('.rs'):
                    oo=roff(t+16-IB); line=int.from_bytes(raw[oo:oo+4],'little') if oo else 0
                    ev.append((a,"LOC %s:%d"%(s.split(chr(92))[-1],line)))
    if ins.mnemonic=='call' and len(ins.operands)==1 and ins.operands[0].type==capstone.x86.X86_OP_IMM:
        tg=ins.operands[0].imm-IB
        if tg in marks or a in marks or (a+ins.size) in marks:
            ev.append((a,"*** CALL %#x (ret %#x) ***"%(tg,a+ins.size)))
for a,t in ev: print("  %#x  %s"%(a,t))
