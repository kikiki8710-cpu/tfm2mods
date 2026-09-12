exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def win(img,fn,lo,hi,tag):
    b,e=img.frange(fn); print("### %s"%tag)
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        a=ins.address-BASE
        if lo<=a<=hi: print("  %08x  %-9s %s"%(a,ins.mnemonic,ins.op_str))
# 0.5.7 plan3 handler: [rsi]=7 @cea1cf, [rsi]=6 @cea204
win(A,0xcea1b0,0xcea1b0,0xcea240,"0.5.7 plan3 head (7 then 6)")
print()
win(B,0xd2c5d0,0xd2c5d0,0xd2c660,"0.5.8 plan3 head (5 then 4)")
