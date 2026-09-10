exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
def ctx(fn,site,n=10):
    b,e=A.frange(fn); ins=list(md.disasm(A.code(b,e-b),BASE+b))
    idx={x.address-BASE:i for i,x in enumerate(ins)}
    i=idx[site]
    print("### %s"%hex(site))
    for j in range(max(0,i-3),min(len(ins),i+n)):
        print("   %08x  %-9s %s"%(ins[j].address-BASE,ins[j].mnemonic,ins[j].op_str))
for s in (0xe8b8d4,0xe8c7f5,0xe8cab0):
    ctx(0xe8b800,s,8)
ctx(0xe03630,0xe0364d,10)
print()
# what does 0xcc21e0 do
b,e=A.frange(0xcc21e0); print("### 0xcc21e0 sz=%s"%hex(e-b))
for ins in md.disasm(A.code(b,min(e-b,200)),BASE+b):
    print("   %08x  %-9s %s"%(ins.address-BASE,ins.mnemonic,ins.op_str))
