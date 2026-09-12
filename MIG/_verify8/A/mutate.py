# -*- coding: utf-8 -*-
u"""**G14 변이 시험** — 게이트가 「0건」을 내는 게 *무능해서*가 아님을 보이는 반증 장치.

적발 0 은 두 가지로 읽힌다: ①축이 실제로 깨끗하다 ②게이트가 아무것도 못 잡는다.
그래서 **일부러 방향을 뒤집어** 넣고 몇 건을 되잡는지 센다(정본은 건드리지 않는다 — 메모리 사본만).

- `--flip` : 모든 `r`/`w` 행을 하나씩 뒤집어 **개별** 포착률을 잰다(행당 실행 1회).
- 결과의 「못 잡음」은 **게이트의 사각지대 목록**이다 — 한계 §3 의 실측 근거.

사용: `python -X utf8 mutate.py`
"""
import io, json, os, sys, copy, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

caught = miss = 0
misses = collections.Counter()
per = {}
for i, sp0 in enumerate(D["specs"]):
    mem = sp0.get("mem") or []
    c = m = 0
    for j, row in enumerate(mem):
        d = (row.get("dir") or "").strip()
        if d not in ("r", "w") or not G.PUREOFF.match(str(row.get("offset") or "")):
            continue
        sp = copy.deepcopy(sp0)
        sp["mem"][j]["dir"] = "w" if d == "r" else "r"
        hit = any(jj == j for (jj, _, _) in G.check_spec(sp))
        if hit:
            c += 1
        else:
            m += 1
            misses[(sp0["name"], row.get("base"), row.get("offset"), d)] += 1
    per[i] = (c, m)
    caught += c
    miss += m

print(u"== 변이 시험(행마다 dir 을 뒤집어 넣고 되잡는지) ==")
for i in sorted(per):
    c, m = per[i]
    if c + m:
        print(u"  specs[%2d] %-46s 포착 %3d / %3d  (%.0f%%)"
              % (i, D["specs"][i]["name"], c, c + m, 100.0 * c / (c + m)))
print(u"\n합계: 포착 %d / %d (%.1f%%) · 사각 %d" % (caught, caught + miss, 100.0 * caught / (caught + miss), miss))
print(u"\n-- 사각지대 상위 --")
for (nm, base, off, d), n in misses.most_common(25):
    print(u"   %-40s %-34s %-8s dir=%s" % (nm[:40], (base or "")[:34], off, d))
