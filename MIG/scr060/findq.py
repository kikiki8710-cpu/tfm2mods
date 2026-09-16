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
def off2rva(o):
    for n,va,vs,ro,rs in secs:
        if ro<=o<ro+rs: return va+(o-ro),n
for a in sys.argv[1:]:
    v=int(a,16)
    pat=struct.pack('<Q',v)
    i=d.find(pat)
    while i>=0:
        r=off2rva(i)
        print(a, '->', hex(r[0]+0x140000000) if r else '?', r[1] if r else '')
        i=d.find(pat,i+1)
