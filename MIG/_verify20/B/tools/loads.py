# -*- coding: utf-8 -*-
"""함수 범위 안 load 를 루트 인자별·오프셋별로 집계. 사용: python loads.py <ll> <from> <to>"""
import re, sys, collections
sys.path.insert(0, __import__('os').path.dirname(__file__))
from irlib import LL
ll = LL(sys.argv[1]); a = int(sys.argv[2]); b = int(sys.argv[3])
defs = {}
for n in range(a, b + 1):
    m = re.match(r'\s*(%[\w.]+) = (.*)', ll.line(n))
    if m: defs[m.group(1)] = (n, m.group(2))

def trace(v):
    path = []; seen = set()
    while v in defs and v not in seen:
        seen.add(v)
        n, t = defs[v]
        if t.startswith('getelementptr'):
            m = re.match(r'getelementptr[^,]*, ptr (%[\w.]+), (.*?)(, !dbg.*)?$', t)
            if not m: break
            path.append(('gep', m.group(2).strip())); v = m.group(1)
        elif t.startswith('load ptr'):
            m = re.match(r'load ptr, ptr (%[\w.]+)', t)
            path.append(('load',));
            if not m: break
            v = m.group(1)
        else:
            path.append((t.split(' ')[0],)); break
    return v, path

def key(path):
    return ' <- '.join('gep[' + p[1] + ']' if p[0] == 'gep' else p[0] for p in path)

agg = collections.OrderedDict()
for n in range(a, b + 1):
    ln = ll.line(n).strip()
    m = re.match(r'(%[\w.]+) = load (\S+), ptr (%[\w.]+)', ln)
    if not m: continue
    root, path = trace(m.group(3))
    k = (root, key(path), m.group(2))
    rl = ll.root_of_line(n)
    agg.setdefault(k, []).append((n, rl[1] if rl else 0))
for (root, k, ty), v in sorted(agg.items(), key=lambda kv: (kv[0][0], kv[0][1])):
    print(f"{root:10s} {ty:6s} {k:80s} n={len(v)} lines={[x[0] for x in v][:6]} src={sorted(set(x[1] for x in v))[:8]}")
