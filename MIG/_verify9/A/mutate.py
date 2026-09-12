# -*- coding: utf-8 -*-
u"""**G12 변이 시험** — 고친 게이트가 「0건」을 내는 게 *무능해서*가 아님을 보이는 반증 장치.

적발 0 은 두 가지로 읽힌다: ①축이 실제로 깨끗하다 ②게이트가 아무것도 못 잡는다.
그래서 **일부러 틀린 `src_line` 을 넣고** 몇 건을 되잡는지 센다(정본은 안 건드린다 — 메모리 사본만).

변이 3종 — 난이도 순:
  `far`   claim + 1000 (함수 밖 줄) — 게이트가 **켜져 있는가**의 하한 시험
  `mix`   그 함수 본문이 실제로 참조하는 **다른 소스 줄**로 바꿔치기(형제 상수와 헷갈린 형태.
          단 그 상수의 강·약 후보에 들어 있는 줄은 제외한다 — 그건 오답이 아니라 정답의 이표기다)
  `adj`   claim + 1 (한 줄 밀림) — 가장 어려움

분모에서 **검사 불가**(강한 후보 0 = 리터럴이 접혔거나 범위 밖) 상수는 뺀다.
그건 게이트의 무능이 아니라 재료의 부재이고, 어느 게이트를 써도 못 잡는다.
⟹ 두 게이트를 비교할 땐 **분모를 통일**하려고 `gate.collect` 로 검사가능 집합을 정한다.

사용: `python -X utf8 mutate.py [gate|srclinecheck]`
"""
import io, json, os, sys, copy, collections, importlib

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as REF                                   # 분모·변이값 산출 기준(항상 통합판)
MODNAME = sys.argv[1] if len(sys.argv) > 1 else "gate"
G = importlib.import_module(MODNAME)
MIG = REF.MIG

D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def fn_ownlines(sp):
    u"""그 함수 본문이 실제로 참조하는 담당 `.rs` 줄 전체(변이 후보 풀)."""
    ir = sp["ir"]
    f, a, b = ir["file"], ir["frm"], ir["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = REF.load(f)
    out = set()
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = REF.DBG.search(ln)
        if not m:
            continue
        for (fnm, li) in REF.chain(meta, m.group(1)):
            if fnm == own and li:
                out.add(li)
    return sorted(out)


MODES = ["far", "mix", "adj"]
res = {m: [0, 0] for m in MODES}
blind = collections.Counter()
ncheckable = nfolded = 0

for i, sp0 in enumerate(D["specs"]):
    try:
        ref = REF.collect(sp0)
    except Exception:
        continue
    pool_all = fn_ownlines(sp0)
    for j, c in enumerate(sp0.get("consts") or []):
        claim = c.get("src_line")
        if not isinstance(claim, int) or j not in ref:
            continue
        strong, weak, _n = ref[j]
        if not strong:
            nfolded += 1
            continue
        ncheckable += 1
        ok = set(strong) | set(weak) | {claim}
        for m in MODES:
            if m == "far":
                bad = claim + 1000
            elif m == "adj":
                bad = claim + 1
                if bad in ok:
                    continue
            else:
                pool = [x for x in pool_all if x not in ok]
                if not pool:
                    continue
                bad = max(pool, key=lambda x: abs(x - claim))
            sp = copy.deepcopy(sp0)
            sp["consts"][j]["src_line"] = bad
            hit = any(jj == j for (jj, _, _) in G.check_spec(sp))
            res[m][0 if hit else 1] += 1
            if not hit:
                blind[(m, sp0["name"], j, c.get("value"), claim, bad)] += 1

print(u"== G12 변이 시험 (게이트 모듈 = %s) ==" % MODNAME)
print(u"검사 가능 상수 %d개 / 검사 불가(강한 후보 0 = 접힘·범위 밖) %d개" % (ncheckable, nfolded))
for m in MODES:
    c, ms = res[m]
    if c + ms:
        print(u"  %-4s  포착 %3d / %3d  (%.1f%%)" % (m, c, c + ms, 100.0 * c / (c + ms)))
print(u"\n-- 사각지대(변이를 넣었는데 안 잡힌 것) --")
for (m, nm, j, val, claim, bad), n in blind.most_common(40):
    print(u"   [%s] %-42s consts[%d] value=%-8s L%s -> L%s" % (m, nm[:42], j, val, claim, bad))
