# -*- coding: utf-8 -*-
"""m04.ll 44657~44931 원문 줄 + !dbg 사슬 해석 덤프."""
import re, sys, io, os
P = os.environ.get("LL", r"C:\tfm2mods\_gaibc\m04.ll")
txt = io.open(P, encoding="utf-8", errors="replace", newline="").read()
lines = txt.split("\n")
# 메타데이터 사전
md = {}
for ln in lines:
    if ln.startswith("!") and " = " in ln:
        k, v = ln.split(" = ", 1)
        md[k] = v
def loc(n, depth=0):
    v = md.get(n)
    if v is None: return "?"
    m = re.match(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: (!\d+)(?:, inlinedAt: (!\d+))?", v)
    if not m: return v[:60]
    line, scope, ia = m.group(1), m.group(2), m.group(3)
    # scope 의 파일
    s = md.get(scope, "")
    f = ""
    mm = re.search(r"file: (!\d+)", s)
    if mm:
        fv = md.get(mm.group(1), "")
        fm = re.search(r'filename: "([^"]+)"', fv)
        if fm: f = fm.group(1).split("\\")[-1].split("/")[-1]
    out = f"{f}:{line}"
    if ia and depth < 10: out += " < " + loc(ia, depth+1)
    return out
a, b = int(sys.argv[1]), int(sys.argv[2])
for i in range(a-1, b):
    ln = lines[i]
    m = re.search(r"!dbg (!\d+)", ln)
    core = re.sub(r", !dbg !\d+", "", ln)
    core = re.sub(r", !(noundef|nonnull|align|range|noalias|alias\.scope|nonnull|dereferenceable)[^,]*(?:, )?", ", !!", core)
    tag = ("   ;; " + loc(m.group(1))) if m else ""
    print(f"{i+1}| {core[:230]}{tag}")
