#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""IR 구간을 소스 줄번호로 주석해 출력.  python llann.py <ll> <from> <to> [--root]

각 줄의 `!dbg !N` 을 (scope 함수, line) 으로 풀어 앞에 붙인다.
--root 를 주면 inlinedAt 루트(호출자 줄)도 같이 표시.
"""
import io
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

path = sys.argv[1]
a, b = int(sys.argv[2]), int(sys.argv[3])
root = '--root' in sys.argv
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
            return n.group(1) if n else '?'
        sid = f(t, 'scope')
    return '?'


cache = {}


def desc(i):
    if i in cache:
        return cache[i]
    t = md.get(i, '')
    s = '%s:%s' % (spname(f(t, 'scope')), num(t, 'line'))
    if root:
        cur, d = i, 0
        while True:
            tt = md.get(cur, '')
            ia = f(tt, 'inlinedAt')
            if ia is None:
                break
            cur = ia
            d += 1
        if d:
            tt = md.get(cur, '')
            s += ' <=%s:%s' % (spname(f(tt, 'scope')), num(tt, 'line'))
    cache[i] = s
    return s


for i in range(a, min(b, len(src)) + 1):
    ln = src[i - 1]
    m = re.search(r'!dbg !(\d+)', ln)
    tag = desc(int(m.group(1))) if m else ''
    print('%-7d %-34s %s' % (i, tag, ln.rstrip()))
