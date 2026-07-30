import pefile, capstone, struct, sys
from capstone.x86 import (X86_OP_MEM, X86_OP_REG, X86_OP_IMM, X86_REG_RIP,
    X86_REG_RBP, X86_REG_RSP, X86_REG_RDI, X86_REG_RCX)
EXE=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
IB=0x140000000
pe=pefile.PE(EXE,fast_load=True)
def o(rva):
    for s in pe.sections:
        if s.VirtualAddress<=rva<s.VirtualAddress+max(s.Misc_VirtualSize,s.SizeOfRawData):
            return s.PointerToRawData+(rva-s.VirtualAddress)
    raise ValueError(hex(rva))
data=open(EXE,'rb').read()
def rb(rva,n): return data[o(rva):o(rva)+n]
def ri32(rva): return struct.unpack('<i',rb(rva,4))[0]
def ru32(rva): return struct.unpack('<I',rb(rva,4))[0]

GB=0x20def90; SIZE=0x5d08
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True

NM={0x202da10:'F80320_scorer',0x1fecbe0:'CAND_FILTER',0x1af16d0:'vec_grow',
    0x189ae20:'fcd980_RNG',0x189af90:'fcdaf0_RNG',0x222f3c0:'chacha_refill',
    0x18a1da0:'fcdaf0?',0x2220f70:'chacha?'}

code=rb(GB,SIZE)
insns=list(md.disasm(code, IB+GB))

mode=sys.argv[1] if len(sys.argv)>1 else "all"

# Build address->index, and gather labels (jump/call targets within function)
addr2idx={}
for i,ins in enumerate(insns): addr2idx[ins.address-IB]=i

def reg_name(r): return md.reg_name(r) if r else None

# --- collect: rbp-relative loads/stores, cmps, jumps, jumptable-style indirect jmps
def is_rbp(op): return op.type==X86_OP_MEM and op.mem.base==X86_REG_RBP and op.mem.index==0
def rip_tgt(ins,op): return ins.address+ins.size+op.mem.disp-IB

# track which registers currently alias the out pointer (sret).
# entry: rcx=out; 0x20defb? mov rdi,rcx ; mov [rbp+0x290],rdi
# We'll do a light forward dataflow over the linear stream for OUT aliasing.
OUT_SLOT=0x290  # [rbp+0x290] holds out per prompt

def main():
    if mode in("all","jumps"): do_jumps()
    if mode in("all","cmps"): do_cmps()
    if mode in("all","outwrites"): do_outwrites()
    if mode in("all","calls"): do_calls()
    if mode in("all","jt"): do_jumptables()
    if mode=="range": do_range(int(sys.argv[2],16),int(sys.argv[3],16))

def do_jumps():
    print("=== indirect jumps (jmp reg / jmp [mem]) ===")
    for ins in insns:
        if ins.mnemonic in('jmp',) and ins.operands and ins.operands[0].type!=X86_OP_IMM:
            print(f"  {ins.address-IB:#09x}: {ins.mnemonic} {ins.op_str}")
    # count conditional branches
    cond=sum(1 for ins in insns if ins.mnemonic.startswith('j') and ins.mnemonic!='jmp')
    print(f"  [conditional branches total: {cond}]")

def do_cmps():
    # cmp/test against rbp-locals or against the disc key; print with imm
    print("=== cmp/test sites (with imm or against locals of interest) ===")
    for ins in insns:
        if ins.mnemonic in('cmp','test'):
            ops=ins.operands
            # only print if involves an immediate or memory
            has_imm=any(op.type==X86_OP_IMM for op in ops)
            has_mem=any(op.type==X86_OP_MEM for op in ops)
            if has_imm:
                print(f"  {ins.address-IB:#09x}: {ins.mnemonic} {ins.op_str}")

def do_outwrites():
    # Forward dataflow: track regs that hold OUT (alias of [rbp+0x290]).
    print("=== OUT pointer writes (kind@+0, x@+8, y@+0x10, etc.) ===")
    out_regs=set(['rcx'])  # entry rcx=out
    # We'll reset out_regs membership when a reg is overwritten by non-out source.
    for ins in insns:
        rva=ins.address-IB
        mn=ins.mnemonic; ops=ins.operands
        # detect load of OUT from [rbp+0x290] into a reg
        if mn=='mov' and len(ops)==2:
            d,s=ops
            # mov reg, [rbp+0x290]
            if d.type==X86_OP_REG and s.type==X86_OP_MEM and s.mem.base==X86_REG_RBP and s.mem.disp==OUT_SLOT and s.mem.index==0:
                out_regs.add(reg_name(d.reg)); continue
            # mov reg, out_reg  -> propagate
            if d.type==X86_OP_REG and s.type==X86_OP_REG and reg_name(s.reg) in out_regs:
                out_regs.add(reg_name(d.reg)); continue
            # mov rdi,rcx at entry then store -> keep rcx alias too
        # store into [out_reg + disp]
        if ops and ops[0].type==X86_OP_MEM:
            m=ops[0].mem
            base=reg_name(m.base) if m.base else None
            if base in out_regs and m.index==0 and m.disp<0x100:
                src=ins.op_str.split(',',1)[1].strip() if ',' in ins.op_str else '?'
                print(f"  {rva:#09x}: [{base}+{m.disp:#x}] <- {src}    ({mn})")
        # invalidate out aliasing if a reg gets a fresh non-out value (rough)
        if mn in('mov','lea','xor','add','sub','imul','movzx','movsxd','call') and ops and ops[0].type==X86_OP_REG:
            dn=reg_name(ops[0].reg)
            if mn=='mov' and len(ops)==2 and ops[1].type==X86_OP_REG and reg_name(ops[1].reg) in out_regs:
                pass # already handled (propagation)
            elif mn=='mov' and len(ops)==2 and ops[1].type==X86_OP_MEM and ops[1].mem.base==X86_REG_RBP and ops[1].mem.disp==OUT_SLOT:
                pass # already handled (reload)
            else:
                if dn in out_regs and dn!='rcx':
                    out_regs.discard(dn)
            if mn=='call':
                # volatile regs clobbered
                for vr in ('rax','rcx','rdx','r8','r9','r10','r11'):
                    out_regs.discard(vr)

def do_calls():
    print("=== call sites in order (rva : target) ===")
    for ins in insns:
        if ins.mnemonic=='call':
            rva=ins.address-IB
            try:
                t=int(ins.op_str,16)-IB
                print(f"  {rva:#09x}: call {NM.get(t,hex(t))}")
            except:
                print(f"  {rva:#09x}: call {ins.op_str}  [indirect]")

def do_jumptables():
    print("=== jump-table detection (movsxd + jmp reg, or jmp [base+idx*4]) ===")
    # Look for: lea reg,[rip+X] ; movsxd r2,[reg+idx*4] ; add ; jmp
    for i,ins in enumerate(insns):
        if ins.mnemonic=='jmp' and ins.operands and ins.operands[0].type==X86_OP_REG:
            rva=ins.address-IB
            # look back up to 12 insns for lea rip + movsxd
            ctx=insns[max(0,i-12):i+1]
            leas=[]
            for c in ctx:
                if c.mnemonic=='lea':
                    for op in c.operands:
                        if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                            leas.append((c.address-IB, rip_tgt(c,op)))
                if c.mnemonic in('movsxd','mov') and any(op.type==X86_OP_MEM and op.mem.scale==4 for op in c.operands):
                    print(f"    [idx scale=4] {c.address-IB:#09x}: {c.mnemonic} {c.op_str}")
            print(f"  jmp@{rva:#09x}: {ins.op_str}; rip-lea candidates -> {[hex(t) for _,t in leas]}")
            # try dump table at each lea target
            for _,tbl in leas:
                dump_jt(tbl)

def dump_jt(tbl):
    print(f"    table@{tbl:#x}:")
    for k in range(0,18):
        try:
            off=ri32(tbl+k*4)
            tgt=tbl+off
            inside = GB<=tgt<GB+SIZE
            mark=" <-in" if inside else ""
            print(f"      [{k:2}] +{off:#010x} -> {tgt:#09x}{mark}")
        except Exception as e:
            print(f"      [{k}] err {e}"); break

def do_range(lo,hi):
    for ins in insns:
        rva=ins.address-IB
        if lo<=rva<hi:
            ann=""
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                    ann=f"   ; rip->{rip_tgt(ins,op):#x}"
            if ins.mnemonic=='call':
                try: t=int(ins.op_str,16)-IB; ann="   ; "+NM.get(t,f"call {t:#x}")
                except: ann="   ; call ind"
            print(f"{rva:#09x}: {ins.mnemonic} {ins.op_str}{ann}")

main()
