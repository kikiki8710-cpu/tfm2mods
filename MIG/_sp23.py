exec(open('srcmap.py',encoding='utf-8').read())
import difflib
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
A=Img(E57);A.prep(); B=Img(E58);B.prep()
h7={2:0xd476b0,3:0xde48f0,4:0xeab850,5:0xde2470,6:0xcbd450,7:0xcbcfb0,8:0xd52470,9:0xea3270,10:0xcb4b40,
    11:0xcb9fc0,12:0xd41230,13:0xd53d60,14:0xdebe50,15:0xd50b60,16:0xde6880,17:0xd42b20,18:0xe9fd70,19:0xeae620,20:0xea1ab0}
h8={2:0xe83390,3:0xccacf0,4:0xeb43a0,5:0xcc5ca0,6:0xcc4260,7:0xcbbdb0,8:0xea9a20,9:0xcb7540,10:0xcb03b0,
    11:0xeaeda0,12:0xcc6170,13:0xe8b5e0,14:0xcb1ce0,15:0xe7caf0,16:0xe81680,17:0xe928f0,18:0xcba660}
def toks(img,r):
    b,e=img.frange(r); o=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        o.append(ins.mnemonic+" "+re.sub(r'\b0x[0-9a-f]+\b','I',re.sub(r'rip [-+] 0x[0-9a-f]+','rip+X',ins.op_str)))
    return (e-b),o
print("0.5.7 sp -> sizes / 0.5.8 sp-2 sizes / similarity")
for sp,r in sorted(h7.items()):
    s7,t7=toks(A,r)
    tgt=sp-2
    if tgt in h8:
        s8,t8=toks(B,h8[tgt])
        rt=difflib.SequenceMatcher(None,t7,t8,autojunk=False).ratio()
        print("  0.5.7 sp%-2d %s sz=%-7s  <->  0.5.8 sp%-2d %s sz=%-7s  sim=%.3f"%(sp,hex(r),hex(s7),tgt,hex(h8[tgt]),hex(s8),rt))
    else:
        print("  0.5.7 sp%-2d %s sz=%-7s  <->  (none)"%(sp,hex(r),hex(s7)))
