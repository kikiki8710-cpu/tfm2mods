exec(open('srcmap.py',encoding='utf-8').read())
import collections
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def scan(path,tag):
    img=Img(path); img.prep()
    hits=[]
    for (b,e) in img._fs:
        if e<=b or e-b>0x20000: continue
        code=img.code(b,e-b)
        if not code: continue
        if b'\x68\x07\x00\x00' not in code: continue   # disp32 0x768 little endian
        for ins in md.disasm(code,BASE+b):
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.disp==0x768 and op.mem.base not in (0,X86_REG_RIP):
                    hits.append((b,ins.address-BASE,ins.mnemonic+" "+ins.op_str))
    print("== %s  sites=%d"%(tag,len(hits)))
    c=collections.Counter(h[0] for h in hits)
    for f,n in c.most_common(30):
        print("   fn %s n=%d"%(hex(f),n))
        for h in hits:
            if h[0]==f: print("       %08x %s"%(h[1],h[2]))
scan(E57,"0.5.7")
