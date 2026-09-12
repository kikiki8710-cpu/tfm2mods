# -*- coding: utf-8 -*-
u"""G20 추가 규칙 후보 2종을 20명세 전역에 시험 적용한다(제안 근거용, 정본 수정 없음).

R5  타입 크기 — `Ty(NNNB)` 표기가 명세마다 다르면 한쪽이 틀렸다.
R6  열거형 태그 — `Ty::Variant` 와 `consts.value` 의 짝이 명세마다 다르면 한쪽이 틀렸다.
"""
import io, json, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]

SZ = re.compile(r"\b([A-Z][A-Za-z0-9_]*(?:<[^>]{0,40}>)?)\s*\(\s*(\d{2,6})\s*B\s*\)")
VAR = re.compile(r"\b([A-Z][A-Za-z0-9_]*)::([A-Z][A-Za-z0-9_]*)\b")


def texts(sp):
    out = []
    for k in ("one_line", "logic"):
        if sp.get(k):
            out.append((k, sp[k]))
    for arr in ("mem", "consts", "knobs"):
        for j, r in enumerate(sp.get(arr) or []):
            out.append(("%s[%d]" % (arr, j), u" ".join(
                str(r.get(x) or u"") for x in ("base", "name", "note", "meaning", "what", "where", "value", "effect"))))
    for j, p in enumerate(((sp.get("sig") or {}).get("params") or [])):
        out.append(("sig.params[%d]" % j, u"%s %s" % (p.get("ty"), p.get("role"))))
    return out


sizes = collections.defaultdict(set)
for i, sp in enumerate(D):
    for where, t in texts(sp):
        for m in SZ.finditer(t):
            sizes[m.group(1)].add((int(m.group(2)), i, where))
print(u"== R5 타입 크기 불일치 ==")
n5 = 0
for ty, s in sorted(sizes.items()):
    vals = sorted(set(x[0] for x in s))
    specs = set(x[1] for x in s)
    if len(vals) > 1 and len(specs) > 1:
        n5 += 1
        print(u"  %-28s %s" % (ty, vals))
        for (v, i, w) in sorted(s):
            print(u"      %-7d [%02d] %s" % (v, i, w))
print(u"  총 %d건" % n5)

print()
print(u"== R6 열거형 태그값 불일치 (consts) ==")
tag = collections.defaultdict(set)
for i, sp in enumerate(D):
    for j, c in enumerate(sp.get("consts") or []):
        v = c.get("value")
        mm = c.get("meaning") or u""
        if not isinstance(v, int):
            continue
        for m in VAR.finditer(mm):
            tag[(m.group(1), m.group(2))].add((v, i, j))
n6 = 0
for k, s in sorted(tag.items()):
    vals = sorted(set(x[0] for x in s))
    specs = set(x[1] for x in s)
    if len(vals) > 1 and len(specs) > 1:
        n6 += 1
        print(u"  %s::%s -> %s" % (k[0], k[1], vals))
        for (v, i, j) in sorted(s):
            print(u"      %-6d [%02d] consts[%d]" % (v, i, j))
print(u"  총 %d건" % n6)
