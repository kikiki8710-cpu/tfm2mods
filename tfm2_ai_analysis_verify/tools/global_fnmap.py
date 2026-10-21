"""0.6.4 → 0.6.5 전역 함수 대응(분석 아틀라스 밖 포함) — 호출 대상 비교용 (2026-10-21).

씨앗 = aidiff 대응 + 아틀라스 대응(atlas_064_065.csv) + 전 함수 골격 해시 유일 짝 + 머리 해시 유일 짝.
그다음 짝지어진 함수들의 호출 순서를 정렬해 같은 자리의 미매핑 피호출자끼리 짝(크기 ±35%)을 반복 전파한다.
출력: <migration_065>/fnmap_global_065.json {구 rva(hex): 신 rva(hex)}
사용: python -B -X utf8 tools/global_fnmap.py <migration_065 폴더>
"""
import collections, csv, difflib, json, os, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
from _x065 import O, N, PO, PN, calls_in  # noqa: E402

mig = sys.argv[1]
pair = {}
am = json.load(open(os.path.join(mig, "aidiff_map.json"), encoding="utf-8"))
for k, (n, v, _) in am["map"].items():
    pair[int(k, 16)] = int(n, 16)
for r in csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")):
    if r["new"]:
        pair[int(r["old"], 16)] = int(r["new"], 16)
seed = len(pair)
for key in ("skel", "head"):
    bo, bn = collections.defaultdict(list), collections.defaultdict(list)
    for a, e in PO.items():
        bo[e[key]].append(a)
    for a, e in PN.items():
        bn[e[key]].append(a)
    used = set(pair.values())
    for h, os_ in bo.items():
        if len(os_) == 1 and len(bn.get(h, [])) == 1 and os_[0] not in pair and bn[h][0] not in used:
            pair[os_[0]] = bn[h][0]; used.add(bn[h][0])
hashed = len(pair)
cache = {}


def calls(img, idx, f):
    k = (id(img), f)
    if k not in cache:
        cache[k] = [t for _, t in calls_in(img, f, int(idx[f]["size"]))] if f in idx else []
    return cache[k]


for _round in range(10):
    added = 0
    inv = {v: k for k, v in pair.items()}
    for o, n in list(pair.items()):
        A, B = calls(O, PO, o), calls(N, PN, n)
        if not A or not B:
            continue
        ka = [hex(pair[t]) if t in pair else "?" for t in A]
        kb = [hex(t) if t in inv else "?" for t in B]
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(a=ka, b=kb, autojunk=False).get_opcodes():
            if tag in ("equal", "replace") and i2 - i1 == j2 - j1:
                for x, y in zip(A[i1:i2], B[j1:j2]):
                    if x in pair or y in inv or x not in PO or y not in PN:
                        continue
                    so, sn = PO[x]["size"], PN[y]["size"]
                    if abs(so - sn) <= 0.35 * max(so, sn):
                        pair[x] = y; inv[y] = x; added += 1
    if not added:
        break
json.dump({hex(k): hex(v) for k, v in pair.items()}, open(os.path.join(mig, "fnmap_global_065.json"), "w", encoding="utf-8"))
print(f"씨앗 {seed} → 해시 {hashed} → 전파 후 {len(pair)} / 구 함수 {len(PO)}")
