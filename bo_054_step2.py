# -*- coding: utf-8 -*-
from bo_054 import O, N, make_pattern, find, BASE
T=[(0x1c55300,"cont DRAIN"),(0x10a0320,"cont AI1"),(0x10a3c40,"cont AI2"),
   (0x193a940,"cont MATCHUI"),(0x188f360,"cont AI6b"),(0x1890fd0,"cont AI6d")]
for rva,nm in T:
    for nb in (0xa0,0x100,0x180,0x240,0x300):
        pat,mask=make_pattern(O,rva,nb)
        hits=find(N,pat,mask)
        print(f"  {nm:14s} nb={nb:#5x} hits={len(hits)} {[hex(h) for h in hits][:4]}")
        if len(hits)==1: break
