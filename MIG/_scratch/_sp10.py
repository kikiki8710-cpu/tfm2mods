exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
b,e=A.frange(0xcbf340)
print(hex(b),hex(e))
for ins in md.disasm(A.code(b,e-b),BASE+b):
    extra=""
    for op in ins.operands:
        if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
            t=ins.address+ins.size+op.mem.disp-BASE
            extra=" ; ->%s %s"%(hex(t),A.sec(t))
    print("%08x  %-9s %s%s"%(ins.address-BASE,ins.mnemonic,ins.op_str,extra))
