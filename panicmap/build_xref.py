import re, struct, pickle, time, sys
from capstone import *
from pe import load, rva2off
from fn import FN

data, IB, SECTS = load()
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail=False
t=[s for s in SECTS if s['name']=='.text'][0]
RIP = re.compile(r'\[rip ([+-]) (0x[0-9a-f]+)\]')

calls={}   # target -> [sites]
jmps={}
drefs={}   # target -> [(site,mnem)]
t0=time.time()
cnt=0
for (st,en,un) in FN:
    o = rva2off(SECTS, st)
    if o is None: continue
    n = en-st
    if n<=0 or n>0x200000: continue
    code = data[o:o+n]
    for (addr, size, mn, op) in md.disasm_lite(code, st):
        if mn=='call' or mn=='jmp':
            if op.startswith('0x'):
                tgt=int(op,16)
                (calls if mn=='call' else jmps).setdefault(tgt,[]).append(addr)
                continue
        m = RIP.search(op)
        if m:
            d = int(m.group(2),16)
            if m.group(1)=='-': d=-d
            tgt = addr+size+d
            drefs.setdefault(tgt,[]).append((addr,mn))
    cnt+=1
    if cnt%20000==0: print(cnt, '%.1fs'%(time.time()-t0), file=sys.stderr)
print('done %.1fs'%(time.time()-t0), file=sys.stderr)
pickle.dump(dict(calls=calls,jmps=jmps,drefs=drefs), open('xref.pkl','wb'), 2)
print('calls',len(calls),'drefs',len(drefs))
