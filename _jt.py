# -*- coding: utf-8 -*-
# _jt.py <exe> <start> <size> : 함수 내 간접 jmp(점프테이블) 찾기 + i32 오프셋 테이블 디코드
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
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
start=int(sys.argv[2],16); n=int(sys.argv[3],16)
o=roff(start)
ins_list=list(md.disasm(raw[o:o+n],IB+start))
# 패턴: lea rX,[rip+T] ... mov eXX,[rX+rY*4] ... add rX,rY(or rax) ... jmp rX
leas={}
for k,ins in enumerate(ins_list):
    if ins.mnemonic=='lea':
        for op in ins.operands:
            if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
                leas[k]=ins.address+ins.size+op.mem.disp
    if ins.mnemonic=='jmp' and ins.operands[0].type==capstone.x86.X86_OP_REG:
        reg=ins.reg_name(ins.operands[0].reg)
        # 뒤로 40개 스캔해서 같은 reg 로 lea rip-rel 찾기
        for j in range(k-1,max(0,k-40),-1):
            p=ins_list[j]
            if p.mnemonic=='lea' and j in leas and p.op_str.split(',')[0].strip()==reg:
                tbl=leas[j]
                print("jmp %s @%#x  table=%#x"%(reg,ins.address-IB,tbl-IB))
                to=roff(tbl-IB)
                ents=[]
                for i2 in range(140):
                    v=int.from_bytes(raw[to+i2*4:to+i2*4+4],'little',signed=True)
                    t=(tbl+v)-IB
                    if not (start<=t<start+n): break
                    ents.append(t)
                for i2,t in enumerate(ents): print("   arm %3d -> %#x"%(i2,t))
                break
