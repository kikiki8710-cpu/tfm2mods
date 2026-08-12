# -*- coding: utf-8 -*-
"""픽테이블 rip-rel 참조를 .text 전역 raw 스캔으로 수집(빠름) → 컨테이너별 집계."""
import struct, bisect, collections
from bo_055 import O, N, BASE

T54=0x32cf40a; T55=0x33b1ad0

def scan_refs(E, tbl_lo, tbl_hi):
    """모든 파일오프셋 i에서 int32(text[i:i+4]) 를 disp32 로 보고
       명령끝 = va(i+4) 로 가정, va(i+4)+disp ∈ [lo,hi) 이면 참조로 수집.
       (rip-rel disp 필드가 명령의 마지막 4바이트라는 사실 이용 — 오탐률 무시가능)"""
    nm,va,vs,pr,sr = E.text
    text = E.data[pr:pr+sr]
    refs=[]
    n=len(text)-4
    for i in range(n):
        disp = int.from_bytes(text[i:i+4], 'little', signed=True)
        # 작은 disp 는 대부분 무관 — 테이블은 텍스트에서 멀리(+수백MB아니지만 rdata)
        tgt = va + i + 4 + disp
        if tbl_lo <= tgt < tbl_hi:
            refs.append((va+i, tgt))   # va+i = disp 필드 시작 rva
    return refs

def group_by_func(E, refs):
    pd=E.pdata(); starts=[s for s,e in pd]
    byfn=collections.defaultdict(list)
    for dfield, tgt in refs:
        idx=bisect.bisect_right(starts, dfield)-1
        fn = pd[idx][0] if (idx>=0 and pd[idx][0]<=dfield<pd[idx][1]) else None
        byfn[fn].append((dfield,tgt))
    return byfn

if __name__=='__main__':
    for tag,E,lo in (('0.5.4',O,T54),('0.5.5',N,T55)):
        refs=scan_refs(E,lo,lo+28)
        byfn=group_by_func(E,refs)
        print(f"=== {tag} total refs {len(refs)}  containers {len(byfn)} ===")
        for fn in sorted(k for k in byfn if k is not None):
            print(f"  {fn:#x}: {len(byfn[fn])}")
        if None in byfn:
            print(f"  (no-func): {len(byfn[None])}")
