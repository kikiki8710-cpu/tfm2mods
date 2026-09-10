exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
def win(img,fn,lo,hi,tag):
    b,e=img.frange(fn)
    print("### %s"%tag)
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        a=ins.address-BASE
        if lo<=a<=hi: print("  %08x  %-9s %s"%(a,ins.mnemonic,ins.op_str))
win(A,0xe8b800,0xe8c3c0,0xe8c4a0,"0.5.7 dispatcher caller @e8c3c0")
