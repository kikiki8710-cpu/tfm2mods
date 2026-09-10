#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""함수 본문에서 특정 SSA 인자(기본 %1 = version)에 대한 비교를 전부 뽑고
각 비교의 !dbg 를 inlinedAt 루트까지 펴서 소스 줄을 붙인다.

사용: python vbr.py <ll> <define줄번호> [%1]
"""
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
start = int(sys.argv[2])
arg = sys.argv[3] if len(sys.argv) > 3 else '%1'

lines = io.open(path, 'r', encoding='utf-8', errors='replace').read().split('\n')
md = {}
for ln in lines:
    if ln.startswith('!'):
        m = re.match(r'^!(\d+) = (.*)$', ln)
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
            return m.group(1), (f and int(f.group(1)))
        m = re.search(r'scope: !(\d+)', s)
        i = int(m.group(1)) if m else None
    return '?', None


def filen(i):
    s = md.get(i, '')
    m = re.search(r'filename: "([^"]+)"', s)
    return m.group(1).split('\\\\')[-1] if m else '?'


def chain(i):
    out = []
    while i is not None:
        s = md.get(i, '')
        m = re.search(r'DILocation\(line: (\d+),.*?scope: !(\d+)(?:, inlinedAt: !(\d+))?', s)
        if not m:
            break
        nm, fi = fname(int(m.group(2)))
        out.append('%s:%s@%s' % (filen(fi), m.group(1), nm))
        i = int(m.group(3)) if m.group(3) else None
    return ' <- '.join(out)


end = start
while end < len(lines) and lines[end].rstrip() != '}':
    end += 1

# 인자 별칭 추적 (phi/select/trunc/zext 등)
alias = {arg}
body = lines[start - 1:end + 1]
pat = re.compile(r'(?:^|[^%\w])' + re.escape(arg) + r'(?![0-9])')
for it in range(3):
    for L in body:
        m = re.match(r'\s*(%\w+) = (?:zext|sext|trunc|freeze|phi[^ ]*)\s.*', L)
        if m and any(re.search(r'(?:^|[^%\w])' + re.escape(a) + r'(?![0-9])', L) for a in alias):
            alias.add(m.group(1))

n = 0
for k, L in enumerate(body):
    if ' = icmp ' not in L and not L.strip().startswith('switch'):
        continue
    if not any(re.search(r'(?:^|[^%\w])' + re.escape(a) + r'(?![0-9])', L) for a in alias):
        continue
    n += 1
    d = re.search(r'!dbg !(\d+)', L)
    print('L%d  %s' % (start - 1 + k + 1, L.strip()[:180]))
    if d:
        print('      %s' % chain(int(d.group(1))))
print('--- %d 비교 (alias=%s)' % (n, ','.join(sorted(alias))))
