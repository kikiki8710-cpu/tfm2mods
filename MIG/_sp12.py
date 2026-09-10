exec(open('srcmap.py',encoding='utf-8').read())
import collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
def scan(img,rva,tag):
    b,e=img.frange(rva); out=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        ops=ins.operands
        if ins.mnemonic=='mov' and len(ops)==2 and ops[1].type==X86_OP_IMM and 2<=ops[1].imm<=20:
            if ops[0].type==X86_OP_MEM and ins.operands[0].size==8:
                out.append((ins.address-BASE,"MEM8",ops[1].imm,ins.op_str))
    print("== %s %s : %d qword-imm(2..20) stores"%(tag,hex(rva),len(out)))
    c=collections.Counter(x[2] for x in out); print("   hist",sorted(c.items()))
    return out
o7=scan(A,0xe723c0,"0.5.7")
o8=scan(B,0xe4c5c0,"0.5.8")
for x in o7: print("  7 %08x %s"%(x[0],x[3]))
print()
for x in o8: print("  8 %08x %s"%(x[0],x[3]))
