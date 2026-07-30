# -*- coding: utf-8 -*-
# dov_053c.py — 형제 매핑 역방향 확증 + callee 지문 대조.
#   ①역방향: 0.5.3 후보의 콜러 → 앵커맵 역인덱스로 0.5.2 대응 → 어느 0.5.2 형제를 부르나 집계
#   ②callee: 각 형제가 부르는 함수 집합을 앵커맵으로 옮겨 교집합 비율
import collections, bisect, pickle
import bytepatch_053 as B
import dov_053b as G      # 그래프·앵커맵 재사용 (CO/EO/CN/EN/A/FAM_N/FAM_O)

A, CO, EO, CN, EN = G.A, G.CO, G.EO, G.CN, G.EN
RA = {}
for k, v in A.items():
    RA.setdefault(v, k)

PAIRS = [("LOADER(정답검증)", 0x5ac950, [0x2e1550]),
         ("RVA_ASSET_GET", 0x99c860, [0x143d50, 0x888fd0]),
         ("RVA_ANIM_GET", 0x5ab7d0, [0x888fd0, 0x143d50])]

print("\n" + "=" * 92)
print("① 역방향 투표 (0.5.3 후보의 콜러 → 0.5.2 대응 → 부르는 0.5.2 형제)")
print("=" * 92)
for nm, old, cands in PAIRS:
    print(f"{nm}  0.5.2=0x{old:x}")
    for c in cands:
        votes = collections.Counter()
        mapped = 0
        for cf, cnt in CN[c].items():
            of = RA.get(cf)
            if of is None:
                continue
            mapped += 1
            for t, k in EO.get(of, {}).items():
                if t in G.FAM_O:
                    votes[t] += 1
        top = votes.most_common(3)
        hit = votes.get(old, 0)
        tot = sum(votes.values()) or 1
        print(f"   후보 0x{c:<9x} 역매핑콜러={mapped:<4d} → 0x{old:x} 득표 {hit} "
              f"({hit*100//max(1,mapped)}%)  상위: " +
              ", ".join(f"0x{t:x}×{v}" for t, v in top))
    print()

print("=" * 92)
print("② callee 지문 (형제가 부르는 함수들의 앵커 대응 일치율)")
print("=" * 92)
for nm, old, cands in PAIRS:
    oc = {t for t in EO.get(old, {})}
    om = {A[t] for t in oc if t in A}
    print(f"{nm} 0.5.2 callee {len(oc)}개(앵커대응 {len(om)}개)")
    for c in cands:
        nc = set(EN.get(c, {}))
        inter = om & nc
        print(f"   후보 0x{c:<9x} callee {len(nc)}개 → 교집합 {len(inter)} "
              f"({len(inter)*100//max(1,len(om))}%)  {[hex(x) for x in sorted(inter)][:6]}")
    print()
