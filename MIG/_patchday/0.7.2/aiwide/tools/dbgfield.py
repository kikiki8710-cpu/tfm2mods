# parse debug_struct().field(name, &val) chains: report name + r9 stack slot / value vtable
from w7lib import *
def run(w, f, show=True):
    im=img(w); ins=disf(im,f)
    regs={}; out=[]
    stk={}
    for i in ins:
        ops=i.operands
        if i.mnemonic=='lea' and ops[1].type==X86_OP_MEM:
            m=ops[1].mem; dst=md.reg_name(ops[0].reg)
            if m.base==X86_REG_RIP: regs[dst]=('rip',i.address+i.size+m.disp-BASE)
            elif m.base and m.index==0: regs[dst]=(md.reg_name(m.base),m.disp)
        elif i.mnemonic=='mov' and ops[0].type==X86_OP_REG and ops[1].type==X86_OP_IMM:
            regs[md.reg_name(ops[0].reg)]=('imm',ops[1].imm)
        elif i.mnemonic=='mov' and ops[0].type==X86_OP_REG and ops[1].type==X86_OP_REG:
            s=md.reg_name(ops[1].reg)
            regs[md.reg_name(ops[0].reg)]=regs.get(s,('reg',s))
        elif i.mnemonic=='mov' and ops[0].type==X86_OP_MEM and ops[0].mem.base and md.reg_name(ops[0].mem.base)=='rsp' and ops[1].type==X86_OP_REG:
            stk[ops[0].mem.disp]=regs.get(md.reg_name(ops[1].reg),('reg',md.reg_name(ops[1].reg)))
        elif i.mnemonic=='call':
            rdx=regs.get('rdx'); r8=regs.get('r8d') or regs.get('r8'); r9=regs.get('r9')
            nm=None
            if rdx and rdx[0]=='rip' and r8 and r8[0]=='imm' and 0<r8[1]<64:
                nm=im.code(rdx[1],r8[1]).decode('latin1','replace')
            val=None; vt=None
            if r9 and r9[0]=='rsp':
                val=stk.get(r9[1]); vt=stk.get(r9[1]+8)
            out.append((i.address-BASE, nm, val, vt, i.op_str))
            for r in ('rax','rcx','rdx','r8','r9','r10','r11','r8d','edx'): regs.pop(r,None)
    if show:
        for o in out: print('%#x %-28s val=%s vt=%s  %s'%(o[0], o[1], o[2], (hex(o[3][1]) if o[3] and o[3][0]=='rip' else o[3]), o[4]))
    return out
if __name__=='__main__':
    run(sys.argv[1], int(sys.argv[2],16))
