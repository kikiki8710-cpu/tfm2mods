"""분석 아틀라스(0.6.4) 함수 → 새 버전 대응·변화 판정 (2026-10-21, 0.6.4→0.6.5).

1) aidiff(Rust 패닉 Location) 판정이 있으면 그대로: IDENTICAL / SHIFTED / EDITED / MERGED / DELETED.
2) 없으면(Location 없는 보조 함수) 함수 골격 해시(_fnidx skel)가 구·신 양쪽에서 유일하면 짝.
3) 그래도 없으면 이미 짝지어진 호출자의 '호출 순서'를 정렬해 같은 자리의 미매핑 피호출자끼리 짝(크기 ±35%).
짝이 생기면 즉치(주소가 아닌 상수) 다중집합을 비교해 상수 변화 수를 센다.
사용: python -B -X utf8 tools/atlas_migrate.py <aidiff_map.json> <출력 폴더>
"""
import collections, csv, difflib, json, os, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns, calls_in  # noqa: E402  (0.6.4/0.6.5 로더·함수 지문)
from capstone.x86 import X86_OP_IMM  # noqa: E402
_x065.cs.detail = True  # 즉치 비교에 피연산자 정보가 필요하다

ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\function_atlas_064_20261018\functions.csv"
amap_p, out = sys.argv[1:3]
am = json.load(open(amap_p, encoding="utf-8"))
fs = list(csv.DictReader(open(ATLAS, encoding="utf-8-sig")))
olds = [int(r["RVA(0.6.4)"], 16) for r in fs]

pair, how = {}, {}
for r in fs:
    o = int(r["RVA(0.6.4)"], 16)
    k = hex(o)
    if k in am["map"]:
        n, v, _ = am["map"][k]
        pair[o] = int(n, 16); how[o] = "aidiff:" + v
    elif k in am["deleted"]:
        how[o] = "aidiff:DELETED"

# 1-a) aidiff 짝 교차 교정: 같은 소스 파일 안에서 두 짝을 맞바꾸면 명령 분포가 훨씬 잘 맞는 경우(0.6.5 tower_discipline
#      0xeee590↔0xeee820 가 서로 뒤바뀌어 있었다 — Location 이 둘 다 같은 행 묶음이라 생기는 오대응).
def _sim(a, b):
    x, y = a["mnem"], b["mnem"]
    ks = set(x) | set(y)
    inter = sum(min(x.get(k, 0), y.get(k, 0)) for k in ks); uni = sum(max(x.get(k, 0), y.get(k, 0)) for k in ks)
    return inter / uni if uni else 0


_byfile = collections.defaultdict(list)
for k, (n, v, src) in am["map"].items():
    o = int(k, 16)
    if o in pair and o in PO and pair[o] in PN:
        _byfile[src].append(o)
for src, os_ in _byfile.items():
    for i, a in enumerate(os_):
        for b in os_[i + 1:]:
            na, nb = pair[a], pair[b]
            cur = _sim(PO[a], PN[na]) + _sim(PO[b], PN[nb])
            alt = _sim(PO[a], PN[nb]) + _sim(PO[b], PN[na])
            if alt > cur + 0.2:
                pair[a], pair[b] = nb, na
                how[a] = how[a] + "(교차교정)"; how[b] = how[b] + "(교차교정)"

# 1-b) 파일 이름만 바뀐 함수: aidiff 는 소스 파일이 달라지면 사라짐+새로생김으로 낸다(0.6.5 chat.rs→signal.rs).
#      사라진 함수와 크기가 같은 '새로 생긴 함수'가 하나뿐이면 짝(골격 해시는 경로 문자열 탓에 달라질 수 있다).
_md = os.path.join(os.path.dirname(amap_p), "aidiff_064_065.md")
_new = []
if os.path.exists(_md):
    _sec = open(_md, encoding="utf-8").read().split("## 새로 생긴 함수")[-1].split("\n## ")[0]
    _new = [int(x, 16) for x in __import__("re").findall(r"`(0x[0-9a-f]+)`", _sec)]
for r in fs:
    o = int(r["RVA(0.6.4)"], 16)
    if how.get(o) == "aidiff:DELETED" and o in PO:
        c = [n for n in _new if n in PN and PN[n]["size"] == PO[o]["size"] and n not in pair.values()]
        if len(c) == 1:
            pair[o] = c[0]; how[o] = "aidiff:RENAMED"

# 2) 골격 해시 유일 짝
skel_o = collections.defaultdict(list); skel_n = collections.defaultdict(list)
for a, e in PO.items():
    skel_o[e["skel"]].append(a)
for a, e in PN.items():
    skel_n[e["skel"]].append(a)
used_new = set(pair.values())
for o in olds:
    if o in how or o not in PO:
        continue
    s = PO[o]["skel"]
    if len(skel_o[s]) == 1 and len(skel_n[s]) == 1 and skel_n[s][0] not in used_new:
        pair[o] = skel_n[s][0]; how[o] = "skel"; used_new.add(pair[o])

# 3) 호출 순서 전파 (전 함수 대상 — 아틀라스 함수에 닿을 때까지 반복)
want = {o for o in olds if o not in how}
calls_cache = {}


def ordered_calls(img, idx, f):
    key = (id(img), f)
    if key not in calls_cache:
        calls_cache[key] = [t for _, t in calls_in(img, f, int(idx[f]["size"]))] if f in idx else []
    return calls_cache[key]


allpair = dict(pair)  # 전파용(아틀라스 밖 함수 포함)
for _round in range(6):
    added = 0
    for o, n in list(allpair.items()):
        A = ordered_calls(O, PO, o); B = ordered_calls(N, PN, n)
        if not A or not B:
            continue
        inv = {v: k for k, v in allpair.items()}
        ka = [hex(allpair[t]) if t in allpair else "?" for t in A]
        kb = [hex(t) if t in inv else "?" for t in B]
        sm = difflib.SequenceMatcher(a=ka, b=kb, autojunk=False)
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            # '?'(미매핑) 끼리는 equal 로 묶이므로 equal 구간도 본다. replace 는 길이가 같을 때만.
            if tag in ("equal", "replace") and i2 - i1 == j2 - j1:
                for x, y in zip(A[i1:i2], B[j1:j2]):
                    if x in allpair or y in inv or x not in PO or y not in PN:
                        continue
                    so, sn = PO[x]["size"], PN[y]["size"]
                    if abs(so - sn) <= 0.35 * max(so, sn):
                        allpair[x] = y; inv[y] = x; added += 1
                        if x in want:
                            pair[x] = y; how[x] = "callorder"
    if not added:
        break


# 4) 머리 해시(_fnidx head) 유일 짝
head_o = collections.defaultdict(list); head_n = collections.defaultdict(list)
for a, e in PO.items():
    head_o[e["head"]].append(a)
for a, e in PN.items():
    head_n[e["head"]].append(a)
used_new = set(allpair.values())
for o in olds:
    if o in how or o not in PO:
        continue
    h = PO[o]["head"]
    if len(head_o[h]) == 1 and len(head_n[h]) == 1 and head_n[h][0] not in used_new:
        pair[o] = allpair[o] = head_n[h][0]; how[o] = "head"; used_new.add(pair[o])

# 5) 배치 순서 보간: 구 순서에서 앞뒤로 짝지어진 이웃 p<o<q 가 있으면, 신 (pair[p], pair[q]) 구간의
#    미사용 함수 중 크기·명령 분포가 가장 비슷한 것(유사도 ≥0.85)을 고른다.
import bisect  # noqa: E402
SO = sorted(PO); SN = sorted(PN)
anch = sorted((a, b) for a, b in allpair.items())
anch_o = [a for a, _ in anch]


def mnem_sim(a, b):
    x, y = a["mnem"], b["mnem"]
    ks = set(x) | set(y)
    inter = sum(min(x.get(k, 0), y.get(k, 0)) for k in ks); uni = sum(max(x.get(k, 0), y.get(k, 0)) for k in ks)
    return inter / uni if uni else 0


for o in olds:
    if o in how or o not in PO:
        continue
    i = bisect.bisect_left(anch_o, o)
    if i == 0 or i >= len(anch_o):
        continue
    lo, hi = anch[i - 1][1], anch[i][1]
    if lo >= hi:
        continue
    j1, j2 = bisect.bisect_right(SN, lo), bisect.bisect_left(SN, hi)
    cands = [n for n in SN[j1:j2] if n not in used_new and abs(PN[n]["size"] - PO[o]["size"]) <= 0.35 * max(PN[n]["size"], PO[o]["size"])]
    if not cands or len(SN[j1:j2]) > 400:
        continue
    best = max(cands, key=lambda n: mnem_sim(PO[o], PN[n]))
    if mnem_sim(PO[o], PN[best]) >= 0.85:
        pair[o] = best; how[o] = "layout"; used_new.add(best)


def imms(img, idx, f):
    c = collections.Counter()
    if f not in idx:
        return c
    for i in insns(img, f, int(idx[f]["size"])):
        for op in i.operands:
            if op.type == X86_OP_IMM and not (0x140000000 <= op.imm < 0x150000000) and i.mnemonic not in ("call", "jmp") and not i.mnemonic.startswith("j"):
                c[op.imm & 0xFFFFFFFFFFFFFFFF] += 1
    return c


rows = []
for r in fs:
    o = int(r["RVA(0.6.4)"], 16)
    n = pair.get(o)
    v = how.get(o, "미매핑")
    d_add = d_rem = 0
    if n:
        a, b = imms(O, PO, o), imms(N, PN, n)
        d_add = sum((b - a).values()); d_rem = sum((a - b).values())
        if v in ("skel", "callorder", "head", "layout"):
            v += ":상수동일" if not (d_add or d_rem) else ":상수변경"
    rows.append(dict(old=hex(o), new=hex(n) if n else "", verdict=v, name=r["의미ID"], set=r["집합"], area=r["구역"],
                     size_old=PO[o]["size"] if o in PO else "", size_new=PN[n]["size"] if n and n in PN else "",
                     imm_added=d_add, imm_removed=d_rem))
os.makedirs(out, exist_ok=True)
with open(os.path.join(out, "atlas_064_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
print(collections.Counter(r["verdict"].split(":")[0] + (":" + r["verdict"].split(":")[1] if ":" in r["verdict"] else "") for r in rows))
