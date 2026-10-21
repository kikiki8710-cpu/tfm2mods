"""9회차(0.6.5) 미도달 원인 분류 (2026-10-21).

0.6.4 때 실행된 지점이 0.6.5 9회차에서 안 잡힌 이유를 나눈다.
 ① 어느 0.6.4 회차에서 잡혔나: 02(MOBA 시즌)·03(데스매치)·04(전 챔피언 새 게임+집중)·05~07(MOBA 집중)
 ② 0.6.5 함수에 들어가긴 했나(같은 함수의 다른 지점 도달 여부)
 ③ 대응이 의심스러운가: 구·신 명령이 같은 꼴(k2)인가
사용: python -B -X utf8 tools/unreached_065.py <mods_report/tfm2_ai_analysis_verify> <coverage_065.csv> <9회차 final.json> <out 폴더>
"""
import collections, csv, glob, json, os, re, sys
sys.path.insert(0, r"C:\tfm2mods\MIG")
import _x065  # noqa: E402
from _x065 import O, N  # noqa: E402

rep, cov, final9, out = sys.argv[1:5]
os.makedirs(out, exist_ok=True)
HERE = os.path.dirname(os.path.abspath(__file__))
NUM = re.compile(r"0x[0-9a-f]+|\b\d+\b")
REG = re.compile(r"\b(?:[re]?(?:ax|bx|cx|dx|si|di|bp|sp)|r\d+[dwb]?|[abcd][lh]|[sd]il|[sb]pl|xmm\d+)\b")


def k2(img, rva):
    i = next(_x065.cs.disasm(_x065.rd(img, rva, 16), 0x140000000 + rva, 1), None)
    return (i.mnemonic + "|" + REG.sub("R", NUM.sub("N", i.op_str))) if i else "?"


ROUNDS = {"02": "MOBA시즌", "03": "데스매치", "04": "전챔피언", "05": "MOBA집중", "06": "MOBA전술", "07": "MOBA손잡이"}
seen = collections.defaultdict(set)  # 0.6.4 rva -> 회차들
for r, lab in ROUNDS.items():
    for f in glob.glob(os.path.join(rep, f"runtime_20261021_sites{r}", "after", "sites_*_final.json")):
        for s in json.load(open(f, encoding="utf-8"))["sites"]:
            seen[int(s["rva"], 16)].add(r)
hit9 = {int(s["rva"], 16) for s in json.load(open(final9, encoding="utf-8"))["sites"]}
cat = json.load(open(os.path.join(HERE, "sites", "site_catalog.json"), encoding="utf-8"))["sites"]
fn_of = {s["rva"]: (s["function"], s["area"]) for s in cat}
fn_hit = collections.Counter(fn_of[r][0] for r in hit9 if r in fn_of)
names = {r["RVA(0.6.4)"]: r["의미ID"] for r in csv.DictReader(open(os.path.join(HERE, "atlas065", "functions.csv"), encoding="utf-8-sig"))}

rows = []
for r in csv.DictReader(open(cov, encoding="utf-8-sig")):
    if not r["new"]:
        continue
    o, n = int(r["old"], 16), int(r["new"], 16)
    if n in hit9:
        continue
    rs = seen.get(o, set())
    f, area = fn_of.get(n, ("?", "?"))
    if rs and rs <= {"03"}:
        why = "데스매치 전용"
    elif rs and rs <= {"03", "04"} and "04" in rs:
        why = "전챔피언 판에서만"
    elif not rs:
        why = "0.6.4 회차 기록 없음"
    elif fn_hit[f] == 0:
        why = "함수 미진입(MOBA)"
    else:
        why = "함수 진입·지점 미도달(MOBA)"
    same = k2(O, o) == k2(N, n)
    rows.append(dict(old=r["old"], new=r["new"], stage=r["stage"], function=f, name=names.get(f, "")[:80], area=area,
                     rounds="+".join(sorted(rs)), why=why, insn_same=int(same)))
with open(os.path.join(out, "unreached_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
c = collections.Counter(r["why"] for r in rows)
print("미도달", len(rows), dict(c))
print("명령 꼴 다름(대응 의심):", collections.Counter(r["why"] for r in rows if not r["insn_same"]))
for why in ("함수 미진입(MOBA)", "함수 진입·지점 미도달(MOBA)", "전챔피언 판에서만"):
    fc = collections.Counter((r["function"], r["name"]) for r in rows if r["why"] == why)
    print(f"\n[{why}] 함수 상위")
    for (f, nm), k in fc.most_common(15):
        print(f"{k:5d} {f} {nm}")
