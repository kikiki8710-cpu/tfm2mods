import pefile, capstone, sys
EXE=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
pe = pefile.PE(EXE, fast_load=True)
IB = 0x140000000
secs=[(s.VirtualAddress, max(s.Misc_VirtualSize,s.SizeOfRawData), s.PointerToRawData) for s in pe.sections]
raw=open(EXE,'rb').read()
def rva2off(rva):
    for va,sz,po in secs:
        if va<=rva<va+sz: return po+(rva-va)
    return None
def readstr(abs_addr, maxlen=120):
    off=rva2off(abs_addr-IB)
    if off is None: return None
    b=raw[off:off+maxlen]
    end=b.find(b'\x00')
    if end<0: end=maxlen
    try: return b[:end].decode('utf-8','replace')
    except: return None
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail=True
# scan region for lea reg,[rip+disp] whose target string contains ".rs" or "src\\"
start=int(sys.argv[1],16); n=int(sys.argv[2],16)
off=rva2off(start-IB); code=raw[off:off+n]
seen=set()
for insn in md.disasm(code, start):
    if insn.mnemonic=='lea':
        for op in insn.operands:
            if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
                tgt=insn.address+insn.size+op.mem.disp
                s=readstr(tgt)
                if s and ('.rs' in s or 'system' in s or 'src' in s):
                    if tgt not in seen:
                        seen.add(tgt)
                        print(f"0x{insn.address:x} -> 0x{tgt:x}: {s[:90]}")
