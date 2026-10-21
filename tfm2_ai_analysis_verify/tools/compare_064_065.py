"""0.6.4 측정값 ↔ 0.6.5 재측정값 대조 (2026-10-21, 9회차).

입력: 0.6.4 합산 sites.json(analyze_sites 출력) · 0.6.5 9회차 sites.json · coverage_065.csv(구→신 지점 대응) · extra_v065_changed.txt
출력(<out>):
  compare_064_065.csv  대응 지점별 단계·임계값 T·참 비율·값 범위(구/신)
  compare_064_065.md   요약: 단계 이동, T 가 바뀐 지점, 참 비율이 크게 바뀐 지점, 바뀐 코드·신규 함수 지점의 단계
사용: python -B -X utf8 tools/compare_064_065.py <064 sites.json> <065 sites.json> <coverage_065.csv> <extra_v065_changed.txt> <out>
"""
import collections, csv, json, os, sys

a_json, b_json, cov, changed, out = sys.argv[1:6]
os.makedirs(out, exist_ok=True)
A = {s["rva"]: s for s in json.load(open(a_json, encoding="utf-8"))["sites"]}
B = {s["rva"]: s for s in json.load(open(b_json, encoding="utf-8"))["sites"]}
RANK = {"미도달": 0, "도달": 1, "한쪽만": 2, "양쪽": 3, "경계확정": 4}


def ratio(s):
    t, f = s.get("cond_true") or 0, s.get("cond_false") or 0
    return t / (t + f) if t + f else None


rows = []
for r in csv.DictReader(open(cov, encoding="utf-8-sig")):
    if not r["new"]:
        continue
    a, b = A.get(r["old"]), B.get(r["new"])
    if not a or not b:
        continue
    ra, rb = ratio(a), ratio(b)
    rows.append(dict(old=r["old"], new=r["new"], function=b["function"], area=b["area"], insn_old=a["insn"], insn_new=b["insn"],
                     kind=b["kind"], rel=b.get("rel") or "", T_old=a.get("T"), T_new=b.get("T"),
                     stage_old=a["stage"], stage_new=b["stage"], hits_old=a.get("ingame_hits") or 0, hits_new=b.get("ingame_hits") or 0,
                     true_old="" if ra is None else round(ra, 4), true_new="" if rb is None else round(rb, 4),
                     lhs_old=f"{a.get('lhs_min')}..{a.get('lhs_max')}", lhs_new=f"{b.get('lhs_min')}..{b.get('lhs_max')}"))
with open(os.path.join(out, "compare_064_065.csv"), "w", encoding="utf-8-sig", newline="") as h:
    w = csv.DictWriter(h, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)

L = ["# 0.6.4 ↔ 0.6.5 재측정 대조 (9회차)", "",
     f"대응 지점 {len(rows)}곳(구·신 모두 판독 대상). 0.6.4 = 2·3회차 합산(MOBA 시즌 + 데스매치), 0.6.5 = 9회차(MOBA 6세트 + 배경 리그 경기).", ""]
mv = collections.Counter((r["stage_old"], r["stage_new"]) for r in rows)
L += ["## 1. 단계 이동 (0.6.4 → 0.6.5)", "", "| 0.6.4 | 0.6.5 | 지점 |", "|---|---|---|"]
L += [f"| {a} | {b} | {n} |" for (a, b), n in sorted(mv.items(), key=lambda x: -x[1])]
tch = [r for r in rows if r["kind"].startswith("CMP") and r["T_old"] is not None and r["T_old"] != r["T_new"]]
L += ["", f"## 2. 비교 임계값 T 가 바뀐 지점 ({len(tch)}곳)", "", "| 0.6.5 지점 | 함수 | 명령(0.6.5) | T 0.6.4 → 0.6.5 | 단계(신) | 참 비율 구→신 |", "|---|---|---|---|---|---|"]
for r in sorted(tch, key=lambda r: r["new"])[:200]:
    L.append(f"| {r['new']} | {r['function']} | `{r['insn_new']}` | {r['T_old']} → {r['T_new']} | {r['stage_new']} | {r['true_old']} → {r['true_new']} |")
big = [r for r in rows if r["true_old"] != "" and r["true_new"] != "" and r["hits_old"] >= 100 and r["hits_new"] >= 100
       and abs(r["true_new"] - r["true_old"]) >= 0.3]
L += ["", f"## 3. 같은 비교인데 참 비율이 30%p 이상 바뀐 지점 ({len(big)}곳, 양쪽 100회 이상)", "",
      "판단이 실제로 다르게 갈리고 있다는 신호(0.6.5 로직·스탯 감쇠·기분 판정 영향 후보). 상위 80개.", "",
      "| 0.6.5 지점 | 함수 | 명령 | 참 비율 구→신 | 값 범위 구 / 신 |", "|---|---|---|---|---|"]
for r in sorted(big, key=lambda r: -abs(r["true_new"] - r["true_old"]))[:80]:
    L.append(f"| {r['new']} | {r['function']} | `{r['insn_new']}` | {r['true_old']} → {r['true_new']} | {r['lhs_old']} / {r['lhs_new']} |")
# 바뀐 코드·신규 함수 지점
kinds = {}
for line in open(changed, encoding="utf-8"):
    if line.startswith("#") or not line.strip():
        continue
    rva, note = line.split(" ", 1)
    kinds[hex(int(rva, 16))] = note.strip()
st = collections.defaultdict(collections.Counter)
for rva, note in kinds.items():
    s = B.get(rva)
    st[note.split(":")[0] if not note.startswith("newfn") else note][s["stage"] if s else "미도달"] += 1
L += ["", "## 4. 바뀐 코드(changed)·신규 판정 함수(newfn) 지점 단계", "", "| 묶음 | 미도달 | 도달 | 한쪽만 | 양쪽 | 경계확정 |", "|---|---|---|---|---|---|"]
for k in sorted(st):
    c = st[k]
    L.append(f"| {k} | {c['미도달']} | {c['도달']} | {c['한쪽만']} | {c['양쪽']} | {c['경계확정']} |")
open(os.path.join(out, "compare_064_065.md"), "w", encoding="utf-8").write("\n".join(L) + "\n")
print("대응", len(rows), "· T변경", len(tch), "· 참비율 급변", len(big))
print(dict(mv.most_common(8)))
