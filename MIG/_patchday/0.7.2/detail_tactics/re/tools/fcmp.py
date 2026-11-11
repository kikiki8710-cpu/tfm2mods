# normalized function compare: python -X utf8 fcmp.py <old_start> <new_start>
import sys, re
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
o=A.Img(r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'); n=A.Img(r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe')
md=Cs(CS_ARCH_X86,CS_MODE_64)
def norm(img,s):
    f=img.owner(s)
    if not f: return None,None
    out=[]
    for x in md.disasm(img.code(f[0],f[1]-f[0]),A.BASE+f[0]):
        t=x.op_str
        t=re.sub(r'rip \+ 0x[0-9a-f]+','rip+X',t)
        if x.mnemonic.startswith('j') or x.mnemonic=='call':
            t=re.sub(r'0x[0-9a-f]+','T',t)
        out.append(x.mnemonic+' '+t)
    return f,out
for a,b in zip(sys.argv[1::2],sys.argv[2::2]):
    fa,oa=norm(o,int(a,16)); fb,ob=norm(n,int(b,16))
    if oa is None or ob is None:
        print(a,b,'no owner'); continue
    same=sum(1 for x,y in zip(oa,ob) if x==y)
    print('%s(%x..%x,%d) vs %s(%x..%x,%d): same-pos %d/%d %s'%(a,fa[0],fa[1],len(oa),b,fb[0],fb[1],len(ob),same,max(len(oa),len(ob)),'IDENT' if oa==ob else ''))