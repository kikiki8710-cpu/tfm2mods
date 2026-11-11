import os
HERE=os.path.dirname(os.path.abspath(__file__))
import sys, pickle, collections, struct, bisect
sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_OP_REG, X86_REG_RIP, X86_REG_RSP, X86_REG_RBP
OLD=r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'
NEW=r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'
BASE=0x140000000
io_, fo = A.load(OLD,'game-ai')
inn, fn = A.load(NEW,'game-ai')
S=pickle.load(open(os.path.join(HERE,'s1.pkl'),'rb'))
pairs,gone,added,rows=S['pairs'],S['gone'],S['added'],S['rows']
cgo=pickle.load(open(r'F:\claude\tfm2mods\_cg_071.pkl','rb'))
cgn=pickle.load(open(r'F:\claude\tfm2mods\_cg_072.pkl','rb'))
LOCO=None; LOCN=None
def locs():
    global LOCO,LOCN
    if LOCO is None:
        import os
        if os.path.exists(os.path.join(HERE,'locs.pkl')):
            LOCO,LOCN=pickle.load(open(os.path.join(HERE,'locs.pkl'),'rb'))
        else:
            LOCO=A.find_locations(io_); LOCN=A.find_locations(inn)
            pickle.dump((LOCO,LOCN),open(os.path.join(HERE,'locs.pkl'),'wb'))
    return LOCO,LOCN
md=Cs(CS_ARCH_X86, CS_MODE_64); md.detail=True
def fkey(img, F, rva):
    o=img.owner(rva); return o
def short(m): return m.replace('game-ai'+chr(92)+'src'+chr(92),'')
def name_of(F, img, rva):
    k=img.owner(rva)
    if k and k in F: return short(F[k]['top'])
    return None
def dis(img, k):
    b,e=k
    return list(md.disasm(img.code(b,e-b), BASE+b))
def rd(img, rva, n):
    return img.code(rva, n)
def census2(img, k, LOC):
    ins=dis(img,k)
    imms=collections.Counter(); fld=collections.Counter(); vs=collections.Counter(); calls=[]; fl=collections.Counter(); ripd=collections.Counter()
    for i in ins:
        mn=i.mnemonic
        for op in i.operands:
            if op.type==X86_OP_IMM:
                v=op.imm
                if mn in ('call','jmp') or mn.startswith('j'): continue
                if 0x140000000<=v<0x146000000: continue
                if abs(v)>=3: imms[v]+=1
            elif op.type==X86_OP_MEM:
                m=op.mem
                if m.base==X86_REG_RIP:
                    t=i.address+i.size+m.disp-BASE
                    if t in LOC: continue
                    # float consts
                    if mn in ('movss','addss','subss','mulss','divss','comiss','ucomiss','maxss','minss','cvtss2sd'):
                        fl[('f',struct.unpack('<f',rd(img,t,4))[0])]+=1
                    elif mn in ('movsd','addsd','subsd','mulsd','divsd','comisd','ucomisd','maxsd','minsd'):
                        fl[('d',struct.unpack('<d',rd(img,t,8))[0])]+=1
                    else:
                        ripd[t]+=1
                elif m.base not in (X86_REG_RSP, X86_REG_RBP) and m.base!=0:
                    if mn=='call': vs[m.disp]+=1
                    elif m.disp: fld[m.disp]+=1
        if mn=='call' and i.operands and i.operands[0].type==X86_OP_IMM:
            calls.append(i.operands[0].imm-BASE)
    return dict(imms=imms,fld=fld,vs=vs,calls=calls,fl=fl,ripd=ripd,n=len(ins))
def dc(a,b):
    return [(k,a.get(k,0),b.get(k,0)) for k in sorted(set(a)|set(b), key=lambda x:(str(type(x)),x)) if a.get(k,0)!=b.get(k,0)]
import os
HERE=os.path.dirname(os.path.abspath(__file__))
_FULL=None
def full():
    global _FULL
    if _FULL is None:
        _FULL=(pickle.load(open(os.path.join(HERE,'full071.pkl'),'rb')), pickle.load(open(os.path.join(HERE,'full072.pkl'),'rb')))
    return _FULL
