# -*- coding: utf-8 -*-
from bo_align import align, ctx
from bo_054 import O,N
CONT = {
 'AI1':   (0x10a0320,0x10a0df5, 0x149e380),
 'AI2':   (0x10a3c40,0x10a42ce, 0x14a1e60),
 'AITURN':(0x1827e00,0x18292e1, 0x211dd40),
 'DRAIN': (0x1c55300,0x1c749be, 0x1e19640),
 'MATCHUI':(0x193a940,0x19460a2, 0x237c030),
 'AI6a':  (0x188dd30,0x188f057, 0x215e050),
 'AI6b':  (0x188f360,0x1890162, 0x215f680),
 'AI6c':  (0x1890450,0x1890e47, 0x2160680),
 'AI6d':  (0x1890fd0,0x18919a8, 0x2161200),
}
SITES = {
 'AI1':[('AI_SITE1',0x10a04e2),('AI_JOIN1',0x10a05f0)],
 'AI2':[('AI_SITE2',0x10a3cf8),('AI_JOIN2',0x10a3dc9)],
 'AITURN':[('AITURN_SITE',0x1828213),('AITURN_JOIN',0x18282fa)],
 'DRAIN':[('SFX_SITE',0x1c56245),('SFX_END',0x1c56294),
          ('J_cur',0x1c6605d),('J_cur_join',0x1c66323),('J_next',0x1c66374),('J_next_join',0x1c66434),
          ('K_A',0x1c5a0b2),('K_A_join',0x1c5a288),('K_A_done',0x1c5a274),
          ('K_B',0x1c5a5b9),('K_B_join',0x1c5a90b),
          ('K_C',0x1c5a9b1),('K_C_join',0x1c5aa67),('K_C_done',0x1c5ab4e),
          ('L_site',0x1c6fb05),('L_j1_T1',0x1c6fc40),('L_j2_done',0x1c70508),('L_j3_T2',0x1c6fc3a),
          ('M_site',0x1c5aa99),('M_join',0x1c5ab31)],
 'MATCHUI':[('HL_site',0x193b434),('HL_join',0x193b570)],
 'AI6a':[('AI6_1',0x188e206),('AI6_1join',0x188e6b6),('AI6_2',0x188e403)],
 'AI6b':[('AI6_3',0x188f79e),('AI6_3join',0x188f8df)],
 'AI6c':[('AI6_4',0x18905fc),('AI6_4join',0x189083f),('AI6_5',0x18906e8)],
 'AI6d':[('AI6_6',0x1891090),('AI6_6join',0x1891390)],
}
MAPS={}
def run():
    res={}
    for k,(os_,oe,ns_) in CONT.items():
        f=N.func_of(ns_); ne = f[1] if f else ns_+ (oe-os_)
        m,oi,ni = align(os_,oe,ns_,ne)
        MAPS[k]=m
        print(f"### {k}: old {os_:#x}-{oe:#x} -> new {ns_:#x}-{ne:#x} matched {len(m)}/{len(oi)} ({len(m)/len(oi):.3f})")
        for nm,rva in SITES[k]:
            nv=m.get(rva)
            ob=O.read(rva,10).hex(); nb=N.read(nv,10).hex() if nv else '-'
            same = 'SAME' if (nv and ob==nb) else 'DIFF'
            print(f"   {nm:12s} {rva:#9x} -> {hex(nv) if nv else 'UNMAPPED':>10s}  {same}  old={ob} new={nb}")
            res[nm]=nv
    return res
if __name__=='__main__':
    run()
