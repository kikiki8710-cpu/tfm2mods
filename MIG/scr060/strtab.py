import struct,sys
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
a=int(sys.argv[1],16); before=int(sys.argv[2],16); after=int(sys.argv[3],16)
for x in range(a-before,a+after,16):
    o=rva2off(x-base)
    p_,l=struct.unpack_from('<QQ',d,o)
    s=''
    if base<=p_<base+0x5000000 and 0<l<200:
        so=rva2off(p_-base)
        if so: s=d[so:so+l].decode('latin1')
    print(hex(x), hex(p_), hex(l), repr(s))
