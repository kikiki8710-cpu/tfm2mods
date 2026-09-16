import struct,sys,bisect,re
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
lo=int(sys.argv[2],16); hi=int(sys.argv[3],16)
for a in sys.argv[1].split(','):
    disp=struct.pack('<i',int(a,16))
    # patterns: C6 modrm disp32 imm8 (mov byte [r+d],imm) ; 88 modrm disp32 (mov [r+d],r8) ; 80 modrm(/7 cmp, /0 add.. ) disp32 imm8 ; 0F B6 modrm disp32 (movzx) ; 38/3A cmp
    res={}
    i=t.find(disp)
    while i>=0:
        r=va+i
        if lo<=r<hi:
            op=t[i-2:i]
            modrm=t[i-1]
            if (modrm&0xC0)==0x80:
                opc=t[i-2]
                kind=None
                if opc==0xC6: kind='mov8 imm'
                elif opc==0x88: kind='mov8 store'
                elif opc==0x80: kind='op8 imm(/%d)'%((modrm>>3)&7)
                elif opc==0xB6 and t[i-3]==0x0F: kind='movzx8'
                elif opc==0x38 or opc==0x3A: kind='cmp8'
                elif opc==0xF6: kind='test8'
                elif opc==0x8A: kind='mov8 load'
                if kind:
                    f=func(r)
                    res.setdefault(f,[]).append((r,kind))
        i=t.find(disp,i+1)
    print('== off',a)
    for f,l in sorted(res.items(), key=lambda x:x[0] or (0,0)):
        print('  func %s: %s'%(('%x'%f[0]) if f else None, ' '.join('%x:%s'%(r-2,k) for r,k in l)))
