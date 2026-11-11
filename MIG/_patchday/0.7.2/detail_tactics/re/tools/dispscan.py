# usage: dispscan.py old|new DISP(hex) [mnemonic-filter]  -> instructions with memory displacement DISP (disasm per function containing raw disp bytes)
import sys, struct
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM
P = {'old': r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe', 'new': r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
img = A.Img(P[sys.argv[1]]); disp = int(sys.argv[2], 16); filt = sys.argv[3] if len(sys.argv) > 3 else None
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
needle = struct.pack('<i', disp)
d = img.data; funcs = set()
for nm, va, sz, pr in img.secs:
    if nm != '.text': continue
    blob = d[pr:pr+sz]; i = blob.find(needle)
    while i >= 0:
        f = img.owner(va + i)
        if f: funcs.add(f)
        i = blob.find(needle, i+1)
for b, e in sorted(funcs):
    for x in md.disasm(img.code(b, e-b), A.BASE + b):
        for op in x.operands:
            if op.type == X86_OP_MEM and op.mem.disp == disp:
                if filt and filt not in x.mnemonic: continue
                # write if first operand is mem
                w = 'W' if x.operands[0].type == X86_OP_MEM and x.mnemonic.startswith(('mov', 'add', 'sub', 'inc', 'or', 'and')) else 'R'
                print(f'{w} {x.address-A.BASE:#x} (fn {b:#x}) {x.mnemonic} {x.op_str}')
