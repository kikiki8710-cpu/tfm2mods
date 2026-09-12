exec(open('srcmap.py',encoding='utf-8').read())
import struct,pickle,os,re
BS=bytes([92])
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
PAT=re.compile(b'[a-z][a-z0-9_-]{2,20}'+BS+b'src'+BS+b'[A-Za-z0-9_'+BS+b'.]{2,80}'+BS+b'[a-z0-9_]{2,40}[.]rs')
def build2(path,cache):
    if os.path.exists(cache):
        img=Img(path); img.prep()
        strs,ptr2str=pickle.load(open(cache,'rb'))
        return img,strs,ptr2str
    img=Img(path); img.prep(); d=img.data
    strs={}
    for m in PAT.finditer(d):
        off=m.start()
        for nm,va,sz,pr in img.secs:
            if pr<=off<pr+sz: strs[va+(off-pr)]=m.group(0).decode('latin1'); break
    tgt={BASE+r:t for r,t in strs.items()}
    ptr2str={}
    for nm,va,sz,pr in img.secs:
        if nm not in ('.rdata','.data'): continue
        blob=d[pr:pr+sz]
        for i in range(0,len(blob)-8,8):
            v=struct.unpack_from('<Q',blob,i)[0]
            if v in tgt: ptr2str[va+i]=tgt[v]
    pickle.dump((strs,ptr2str),open(cache,'wb'))
    return img,strs,ptr2str
A=build2(E57,'_s57.pkl'); B=build2(E58,'_s58.pkl')
print("57 strs",len(A[1]),"ptr",len(A[2]))
print("58 strs",len(B[1]),"ptr",len(B[2]))
pairs=[(3,0xcea1b0,0xd2c5d0),(4,0xce30e0,0xd781e0),(9,0xcf5b90,0xdfdfc0),(10,0xe33540,0xdb8ba0),
(11,0xd1bed0,0xdf1c80),(12,0xd59720,0xdefcd0),(13,0xda9b30,0xccc010),(14,0xd1b0e0,0xdf0e90),
(15,0xda9ee0,0xccc3c0),(17,0xe14e50,0xd2da10)]
def name(pack,rva):
    img,strs,p2s=pack
    r=analyze(img,strs,p2s,rva)
    if r[0] is None: return "??",None
    (b,e),hits=r
    top=hits.most_common(4)
    return "sz=%s %s"%(hex(e-b),top)
for tag,r7,r8 in pairs:
    print("== Plan tag %d"%tag)
    print("   0.5.7 %s : %s"%(hex(r7),name(A,r7)))
    print("   0.5.8 %s : %s"%(hex(r8),name(B,r8)))
