"""집중 관측 목록 생성 (v0.5.2).

analyze_sites.py 가 만든 sites.json 에서 아직 결론이 안 난 지점 id 를 골라 focus_sites.txt 로 쓴다.
  - 미도달: 전부(값 기록 지점 포함)
  - 비교(CMP/TEST): 한쪽만, 양쪽(경계 미확정)
이미 결론 난 지점(경계확정 비교, 도달한 값 기록 지점)은 빼서 예산을 남은 지점에 몰아준다.
사용: python -B -X utf8 tools/make_focus.py <sites.json> <출력 focus_sites.txt>
"""
import json, sys, collections

src, dst = sys.argv[1], sys.argv[2]
sites = json.load(open(src, encoding="utf-8"))["sites"]
pick = []
why = collections.Counter()
for s in sites:
    st = s["stage"]
    if st == "미도달" or (s["kind"] != "VALUE" and st in ("한쪽만", "양쪽")):
        pick.append(s["id"])
        why[f"{s['kind']}:{st}"] += 1
pick.sort()
open(dst, "w", encoding="utf-8", newline="\n").write("\n".join(map(str, pick)) + "\n")
print(len(pick), dict(why))
