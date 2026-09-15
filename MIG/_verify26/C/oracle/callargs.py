# -*- coding: utf-8 -*-
"""지정 줄의 call/invoke 인자를 하나씩 분해해 출력 (속성 제거). usage: python callargs.py 21664 21682 ..."""
import io, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
RAW = r"C:\tfm2mods\_gaibc\m04.ll"
raw = io.open(RAW, encoding="utf-8", errors="replace").read().split("\n")
ATTR = re.compile(r"\b(noalias|noundef|nonnull|readonly|readnone|zeroext|signext|align \d+|captures\([^)]*\)|dereferenceable(?:_or_null)?\(\d+\)|sret\([^)]*\)|writable|dead_on_unwind|nocapture|nofree|immarg|range\([^)]*\))\s*")
for a in sys.argv[1:]:
    l = raw[int(a) - 1]
    m = re.search(r"(@[\w.$]+|%[\w.]+)\((.*)\)(?:\s*$|,\s*!dbg|\s+to label|\s*#)", l)
    if not m:
        print(a, "no match"); continue
    args = m.group(2)
    # split top-level commas
    parts, depth, cur = [], 0, ""
    for ch in args:
        if ch in "([{": depth += 1
        if ch in ")]}": depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip(): parts.append(cur)
    print(f"== {a} {m.group(1)[:100]}")
    for i, p in enumerate(parts):
        print(f"  [{i}] {ATTR.sub('', p).strip()}")
