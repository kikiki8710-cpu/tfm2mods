exec(open('srcmap.py',encoding='utf-8').read())
import struct,pickle,os,re,collections
B1=bytes([92]); B2=bytes([92,92])
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
PAT=re.compile(b'[a-z][a-z0-9_-]{2,20}'+B2+b'src'+B2+b'[A-Za-z0-9_.'+B2+b']{2,90}[.]rs')
def build2(path,cache):
    img=Img(path); img.prep()
    if os.path.exists(cache):
        strs,ptr2str=pickle.load(open(cache,'rb')); return img,strs,ptr2str
    d=img.data; strs={}
    for m in PAT.finditer(d):
        off=m.start()
        for nm,va,sz,pr in img.secs:
            if pr<=off<pr+sz: strs[va+(off-pr)]=m.group(0).decode('latin1'); break
    tgt={BASE+r:t for r,t in strs.items()}; ptr2str={}
    for nm,va,sz,pr in img.secs:
        if nm not in ('.rdata','.data'): continue
        blob=d[pr:pr+sz]
        for i in range(0,len(blob)-8,8):
            v=struct.unpack_from('<Q',blob,i)[0]
            if v in tgt: ptr2str[va+i]=tgt[v]
    pickle.dump((strs,ptr2str),open(cache,'wb')); return img,strs,ptr2str
A=build2(E57,'_s57c.pkl'); B=build2(E58,'_s58c.pkl')
print("57 strs",len(A[1]),"ptr",len(A[2]),"| 58 strs",len(B[1]),"ptr",len(B[2]))
def nm(pack,rva):
    img,strs,p2s=pack
    r=analyze(img,strs,p2s,rva)
    if r[0] is None: return "??"
    (b,e),hits=r
    return "%s %s"%(hex(e-b),hits.most_common(3))
tab=[("movepri",0xdb2760,0xcaf9f0),("arm1 in3",0xcea1b0,0xd2c5d0),("arm2 in4",0xce30e0,0xd781e0),
 ("arm5 in7",0xe49a70,0xd2e500),("arm7 in9",0xcf5b90,0xdfdfc0),("arm8 in10",0xe33540,0xdb8ba0),("arm9 in11",0xd1bed0,0xdf1c80),
 ("arm10 in12",0xd59720,0xdefcd0),("arm11 in13",0xda9b30,0xccc010),("arm12 in14",0xd1b0e0,0xdf0e90),("arm13 in15",0xda9ee0,0xccc3c0),
 ("arm15 in17",0xe14e50,0xd2da10),("AItick",0xe723c0,0xe4c5c0),("AItick2",0xe713d0,0xe4b5d0),("exec",0xe8b800,0xe65b10),
 ("subdisp",0xcbf340,0xe35bd0),("w768=7",0xe71820,None),("sp6 linewait57",0xcbd450,None),("sp8 jungle57",0xd52470,None)]
for lbl,r7,r8 in tab:
    print("== %s"%lbl)
    print("   57 %s : %s"%(hex(r7),nm(A,r7)))
    if r8: print("   58 %s : %s"%(hex(r8),nm(B,r8)))
print("== deleted handlers 0.5.7")
for r in (0xde48f0,0xde2470):
    print("   57 %s : %s"%(hex(r),nm(A,r)))
