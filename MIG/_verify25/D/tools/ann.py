import io, re, sys, json
OUT = r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad25D"
ANN = r"C:\tfm2mods\MIG\_next\reach\eaeda0.ll"
ann = {}
prev = None
for l in io.open(ANN, encoding="utf-8", errors="replace"):
    m = re.match(r"\s*(\d+)\|(.*)$", l.rstrip("\n"))
    if not m: continue
    n = int(m.group(1)); body = m.group(2)
    c = None
    mm = re.search(r";L([^\s]+)\s*$", body)
    if mm: c = mm.group(1)
    ann[n] = (body, c)
# fill chain for invoke lines from the following 'to label' line
keys = sorted(ann)
for i, n in enumerate(keys):
    body, c = ann[n]
    if c is None and i + 1 < len(keys):
        nb, nc = ann[keys[i + 1]]
        if nb.strip().startswith("to label") and nc:
            ann[n] = (body, nc)
orig = {}
for l in io.open(OUT + r"\fn.ll", encoding="utf-8"):
    m = re.match(r"(\d+)\| (.*)$", l.rstrip("\n"))
    if m: orig[int(m.group(1))] = m.group(2)
def root(c):
    if not c: return None
    return c.split("<")[-1]
mode = sys.argv[1]
if mode == "grep":
    pat = re.compile(sys.argv[2])
    w = int(sys.argv[3]) if len(sys.argv) > 3 else 200
    for n in keys:
        body, c = ann[n]
        if pat.search(body) or pat.search(orig.get(n, "")):
            print("%d| %s   ;L%s" % (n, orig.get(n, body)[:w], c))
elif mode == "range":
    a, b = int(sys.argv[2]), int(sys.argv[3])
    w = int(sys.argv[4]) if len(sys.argv) > 4 else 200
    for n in range(a, b + 1):
        if n in ann:
            body, c = ann[n]
            print("%d| %s   ;L%s" % (n, body[:w], c))
        elif n in orig:
            print("%d| %s   ;(unann)" % (n, orig[n][:w]))
elif mode == "roots":
    # histogram of root lines
    from collections import Counter
    cnt = Counter(root(c) for n in keys for _, c in [ann[n]] if c)
    for k in sorted(cnt, key=lambda x: int(x) if x and x.isdigit() else -1):
        print(k, cnt[k])
elif mode == "const":
    # find lines where literal value appears as an operand
    v = sys.argv[2]
    pat = re.compile(r"(?<![\w.%!-])" + re.escape(v) + r"(?![\w.])")
    for n in keys:
        body, c = ann[n]
        o = orig.get(n, body)
        if pat.search(o) and not o.strip().startswith(";") and "!dbg" in o or (pat.search(o) and "to label" in orig.get(n+1, "")):
            # ignore dbg metadata ids
            print("%d| %s   ;L%s" % (n, o[:180], c))
