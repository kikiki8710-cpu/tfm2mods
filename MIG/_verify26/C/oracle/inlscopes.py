# -*- coding: utf-8 -*-
"""본문(19038~24726)의 모든 !dbg 사슬에서 인라인된 DISubprogram(name·linkageName·file:line) 집합을 뽑는다."""
import io, re, sys, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
RAW = r"C:\tfm2mods\_gaibc\m04.ll"
A, B = 19038, 24726
raw = io.open(RAW, encoding="utf-8", errors="replace").read().split("\n")
meta = {}
pat = re.compile(r"^!(\d+) = (.*)$")
for l in raw:
    if l.startswith("!"):
        m = pat.match(l)
        if m: meta[int(m.group(1))] = m.group(2)
files = {}
for k, v in meta.items():
    if v.startswith("!DIFile("):
        m = re.search(r'filename: "([^"]+)"', v); files[k] = m.group(1).replace("\\\\", "\\") if m else "?"
def scope_sp(sid):
    # climb lexical blocks to the DISubprogram
    seen = 0
    while sid in meta and seen < 20:
        v = meta[sid]
        if "DISubprogram" in v:
            nm = re.search(r'name: "([^"]+)"', v); ln = re.search(r'linkageName: "([^"]+)"', v)
            fl = re.search(r"file: !(\d+)", v); li = re.search(r"\bline: (\d+)", v)
            return (nm.group(1) if nm else "?", ln.group(1) if ln else "-", files.get(int(fl.group(1)), "?") if fl else "?", int(li.group(1)) if li else 0)
        m = re.search(r"scope: !(\d+)", v)
        if not m: return None
        sid = int(m.group(1)); seen += 1
    return None
cnt = collections.Counter()
for i in range(A, B + 1):
    for m in re.finditer(r"!dbg !(\d+)", raw[i - 1]):
        did = int(m.group(1)); hops = 0
        while did in meta and hops < 64:
            v = meta[did]
            ms = re.search(r"scope: !(\d+)", v)
            if ms:
                sp = scope_sp(int(ms.group(1)))
                if sp: cnt[sp] += 1
            mi = re.search(r"inlinedAt: !(\d+)", v)
            if not mi: break
            did = int(mi.group(1)); hops += 1
for (nm, ln, fl, li), c in sorted(cnt.items(), key=lambda x: (x[0][2], x[0][3])):
    if "game-ai" in fl or "game-core" in fl:
        print(f"{c:5d}  {fl}:{li}  {nm}  {ln[:110]}")
