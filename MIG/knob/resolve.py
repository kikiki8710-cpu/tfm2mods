# -*- coding: utf-8 -*-
exec(open(r'C:\tfm2mods\MIG\knob\align.py',encoding='utf-8').read())
import struct
def winsearch2(ta,ia,tb,K=10,lo=2):
    res=[]
    for k in range(K,lo-1,-1):
        if ia-k<0 or ia+k+1>len(ta): continue
        pat=ta[ia-k:ia+k+1]
        hits=[j for j in range(k,len(tb)-k) if tb[j-k:j+k+1]==pat]
        res.append((k,hits))
        if len(hits)==1: return k,hits
    for k,h in res:
        if h: return k,h
    return 0,[]
def bytescan(img,fn,pat):
    fr=img.frange(fn); out=[]
    blob=img.code(fr[0],fr[1]-fr[0])
    i=blob.find(pat)
    while i>=0:
        out.append(fr[0]+i); i=blob.find(pat,i+1)
    return out
