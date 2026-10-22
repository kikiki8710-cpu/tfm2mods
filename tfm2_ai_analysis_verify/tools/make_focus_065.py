"""10회차 집중 관측 목록 (2026-10-22, 0.6.5).

9회차 결과에서 결론이 안 난 지점만 골라 현재 site_catalog 의 id 로 focus_sites.txt 를 쓴다.
  ① 예산 포화(적중 ≥ 지점 상한 누적의 90%) 비교 지점 중 경계확정이 아닌 것 — 드문 값이 예산에 밀려 빠졌을 수 있음
  ② 9회차 미도달 중 분모 제외(exclude_065.txt)가 아닌 것 — 새로 추가한 지점(9회차 표에 없던 rva)도 포함
사용: python -B -X utf8 tools/make_focus_065.py <9회차 site_report/sites.json> <9회차 final.json[,이후 회차 final.json…]> <exclude_065.txt> <출력 focus_sites.txt> [--unreached-only]
  final 을 쉼표로 여러 개 주면 도달은 합집합으로 본다(포화 판정은 첫 파일 기준). --unreached-only = ① 생략(11회차: 미도달만).
"""
import collections, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
rep, finals, excl, dst = sys.argv[1:5]
only_unreached = "--unreached-only" in sys.argv[5:]
finals = finals.split(",")
f9 = json.load(open(finals[0], encoding="utf-8"))
cap = 64 * f9["t_ms"] / 10000
hits = {int(s["rva"], 16): sum(r[0] for r in s["c"]) for s in f9["sites"]}
seen = set(hits)
for f in finals[1:]:
    seen |= {int(s["rva"], 16) for s in json.load(open(f, encoding="utf-8"))["sites"]}
stage = {int(s["rva"], 16): s for s in json.load(open(rep, encoding="utf-8"))["sites"]}
ex = {int(l.split()[0], 16) for l in open(excl, encoding="utf-8") if l.strip() and not l.startswith("#")}
cat = json.load(open(os.path.join(HERE, "sites", "site_catalog.json"), encoding="utf-8"))["sites"]
pick, why = [], collections.Counter()
for s in cat:
    r = s["rva"]
    if r in ex:
        continue
    st = stage.get(r)
    if r not in seen:
        pick.append(s["id"]); why["미도달" if st else "신규 지점"] += 1
    elif not only_unreached and st and st["kind"] != "VALUE" and st["stage"] in ("한쪽만", "양쪽") and hits[r] >= 0.9 * cap:
        pick.append(s["id"]); why["포화:" + st["stage"]] += 1
pick.sort()
open(dst, "w", encoding="utf-8", newline="\n").write("\n".join(map(str, pick)) + "\n")
print(len(pick), dict(why))
