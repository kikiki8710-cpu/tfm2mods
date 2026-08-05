# -*- coding: utf-8 -*-
from bo_054 import O, N, make_pattern, find, BASE
sites = [
 ('AI_SITE1',0x10a04e2),('AI_JOIN1',0x10a05f0),('AI_SITE2',0x10a3cf8),('AI_JOIN2',0x10a3dc9),
 ('AITURN_SITE',0x1828213),('AITURN_JOIN',0x18282fa),('SFX_SITE',0x1c56245),('SFX_END',0x1c56294),
 ('HL_site',0x193b434),('HL_join',0x193b570),
 ('J_cur',0x1c6605d),('J_cur_join',0x1c66323),('J_next',0x1c66374),('J_next_join',0x1c66434),
 ('K_A',0x1c5a0b2),('K_A_join',0x1c5a288),('K_A_done',0x1c5a274),
 ('K_C',0x1c5a9b1),('K_C_join',0x1c5aa67),('K_C_done',0x1c5ab4e),
 ('K_B',0x1c5a5b9),('K_B_join',0x1c5a90b),
 ('L_site',0x1c6fb05),('L_j1',0x1c6fc40),('L_j2',0x1c70508),('L_j3',0x1c6fc3a),
 ('M_site',0x1c5aa99),('M_join',0x1c5ab31),
 ('AI6_1',0x188e206),('AI6_1j',0x188e6b6),('AI6_2',0x188e403),('AI6_3',0x188f79e),('AI6_3j',0x188f8df),
 ('AI6_4',0x18905fc),('AI6_4j',0x189083f),('AI6_5',0x18906e8),('AI6_6',0x1891090),('AI6_6j',0x1891390),
]
out={}
for nm,rva in sites:
    got=None
    for nb in (0x40,0x60,0x80,0xc0,0x120):
        pat,mask=make_pattern(O,rva,nb)
        hits=find(N,pat,mask)
        if len(hits)==1:
            got=(nb,hits[0],'UNIQ'); break
        if len(hits)==0:
            got=(nb,None,'NONE'); break
        got=(nb,hits,'MULTI%d'%len(hits))
    out[nm]=got
    if got[2]=='UNIQ':
        f=N.func_of(got[1])
        print(f"{nm:12s} {rva:#9x} -> {got[1]:#x}  (nb={got[0]:#x}) cont={f and hex(f[0])}")
    else:
        print(f"{nm:12s} {rva:#9x} -> {got[2]} (nb={got[0]:#x}) {got[1] if got[2].startswith('MULTI') else ''}")
