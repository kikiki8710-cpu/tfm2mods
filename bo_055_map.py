# -*- coding: utf-8 -*-
"""사이트 명령단위 정렬 → 0.5.4→0.5.5 매핑 + orig 바이트 대조."""
from bo_055 import O, N, BASE
from bo_align_055 import align, ctx

# 확정 컨테이너 (0.5.4 start, 0.5.4 end, 0.5.5 start)
CONT = {
 'AI1':    (0x149e380,0x149ef6c, 0x12acaf0),
 'AI2':    (0x14a1e60,0x14a2585, 0x12b0480),
 'AITURN': (0x211dd40,0x211f2b3, 0x21b6170),
 'DRAIN':  (0x1e19640,0x1e388df, 0x19e88e0),
 'MATCHUI':(0x237c030,0x238761b, 0x1c532b0),
 'AI6a':   (0x215e050,0x215f377, 0x1cb8640),
 'AI6b':   (0x215f680,0x216042f, 0x1cb9dd0),
 'AI6c':   (0x2160680,0x2161077, 0x1cbb1a0),
 'AI6d':   (0x2161200,0x2161b5e, 0x1cbbdc0),
}
SITES = {
 'AI1':[('AI_SITE1',0x149e561),('AI_JOIN1',0x149e680)],
 'AI2':[('AI_SITE2',0x14a1f1e),('AI_JOIN2',0x14a1fef)],
 'AITURN':[('AITURN_SITE',0x211e14f),('AITURN_JOIN',0x211e236)],
 'DRAIN':[('SFX_SITE',0x1e1a575),('SFX_END',0x1e1a5c4),
          ('J_cur',0x1e2a37d),('J_cur_join',0x1e2a643),('J_next',0x1e2a694),('J_next_join',0x1e2a754),
          ('K_A',0x1e1e3b2),('K_A_join',0x1e1e588),('K_A_done',0x1e1e574),
          ('K_B',0x1e1e8c0),('K_B_join',0x1e1ec12),
          ('K_C',0x1e1ecb8),('K_C_join',0x1e1ed6e),('K_C_done',0x1e1ee55),
          ('L_site',0x1e33aeb),('L_j1_T1',0x1e33c26),('L_j2_done',0x1e3453a),('L_j3_T2',0x1e33c20),
          ('M_site',0x1e1eda0),('M_join',0x1e1ee38)],
 'MATCHUI':[('HL_site',0x237cb14),('HL_join',0x237cc50)],
 'AI6a':[('AI6_1',0x215e526),('AI6_1join',0x215e9d6),('AI6_2',0x215e723)],
 'AI6b':[('AI6_3',0x215fac8),('AI6_3join',0x215fc10)],
 'AI6c':[('AI6_4',0x216082c),('AI6_4join',0x2160a6f),('AI6_5',0x2160918)],
 'AI6d':[('AI6_6',0x21612c3),('AI6_6join',0x21615b3)],
}
def run():
    res={}
    for k,(os_,oe,ns_) in CONT.items():
        f=N.func_of(ns_); ne=f[1] if f else ns_+(oe-os_)
        m,oi,ni=align(os_,oe,ns_,ne)
        print(f"### {k}: {os_:#x}-{oe:#x} -> {ns_:#x}-{ne:#x} matched {len(m)}/{len(oi)} ({len(m)/len(oi):.3f})")
        for nm,rva in SITES[k]:
            nv=m.get(rva)
            ob=O.read(rva,12).hex(); nb=N.read(nv,12).hex() if nv else '-'
            same='SAME' if (nv and ob==nb) else 'DIFF'
            print(f"   {nm:12s} {rva:#9x} -> {hex(nv) if nv else 'UNMAPPED':>10s}  {same}")
            print(f"        old={ob}")
            if nv: print(f"        new={nb}")
            res[nm]=nv
    return res
if __name__=='__main__':
    run()
