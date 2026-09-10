#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""!dbg 노드의 inlinedAt 사슬을 전부 펼친다.  python dbgchain.py <ll> <mdid> [<mdid>...]"""
import io
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

path = sys.argv[1]
src = io.open(path, encoding='utf-8', errors='replace').read().split('\n')
md = {}
for ln in src:
    m = re.match(r'^!(\d+) = (.*)$', ln)
    if m:
        md[int(m.group(1))] = m.group(2)


def f(t, k):
    m = re.search(r'\b%s: !(\d+)' % k, t)
    return int(m.group(1)) if m else None


def num(t, k):
    m = re.search(r'\b%s: (\d+)' % k, t)
    return int(m.group(1)) if m else None


def spname(sid):
    seen = 0
    while sid is not None and seen < 40:
        seen += 1
        t = md.get(sid, '')
        if 'DISubprogram' in t:
            n = re.search(r'name: "([^"]*)"', t)
            fl = f(t, 'file')
            fn = re.search(r'filename: "([^"]*)"', md.get(fl, '')) if fl else None
            return (n.group(1) if n else '?'), (fn.group(1) if fn else '?')
        sid = f(t, 'scope')
    return '?', '?'


for a in sys.argv[2:]:
    cur = int(a.lstrip('!'))
    print('--- !%d ---' % cur)
    d = 0
    while cur is not None and d < 60:
        t = md.get(cur, '')
        nm, fl = spname(f(t, 'scope'))
        print('  %s!%-8d line %-6s  %s   [%s]' % ('  ' * d, cur, num(t, 'line'), nm, fl))
        cur = f(t, 'inlinedAt')
        d += 1
