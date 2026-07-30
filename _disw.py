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
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
def dis(start_abs, nbytes):
    off=rva2off(start_abs-IB); code=raw[off:off+nbytes]
    for insn in md.disasm(code, start_abs):
        print(f"0x{insn.address:x}: {insn.mnemonic}\t{insn.op_str}")
if __name__=="__main__":
    dis(int(sys.argv[1],16), int(sys.argv[2],16))
