# -*- coding: utf-8 -*-
# dov_053b.py — asset-get family 형제(0x99c860 ASSET_GET / 0x5ab7d0 ANIM_GET) 를 콜러-대응 투표로 확정.
#   방법: 0.5.2 에서 형제 F 를 부르는 함수들 → 앵커맵(_anchor_052_053.pkl)으로 0.5.3 대응 함수 →
#         그 함수가 실제로 부르는 family 멤버를 집계. 최다득표 = 대응 형제.
#   검증: 알려진 정답 0x5ac950 → 0x2e1550 이 같은 절차로 재현되는지 먼저 확인.
import re, collections, bisect, pickle
import bytepatch_053 as B

roff, owner = B.roff, B.owner
A = pickle.load(open(r"C:\tfm2mods\_anchor_052_053.pkl", "rb"))
if isinstance(A, dict) and "idx" in A:
    A = A["idx"]
A = {(int(k, 16) if isinstance(k, str) else k): (int(v, 16) if isinstance(v, str) else v)
     for k, v in A.items()}
print(f"앵커맵 {len(A)}쌍")


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def graph(d, secs, fns):
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr + rs]
    starts = [f[0] for f in fns]
    caller = collections.defaultdict(collections.Counter)   # tgt -> Counter(caller_fn)
    callee = collections.defaultdict(collections.Counter)   # fn  -> Counter(tgt)
    i, n = 0, len(blob)
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > n:
            break
        rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
        site = va + i
        t = site + 5 + rel
        if va <= t < va + rs:
            j = bisect.bisect_right(starts, site) - 1
            if j >= 0:
                w = starts[j]
                caller[t][w] += 1
                callee[w][t] += 1
        i += 1
    return caller, callee


print("그래프 구축 중...")
CO, EO = graph(B.DO, B.SO, B.FO)
CN, EN = graph(B.DN, B.SN, B.FN)
print("  완료")

SIG = bytes.fromhex("55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec 98 00 00 00 48 8d ac 24 80".replace(" ", ""))


def entry(d, secs, rva, n=24):
    o = roff(secs, rva)
    return d[o:o + n] if o is not None else b""


FAM_N = {f[0] for f in B.FN if entry(B.DN, B.SN, f[0]) == SIG}
FAM_O = {f[0] for f in B.FO if entry(B.DO, B.SO, f[0]) == SIG}
print(f"family: 0.5.2 {len(FAM_O)}개 / 0.5.3 {len(FAM_N)}개\n")

TARGETS = [("검증 LOADER(정답 0x2e1550)", 0x5ac950),
           ("illust RVA_ASSET_GET", 0x99c860),
           ("illust RVA_ANIM_GET", 0x5ab7d0)]

for nm, f in TARGETS:
    print("=" * 92)
    print(f"{nm}  0.5.2 = 0x{f:x}   콜러함수 {len(CO[f])}개")
    mapped = 0
    votes = collections.Counter()
    detail = collections.defaultdict(list)
    for cf, cnt in CO[f].items():
        if cf not in A:
            continue
        mapped += 1
        nf = A[cf]
        for t, k in EN.get(nf, {}).items():
            if t in FAM_N:
                votes[t] += 1
                detail[t].append((cf, nf, cnt, k))
    print(f"  앵커맵 매핑된 콜러 {mapped}개 → family 호출 집계:")
    for t, c in votes.most_common(8):
        # 같은 횟수로 부르는지까지 본 엄격 득표
        strict = sum(1 for (_, _, a, b) in detail[t] if a == b)
        print(f"     0x{t:<9x} 득표={c:<4d} (동일횟수 {strict})  전체콜러={sum(CN[t].values())}")
    print()
