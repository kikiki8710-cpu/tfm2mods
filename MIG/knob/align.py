# -*- coding: utf-8 -*-
exec(open(r'C:\tfm2mods\MIG\knob\ownmap.py',encoding='utf-8').read())
import difflib,io,sys
def stream(img,fn):
    pb,ins=fn_insns(img,fn)
    return ins
def toks(ins): return [norm(i) for i in ins]
def idx_of(ins,addr):
    for k,i in enumerate(ins):
        if i.address-BASE==addr: return k
    return None
def align(insA,insB):
    ta,tb=toks(insA),toks(insB)
    sm=difflib.SequenceMatcher(None,ta,tb,autojunk=False)
    return sm,ta,tb
def map_idx(sm,i):
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if i1<=i<i2:
            if tag=='equal': return 'equal',j1+(i-i1)
            return tag,(j1,j2)
    return 'oob',None
def winsearch(insA,ia,insB,K=6):
    """search unique window match around index ia"""
    ta,tb=toks(insA),toks(insB)
    for k in range(K,1,-1):
        pat=ta[ia-k:ia+k+1]
        hits=[j for j in range(k,len(tb)-k) if tb[j-k:j+k+1]==pat]
        if len(hits)==1: return hits[0],k,1
        if len(hits)>1: return hits,k,len(hits)
    return None,0,0
def dump(img,ins,j,n=4):
    out=[]
    for k in range(max(0,j-n),min(len(ins),j+n+1)):
        i=ins[k]
        out.append('%s%08x %-22s %-8s %s'%('>' if k==j else ' ',i.address-BASE,i.bytes.hex(),i.mnemonic,i.op_str))
    return '\n'.join(out)
