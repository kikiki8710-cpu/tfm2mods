exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def win(img,fn,lo,hi,tag):
    b,e=img.frange(fn)
    print("### %s"%tag)
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        a=ins.address-BASE
        if lo<=a<=hi: print("  %08x  %-9s %s"%(a,ins.mnemonic,ins.op_str))
win(A,0xe723c0,0xe78ec0,0xe79010,"0.5.7 @e78ec0")
print()
win(B,0xe4c5c0,0xe53240,0xe53390,"0.5.8 @e53240")
