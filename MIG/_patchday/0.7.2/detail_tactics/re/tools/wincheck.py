import sys
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_REG_RIP
img = A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe')
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
W = [(0xe42f1a,10),(0xe4a14b,6),(0xe3f6d0,7),(0xe42e1e,7),(0xe637c5,10),(0xe638dd,10),(0xe639a3,10),(0xe63a69,10),(0xe63b49,10),(0xe63065,10),
     (0xe62e7c,6),(0xe6766f,10),(0xe676ab,5),(0xe67037,7),(0xe6694b,7),(0xe66aa0,9),(0xe6524f,7),(0xe8f718,7),(0xe94401,7),(0x10e4d26,7),(0xe67776,7),
     (0xfcdc34,7),(0xfcdce9,7),(0xfcdd9e,7),(0xfcde53,7),(0xfcdefc,7),(0xe736a2,9),(0xe67702,7),(0xe49fe8,0)]
cache = {}
for rva, ln in W:
    if ln == 0: continue
    f = img.owner(rva); b, e = f
    if f not in cache:
        cache[f] = list(md.disasm(img.code(b, e-b), A.BASE+b))
    ins = cache[f]
    starts = {x.address-A.BASE for x in ins}
    targets = set()
    for x in ins:
        if x.group(1) or x.group(7) or x.mnemonic.startswith('j') or x.mnemonic == 'call':
            for op in x.operands:
                if op.type == X86_OP_IMM: targets.add(op.imm - A.BASE)
    inside = [t for t in targets if rva < t < rva+ln]
    ok_b = rva in starts and (rva+ln) in starts
    wi = [x for x in ins if rva <= x.address-A.BASE < rva+ln]
    desc = ' ; '.join(f'{x.mnemonic} {x.op_str}' for x in wi)
    riprel = any(op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP for x in wi for op in x.operands)
    hexb = img.code(rva, ln).hex(' ')
    print(f'{rva:#x} len{ln} owner {b:#x} bound={ok_b} jump_into={[hex(t) for t in inside]} rip={riprel} | {hexb} | {desc}')
