# -*- coding: utf-8 -*-
# mig3_053b.py — 콜러가 적어 투표가 약한 훅(DISP/LOADING/EF1EA0/ORACLE/COLLECT)을 콜러문맥으로 정밀 확정.
#   각 타깃: 0.5.2 콜러함수 → 앵커 대응 0.5.3 함수 → 그 함수의 call 타겟을 "0.5.2 콜러가 부르던 순서"와 대조.
#   + dealloc 실체(0.5.2 0x25c4d90) 대응 도출 + 후보 프롤로그 실측.
import collections, re, bisect
import bytepatch_053 as B
import dov_053b as G

A, CO, EO, CN, EN = G.A, G.CO, G.EO, G.CN, G.EN
roff, owner = B.roff, B.owner
RA = {}
for k, v in A.items():
    RA.setdefault(v, k)


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def head(d, secs, r, n=16):
    o = roff(secs, r)
    return d[o:o + n].hex(' ') if o is not None else "?"


def size_of(fns, r):
    f = owner(fns, r)
    return (f[1] - f[0]) if f else 0


def call_seq(d, secs, fns, fn):
    """함수 fn 안의 call 타겟을 등장 순서대로"""
    f = owner(fns, fn)
    if not f:
        return []
    o = roff(secs, f[0])
    blob = d[o:o + (f[1] - f[0])]
    va, vsz, rr, rs = sec(secs, ".text")
    out = []
    i = 0
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > len(blob):
            break
        rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
        site = f[0] + i
        t = site + 5 + rel
        if va <= t < va + rs:
            out.append((site - f[0], t))
        i += 1
    return out


TARGETS = [("DISP_RVA", 0xd3f780), ("LOADING_RVA", 0xd186f0), ("EF1EA0_RVA", 0xe58c30),
           ("ORACLE_RVA", 0x1d94720), ("COLLECT_RVA", 0xd0bd80), ("CT_REGION_LO", 0xe7ccd0),
           ("SLOT_RVA", 0xd1acf0), ("ITEMCONV_RVA", 0xed8770), ("dealloc실체", 0x25c4d90)]

for nm, old in TARGETS:
    print("=" * 100)
    fo = owner(B.FO, old)
    print(f"{nm}  0.5.2=0x{old:x} 크기={size_of(B.FO,old)} 선두16B={head(B.DO,B.SO,old)}")
    print(f"   앵커직접={'0x%x' % A[old] if old in A else '없음'}   콜러함수 {len(CO.get(old,{}))}개")
    seen = collections.Counter()
    for cf, cnt in sorted(CO.get(old, {}).items()):
        nf = A.get(cf)
        # 0.5.2 콜러 안에서 old 가 몇 번째 call 인가
        seq = call_seq(B.DO, B.SO, B.FO, cf)
        idxs = [i for i, (_, t) in enumerate(seq) if t == old]
        line = f"   콜러 0x{cf:<9x}(x{cnt}) 위치 {idxs[:4]}/{len(seq)}call"
        if nf is None:
            print(line + "  → 앵커 대응 없음")
            continue
        nseq = call_seq(B.DN, B.SN, B.FN, nf)
        picks = [nseq[i][1] for i in idxs if i < len(nseq)]
        for p in picks:
            seen[p] += 1
        print(line + f"  → 0.5.3 0x{nf:x}({len(nseq)}call) 같은순번 = " +
              ", ".join(f"0x{p:x}" for p in picks[:4]))
    if seen:
        print("   ▶ 순번대응 집계: " + ", ".join(f"0x{t:x}×{c}" for t, c in seen.most_common(5)))
        for t, c in seen.most_common(3):
            print(f"      0x{t:<9x} 크기={size_of(B.FN,t):<6d}(비 {size_of(B.FN,t)/max(1,size_of(B.FO,old)):.2f}) "
                  f"콜러={sum(CN.get(t,{}).values())} 선두16B={head(B.DN,B.SN,t)}")
    print()
