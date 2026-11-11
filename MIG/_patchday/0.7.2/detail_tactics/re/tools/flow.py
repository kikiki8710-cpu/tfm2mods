# symbolic copy propagation (must): where entry-arg labels live at window time
# usage: python -X utf8 flow.py <old|new> <func_start> <func_end> <win_rva>... [-filter]
import sys, struct
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import *
from capstone import CS_AC_WRITE
P={'old':r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe','new':r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
VOL={'rax','rcx','rdx','r8','r9','r10','r11'}
SUB={'eax':'rax','ax':'rax','al':'rax','ah':'rax','ebx':'rbx','bx':'rbx','bl':'rbx','bh':'rbx','ecx':'rcx','cx':'rcx','cl':'rcx','ch':'rcx','edx':'rdx','dx':'rdx','dl':'rdx','dh':'rdx','esi':'rsi','si':'rsi','sil':'rsi','edi':'rdi','di':'rdi','dil':'rdi','ebp':'rbp','bp':'rbp','bpl':'rbp','esp':'rsp','sp':'rsp','spl':'rsp'}
G64=set(['rax','rbx','rcx','rdx','rsi','rdi','rbp','rsp']+['r%d'%i for i in range(8,16)])
def full(name):
    if not name: return None
    if name in SUB: return SUB[name]
    if name in G64: return name
    if name.startswith('r') and name[1:].rstrip('dwb').isdigit(): return 'r'+name[1:].rstrip('dwb')
    return None
_cache={}
def img_of(v):
    if v not in _cache: _cache[v]=A.Img(P[v])
    return _cache[v]
def analyze(v,fs,fe,wins,entry_labels=None,show=None,quiet=False):
    img=img_of(v)
    ins=list(md.disasm(img.code(fs,fe-fs),A.BASE+fs))
    idx={x.address-A.BASE:i for i,x in enumerate(ins)}
    rsp=0; rbp_off=None; prol_end=0; chk=None
    for i,x in enumerate(ins[:24]):
        if x.mnemonic=='push': rsp-=8
        elif x.mnemonic=='mov' and x.op_str.startswith('eax, ') and x.operands[1].type==X86_OP_IMM: chk=x.operands[1].imm
        elif x.mnemonic=='sub' and x.op_str=='rsp, rax' and chk is not None:   # chkstk 프롤로그(0.7.2 G5 가 찾은 결함: 안 세면 A5+ 슬롯이 틀린다)
            rsp-=chk; prol_end=i+1
        elif x.mnemonic=='sub' and x.op_str.startswith('rsp,') and x.operands[1].type==X86_OP_IMM:
            rsp-=x.operands[1].imm; prol_end=i+1
        elif x.mnemonic=='lea' and x.op_str.startswith('rbp, [rsp'):
            rbp_off=rsp+x.operands[1].mem.disp; prol_end=i+1; break
        elif x.mnemonic=='mov' and x.op_str=='rbp, rsp':
            rbp_off=rsp; prol_end=i+1; break
        elif x.mnemonic=='call' and rsp<0 and prol_end==0:
            pass
    frame=rsp
    def xr(r): return md.reg_name(r) if r else None
    def slot(op):
        m=op.mem
        if m.index!=0: return None
        b=xr(m.base)
        if b=='rsp': return 'S%+x'%(frame+m.disp)
        if b=='rbp' and rbp_off is not None: return 'S%+x'%(rbp_off+m.disp)
        return None
    def succ(i):
        x=ins[i]; m=x.mnemonic
        if m in ('ret','int3','ud2'): return []
        if m=='jmp':
            op=x.operands[0]
            if op.type==X86_OP_IMM:
                t=op.imm-A.BASE
                return [idx[t]] if t in idx else []
            base=None; cnt=None
            for k in range(i-1,max(0,i-40),-1):
                y=ins[k]
                if base is None and y.mnemonic=='lea' and y.operands[1].type==X86_OP_MEM and y.operands[1].mem.base==X86_REG_RIP:
                    base=y.address+y.size+y.operands[1].mem.disp-A.BASE
                if cnt is None and y.mnemonic=='cmp' and len(y.operands)==2 and y.operands[1].type==X86_OP_IMM:
                    cnt=y.operands[1].imm+1
            if base is None: return []
            res=[]
            for k in range(cnt if cnt and cnt<200 else 64):
                t=base+struct.unpack('<i',img.code(base+4*k,4))[0]
                if t in idx: res.append(idx[t])
                else: break
            return res
        out=[]
        if m.startswith('j'):
            t=x.operands[0].imm-A.BASE
            if t in idx: out.append(idx[t])
        if i+1<len(ins): out.append(i+1)
        return out
    st0=dict(entry_labels or {'rcx':'A1','rdx':'A2','r8':'A3','r9':'A4'})
    for n in range(5,14): st0['S+%x'%(0x28+8*(n-5))]='A%d'%n
    states=[None]*len(ins); states[prol_end]=st0
    work=[prol_end]
    def transfer(i,s):
        s=dict(s); x=ins[i]; m=x.mnemonic; ops=x.operands
        def val(op):
            if op.type==X86_OP_REG: return s.get(full(xr(op.reg)) or '')
            if op.type==X86_OP_MEM:
                sl=slot(op)
                if sl: return s.get(sl)
                if op.mem.base and op.mem.base!=X86_REG_RIP and op.mem.index==0:
                    b=full(xr(op.mem.base))
                    if b in s: return '[%s%+x]'%(s[b],op.mem.disp)
            return None
        def put(op,vv):
            if op.type==X86_OP_REG:
                r=full(xr(op.reg))
                if r:
                    if vv is None or op.size!=8: s.pop(r,None)
                    else: s[r]=vv
            elif op.type==X86_OP_MEM:
                sl=slot(op)
                if sl:
                    if vv is None or op.size!=8: s.pop(sl,None)
                    else: s[sl]=vv
        if m=='mov' and len(ops)==2:
            put(ops[0],val(ops[1]) if ops[0].size==8 else None); return s
        if m=='lea':
            mo=ops[1].mem; vv=None
            if mo.base and mo.base!=X86_REG_RIP and mo.index==0 and mo.disp==0:
                b=full(xr(mo.base)); vv=s.get(b)
            put(ops[0],vv); return s
        if m=='call':
            for r in VOL: s.pop(r,None)
            return s
        if m=='xchg':
            a,b=val(ops[0]),val(ops[1]); put(ops[0],b); put(ops[1],a); return s
        try:
            rr,ww=x.regs_access()
        except Exception:
            ww=[]
        for r in ww:
            f=full(md.reg_name(r))
            if f and f not in ('rsp',): s.pop(f,None)
        for op in ops:
            if op.type==X86_OP_MEM and (op.access & CS_AC_WRITE):
                sl=slot(op)
                if sl: s.pop(sl,None)
        return s
    while work:
        i=work.pop()
        out=transfer(i,states[i])
        for j in succ(i):
            if states[j] is None:
                states[j]=out; work.append(j)
            else:
                nw={k:vv for k,vv in states[j].items() if out.get(k)==vv}
                if nw!=states[j]:
                    states[j]=nw; work.append(j)
    res={}
    for w in wins:
        i=idx.get(w)
        if i is None:
            print('win %x not insn boundary'%w); continue
        s=states[i]
        if not quiet:
            print('== %s win %x  body_frame=%#x rbp=entry%s'%(v,w,-frame,hex(rbp_off) if rbp_off is not None else None))
            if s is None: print('   UNREACHABLE'); continue
            for k,vv in sorted(s.items(), key=lambda kv:kv[1]):
                if show and not any(p in vv for p in show): continue
                if k.startswith('S'):
                    off=int(k[1:],16)
                    print('   %-8s = [winrsp%+#x]%s  <- %s'%(k,off-frame, ('  [rbp%+#x]'%(off-rbp_off)) if rbp_off is not None else '',vv))
                else:
                    print('   %-8s <- %s'%(k,vv))
        res[w]=s
    return res
if __name__=='__main__':
    v=sys.argv[1]; fs=int(sys.argv[2],16); fe=int(sys.argv[3],16)
    wins=[int(a,16) for a in sys.argv[4:] if not a.startswith('-')]
    show=[a[1:] for a in sys.argv[4:] if a.startswith('-')] or None
    analyze(v,fs,fe,wins,show=show)