# -*- coding: utf-8 -*-
u"""`_rank_callees` 의 `all(seg in sym)` 앵커를 **길이접두 경계**로 다시 판정해
거짓 앵커(ev3 인데 실제로는 그 심볼이 아닌 것)와 놓친 앵커를 센다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
IRDIR = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"]
_CALLSYM = re.compile(r"@(_R[\w.$]+)")


def mangled_names(sym):
    out = []
    for i in range(len(sym)):
        if not sym[i].isdigit() or (i and sym[i - 1].isdigit()):
            continue
        j = i
        while j < len(sym) and sym[j].isdigit():
            j += 1
        n = int(sym[i:j])
        if 0 < n <= 120 and j + n <= len(sym):
            w = sym[j:j + n]
            if re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", w):
                out.append(w)
    return out


def irsyms(sp):
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f:
        return []
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    out = set()
    for ln in src[a - 1:b]:
        if "call" not in ln and "invoke" not in ln:
            continue
        out.update(_CALLSYM.findall(ln))
    return sorted(out)


SEGOK = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
falseanch = []
missanch = []
for i, sp in enumerate(D):
    syms = irsyms(sp)
    toks = [set(mangled_names(s)) for s in syms]
    for j, c in enumerate(sp.get("callees") or []):
        segs = [s for s in (c.get("path") or u"").split("::") if s]
        good = [s for s in segs if SEGOK.match(s)]
        loose = bool(segs) and any(all(s in sym for s in segs) for sym in syms)
        strict = bool(good) and any(all(s in t for s in good) for t in toks)
        if loose and not strict:
            falseanch.append((i, j, c["name"], c["path"]))
        if strict and not loose:
            missanch.append((i, j, c["name"], c["path"], [s for s in segs if not SEGOK.match(s)]))
print(u"거짓 앵커(ev3 도장인데 길이접두 경계로는 불일치) %d건" % len(falseanch))
for r in falseanch:
    print(u"  [%02d] callees[%d] %-24s %s" % r)
print()
print(u"놓친 앵커(ev4 인데 실제로는 IR 에 있다) %d건" % len(missanch))
for r in missanch:
    print(u"  [%02d] callees[%d] %-24s %s   막은 seg=%s" % r)
