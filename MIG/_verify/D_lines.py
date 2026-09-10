# -*- coding: utf-8 -*-
u"""D_lines — IR 줄범위의 !dbg 를 inlinedAt 루트까지 펼쳐 (소스파일,줄) 분포를 낸다.
사용: python -X utf8 D_lines.py <ll> <from> <to> [필터파일명]
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
FR, TO = int(sys.argv[2]), int(sys.argv[3])
FILT = sys.argv[4] if len(sys.argv) > 4 else None

md = {}
body = []
rx = re.compile(r'^!(\d+) = (.*)$')
with io.open(path, encoding='utf-8', errors='replace') as f:
    for i, ln in enumerate(f, 1):
        if ln.startswith('!'):
            m = rx.match(ln.rstrip('\n'))
            if m:
                md[int(m.group(1))] = m.group(2)
        if FR <= i <= TO:
            body.append((i, ln.rstrip('\n')))

RXLOC = re.compile(r'line:\s*(\d+).*?scope:\s*!(\d+)(?:.*?inlinedAt:\s*!(\d+))?')
RXSP = re.compile(r'name:\s*"([^"]*)"')
RXFILE = re.compile(r'file:\s*!(\d+)')
RXFN = re.compile(r'filename:\s*"([^"]*)"')


def scope_file(sid, depth=0):
    while sid in md and depth < 12:
        s = md[sid]
        if s.startswith('!DIFile'):
            m = RXFN.search(s)
            return m.group(1) if m else '?'
        m = RXFILE.search(s)
        if m:
            return scope_file(int(m.group(1)), depth + 1)
        m = re.search(r'scope:\s*!(\d+)', s)
        if not m:
            return '?'
        sid = int(m.group(1))
        depth += 1
    return '?'


def root(lid):
    seen = 0
    cur = lid
    while seen < 40:
        s = md.get(cur, '')
        m = RXLOC.search(s)
        if not m:
            return None
        line, sc, inl = int(m.group(1)), int(m.group(2)), m.group(3)
        if inl:
            cur = int(inl)
            seen += 1
            continue
        return (scope_file(sc), line)
    return None


out = {}
for irln, txt in body:
    for m in re.finditer(r'!dbg !(\d+)', txt):
        r = root(int(m.group(1)))
        if not r:
            continue
        fn, line = r
        if FILT and FILT not in fn:
            continue
        out.setdefault((fn, line), []).append(irln)

for (fn, line) in sorted(out, key=lambda k: (k[0], k[1])):
    v = out[(fn, line)]
    print(u"%-40s L%-6d n=%-4d ir=%s" % (os.path.basename(fn), line, len(v),
                                         ",".join(str(x) for x in v[:8]) + (" …" if len(v) > 8 else "")))
