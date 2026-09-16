import struct,sys,bisect
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
text=[s for s in secs if s[0]=='.text'][0]
pd=[s for s in secs if s[0]=='.pdata'][0]
ents=[]
for i in range(0,pd[2],12):
    b,e,u=struct.unpack_from('<III',d,pd[3]+i)
    if b==0: break
    ents.append((b,e))
begins=[b for b,e in ents]
def func(r):
    i=bisect.bisect_right(begins,r)-1
    return ents[i] if i>=0 and ents[i][0]<=r<ents[i][1] else None
_,va,vs,ro,rs=text
t=d[ro:ro+rs]
base=0x140000000
import numpy as np
arr=np.frombuffer(t,dtype=np.uint8)
# compute int32 at every offset
views=[np.frombuffer(t[k:len(t)-((len(t)-k)%4)],dtype='<i4') for k in range(4)]
for a in sys.argv[1:]:
    tgt=int(a,16)-base
    hits=[]
    for k in range(4):
        v=views[k]
        idx=np.arange(len(v))*4+k
        for extra in (4,5,6,8):
            m=np.nonzero((va+idx+extra+v.astype(np.int64))==tgt)[0]
            for j in m:
                hits.append((int(idx[j]),extra))
    print(a, [('%x'%(va+h), e, ('%x'%func(va+h)[0]) if func(va+h) else None) for h,e in sorted(hits)])
