import pickle, struct, sys, re
from pe import load, rva2off, off2rva
from fn import func_of, FN
from capstone import *
X = pickle.load(open(r'C:\tfm2mods\panicmap\xref.pkl','rb'))
data, IB, SECTS = load()
calls, jmps, drefs = X['calls'], X['jmps'], X['drefs']
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RIP=re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')

def find_bytes(b):
    res=[];i=0
    while True:
        i=data.find(b,i)
        if i<0:break
        r=off2rva(SECTS,i)
        if r:res.append(r)
        i+=1
    return res

def dref(rva): return drefs.get(rva,[])
def callers(rva): return calls.get(rva,[])

def dis(st,n=200,end=None):
    o=rva2off(SECTS,st)
    ln = (end-st) if end else n*8
    code=data[o:o+ln]
    out=[];cnt=0
    for (a,sz,mn,op) in md.disasm_lite(code,st):
        s='%08x  %-7s %s'%(a,mn,op)
        m=RIP.search(op)
        if m:
            d=int(m.group(2),16)
            if m.group(1)=='-': d=-d
            s+='   ; ->0x%x'%(a+sz+d)
        out.append(s); cnt+=1
        if end is None and cnt>=n: break
        if end and a>=end: break
    return '\n'.join(out)

def fnof(rva):
    f=func_of(rva)
    return '0x%x'%f[0] if f else None

if __name__=='__main__':
    cmd=sys.argv[1]
    if cmd=='dref':
        r=int(sys.argv[2],16)
        for a,m in dref(r): print('0x%x %s  in %s'%(a,m,fnof(a)))
    elif cmd=='call':
        r=int(sys.argv[2],16)
        for a in callers(r): print('0x%x in %s'%(a,fnof(a)))
        for a in jmps.get(r,[]): print('JMP 0x%x in %s'%(a,fnof(a)))
    elif cmd=='dis':
        st=int(sys.argv[2],16)
        n=int(sys.argv[3]) if len(sys.argv)>3 else 120
        print(dis(st,n))
    elif cmd=='fdis':
        st=int(sys.argv[2],16)
        f=func_of(st); print('func 0x%x-0x%x'%f)
        print(dis(f[0],end=f[1]))
    elif cmd=='str':
        s=sys.argv[2].encode()
        for r in find_bytes(s)[:20]: print('0x%x'%r, [( '0x%x'%a,m,fnof(a)) for a,m in dref(r)][:20])
    elif cmd=='fn':
        r=int(sys.argv[2],16); print(fnof(r), func_of(r))
