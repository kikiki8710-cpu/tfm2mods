"""상세전술 후보 ↔ 측정 결과 대조 (2026-10-21).

기획(tfm2_detail_tactics/00_기획_전술목록.md)의 후보 행마다
  - 근거 함수(문장 속 0x… 주소) 각각의 관측 범주(measured_summary function_categories.csv, A~H)
  - 근거 문장의 수치(손잡이 값)와 같은 즉치(T)·기록 값을 가진 그 함수의 측정 지점과 실측 단계
를 모은다. 손잡이 지점 매칭은 수치 일치 기준의 추정이다(같은 값의 다른 비교가 섞일 수 있음).
사용: python -B -X utf8 tools/tactic_coverage.py <기획 md> <site_report 폴더> <measured_summary 폴더> <출력 md>
"""
import collections, csv, json, os, re, sys

plan, rep, ms, out = sys.argv[1:5]
cat = {r["f"]: r for r in csv.DictReader(open(os.path.join(ms, "function_categories.csv"), encoding="utf-8-sig"))}
S = json.load(open(os.path.join(rep, "sites.json"), encoding="utf-8"))["sites"]
C = json.load(open(os.path.join(rep, "claims.json"), encoding="utf-8"))["claims"]
rec_vals = collections.defaultdict(set)
for c in C:
    for v in c["values"]:
        if isinstance(v, (int, float)):
            rec_vals[c["record"]].add(int(v))
by_fn = collections.defaultdict(list)
for s in S:
    if [r for r in s.get("records", []) if r != "extra"]:
        by_fn[s["function"]].append(s)
RANK = {"경계확정": 4, "양쪽": 3, "한쪽만": 2, "도달": 1, "미도달": 0}

rows, section = [], ""
for line in open(plan, encoding="utf-8"):
    if line.startswith("### "):
        section = line[4:].strip()
    if not line.startswith("| ") or line.startswith("| 전술") or line.startswith("|---"):
        continue
    cells = [x.strip() for x in line.strip().strip("|").split("|")]
    if len(cells) < 6:
        continue
    name, basis, stage = cells[0], cells[3], cells[5]
    fns = []
    for m in re.findall(r"0x[0-9a-fA-F]{5,7}", basis):
        f = "0x" + m[2:].lower()
        if f not in fns:
            fns.append(f)
    if not fns:
        continue
    nums = {int(x.replace(",", "")) for x in re.findall(r"(?<![0-9a-fA-Fx])\d[\d,]*", basis) if x.replace(",", "").isdigit()}
    nums = {n for n in nums if n >= 2}  # 0·1 은 너무 흔하다
    if any(n >= 10 for n in nums):
        nums = {n for n in nums if n >= 10}  # 작은 수(2·3·7)는 다른 비교와 우연히 겹친다
    fn_info, knob = [], []
    for f in fns:
        r = cat.get(f)
        fn_info.append(f"`{f}` {r['cat'] if r else '-'}")
        for s in by_fn.get(f, []):
            vals = set()
            if s["T"] is not None:
                vals.add(abs(int(s["T"])))
            for rid in s.get("records", []):
                vals |= rec_vals.get(rid, set())
            hit = vals & nums
            if hit:
                knob.append((f, s["rva"], s["insn"], s["stage"], sorted(hit)[:3]))
    best = max((RANK[k[3]] for k in knob), default=None)
    rows.append(dict(section=section, name=re.sub(r"\*", "", name), stage=stage, fns=fn_info, knob=knob, best=best,
                     cats=[cat[f]["cat"] for f in fns if f in cat]))

LV = {4: "경계확정", 3: "양쪽", 2: "한쪽만", 1: "도달", 0: "미도달", None: "매칭 없음"}
L = ["# 상세전술 후보 ↔ 측정 결과 (0.6.4 · 2~6회차 합산)", "",
     "> 근거 함수 범주 = `function_categories.md` 의 A~H(A 임계점 전부 확인 · B 비교 전부 양쪽 · C 값만 전부 실행 · D 전부 실행·일부 한쪽만 · E 90%↑ · F 50~90% · G 50%↓ · H 미도달).",
     "> **손잡이 지점** = 근거 문장의 수치와 같은 즉치·기록 값을 가진 그 함수의 측정 지점(수치 일치 기준 추정). 최고 단계 = 그 지점들 중 가장 잘 관측된 단계.", ""]
cnt = collections.Counter(LV[r["best"]] for r in rows)
L += ["## 요약", "", f"후보 행 {len(rows)}개(근거 주소가 있는 행).", "", "| 손잡이 최고 단계 | 후보 |", "|---|---:|"]
for k in ("경계확정", "양쪽", "한쪽만", "도달", "미도달", "매칭 없음"):
    L.append(f"| {k} | {cnt[k]} |")
L.append("")
for sec in dict.fromkeys(r["section"] for r in rows):
    L += [f"## {sec}", "", "| 후보 | 기획 단계 | 근거 함수(범주) | 손잡이 최고 | 손잡이 지점(단계·값) |", "|---|---|---|---|---|"]
    for r in [x for x in rows if x["section"] == sec]:
        ks = "; ".join(f"`{k[1]}` {k[3]} {k[4]}" for k in sorted(r["knob"], key=lambda k: -RANK[k[3]])[:4])
        L.append(f"| {r['name'][:40]} | {r['stage']} | {' · '.join(r['fns'][:6])} | {LV[r['best']]} | {ks} |")
    L.append("")
open(out, "w", encoding="utf-8", newline="\n").write("\n".join(L) + "\n")
json.dump(rows, open(out.replace(".md", ".json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(rows), dict(cnt))
