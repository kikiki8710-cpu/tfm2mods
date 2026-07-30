import pickle, struct, sys
from pe import load, rva2off, off2rva
from fn import func_of
X = pickle.load(open('xref.pkl','rb'))
data, IB, SECTS = load()
calls, jmps, drefs = X['calls'], X['jmps'], X['drefs']

def find_str(b):
    res=[]; i=0
    while True:
        i = data.find(b, i)
        if i<0: break
        r = off2rva(SECTS, i)
        if r: res.append((i,r))
        i+=1
    return res

def callers(rva, lim=200):
    return calls.get(rva,[])[:lim]

def dref(rva, lim=200):
    return drefs.get(rva,[])[:lim]

if __name__=='__main__':
    for pat in [b'index out of bounds: the len is ', b'called `Option::unwrap()` on a `None` value',
                b'range end index ', b'slice index starts at ', b'attempt to add with overflow',
                b'attempt to subtract with overflow']:
        for off,r in find_str(pat)[:3]:
            print(pat[:30], 'off=0x%x rva=0x%x'%(off,r), 'drefs=', [( '0x%x'%a, m) for a,m in dref(r)][:20])
