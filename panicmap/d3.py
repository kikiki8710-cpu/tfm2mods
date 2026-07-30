import struct,re,sys
from capstone import *
from pe import load, rva2off
from fn import func_of
data,IB,S=load()
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RIP=re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')
def dump(st,en=None):
    if en is None:
        r=func_of(st); en=r[1] if r else st+0x200
    o=rva2off(S,st); code=data[o:o+(en-st)]
    print('=== 0x%x .. 0x%x (%d)'%(st,en,en-st))
    for (a,sz,mn,op) in md.disasm_lite(code,st):
        m=RIP.search(op); ann=''
        if m:
            t=a+sz+(int(m.group(2),16) if m.group(1)=='+' else -int(m.group(2),16))
            ann='  ; ->0x%x'%t
        print('%08x  %-8s %s%s'%(a,mn,op,ann))
if __name__=='__main__':
    a=int(sys.argv[1],16); b=int(sys.argv[2],16) if len(sys.argv)>2 else None
    dump(a,b)
