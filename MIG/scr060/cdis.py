import struct,sys
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
base=0x140000000
start=int(sys.argv[1],16); end=int(sys.argv[2],16)
md=Cs(CS_ARCH_X86,CS_MODE_64)
code=d[rva2off(start-base):rva2off(end-base)]
strs={}
def strat(addr):
    o=rva2off(addr-base)
    if o is None: return ''
    s=d[o:o+40]
    if all(32<=c<127 for c in s[:8]):
        return ' ; "'+s.split(b'\0')[0][:60].decode('latin1')+'"'
    return ''
for ins in md.disasm(code,start):
    extra=''
    if ins.mnemonic in('lea',) and 'rip' in ins.op_str:
        # compute
        import re
        m=re.search(r'rip ([+-]) 0x([0-9a-f]+)',ins.op_str)
        if m:
            v=ins.address+ins.size+(int(m.group(2),16) if m.group(1)=='+' else -int(m.group(2),16))
            extra=' ; ->%x'%v+strat(v)
    print(("%x: %s %s%s"%(ins.address,ins.mnemonic,ins.op_str,extra)).encode("ascii","replace").decode())
