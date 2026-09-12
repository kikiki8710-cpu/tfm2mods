exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def lines(img,rva):
    b,e=img.frange(rva); out=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        s=ins.op_str
        # normalize rip-rel and absolute targets
        s=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',s)
        s=re.sub(r'\b0x14[0-9a-f]{6,7}\b','ABS',s)
        out.append((ins.address-b-BASE,ins.mnemonic,s,ins.bytes.hex()))
    return b,e,out
pairs=[(3,0xcea1b0,0xd2c5d0),(4,0xce30e0,0xd781e0),(9,0xcf5b90,0xdfdfc0),(10,0xe33540,0xdb8ba0),
(11,0xd1bed0,0xdf1c80),(12,0xd59720,0xdefcd0),(13,0xda9b30,0xccc010),(14,0xd1b0e0,0xdf0e90),
(15,0xda9ee0,0xccc3c0),(17,0xe14e50,0xd2da10)]
for tag,r7,r8 in pairs:
    b7,e7,L7=lines(A,r7); b8,e8,L8=lines(B,r8)
    diffs=[]
    if len(L7)==len(L8):
        for x,y in zip(L7,L8):
            if x[1:3]!=y[1:3]: diffs.append((x,y))
    else:
        diffs=[("LENDIFF",len(L7),len(L8))]
    print("== Plan %d  %s -> %s  ndiff=%d (ins %d/%d)"%(tag,hex(r7),hex(r8),len(diffs),len(L7),len(L8)))
    for d in diffs[:40]:
        if d[0]=="LENDIFF": print("   ",d)
        else: print("   +%04x  %s %s   ||   %s %s"%(d[0][0],d[0][1],d[0][2],d[1][1],d[1][2]))
