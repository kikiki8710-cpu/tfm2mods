#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""인라인된 헬퍼의 **호출 사이트**를 전부 찾는다.

  python inlsites.py <ll파일> <헬퍼이름> [--store]

`!DILocation` 의 scope 를 DILexicalBlock/DISubprogram 사슬로 타 올라가
그 위치가 <헬퍼이름> 안이면, `inlinedAt` 루트까지 타서 **호출자 함수+줄**을 낸다.
--store 를 주면 그 !dbg 를 단 `store i8 <상수>` 의 상수도 같이 낸다.
"""
import io
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

path, want = sys.argv[1], sys.argv[2]
src = io.open(path, encoding='utf-8', errors='replace').read().split('\n')

md = {}          # id -> raw text
for ln in src:
    m = re.match(r'^!(\d+) = (.*)$', ln)
    if m:
        md[int(m.group(1))] = m.group(2)


def field(txt, key):
    m = re.search(r'\b%s: !(\d+)' % key, txt)
    return int(m.group(1)) if m else None


def numfield(txt, key):
    m = re.search(r'\b%s: (\d+)' % key, txt)
    return int(m.group(1)) if m else None


def sp_of(sid, depth=0):
    """scope id -> (subprogram name, subprogram id) 까지 타고 올라감"""
    seen = 0
    while sid is not None and seen < 40:
        seen += 1
        t = md.get(sid, '')
        if 'DISubprogram' in t:
            n = re.search(r'name: "([^"]*)"', t)
            return (n.group(1) if n else '?'), sid
        sid = field(t, 'scope')
    return None, None


def root_loc(locid, depth=0):
    """inlinedAt 루트까지 타서 (파일없음) (line, scopeSubprogramName) 반환"""
    seen = 0
    cur = locid
    while seen < 60:
        seen += 1
        t = md.get(cur, '')
        ia = field(t, 'inlinedAt')
        if ia is None:
            ln = numfield(t, 'line')
            nm, _ = sp_of(field(t, 'scope'))
            return ln, nm
        cur = ia
    return None, None


def file_of(sid):
    t = md.get(sid, '')
    f = field(t, 'file')
    if f is None:
        return '?'
    m = re.search(r'filename: "([^"]*)"', md.get(f, ''))
    return m.group(1) if m else '?'


# 1) want 안에 있는 DILocation id 수집
inside = {}
for i, t in md.items():
    if not t.startswith('!DILocation'):
        continue
    nm, spid = sp_of(field(t, 'scope'))
    if nm == want:
        inside[i] = (numfield(t, 'line'), spid)

if not inside:
    print('DILocation 없음: %s' % want)
    sys.exit()

# 2) 그 !dbg 를 쓰는 명령 수집
use = re.compile(r'!dbg !(\d+)')
rows = []
for lineno, ln in enumerate(src, 1):
    if '!dbg !' not in ln:
        continue
    for m in use.finditer(ln):
        i = int(m.group(1))
        if i in inside:
            rows.append((lineno, ln.strip(), i))
            break

out = {}
for lineno, txt, i in rows:
    hl, hs = inside[i]
    rl, rn = root_loc(i)
    st = re.search(r'store i8 (-?\d+)', txt)
    key = (rn, rl)
    out.setdefault(key, []).append((lineno, hl, st.group(1) if st else '', txt[:150]))

for (rn, rl), v in sorted(out.items(), key=lambda x: (str(x[0][0]), x[0][1] or 0)):
    consts = sorted(set(c for _, _, c, _ in v if c))
    print('=== 호출자 %s : line %s   (%d개 명령, store i8 상수=%s)' %
          (rn, rl, len(v), ','.join(consts) or '-'))
    for lineno, hl, c, txt in v[:6]:
        print('    %s:%d  [헬퍼 line %s] %s' % (path.split('/')[-1], lineno, hl, txt))
