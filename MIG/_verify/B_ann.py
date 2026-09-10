# -*- coding: utf-8 -*-
u"""B_ann — IR 구간을 소스줄(인라인 루트 + 체인)과 함께 덤프.
사용: python -X utf8 B_ann.py <ll> <from> <to> [--all]
"""
import io, os, re, sys
try:
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

path, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
showall = "--all" in sys.argv

body = []
for i, l in enumerate(io.open(path, encoding="utf-8", errors="ignore"), 1):
    if a <= i <= b:
        body.append((i, l.rstrip()))
    if i > b:
        break

md = {}
files = {}
for l in io.open(path, encoding="utf-8", errors="ignore"):
    if not l.startswith("!"):
        continue
    m = re.match(r"!(\d+) = (.*)", l)
    if not m:
        continue
    s = m.group(2)
    if s.startswith("!DILocation") or "DISubprogram" in s or s.startswith("!DILexicalBlock") or s.startswith("distinct !DILexicalBlock"):
        md[m.group(1)] = s
    elif s.startswith("!DIFile"):
        fn = re.search(r'filename: "([^"]*)"', s)
        files[m.group(1)] = fn.group(1) if fn else "?"


def scope_file(sid, depth=0):
    s = md.get(sid)
    if not s or depth > 12:
        return "?"
    f = re.search(r"file: !(\d+)", s)
    if f:
        return files.get(f.group(1), "?").split("\\")[-1]
    sc = re.search(r"scope: !(\d+)", s)
    return scope_file(sc.group(1), depth + 1) if sc else "?"


def chain(mid, depth=0):
    s = md.get(mid)
    if not s or depth > 16:
        return []
    ln = re.search(r"line: (\d+)", s)
    sc = re.search(r"scope: !(\d+)", s)
    out = ["%s:%s" % (scope_file(sc.group(1)) if sc else "?", ln.group(1) if ln else "?")]
    ia = re.search(r"inlinedAt: !(\d+)", s)
    if ia:
        out += chain(ia.group(1), depth + 1)
    return out


for i, l in body:
    s = l.strip()
    if not showall and (s.startswith("#dbg") or "noalias.scope.decl" in s):
        continue
    ms = re.findall(r"!dbg !(\d+)", l)
    ann = ""
    if ms:
        c = chain(ms[0])
        ann = "   ; " + " <= ".join(c)
    print(u"%d %s%s" % (i, s[:150], ann))
