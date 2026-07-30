import struct, bisect
from pe import load, rva2off, off2rva

data, IB, SECTS = load()
_p = [s for s in SECTS if s['name']=='.pdata'][0]
N = _p['vsize']//12
FN=[]  # (start, end, unwind)
for i in range(N):
    o = _p['rawptr']+i*12
    st,en,un = struct.unpack_from('<III', data, o)
    if st==0 and en==0: continue
    FN.append((st,en,un))
FN.sort()
STARTS=[f[0] for f in FN]

def func_of(rva):
    i = bisect.bisect_right(STARTS, rva)-1
    if i<0: return None
    st,en,un = FN[i]
    if st<=rva<en: return (st,en)
    return None

if __name__=='__main__':
    print('funcs', len(FN))
    print(func_of(0x11e2140), func_of(0x11d8ef0), func_of(0xd5f240), func_of(0xd60757))
