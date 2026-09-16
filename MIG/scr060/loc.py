import struct,sys,re
p=sys.argv[2] if len(sys.argv)>2 else r"C:SERSJUNGSDESKTOPAUDE	FM2	FM2_0.6.0TEAMFIGHTMANAGER2.EXE"
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
def loc(a):
    o=rva2off(a-base)
    p_,l,line,col=struct.unpack_from('<QQII',d,o)
    s=d[rva2off(p_-base):rva2off(p_-base)+l].decode('latin1')
    return s,line,col
src=open(sys.argv[1],encoding='utf-8').read() if len(sys.argv)>1 else sys.stdin.read()
seen=set()
out=[]
for m in re.findall(r'PTR_s_[A-Za-z_0-9]+_(14[0-9a-f]{7})',src):
    a=int(m,16)
    if a in seen: continue
    seen.add(a)
    try:
        s,line,col=loc(a)
        out.append((s.split('src')[-1],line,col,m))
    except Exception as e: out.append(('?',0,0,m))
for o in sorted(out,key=lambda x:(x[0],x[1])): print(o)
