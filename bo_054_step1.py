# -*- coding: utf-8 -*-
from bo_054 import O, N, make_pattern, find, BASE
TARGETS = [
 (0x1bf3dd0,"A' PHASE_SCENE"),(0x167c0e0,"B PHASE_SCALAR"),(0x1bd8c20,"C APPLIER"),
 (0x1c252c0,"O SLOTUPD"),(0x1bce8e0,"N PHASE_RAW"),
 (0x1bc47f0,"APP_PICK_T1"),(0x1bc4980,"APP_PICK_T2"),(0x1c028d0,"APP_BAN_T1"),(0x1c02a50,"APP_BAN_T2"),
 (0x1bcf010,"TRANSITION"),(0x1bd63a0,"BANNER"),(0x1bc52b0,"E LINEUP"),(0x167fdd0,"F COMMIT"),
 (0x1680500,"D' TURN"),(0x1bf77d0,"TRIGGER"),(0x28f2f34,"PANIC_HOOK"),
 # containers
 (0x10a0320,"cont AI1"),(0x10a3c40,"cont AI2"),(0x1827e00,"cont AITURN"),(0x1c55300,"cont DRAIN"),
 (0x193a940,"cont MATCHUI"),(0x188dd30,"cont AI6a"),(0x188f360,"cont AI6b"),(0x1890450,"cont AI6c"),(0x1890fd0,"cont AI6d"),
]
for rva,nm in TARGETS:
    res=[]
    for nb in (0xa0,0x60,0x140):
        pat,mask=make_pattern(O,rva,nb)
        hits=find(N,pat,mask)
        res.append((nb,hits))
        if len(hits)==1: break
    nb,hits=res[-1]
    f=O.func_of(rva); sz=(f[1]-f[0]) if f else 0
    tag="OK" if len(hits)==1 else ("MULTI" if len(hits)>1 else "NONE")
    newsz=""
    if len(hits)==1:
        g=N.func_of(hits[0])
        newsz=f" size {sz:#x}->{(g[1]-g[0]) if g else 0:#x} start_match={g and g[0]==hits[0]}"
    print(f"{nm:16s} {rva:#9x} nb={nb:#5x} -> {[hex(h) for h in hits][:5]} {tag}{newsz}")
