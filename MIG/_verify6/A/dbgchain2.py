# -*- coding: utf-8 -*-
u"""!dbg !N 의 **inlinedAt 사슬 전체**를 (파일:줄) 로 펼친다.
`consts[].src_line` 이 사슬 어디에도 없으면 그 src_line 은 **틀린 것**이다.
사용: python -X utf8 dbgchain2.py <m10.ll> <!N> [<!N> ...]
"""
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
f = sys.argv[1]
ids = sys.argv[2:]
lines = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
meta = {}
mre = re.compile(r"^!(\d+) = (.*)$")
for ln in lines:
    if ln.startswith("!"):
        m = mre.match(ln)
        if m:
            meta[m.group(1)] = m.group(2)
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: (\d+))?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')


def scope_file(sid, depth=0):
    cur = sid
    while cur and cur in meta and depth < 40:
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
        depth += 1
    return "?"


for n in ids:
    n = n.lstrip("!")
    print(u"\n== !%s 사슬 ==" % n)
    cur, d = n, 0
    while cur and cur in meta and d < 40:
        m = LOC.search(meta[cur])
        if not m:
            print(u"   !%s  (DILocation 아님: %s)" % (cur, meta[cur][:90])); break
        line, col, scope, inl = m.groups()
        print(u"   [%d] !%-7s %s:L%-6s col=%s" % (d, cur, scope_file(scope), line, col or "0"))
        if not inl:
            break
        cur, d = inl, d + 1
