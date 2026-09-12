# -*- coding: utf-8 -*-
u"""**G16 변이 시험** — 고친 게이트의 「적발 0」이 *무능* 이 아님을 보이는 반증 장치.

적발 0 은 두 가지로 읽힌다: ①축이 실제로 깨끗하다 ②게이트가 아무것도 못 잡는다.
그래서 **일부러 틀린 주장을 넣고** 몇 %를 되잡는지 센다(정본은 안 건드린다 — 메모리 사본만).
참조구현 = `_verify8/A/mutate.py`(G14). 변이 7종은 이 축의 실제 실패 모드다.

| 코드 | 변이 | 노리는 검사 |
|---|---|---|
| `A` | role 에 **IR 에 없는 속성어**를 덧붙임(`readonly`/`writeonly`/`captures(none)`/`nonnull`) | P4 |
| `B` | role 이 인용한 **줄번호를 +3** 으로 어긋냄 | P6 |
| `C` | role 이 인용한 **IR 조각의 마지막 정수를 +7** 로 위조 | P6 |
| `D` | 실제로 쓰이는 인자의 role 을 「본문에서 전혀 안 씀」으로 위조 | P5 |
| `E` | `params[j].i` 를 +1 로 어긋냄 | P3 |
| `F` | `params[]` 에서 한 행을 **삭제**(자리 밀림) | P1/P2 |
| `G` | sret 행의 `name` 에서 `(sret` 표식을 제거 | P2/P3 |

사용: `python -X utf8 mutate.py [--list]`
"""
import io, json, os, re, sys, copy, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
SHOW = "--list" in sys.argv

res = collections.defaultdict(lambda: [0, 0])      # class -> [caught, total]
missed = []
BASE = {}


def base_findings(i):
    if i not in BASE:
        BASE[i] = G.check_spec(D["specs"][i])
    return BASE[i]


def run(i, sp, cls, tag, expect=True):
    u"""변이본을 돌려 **기준선에 없던** 적발이 새로 나오는지 본다.
    `expect=False` = **음성 대조**(뜻을 안 바꾼 변형) — 여기서 잡히면 그게 오탐이다."""
    b = {(x[1], x[2]) for x in base_findings(i)}
    f = [x for x in G.check_spec(sp) if (x[1], x[2]) not in b]
    res[cls][1] += 1
    good = bool(f) if expect else (not f)
    if good:
        res[cls][0] += 1
        if SHOW:
            print(u"  [OK  ] %-3s %s%s" % (cls, tag, (u"  ⟵ " + f[0][1]) if f else u""))
    else:
        missed.append((cls, tag + ((u"  ⟵ " + f[0][1]) if f else u"")))
        if SHOW:
            print(u"  [%s] %-3s %s" % (u"놓침" if expect else u"오탐", cls, tag))


for i, sp0 in enumerate(D["specs"]):
    ps0 = (sp0.get("sig") or {}).get("params") or []
    if not ps0:
        continue
    head, body, lno = G.defhead(sp0)
    if head is None:
        continue
    args = G.split_args(head)
    mapping, _a, _f = G.align(sp0)
    nm = sp0["name"]
    use, outcall, spill, off0, off1, dyn = G.analyze(
        body, ["%%%d" % k for k in range(len(args))])

    for j, p0 in enumerate(ps0):
        role0 = p0.get("role") or u""
        rl = (mapping or {}).get(j) or []
        irtxt = u" ".join(args[int(r[1:])] for r in rl) if rl else u""

        # ── A. IR 에 없는 속성어를 덧붙인다 ────────────────────────────
        if rl:
            for at in ("readonly", "writeonly", "captures(none)", "nonnull"):
                if at in irtxt or at in role0:
                    continue
                sp = copy.deepcopy(sp0)
                sp["sig"]["params"][j]["role"] = role0 + u". 이 인자는 " + at + u" 다"
                run(i, sp, "A", u"%s p[%d] +%s" % (nm, j, at))
                break

        # ── B/C. 인용 줄번호·IR 조각 위조 ─────────────────────────────
        cl = G.cites(role0)
        qs = [(pos, q) for pos, q in G.backticks(role0) if G.is_ir_quote(q)]
        if cl and qs:
            c0 = cl[0]
            sp = copy.deepcopy(sp0)
            sp["sig"]["params"][j]["role"] = role0.replace(
                c0[5], u"%s:%d" % (c0[2], c0[3][0] + 3), 1)
            run(i, sp, "B", u"%s p[%d] %s +3줄" % (nm, j, c0[5]))

            for pos, q in qs:
                ints = re.findall(r"\d+", q)
                if not ints:
                    continue
                bad = q[::-1].replace(ints[-1][::-1], str(int(ints[-1]) + 7)[::-1], 1)[::-1]
                sp = copy.deepcopy(sp0)
                sp["sig"]["params"][j]["role"] = role0.replace(u"`" + q + u"`",
                                                               u"`" + bad + u"`", 1)
                run(i, sp, "C", u"%s p[%d] `%s` 위조" % (nm, j, G._norm(q)[:42]))
                break

        # ── D. 실제로 쓰이는 인자를 「전혀 안 씀」으로 위조 ────────────
        if rl and all((outcall[r] - spill[r]) > 0 for r in rl):
            sp = copy.deepcopy(sp0)
            sp["sig"]["params"][j]["role"] = u"본문에서 전혀 안 씀"
            run(i, sp, "D", u"%s p[%d] %s→전혀 안 씀" % (nm, j, p0.get("name")))

        # ── E. i 번호 어긋내기 ────────────────────────────────────────
        if isinstance(p0.get("i"), int):
            sp = copy.deepcopy(sp0)
            sp["sig"]["params"][j]["i"] = p0["i"] + 1
            run(i, sp, "E", u"%s p[%d] i=%s→%s" % (nm, j, p0["i"], p0["i"] + 1))

        # ── F. 행 삭제(자리 밀림) ─────────────────────────────────────
        if len(ps0) > 1:
            sp = copy.deepcopy(sp0)
            del sp["sig"]["params"][j]
            run(i, sp, "F", u"%s p[%d] 행 삭제" % (nm, j))

    for j, p0 in enumerate(ps0):
        role0 = p0.get("role") or u""
        rl = (mapping or {}).get(j) or []
        irtxt = u" ".join(args[int(r[1:])] for r in rl) if rl else u""

        # ── H1. **맞는** 속성 주장을 틀린 속성으로 바꿔치기 ────────────
        if rl:
            for at in ("readonly", "readnone", "writeonly"):
                if at not in role0 or at not in irtxt:
                    continue
                alt = next((a for a in ("readonly", "readnone", "writeonly")
                            if a != at and a not in irtxt), None)
                if not alt:
                    continue
                sp = copy.deepcopy(sp0)
                sp["sig"]["params"][j]["role"] = role0.replace(at, alt)
                run(i, sp, "H1", u"%s p[%d] %s→%s" % (nm, j, at, alt))
                break

        # ── H2. 인용 두 개의 줄번호를 서로 맞바꾼다 ────────────────────
        cl = G.cites(role0)
        if len(cl) >= 2 and [q for _p, q in G.backticks(role0) if G.is_ir_quote(q)]:
            a, b = cl[0], cl[1]
            if a[3][0] != b[3][0] and not a[4] and not b[4]:
                r2 = role0.replace(a[5], u"\x00", 1).replace(b[5], a[5], 1).replace(u"\x00", b[5], 1)
                sp = copy.deepcopy(sp0)
                sp["sig"]["params"][j]["role"] = r2
                run(i, sp, "H2", u"%s p[%d] %s↔%s 맞바꿈" % (nm, j, a[5], b[5]))

        # ── N1(음성 대조). 인용을 **더 줄여** 쓴다 — 잡히면 오탐 ───────
        for _pos, q in G.backticks(role0):
            if not G.is_ir_quote(q) or len(q) < 40:
                continue
            mid = re.search(r"(?<= )([A-Za-z_(),\[\] ]{12,40})(?= )", q[10:-10])
            if not mid:
                continue
            sp = copy.deepcopy(sp0)
            sp["sig"]["params"][j]["role"] = role0.replace(
                u"`" + q + u"`", u"`" + q.replace(mid.group(1), u"…", 1) + u"`", 1)
            run(i, sp, "N1", u"%s p[%d] 인용 축약" % (nm, j), expect=False)
            break

        # ── N2(음성 대조). 무해한 산문 덧붙이기 — 잡히면 오탐 ──────────
        sp = copy.deepcopy(sp0)
        sp["sig"]["params"][j]["role"] = role0 + u". 재구현 시 이 인자는 그대로 둔다"
        run(i, sp, "N2", u"%s p[%d] 산문 추가" % (nm, j), expect=False)

    # ── G. sret 표식 제거 ─────────────────────────────────────────────
    if (ps0[0].get("name") or u"").strip().startswith(u"(sret"):
        sp = copy.deepcopy(sp0)
        sp["sig"]["params"][0]["name"] = u"ret_slot"
        run(i, sp, "G", u"%s p[0] (sret)→ret_slot" % nm)

print(u"\n== 변이 시험 결과 (기준선 적발 0 · 변이본에서 새 적발이 나오면 「잡음」) ==")
tc = tt = 0
for k in sorted(res):
    c, t = res[k]
    tc += c
    tt += t
    print(u"  %-2s  포착 %3d / %3d  (%5.1f%%)" % (k, c, t, 100.0 * c / t if t else 0))
print(u"  %s\n  합계 포착 %d / %d (%.1f%%)" % (u"-" * 34, tc, tt, 100.0 * tc / tt if tt else 0))

if missed:
    print(u"\n-- 사각지대(놓친 변이) 상위 --")
    for cls, tag in missed[:40]:
        print(u"   %-2s %s" % (cls, tag))
