import sys, re
from capstone import *
from pe import load, rva2off
from fn import func_of
data,IB,S=load(); md=Cs(CS_ARCH_X86,CS_MODE_64)
tgts=set(int(x,16) for x in sys.argv[1:])
f=func_of(list(tgts)[0])
o=rva2off(S,f[0])
for a,sz,mn,op in md.disasm_lite(data[o:o+f[1]-f[0]], f[0]):
    if op.startswith('0x') and int(op,16) in tgts and mn[0]=='j':
        print('0x%08x %s %s'%(a,mn,op))
