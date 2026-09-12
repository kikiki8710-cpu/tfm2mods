# -*- coding: utf-8 -*-
u"""**G18(knobs.value) 변이 시험** — 「적발 1건」이 *무능* 때문이 아님을 보이는 반증 장치. (9차 배치C)

적발이 적으면 두 가지로 읽힌다: ①축이 실제로 깨끗하다 ②게이트가 아무것도 못 잡는다.
그래서 **일부러 값을 틀리게** 넣고 몇 %를 되잡는지 센다(정본은 안 건드린다 — 메모리 사본만).

변이 3종 — 실제로 일어나는 오류 형태를 본떴다:
- `off1`  값 ±1 (자리 옮겨적기 · 경계 오독)  ← **가장 어려운 변이**(창 안에 이웃 수가 흔하다)
- `x10`   자릿수 하나 밀림 (150000 → 1500000)
- `swap`  ★**arm 방향 뒤집힘** — 같은 명세 안 **다른 노브의 값**으로 바꿔치기.
          7차 `01 knobs[2]/[3]`(20↔40) 이 실제로 이 형태였다

「못 잡음」 목록이 곧 **게이트의 사각지대**다.

사용: `python -X utf8 mutate.py`  (또는 `gate.py --mutate`)
"""
import collections, copy, io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G


def mutate_value(v, kind, other=None):
    u"""값을 틀리게 만든다. 문자열이면 **첫 정수만** 바꾼다(문면은 그대로)."""
    def f(n):
        if kind == "off1":
            return n + 1 if n >= 0 else n - 1
        if kind == "x10":
            return n * 10 if n else 7
        return other
    if isinstance(v, int) and not isinstance(v, bool):
        r = f(v)
        return None if r is None or r == v else r
    if isinstance(v, str):
        m = re.search(r"-?\d+", v.replace(u",", u""))
        if not m:
            return None
        n = int(m.group(0))
        r = f(n)
        if r is None or r == n:
            return None
        s = v.replace(u",", u"")
        return s[:m.start()] + str(r) + s[m.end():]
    return None


def applicable(sp, j):
    u"""V1 이 **발화할 수 있는 상태**인가(앵커·관측이 있고 수 주장인가). 분모를 정직하게 잡는다."""
    k = (sp.get("knobs") or [])[j]
    if not G.numeric_claim(k.get("value")) or not G.value_lits(k.get("value")):
        return False
    w = k.get("where") or u""
    anch = G.anchors(w) or G.src_anchors(sp, w)       # ★`check_spec` 과 **같은** 앵커 규칙
    if not anch:
        return False
    obs, nread = G.window_lits(anch, G.W)
    return bool(obs and nread)


def main():
    D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    tot = collections.Counter()
    caught = collections.Counter()
    misses = collections.defaultdict(list)
    for i, sp0 in enumerate(D["specs"]):
        ks = sp0.get("knobs") or []
        pool = [k.get("value") for k in ks
                if isinstance(k.get("value"), int) and not isinstance(k.get("value"), bool)]
        for j, k in enumerate(ks):
            if not applicable(sp0, j):
                continue
            for kind in ("off1", "x10", "swap"):
                other = None
                if kind == "swap":
                    cand = [p for p in pool if p != k.get("value")]
                    if not cand:
                        continue
                    other = cand[0]
                nv = mutate_value(k.get("value"), kind, other)
                if nv is None:
                    continue
                sp = copy.deepcopy(sp0)
                sp["knobs"][j]["value"] = nv
                tot[kind] += 1
                if any(jj == j and u"IR 관측" in why for (jj, why, _) in G.check_spec(sp, ("V1",))):
                    caught[kind] += 1
                else:
                    misses[kind].append((i, sp0["name"], j, k.get("what"), k.get("value"), nv))
    print(u"== 변이 시험 (knobs[].value 를 일부러 틀리게 넣고 되잡는가) ==")
    ct = sum(caught.values()); tt = sum(tot.values())
    for kind in ("off1", "x10", "swap"):
        if tot[kind]:
            print(u"  %-5s 포착 %3d / %3d  (%.1f%%)"
                  % (kind, caught[kind], tot[kind], 100.0 * caught[kind] / tot[kind]))
    print(u"\n합계: 포착 %d / %d (%.1f%%)" % (ct, tt, 100.0 * ct / tt if tt else 0))
    print(u"\n-- 사각지대(off1 제외 · 자릿수/arm 변이를 놓친 행) --")
    for kind in ("x10", "swap"):
        for (i, nm, j, what, ov, nv) in misses[kind][:20]:
            print(u"   [%s] %02d %-28s knobs[%d] %-26s %r → %r"
                  % (kind, i, nm[:28], j, (what or u"")[:26], ov, nv))


if __name__ == "__main__":
    main()
