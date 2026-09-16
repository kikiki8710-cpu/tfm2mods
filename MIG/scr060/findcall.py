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
text=[s for s in secs if s[0]=='.text'][0]
_,va,vs,ro,rs=text
t=d[ro:ro+rs]
base=0x140000000
for a in sys.argv[1:]:
    tgt=int(a,16)-base
    hits=[]
    # scan for E8 / E9 rel32 and 8D 0D/15/05... lea rip
    import re
    for i in range(len(t)-5):
        b=t[i]
        if b in (0xE8,0xE9):
            rel=struct.unpack_from('<i',t,i+1)[0]
            if va+i+5+rel==tgt: hits.append(('call' if b==0xE8 else 'jmp', va+i))
        elif b==0x8D and i>0 and t[i-1] in (0x48,0x4C) and (t[i+1]&0xC7)==0x05:
            rel=struct.unpack_from('<i',t,i+2)[0]
            if va+i+6+rel==tgt: hits.append(('lea', va+i-1))
    print(a, [(k,hex(base+x)) for k,x in hits])
