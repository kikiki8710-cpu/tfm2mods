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
pd=[s for s in secs if s[0]=='.pdata'][0]
_,va,vs,ro,rs=pd
ents=[]
for i in range(0,vs,12):
    b,e,u=struct.unpack_from('<III',d,ro+i)
    if b==0: break
    ents.append((b,e))
# merge chained entries: unwind info with chained flag -> treat contiguous? just find containing entry, then walk back while previous end==begin
begins=[b for b,e in ents]
base=0x140000000
def find(addr):
    r=addr-base
    i=bisect.bisect_right(begins,r)-1
    if i<0: return None
    b,e=ents[i]
    if not (b<=r<e): return None
    # chained: check unwind info flag
    while True:
        ub=struct.unpack_from('<I',d,ro+ (ents.index((b,e)))*12+8)[0]
        # read unwind info
        uo=None
        for n,sva,svs,sro,srs in secs:
            if sva<=ub<sva+max(svs,srs): uo=sro+(ub-sva)
        flags=d[uo]>>3
        if flags&4:
            cnt=d[uo+2]
            cb=struct.unpack_from('<I',d,uo+4+cnt*2+ ( (cnt&1)*2))[0]
            j=bisect.bisect_right(begins,cb)-1
            b,e=ents[j]
        else: break
    return b,e
for a in sys.argv[1:]:
    r=find(int(a,16))
    print(a,'->', 'func %x..%x size %d'%(r[0],r[1],r[1]-r[0]) if r else None)
