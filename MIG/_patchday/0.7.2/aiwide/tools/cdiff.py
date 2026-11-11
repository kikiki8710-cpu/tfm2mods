import sys, difflib; sys.path.insert(0,'.')
from common import *
LO,LN=locs()
STRUCT_LIKE=set()
def seq(img,k,LOC,F):
    out=[]
    for i in dis(img,k):
        mn=i.mnemonic; tok=None
        if mn=='call':
            op=i.operands[0]
            if op.type==X86_OP_IMM:
                t=op.imm-BASE; kk=img.owner(t)
                tok=('call', short(F[kk]['top']) if kk in F else 'ext')
            elif op.type==X86_OP_MEM and op.mem.base!=X86_REG_RIP:
                tok=('vcall', hex(op.mem.disp))
            out.append((tok or ('vcall','reg'),i.address-BASE)); continue
        for op in i.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=i.address+i.size+op.mem.disp-BASE
                if t in LOC: out.append((('LOC','%s:%d'%(short(LOC[t][0]).split(chr(92))[-1],LOC[t][1])),i.address-BASE))
                elif mn in ('movss','movsd','mulss','mulsd','addss','addsd','comiss','ucomiss','comisd','ucomisd','divss','divsd','subss','subsd'):
                    w=4 if mn.endswith('ss') else 8
                    v=struct.unpack('<f' if w==4 else '<d', img.code(t,w))[0]
                    out.append((('flt',mn,round(v,6)),i.address-BASE))
        if mn.startswith('j') or mn in ('jmp',): continue
        for op in i.operands:
            if op.type==X86_OP_IMM:
                v=op.imm
                if 0x140000000<=v<0x146000000: continue
                if mn in ('sub','add') and i.operands[0].type==X86_OP_REG and md.reg_name(i.operands[0].reg)=='rsp': continue
                if abs(v)<3: continue
                out.append(((mn if mn in ('cmp','imul','mov','add','sub','and','or','test','shl','shr','sar','xor','lea') else 'op', v),i.address-BASE))
    return out
def show(a,b,ctx=0,maxl=80):
    ko=io_.owner(a); kn=inn.owner(b)
    so=seq(io_,ko,LO,fo); sn=seq(inn,kn,LN,fn)
    def norm(t):
        t=t[0]
        if t[0]=='LOC': return ('LOC', t[1].split(':')[0])
        if t[0] in ('mov','add','sub','imul','cmp','lea','and','test') and isinstance(t[1],int):
            v=t[1]
            if v in (0xae0,): v=0xea0
            if v==0x348: v=0x400
            if v==-0xae0: v=-0xea0
            return (t[0],v)
        return t
    A_=[norm(x) for x in so]; B_=[norm(x) for x in sn]
    sm=difflib.SequenceMatcher(None,A_,B_,autojunk=False)
    lines=[]
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag=='equal': continue
        o=' '.join(fmt(so[i]) for i in range(i1,i2)); n=' '.join(fmt(sn[j]) for j in range(j1,j2))
        lines.append('  %s @o%#x/n%#x: [%s] -> [%s]'%(tag, so[i1][1] if i1<len(so) else 0, sn[j1][1] if j1<len(sn) else 0, o[:400], n[:400]))
    print('=== %#x -> %#x  ratio %.3f  ops %d'%(ko[0],kn[0],sm.ratio(),len(lines)))
    for l in lines[:maxl]: print(l)
def fmt(x):
    t=x[0]
    if t[0] in ('call','vcall','LOC'): return '%s(%s)'%(t[0],t[1])
    if t[0]=='flt': return 'f%s=%g'%(t[1][-2:],t[2])
    v=t[1]
    return '%s %s'%(t[0], (('%d'%v) if abs(v)<100000 else hex(v & 0xffffffffffffffff)))
if __name__=='__main__':
    a=int(sys.argv[1],16); b=int(sys.argv[2],16); m=int(sys.argv[3]) if len(sys.argv)>3 else 80
    show(a,b,maxl=m)
