#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""어떤 store 지점이 **어떤 조건 아래** 실행되는지 역추적한다.

  python guard.py <ll> <함수define줄> <타깃정규식> [깊이]

타깃정규식에 걸리는 IR 줄마다:
  - 그 줄이 속한 basic block 을 찾고
  - 그 블록으로 들어오는 br i1 조건(들)을 깊이만큼 거슬러 올라가며
  - 각 조건값을 정의한 icmp/call 을 소스줄과 함께 보여준다.
"""
import io
import os
import re
import sys
from collections import defaultdict

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

path = sys.argv[1]
if not os.path.isabs(path):
    for base in (r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc'):
        if os.path.exists(os.path.join(base, path)):
            path = os.path.join(base, path)
            break
start = int(sys.argv[2])
targ = re.compile(sys.argv[3])
depth = int(sys.argv[4]) if len(sys.argv) > 4 else 3

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
            _sp[i] = (m.group(1).split('<')[0], int(f.group(1)) if f else -1)
            return _sp[i]
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
    _fn[i] = m.group(1).replace('\\\\', '/').split('/')[-1] if m else '?'
    return _fn[i]


GAME = re.compile(r'game-(ai|core|view)')


def loc(L):
    m = re.search(r'!dbg !(\d+)', L)
    if not m:
        return ''
    i = int(m.group(1))
    fr = []
    while i is not None:
        s = md.get(i, '')
        mm = re.search(r'DILocation\(line: (\d+),.*?scope: !(\d+)(?:, inlinedAt: !(\d+))?', s)
        if not mm:
            break
        nm, fi = sp(int(mm.group(2)))
        fr.append((filen(fi), mm.group(1), nm))
        i = int(mm.group(3)) if mm.group(3) else None
    for f in fr:
        if GAME.search(f[0]) or f[0].endswith('.rs') and 'library' not in f[0]:
            pass
    for f in fr:
        if f[0] not in ('?',) and not f[0].startswith(('iter', 'slice', 'option', 'cmp', 'mod')):
            return '%s:%s' % (f[0], f[1])
    return ''


end = start
while end < len(lines) and lines[end].rstrip() != '}':
    end += 1
body = lines[start - 1:end + 1]           # body[k] -> file line start+k

blk_of = {}       # ir index -> block label
blk_start = {}    # label -> ir index
cur = 'entry'
blk_start['entry'] = 0
defs = {}         # %v -> ir index
preds = defaultdict(list)   # label -> [(pred_label, cond, taken)]
for k, L in enumerate(body):
    m = re.match(r'^(\S+):\s*(;.*)?$', L)
    if m and not L.startswith((' ', '\t', 'define', '}')):
        cur = m.group(1)
        blk_start[cur] = k
    blk_of[k] = cur
    m = re.match(r'\s*(%\w+) = ', L)
    if m:
        defs[m.group(1)] = k
    m = re.match(r'\s*br i1 (%\w+), label %(\w+), label %(\w+)', L)
    if m:
        preds[m.group(2)].append((cur, m.group(1), True))
        preds[m.group(3)].append((cur, m.group(1), False))
    m = re.match(r'\s*br label %(\w+)', L)
    if m:
        preds[m.group(1)].append((cur, None, None))
    m = re.match(r'\s*switch \w+ (%\w+), label %(\w+)', L)
    if m:
        preds[m.group(2)].append((cur, m.group(1), 'switch-default'))


def show(k, pre=''):
    print('%s%-16s L%-7d %s' % (pre, loc(body[k]), start + k, body[k].strip()[:150]))


def walk(label, d, seen):
    if d <= 0 or label in seen:
        return
    seen.add(label)
    for (pl, cond, taken) in preds.get(label, [])[:3]:
        if cond is None:
            walk(pl, d, seen)
            continue
        print('   %s<= [%s] %s' % ('  ' * (depth - d), 'TRUE' if taken is True else ('FALSE' if taken is False else taken), cond))
        if cond in defs:
            show(defs[cond], '      ' + '  ' * (depth - d))
            # 조건이 icmp 면 피연산자 정의도 한 줄
            dl = body[defs[cond]]
            for op in re.findall(r'(%\w+)', dl)[1:3]:
                if op in defs and ('call' in body[defs[op]] or 'icmp' in body[defs[op]] or 'load' in body[defs[op]]):
                    show(defs[op], '         ' + '  ' * (depth - d))
        walk(pl, d - 1, seen)


for k, L in enumerate(body):
    if targ.search(L):
        print('=' * 90)
        show(k, '>> ')
        walk(blk_of[k], depth, set())
