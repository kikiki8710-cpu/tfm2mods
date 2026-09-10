#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""IR 줄범위를 '게임 소스 줄' 로 접어 보여준다.
!dbg 체인을 inlinedAt 루트까지 타면서 **game-ai/game-core/game-view 파일**의
가장 안쪽 프레임을 골라 붙인다(라이브러리 프레임은 버린다).

사용: python srcmap.py <ll> <시작> <끝> [필터정규식]
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
a, b = int(sys.argv[2]), int(sys.argv[3])
filt = re.compile(sys.argv[4]) if len(sys.argv) > 4 else None

lines = io.open(path, 'r', encoding='utf-8', errors='replace').read().split('\n')
md = {}
for ln in lines:
    if ln.startswith('!'):
        m = re.match(r'^!(\d+) = (.*)$', ln)
        if m:
            md[int(m.group(1))] = m.group(2)

_sp = {}


def sp(i, seen=None):
    if i in _sp:
        return _sp[i]
    seen = seen or set()
    j = i
    while j is not None and j not in seen:
        seen.add(j)
        s = md.get(j, '')
        m = re.search(r'DISubprogram\(name: "([^"]+)"', s)
        if m:
            f = re.search(r'file: !(\d+)', s)
            r = (m.group(1).split('<')[0], int(f.group(1)) if f else -1)
            _sp[i] = r
            return r
        m = re.search(r'scope: !(\d+)', s)
        j = int(m.group(1)) if m else None
    _sp[i] = ('?', -1)
    return _sp[i]


_fn = {}


def filen(i):
    if i in _fn:
        return _fn[i]
    s = md.get(i, '')
    m = re.search(r'filename: "([^"]+)"', s)
    _fn[i] = m.group(1).replace('\\\\', '/') if m else '?'
    return _fn[i]


GAME = re.compile(r'game-(ai|core|view)')


def loc(i):
    frames = []
    while i is not None:
        s = md.get(i, '')
        m = re.search(r'DILocation\(line: (\d+),.*?scope: !(\d+)(?:, inlinedAt: !(\d+))?', s)
        if not m:
            break
        nm, fi = sp(int(m.group(2)))
        frames.append((filen(fi), m.group(1), nm))
        i = int(m.group(3)) if m.group(3) else None
    for f in frames:
        if GAME.search(f[0]):
            return '%s:%s@%s' % (f[0].split('/')[-1], f[1], f[2])
    return frames[0][0].split('/')[-1] + ':' + frames[0][1] if frames else '?'


prev = None
for k in range(a - 1, min(b, len(lines))):
    L = lines[k]
    if '#dbg' in L:
        continue
    if filt and not filt.search(L):
        continue
    m = re.search(r'!dbg !(\d+)', L)
    tag = loc(int(m.group(1))) if m else ''
    print('%-28s L%-7d %s' % (tag, k + 1, L.strip()[:170]))
