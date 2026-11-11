from w7lib import *
import hashlib
def norm_body(im,e,n=40):
    toks=[]
    for i in dis(im,e,0x200):
        toks.append(i.mnemonic)
        if i.mnemonic in ('ret',) or (i.mnemonic=='jmp' and i.operands[0].type!=X86_OP_IMM) or len(toks)>=n: break
    return ' '.join(toks)
def short(im,e,n=6):
    out=[]
    for i in dis(im,e,0x80):
        out.append(i.mnemonic+' '+i.op_str)
        if i.mnemonic in ('ret',) or len(out)>=n: break
    return ' ; '.join(out)
def find_vt(w, champ):
    FN=pickle.load(open(r'F:\claude\tfm2mods\MIG\_patchday\0.7.2\aiwide\tools\full0%s.pkl'%('70' if w=='n' else '65'),'rb'))
    V=pickle.load(open('vtall_%s.pkl'%w,'rb')); im=img(w)
    res=[]
    for vt,s,a,ms in V:
        if len(ms)<34: continue
        for e in ms:
            k=im.owner(e)
            if k in FN and FN[k]['top'].endswith(champ): res.append((vt,s,ms)); break
    return res
if __name__=='__main__':
    champ=sys.argv[1]
    O=find_vt('o',champ); N=find_vt('n',champ)
    print('old',[(hex(v),hex(s),len(m)) for v,s,m in O]); print('new',[(hex(v),hex(s),len(m)) for v,s,m in N])
    io=img('o'); inn=img('n')
    for (vo,so,mo),(vn,sn,mn) in zip(O,N):
        print('=== old %#x  new %#x'%(vo,vn))
        L=max(len(mo),len(mn))
        for j in range(L):
            a=mo[j] if j<len(mo) else None; b=mn[j] if j<len(mn) else None
            print('+%#05x  o=%-9s %-50s | n=%-9s %s'%(0x18+8*j, hex(a) if a else '-', short(io,a,3)[:50] if a else '', hex(b) if b else '-', short(inn,b,3)[:60] if b else ''))
