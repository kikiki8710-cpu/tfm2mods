# -*- coding: utf-8 -*-
u"""담당 .rs 의 특정 줄이 사슬에 들어 있는 IR 명령을 전부 찍는다.
사용: python -X utf8 atline.py <m10.ll> <from> <to> <파일명.rs> <줄> [<줄> ...]
"""
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
f, a, b, own = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
want = set(int(x) for x in sys.argv[5:])
src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
meta = {}
for ln in src:
    if ln.startswith("!"):
        m = re.match(r"^!(\d+) = (.*)$", ln)
        if m:
            meta[m.group(1)] = m.group(2)
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')
DBG = re.compile(r"!dbg !(\d+)")


def sf(sid, d=0):
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
    out, cur, d = [], n, 0
    while cur and cur in meta and d < 48:
        m = LOC.search(meta[cur])
        if not m:
            break
        line, scope, inl = m.groups()
        out.append((sf(scope), int(line)))
        if not inl:
            break
        cur, d = inl, d + 1
    return out


for k in range(a - 1, min(b, len(src))):
    ln = src[k]
    if "#dbg_" in ln:
        continue
    m = DBG.search(ln)
    if not m:
        continue
    ch = chain(m.group(1))
    hit = [li for (fn, li) in ch if fn == own and li in want]
    if hit:
        print(u"IR:%-7d %-8s %s" % (k + 1, "L" + ",".join(str(x) for x in sorted(set(hit))), ln.strip()[:150]))
        print(u"          사슬 = %s" % (" <- ".join("%s:%d" % c for c in ch)))
