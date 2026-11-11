import sys,os,pickle,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from semlib import *
X=pickle.load(open(os.path.join(HERE2,'sweep2.pkl'),'rb'))
T=pickle.load(open(os.path.join(HERE2,'triage.pkl'),'rb'))
def uses_fp(img,k):
    ins=dis(img,k)[:12]
    return any(i.mnemonic in('mov','lea') and i.op_str.startswith('rbp,') for i in ins)
def rv(img,k):
    c=collections.Counter()
    for i in dis(img,k):
        if i.mnemonic in('call','jmp') and i.operands and i.operands[0].type==X86_OP_MEM:
            m=i.operands[0].mem
            if m.base==X86_REG_RBP or m.base==X86_REG_RSP: c[('rbp' if m.base==X86_REG_RBP else 'rsp',m.disp)]+=1
    return c
sets=[('R',k,v) for k,v in T['auto-noise']+T['auto-C120']]+[('H',k,v) for k,v in T['H-auto-noise']+T['H-auto-C120']]
hit=0
for tag,ko,kn in sets:
    fo_=uses_fp(io_,ko); fn_=uses_fp(inn,kn)
    a=rv(io_,ko); b=rv(inn,kn)
    if not fo_ and not fn_:
        A_={d for (r,d) in a.elements() if r=='rbp'}; B_={d for (r,d) in b.elements() if r=='rbp'}
        aa=collections.Counter(d for (r,d) in a.elements() if r=='rbp'); bb=collections.Counter(d for (r,d) in b.elements() if r=='rbp')
        if aa!=bb:
            # allow champion-trait shift: +0x120 add, >=0x130 shift +0x10
            rm=aa-bb; ad=bb-aa
            for s in list(rm):
                if ad.get(s+0x10,0)>=rm[s]: ad[s+0x10]-=rm[s]; rm[s]=0
            ad[0x120]=0
            rm=+rm; ad=+ad
            if rm or ad:
                hit+=1; print(tag,'%#x->%#x'%(ko[0],kn[0]),'rm',{hex(k):v for k,v in rm.items()},'ad',{hex(k):v for k,v in ad.items()})
print('checked',len(sets),'flagged',hit)
