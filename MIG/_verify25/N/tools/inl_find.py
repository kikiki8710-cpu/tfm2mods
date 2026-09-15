# -*- coding: utf-8 -*-
"""<file.ll> <subprogram name> [max]: 그 DISubprogram(인라인 포함) scope 에 귀속된 명령 줄을 찾아 앞뒤 문맥과 함께 출력.
   파이썬 통째 로드(sed 루프 금지)."""
import re, sys, io
P, NAME = sys.argv[1], sys.argv[2]
MAX = int(sys.argv[3]) if len(sys.argv) > 3 else 3
lines = io.open(P, encoding="utf-8", errors="replace", newline="").read().split("\n")
md = {}
for ln in lines:
    if ln.startswith("!") and " = " in ln:
        k, v = ln.split(" = ", 1); md[k] = v
# subprogram ids with that name
sps = set(k for k, v in md.items() if v.startswith("distinct !DISubprogram(name: \"%s\"" % NAME) or v.startswith("!DISubprogram(name: \"%s\"" % NAME))
print("subprograms", len(sps))
# scope -> subprogram resolution (lexical blocks)
cache = {}
def sp_of(scope):
    if scope in cache: return cache[scope]
    v = md.get(scope, "")
    if scope in sps: r = scope
    elif v.startswith("!DILexicalBlock") or v.startswith("distinct !DILexicalBlock"):
        m = re.search(r"scope: (!\d+)", v); r = sp_of(m.group(1)) if m else None
    else: r = None
    cache[scope] = r; return r
def chain(locid):
    out = []
    while locid:
        v = md.get(locid, "")
        m = re.match(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: (!\d+)(?:, inlinedAt: (!\d+))?", v)
        if not m: break
        s = md.get(m.group(2), ""); nm = re.search(r'name: "([^"]+)"', s)
        sp = sp_of(m.group(2))
        spn = re.search(r'name: "([^"]+)"', md.get(sp, "")).group(1) if sp else "?"
        out.append(f"{spn}:{m.group(1)}")
        locid = m.group(3)
    return " < ".join(out)
locs = {}
for k, v in md.items():
    if v.startswith("!DILocation"):
        m = re.match(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: (!\d+)", v)
        if m and sp_of(m.group(2)) in sps: locs[k] = m.group(1)
print("locations", len(locs))
hits = 0
for i, ln in enumerate(lines):
    if ln.startswith("!") or ln.startswith(";"): continue
    m = re.search(r"!dbg (!\d+)", ln)
    if m and m.group(1) in locs and "#dbg_value" not in ln:
        hits += 1
        if hits <= MAX:
            print(f"--- {i+1}: {chain(m.group(1))}")
            for j in range(max(0, i-2), min(len(lines), i+6)):
                l2 = lines[j]
                mm = re.search(r"!dbg (!\d+)", l2)
                c = chain(mm.group(1)) if mm else ""
                l3 = re.sub(r', !dbg !\d+', '', l2)[:150]
                print(f"  {j+1}| {l3}   ;; {c}")
print("hits", hits)
