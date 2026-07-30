import sys, struct, re
from capstone import *
from capstone.x86 import *
from pe import load, rva2off, off2rva
from fn import func_of
data,IB,S=load()
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RIP=re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')
PANIC={0x2c08353:'panic_bounds_check',0x2c082b0:'unwrap_none',0x2c08560:'slice_range_fail',
       0x2c083f5:'panic_misc1',0x2c084ef:'panic_misc2',0x2c082d0:'expect/panic_str',0x2c08530:'panic_fmt'}

def cstr(rva,maxn=160):
    o=rva2off(S,rva)
    if o is None: return None
    b=data[o:o+maxn]; i=b.find(b'\0')
    return b[:i if i>=0 else maxn]

def scan(st,en,show_str=True,show_panic=True,show_calls=False):
    o=rva2off(S,st); code=data[o:o+(en-st)]
    for (a,sz,mn,op) in md.disasm_lite(code,st):
        if mn=='call' and op.startswith('0x'):
            t=int(op,16)
            if t in PANIC and show_panic:
                print('0x%08x  PANIC %s'%(a,PANIC[t]))
            elif show_calls:
                print('0x%08x  call 0x%x'%(a,t))
        if mn=='lea' and show_str:
            m=RIP.search(op)
            if m:
                d=int(m.group(2),16)
                if m.group(1)=='-': d=-d
                t=a+sz+d
                c=cstr(t,120)
                if c and len(c)>=6 and sum(1 for x in c if 32<=x<127 or x>=0xC0)>=len(c)*0.9:
                    try: sdec=c.decode('utf-8')
                    except: continue
                    print('0x%08x  STR 0x%x %r'%(a,t,sdec[:110]))
if __name__=='__main__':
    a=int(sys.argv[1],16); 
    f=func_of(a)
    b=int(sys.argv[2],16) if len(sys.argv)>2 else f[1]
    scan(a,b)
