import pickle, struct
from pe import load, rva2off, off2rva
from fn import func_of
X=pickle.load(open('xref.pkl','rb')); drefs=X['drefs']; calls=X['calls']
data, IB, S = load()

def ptr_sites(va):
    b = struct.pack('<Q', va); res=[]; i=0
    while True:
        i = data.find(b,i)
        if i<0: break
        r = off2rva(S,i)
        if r: res.append(r)
        i+=1
    return res

def find_str(b):
    res=[];i=0
    while True:
        i=data.find(b,i)
        if i<0: break
        r=off2rva(S,i)
        if r: res.append(r)
        i+=1
    return res

for pat in [b'index out of bounds: the len is ', b'range end index ', b'slice index starts at ',
            b'called `Option::unwrap()` on a `None` value', b'attempt to add with overflow']:
    for r in find_str(pat)[:2]:
        ps = ptr_sites(IB+r)
        print('---', pat[:34], 'strRVA=0x%x'%r, 'ptrsites=', ['0x%x'%p for p in ps][:10])
        for p in ps[:6]:
            ds = drefs.get(p, [])
            for a,m in ds[:8]:
                f=func_of(a)
                print('     ptr@0x%x  ref 0x%x %s  in FN 0x%x'%(p,a,m,f[0] if f else 0))
