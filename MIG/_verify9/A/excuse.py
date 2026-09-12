# -*- coding: utf-8 -*-
u"""**면죄율 측정** — 구제(약한 후보)를 늘리면 게이트가 눈이 먼다. 얼마나 멀었는지 센다.

`mix` 변이가 100% 인 것은 「후보에 없는 줄을 넣으면 잡는다」는 동어반복일 수 있다.
진짜 물음은 **약한 후보가 함수 줄 공간의 몇 %를 면죄하는가**이다.
면죄율이 100% 에 가까우면 그 상수는 사실상 검사받지 않는다.

사용: `python -X utf8 excuse.py`
"""
import io, json, os, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G
import srclinecheck as S
from mutate import fn_ownlines

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def old_sets(sp, j):
    u"""`srclinecheck` 의 강/약 집합을 같은 방식으로 재현(원본 함수 재사용)."""
    ir = sp["ir"]; f, a, b = ir["file"], ir["frm"], ir["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = S.load(f)
    c = sp["consts"][j]; val = c.get("value")
    pat = S.litpat(val)
    found, soft = {}, set()
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln or not pat.search(ln):
            continue
        st, wk = S.dbg_ids(src, meta, k, a, b, val)
        for did in st:
            for (fn, li) in S.chain(meta, did):
                if fn == own:
                    found[li] = found.get(li, 0) + 1
        for did in wk:
            for (fn, li) in S.chain(meta, did):
                if fn == own:
                    soft.add(li)
    return {x for x in found if x}, {x for x in soft if x}


rows = []
for i, sp in enumerate(D["specs"]):
    try:
        ref = G.collect(sp)
    except Exception:
        continue
    pool = set(fn_ownlines(sp))
    if not pool:
        continue
    for j, c in enumerate(sp.get("consts") or []):
        if not isinstance(c.get("src_line"), int) or j not in ref:
            continue
        ns, nw, _ = ref[j]
        if not ns:
            continue
        os_, ow_ = old_sets(sp, j)
        rows.append((sp["name"], j, len(pool),
                     len((set(ns) | nw) & pool), len((os_ | ow_) & pool)))

print(u"== 면죄율(그 상수가 「맞다」고 인정하는 줄 / 함수가 참조하는 줄 전체) ==")
print(u"   대상 = 검사 가능 상수 %d개" % len(rows))
tn = sum(r[3] for r in rows); to = sum(r[4] for r in rows); tp = sum(r[2] for r in rows)
print(u"   현행 srclinecheck : %d / %d = %.1f%%" % (to, tp, 100.0 * to / tp))
print(u"   9차 통합판        : %d / %d = %.1f%%" % (tn, tp, 100.0 * tn / tp))
print(u"   ⟹ 늘어난 면죄 폭 = %+.1f%%p (상수당 평균 %+.2f줄)"
      % (100.0 * (tn - to) / tp, float(tn - to) / len(rows)))
print(u"\n-- 면죄 폭이 가장 커진 상수 상위 10 --")
for nm, j, p, n, o in sorted(rows, key=lambda r: r[4] - r[3])[:10]:
    print(u"   %-44s consts[%-2d]  함수줄 %3d 중  현행 %2d → 통합 %2d" % (nm[:44], j, p, o, n))
print(u"\n-- 통합판 면죄율 50%% 초과(사실상 검사 안 됨) --")
bad = [r for r in rows if r[2] and r[3] * 2 > r[2]]
for nm, j, p, n, o in bad:
    print(u"   %-44s consts[%-2d]  %d/%d" % (nm[:44], j, n, p))
print(u"   총 %d개" % len(bad))
