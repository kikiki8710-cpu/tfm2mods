# -*- coding: utf-8 -*-
# _regmap.py <exe> <lo> <hi> : .pdata 함수별 대표 panic Location 파일
import sys, io, struct, bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
import capstone
EXE=sys.argv[1]; raw=open(EXE,'rb').read()
pe=struct.unpack_from("<I",raw,0x3c)[0]; nsec=struct.unpack_from("<H",raw,pe+6)[0]; opt=pe+24
IB=struct.unpack_from("<Q",raw,opt+24)[0]; sectab=opt+struct.unpack_from("<H",raw,pe+20)[0]
secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rr=struct.unpack_from("<IIII",raw,o+8); secs.append((nm,va,max(vsz,rsz),rr))
magic=struct.unpack_from("<H",raw,opt)[0]; ddir=opt+(112 if magic==0x20b else 96)
ex_rva,ex_sz=struct.unpack_from("<II",raw,ddir+3*8)
def roff(rva):
    for nm,va,sz,rr in secs:
        if va<=rva<va+sz: return rr+(rva-va)
po=roff(ex_rva); ranges={}
for i in range(ex_sz//12):
    b,e,u=struct.unpack_from("<III",raw,po+i*12)
    if e<=b or e-b>(1<<20): continue
    if b not in ranges or e>ranges[b]: ranges[b]=e
def u64(va):
    o=roff(va-IB); return int.from_bytes(raw[o:o+8],'little') if o is not None else 0
def rsl(va,ln):
    o=roff(va-IB)
    if o is None: return None
    try: s=raw[o:o+ln].decode('utf-8')
    except Exception: return None
    return s if all(9<=ord(c)<127 for c in s) else None
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
lo=int(sys.argv[2],16); hi=int(sys.argv[3],16)
import collections
for f in sorted(ranges):
    if not (lo<=f<hi): continue
    e=ranges[f]; o=roff(f)
    c=collections.Counter()
    for ins in md.disasm(raw[o:o+(e-f)], IB+f):
        if ins.mnemonic=='lea':
            for op in ins.operands:
                if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
                    t=ins.address+ins.size+op.mem.disp
                    p,ln=u64(t),u64(t+8)
                    s=rsl(p,ln) if 4<ln<200 else None
                    if s and s.endswith('.rs'):
                        oo=roff(t+16-IB); line=int.from_bytes(raw[oo:oo+4],'little') if oo else 0
                        c["%s:%d"%(s.split(chr(92))[-1],line)]+=1
    top=[k for k,_ in c.most_common(3)]
    print("  %#x size %#6x  %s"%(f,e-f,top))
