# -*- coding: utf-8 -*-
u"""D_mdlines — 모듈 전체 !DILocation 을 스코프 파일별로 집계.
사용: python -X utf8 D_mdlines.py <ll> <파일명조각> [<줄a> <줄b>]
"""
import io, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
path = sys.argv[1]
if not os.path.isabs(path):
    for base in (r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc'):
        p = os.path.join(base, path)
        if os.path.exists(p):
            path = p
            break
FILT = sys.argv[2]
A = int(sys.argv[3]) if len(sys.argv) > 4 else 0
B = int(sys.argv[4]) if len(sys.argv) > 4 else 10 ** 9

md = {}
rx = re.compile(r'^!(\d+) = (.*)$')
with io.open(path, encoding='utf-8', errors='replace') as f:
    for ln in f:
        if ln.startswith('!'):
            m = rx.match(ln.rstrip('\n'))
            if m:
                md[int(m.group(1))] = m.group(2)

RXFILE = re.compile(r'file:\s*!(\d+)')
RXFN = re.compile(r'filename:\s*"([^"]*)"')
cache = {}


def scope_file(sid, depth=0):
    if sid in cache:
        return cache[sid]
    r = '?'
    s = md.get(sid, '')
    if s.startswith('!DIFile'):
        m = RXFN.search(s)
        r = m.group(1) if m else '?'
    else:
        m = RXFILE.search(s)
        if m:
            r = scope_file(int(m.group(1)), depth + 1)
        else:
            m = re.search(r'scope:\s*!(\d+)', s)
            if m and depth < 12:
                r = scope_file(int(m.group(1)), depth + 1)
    cache[sid] = r
    return r


RXLOC = re.compile(r'^!DILocation\(line:\s*(\d+),.*?scope:\s*!(\d+)')
seen = {}
for k, v in md.items():
    m = RXLOC.match(v)
    if not m:
        continue
    line, sc = int(m.group(1)), int(m.group(2))
    fn = scope_file(sc)
    if FILT not in fn or not (A <= line <= B):
        continue
    seen.setdefault(line, []).append(k)
for line in sorted(seen):
    ids = seen[line]
    print(u"L%-6d n=%-4d ids=%s" % (line, len(ids), ",".join("!%d" % i for i in ids[:6])))
