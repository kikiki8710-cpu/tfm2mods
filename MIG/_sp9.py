exec(open('srcmap.py',encoding='utf-8').read())
import difflib
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def lines(img,rva):
    b,e=img.frange(rva); out=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        s=ins.op_str
        s=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',s); s=re.sub(r'rip - 0x[0-9a-f]+','rip+X',s)
        s=re.sub(r'\b0x14[0-9a-f]{6,7}\b','ABS',s)
        out.append((ins.address-BASE,ins.mnemonic+" "+s))
    return b,e,out
b7,e7,L7=lines(A,0xe723c0); b8,e8,L8=lines(B,0xe4c5c0)
print("ins",len(L7),len(L8))
k7=[x[1] for x in L7]; k8=[x[1] for x in L8]
# further normalize: strip immediates for alignment key
def key(s): return re.sub(r'\b0x[0-9a-f]+\b','I',s)
n7=[key(x) for x in k7]; n8=[key(x) for x in k8]
sm=difflib.SequenceMatcher(None,n7,n8,autojunk=False)
ops=sm.get_opcodes()
print("opcodes",len(ops))
for tag,i1,i2,j1,j2 in ops:
    if tag=='equal':
        # find immediate-only differences
        for a,b in zip(range(i1,i2),range(j1,j2)):
            if k7[a]!=k8[b]:
                print("  IMM  %08x  %-50s || %-50s"%(L7[a][0],k7[a],k8[b]))
    else:
        print("== %s 0.5.7[%d:%d] @%08x  0.5.8[%d:%d] @%08x"%(tag,i1,i2,L7[i1][0] if i1<len(L7) else 0,j1,j2,L8[j1][0] if j1<len(L8) else 0))
        for a in range(i1,min(i2,i1+30)): print("   -%08x %s"%(L7[a][0],k7[a]))
        for b in range(j1,min(j2,j1+30)): print("   +%08x %s"%(L8[b][0],k8[b]))
