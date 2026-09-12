exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
fns=[("plan3",0xcea1b0),("plan4",0xce30e0),("plan9",0xcf5b90),("plan10",0xe33540),("plan11",0xd1bed0),
     ("plan12",0xd59720),("plan13",0xda9b30),("plan14",0xd1b0e0),("plan15",0xda9ee0),("plan17",0xe14e50)]
for nm,f in fns:
    b,e=A.frange(f); ins=list(md.disasm(A.code(b,e-b),BASE+b))
    print("### %s %s sz=%s"%(nm,hex(f),hex(e-b)))
    for i,x in enumerate(ins):
        ops=x.operands
        if x.mnemonic=='mov' and len(ops)==2 and ops[0].type==X86_OP_MEM and ops[0].mem.disp==0 and ops[0].size==8 and ops[0].mem.index==0:
            src="imm %d"%ops[1].imm if ops[1].type==X86_OP_IMM else x.reg_name(ops[1].reg) if ops[1].type==X86_OP_REG else "?"
            print("   %08x  %s   <= %s"%(x.address-BASE,x.op_str,src))
            if ops[1].type==X86_OP_REG:
                for j in range(max(0,i-8),i):
                    print("        %08x  %s %s"%(ins[j].address-BASE,ins[j].mnemonic,ins[j].op_str))
