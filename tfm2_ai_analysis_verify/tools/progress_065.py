"""9회차 진행 점검 — 분류별 도달률 (2026-10-21).

분류: 이전측정(0.6.4 에서 실행된 지점의 0.6.5 대응) · changed(바뀐 코드) · newfn(신규·재작성 판정 함수) · 전체.
사용: python -B -X utf8 tools/progress_065.py <sites_*_latest.json 또는 _final.json> <coverage_065.csv> <extra_v065_changed.txt>
"""
import collections, csv, json, sys

latest, cov, changed = sys.argv[1:4]
j = json.load(open(latest, encoding="utf-8"))
hit = {}
for s in j["sites"]:
    n = sum(row[0] for row in s["c"]) if s.get("c") else 0
    hit[int(s["rva"], 16)] = n
groups = collections.defaultdict(set)
for r in csv.DictReader(open(cov, encoding="utf-8-sig")):
    if r["new"]:
        groups["이전측정:" + r["stage"]].add(int(r["new"], 16))
        groups["이전측정(전체)"].add(int(r["new"], 16))
for line in open(changed, encoding="utf-8"):
    if line.startswith("#") or not line.strip():
        continue
    rva, note = line.split(" ", 1)
    kind, _, fn = note.strip().partition(":")
    groups[kind].add(int(rva, 16))
    if kind == "newfn":
        groups["newfn:" + fn].add(int(rva, 16))
print(f"프레임 {j['frames']} · 도달 {sum(1 for v in hit.values() if v)} / 표 {len(j['sites'])}")
for k in sorted(groups):
    s = groups[k]
    h = sum(1 for r in s if hit.get(r, 0) > 0)
    print(f"{k:28s} {h:6d} / {len(s):6d}  ({100 * h / max(1, len(s)):.1f}%)")
