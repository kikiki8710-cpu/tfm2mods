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
def rd8(rva): return struct.unpack_from('<Q',d,rva2off(rva))[0]
base=0x140000000
addr=int(sys.argv[1],16)-base
before=int(sys.argv[2],16) if len(sys.argv)>2 else 0x40
after=int(sys.argv[3],16) if len(sys.argv)>3 else 0x60
for a in range(addr-before,addr+after,8):
    v=rd8(a)
    print(hex(a+base), hex(v), ('RVA=%x'%(v-base)) if base<=v<base+0x5000000 else '')
