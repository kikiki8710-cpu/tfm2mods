# -*- coding: utf-8 -*-
"""패치 구간에 착지하는 외부 분기가 없는지 검사(0.5.4)."""
from bo_054 import N,BASE
from capstone.x86 import X86_OP_IMM
def inbound(cstart,cend,site,length):
    bad=[]
    b=N.read(cstart,cend-cstart)
    for ins in N.md.disasm(b,BASE+cstart):
        a=ins.address-BASE
        for op in ins.operands:
            if op.type==X86_OP_IMM and ins.mnemonic[0]=='j':
                t=op.imm-BASE
                if site < t < site+length:
                    bad.append((hex(a),hex(t)))
    return bad
CASES=[
 ('G aiturn',0x211dd40,0x211f2b3,0x211e14f,38),
 ('SFX',0x1e19640,0x1e388df,0x1e1a575,0x4f),
 ('J_cur',0x1e19640,0x1e388df,0x1e2a37d,14),
 ('J_next',0x1e19640,0x1e388df,0x1e2a694,14),
 ('K_A',0x1e19640,0x1e388df,0x1e1e3b2,14),
 ('K_B',0x1e19640,0x1e388df,0x1e1e8c0,14),
 ('K_C',0x1e19640,0x1e388df,0x1e1ecb8,14),
 ('M',0x1e19640,0x1e388df,0x1e1eda0,14),
 ('L',0x1e19640,0x1e388df,0x1e33aeb,14),
 ('HL',0x237c030,0x238761b,0x237cb14,14),
 ('AI1',0x149e380,0x149ef6c,0x149e561,64),
 ('AI2',0x14a1e60,0x14a2585,0x14a1f1e,54),
 ('AI6_1',0x215e050,0x215f377,0x215e526,14),
 ('AI6_2',0x215e050,0x215f377,0x215e723,14),
 ('AI6_3',0x215f680,0x216042f,0x215fac8,14),
 ('AI6_4',0x2160680,0x2161077,0x216082c,14),
 ('AI6_5',0x2160680,0x2161077,0x2160918,14),
 ('AI6_6',0x2161200,0x2161b5e,0x21612c3,14),
]
for nm,cs,ce,s,l in CASES:
    b=inbound(cs,ce,s,l)
    print(f"{nm:10s} site={s:#x} len={l:#x} inbound={b if b else 'NONE(OK)'}")
