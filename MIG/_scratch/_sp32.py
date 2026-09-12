exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
b,e=A.frange(0xe723c0)
ins=list(md.disasm(A.code(b,e-b),BASE+b))
idx={x.address-BASE:i for i,x in enumerate(ins)}
for site in (0xe7b53a,0xe76de4,0xe736ee,0xe78e88):
    i=idx[site]
    print("### around %08x"%site)
    for j in range(max(0,i-6),min(len(ins),i+22)):
        print("   %08x  %-9s %s"%(ins[j].address-BASE,ins[j].mnemonic,ins[j].op_str))
    print()
