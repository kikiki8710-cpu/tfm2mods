import re, struct, pickle, sys, collections
from capstone import *
from pe import load, rva2off, off2rva
from fn import func_of, FN
data,IB,S=load()
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=False
RIP=re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')
PANIC={0x2c08353:('bounds','r8'),0x2c082b0:('unwrap_none','rcx'),0x2c08560:('slice_range','r9'),
       0x2c082d0:('expect','stack'),0x2c084ef:('misc2','?'),0x2c083f5:('misc1','?')}

def decode_loc(rva):
    o=rva2off(S,rva)
    if o is None: return None
    p,l,line,col=struct.unpack_from('<QQII',data,o)
    fr=p-IB
    fo=rva2off(S,fr)
    if fo is None or not (1<l<300): return None
    f=data[fo:fo+l]
    try: f=f.decode('utf-8')
    except: return None
    if not all(32<=ord(c)<127 for c in f): return None
    return (f,line,col)

def scan_fn(st,en):
    """returns list of (site, kind, loc)"""
    o=rva2off(S,st); code=data[o:o+(en-st)]
    res=[]; leas={}  # reg -> target
    for (a,sz,mn,op) in md.disasm_lite(code,st):
        if mn=='lea':
            m=RIP.search(op)
            if m:
                d=int(m.group(2),16)
                if m.group(1)=='-': d=-d
                reg=op.split(',')[0].strip()
                leas[reg]=a+sz+d
        elif mn=='call' and op.startswith('0x'):
            t=int(op,16)
            if t in PANIC:
                kind,reg=PANIC[t]
                tgt=leas.get(reg)
                res.append((a,kind,decode_loc(tgt) if tgt else None))
            leas={}
        elif mn in ('jmp','ret','je','jne','jb','ja','jbe','jae','jl','jg','jle','jge','js','jns'):
            pass
    return res

if __name__=='__main__':
    allres=[]
    for (st,en,un) in FN:
        if en-st<=0 or en-st>0x200000: continue
        try: r=scan_fn(st,en)
        except Exception: continue
        for site,kind,loc in r: allres.append((st,site,kind,loc))
    pickle.dump(allres, open('locs.pkl','wb'),2)
    print('panic sites with loc:', sum(1 for x in allres if x[3]), '/', len(allres))
    c=collections.Counter(x[3][0] for x in allres if x[3])
    for f,n in c.most_common(80): print('%6d  %s'%(n,f))
