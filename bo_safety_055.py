# -*- coding: utf-8 -*-
"""패치 구간에 착지하는 외부 분기가 없는지 검사(0.5.5). 컨테이너는 func_of 로 자동."""
from bo_055 import N, BASE
from capstone.x86 import X86_OP_IMM
def inbound(site, length):
    f=N.func_of(site); cs,ce=(f if f else (site-0x2000, site+0x2000))
    bad=[]; b=N.read(cs,ce-cs)
    for ins in N.md.disasm(b,BASE+cs):
        a=ins.address-BASE
        for op in ins.operands:
            if op.type==X86_OP_IMM and ins.mnemonic[0]=='j':
                t=op.imm-BASE
                if site < t < site+length: bad.append((hex(a),hex(t)))
    return bad, (hex(cs),hex(ce)) if f else 'no-func'
CASES=[
 ('G aiturn',0x21b657f,38),('SFX',0x19e9825,0x4f),
 ('J_cur',0x19f963d,14),('J_next',0x19f9954,14),
 ('K_A',0x19ed692,14),('K_B',0x19edb99,14),('K_C',0x19edf91,14),
 ('M',0x19ee079,14),('L',0x1a02dab,14),('HL',0x1c53d8b,14),
 ('AI1(site1)',0x12accb2,35),('AI2(site2)',0x12b0538,40),
 ('AI6_1',0x1cb8b72,14),('AI6_2',0x1cb8d60,14),('AI6_3',0x1cba2a5,14),
 ('AI6_4',0x1cbb38d,14),('AI6_5',0x1cbb479,14),('AI6_6',0x1cbbeea,14),
]
bad_total=0
for nm,s,l in CASES:
    b,cont=inbound(s,l)
    if b: bad_total+=1
    print(f"{nm:12s} site={s:#x} len={l:#x} cont={cont} inbound={b if b else 'NONE(OK)'}")
print("\nBAD windows =", bad_total)
