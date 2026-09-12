exec(open('srcmap.py',encoding='utf-8').read())
import sys
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def dump(p,rva,tag):
    img=Img(p); img.prep()
    b,e=img.frange(rva)
    print("#### %s  %s..%s"%(tag,hex(b),hex(e)))
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        extra=""
        for op in ins.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=ins.address+ins.size+op.mem.disp-BASE
                extra=" ; ->%s %s"%(hex(t),img.sec(t))
        print("%08x  %-9s %s%s"%(ins.address-BASE,ins.mnemonic,ins.op_str,extra))
    print()
dump(E57,0xdb2760,"0.5.7")
dump(E58,0xcaf9f0,"0.5.8")
