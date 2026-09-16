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
tbl=int(sys.argv[1],16); n=int(sys.argv[2]); width=int(sys.argv[3]) if len(sys.argv)>3 else 4
o=rva2off(tbl-base)
for i in range(n):
    if width==4:
        v=struct.unpack_from('<i',d,o+i*4)[0]; print(i, hex(tbl+v))
    else:
        v=struct.unpack_from('<Q',d,o+i*8)[0]; print(i, hex(v))
