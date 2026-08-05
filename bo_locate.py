# -*- coding: utf-8 -*-
"""컨테이너 함수 재핀 — 내부 여러 창의 마스크시그를 투표."""
from bo_054 import O, N, make_pattern, find, BASE
from collections import Counter

def locate(ostart, oend, step=0x80, nb=0x50, maxprobe=80):
    votes=Counter(); detail=[]
    # 명령 경계 확보: 함수 전체 디스어셈 후 일정 간격 명령 주소를 프로브로
    b=O.read(ostart, oend-ostart)
    addrs=[]
    for ins in O.md.disasm(b, BASE+ostart):
        addrs.append(ins.address-BASE)
    if not addrs: return None,[]
    picks = addrs[::max(1,len(addrs)//maxprobe)][:maxprobe]
    for a in picks:
        pat,mask=make_pattern(O,a,nb)
        if len(pat)<nb: continue
        hits=find(N,pat,mask,limit=8)
        for h in hits[:8]:
            f=N.func_of(h)
            if f: votes[f[0]]+=1
            else: votes[('leaf',h)] += 1
        detail.append((a,[hex(x) for x in hits[:4]]))
    return votes, detail
