"""sretstores.py <file> <define-regex> [%argname] — list stores/memcpy/memset into a pointer arg (default %0) of a define, with byte offsets."""
import io, re, sys
path = sys.argv[1]; pat = re.compile(sys.argv[2]); base = sys.argv[3] if len(sys.argv) > 3 else "%0"
txt = io.open(path, encoding="utf-8", errors="replace").read().split("\n")
TYSZ = {"i8": 1, "i16": 2, "i32": 4, "i64": 8, "ptr": 8, "i1": 1, "double": 8, "float": 4, "i128": 16}
def body(i):
    j = i
    while j < len(txt) and txt[j] != "}":
        j += 1
    return txt[i:j + 1], i
for i, l in enumerate(txt):
    if l.startswith("define") and pat.search(l):
        print("=== %d| %s" % (i + 1, l[:200]))
        m = re.search(r"initializes\(\(([^)]*\)(?:, \([^)]*\))*)\)", l)
        print("  initializes:", m.group(0) if m else "(none)")
        B, off0 = body(i)
        # alias map: reg -> (basereg, offset)
        alias = {base: (base, 0)}
        writes = []
        for k, s in enumerate(B):
            ln = off0 + k + 1
            m = re.match(r"\s*(%[\w.]+) = getelementptr (?:inbounds )?(?:nuw )?i8, ptr (%[\w.]+), i64 (-?\d+)", s)
            if m and m.group(2) in alias:
                b, o = alias[m.group(2)]
                alias[m.group(1)] = (b, o + int(m.group(3)))
                continue
            m = re.match(r"\s*store (?:volatile )?(\w+) ([^,]+), ptr (%[\w.]+)", s)
            if m and m.group(3) in alias:
                b, o = alias[m.group(3)]
                sz = TYSZ.get(m.group(1))
                if sz is None:
                    mm = re.match(r"<(\d+) x (\w+)>", m.group(1))
                    sz = int(mm.group(1)) * TYSZ.get(mm.group(2), 0) if mm else 0
                writes.append((o, o + sz, "store %s %s" % (m.group(1), m.group(2)[:40]), ln))
                continue
            m = re.search(r"llvm\.mem(cpy|set)\.[\w.]*\(ptr [^%]*(%[\w.]+), (?:ptr [^%@]*(%[\w.]+|@[\w.$]+), )?(?:i8 (-?\d+), )?i64 (\d+)", s)
            if m and m.group(2) in alias:
                b, o = alias[m.group(2)]
                n = int(m.group(5))
                writes.append((o, o + n, "mem%s %s %s" % (m.group(1), m.group(3) or "", m.group(4) or ""), ln))
                continue
            # pass-through of alias pointer to a call (sret or arg): report
            m = re.search(r"(?:call|invoke) [^@]*@([\w.$]+)\((.*)$", s)
            if m:
                for a in re.findall(r"ptr (?:[\w() ,]+ )?(%[\w.]+)", m.group(2)):
                    if a in alias:
                        b, o = alias[a]
                        writes.append((o, None, "→call %s (ptr %s)" % (m.group(1)[:90], a), ln))
        for w in sorted(writes, key=lambda x: (x[0], x[3])):
            print("  [%d..%s) %s   @%d" % (w[0], w[1], w[2], w[3]))
