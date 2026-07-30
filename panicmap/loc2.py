import re, struct, pickle, collections
from capstone import *
from pe import load, rva2off
from fn import FN
data,IB,S=load(); md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RIP=re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')
def decode_loc(rva):
    if rva is None: return None
    o=rva2off(S,rva)
    if o is None: return None
    try: p,l,line,col=struct.unpack_from('<QQII',data,o)
    except: return None
    fo=rva2off(S,p-IB)
    if fo is None or not (1<l<300): return None
    f=data[fo:fo+l]
    try: f=f.decode('utf-8')
    except: return None
    if not all(32<=ord(c)<127 for c in f): return None
    return (f,line,col)
res=[]
for (st,en,un) in FN:
    if en-st<=0 or en-st>0x200000: continue
    o=rva2off(S,st)
    if o is None: continue
    leas={}; imm={}
    for (a,sz,mn,op) in md.disasm_lite(data[o:o+en-st],st):
        if mn=='lea':
            m=RIP.search(op)
            if m:
                d=int(m.group(2),16)
                if m.group(1)=='-': d=-d
                leas[op.split(',')[0].strip()]=a+sz+d
        elif mn=='mov':
            p=op.split(',')
            if len(p)==2 and p[1].strip().startswith('0x') and p[0].strip() in ('ecx','edx','rcx','rdx','r8d','r8'):
                try: imm[p[0].strip()]=int(p[1].strip(),16)
                except: pass
        elif mn=='call' and op.startswith('0x'):
            t=int(op,16)
            if t==0x2c08353:
                res.append((st,a,'bounds',decode_loc(leas.get('r8')),imm.get('ecx'),imm.get('edx')))
            leas={}; imm={}
pickle.dump(res,open('bounds.pkl','wb'),2)
print(len(res),'bounds sites;', sum(1 for r in res if r[5] is not None),'with const len')
c=collections.Counter(r[5] for r in res if r[5] is not None)
print(c.most_common(20))
