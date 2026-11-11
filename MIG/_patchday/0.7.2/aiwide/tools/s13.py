import sys,re; sys.path.insert(0,'.')
from common import *
def parse(img, fstart, names_at=None):
    k=img.owner(fstart)
    regs={}; slots={}
    for i in dis(img,k):
        ops=i.operands
        if i.mnemonic=='lea' and ops[1].type==X86_OP_MEM:
            m=ops[1].mem
            dst=md.reg_name(ops[0].reg)
            if m.base==X86_REG_RIP: regs[dst]=('rip',i.address+i.size+m.disp-BASE)
            elif md.reg_name(m.base)=='rax': regs[dst]=('f',m.disp)
            elif md.reg_name(m.base)=='rsp': regs[dst]=('stk',m.disp)
        elif i.mnemonic=='mov' and ops[0].type==X86_OP_MEM and md.reg_name(ops[0].mem.base)=='rsp' and ops[1].type==X86_OP_REG:
            src=md.reg_name(ops[1].reg)
            if src=='rax' and src not in regs: slots[ops[0].mem.disp]=('f',0)
            elif src in regs: slots[ops[0].mem.disp]=regs[src]
            elif src=='rax': slots[ops[0].mem.disp]=('f',0)
        elif i.mnemonic=='call': break
        if i.mnemonic=='mov' and ops[0].type==X86_OP_REG and md.reg_name(ops[0].reg)=='rax' and ops[1].type==X86_OP_MEM: regs.pop('rax',None)
    return slots
def names(img,at,n):
    out=[]
    for j in range(n):
        p,l=struct.unpack('<QQ',img.code(at+16*j,16))
        out.append(img.code(p-BASE,l).decode('latin1'))
    return out
def vtsize(img,vt): return struct.unpack('<Q',img.code(vt+8,8))[0]
img=inn if sys.argv[1]=='n' else io_
f=int(sys.argv[2],16); na=int(sys.argv[3],16); n=int(sys.argv[4])
sl=parse(img,f)
nm=names(img,na,n)
xs=sorted(x for x in sl if x>=0x40)
j=0
res=[]
for x in range(0x48,0x48+16*n,16):
    a=sl.get(x); b=sl.get(x+8)
    if a and a[0]=='stk': a=sl.get(a[1])
    res.append((nm[(x-0x48)//16], a, b, vtsize(img,b[1]) if b and b[0]=='rip' else None))
for r in sorted(res,key=lambda r:(r[1][1] if r[1] and r[1][0]=='f' else 99999)):
    print('%-26s %-14s size=%s'%(r[0], (hex(r[1][1]) if r[1] and r[1][0]=='f' else str(r[1])), hex(r[3]) if r[3] is not None else '?'))
