"""변한 지점(unaligned·imm_*)의 구 즉치가 0.6.5 대응 함수에 남아 있는지 판정 (2026-10-21).

정렬이 못 맞춘 지점도 '같은 니모닉 + 같은 즉치' 명령이 신 함수에 있으면 값은 살아 있다고 본다(위치만 재배치).
스탯 배율(stat×m/1000) 도입으로 비교값이 ×1000(또는 ×1000−1) 된 경우도 따로 센다.
  survive   = 같은 니모닉·같은 즉치가 신 함수에 있음
  scaled    = 즉치×1000 또는 ×1000−1 이 신 함수에 있음(스탯 배율)
  gone      = 둘 다 없음 → 값이 바뀌었거나 코드가 사라짐(확인 대상)
  noimm     = 구 명령에 즉치 없음(레지스터·메모리 비교) → 판정 불가
결과를 sites_064_065.csv 에 survival 열로 덧붙인다.
사용: python -B -X utf8 tools/site_survival.py <migration_065 폴더> <site_catalog.json>
"""
import collections, csv, json, os, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import N, PN, insns  # noqa: E402
from capstone.x86 import X86_OP_IMM  # noqa: E402
_x065.cs.detail = True

mig, cat_p = sys.argv[1:3]
p = os.path.join(mig, "sites_064_065.csv")
rows = list(csv.DictReader(open(p, encoding="utf-8-sig")))
cat = {hex(s["rva"]): s for s in json.load(open(cat_p, encoding="utf-8"))["sites"]}
cache = {}


def new_imms(f):
    if f not in cache:
        n = int(f, 16)
        s = collections.defaultdict(set)
        if n in PN:
            for i in insns(N, n, int(PN[n]["size"])):
                for op in i.operands:
                    if op.type == X86_OP_IMM:
                        s[i.mnemonic].add(op.imm & 0xFFFFFFFFFFFFFFFF)
                        s["*"].add(op.imm & 0xFFFFFFFFFFFFFFFF)
        cache[f] = s
    return cache[f]


for r in rows:
    r["survival"] = ""
    if r["status"] not in ("unaligned", "imm_struct", "imm_knob") or not r["fn_new"]:
        continue
    s = cat[r["old"]]
    imms = [o[1]["value"] & 0xFFFFFFFFFFFFFFFF for o in s.get("ops", []) if o[0] == "imm"]
    if not imms:
        r["survival"] = "noimm"; continue
    ni = new_imms(r["fn_new"])
    mn = s["mnemonic"]
    if all(v in ni.get(mn, ()) for v in imms):
        r["survival"] = "survive"
    elif all((v * 1000) in ni["*"] or (v * 1000 - 1) in ni["*"] for v in imms if v):
        r["survival"] = "scaled"
    else:
        r["survival"] = "gone"
with open(p, "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
print(collections.Counter((r["status"], r["survival"]) for r in rows if r["survival"]))
