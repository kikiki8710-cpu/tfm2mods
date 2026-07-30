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
# scan backward for int3 padding (cc cc) boundary => function start after it
def find_start(target):
    off=rva2off(target-IB)
    # look back up to 0x8000 bytes
    for back in range(0, 0x9000):
        o=off-back
        # prologue heuristic: preceding bytes are cc cc (int3 pad) or c3 cc (ret+pad)
        if raw[o-1]==0xcc and raw[o-2]==0xcc:
            return target-back
        if raw[o-1]==0xcc and (raw[o-2]==0xc3):
            return target-back
    return None
for t in [int(x,16) for x in sys.argv[1:]]:
    print(hex(t), "->", hex(find_start(t)) if find_start(t) else "??")
