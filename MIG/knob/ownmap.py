# -*- coding: utf-8 -*-
exec(open('lib.py',encoding='utf-8').read())
import struct,collections,pickle,os,time
def ownmap(img,strs,p2s,cache):
    if os.path.exists(cache): return pickle.load(open(cache,'rb'))
    tgt={}
    for r,t in strs.items(): tgt[r]=t
    for r,t in p2s.items(): tgt.setdefault(r,t)
    d=img.data; out=collections.defaultdict(collections.Counter)
    t0=time.time()
    for nm,va,sz,pr in img.secs:
        if nm!='.text': continue
        for i in range(0,sz-4):
            disp=struct.unpack_from('<i',d,pr+i)[0]
            t=va+i+4+disp
            if t in tgt:
                fr=img.fparts(va+i)
                if fr[0] is not None: out[fr[0]][tgt[t]]+=1
    print('ownmap %.1fs fns=%d'%(time.time()-t0,len(out)))
    out=dict(out); pickle.dump(out,open(cache,'wb')); return out
