# -*- coding: utf-8 -*-
# _reach.py <exe> <fnstart> <size> <table> <narm> <site1,site2,...>
import sys, io, struct, collections
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
st=int(sys.argv[2],16); n=int(sys.argv[3],16); tb=int(sys.argv[4],16); na=int(sys.argv[5])
sites=[int(x,16) for x in sys.argv[6].split(',')]
o=roff(tb)
arms=[tb+int.from_bytes(raw[o+i*4:o+i*4+4],'little',signed=True) for i in range(na)]
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
code=raw[roff(st):roff(st)+n]
# 선형 디스어셈 인덱스
ins={}
for i in md.disasm(code, IB+st):
    ins[i.address-IB]=i
def succ(a):
    i=ins.get(a)
    if i is None: return []
    m=i.mnemonic
    nxt=a+i.size
    if m=='jmp':
        op=i.operands[0]
        return [op.imm-IB] if op.type==capstone.x86.X86_OP_IMM else []
    if m in ('ret','ud2'): return []
    if m.startswith('j'):
        op=i.operands[0]
        r=[nxt]
        if op.type==capstone.x86.X86_OP_IMM: r.append(op.imm-IB)
        return r
    if m=='call':
        # noreturn 가능성 무시
        return [nxt]
    return [nxt]
res=collections.defaultdict(list)
for k,a0 in enumerate(arms):
    seen=set(); stack=[a0]
    while stack:
        a=stack.pop()
        if a in seen or not (st<=a<st+n): continue
        seen.add(a)
        stack.extend(succ(a))
    for s in sites:
        if s in seen: res[s].append(k)
for s in sites:
    print("site %#x reached by arms: %s"%(s,res[s]))
