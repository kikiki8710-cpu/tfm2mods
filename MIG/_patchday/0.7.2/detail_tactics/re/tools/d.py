# usage: python -X utf8 d.py <old|new> <start_hex> <end_hex|+count>  [--fn]
import sys, os, pickle
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_REG_RIP
HERE=os.path.dirname(os.path.abspath(__file__))
P={'old':r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe','new':r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'}
def load(v):
    img=A.Img(P[v])
    cp=os.path.join(HERE,'loc_%s.pkl'%v)
    if os.path.exists(cp): loc=pickle.load(open(cp,'rb'))
    else:
        loc=A.find_locations(img); pickle.dump(loc,open(cp,'wb'))
    return img,loc
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
def dis(v,s,e,mark=()):
    img,loc=load(v)
    for x in md.disasm(img.code(s,e-s),A.BASE+s):
        r=x.address-A.BASE; note=''
        for op in x.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                tg=x.address+x.size+op.mem.disp-A.BASE
                L=loc.get(tg)
                if L: note='   ; LOC %s:%d:%d'%(L[0].replace(chr(92),'/').split('/')[-1],L[1],L[2])
                else: note='   ; [%x]'%tg
        if x.mnemonic=='call' and x.operands[0].type!=X86_OP_MEM:
            pass
        m='▶' if r in mark else ' '
        print('%s %8x  %-24s %s %s%s'%(m,r,x.bytes.hex(),x.mnemonic,x.op_str,note))
if __name__=='__main__':
    v=sys.argv[1]; s=int(sys.argv[2],16); e=sys.argv[3]
    e = s+int(e[1:],16) if e.startswith('+') else int(e,16)
    marks=set(int(x,16) for x in sys.argv[4:])
    dis(v,s,e,marks)
