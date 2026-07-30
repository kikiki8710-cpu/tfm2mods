import sys, struct, bisect
from capstone import *
from capstone.x86 import *
from pe import load, rva2off, off2rva
from fn import func_of, FN, STARTS, data, IB, SECTS

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True

def read(rva, n):
    o = rva2off(SECTS, rva)
    if o is None: return None
    return data[o:o+n]

def cstr(rva, maxn=200):
    b = read(rva, maxn)
    if b is None: return None
    i = b.find(b'\0')
    return b[:i if i>=0 else maxn]

def annot(ins):
    s=''
    # rip-relative
    for op in ins.operands:
        if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
            tgt = ins.address + ins.size + op.mem.disp
            s += ' ; [rip]=0x%x' % tgt
            v = read(tgt, 8)
            if v: s += ' q=0x%x' % struct.unpack('<Q', v)[0]
    if ins.group(X86_GRP_CALL) or ins.group(X86_GRP_JUMP):
        for op in ins.operands:
            if op.type == X86_OP_IMM:
                t = op.imm
                f = func_of(t)
                if f and f[0]==t: s += ' ; ->FN(0x%x)'%t
    return s

def dis(start, end, lea_str=True):
    out=[]
    o = rva2off(SECTS, start)
    code = data[o:o+(end-start)]
    for ins in md.disasm(code, start):
        line = '0x%08x  %-24s %s %s' % (ins.address, ins.bytes.hex(), ins.mnemonic, ins.op_str)
        line += annot(ins)
        if lea_str and ins.mnemonic=='lea':
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                    t = ins.address+ins.size+op.mem.disp
                    c = cstr(t, 80)
                    if c and len(c)>=4 and all(32<=x<127 for x in c[:min(len(c),40)]):
                        line += ' ; STR "%s"' % repr(c[:60])
        out.append(line)
    return '\n'.join(out)

def disfn(rva):
    f = func_of(rva)
    if not f:
        print('no func for 0x%x'%rva); return
    print('== FN 0x%x - 0x%x (%d bytes) =='%(f[0],f[1],f[1]-f[0]))
    print(dis(f[0], f[1]))

if __name__=='__main__':
    a = int(sys.argv[1],16)
    if len(sys.argv)>2:
        b=int(sys.argv[2],16)
        print(dis(a,b))
    else:
        disfn(a)
