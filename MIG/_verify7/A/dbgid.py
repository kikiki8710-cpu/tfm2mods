# -*- coding: utf-8 -*-
u"""!dbg 메타데이터 id 를 (파일:줄) inlinedAt 사슬로 풀어 찍는다.
사용: python -X utf8 dbgid.py m04.ll 56416 44402 ...
"""
import io, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')

f = sys.argv[1]
src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
meta = {}
for ln in src:
    if ln.startswith("!"):
        m = re.match(r"^!(\d+) = (.*)$", ln)
        if m:
            meta[m.group(1)] = m.group(2)


def scope_file(sid, d=0):
    cur = sid
    while cur and cur in meta and d < 40:
        t = meta[cur]
        m = re.search(r"file: !(\d+)", t)
        if m:
            fn = FILEOF.search(meta.get(m.group(1), ""))
            if fn:
                return fn.group(1).split("\\")[-1]
        m2 = re.search(r"scope: !(\d+)", t)
        if not m2:
            return "?"
        cur = m2.group(1)
        d += 1
    return "?"


def chain(n):
    out, cur, d = [], str(n), 0
    while cur and cur in meta and d < 48:
        m = LOC.search(meta[cur])
        if not m:
            break
        line, scope, inl = m.groups()
        out.append((scope_file(scope), int(line)))
        if not inl:
            break
        cur, d = inl, d + 1
    return out


for a in sys.argv[2:]:
    print(u"!%s = %s" % (a, u" <- ".join(u"%s:%d" % (fn, li) for fn, li in chain(a))))
