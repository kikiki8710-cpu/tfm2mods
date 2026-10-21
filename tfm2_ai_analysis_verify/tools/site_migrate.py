"""측정 지점(임계점·계수 명령) 0.6.4 → 0.6.5 대응·값 변화 판정 (2026-10-21).

함수 대응(atlas_064_065.csv)마다 구·신 명령열을 (니모닉 + 즉치·주소를 N 으로 바꾼 피연산자) 키로 정렬하고,
지점 명령이 신 명령에 대응되면 원문·즉치를 비교한다.
  same      = 명령 원문 동일(레지스터·즉치·변위 모두)
  imm_knob  = 즉치가 바뀌었고 구 값이 의미 기록(임계·계수·enum·동적입력) 값이거나 M1 손잡이 지점  ← 값이 바뀐 손잡이
  imm_struct= 즉치가 바뀌었지만 분석 수치와 무관(구조체 크기·필드 오프셋·스택 크기 등)
  disp      = 즉치는 같고 메모리 변위만 바뀜(필드 오프셋 이동)
  reg       = 레지스터만 바뀜(코드젠)
  unaligned = 대응 명령을 못 찾음(그 근처 소스가 바뀜)
  nofunc    = 함수가 미매핑·삭제
사용: python -B -X utf8 tools/site_migrate.py <atlas_064_065.csv> <site_catalog.json> <출력 폴더>
"""
import collections, csv, difflib, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N, PO, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM  # noqa: E402
_x065.cs.detail = True

amap_p, cat_p, out = sys.argv[1:4]
fmap = {r["old"]: r for r in csv.DictReader(open(amap_p, encoding="utf-8-sig"))}
sites = json.load(open(cat_p, encoding="utf-8"))["sites"]
KNOB_CLASS = {"threshold", "coefficient", "enum", "dynamic_input"}
M1 = {int(l.split()[0], 16) for l in open(os.path.join(os.path.dirname(__file__), "sites", "extra_v054_m1.txt"), encoding="utf-8")
      if l.strip() and not l.startswith("#")}
NUM = re.compile(r"0x[0-9a-f]+|\b\d+\b")


def key(i):
    return i.mnemonic + "|" + NUM.sub("N", i.op_str)


def imm_of(i):
    return [op.imm & 0xFFFFFFFFFFFFFFFF for op in i.operands if op.type == X86_OP_IMM]


by_fn = collections.defaultdict(list)
for s in sites:
    by_fn[s["function"]].append(s)
rows = []
for f, ss in by_fn.items():
    m = fmap.get(f)
    if not m or not m["new"]:
        for s in ss:
            rows.append(dict(id=s["id"], old=hex(s["rva"]), new="", status="nofunc", fn_old=f, fn_new="", fn_verdict=m["verdict"] if m else "아틀라스 밖",
                             insn_old=s["insn"], insn_new="", imm_old="", imm_new=""))
        continue
    o, n = int(f, 16), int(m["new"], 16)
    if o not in PO or n not in PN:
        continue
    A = insns(O, o, int(PO[o]["size"])); B = insns(N, n, int(PN[n]["size"]))
    ia = {i.address - 0x140000000: k for k, i in enumerate(A)}
    sm = difflib.SequenceMatcher(a=[key(i) for i in A], b=[key(i) for i in B], autojunk=False)
    amap = {}
    for a0, b0, sz in sm.get_matching_blocks():
        for k in range(sz):
            amap[a0 + k] = b0 + k
    # 2차: 레지스터까지 지운 거친 키로, 1차 대응 사이 빈틈만 다시 맞춘다(레지스터 재배정 대응)
    REG = re.compile(r"\b(?:[re]?(?:ax|bx|cx|dx|si|di|bp|sp)|r\d+[dwb]?|[abcd][lh]|[sd]il|[sb]pl|xmm\d+)\b")
    k2 = lambda i: i.mnemonic + "|" + REG.sub("R", NUM.sub("N", i.op_str))
    ka2 = [k2(i) for i in A]; kb2 = [k2(i) for i in B]
    keys = sorted(amap)
    bounds = [(-1, -1)] + [(a, amap[a]) for a in keys] + [(len(A), len(B))]
    for (pa, pb), (qa, qb) in zip(bounds, bounds[1:]):
        if qa - pa > 1 and qb - pb > 1:
            sm2 = difflib.SequenceMatcher(a=ka2[pa + 1:qa], b=kb2[pb + 1:qb], autojunk=False)
            for a0, b0, sz in sm2.get_matching_blocks():
                for t in range(sz):
                    amap[pa + 1 + a0 + t] = pb + 1 + b0 + t
    for s in ss:
        k = ia.get(s["rva"])
        if k is None or k not in amap:
            rows.append(dict(id=s["id"], old=hex(s["rva"]), new="", status="unaligned", fn_old=f, fn_new=m["new"], fn_verdict=m["verdict"],
                             insn_old=s["insn"], insn_new="", imm_old="", imm_new=""))
            continue
        a, b = A[k], B[amap[k]]
        ta, tb = f"{a.mnemonic} {a.op_str}", f"{b.mnemonic} {b.op_str}"
        xa, xb = imm_of(a), imm_of(b)
        if ta == tb:
            st = "same"
        elif xa != xb:
            # 손잡이 = 의미 기록(임계·계수·enum·동적입력)의 값이거나 M1 손잡이 지점. raw_numeric 주석엔 구조 상수(0xac8 등)도 섞여 있어 제외.
            vals = set()
            for c in s.get("claims", []):
                if c.get("class") in KNOB_CLASS:
                    for v in c.get("values", []):
                        if isinstance(v, (int, float)):
                            vals.add(int(v) & 0xFFFFFFFFFFFFFFFF)
            st = "imm_knob" if any(v in vals for v in xa) or s["rva"] in M1 else "imm_struct"
        elif re.sub(r"\b[re]?[a-z0-9]{2,3}\b", "R", ta) == re.sub(r"\b[re]?[a-z0-9]{2,3}\b", "R", tb):
            st = "reg"
        else:
            st = "disp"
        rows.append(dict(id=s["id"], old=hex(s["rva"]), new=hex(b.address - 0x140000000), status=st, fn_old=f, fn_new=m["new"], fn_verdict=m["verdict"],
                         insn_old=ta, insn_new=tb, imm_old=",".join(map(str, xa)), imm_new=",".join(map(str, xb))))
os.makedirs(out, exist_ok=True)
rows.sort(key=lambda r: r["id"])
with open(os.path.join(out, "sites_064_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
print(len(rows), collections.Counter(r["status"] for r in rows))
print(collections.Counter((r["fn_verdict"].split(":")[-1] if r["fn_verdict"] else "", r["status"]) for r in rows).most_common(30))
