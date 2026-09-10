#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""!DILocation 체인을 inlinedAt 루트까지 펼친다.
사용: python dloc.py <ll파일> <id> [<id> ...]
각 단계의 line/scope 함수명/파일명을 출력."""
import io
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

path = sys.argv[1]
if not os.path.isabs(path):
    for base in (r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc'):
        if os.path.exists(os.path.join(base, path)):
            path = os.path.join(base, path)
            break
md = {}
rx = re.compile(r'^!(\d+) = (.*)$')
for ln in io.open(path, 'r', encoding='utf-8', errors='replace'):
    if ln.startswith('!'):
        m = rx.match(ln.rstrip('\n'))
        if m:
            md[int(m.group(1))] = m.group(2)


def fname(i, seen=None):
    seen = seen or set()
    while i is not None and i not in seen:
        seen.add(i)
        s = md.get(i, '')
        m = re.search(r'DISubprogram\(name: "([^"]+)"', s)
        if m:
            f = re.search(r'file: !(\d+)', s)
            l = re.search(r'line: (\d+)', s)
            return m.group(1), (f and int(f.group(1))), (l and l.group(1))
        m = re.search(r'scope: !(\d+)', s)
        i = int(m.group(1)) if m else None
    return '?', None, None


def filen(i):
    s = md.get(i, '')
    m = re.search(r'filename: "([^"]+)"', s)
    return m.group(1) if m else '?'


for a in sys.argv[2:]:
    i = int(a.lstrip('!'))
    print('--- !%d' % i)
    d = 0
    while i is not None:
        s = md.get(i, '')
        m = re.search(r'DILocation\(line: (\d+),.*?scope: !(\d+)(?:, inlinedAt: !(\d+))?', s)
        if not m:
            print('  %s(비-DILocation) %s' % ('  ' * d, s[:120]))
            break
        line, scope, ia = m.group(1), int(m.group(2)), m.group(3)
        nm, fi, _ = fname(scope)
        print('  %s line %s in %s  [%s]' % ('  ' * d, line, nm, filen(fi) if fi else '?'))
        d += 1
        i = int(ia) if ia else None
