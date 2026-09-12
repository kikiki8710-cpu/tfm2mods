exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
# find 'movd dword ptr [reg+0x41], xmm' sites
for (b,e) in A._fs:
    if e<=b or e-b>0x20000: continue
    code=A.code(b,e-b)
    if not code: continue
    if b'\x66\x0f\x7e' not in code: continue
    for ins in md.disasm(code,BASE+b):
        if ins.mnemonic in ('movd','movq') and 'ptr [' in ins.op_str:
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.disp in (0x41,0x39,0x45):
                    print("fn %s @%08x  %s %s"%(hex(b),ins.address-BASE,ins.mnemonic,ins.op_str))
