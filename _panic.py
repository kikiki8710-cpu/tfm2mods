import pefile, capstone, sys
EXE=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
pe=pefile.PE(EXE,fast_load=True); IB=0x140000000
secs=[(s.VirtualAddress,max(s.Misc_VirtualSize,s.SizeOfRawData),s.PointerToRawData) for s in pe.sections]
raw=open(EXE,'rb').read()
def r2o(rva):
    for va,sz,po in secs:
        if va<=rva<va+sz: return po+(rva-va)
def u64(a):
    o=r2o(a-IB); return int.from_bytes(raw[o:o+8],'little') if o else 0
def rs(a,ml=100):
    o=r2o(a-IB)
    if o is None: return None
    b=raw[o:o+ml]; e=b.find(b'\x00'); e=ml if e<0 else e
    try:
        s=b[:e].decode('utf-8')
        return s if all(9<=ord(c)<127 for c in s) else None
    except: return None
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64); md.detail=True
start=int(sys.argv[1],16); n=int(sys.argv[2],16)
o=r2o(start-IB); code=raw[o:o+n]; seen=set()
for insn in md.disasm(code,start):
    if insn.mnemonic=='lea':
        for op in insn.operands:
            if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
                t=insn.address+insn.size+op.mem.disp
                # try interpret target as Location struct: [t]=file ptr, [t+8]=len
                p=u64(t); ln=u64(t+8)
                s=rs(p) if 4<ln<200 else None
                if s and '.rs' in s and t not in seen:
                    seen.add(t); print(f"  0x{insn.address:x}: {s} (len={ln})")
