# -*- coding: utf-8 -*-
u"""**G18 변이 시험** — 적발 0/1 이 *무능*이 아님을 보이는 반증 장치. (`_verify8/A/mutate.py` 본)

G18 이 잡으려는 실패 모드는 **「logic 이 인용하는데 표에 행이 없다」** 하나다.
그래서 변이도 그것이어야 한다 — **표에서 행을 하나 지운다.**

- `--del`  : `mem` 행을 하나씩 지우고, 그 행이 덮던 인용을 되잡는지 센다(기본).
- `--off`  : `mem[].offset` 을 **표에 없는 값**으로 바꿔 넣고 되잡는지 센다.
- 「못 잡음」은 **사각지대 목록**이다 — 그 행을 `logic` 이 애초에 인용하지 않았다면
  게이트가 못 잡는 게 **정상**이므로, `logic` 이 인용하는 행만 분모로 센다(`--strict`).

사용: `python -X utf8 mutate.py [--del|--off] [--strict]`
"""
import io, json, os, re, sys, copy, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
MODE = "off" if "--off" in sys.argv else "del"
STRICT = "--strict" in sys.argv


def cited_offsets(sp):
    lg = G.STRIKE.sub(u" ", sp.get("logic") or u"")
    s = {int((a or b), 16) for a, b in G.CITE.findall(lg)}
    s |= {int(m.group(2), 16) for m in G.TYOFF.finditer(lg)}
    return s


def cited_names(sp):
    lg = G.STRIKE.sub(u" ", sp.get("logic") or u"")
    return {m.group(2).split(u".")[-1] for m in G.FLD.finditer(lg)}


caught = miss = 0
misses = []
per = {}
for i, sp0 in enumerate(D["specs"]):
    mem = sp0.get("mem") or []
    base0 = len(G.check_spec(sp0))
    co, cn = cited_offsets(sp0), cited_names(sp0)
    c = m = 0
    for j, row in enumerate(mem):
        mm = re.match(r"^\s*0x([0-9a-fA-F]+)\s*$", str(row.get("offset") or u""))
        if not mm:
            continue
        off = int(mm.group(1), 16)
        nm = str(row.get("name") or u"")
        # ★분모 — `logic` 이 그 행을 **인용하고 있는** 행만. 인용이 없으면 지워도
        #   게이트가 잡을 근거가 없고, 그건 사각지대가 아니라 **적용 범위 밖**이다.
        if STRICT and off not in co and not any(
                re.search(r"(?<![A-Za-z0-9_])%s(?![A-Za-z0-9_])" % re.escape(t), u" ".join(cn))
                for t in re.findall(r"[a-z_][a-z0-9_]{3,}", nm)):
            continue
        sp = copy.deepcopy(sp0)
        if MODE == "del":
            del sp["mem"][j]
        else:
            sp["mem"][j]["offset"] = u"0x%x" % (off ^ 0x4000)
        if len(G.check_spec(sp)) > base0:
            c += 1
        else:
            m += 1
            misses.append((i, sp0["name"], j, row.get("base"), row.get("offset"), nm[:34]))
    per[i] = (c, m)
    caught += c
    miss += m

print(u"== G18 변이 시험 (%s · 분모=%s) ==" % (
    u"행 삭제" if MODE == "del" else u"offset 교란", u"logic 인용 행만" if STRICT else u"전 행"))
for i in sorted(per):
    c, m = per[i]
    if c or m:
        print(u"  specs[%-2d] 포착 %-3d / 놓침 %-3d  (%s)"
              % (i, c, m, D["specs"][i]["name"]))
t = caught + miss
print(u"\n---- 포착 %d / %d = **%.1f%%**" % (caught, t, 100.0 * caught / t if t else 0))
if "--miss" in sys.argv:
    print(u"\n-- 못 잡은 행(사각지대)")
    for r in misses[:80]:
        print(u"   %02d %-26s mem[%-2d] %-30s %-8s %s" % r)
