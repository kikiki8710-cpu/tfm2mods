exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def win(img,fn,lo,hi,tag):
    b,e=img.frange(fn)
    print("### %s %s  +%x..+%x"%(tag,hex(fn),lo,hi))
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        off=ins.address-BASE-b
        if lo<=off<=hi:
            print("  +%04x %08x  %-9s %s"%(off,ins.address-BASE,ins.mnemonic,ins.op_str))
win(A,0xcea1b0,0x440,0x4b0,"0.5.7 plan3")
win(B,0xd2c5d0,0x400,0x470,"0.5.8 plan3")
