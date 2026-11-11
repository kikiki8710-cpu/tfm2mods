import sys, os, pickle, hashlib, collections
T=r'F:\claude\tfm2mods\MIG\_patchday\0.7.2\aiwide\tools'
sys.path.insert(0,T)
from common import *
HERE2=os.path.dirname(os.path.abspath(__file__))
a,b=full()
pf,gf,af=A.pair(a,b)
G={k[0]:v[0][0] for k,v in pf.items()}
print('loc pairs',len(G),flush=True)
m=Cs(CS_ARCH_X86,CS_MODE_64); m.detail=True
def mh(img,f):
    s,e=f
    if e-s>20000: return None
    code=bytearray(img.code(s,e-s))
    for i in m.disasm(bytes(code),BASE+s):
        o=i.address-BASE-s
        if i.disp_size and any(op.type==3 and op.mem.base==X86_REG_RIP for op in i.operands):
            for q in range(i.disp_offset,i.disp_offset+i.disp_size): code[o+q]=0
        if (i.mnemonic=='call' or i.mnemonic.startswith('j')) and i.imm_size and i.operands and i.operands[0].type==2:
            for q in range(i.imm_offset,i.imm_offset+i.imm_size): code[o+q]=0
        if i.imm_size and i.operands:
            for op in i.operands:
                if op.type==2 and 0x140000000<=op.imm<0x146000000:
                    for q in range(i.imm_offset,i.imm_offset+i.imm_size): code[o+q]=0
    return hashlib.md5(bytes(code)).hexdigest()
def hm(img):
    d=collections.defaultdict(list)
    for f in img.funcs:
        h=mh(img,f)
        if h: d[h].append(f[0])
    return d
ho=hm(io_); print('old hashed',flush=True); hn=hm(inn)
n=0
for h,lo in ho.items():
    ln=hn.get(h)
    if ln and len(lo)==1 and len(ln)==1 and lo[0] not in G:
        G[lo[0]]=ln[0]; n+=1
print('hash pairs',n,'total',len(G),'funcs',len(io_.funcs),len(inn.funcs))
pickle.dump(dict(G=G,ho=ho,hn=hn),open(os.path.join(HERE2,'gmap.pkl'),'wb'))
