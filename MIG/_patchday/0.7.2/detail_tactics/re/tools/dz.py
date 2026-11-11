# -*- coding: utf-8 -*-
# usage: python -X utf8 dz.py old|new START END   (RVA hex) ; or old|new func RVA
import sys, os, pickle
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP
P = {'old': r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe', 'new': r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
HERE = os.path.dirname(os.path.abspath(__file__))
which = sys.argv[1]
img = A.Img(P[which])
cp = os.path.join(HERE, f'loc_{which}.pkl')
if os.path.exists(cp):
    loc = pickle.load(open(cp, 'rb'))
else:
    loc = A.find_locations(img); pickle.dump(loc, open(cp, 'wb'))
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
if sys.argv[2] == 'func':
    r = int(sys.argv[3], 16); f = img.owner(r); b, e = f
    print(f'owner {b:#x}-{e:#x} size {e-b}')
    if len(sys.argv) > 4:
        b = int(sys.argv[4], 16); e = int(sys.argv[5], 16)
else:
    b = int(sys.argv[2], 16); e = int(sys.argv[3], 16)
f = img.owner(b)
fb = f[0] if f else b
fe = max(e, f[1]) if f else e
for x in md.disasm(img.code(fb, fe - fb), A.BASE + fb):
    r = x.address - A.BASE
    if r < b: continue
    if r >= e: break
    note = ''
    for op in x.operands:
        if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
            tg = x.address + x.size + op.mem.disp - A.BASE
            L = loc.get(tg)
            if L: note += f'  LOC {L[0].split(chr(92))[-1]}:{L[1]}:{L[2]}'
            else: note += f'  [{tg:#x}]'
    print(f'  {r:x}  {x.bytes.hex(" "):<34} {x.mnemonic} {x.op_str}{note}')
