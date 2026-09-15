import io, re, sys, json
SRC = r"C:\tfm2mods\_gaibc\m15.ll"
OUT = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25D"
lines = io.open(SRC, encoding="utf-8", errors="replace").read().split("\n")
print("total lines", len(lines))
a, b = 12793, 18168
with io.open(OUT + r"\fn.ll", "w", encoding="utf-8") as f:
    for i in range(a - 1, b):
        f.write("%d| %s\n" % (i + 1, lines[i]))
# collect all !dbg metadata ids used in the function, then dump their DILocation lines
ids = set()
for i in range(a - 1, b):
    for m in re.finditer(r"!dbg !(\d+)", lines[i]):
        ids.add(int(m.group(1)))
print("dbg ids", len(ids))
# metadata index: lines starting with !N =
meta = {}
for i, l in enumerate(lines):
    if l.startswith("!") and " = " in l:
        m = re.match(r"!(\d+) = ", l)
        if m:
            meta[int(m.group(1))] = l
print("meta count", len(meta))
json.dump({k: meta[k] for k in meta}, io.open(OUT + r"\meta.json", "w", encoding="utf-8"))
# resolve each dbg id to root line through inlinedAt chain
def parse(l):
    d = {}
    for m in re.finditer(r"(\w+): (!?\w+)", l):
        d[m.group(1)] = m.group(2)
    return d
def chain(idn):
    out = []
    cur = idn
    seen = 0
    while cur is not None and seen < 64:
        seen += 1
        l = meta.get(cur, "")
        d = parse(l)
        line = d.get("line")
        scope = d.get("scope")
        sc = meta.get(int(scope[1:]), "") if scope else ""
        # scope name/file
        sd = parse(sc)
        nm = ""
        m = re.search(r'name: "([^"]*)"', sc)
        if m: nm = m.group(1)
        # find file of scope
        out.append((line, nm))
        ia = d.get("inlinedAt")
        cur = int(ia[1:]) if ia else None
    return out
res = {}
for k in sorted(ids):
    res[k] = chain(k)
json.dump(res, io.open(OUT + r"\dbgchain.json", "w", encoding="utf-8"))
# print define line
print(lines[a - 1][:400])
