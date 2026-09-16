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
imm=struct.pack('<Q',int(sys.argv[1],16))
i=t.find(imm)
res={}
while i>=0:
    r=va+i
    f=func(r)
    res.setdefault(f,[]).append(r)
    i=t.find(imm,i+1)
for f,l in sorted(res.items(), key=lambda x:(x[0] or (0,0))):
    print('func %s: %s'%(('%x..%x'%f) if f else None, ' '.join('%x'%x for x in l)))
