# -*- coding: utf-8 -*-
"""픽 순서 테이블 rip-rel 참조 지문으로 컨테이너 20:20 대응 확정(0.5.4->0.5.5)."""
from bo_055 import O, N, BASE
from capstone.x86 import X86_OP_MEM, X86_REG_RIP

# PICK_TABLES 28B 연접(소스 상수). rdata 에 이 바이트열이 있다.
PT = bytes([0,1,0,1, 0,1,1,0,0,1, 0,1,1,0,1,0,0,1, 0,1,1,0,0,1,1,0,0,1])

def find_table(E):
    hits=[]
    for nm,va,vs,pr,sr in E.secs:
        if nm not in ('.rdata','.data'): continue
        blob=E.data[pr:pr+sr]; s=0
        while True:
            i=blob.find(PT,s)
            if i<0: break
            hits.append(va+i); s=i+1
    return hits

def riprel_refs(E, tbl_lo, tbl_hi):
    """텍스트 전체에서 tbl_lo..tbl_hi 범위를 가리키는 rip-rel lea/mov 참조를 수집."""
    text,va=E.text_blob()
    refs=[]
    # capstone 전수는 느리므로 함수단위로 훑되, 여기선 pdata 함수별로.
    for (s,e) in E.pdata():
        b=E.read(s, min(e-s, 0x4000))
        if not b: continue
        for ins in E.md.disasm(b, BASE+s):
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                    tgt = ins.address + ins.size + op.mem.disp - BASE
                    if tbl_lo <= tgt < tbl_hi:
                        refs.append((ins.address-BASE, s, tgt))
    return refs

if __name__=='__main__':
    for tag,E in (('0.5.4',O),('0.5.5',N)):
        t=find_table(E)
        print(f"{tag} pick table @", [hex(x) for x in t])
