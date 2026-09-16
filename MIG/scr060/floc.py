import struct,sys,bisect,re
from capstone import *
p=r'C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.0\TeamfightManager2.exe'
d=open(p,'rb').read()
pe=struct.unpack_from('<I',d,0x3c)[0]
nsec=struct.unpack_from('<H',d,pe+6)[0]
opt=struct.unpack_from('<H',d,pe+20)[0]
secs=[]
off=pe+24+opt
for i in range(nsec):
    name=d[off:off+8].rstrip(b'\0').decode()
    vs,va,rs,ro=struct.unpack_from('<IIII',d,off+8)
    secs.append((name,va,vs,ro,rs)); off+=40
def rva2off(r):
    for n,va,vs,ro,rs in secs:
        if va<=r<va+max(vs,rs): return ro+(r-va)
pd=[s for s in secs if s[0]=='.pdata'][0]
ents=[]
for i in range(0,pd[2],12):
    b,e,u=struct.unpack_from('<III',d,pd[3]+i)
    if b==0: break
    ents.append((b,e))
begins=[b for b,e in ents]
base=0x140000000
def func(r):
    i=bisect.bisect_right(begins,r)-1
    return ents[i] if i>=0 and ents[i][0]<=r<ents[i][1] else None
def loc(rva):
    o=rva2off(rva)
    if o is None: return None
    try:
        p_,l,line,col=struct.unpack_from('<QQII',d,o)
    except: return None
    if not(base<=p_<base+0x5000000) or not (5<l<200): return None
    so=rva2off(p_-base)
    if so is None: return None
    s=d[so:so+l]
    if b'src' not in s or b'.rs' not in s: return None
    return s.decode('latin1'),line,col
md=Cs(CS_ARCH_X86,CS_MODE_64)
for a in sys.argv[1:]:
    r=int(a,16)-base
    f=func(r)
    if not f: print(a,'no func'); continue
    b,e=f
    code=d[rva2off(b):rva2off(e)]
    locs={}
    calls=set()
    for ins in md.disasm(code,b):
        if ins.mnemonic=='lea' and 'rip' in ins.op_str:
            m=re.search(r'rip ([+-]) 0x([0-9a-f]+)',ins.op_str)
            v=ins.address+ins.size+(int(m.group(2),16) if m.group(1)=='+' else -int(m.group(2),16))
            L=loc(v)
            if L: locs.setdefault((L[0].split('src')[-1],L[1]),0); locs[(L[0].split('src')[-1],L[1])]+=1
        elif ins.mnemonic=='call' and ins.op_str.startswith('0x'):
            calls.add(int(ins.op_str,16))
    print('== %s func %x..%x size %d'%(a,b,e,e-b))
    for k in sorted(locs): print('   ',k)
    print('   calls:',' '.join('%x'%c for c in sorted(calls)))
