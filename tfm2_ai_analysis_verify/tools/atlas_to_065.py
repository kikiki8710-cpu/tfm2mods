"""0.6.4 아틀라스(함수 표·수치 기록의 소비 위치)와 추가 지점 파일을 0.6.5 주소로 옮긴다 (2026-10-21).

함수 대응(atlas_064_065.csv)마다 구·신 명령열을 3단으로 정렬해 명령 단위 주소표를 만든다.
  1) 니모닉 + 숫자→N 키 정렬
  2) 1차 대응 사이 빈틈만 레지스터까지 R 로 지운 키로 재정렬(레지스터 재배정)
  3) 그래도 남은 빈틈에서 '같은 니모닉 + 같은 즉치' 명령이 신 쪽 빈틈에 하나뿐이면 짝(재배치된 손잡이)
RE 판정(re_overrides.csv)이 있으면 그것이 우선: 소멸 = 버림, 그 밖 = new 열 주소.
출력(<out>): functions.csv · parameters.csv(gen_sites.py 입력과 같은 열) · extra_*.txt · map_report.json
사용: python -B -X utf8 tools/atlas_to_065.py <migration_065 폴더> <out 폴더>
"""
import bisect, collections, csv, difflib, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM  # noqa: E402
_x065.cs.detail = True

ATLAS = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\function_atlas_064_20261018"
EXTRA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "sites")
mig, out = sys.argv[1:3]
os.makedirs(out, exist_ok=True)
pairs = {int(r["old"], 16): int(r["new"], 16) for r in csv.DictReader(open(os.path.join(mig, "atlas_064_065.csv"), encoding="utf-8-sig")) if r["new"]}
am = json.load(open(os.path.join(mig, "aidiff_map.json"), encoding="utf-8"))
for k, (n, v, _) in am["map"].items():  # 아틀라스 밖 함수(추가 지점용)
    pairs.setdefault(int(k, 16), int(n, 16))
OV = {int(r["old"], 16): r for r in csv.DictReader(open(os.path.join(mig, "re_overrides.csv"), encoding="utf-8-sig"))}

NUM = re.compile(r"0x[0-9a-f]+|\b\d+\b")
REG = re.compile(r"\b(?:[re]?(?:ax|bx|cx|dx|si|di|bp|sp)|r\d+[dwb]?|[abcd][lh]|[sd]il|[sb]pl|xmm\d+)\b")


def k1(i):
    return i.mnemonic + "|" + NUM.sub("N", i.op_str)


def k2(i):
    return i.mnemonic + "|" + REG.sub("R", NUM.sub("N", i.op_str))


def k3(i):
    return (i.mnemonic, tuple(op.imm & 0xFFFFFFFFFFFFFFFF for op in i.operands if op.type == X86_OP_IMM))


def gaps(amap, la, lb):
    keys = sorted(amap)
    b = [(-1, -1)] + [(a, amap[a]) for a in keys] + [(la, lb)]
    for (pa, pb), (qa, qb) in zip(b, b[1:]):
        if qa - pa > 1 and qb - pb > 1:
            yield pa + 1, qa, pb + 1, qb


fmap_cache = {}


def fn_map(o):
    """구 함수 o → {구 명령 rva: 신 명령 rva}"""
    if o in fmap_cache:
        return fmap_cache[o]
    n = pairs.get(o)
    if n is None or o not in PO or n not in PN:
        fmap_cache[o] = None; return None
    A = insns(O, o, int(PO[o]["size"])); B = insns(N, n, int(PN[n]["size"]))
    amap = {}
    for a0, b0, sz in difflib.SequenceMatcher(a=[k1(i) for i in A], b=[k1(i) for i in B], autojunk=False).get_matching_blocks():
        for t in range(sz):
            amap[a0 + t] = b0 + t
    ka2 = [k2(i) for i in A]; kb2 = [k2(i) for i in B]
    for a1, a2, b1, b2 in list(gaps(amap, len(A), len(B))):
        for a0, b0, sz in difflib.SequenceMatcher(a=ka2[a1:a2], b=kb2[b1:b2], autojunk=False).get_matching_blocks():
            for t in range(sz):
                amap[a1 + a0 + t] = b1 + b0 + t
    for a1, a2, b1, b2 in list(gaps(amap, len(A), len(B))):
        cb = collections.defaultdict(list)
        for j in range(b1, b2):
            if k3(B[j])[1]:
                cb[k3(B[j])].append(j)
        ca = collections.Counter(k3(A[i]) for i in range(a1, a2))
        for i in range(a1, a2):
            kk = k3(A[i])
            if kk[1] and ca[kk] == 1 and len(cb.get(kk, ())) == 1:
                amap[i] = cb[kk][0]
    m = {A[i].address - 0x140000000: B[j].address - 0x140000000 for i, j in amap.items()}
    fmap_cache[o] = m
    return m


starts = sorted(pairs)


def owner_old(rva):
    i = bisect.bisect_right(starts, rva) - 1
    if i >= 0 and starts[i] in PO and starts[i] <= rva < starts[i] + int(PO[starts[i]]["size"]):
        return starts[i]
    return None


stat = collections.Counter()


def tr(rva):
    """구 명령 주소 → 신 주소(None = 못 옮김)"""
    if rva in OV:
        r = OV[rva]
        if r["re_verdict"] == "소멸" or not r["new"]:
            stat["RE소멸"] += 1; return None
        stat["RE"] += 1; return int(r["new"], 16)
    f = owner_old(rva)
    if f is None:
        stat["함수없음"] += 1; return None
    m = fn_map(f)
    if m is None:
        stat["함수미매핑"] += 1; return None
    if rva not in m:
        stat["명령미대응"] += 1; return None
    stat["대응"] += 1
    return m[rva]


# 함수 표
fr = list(csv.DictReader(open(os.path.join(ATLAS, "functions.csv"), encoding="utf-8-sig")))
fo = []
for r in fr:
    o = int(r["RVA(0.6.4)"], 16)
    n = pairs.get(o)
    if n is None or n not in PN:
        continue
    r = dict(r); r["RVA(0.6.4)"] = hex(n); r["끝RVA"] = hex(n + int(PN[n]["size"]) - 1)  # 열 이름은 gen_sites 호환용(값은 0.6.5)
    fo.append(r)
with open(os.path.join(out, "functions.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(fr[0])); w.writeheader(); w.writerows(fo)

# 수치 기록
pr = list(csv.DictReader(open(os.path.join(ATLAS, "parameters.csv"), encoding="utf-8-sig")))
po = []
pstat = collections.Counter()
for r in pr:
    if r["원근거버전"] != "current064":
        continue
    olds = re.findall(r"0x[0-9a-fA-F]+", r["현행 소비 위치"])
    news = [tr(int(s, 16)) for s in olds]
    keep = [hex(x) for x in news if x is not None]
    pstat["전부" if len(keep) == len(olds) else ("일부" if keep else "없음")] += 1
    if not keep:
        continue
    r = dict(r); r["현행 소비 위치"] = " ".join(keep)
    r["현행 함수RVA"] = re.sub(r"0x[0-9a-fA-F]+", lambda m: hex(pairs.get(int(m.group(0), 16), int(m.group(0), 16))), r["현행 함수RVA"])
    po.append(r)
with open(os.path.join(out, "parameters.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(pr[0])); w.writeheader(); w.writerows(po)

# 추가 지점 파일
estat = {}
for name in ("extra_v054.txt",):
    lines, c = [], collections.Counter()
    for line in open(os.path.join(EXTRA, name), encoding="utf-8"):
        body = line.split("#")[0].strip()
        if not body:
            lines.append(line.rstrip("\n")); continue
        rva, note = (body.split(None, 1) + [""])[:2]
        x = tr(int(rva, 16))
        if x is None:
            c["못옮김"] += 1; lines.append(f"# 0.6.5 못옮김 {rva} {note}"); continue
        c["옮김"] += 1
        lines.append(f"{hex(x)} {note} (0.6.4 {rva})")
    open(os.path.join(out, name.replace("v054", "v065")), "w", encoding="utf-8").write("\n".join(lines) + "\n")
    estat[name] = dict(c)

rep = {"주소": dict(stat), "기록": dict(pstat), "기록_출력": len(po), "함수_출력": len(fo), "추가지점": estat}
json.dump(rep, open(os.path.join(out, "map_report.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps(rep, ensure_ascii=False))
