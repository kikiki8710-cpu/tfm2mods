exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def head(img,fn,n,tag):
    b,e=img.frange(fn); print("### %s %s"%(tag,hex(fn)))
    for i,ins in enumerate(md.disasm(img.code(b,min(e-b,400)),BASE+b)):
        if i>=n: break
        print("  %08x  %-9s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
head(A,0xcbd450,28,"0.5.7 sp6 line_wait")
head(A,0xcb4b40,28,"0.5.7 sp10 death_battle")
head(B,0xcc4260,20,"0.5.8 sp6 jungle")
head(A,0xd52470,20,"0.5.7 sp8 jungle")
# dispatcher arm stubs 0.5.7: does arm for sp6 add rdx,8 ?
b,e=A.frange(0xcbf340)
for t,lbl in ((0xcbf4af,"0.5.7 arm sp6 stub"),(0xcbf70c,"0.5.7 arm sp10 stub"),(0xcbf6a8,"0.5.7 arm sp8 stub")):
    print("### %s"%lbl)
    for ins in md.disasm(A.code(t,60),BASE+t):
        print("  %08x  %-9s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
        if ins.mnemonic=='call': break
