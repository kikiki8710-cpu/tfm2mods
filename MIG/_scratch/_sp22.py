exec(open('srcmap.py',encoding='utf-8').read())
import collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
BAD={X86_REG_RBP,X86_REG_RSP,0,X86_REG_RIP}
def scan(path,tag):
    img=Img(path); img.prep(); res=collections.defaultdict(list)
    for (b,e) in img._fs:
        if e<=b or e-b>0x20000: continue
        code=img.code(b,e-b)
        if not code or b'\x68\x07\x00\x00' not in code: continue
        for ins in md.disasm(code,BASE+b):
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.disp==0x768 and op.mem.base not in BAD:
                    res[b].append((ins.address-BASE,ins.mnemonic+" "+ins.op_str))
    print("== %s fns=%d"%(tag,len(res)))
    for f,v in sorted(res.items()):
        print("   fn %s sz=%s"%(hex(f),hex(img.frange(f)[1]-f)))
        for a,s in v: print("       %08x %s"%(a,s))
scan(E57,"0.5.7")
